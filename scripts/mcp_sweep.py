#!/usr/bin/env python3
"""Drive artifacthub-mcp over stdio, exercising all tools against the LIVE API.

Usage: python3 scripts/mcp_sweep.py /path/to/artifacthub-mcp
(or `just sweep` from the repo root).
Prints per-tool ok/FAIL with error excerpts. Exit 1 if any tool failed.

Note: requires network access to artifacthub.io. A FAIL for
get_changelog_md with a 404 is expected for packages that
genuinely have no changelog file upstream.
"""
import json
import subprocess
import sys

BIN = sys.argv[1] if len(sys.argv) > 1 else "./target/debug/artifacthub-mcp"


class MCP:
    def __init__(self, bin):
        self.p = subprocess.Popen(
            [bin], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, text=True, bufsize=1,
        )
        self._id = 0

    def _send(self, msg):
        self.p.stdin.write(json.dumps(msg) + "\n")
        self.p.stdin.flush()
        return json.loads(self.p.stdout.readline())

    def init(self):
        self._id += 1
        self._send({"jsonrpc": "2.0", "id": self._id, "method": "initialize",
                    "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                               "clientInfo": {"name": "sweep", "version": "0"}}})
        self.p.stdin.write(json.dumps({"jsonrpc": "2.0",
                                       "method": "notifications/initialized"}) + "\n")
        self.p.stdin.flush()

    def call(self, name, args):
        self._id += 1
        return self._send({"jsonrpc": "2.0", "id": self._id, "method": "tools/call",
                           "params": {"name": name, "arguments": args}})

    def close(self):
        self.p.stdin.close()
        self.p.terminate()


def check(resp):
    """Return (ok, detail)."""
    if "error" in resp:
        return False, f"protocol error: {json.dumps(resp['error'])[:300]}"
    r = resp.get("result", {})
    if r.get("isError"):
        content = json.dumps(r.get("content"))[:300]
        return False, f"tool error: {content}"
    return True, ""


def main():
    m = MCP(BIN)
    m.init()
    results = []

    def run(name, args, desc=""):
        ok, detail = check(m.call(name, args))
        results.append((name, desc, ok, detail))
        print(f"{'ok  ' if ok else 'FAIL'} {name} {desc} {detail}")

    run("get_server_info", {})
    run("search_packages", {"q": "nginx", "kind": ["helm"], "limit": 5}, "[happy]")
    run("search_packages", {"org": ["kvalitetsit"], "kind": ["helm"], "limit": 60}, "[edge]")

    # Chain: pick a kvalitetsit package for per-package tools
    resp = m.call("search_packages", {"org": ["kvalitetsit"], "kind": ["helm"], "limit": 5})
    chain = resp.get("result", {}).get("structuredContent")
    if chain is None:
        print(f"FAIL search_packages [chain] {json.dumps(resp)[:300]} - cannot continue")
        m.close()
        sys.exit(1)
    pkgs = chain["packages"]
    assert pkgs, "no kvalitetsit packages returned"
    # prefer one with a description AND one without, to cover both
    with_desc = next((p for p in pkgs if p.get("description")), pkgs[0])
    without_desc = next((p for p in pkgs if not p.get("description")), None)
    print(f"chained package: {with_desc['name']} "
          f"(desc={bool(with_desc.get('description'))}); "
          f"nodesc sample: {without_desc['name'] if without_desc else 'none in batch'}")

    triple = {"kind": "helm", "repo": with_desc["repository"]["name"],
              "name": with_desc["name"]}
    run("get_package", triple, "[edge-pkg]")
    run("get_package", {"kind": "helm", "repo": "bitnami", "name": "nginx"}, "[happy]")
    run("get_package_readme", triple, "[edge-pkg]")
    run("get_package_versions", dict(triple, limit=3), "[edge-pkg]")
    run("get_package_changelog", triple, "[edge-pkg]")
    # The Hub chart publishes structured changes; bitnami/nginx currently does not.
    run("get_changelog_md",
        {"kind": "helm", "repo": "artifact-hub", "name": "artifact-hub"}, "[happy]")
    # Edge package may genuinely have no changelog file upstream (404).
    # That is correct tool behavior, not a bug: count it as ok, loudly.
    md_resp = m.call("get_changelog_md", triple)
    md_result = md_resp.get("result", {})
    if md_result.get("isError") and "404" in json.dumps(md_result.get("content")):
        results.append(("get_changelog_md", "[edge-pkg]", True, ""))
        print("ok   get_changelog_md [edge-pkg] (expected upstream 404: no changelog file)")
    else:
        ok, detail = check(md_resp)
        results.append(("get_changelog_md", "[edge-pkg]", ok, detail))
        print(f"{'ok  ' if ok else 'FAIL'} get_changelog_md [edge-pkg] {detail}")
    run("get_package_star_stats", triple, "[edge-pkg]")
    run("get_package_values", triple, "[edge-pkg]")

    # package_id/version-gated tools
    pkg_resp = m.call("get_package", triple)
    if "structuredContent" not in pkg_resp.get("result", {}):
        print(f"FAIL get_package {triple} "
              f"{json.dumps(pkg_resp)[:300]} - cannot chain id-gated tools")
        m.close()
        sys.exit(1)
    pkg = pkg_resp["result"]["structuredContent"]
    pid, ver = pkg["package_id"], pkg["version"]
    print(f"package_id={pid} version={ver}")
    run("get_package_security_report", {"package_id": pid, "version": ver}, "[edge-pkg]")
    run("get_package_values_schema", {"package_id": pid, "version": ver}, "[edge-pkg]")
    run("get_package_templates", {"package_id": pid, "version": ver}, "[edge-pkg]")

    tmpl_resp = m.call("get_package_templates", {"package_id": pid, "version": ver})
    tmpl_struct = tmpl_resp.get("result", {}).get("structuredContent")
    if tmpl_struct is None:
        print(f"FAIL get_package_templates {json.dumps(tmpl_resp)[:300]}")
        m.close()
        sys.exit(1)
    tmpls = tmpl_struct["templates"]
    if tmpls:
        tname = tmpls[0]["name"]
        run("get_package_template",
            {"package_id": pid, "version": ver, "name": tname}, f"[{tname}]")
        run("get_package_template_data",
            {"package_id": pid, "version": ver, "name": tname}, f"[{tname}]")
    else:
        print("note: no templates for chained package, trying bitnami/nginx")
        pkg2 = m.call("get_package", {"kind": "helm", "repo": "bitnami",
                                      "name": "nginx"})["result"]["structuredContent"]
        pid2, ver2 = pkg2["package_id"], pkg2["version"]
        tmpl2 = m.call("get_package_templates",
                       {"package_id": pid2, "version": ver2})["result"]["structuredContent"]["templates"]
        tname = tmpl2[0]["name"]
        run("get_package_template",
            {"package_id": pid2, "version": ver2, "name": tname}, f"[nginx {tname}]")
        run("get_package_template_data",
            {"package_id": pid2, "version": ver2, "name": tname}, f"[nginx {tname}]")

    run("search_repositories", {"name": "bitnami", "kind": ["helm"], "limit": 5}, "[happy]")
    run("search_repositories",
        {"org": ["kvalitetsit"], "kind": ["helm"], "limit": 60}, "[edge]")

    m.close()
    fails = [r for r in results if not r[2]]
    print(f"\n{len(results) - len(fails)}/{len(results)} tools ok")
    sys.exit(1 if fails else 0)


main()
