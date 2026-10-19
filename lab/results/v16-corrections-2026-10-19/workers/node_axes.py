"""Compile copied real workspaces to isolate both axes of V16-082.

Run through the resource-constrained coordinator. This worker never changes
productive sources, contracts, enums, dependencies, or the existing workers.
"""

from pathlib import Path
import datetime
import gzip
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile


ROOT = Path.cwd().resolve()
EVIDENCE = ROOT / "lab/results/v16-corrections-2026-10-19/batch-a"
OUTPUT = EVIDENCE / "node-axis-proof"
SOURCE = "01_core/src/entities/syntax_node/representation.rs"
LAYERS = ("01_core", "02_shell", "03_infra", "04_wiring")
RECIPE_FILES = (
    "Cargo.toml", "Cargo.lock", "clippy.toml", "rustfmt.toml",
    "rust-toolchain.toml", ".cargo/config.toml",
)
PROBE = "TektV16EvolutionProbe"
OLD_VARIANTS = ("Leaf", "Inner", "Error")
SCENARIOS = {
    # Cover the entire opposite axis, including new/new.
    "new-old": f"            (_, NodeKind::{PROBE}) => false,\n",
    "old-new": f"            (NodeKind::{PROBE}, _) => false,\n",
    # Cover only the six mixed pairs, leaving exactly new/new.
    "new-new": "".join(
        f"            (NodeKind::{PROBE}, NodeKind::{old}(_)) => false,\n"
        f"            (NodeKind::{old}(_), NodeKind::{PROBE}) => false,\n"
        for old in OLD_VARIANTS
    ),
}


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def write_gzip(path, value):
    with gzip.open(path, "wt") as stream:
        json.dump(value, stream)


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def snapshot(base, owners):
    paths = {
        str(path.relative_to(base))
        for layer in LAYERS
        for path in (base / layer).rglob("*.rs")
    }
    paths.update(
        str(path.relative_to(base))
        for layer in LAYERS
        for path in (base / layer).rglob("Cargo.toml")
    )
    paths.update(RECIPE_FILES)
    paths.update(path for path in owners if (base / path).is_file())
    return {path: digest((base / path).read_bytes()) for path in sorted(paths)}


def changed_since(base, frozen):
    return [
        path for path, expected in frozen.items()
        if not (base / path).is_file()
        or digest((base / path).read_bytes()) != expected
    ]


def locate_match(source):
    function = source.index("pub fn spanless_eq(&self, other: &Self) -> bool {")
    anchor = "match (&self.0, &other.0) {"
    start = source.index(anchor, function)
    return {
        "file": SOURCE,
        "function": "SyntaxNode::spanless_eq",
        "match_line": source[:start].count("\n") + 1,
        "match_column": start - source.rfind("\n", 0, start),
        "source_anchor": anchor,
        "byte_start": len(source[:start].encode()),
        "byte_end": len(source[:start + len(anchor)].encode()),
    }


def mutate(original, scenario):
    assert PROBE not in original, "sentinel already present in source input"
    enum_start = original.index("enum NodeKind {")
    enum_end = original.index("\n}", enum_start)
    declaration = original[enum_start:enum_end]
    variants = re.findall(r"^    (\w+)\(", declaration, flags=re.M)
    assert tuple(variants) == OLD_VARIANTS, variants
    extended = original[:enum_end] + f"\n    {PROBE}," + original[enum_end:]
    target = locate_match(extended)
    anchor = target["source_anchor"]
    start = extended.index(anchor, extended.index("pub fn spanless_eq("))
    insertion = start + len(anchor)
    assert extended[insertion] == "\n"
    mutated = extended[:insertion + 1] + SCENARIOS[scenario] + extended[insertion + 1:]
    return mutated, locate_match(mutated)


def selected_diagnostic(diagnostic, target, stage):
    if (diagnostic.get("code") or {}).get("code") != "E0004":
        return False
    for span in diagnostic["spans"]:
        path = span["file_name"].removeprefix(str(stage) + "/")
        if (
            span["is_primary"] and path == SOURCE
            and span["line_start"] == target["match_line"]
            and target["byte_start"] <= span["byte_start"] < target["byte_end"]
        ):
            return True
    return False


def witness_pairs(diagnostic):
    """Read compiler witnesses, never help suggestions or unrelated diagnostics."""
    patterns = re.findall(r"`([^`]+)`", diagnostic["message"])
    pairs = []
    for pattern in patterns:
        if not (pattern.startswith("(") and pattern.endswith(")")):
            continue
        slots = pattern[1:-1].split(", ")
        if len(slots) != 2:
            continue
        pair = []
        for slot in slots:
            found = re.fullmatch(r"&?(?:[\w]+::)*NodeKind::(\w+)(?:\(_\))?", slot)
            pair.append(found.group(1) if found else slot)
        pairs.append(pair)
    return pairs


def expected_witness(pair, scenario):
    left, right = pair
    if scenario == "new-old":
        return left == PROBE and right in OLD_VARIANTS
    if scenario == "old-new":
        return left in OLD_VARIANTS and right == PROBE
    return left == PROBE and right == PROBE


def run_phase(phase):
    base = ROOT / "lab/scratch/v16-batch-a-base-2026-10-19" if phase == "baseline" else ROOT
    out = OUTPUT / phase
    out.mkdir(parents=True, exist_ok=True)
    assert not (out / "results.json").exists(), "preserve prior evidence; choose a new output before rerunning"
    scope = json.loads((EVIDENCE / "scope.json").read_text())
    frozen = snapshot(base, scope["owners"])
    start = utc()
    stage_parent = ROOT / "lab/scratch/v16-node-axis-proof-2026-10-19"
    stage_parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=phase + "-", dir=stage_parent))
    for top in (*LAYERS, "lab/benches"):
        shutil.copytree(base / top, stage / top)
    for path in RECIPE_FILES:
        destination = stage / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(base / path, destination)
    originals = {
        str(path.relative_to(stage)): path.read_text()
        for layer in LAYERS for path in (stage / layer).rglob("*.rs")
    }
    assert all(PROBE not in text for text in originals.values()), "sentinel in productive inputs"
    assert all(digest(text.encode()) == frozen[path] for path, text in originals.items())
    recipes = {
        path: (stage / path).read_text()
        for path in frozen if path.endswith("Cargo.toml") or path in RECIPE_FILES
    }
    common = {
        "phase": phase, "utc_start": start, "head": git("rev-parse", "HEAD"),
        "working_tree": "uncommitted working tree",
        "git_diff_head_stat": git("diff", "HEAD", "--stat"),
        "git_diff_head_name_status": git("diff", "HEAD", "--name-status"),
        "source_base": str(base), "cwd": str(stage), "frozen_sha256": frozen,
        "real_lock_sha256": frozen["Cargo.lock"],
        "worker_sha256": digest(Path(__file__).read_bytes()),
        "originals_file": "originals.json.gz",
        "resource_policy": "Run under coordinator: one Cargo job, 12 GiB max, 8 GiB high, zero swap",
    }
    write_gzip(out / "originals.json.gz", {"record": common, "sources": originals, "recipe_files": recipes})
    write_json(out / "freeze-before.json", common)
    records = []
    try:
        for scenario in SCENARIOS:
            mutated, target = mutate(originals[SOURCE], scenario)
            (stage / SOURCE).write_text(mutated)
            changed = [path for path, text in originals.items() if (stage / path).read_text() != text]
            assert changed == [SOURCE], changed
            assert all(digest((stage / path).read_bytes()) == frozen[path] for path in recipes)
            command = ["cargo", "check", "--offline", "--locked", "-p", "typst-core", "--message-format=json", "--lib"]
            record = {
                "id": "V16-082", "scenario": scenario, "phase": phase,
                "head": common["head"], "source_base": str(base), "cwd": str(stage),
                "utc_start": utc(), "command": command, "target": target,
                "original_sha256": digest(originals[SOURCE].encode()),
                "mutated_sha256": digest(mutated.encode()),
                "changed_paths": changed, "real_lock_sha256": frozen["Cargo.lock"],
            }
            write_gzip(out / (scenario + "-inputs.json.gz"), {
                "record": record, "original": originals[SOURCE], "mutated": mutated,
                "inserted_match_arms": SCENARIOS[scenario],
            })
            environment = dict(
                os.environ, CARGO_BUILD_JOBS="1", CARGO_PROFILE_DEV_DEBUG="0",
                CARGO_PROFILE_TEST_DEBUG="0", CARGO_INCREMENTAL="0",
                CARGO_TARGET_DIR=str(ROOT / "lab/target/product"),
            )
            print(utc(), phase, scenario, "START", flush=True)
            with (out / (scenario + ".log")).open("w") as log:
                result = subprocess.run(command, cwd=stage, env=environment, stdout=log, stderr=subprocess.STDOUT)
            errors = []
            for line in (out / (scenario + ".log")).read_text().splitlines():
                try:
                    item = json.loads(line)
                except ValueError:
                    continue
                if item.get("reason") == "compiler-message" and item["message"]["level"] == "error":
                    errors.append(item["message"])
            selected = [error for error in errors if selected_diagnostic(error, target, stage)]
            other = [error for error in errors if (error.get("code") or {}).get("code") != "E0004"]
            pairs = [pair for error in selected for pair in witness_pairs(error)]
            valid_witness = any(expected_witness(pair, scenario) for pair in pairs)
            wrong_witnesses = [pair for pair in pairs if not expected_witness(pair, scenario)]
            frozen_changes = changed_since(base, frozen)
            record.update(
                utc_end=utc(), exit=result.returncode, error_count=len(errors),
                selected_e0004=selected, other_errors=other,
                unselected_e0004=[error for error in errors if error not in selected and error not in other],
                compiler_witness_pairs=pairs, scenario_witness_verified=valid_witness,
                unexpected_witness_pairs=wrong_witnesses, source_freeze_changes=frozen_changes,
            )
            records.append(record)
            write_json(out / "results.json", records)
            print(utc(), phase, scenario, "END", result.returncode, "selected", len(selected), "witness", pairs, flush=True)
            assert not other, "non-E0004 failure is not structural evidence"
            assert not frozen_changes, "source inputs changed during proof"
            assert result.returncode == 101, "actual NodeKind extension must expose existing closed decisions"
            if phase == "baseline":
                assert not selected, "baseline selected match unexpectedly rejects extension"
            else:
                assert len(selected) == 1, "need one attributable selected-match E0004"
                assert valid_witness and not wrong_witnesses, "compiler witness does not isolate requested pair class"
    finally:
        (stage / SOURCE).write_text(originals[SOURCE])
        write_json(out / "freeze-after.json", {
            "utc": utc(), "source_base": str(base), "changed": changed_since(base, frozen),
            "copied_source_changed": [path for path, text in originals.items() if (stage / path).read_text() != text],
        })
    assert len(records) == 3
    print("PASS", phase, "three isolated NodeKind pair classes", flush=True)


if __name__ == "__main__":
    selected_phase = sys.argv[1] if len(sys.argv) == 2 else "all"
    assert selected_phase in ("baseline", "green", "all"), "usage: node_axes.py [baseline|green|all]"
    for phase in ("baseline", "green") if selected_phase == "all" else (selected_phase,):
        run_phase(phase)
