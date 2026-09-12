# MCP conformance checks

Run `just sweep` to build the server and compare its existing 15 tools with
independent requests to Artifact Hub's public API. No credentials are needed.
The sweep uses the Artifact Hub chart and Bitnami nginx (an OCI chart), checking
latest and older pinned versions, metadata, README, values, schemas, templates,
security reports, changelogs, stars, search facets, pagination and repeated
organization filters. Missing versions must return the same HTTP error as Hub.

Comparisons require identical fields, scalar types, values and array order.
The only allowed transformations reflect the existing tool contracts:

- Package metadata excludes top-level `readme` and `available_versions`, which
  have dedicated tools. Missing default fields become empty strings, empty
  arrays, zero or false according to the client models.
- Version lists include the full count even when the requested list is limited.
  Search totals come from Hub's `Pagination-Total-Count` header.
- README, values, schema, changelog and template results use the tool's wrappers.
  Template lists expose names; template source is decoded from base64. YAML,
  Markdown and decoded template source are compared as exact text.
- Empty schema responses become null; empty security responses become `{}`.
  Null security result/vulnerability collections become empty arrays. Known
  optional security fields with null values are omitted; arbitrary Trivy fields
  are preserved. Empty search signatures are omitted.

The script exits with `0` only when all comparisons pass and every advertised
tool has a successful response comparison. Exit `1` means a mismatch or missing
tool coverage. Exit `2` means verification was unavailable, including transport
failures, rate limits, upstream server errors or data changing between reads.
An expected 404 counts as an error-contract check, not successful tool coverage.
No unavailable result is counted as a pass.

Run `python3 -B -m unittest discover -s scripts -p 'test_*.py'` for offline tests
of the comparison and reporting logic. These deliberately supply missing fields,
wrong versions, truncated arrays, incorrect errors and changing responses.
Rust tests cover request construction and deterministic empty/error responses.
The live sweep runs on the existing schedule and can be dispatched manually.

These checks cover representative public Helm reads through the existing tools.
They do not prove every chart or future upstream response works, nor cover
authenticated operations or additional Hub features.
