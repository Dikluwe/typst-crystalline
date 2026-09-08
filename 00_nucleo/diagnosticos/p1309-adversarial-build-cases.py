"""Construct 18 concrete JSON-patch attacks from the sealed P1309 bundle.

Only stdout is written. No auditor logic or product mutation lives here.
"""
import base64
import copy
import hashlib
import json
from pathlib import Path

D = Path(__file__).resolve().parent
B = json.loads((D / "p1309-audit-bundle.json").read_text())
CASES = []


def replace(path, value):
    return {"op": "replace", "path": path, "value": value}


def remove(path):
    return {"op": "remove", "path": path}


def test(path, value):
    return {"op": "test", "path": path, "value": value}


def index(rows, predicate):
    return next(i for i, row in enumerate(rows) if predicate(row))


def add(identifier, scope, description, codes, witness, operations):
    CASES.append({"id": identifier, "description": description,
                  "target_codes": codes, "witness": witness,
                  "control_operations": [replace("/scope", scope)],
                  "operations": operations})


def channel(observation, name, text):
    value = copy.deepcopy(observation)
    raw = text.encode()
    value[name] = text
    value[name + "_base64"] = base64.b64encode(raw).decode()
    value[name + "_sha256"] = hashlib.sha256(raw).hexdigest()
    if name == "stderr" and "diagnostics" in value:
        value["diagnostics"]["raw_stderr"] = text
    if name == "stdout":
        decoded = json.loads(text)
        if "parsed_value" in value:
            value["parsed_value"] = decoded
        if "observable" in value and value["observable"].get("kind") == "value":
            value["observable"]["value"] = decoded
    return value


probes = B["catalog"]["probes"]
historical = {p["id"] for p in B["anchors"]["historical_probes"]}
i = index(probes, lambda p: p["id"] == "path-angle.deg")
add("A01", "catalog", "Omitir probe histórico angle.deg", ["HISTORICAL_PROBE_CHANGED"],
    {"historical_id": probes[i]["id"], "probe": probes[i]},
    [test(f"/catalog/probes/{i}/id", probes[i]["id"]), remove(f"/catalog/probes/{i}")])

i = index(probes, lambda p: p["id"] not in historical and p["path"] in B["anchors"]["inventory_paths"])
add("A02", "catalog", "Omitir rota reenumerada ausente do catálogo histórico", ["DISCOVERED_MEMBER_OMITTED"],
    {"fresh_discovery": probes[i], "historical_id_absent": True}, [remove(f"/catalog/probes/{i}")])

i = index(probes, lambda p: p["path"] == "angle.rad")
add("A03", "catalog", "Fundir por semelhança nominal angle.rad em angle.deg mantendo apenas uma rota", ["HISTORICAL_PROBE_CHANGED", "DISCOVERED_MEMBER_OMITTED"],
    {"distinct_paths": ["angle.deg", "angle.rad"], "original_probe": probes[i]},
    [replace(f"/catalog/probes/{i}/path", "angle.deg"), replace(f"/catalog/probes/{i}/expression", "repr((type(angle.deg), repr(angle.deg)))")])

rows = B["matrices"]["normal"]
i = index(rows, lambda r: r["id"] == "path-angle.deg" and r["profile"] == "default")
r = rows[i]
add("A04", "matrix", "Trocar envelopes vanilla/cristalino de célula real", ["BINARY_SIDE_IDENTITY"],
    {"cell": [r["id"], r["profile"]], "binaries": B["anchors"]["binaries"]},
    [replace(f"/matrices/normal/{i}/vanilla", r["crystalline"]), replace(f"/matrices/normal/{i}/crystalline", r["vanilla"])])

old = json.loads((D / "p1308-r2-public-matrix.json").read_text())
old = json.loads(old["stdout"])
identity = {"path": old["binary"], "sha256": old["binary_sha256"]}
add("A05", "matrix", "Substituir recibo de candidato fresco pelo binário real P1308", ["FRESH_BUILD_PROVENANCE"],
    {"historical_receipt": "p1308-r2-public-matrix.json", "historical_binary": identity,
     "fresh_binary": B["build"]["candidate"]}, [replace("/build/candidate", identity)])

i = index(rows, lambda r: r["profile"] == "html")
o = copy.deepcopy(rows[i]["crystalline"])
o["features"] = ["a11y-extras"]
o["argv"][o["argv"].index("--features") + 1] = "a11y-extras"
add("A06", "matrix", "Executar perfil a11y sob rótulo HTML", ["PROFILE_ARGV_MISMATCH"],
    {"cell": [rows[i]["id"], "html"], "wrong_features": o["features"]},
    [replace(f"/matrices/normal/{i}/crystalline", o)])

i = index(B["sentinels"], lambda r: r["vanilla"]["stdout"] == r["crystalline"]["stdout"] and r["vanilla"]["stderr"] != r["crystalline"]["stderr"])
r = B["sentinels"][i]
o = channel(r["crystalline"], "stderr", r["vanilla"]["stderr"])
if "observable" in o:
    o["observable"] = copy.deepcopy(r["vanilla"].get("observable", o["observable"]))
add("A07", "sentinels", "Apagar divergência real de stderr mantendo stdout igual e hashes coerentes", ["SENTINEL_RAW_EVIDENCE_CHANGED"],
    {"sentinel": r["id"], "equal_stdout": r["vanilla"]["stdout"],
     "vanilla_stderr": r["vanilla"]["stderr"], "crystalline_stderr": r["crystalline"]["stderr"]},
    [replace(f"/sentinels/{i}/crystalline", o), replace(f"/sentinels/{i}/bilateral_class", "MATCH_DIAGNOSTIC")])

i = index(B["ledger"], lambda r: r["path"] == "angle.deg")
add("A08", "ledger", "VANILLA_ONLY tratado como feature desligada sem fonte", ["NORMATIVE_AUTHORITY_MISSING"],
    {"path": "angle.deg", "observed": B["ledger"][i]["runtime_class_by_profile"]},
    [replace(f"/ledger/{i}/current_language_class", "EXPECTED_FEATURE_DISABLED"), replace(f"/ledger/{i}/normative_evidence", "")])

i = index(B["ledger"], lambda r: r["path"] == "calc.deg")
add("A09", "ledger", "Aceitação cristalina calc.deg promovida a intenção sem cláusula L0", ["NORMATIVE_AUTHORITY_MISSING"],
    {"path": "calc.deg", "actual_class": B["ledger"][i]["current_language_class"], "normative_anchor": B["anchors"]["normative_facts"].get("calc.deg")},
    [replace(f"/ledger/{i}/current_language_class", "INTENTIONAL_PRODUCT_EXTENSION"), replace(f"/ledger/{i}/normative_evidence", "Apenas aceitação runtime cristalina.")])

drop = set(B["anchors"]["encoder_behaviour_witnesses"]["json.encode"])
indices = [i for i, r in enumerate(B["sentinels"]) if r["id"] in drop]
assert indices and "json.encode" in B["closed_encoder_paths"]
add("A10", "sentinels", "Manter json.encode fechado e sua presença global apagando testemunhas comportamentais", ["ENCODER_CLOSED_BY_PRESENCE_ONLY"],
    {"route": "json.encode", "presence_probe": next(p for p in probes if p["id"] == "path-json.encode"), "removed_behaviour_ids": sorted(drop)},
    [remove(f"/sentinels/{i}") for i in reversed(indices)])

a = next(a for a in B["array_integrity"] if a["length"] == 256)
i = index(B["sentinels"], lambda r: r["id"] == a["sentinel_id"])
r = B["sentinels"][i]
value = json.loads(r["crystalline"]["stdout"])
before = copy.deepcopy(value)
value[1][128] = -999
o = channel(r["crystalline"], "stdout", json.dumps(value, separators=(",", ":")) + "\n")
add("A11", "sentinels", "Corromper dado integral do array mantendo comprimento e repr elidida", ["REPR_DATA_LOSS"],
    {"sentinel": r["id"], "length": 256, "index": 128, "old_value": before[1][128], "new_value": -999, "repr_preserved": value[2:] == before[2:]},
    [replace(f"/sentinels/{i}/crystalline", o)])

i = index(B["transitions"], lambda t: t["previous_runtime_class"] == "MATCH_VALUE" and t["current_runtime_class"] == "MATCH_VALUE")
t = B["transitions"][i]
add("A12", "ledger", "Ocultar transição contrafactual de regressão histórica por renome de ID", ["HISTORICAL_TRANSITION_OMITTED"],
    {"historical_cell": t, "mutation_kind": "Inject DIFFERENT_VALUE into real MATCH predecessor and rename its ID; no product regression is asserted."},
    [replace(f"/transitions/{i}/current_runtime_class", "DIFFERENT_VALUE"), replace(f"/transitions/{i}/probe_id", "p1309-hidden-regression-renamed"), replace(f"/transitions/{i}/transition", "NEW_DISCOVERY")])

i = index(rows, lambda r: r["runtime_class"] == "MATCH_VALUE")
o = copy.deepcopy(rows[i]["crystalline"])
o.update(complete=False, reason_code="TIMEOUT", exit_code=None)
add("A13", "matrix", "Converter observação timeout em MATCH preservando rótulo de sucesso", ["EXECUTION_UNKNOWN", "RUNTIME_CLASS_MISMATCH"],
    {"cell": [rows[i]["id"], rows[i]["profile"]], "injected_opacity": "TIMEOUT", "original_runtime_class": rows[i]["runtime_class"]},
    [replace(f"/matrices/normal/{i}/crystalline", o)])

add("A14", "catalog", "Inflar catálogo principal com cardinalidade do suplemento", ["PRINCIPAL_DENOMINATOR_INFLATED"],
    {"principal": B["principal_probe_count"], "supplement_rows": len(B["supplement_ids"])},
    [replace("/principal_probe_count", B["principal_probe_count"] + len(B["supplement_ids"]))])

debt = json.loads((D / "p1309-certification-debt.json").read_text())
add("A15", "ledger", "Contar dívida adversarial pendente como falha funcional sem witness bilateral", ["CERTIFICATION_AS_LANGUAGE_FAILURE"],
    {"debt_receipt": "p1309-certification-debt.json", "sha256": hashlib.sha256((D / "p1309-certification-debt.json").read_bytes()).hexdigest(), "functional_failure_count_before": B["functional_failure_count"], "injected_debt_failure": 1},
    [replace("/functional_failure_count", B["functional_failure_count"] + 1)])

i = index(B["selection"]["cohorts"], lambda c: c["eligible"] and c["priority"] > 3)
c = copy.deepcopy(B["selection"]["cohorts"][i])
old_priority = c["priority"]
c["priority"] = 1
c["rank_key"][0] = 1
add("A16", "selection", "Eleger coorte inferior falsificando coerentemente prioridade e rank_key", ["COHORT_PRIORITY_INVALID"],
    {"correct_winner": B["selection"]["selected_cohort"], "inferior_cohort": c["id"], "real_priority": old_priority, "falsified_priority": 1},
    [replace(f"/selection/cohorts/{i}", c), replace("/selection/selected_cohort", c["id"])])

owner, consumers = next(iter(B["ownership"].items()))
second = next(v[0] for k, v in B["ownership"].items() if k != owner and len(v) == 1)
pointer = owner.replace("~", "~0").replace("/", "~1")
add("A17", "integrity", "Declarar Prompt L0 com dois consumers produtivos como íntegro", ["PROMPT_OWNERSHIP_INVALID"],
    {"owner": owner, "original_consumer": consumers, "second_real_consumer": second},
    [replace("/ownership/" + pointer, consumers + [second])])

i = index(B["matrices"]["reverse"], lambda r: r["runtime_class"] == "MATCH_VALUE")
r = B["matrices"]["reverse"][i]
o = channel(r["crystalline"], "stdout", json.dumps({"p1309_order_injected": True}) + "\n")
add("A18", "matrix", "Alterar saída válida somente na ordem invertida", ["ORDER_INSTABILITY"],
    {"cell": [r["id"], r["profile"]], "phase_changed": "reverse", "normal_and_repeat": "untouched"},
    [replace(f"/matrices/reverse/{i}/crystalline", o), replace(f"/matrices/reverse/{i}/runtime_class", "DIFFERENT_VALUE")])

assert len(CASES) == 18
print(json.dumps({"schema": "p1309-adversarial-cases-v1", "bundle_sha256": hashlib.sha256((D / "p1309-audit-bundle.json").read_bytes()).hexdigest(), "cases": CASES}, ensure_ascii=False, indent=2))
