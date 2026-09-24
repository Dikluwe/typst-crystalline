import json
import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

import merge


def side(
    *,
    present=True,
    kind="function",
    params=(),
    owner_path=None,
    owner_kind=None,
    slot_kind="global_binding",
    access_form="not_applicable",
    structural_verified=True,
    symbol=None,
):
    return {
        "present": present,
        "availability": "active" if present else "absent",
        "kind": kind if present else None,
        "params": list(params) if params is not None else None,
        "source": "fixture",
        "owner_path": owner_path,
        "owner_kind": owner_kind,
        "slot_kind": slot_kind,
        "access_form": access_form,
        "structural_verified": structural_verified,
        "symbol": symbol,
    }


class ClassificationTests(unittest.TestCase):
    def test_missing_global_is_binding(self):
        got = merge.classify_entry("default", "foo", side(), merge.absent_side())
        self.assertEqual(got["classification"], "MISSING_BINDING")

    def test_dot_does_not_make_member_without_compatible_owner(self):
        vanilla = side(
            owner_path="foo",
            owner_kind="module",
            slot_kind="member",
            access_form="module_member",
        )
        got = merge.classify_entry(
            "default", "foo.bar", vanilla, merge.absent_side(), owners={}
        )
        self.assertEqual(got["classification"], "UNKNOWN")
        self.assertEqual(got["unknown_reason"], "blocked_by_ancestor")

    def test_missing_member_requires_compatible_owner(self):
        member = side(
            owner_path="foo",
            owner_kind="module",
            slot_kind="member",
            access_form="module_member",
        )
        owners = {"foo": (side(kind="module"), side(kind="module"))}
        got = merge.classify_entry(
            "default", "foo.bar", member, merge.absent_side(), owners=owners
        )
        self.assertEqual(got["classification"], "MISSING_MEMBER")

    def test_wrong_kind_is_not_match(self):
        got = merge.classify_entry(
            "default", "foo", side(kind="function"), side(kind="module")
        )
        self.assertEqual(got["classification"], "WRONG_KIND")

    def test_null_params_are_unverified_metadata(self):
        got = merge.classify_entry(
            "default", "foo", side(params=[]), side(params=None)
        )
        self.assertEqual(got["classification"], "UNVERIFIED_METADATA")
        self.assertEqual(got["metadata_state"], "MISSING_OBSERVATION")

    def test_both_absent_are_never_match(self):
        got = merge.classify_entry(
            "default", "foo", merge.absent_side(), merge.absent_side()
        )
        self.assertEqual(got["classification"], "UNKNOWN")

    def test_symbol_metadata_difference_is_verified_not_unverified(self):
        vanilla = side(
            kind="symbol",
            params=None,
            symbol={"value": "→", "variants": [["r", "→"]]},
        )
        crystalline = side(
            kind="symbol",
            params=None,
            symbol={"value": "←", "variants": [["l", "←"]]},
        )
        got = merge.classify_entry("default", "sym.arrow", vanilla, crystalline)
        self.assertEqual(got["classification"], "VERIFIED_METADATA_DIFFERENCE")
        self.assertEqual(got["metadata_state"], "VERIFIED_DIFFERENCE")

    def test_function_parameter_difference_is_verified_not_unverified(self):
        got = merge.classify_entry(
            "default", "foo", side(params=["first"]), side(params=["second"])
        )
        self.assertEqual(got["classification"], "VERIFIED_METADATA_DIFFERENCE")
        self.assertEqual(got["metadata_state"], "VERIFIED_DIFFERENCE")

    def test_extra_keeps_member_subtype(self):
        crystalline = side(
            owner_path="foo",
            owner_kind="module",
            slot_kind="member",
            access_form="module_member",
        )
        got = merge.classify_entry(
            "default", "foo.extra", merge.absent_side(), crystalline
        )
        self.assertEqual(got["classification"], "EXTRA_BINDING")
        self.assertEqual(got["extra_subtype"], "member")

    def test_unverified_access_form_does_not_match(self):
        vanilla = side(
            owner_path="array",
            owner_kind="type",
            slot_kind="member",
            access_form="instance_method",
        )
        crystalline = side(
            owner_path="array",
            owner_kind="type",
            slot_kind="member",
            access_form="instance_method",
            structural_verified=False,
        )
        got = merge.classify_entry("default", "array.all", vanilla, crystalline)
        self.assertEqual(got["classification"], "UNVERIFIED_METADATA")


class ProfileAndDeterminismTests(unittest.TestCase):
    def catalog(self, side_name, *, profile="default", features=None):
        if features is None:
            features = merge.PROFILE_FEATURES[profile]
        return {
            "schema_version": "p1282-catalog-v1",
            "side": side_name,
            "profile": profile,
            "features": features,
            "vanilla_revision": "9f2b6e87" if side_name == "vanilla" else None,
            "entries": {"same": side(kind="module", params=None)},
        }

    def test_html_must_be_symmetric(self):
        with self.assertRaisesRegex(ValueError, "feature set must be symmetric"):
            merge.validate_profiles(
                {"profile": "html", "features": ["html"]},
                {"profile": "html", "features": []},
            )

    def test_bundle_profiles_have_exact_provenance_features(self):
        for profile in ("bundle", "html+bundle", "a11y-extras", "html+a11y"):
            with self.subTest(profile=profile):
                payload = merge.merge_catalog_payloads(
                    profile,
                    self.catalog("vanilla", profile=profile),
                    self.catalog("crystalline", profile=profile),
                )
                self.assertEqual(payload["provenance_errors"], [])
                self.assertEqual(payload["features"], merge.PROFILE_FEATURES[profile])
                self.assertEqual(payload["counts"]["MATCH"], 1)

    def test_bundle_mismatched_feature_is_not_absence(self):
        payload = merge.merge_catalog_payloads(
            "bundle",
            self.catalog("vanilla", profile="bundle", features=[]),
            self.catalog("crystalline", profile="bundle"),
        )
        self.assertEqual(payload["counts"]["UNKNOWN"], 1)
        self.assertIn("vanilla feature set", payload["provenance_errors"])

    def test_default_html_is_disabled_not_missing(self):
        ledger = merge.feature_ledger(
            default_vanilla={},
            default_crystalline={},
            html_vanilla={"html": side(kind="module", params=None)},
            html_crystalline={"html": side(kind="module", params=None)},
        )
        self.assertEqual(ledger["html"]["default"], "disabled_by_profile")
        self.assertEqual(ledger["html"]["html"], "active_bilateral")

    def test_reordering_produces_same_canonical_json(self):
        first = {"b": side(kind="module", params=None), "a": side(params=[])}
        second = dict(reversed(list(first.items())))
        payload_a = merge.merge_catalogs("default", first, first)
        payload_b = merge.merge_catalogs("default", second, second)
        self.assertEqual(
            json.dumps(payload_a, sort_keys=True),
            json.dumps(payload_b, sort_keys=True),
        )

    def test_blocked_descendants_are_ledgered_not_multiplied(self):
        vanilla = {
            "owner": side(kind="module", params=None),
            "owner.child": side(
                owner_path="owner",
                owner_kind="module",
                slot_kind="member",
                access_form="module_member",
            ),
        }
        payload = merge.merge_catalogs("default", vanilla, {})
        self.assertEqual(payload["counts"]["MISSING_BINDING"], 1)
        self.assertEqual(payload["blocked_by_ancestor_count"], 1)
        self.assertEqual(len(payload["entries"]), 1)

    def test_catalog_profile_mismatch_becomes_unknown(self):
        payload = merge.merge_catalog_payloads(
            "default",
            self.catalog("vanilla", profile="html"),
            self.catalog("crystalline"),
        )
        self.assertEqual(payload["counts"]["UNKNOWN"], 1)
        self.assertEqual(payload["entries"][0]["unknown_reason"], "invalid_provenance")
        self.assertEqual(payload["counts"]["VERIFIED_METADATA_DIFFERENCE"], 0)

    def test_catalog_feature_mismatch_becomes_unknown(self):
        payload = merge.merge_catalog_payloads(
            "html",
            self.catalog("vanilla", profile="html", features=[]),
            self.catalog("crystalline", profile="html"),
        )
        self.assertEqual(payload["counts"]["UNKNOWN"], 1)

    def test_enumerator_unknown_is_never_match(self):
        truncated = side(present=False, kind=None, params=None)
        truncated["availability"] = "unknown"
        got = merge.merge_catalogs(
            "default",
            {"deep.__p1282_scope_truncated__": truncated},
            {"deep.__p1282_scope_truncated__": truncated},
        )
        self.assertEqual(got["counts"]["UNKNOWN"], 1)

    def test_verified_difference_and_missing_observation_have_separate_counts(self):
        vanilla = {
            "different": side(
                kind="symbol",
                params=None,
                symbol={"value": "→", "variants": [["r", "→"]]},
            ),
            "unobserved": side(params=[]),
        }
        crystalline = {
            "different": side(
                kind="symbol",
                params=None,
                symbol={"value": "←", "variants": [["l", "←"]]},
            ),
            "unobserved": side(params=None),
        }
        got = merge.merge_catalogs("default", vanilla, crystalline)
        self.assertEqual(got["schema_version"], "p1282-v2")
        self.assertEqual(got["counts"]["VERIFIED_METADATA_DIFFERENCE"], 1)
        self.assertEqual(got["counts"]["UNVERIFIED_METADATA"], 1)


if __name__ == "__main__":
    unittest.main()
