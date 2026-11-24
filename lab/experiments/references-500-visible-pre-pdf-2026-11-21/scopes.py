"""Bracket phase entries on the exact measured executables."""
import json
import os
from pathlib import Path
from random import Random
import subprocess

from sample import D, EXES, STATES, WORK, cycle, golden, measure

NATIVE_RESULT = D.parents[1] / "results/construction/diagnostics/references-500-visible-page-operator-cost-2026-11-21"
NAMES = {
    "watch_wait": "::watch::wait_for_change_since",
    "eval": "::eval_to_module_in_session",
    "paged": "::pipeline::paged::compile_paged_in_session",
    "compile_pdf": "::pipeline::pdf::compile_to_pdf_bytes_impl",
    "realize_context": "::context_stabilization::Session::realize",
    "substitute_context": "::pipeline::substitute_context_blocks",
    "physical_root": "::physical_memo::lab_retain",
    "memo_overhead": "::physical_memo::layout_with",
    "outline": "::prepare_outline_candidate",
    "source_plan": "::prepare_running_body",
    "index": "::index_physical_steps",
    "shape_document": "::shaper::traversal::shape_document_with_faces",
}
VANILLA_NAMES = {
    "watch_wait": "typst_kit::watcher::Watcher::wait",
    "eval": "typst_eval::eval",
    "compile_pdf": "typst::compile::compile_once",
    "layout_document": "typst_layout::pages::layout_document",
    "realize": "typst_realize::realize",
    "pdf": "typst_pdf::pdf",
}


def symbols(arm):
    if arm == "vanilla":
        assert (NATIVE_RESULT / "vanilla-symbols-verified").is_file()
        assert measure.sha(EXES[arm]) == "e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b"
        table = (NATIVE_RESULT / "vanilla-symbols-twin.txt").read_text()
        names = VANILLA_NAMES
    else:
        table = subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", str(EXES[arm])],
                                        text=True)
        names = NAMES
    entries = [line.split(maxsplit=3) for line in table.splitlines()]
    result = {kind: sorted({int(row[0], 16) for row in entries if len(row) == 4
                            and row[2] in ("t", "T") and row[3].split(".llvm.", 1)[0].endswith(name)})
              for kind, name in names.items()}
    assert all(result.values()), (arm, result)
    return result


def run(block, arm, addresses):
    case = WORK / "scopes" / f"{block:02d}-{arm}"
    (case / "edits").mkdir(parents=True)
    for number, (_, source, leaf) in enumerate(STATES):
        assert leaf is None
        (case / ("main.typ" if not number else f"edits/{number}.typ")).write_bytes(source)
    (case / "symbols.json").write_text(json.dumps(addresses))
    args = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch", "-x", str(D / "scopes.gdb"),
            "--args", str(EXES[arm]), "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
    with (case / "gdb.log").open("wb") as log:
        subprocess.run(args, cwd=case, env=cycle.environment(case, arm, False), stdout=log,
                       stderr=subprocess.STDOUT, timeout=60, check=True)
    for number, (name, _, _) in enumerate(STATES):
        assert cycle.cycle.key(arm, case / f"publication-{number}.pdf") == golden(arm, name), (arm, name)
    spans = json.loads((case / "scopes.json").read_text())
    assert "out_of_scope" not in spans, (arm, spans)
    return dict(block=block, arm=arm, utc=measure.utc(), spans=spans)


def main():
    os.sched_setaffinity(0, {2})
    addresses = {arm:symbols(arm) for arm in EXES}
    rng = Random(50089)
    with (WORK / "scopes.jsonl").open("w") as out:
        for block in range(int(os.environ.get("TEKT_SCOPE_REPEATS", "8"))):
            for arm in rng.sample(list(EXES), len(EXES)):
                print(json.dumps(run(block, arm, addresses[arm])), file=out, flush=True)
            print(f"scopes {block + 1}", flush=True)


if __name__ == "__main__":
    main()
