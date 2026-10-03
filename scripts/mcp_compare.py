#!/usr/bin/env python3
"""Hub vs MCP diff. Edit the comparison block; run with `just compare-mcp`."""

import json
import subprocess
import sys
import time
import urllib.error
import urllib.request

HUB = "https://artifacthub.io/api/v1"
BIN = "./target/debug/artifacthub-mcp"
MAX_RETRIES = 3
RETRY_BACKOFF_S = [1, 2]


def _preview(body: bytes, limit: int = 200) -> str:
    text = body.decode("utf-8", errors="replace")
    shortened = text[:limit]
    return shortened if len(text) <= limit else shortened + "…"


def _fetch(path: str) -> tuple[int, bytes]:
    """Single Hub GET. Returns (status, body). Raises URLError/HTTPError."""
    url = HUB + path
    with urllib.request.urlopen(url, timeout=30) as response:
        return response.status, response.read()


def _fetch_with_retry(path: str) -> tuple[int, bytes, int]:
    url = HUB + path
    last_error: Exception | None = None
    for attempt in range(1, MAX_RETRIES + 1):
        try:
            status, body = _fetch(path)
            # Empty 200 (the Sep 28 flake) is retryable, not a drift signal.
            if status == 200 and len(body) == 0:
                raise ValueError("empty body with status 200")
            return status, body, attempt
        except urllib.error.HTTPError as e:
            # Retry rate-limit / transient 5xx only; fail fast on other 4xx.
            retryable = e.code == 429 or 500 <= e.code <= 599
            detail = f"hub {url} attempt {attempt}/{MAX_RETRIES}: HTTP {e.code} {e.reason}"
            print(detail, file=sys.stderr)
            last_error = e
            if not retryable:
                break
        except (urllib.error.URLError, ValueError, TimeoutError, ConnectionError) as e:
            print(f"hub {url} attempt {attempt}/{MAX_RETRIES}: {type(e).__name__}: {e}", file=sys.stderr)
            last_error = e
        if attempt < MAX_RETRIES:
            time.sleep(RETRY_BACKOFF_S[min(attempt - 1, len(RETRY_BACKOFF_S) - 1)])
    raise last_error if last_error else RuntimeError(f"hub {url}: unknown fetch failure")


def hub_json(path: str):
    url = HUB + path
    status, body, attempt = _fetch_with_retry(path)
    try:
        return json.loads(body.decode())
    except json.JSONDecodeError as e:
        print(
            f"compare unavailable: hub {url} attempt={attempt}/{MAX_RETRIES} "
            f"status={status} len={len(body)} "
            f"body={_preview(body)!r} json_error={e}",
            file=sys.stderr,
        )
        raise SystemExit(2)


def hub_text(path: str):
    url = HUB + path
    status, body, _attempt = _fetch_with_retry(path)
    # hub_text has no JSON parsing, so an empty body here is already
    # retried above; anything reaching here is returned as-is.
    if len(body) == 0:
        print(
            f"compare unavailable: hub {url} status={status} len=0 (empty text body)",
            file=sys.stderr,
        )
        raise SystemExit(2)
    return body.decode()


def report_error(error_type, error, traceback):
    if issubclass(
        error_type,
        (
            urllib.error.HTTPError,
            urllib.error.URLError,
            json.JSONDecodeError,
            ValueError,
            ConnectionError,
            TimeoutError,
        ),
    ):
        print(f"compare unavailable: {error}", file=sys.stderr)
        raise SystemExit(2)
    sys.__excepthook__(error_type, error, traceback)


sys.excepthook = report_error


proc = subprocess.Popen(
    [BIN], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1
)
request_id = 0


def rpc(method, params):
    global request_id
    request_id += 1
    proc.stdin.write(
        json.dumps(
            {"jsonrpc": "2.0", "id": request_id, "method": method, "params": params}
        )
        + "\n"
    )
    proc.stdin.flush()
    while True:
        raw = proc.stdout.readline()
        if raw == "":
            code = proc.poll()
            print(
                f"compare unavailable: mcp method={method} id={request_id} "
                f"got empty stdout (process exited? returncode={code})",
                file=sys.stderr,
            )
            raise SystemExit(2)
        try:
            response = json.loads(raw)
        except json.JSONDecodeError as e:
            print(
                f"compare unavailable: mcp method={method} id={request_id} "
                f"len={len(raw)} line={raw[:200]!r} json_error={e}",
                file=sys.stderr,
            )
            raise SystemExit(2)
        if not isinstance(response, dict):
            print(
                f"compare unavailable: mcp method={method} id={request_id} "
                f"got non-object JSON response type={type(response).__name__}",
                file=sys.stderr,
            )
            raise SystemExit(2)
        if "id" in response:
            return response["result"]


def compare(*, tool, url, arguments, expected):
    result = rpc("tools/call", {"name": tool, "arguments": arguments})
    actual = result["structuredContent"]
    if actual == expected:
        print(f"ok {tool} {HUB + url}")
        return True
    print(f"FAIL {tool} {HUB + url}")
    print("hub:", json.dumps(expected, sort_keys=True)[:1000])
    print("mcp:", json.dumps(actual, sort_keys=True)[:1000])
    return False


rpc(
    "initialize",
    {
        "protocolVersion": "2025-11-25",
        "capabilities": {},
        "clientInfo": {"name": "compare", "version": "1"},
    },
)
proc.stdin.write(
    json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n"
)
proc.stdin.flush()

nginx = hub_json("/packages/helm/bitnami/nginx")
nginx_expected = {
    key: value
    for key, value in nginx.items()
    if key not in ("readme", "available_versions")
}
nginx_expected.setdefault("keywords", [])

redis = hub_json("/packages/helm/bitnami/redis")
postgresql = hub_json("/packages/helm/bitnami/postgresql")
loki = hub_json("/packages/helm/grafana-community/loki")
nginx_security = hub_json(
    f"/packages/{nginx['package_id']}/{nginx['version']}/security-report"
)
for report in nginx_security.values():
    for result in report.setdefault("Results", []):
        result.setdefault("Vulnerabilities", [])

results = [
    compare(
        tool="get_package",
        url="/packages/helm/bitnami/nginx",
        arguments={"kind": "helm", "repo": "bitnami", "name": "nginx"},
        expected=nginx_expected,
    ),
    compare(
        tool="get_package_readme",
        url="/packages/helm/bitnami/redis",
        arguments={"kind": "helm", "repo": "bitnami", "name": "redis"},
        expected={"readme": redis["readme"]},
    ),
    compare(
        tool="get_package_versions",
        url="/packages/helm/bitnami/postgresql",
        arguments={"kind": "helm", "repo": "bitnami", "name": "postgresql"},
        expected={
            "versions": postgresql["available_versions"],
            "count": len(postgresql["available_versions"]),
        },
    ),
    compare(
        tool="get_package_changelog",
        url=f"/packages/{redis['package_id']}/changelog",
        arguments={"kind": "helm", "repo": "bitnami", "name": "redis"},
        expected={"entries": hub_json(f"/packages/{redis['package_id']}/changelog")},
    ),
    compare(
        tool="get_package_star_stats",
        url=f"/packages/{nginx['package_id']}/stars",
        arguments={"kind": "helm", "repo": "bitnami", "name": "nginx"},
        expected=hub_json(f"/packages/{nginx['package_id']}/stars"),
    ),
    compare(
        tool="get_package_values",
        url=f"/packages/{postgresql['package_id']}/{postgresql['version']}/values",
        arguments={
            "kind": "helm",
            "repo": "bitnami",
            "name": "postgresql",
            "version": postgresql["version"],
        },
        expected={
            "package": "postgresql",
            "version": postgresql["version"],
            "values": hub_text(
                f"/packages/{postgresql['package_id']}/{postgresql['version']}/values"
            ),
        },
    ),
    compare(
        tool="get_package_security_report",
        url=f"/packages/{nginx['package_id']}/{nginx['version']}/security-report",
        arguments={"package_id": nginx["package_id"], "version": nginx["version"]},
        expected=nginx_security,
    ),
    compare(
        tool="get_package_values_schema",
        url=f"/packages/{loki['package_id']}/{loki['version']}/values-schema",
        arguments={"package_id": loki["package_id"], "version": loki["version"]},
        expected={
            "schema": hub_json(
                f"/packages/{loki['package_id']}/{loki['version']}/values-schema"
            )
        },
    ),
    compare(
        tool="get_changelog_md",
        url="/packages/helm/kyverno/kyverno/changelog.md",
        arguments={"kind": "helm", "repo": "kyverno", "name": "kyverno"},
        expected={"changelog": hub_text("/packages/helm/kyverno/kyverno/changelog.md")},
    ),
]


proc.terminate()
raise SystemExit(0 if all(results) else 1)
