# Reproduce in a fresh evidence directory

Preserve this recorded dataset. Do not rerun its collector in place: it refuses an
existing round-receipts.json. Use a separate repository checkout at the recorded HEAD,
overlay measured-sources.tar.gz, and restore the recorded dirty inputs. Keep only the
helper sources (Cargo.toml, Cargo.lock, src/main.rs, main.typ, run.py, protocol.md) in
that checkout's diagnostic directory before collecting new evidence. Retain this
original dataset elsewhere; never delete or overwrite it. Dependencies/toolchain
must be provisioned beforehand for the locked offline build.

From that repository root:

```sh
RUSTFLAGS='' CARGO_ENCODED_RUSTFLAGS='' cargo build --release --offline --locked \
  --manifest-path 00_nucleo/diagnosticos/place-metrics-microbenchmarks/Cargo.toml \
  --target-dir target -j 1 --message-format=json \
  > 00_nucleo/diagnosticos/place-metrics-microbenchmarks/build-final.jsonl \
  2> 00_nucleo/diagnosticos/place-metrics-microbenchmarks/build-final.stderr
PYTHONDONTWRITEBYTECODE=1 python3 \
  00_nucleo/diagnosticos/place-metrics-microbenchmarks/run.py \
  > 00_nucleo/diagnosticos/place-metrics-microbenchmarks/run.stdout \
  2> 00_nucleo/diagnosticos/place-metrics-microbenchmarks/run.stderr
```

Do not run tests, builds, or other benchmarks during collection. Capture fresh HEAD,
exact git status --porcelain=v1 --untracked-files=all, git diff HEAD --stat, UTC time,
rustc -Vv, cargo -V, uname -a, and lscpu before the run, as in provenance.json.
The collector checks the executable profile, performs untimed smoke checks, archives
the executable and source overlay, then runs normal/reversed/normal case ordering.
Changed input fingerprints or known competing processes reject the block. Failed
or unstable results must be preserved, not replaced with selectively favorable runs.

Evidence: artifact.json (profile, build recipe, executable hash); provenance.json
(machine, toolchain, HEAD, exact dirty inventory); before-sources.json plus initial,
per-round and final fingerprints; measured-sources.tar.gz; bin/microbench; smoke and
round receipts/stdout/stderr; round*/setup.json (embedded face hashes); Criterion
round*/**/new/sample.json and estimates.json; process-monitor.json; summary.json
(normalized distributions/admission); completion-audit.json; MANIFEST.sha256.

The report is outside this directory; its SHA-256 is recorded in completion-audit.json.
Verify the manifest from this directory with sha256sum -c MANIFEST.sha256. A successful
reproduction requires matching fixtures, sources, fonts and timing policy; host noise
and different toolchains remain limitations.
