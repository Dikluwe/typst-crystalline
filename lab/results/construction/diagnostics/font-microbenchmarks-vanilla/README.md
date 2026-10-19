# Actual pinned vanilla comparison

New diagnostic-only crate, measuring both current Tekt and actual vanilla in one executable. Read protocol.md before interpreting results. Four identical fonts, 96 cases, three rounds. There is no production, L0, ADR or existing benchmark edit.

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks-vanilla/stage.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks-vanilla/prepare.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks-vanilla/measure.py
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/font-microbenchmarks-vanilla/analyze.py
```

Staging verifies the ratified vanilla source record before copying byte-identically outside nested Cargo workspace ancestry. Only this diagnostic manifest is adjusted. Pin and tree are in staged-provenance.json; the ratified vanilla local Cargo.lock deviation is preserved explicitly. The new combined diagnostic Cargo.lock may resolve different dependencies than the ratified CLI. The CLI remains untouched; its hash is auxiliary, not the identity of the measured libraries.

prepare.py exercises the actual debug guard, rebuilds supported release, verifies fixture controls, and compiles all existing benchmarks without timing. measure.py admits only unchanged source/HEAD/executable rounds without concurrent build/test/benchmark processes and refuses to overwrite rounds. analyze.py preserves raw results and classifies predefined absolute stability failures; sequential ratios do not have paired causal meaning.

Full metadata/coverage and first acquisition are composed workloads. Public API differences and TTC reconstruction are explicit in protocol.md. Generic constructor, query, common collection and unmatched metadata cases are distinct. No aggregate CLI attribution or comparison against previous noisy absolute timings is valid.

Source/input archive and executable are retained independently of mutable Cargo target artifacts. External fonts require the exact bytes identified by inputs.json. See ../font-microbenchmarks-vanilla.md for observations and limitations.

Recorded dataset is sealed: reproduce in a fresh sibling diagnostic copy with no existing rounds/state/log outputs. Do not run writer scripts against sealed evidence. Restore vanilla-source/ from consumers-inputs.tar.gz to the recorded staged path for read-only verification if /tmp was removed. Run `PYTHONDONTWRITEBYTECODE=1 python3 verification/verify.py` for integrity checks. Final result: all three rounds admitted; 88/96 cases fail absolute stability. Treat costs and sequential ratios as exploratory.
