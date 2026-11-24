"""Count actual page deflate misses on the exact stripped vanilla watch ELF."""
import json
import os
import subprocess

from sample import D, EXES, STATES, WORK, cycle, measure

exe = EXES["vanilla"]
assert measure.sha(exe) == "e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b"
symbols = {
    "watch_wait": [0x25e5440],
    "vanilla_page_filter": [0x1e11ae0],
    "vanilla_deflate": [0x1e118c0],
    "tag_resolve": [0x2c899f0],
}
os.sched_setaffinity(0, {2})
for block in range(8):
    case = WORK / f"vanilla-{block:02d}"
    (case / "edits").mkdir(parents=True)
    for number, (_, source, _) in enumerate(STATES):
        (case / ("main.typ" if number == 0 else f"edits/{number}.typ")).write_bytes(source)
    (case / "symbols.json").write_text(json.dumps(symbols))
    command = ["taskset", "-c", "1", "gdb", "-q", "-nx", "-batch", "-x", str(D / "vanilla-scopes.gdb"),
               "--args", str(exe), "watch", "main.typ", "out.pdf", "--root", str(case), "--pdf-tagged=true"]
    with (case / "gdb.log").open("wb") as log:
        subprocess.run(command, cwd=case, env=cycle.environment(case, "vanilla", False),
                       stdout=log, stderr=subprocess.STDOUT, timeout=40, check=True)
    for number, (state, _, _) in enumerate(STATES):
        assert cycle.cycle.key("vanilla", case / f"publication-{number}.pdf") == cycle.gold("vanilla", state)
    scopes = json.loads((case / "scopes.json").read_text())
    print(json.dumps(dict(block=block, utc=measure.utc(), nanoseconds=scopes)), flush=True)
