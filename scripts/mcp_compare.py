#!/usr/bin/env python3
"""Hub vs MCP diff. Edit the comparison block; run with `just compare-mcp`."""

import json
import subprocess
import sys
import urllib.error
import urllib.request

HUB = "https://artifacthub.io/api/v1"
BIN = "./target/debug/artifacthub-mcp"


def report_error(error_type, error, traceback):
    if issubclass(
        error_type,
        (urllib.error.HTTPError, urllib.error.URLError, json.JSONDecodeError),
    ):
        print(f"compare unavailable: {error}")
        raise SystemExit(2)
    sys.__excepthook__(error_type, error, traceback)


sys.excepthook = report_error


def hub_json(path):
    with urllib.request.urlopen(HUB + path, timeout=30) as response:
        return json.loads(response.read().decode())


def hub_text(path):
    with urllib.request.urlopen(HUB + path, timeout=30) as response:
        return response.read().decode()


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
        response = json.loads(proc.stdout.readline())
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
redis = hub_json("/packages/helm/bitnami/redis")
postgresql = hub_json("/packages/helm/bitnami/postgresql")
loki = hub_json("/packages/helm/grafana-community/loki")

results = [
    compare(
        tool="get_package",
        url="/packages/helm/bitnami/nginx",
        arguments={"kind": "helm", "repo": "bitnami", "name": "nginx"},
        expected={
            **{
                key: value
                for key, value in nginx.items()
                if key not in ("readme", "available_versions")
            },
            "keywords": nginx.get("keywords", []),
        },
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
        expected=hub_json(
            f"/packages/{nginx['package_id']}/{nginx['version']}/security-report"
        ),
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
