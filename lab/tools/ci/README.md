# Tekt CI and release

CI configures the pinned Rust cache action with `. -> lab/target/product` so
the cache follows the repository's Cargo target directory. The action's
[pinned configuration reference](https://github.com/Swatinem/rust-cache/blob/779680da715d629ac1d338a641029a2f4372abb5/README.md)
defines this workspace-to-target mapping. Architecture reports are also written
inside `lab/results`.

The workspace toolchain is pinned in `rust-toolchain.toml`. Helper scripts use
only the Python 3.11+ standard library.

## Prompt lineage and repository layout

CI installs `crystalline-lint` from published commit
`12797073972e375d9ab078cf6750c8b30b71e2b9` of
<https://github.com/Dikluwe/tekt-linter>. A local installation may be a different
version; use this commit to reproduce the CI gate.

```sh
cargo install --locked --git https://github.com/Dikluwe/tekt-linter --rev 12797073972e375d9ab078cf6750c8b30b71e2b9 crystalline-lint
python3 lab/tools/ci/check_mature_layout.py
python3 -m unittest discover -s lab/tools/ci
mkdir -p lab/results
crystalline-lint . --format sarif > lab/results/lineage.sarif
crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning
python3 lab/tools/ci/check_reverse_lineage.py
```

`check_mature_layout.py` checks the root/lab directory layout, active local
Markdown links, the relative `AGENTS.md` symlink, canonical 1:1 prompt ownership,
Rust source/dependency imports from product layers into `lab/`, the Cargo bench
workspace entry, and the configured product target directory. It accepts an
optional exact TSV ledger for known broken links:

```sh
python3 lab/tools/ci/check_mature_layout.py --known-broken-links <ledger.tsv>
```

The CI unit tests exercise this check and the other helper scripts. The linter
checks headers, direct seals, ownership, and Tekt nuclei. The reverse-lineage
helper checks the reverse seal for each consumer in all four layers, including
when its direct seal still matches. This additional check is needed for the
published linter version. It does not rewrite files or certify semantic
conformance to L0.

The full linter report is stored as `lab/results/lineage.sarif`. Warnings from other rules
remain visible; the four lineage codes above block even when the linter reports
them as warnings. Preserved construction measurements and diagnostic reports
are under `lab/results/construction/diagnostics/`; historical construction
records are under `lab/history/construction/records/00_nucleo/materialization/`.
Historical diagnostic records are not productive consumers; product code and
active prompts remain in the lineage gate.

## Build and validation

On Linux, install `poppler-utils`, `qpdf`, `librsvg2-bin`, and `imagemagick`.
Parity tests require `TYPST_VANILLA` to point to an oracle with verified
provenance. Do not use the old `/usr/local/bin/typst` installation. To use the
locally ratified binary from the 2026-10-17 resync:

```sh
echo '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9  lab/typst-original/target/release/typst' | sha256sum --check
export TYPST_VANILLA="$PWD/lab/typst-original/target/release/typst"
```

CI independently checks out upstream `typst/typst` at pinned commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. `build_oracle.py` verifies the
upstream commit and tree, checks that the source has no modifications before or
after the build, and writes the receipt to `lab/results/oracle-build.json`.
It builds with the lockfile, one job, and the workspace Rust toolchain. The
receipt records the recipe and executable SHA-256. The local resync report and
receipt are preserved at
`lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md`.
A rebuild may have a different SHA from the locally ratified binary; record the
actual hash and never substitute a version string for binary identity.

The Linux test-matrix job runs the workspace build and full test suite in this
order:

```sh
cargo build --workspace --all-targets --locked
TMPDIR="$(mktemp -d /dev/shm/tekt-ci.XXXXXX)" cargo testit --no-fail-fast -- --test-threads=1
```

The separate Linux `checks` job runs:

```sh
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo fmt --check --all
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --no-deps
```

The separate 32-bit job runs:

```sh
cargo check --locked --workspace --target i686-unknown-linux-gnu
```

`cargo testit` is the workspace alias for `cargo test --workspace --locked`.
The full suite runs serially on Linux. Recent export-test L0s require fixtures
under `/dev/shm` and do not fall back to disk. CI installs the export observers
listed above. The separate 32-bit job installs the `i686-unknown-linux-gnu`
target and `gcc-multilib` before checking. On Windows, CI compiles all targets,
including test targets, and runs the CLI with `--version`; it does not run the
full suite there.

The aggregate `Tests` check requires every job to succeed. A failed, canceled,
or skipped job blocks approval. Release and Docker workflows require this same
CI workflow to pass before publishing.

## Packaging

The release workflow builds `typst-wiring`’s `typst` binary with `--locked` for
the target matrix in `.github/workflows/release.yml`. It sets
`TYPST_COMMIT_SHA` to the checkout commit; `Cross.toml` passes that variable to
the container build, as described in the
[Cross build environment configuration](https://github.com/cross-rs/cross/wiki/Configuration#buildenv).

After building with `--target` in a clean checkout, package the binary with:

```sh
python3 lab/tools/ci/package_release.py --target x86_64-unknown-linux-musl --builder cross
```

The helper creates `lab/target/release-packages/typst-<target>.tar.xz` (or
`.zip` for Windows) and a `.sha256` file. The package includes the binary,
README, `LICENSE`, `NOTICE`, and `BUILD.json` with the commit, Git tree, binary hash,
lockfile and toolchain hashes, compiler version, and declared build command.
The helper refuses modified files, non-ignored untracked files, and a mismatched
commit identity. It records the declared recipe; it does not run the build or
prove that an arbitrary binary was produced by that recipe. The workflow builds
and packages in sequence in the same checkout.

`BUILD.json` also records `source_date_epoch` as the commit timestamp. This is
packaging metadata; the release build does not set `SOURCE_DATE_EPOCH` from it.
`TYPST_COMMIT_SHA` supplies the source commit identity to the binary build and is
a separate input. The Docker workflow supplies `REVISION` and a UTC wall-clock
`CREATED` value to the Docker build; it does not use the package metadata epoch.
Manual Docker builds should provide those arguments to preserve image
provenance.

The 2026-10-13 local CI/release checks, their limitations, and remaining blocks
are preserved in
`lab/results/construction/diagnostics/linhagem-ci-release-2026-10-13.md`, with
the follow-up at
`lab/results/construction/diagnostics/correcoes-producao-2026-10-13.md`.

## Legacy symbol contract gate

`test_symbols_probe.py` exercises the diagnostic classifier for the approved
native-Symbol contract, including the twelve bare legacy math extensions.
They require native `symbol` content, the original complete glyph and the
existing deprecation policy. Equal glyphs in `text` content remain identity
drift. Qualified legacy representations retain their namespace warning rules.
These extensions are absent from the pinned oracle: success remains `EXTENSION`,
never a new parity `MATCH`. Unexpected oracle results remain baseline defects.

The helper lives in excluded `lab/`, outside productive L0 ownership; it tests
the existing approved contract in `00_nucleo/prompts/core/entities/content.md`
and its pinned native-symbol Nucleus without changing or resealing that contract.
Run its tests and the real probe with explicit binary paths:

```sh
python3 -m unittest discover -s lab/tools/ci -p test_symbols_probe.py -v
python3 lab/parity/tools/p1495_symbols_probe.py --vanilla lab/typst-original/target/release/typst --crystalline lab/target/product/release/typst --details
```

The historical pre-R7 failure receipts remain unchanged. Fresh R7 receipts and
full validation are under
`lab/results/construction/diagnostics/legacy-symbol-gate-r7-2026-10-24/`.
