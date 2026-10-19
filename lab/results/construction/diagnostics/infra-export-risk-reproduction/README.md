# Isolated infrastructure/export risk probes

This directory is diagnostic-only. It adds no productive Rust tests, changes no
L0, and does not propose or implement a repair. See the sibling report
`00_nucleo/diagnosticos/infra-export-risk-reproduction.md` for classifications.

From the repository root:

```sh
/home/dikluwe/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 \
  00_nucleo/diagnosticos/infra-export-risk-reproduction/run.py
```

The runner uses bundled Pillow/pypdf, local qpdf/Poppler/librsvg, Cargo/Rust and the
verified vanilla binary. Cargo runs offline, uses this directory's lockfile and
reuses root `target/`; it does not edit the root manifest or lockfile. The initial
standalone resolution requested an uncached dependency, so the repository lockfile
was copied here and Cargo resolved only this diagnostic package independently.
The saved diagnostic lockfile is the successful resolution, not a production
dependency change. These artifacts remain uncommitted; the lockfile does not imply a committed delivery.

`src/main.rs` calls current public APIs. `src/bin/icc_fault.rs` compiles the actual
unmodified ICC source under a diagnostic namespace with its compressor dependency
replaced by a controlled error. Its absolute source path identifies this checkout;
relocate that diagnostic-only attribute if the checkout moves. This is conditional
fault evidence, not a naturally reachable failure of the production compressor.

Fixtures and generated observations live in `results/`. PNG chunks have valid
CRCs; EXIF uses a valid TIFF rational. The package archive is served only by a
loopback server, with proxy variables removed for the download subprocess. The
race witness uses a controlled competing filesystem publisher after the real
download has passed its initial existence check; it is not two HTTP downloader
processes. Each cache is a fresh TemporaryDirectory and is removed after its
remaining-files inventory is saved. No public registry or existing user cache is
used. Generated fixtures/results are overwritten on rerun.

`commands.json` contains successful-run command arguments, timestamps, return
codes, stdout and stderr. `provenance-before.json` / `provenance-after.json` retain
HEAD, exact tracked/untracked status and content fingerprints outside this task.
`executables.json` records real paths and SHA-256; compiler/tool version commands
are in the command receipt. `artifact-sha256.json` fingerprints generated artifacts
and scripts at measurement time. The later README/report are handoff documents,
not inputs to those measurements. No line-count metric is collected.

The runner refuses vanilla comparisons if the binary SHA-256 does not match the
P1489 receipt for pin `9f2b6e8715237cb086899a42873660fe744622e8`.
PDF/PNG/SVG byte hashes are provenance, not generic parity comparisons. The actual
observations concern geometry, decoded format, extracted text, XML attributes,
MCID/ParentTree relationships, diagnostics and filesystem cleanup.
