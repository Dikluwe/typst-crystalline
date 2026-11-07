# Independent CLI residual checks

These sources implement B's auxiliary contract without importing the new driver
or inspecting A's work. C runs them against the unchanged nominal builds.

```text
PYTHONDONTWRITEBYTECODE=1 python3 -B -m unittest discover -s E/tests -p test_check_cli.py
python3 -B E/tests/check_cli.py BATCH/measurements.csv \
  --variant bas --variant neu --variant laz --variant dup \
  --fixture no_import.typ --ram-dir BATCH \
  --reference bas --output NEW_SUMMARY.csv
```

Use the actual batch variant/fixture names. All rows must be CLI/cycle0 with
successful exit, independent PID/case, correct wait4 CPU, finite timestamps in
execution order and complete rectangular pairing. For each fixture/repeat every
variant covers every ordinal position with frequency difference at most one;
choose rounds divisible by variant count to simplify this control. Aliases of
one ELF remain separately named variants rather than pooled observations.

Snapshots remain under `/dev/shm/tekt-stdlib-cli-residual-20261106`. Validation
runs outside all clocks: it checks regular files, PDF header/EOF, marker0 and
the preserved output mtime within the process interval. This rejects an old
PDF with a matching marker; it supplements rather than replaces the driver's
new-case/publication checks. The independent oracle does not claim that a
marker alone proves freshness.

The paired analyzer averages repeats inside each round, then reports absolute
median CPU/wall ms, median paired delta ms, median paired percentage and paired
bootstrap95% intervals (10000 draws, seed0). No regression budget or binary
"acceptable" classification is invented. A/A is reported as a comparison of
distinct launches of the same ELF, not forced to zero or used to subtract an
estimated bias from the candidate.

`root_contract.rs` uses only public `typst_core` and `comemo` APIs. Compile with
`rustc --test --edition=2021`, matching release dependency artifacts selected
from that variant's compiler-artifact output, plus
`-L dependency=TARGET/release/deps --extern typst_core=CORE_RLIB
--extern comemo=COMEMO_RLIB`. It needs no ecow extern or cfg/private portal.
Run unchanged against bas/neu/laz. Four tests cover full 0/1/512/4096 input pairs,
Unicode >1MiB, ordered dictionary results, A/B/A root reevaluation, last pair,
fresh builtin/raw.line/With identities while retaining aliases inside a root,
one World.inputs query per root and Features/Paged/Html isolation. Resource
or import requests are explicit failures in this root-only World.

Directed checker mutations for C, only in RAM: omit the full wait4 CPU equality
guard; omit snapshot mtime validation; reverse the paired delta sign. All must
compile/import and fail by executed assertions in the unit tests. Report actual
killed/attempted, without treating compilation/import failure as a kill.

`test_shared_root.py` is a separate black-box transport test. C sets
`TEKT_CLI_SHARED_DRIVER=/absolute/path/to/completed/driver.py` only after A/B
finish, then discovers `test_*.py`. Six tests launch a fake compiler created only
inside task RAM. They inspect its captured argv/cwd and source hash/mtime across
launches: shared source/root/cwd, distinct output cases, one appended marker and
no source mutation; absence of the flag retains per-case input paths. Existing,
symlink and overlapping roots must be refused before any fake child launches.
The fake PDF is transport data, not a language/parity fixture. Without the env
variable these six tests skip explicitly; do not report skipped tests as passing
shared-root validation.
