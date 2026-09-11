#!/usr/bin/env python3
"""Validate P1341 R4 oracle authorability without candidate or product access."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


def load(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_sha(value: Any) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def walk_strings(value: Any):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for child in value.values():
            yield from walk_strings(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_strings(child)


def main() -> int:
    if len(sys.argv) != 5:
        print("usage: CHECKER CONTRACT FOCAL BINDINGS EXPECTATIONS", file=sys.stderr)
        return 2
    contract_path, focal_path, manifest_path, expectations_path = map(Path, sys.argv[1:])
    contract, focal, manifest, expectations = map(load, (contract_path, focal_path, manifest_path, expectations_path))
    errors: list[str] = []
    contract_sha, focal_sha = file_sha(contract_path), file_sha(focal_path)
    for name, artifact in (("manifest", manifest), ("expectations", expectations)):
        if artifact.get("status") != "AUTHORED_NOT_SEALED":
            errors.append(name + "-status")
        if artifact.get("contract", {}).get("sha256") != contract_sha:
            errors.append(name + "-contract-pin")
        if artifact.get("focal", {}).get("sha256") != focal_sha:
            errors.append(name + "-focal-pin")

    addressing = manifest.get("content_addressing", {})
    sections = addressing.get("covered_sections", [])
    expected_sections = ["bindings","event_cardinalities","runtime_identity_domains","ledger_types","lifecycle","counter_span","total_validation"]
    if sections != expected_sections:
        errors.append("content-section-coverage")
    payload = {key: manifest.get(key) for key in sections}
    content_sha = canonical_sha(payload)
    if addressing.get("content_sha256") != content_sha:
        errors.append("content-address")
    if expectations.get("binding_manifest", {}).get("content_sha256") != content_sha:
        errors.append("expectations-manifest-pin")

    contract_domains = contract.get("runtime_identity_domains", {})
    authored_domains = manifest.get("runtime_identity_domains", {})
    domain_names = ["runtime_id","func_id","carrier_id","value_id","location","snapshot_id","raw_span.source"]
    if set(authored_domains) != set(domain_names + ["global_non_alias"]):
        errors.append("domain-completeness")
    for domain in domain_names:
        source = contract_domains.get(domain, {})
        authored = authored_domains.get(domain, {})
        source_owners = source.get("roles", source.get("events"))
        if source_owners == "all objects":
            source_owners = "all objects"
        if authored.get("prefix") != source.get("prefix") or authored.get("owners") != source_owners or authored.get("unique") != source.get("unique"):
            errors.append("domain-drift:" + domain)
        if authored.get("json_type") != "string" or authored.get("non_empty") is not True:
            errors.append("domain-type:" + domain)
    if authored_domains.get("global_non_alias") != "No identity scalar may occur in more than one domain-and-owner slot.":
        errors.append("global-non-alias")

    if manifest.get("lifecycle") != contract.get("lifecycle"):
        errors.append("lifecycle-precedence")
    counter = manifest.get("counter_span", {})
    contract_counter = contract.get("counter_span", {})
    if counter.get("strict_keys") != contract_counter.get("strict_keys") or counter.get("source_coordinate") != contract_counter.get("source_coordinate"):
        errors.append("counter-span")
    total = manifest.get("total_validation", {})
    source_total = contract.get("total_validation", {})
    for key in ("accepted_scalar_identity_type", "strict_integer", "unknown_order"):
        if total.get(key) != source_total.get(key):
            errors.append("total-validation:" + key)

    ledger_types = manifest.get("ledger_types", {})
    required_ledger_types = {"top_level","cell_key","objects","events","raw_span","malformed_or_unsupported"}
    if set(ledger_types) != required_ledger_types:
        errors.append("ledger-type-completeness")
    if ledger_types.get("events", {}).get("required_fields", {}).get("seq") != "integer excluding boolean":
        errors.append("event-sequence-type")
    if ledger_types.get("raw_span", {}).get("exact_keys") != contract_counter.get("strict_keys"):
        errors.append("raw-span-exact-keys")

    r4 = expectations.get("r4_contract_expectation", {})
    if r4.get("runtime_identity_domains") is None or set(r4["runtime_identity_domains"]) != set(domain_names + ["global_non_alias"]):
        errors.append("expectation-domain-completeness")
    else:
        for domain in domain_names:
            if r4["runtime_identity_domains"][domain].get("prefix") != contract_domains[domain].get("prefix"):
                errors.append("expectation-domain-prefix:" + domain)
    if r4.get("lifecycle") != contract.get("lifecycle"):
        errors.append("expectation-lifecycle-precedence")
    if r4.get("validation_precedence") != source_total.get("unknown_order"):
        errors.append("expectation-validation-precedence")
    r4_counter = r4.get("counter_span", {})
    if r4_counter.get("strict_keys") != contract_counter.get("strict_keys") or r4_counter.get("source_coordinate") != contract_counter.get("source_coordinate"):
        errors.append("expectation-counter-span")
    required_type_keys = {"identity_scalar","strict_integer","cell_key","objects","events","event_seq","event_kind","event_subject","raw_span","raw_span_start_end","raw_span_lexical_role"}
    if set(r4.get("types", {})) != required_type_keys:
        errors.append("expectation-type-completeness")

    semantic = expectations.get("stable_semantic_expectation", {})
    inherited_sections = {"source_coordinates","required_roles","role_cardinalities","feature_witnesses","callbacks","counter_occurrences","dict_expectations","required_causal_role_edges"}
    if set(semantic) != inherited_sections:
        errors.append("inherited-semantic-completeness")
    roles = semantic.get("required_roles", [])
    if len(roles) != 9 or semantic.get("role_cardinalities") != {role: 1 for role in roles}:
        errors.append("inherited-role-cardinalities")

    cases = expectations.get("external_cases", [])
    if [case.get("expected_classification") for case in cases] != ["Preserved","Unknown","Violated"]:
        errors.append("case-class-vector")
    if len(cases) == 3 and cases[1].get("allowed_reason_code") != focal.get("controls", {}).get("opaque_reason_code"):
        errors.append("opaque-reason")

    gate = expectations.get("negative_gate_expectation", {})
    focal_gate = focal.get("required_gate", {})
    for authored_key, focal_key in (("protected","protected"),("r1_additional","r1_additional"),("r2_new","r2_new"),("combined","total")):
        if gate.get(authored_key) != focal_gate.get(focal_key):
            errors.append("gate-count:" + authored_key)
    if gate.get("required_classification") != focal_gate.get("classification") or gate.get("orders") != focal_gate.get("orders") or gate.get("required_score") != focal_gate.get("score"):
        errors.append("gate-policy")

    concrete = []
    prefixes = [contract_domains[name]["prefix"] for name in domain_names]
    for value in walk_strings(expectations):
        if any(value.startswith(prefix) and value != prefix for prefix in prefixes):
            concrete.append(value)
    if concrete:
        errors.append("concrete-runtime-identity")

    result = {
        "schema":"p1341-oracle-authorability-check-r3",
        "status":"AUTHORABLE_NOT_SEALED" if not errors else "NOT_AUTHORABLE",
        "pins":{"contract":contract_sha,"focal":focal_sha,"manifest_content":content_sha},
        "coverage":{"bindings":len(manifest.get("bindings", [])),"identity_domains":len(domain_names),"ledger_type_groups":len(ledger_types),"temporal_precedence_events":len(contract.get("lifecycle", {}).get("strict_precedence", [])),"validation_precedence_stages":len(source_total.get("unknown_order", [])),"external_cases":len(cases),"negative_cases":gate.get("combined")},
        "concrete_runtime_identity_values":len(concrete),
        "errors":errors,
        "limitations":["Authorability only; not a seal, candidate check, product verdict or reachability proof."],
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
