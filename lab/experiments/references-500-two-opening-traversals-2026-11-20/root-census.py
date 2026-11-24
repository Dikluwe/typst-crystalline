"""Count entries into the planned root in the exact nominal ELF files."""
import json
import os
from pathlib import Path
import subprocess
from bench import BASE, D, WORK, PATHS, ORIGINAL_ENV, base, digest

SUFFIXES = {
    "root": "::layout_planned_root",
    "memo": "::physical_memo::layout_with",
    "production": "::convergence::run_planned",
    "watch_wait": "::watch::wait_for_change_since",
}


def run():
    evidence = {}
    for arm in ("both", "omit", "same-off"):
        case = WORK / arm
        case.mkdir()
        (case / "main.typ").write_bytes(BASE)
        exe = PATHS[arm]
        counts = {}
        nm = subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", str(exe)], text=True)
        for kind, suffix in SUFFIXES.items():
            addresses = set()
            for line in nm.splitlines():
                fields = line.split(" ", 3)
                if len(fields) == 4 and fields[3].split(".llvm.", 1)[0].endswith(suffix):
                    addresses.add(int(fields[0], 16))
            assert addresses, (exe, suffix)
            counts[kind] = sorted(addresses)
        (case / "symbols.json").write_text(json.dumps(counts) + "\n")
        env = ORIGINAL_ENV(case)
        env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1",
                   TEKT_LAB_WATCH_INERT="1", TEKT_LAB_PDF_REUSE="both", TEKT_LAB_WATCH_TRACE="1",
                   TEKT_LAB_SKIP_PROVISIONAL_REPLAY="1")
        if arm == "omit":
            env["TEKT_LAB_SOURCE_ONLY_FIRST_INDEX"] = "1"
        cmd = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch",
               "-x", str(D / "root-census.gdb"), "--args", str(exe),
               "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
        with (case / "gdb.log").open("wb") as output:
            subprocess.run(cmd, cwd=case, env=env, stdout=output, stderr=subprocess.STDOUT,
                           timeout=90, check=True)
        expected = D.parent / "references-500-pdf-export-reuse-2026-11-20/fresh/regular/tekt/startup.pdf"
        assert base.qdf_hash(case / "out.pdf") == base.qdf_hash(expected)
        result = json.loads((case / "counts.json").read_text())
        evidence[arm] = dict(result, sha256=digest(exe), offsets=counts, command=cmd)
        print(arm, result, flush=True)
    (WORK / "root-census.json").write_text(json.dumps(evidence, indent=2) + "\n")


if __name__ == "__main__":
    run()
