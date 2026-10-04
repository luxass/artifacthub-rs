---
title: Artifact Hub MCP
navTitle: Overview
order: 0
description: An MCP server for searching Artifact Hub packages and reading Helm chart data.
---

## What can it do?

[Artifact Hub](https://artifacthub.io) indexes Helm charts and other cloud-native packages. `artifacthub-mcp` gives your MCP-compatible assistant 15 read-only tools to query that data.

- Find packages by kind, repository, or publisher.
- Check package metadata and available versions.
- Read a chart's default `values.yaml`, values schema, and README.
- Inspect chart template names and source.
- Review published changelogs and security reports.
- Search repositories by name, URL, user, or organization.

Start with [installation and client setup](/getting-started/), then try an [example workflow](/workflows/). The [tool reference](/tools/) lists every tool and its inputs.

## A prompt to try

After connecting the server, ask:

> Find the prometheus chart in prometheus-community on Artifact Hub. Read its current default values and explain how persistence is configured. Cite the chart version and the values keys you used.

The assistant can search for the package, read its metadata, and fetch its default values. It doesn't need to infer values keys from its training data.

Your assistant chooses which tools to call. If it skips a needed lookup, ask it explicitly to use the Artifact Hub tools before answering.

## How it connects

Your MCP client starts the installed `artifacthub-mcp` binary as a local process. The client and server communicate over standard input and output. The server makes HTTPS requests to the public Artifact Hub API.

```text
AI assistant / MCP client
          │ stdio
          ▼
    artifacthub-mcp
          │ HTTPS
          ▼
    Artifact Hub API
```

There is no HTTP MCP endpoint to host, and no Artifact Hub API key is required for these public lookups. The binary must be available on the machine running your MCP client.

## What it doesn't do

The server does not install charts, run Helm, connect to your cluster, or change resources. Template tools return source, not rendered Kubernetes manifests.

It reads what Artifact Hub publishes. Some packages have no changelog, values schema, or security report. Missing scan data does not mean a package is safe, and a package's README or templates are untrusted third-party content.

This documentation site is static. It does not run the MCP server or make live package queries in your browser.
