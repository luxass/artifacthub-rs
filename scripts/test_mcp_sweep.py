"""Offline checks for the live comparison oracle; no network or MCP process."""
import contextlib
import copy
import io
import unittest
import urllib.error
from unittest.mock import Mock

from mcp_sweep import (
    Sweep, Unavailable, assert_equal, metadata, result_value, search_result, security,
)


class ComparisonTests(unittest.TestCase):
    def test_rejects_dropped_fields_at_every_depth(self):
        expected = {"repository": {"name": "repo"}, "data": {"custom": 7}}
        for key in expected:
            actual = copy.deepcopy(expected)
            actual[key].clear()
            with self.subTest(key=key), self.assertRaisesRegex(AssertionError, "missing"):
                assert_equal(expected, actual)

    def test_rejects_wrong_version_truncated_arrays_and_scalar_types(self):
        cases = [
            ({"version": "1.2.0"}, {"version": "1.3.0"}),
            (["a", "b"], ["a"]),
            (["a", "b"], ["b", "a"]),
            ({"official": False}, {"official": 0}),
            ({"count": 9007199254740993}, {"count": 9007199254740992}),
            ({}, {"unexpected": True}),
        ]
        for expected, actual in cases:
            with self.subTest(expected=expected), self.assertRaises(AssertionError):
                assert_equal(expected, actual)

    def test_metadata_projection_only_removes_dedicated_large_fields(self):
        raw = {"readme": "large", "available_versions": [], "license": "MIT",
               "data": {"readme": "keep nested data", "new_field": [1, 2]},
               "new_upstream_field": True}
        actual = metadata(raw)
        self.assertNotIn("readme", actual)
        self.assertNotIn("available_versions", actual)
        self.assertEqual(actual["license"], "MIT")
        self.assertEqual(actual["data"], raw["data"])
        self.assertTrue(actual["new_upstream_field"])
        self.assertIn("readme", raw)  # Reference normalization never changes its input.

    def test_search_defaults_do_not_overwrite_real_values_or_facets(self):
        raw = {"packages": [{"description": "keep", "signed": True, "stars": 9,
                             "signatures": [], "repository": {"url": "oci://registry/chart"}}],
               "facets": [{"options": [{"id": 0, "total": 9}]}], "total_count": 9}
        actual = search_result(raw)
        self.assertEqual(actual["packages"][0]["description"], "keep")
        self.assertTrue(actual["packages"][0]["signed"])
        self.assertNotIn("signatures", actual["packages"][0])
        self.assertEqual(actual["facets"], raw["facets"])
        self.assertEqual(actual["total_count"], 9)

    def test_security_normalizes_only_known_optional_collections(self):
        raw = {"image": {"Results": [{"Vulnerabilities": None, "custom": None}],
                         "Metadata": {"unknown": None}}}
        actual = security(raw)
        self.assertEqual(actual["image"]["Results"][0]["Vulnerabilities"], [])
        self.assertIsNone(actual["image"]["Results"][0]["custom"])
        self.assertEqual(actual["image"]["Metadata"], raw["image"]["Metadata"])
        self.assertEqual(security(None), {})

    def test_missing_output_and_wrong_http_errors_fail(self):
        for result in [{}, {"isError": False}, {"content": []}]:
            with self.subTest(result=result), self.assertRaises(AssertionError):
                result_value(result)
        for message in ["something contains 404", "API error 400 Bad Request: 404", "API error 4040: wrong"]:
            with self.subTest(message=message), self.assertRaises(AssertionError):
                result_value({"isError": True, "content": [{"type": "text", "text": message}]}, 404)
        result_value({"isError": True, "content": [{"type": "text", "text": "API error 404 Not Found: "}]}, 404)

    def test_plain_template_source_is_not_json_decoded(self):
        source = '{{ include "chart.name" . }}\n'
        self.assertEqual(result_value({"content": [{"type": "text", "text": source}]}), source)

    def test_outage_while_checking_expected_error_is_unavailable(self):
        with self.assertRaises(Unavailable):
            result_value({"isError": True, "content": [
                {"type": "text", "text": "API error 503 Service Unavailable: "}
            ]}, expected_status=404)


class SweepTests(unittest.TestCase):
    def run_sweep(self, result, reference, advertised=None):
        sweep = Sweep(Mock(call=Mock(return_value=result)))
        with contextlib.redirect_stdout(io.StringIO()):
            sweep.run("get_package", {}, reference)
            code = sweep.finish(advertised or {"get_package"})
        return sweep, code

    def test_matching_data_is_the_only_success(self):
        sweep, code = self.run_sweep({"structuredContent": {"version": "1.2.0"}}, lambda: {"version": "1.2.0"})
        self.assertEqual(code, 0)
        self.assertEqual(sweep.validated, {"get_package"})

    def test_mutated_response_fails_with_nonzero_exit(self):
        for actual in [{}, {"version": "latest"}]:
            with self.subTest(actual=actual):
                sweep, code = self.run_sweep({"structuredContent": actual}, lambda: {"version": "1.2.0"})
                self.assertEqual(code, 1)
                self.assertEqual(sweep.failures, 1)
                self.assertEqual(sweep.validated, set())

    def test_unhandled_advertised_tool_fails(self):
        _, code = self.run_sweep({"structuredContent": {}}, lambda: {}, {"get_package", "new_tool"})
        self.assertEqual(code, 1)

    def test_upstream_outage_is_unavailable_not_pass_or_mismatch(self):
        reference = Mock(side_effect=Unavailable("Hub HTTP 503"))
        sweep, code = self.run_sweep({}, reference)
        self.assertEqual(code, 2)
        self.assertEqual((sweep.passed, sweep.failures, sweep.unavailable), (0, 0, 1))
        sweep.mcp.call.assert_not_called()

    def test_client_transport_error_is_also_unverified(self):
        sweep, code = self.run_sweep({"isError": True, "content": [
            {"type": "text", "text": "Request failed: timed out"}
        ]}, lambda: {})
        self.assertEqual(code, 2)
        self.assertEqual(sweep.validated, set())

    def test_changing_reference_is_not_counted_as_passing(self):
        reference = Mock(side_effect=[{"stars": 1}, {"stars": 2}])
        sweep, code = self.run_sweep({"structuredContent": {"stars": 3}}, reference)
        self.assertEqual(code, 2)
        self.assertEqual(sweep.passed, 0)

    def test_confirmed_404_does_not_replace_successful_tool_coverage(self):
        reference = Mock(side_effect=urllib.error.HTTPError("url", 404, "Not Found", {}, None))
        sweep, code = self.run_sweep({"isError": True, "content": [
            {"type": "text", "text": "API error 404 Not Found: "}
        ]}, reference)
        self.assertEqual(sweep.passed, 1)
        self.assertEqual(sweep.validated, set())
        self.assertEqual(code, 2)


if __name__ == "__main__":
    unittest.main()
