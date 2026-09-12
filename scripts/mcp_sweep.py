#!/usr/bin/env python3
"""Compare the existing MCP tools with Artifact Hub's public Helm API.

Usage: python3 scripts/mcp_sweep.py [path/to/artifacthub-mcp]
Exit 0: all comparisons passed; 1: mismatch; 2: verification unavailable.
Only the response transformations documented in scripts/README.md are allowed.
"""
import base64
import copy
import json
import queue
import re
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.parse
import urllib.request


class Unavailable(Exception):
    """A transport failure or changing upstream data prevented verification."""


class MCP:
    def __init__(self, binary):
        self.p = subprocess.Popen(
            [binary], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=None, text=True, bufsize=1,
        )
        self._id = 0
        self.lines = queue.Queue()
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self):
        for line in self.p.stdout:
            self.lines.put(line)
        self.lines.put(None)

    def request(self, method, params):
        self._id += 1
        self.p.stdin.write(json.dumps({"jsonrpc": "2.0", "id": self._id,
                                      "method": method, "params": params}) + "\n")
        self.p.stdin.flush()
        deadline = time.monotonic() + 40
        while True:
            try:
                line = self.lines.get(timeout=max(0, deadline - time.monotonic()))
            except queue.Empty as error:
                raise Unavailable("MCP response timed out") from error
            if line is None:
                raise AssertionError("MCP process closed stdout before replying")
            response = json.loads(line)
            if "id" not in response:  # Server notifications are not replies.
                continue
            assert response["id"] == self._id, "MCP response ID mismatch"
            assert "error" not in response, f"MCP protocol error: {response['error']}"
            assert isinstance(response.get("result"), dict), "Missing MCP result object"
            return response["result"]

    def init(self):
        self.request("initialize", {"protocolVersion": "2025-11-25", "capabilities": {},
                                    "clientInfo": {"name": "conformance-sweep", "version": "1"}})
        self.p.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
        self.p.stdin.flush()
        return {tool["name"] for tool in self.request("tools/list", {})["tools"]}

    def call(self, name, args):
        return self.request("tools/call", {"name": name, "arguments": args})

    def close(self):
        self.p.terminate()
        try:
            self.p.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.p.kill()
            self.p.wait()
        self.reader.join(timeout=5)
        self.p.stdin.close()
        self.p.stdout.close()


def hub(path, params=(), text=False):
    url = "https://artifacthub.io/api/v1" + path
    if params:
        url += "?" + urllib.parse.urlencode(params)
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            body = response.read().decode("utf-8")
            total = response.headers.get("Pagination-Total-Count")
    except urllib.error.HTTPError as error:
        if error.code == 429 or error.code >= 500:
            raise Unavailable(f"Hub HTTP {error.code}: {url}") from error
        raise
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        raise Unavailable(f"Hub request failed: {url}: {error}") from error
    if text:
        return body
    value = json.loads(body) if body.strip() else None
    if total is not None:
        if isinstance(value, list):
            value = {"repositories": value}
        value["total_count"] = int(total)
    return value


def assert_equal(expected, actual, path="$"):
    """Strict comparison: missing/extra keys, list order, scalar types and values."""
    assert type(expected) is type(actual), f"{path}: expected {type(expected).__name__}, got {type(actual).__name__}"
    if isinstance(expected, dict):
        assert expected.keys() == actual.keys(), (
            f"{path}: missing {sorted(expected.keys() - actual.keys())}; "
            f"unexpected {sorted(actual.keys() - expected.keys())}"
        )
        for key in expected:
            assert_equal(expected[key], actual[key], f"{path}.{key}")
    elif isinstance(expected, list):
        assert len(expected) == len(actual), f"{path}: expected {len(expected)} items, got {len(actual)}"
        for i, (left, right) in enumerate(zip(expected, actual)):
            assert_equal(left, right, f"{path}[{i}]")
    else:
        assert expected == actual, f"{path}: expected {repr(expected)[:160]}, got {repr(actual)[:160]}"


def result_value(result, expected_status=None):
    content = result.get("content", [])
    message = "\n".join(item.get("text", "") for item in content if item.get("type") == "text")
    if result.get("isError") and re.match(r"(?:Request failed:|Failed to read response:|API error (?:429|5\d\d)\b)", message):
        raise Unavailable(message[:300])
    if expected_status is not None:
        assert result.get("isError") is True, f"Expected upstream HTTP {expected_status} error"
        assert re.match(rf"API error {expected_status}(?:\s|:)", message), f"Wrong error: {message[:200]}"
        return None
    if result.get("isError"):
        raise AssertionError(f"Tool error: {message[:300]}")
    if "structuredContent" in result:
        return result["structuredContent"]
    assert len(content) == 1 and content[0].get("type") == "text", "Missing MCP output"
    return content[0]["text"]


def defaults(value, fields):
    for key, default in fields.items():
        value.setdefault(key, copy.deepcopy(default))
    return value


def metadata(raw):
    value = copy.deepcopy(raw)
    value.pop("readme", None)
    value.pop("available_versions", None)
    defaults(value, dict.fromkeys(["package_id", "name", "normalized_name", "version", "description"], ""))
    defaults(value, {"deprecated": False, "prerelease": False, "signed": False,
                     "keywords": [], "links": [], "ts": 0, "contains_security_updates": False})
    defaults(value.setdefault("repository", {}), {"name": "", "display_name": "", "url": "", "kind": 0,
                                                "verified_publisher": False, "official": False})
    defaults(value.setdefault("stats", {}), {"subscriptions": 0, "webhooks": 0})
    for image in value.get("containers_images", []):
        defaults(image, {"whitelisted": False})
    return value


def search_result(raw, repositories=False):
    value = copy.deepcopy(raw)
    for item in value["repositories" if repositories else "packages"]:
        if repositories:
            defaults(item, {"repository_id": "", "name": "", "url": "", "kind": 0,
                            "verified_publisher": False, "official": False})
        else:
            defaults(item, dict.fromkeys(["package_id", "name", "normalized_name", "version", "description"], ""))
            defaults(item, {"deprecated": False, "signed": False, "stars": 0, "ts": 0})
            defaults(item.setdefault("repository", {}), {"name": "", "url": ""})
            if item.get("signatures") == []:
                del item["signatures"]
    return value


def versions(raw, limit=None):
    items = copy.deepcopy(raw["available_versions"])
    for item in items:
        defaults(item, {"contains_security_updates": False, "prerelease": False})
    return {"versions": items if limit is None else items[:limit], "count": len(items)}


def security(raw):
    value = copy.deepcopy(raw) if raw is not None else {}
    for report in value.values():
        report["Results"] = report.get("Results") or []
        for scan in report["Results"]:
            scan.setdefault("Target", "")
            if scan.get("Type", "") is None:
                del scan["Type"]
            scan["Vulnerabilities"] = scan.get("Vulnerabilities") or []
            for vulnerability in scan["Vulnerabilities"]:
                for key in ["VulnerabilityID", "PkgName", "InstalledVersion", "FixedVersion", "Severity", "Title"]:
                    if key in vulnerability and vulnerability[key] is None:
                        del vulnerability[key]
    return value


class Sweep:
    def __init__(self, mcp):
        self.mcp = mcp
        self.attempted = set()
        self.validated = set()
        self.failures = 0
        self.unavailable = 0
        self.passed = 0

    def run(self, tool, args, reference, label=""):
        self.attempted.add(tool)
        try:
            try:
                expected = reference()
            except urllib.error.HTTPError as error:
                result_value(self.mcp.call(tool, args), expected_status=error.code)
                self.passed += 1
                print(f"ok {tool} {label}: confirmed Hub HTTP {error.code}", flush=True)
                return
            actual = result_value(self.mcp.call(tool, args))
            try:
                assert_equal(expected, actual)
            except AssertionError:
                # Counters/latest/search results can change between independent reads.
                # A changing reference is unverified, never a passing comparison.
                if reference() != expected:
                    raise Unavailable("Hub response changed during comparison")
                raise
            self.validated.add(tool)
            self.passed += 1
            print(f"ok {tool} {label}: data matches", flush=True)
        except Unavailable as error:
            self.unavailable += 1
            print(f"UNAVAILABLE {tool} {label}: {error}", flush=True)
        except (AssertionError, KeyError, TypeError, ValueError, urllib.error.HTTPError) as error:
            self.failures += 1
            print(f"FAIL {tool} {label}: {error}", flush=True)

    def finish(self, advertised):
        unhandled = advertised - self.attempted
        unexpected = self.attempted - advertised
        if unhandled or unexpected:
            self.failures += 1
            print(f"FAIL tool coverage: unhandled={sorted(unhandled)}, unadvertised={sorted(unexpected)}")
        unverified = advertised - self.validated
        print(f"\n{self.passed} comparisons passed; {self.failures} mismatches; {self.unavailable} unavailable")
        print(f"Successful response comparisons: {len(self.validated & advertised)}/{len(advertised)} tools")
        if unverified:
            print(f"Unverified successful responses: {', '.join(sorted(unverified))}")
        return 1 if self.failures else 2 if self.unavailable or unverified else 0


def package_cases(sweep, repo, name, require_oci=False):
    args = {"kind": "helm", "repo": repo, "name": name}
    path = "/packages/helm/" + urllib.parse.quote(repo, safe="") + "/" + urllib.parse.quote(name, safe="")
    raw = hub(path)  # All following reference IDs/versions come from Hub, never MCP.
    if require_oci:
        assert raw["content_url"].startswith("oci://"), "OCI sample no longer references an OCI chart"
    sweep.run("get_package", args, lambda: metadata(hub(path)), repo + " latest")
    sweep.run("get_package_readme", args, lambda: {"readme": hub(path)["readme"]}, repo)
    for limit in [None, 2]:
        version_args = args if limit is None else dict(args, limit=limit)
        sweep.run("get_package_versions", version_args, lambda limit=limit: versions(hub(path), limit), f"{repo} limit={limit}")
    pid = urllib.parse.quote(raw["package_id"], safe="")
    sweep.run("get_package_changelog", args, lambda: {"entries": hub(f"/packages/{pid}/changelog")}, repo)
    sweep.run("get_changelog_md", args, lambda: {"changelog": hub(path + "/changelog.md", text=True)}, repo)
    sweep.run("get_package_star_stats", args, lambda: defaults(hub(f"/packages/{pid}/stars"), {"stars": 0}), repo)
    old = next(v["version"] for v in raw["available_versions"] if v["version"] != raw["version"])
    for version in [raw["version"], old]:
        pinned = dict(args, version=version)
        version_path = path + "/" + urllib.parse.quote(version, safe="")
        snapshot = f"/packages/{pid}/{urllib.parse.quote(version, safe='')}"
        identity = {"package_id": raw["package_id"], "version": version}
        sweep.run("get_package", pinned, lambda: metadata(hub(version_path)), f"{repo} pinned {version}")
        sweep.run("get_package_readme", pinned, lambda: {"readme": hub(version_path)["readme"]}, f"{repo} pinned {version}")
        sweep.run("get_package_values", pinned, lambda: {"package": name, "version": version, "values": hub(snapshot + "/values", text=True)}, f"{repo} {version}")
        sweep.run("get_package_values_schema", identity, lambda: {"schema": hub(snapshot + "/values-schema")}, f"{repo} {version}")
        sweep.run("get_package_security_report", identity, lambda: security(hub(snapshot + "/security-report")), f"{repo} {version}")
        sweep.run("get_package_templates", identity, lambda: {"templates": [
            {"name": t["name"]} for t in hub(snapshot + "/templates")["templates"] or [] if t.get("name") is not None
        ]}, f"{repo} {version}")
        templates = hub(snapshot + "/templates")["templates"] or []
        if templates:
            template = next(t for t in templates if t.get("name") is not None and t.get("data") is not None)
            template_args = dict(identity, name=template["name"])
            source = base64.b64decode(template["data"], validate=True).decode("utf-8")
            sweep.run("get_package_template", template_args, lambda: {"name": template["name"], "data": source}, f"{repo} {version}")
            sweep.run("get_package_template_data", template_args, lambda: source, f"{repo} {version}")
    missing = dict(args, version="0.0.0-conformance-missing")
    sweep.run("get_package", missing, lambda: metadata(hub(path + "/" + missing["version"])), repo + " missing version")


def main():
    binary = sys.argv[1] if len(sys.argv) > 1 else "./target/debug/artifacthub-mcp"
    mcp = MCP(binary)
    sweep = Sweep(mcp)
    try:
        advertised = mcp.init()
        version = subprocess.check_output([binary, "--version"], text=True, timeout=5).strip().split()[-1]
        sweep.run("get_server_info", {}, lambda: {"name": "artifacthub-mcp", "version": version,
                                                  "description": "MCP server for interacting with Artifact Hub"})
        search_args = {"q": "nginx", "kind": ["helm"], "facets": True, "limit": 5, "offset": 1}
        sweep.run("search_packages", search_args, lambda: search_result(hub("/packages/search", [
            ("ts_query_web", "nginx"), ("kind", "0"), ("facets", "true"), ("limit", "5"), ("offset", "1")
        ])), "facets and pagination")
        for orgs in [["kvalitetsit"], ["kvalitetsit", "bitnami"]]:
            args = {"org": orgs, "kind": ["helm"], "limit": 5}
            params = [("org", org) for org in orgs] + [("kind", "0"), ("limit", "5")]
            sweep.run("search_packages", args, lambda: search_result(hub("/packages/search", params)), "organizations")
            sweep.run("search_repositories", args, lambda: search_result(hub("/repositories/search", params), True), "organizations")
        for repo, name, oci in [("artifact-hub", "artifact-hub", False), ("bitnami", "nginx", True)]:
            try:
                package_cases(sweep, repo, name, oci)
            except (Unavailable, urllib.error.HTTPError) as error:
                sweep.unavailable += 1
                print(f"UNAVAILABLE sample {repo}/{name}: {error}", flush=True)
            except (AssertionError, KeyError, StopIteration, ValueError) as error:
                sweep.failures += 1
                print(f"FAIL sample {repo}/{name}: {error}", flush=True)
        return sweep.finish(advertised)
    except Unavailable as error:
        print(f"UNAVAILABLE: {error}")
        return 2
    finally:
        mcp.close()


if __name__ == "__main__":
    sys.exit(main())
