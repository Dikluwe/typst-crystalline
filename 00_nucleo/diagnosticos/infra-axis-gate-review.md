# Infra follow-up: matching-axis instancer dependency guard

The early Python/fontTools guard disagreed with the existing instancing contract. With the real Ubuntu Sans variable fixture and an unavailable configured interpreter, the control SVG/PDF compiled; adding only `ZZZZ: 1` failed with the missing-instancer diagnostic, although this axis is absent from the font and the instancer owner specifies unchanged bytes for no matching axes. This is a demonstrated dependency-validation defect, not a measured explanation of the vanilla performance gap.

## Evidence and decision

`probe.json` and `pdf-red.json` retain the pre-correction CLI commands, UTC timestamps, exit codes and executable/font identity. `red.stdout` records the behavioral regression failure through the actual resource-resolution function. A preceding test compilation error is separately retained as `preparation-compile.stderr` and is not counted as behavioral RED evidence.

A private `requires_font_instancing` predicate in `03_infra/src/pipeline/resources.rs` now intersects merged derived/explicit requests with the parsed face's actual axes. Resource resolution, standalone PDF and bundle PDF use that owner for their early dependency guards. Only a matching axis requires the availability probe. The existing instancer, failure diagnostic, font key, explicit override order, actual instancing, single/multiple-font dispatch, export options and L3/L4 boundaries are preserved. No production API or new default/flag was added, and no files in `infra/export/` were edited.

L0s were updated before code under ADR-0127's internal-correction flow. The existing font-variant owner already specified filtering and unchanged bytes; it was inspected and not altered. Resource validation has one normative owner, with PDF/bundle delegating the guard. The test owner specifies a real-font unknown-axis control, source-byte preservation and a matching `wght` dependency control.

## Scope and reproducibility

The starting HEAD was `3dee39ead` (the preceding requested infra commit), with preserved unrelated working-tree changes. The full SHA and starting UTC timestamp are in `infra-axis-gate-review/provenance.json`. Before-images, incoming patch/status, the isolated change patch, exact commands, final input/consumer fingerprint, toolchain and preservation checks are retained in that directory. This record is about the working tree after that commit, not a claim that all inputs were committed or that the CLI's version string proves its source identity.

Changed owner/consumer pairs:

- `00_nucleo/prompts/infra/pipeline/resources.md` and `03_infra/src/pipeline/resources.rs`.
- `00_nucleo/prompts/infra/pipeline/pdf.md` and `03_infra/src/pipeline/pdf.rs`.
- `00_nucleo/prompts/infra/pipeline/bundle.md` and `03_infra/src/pipeline/bundle.rs`.
- `00_nucleo/prompts/infra/pipeline/tests.md` and `03_infra/src/pipeline/tests.rs`.

Formatting and hash refresh were local to those pairs, with existing nucleus pins verified. No ADR, shared nucleus, other consumer or benchmark was changed. All earlier evidence seals remain intact; outside-scope tracked patch sections and fingerprint inputs are unchanged.

## Validation and limits

- Isolated regression with `TYPST_CRYSTALLINE_PYTHON=/nonexistent/infra-axis-probe-python`: RED before correction, GREEN afterward. The test never mutates the process environment and confirms unchanged resource bytes for `ZZZZ`, while matching `wght` still produces the existing dependency error under this command.
- `cargo test -p typst-infra --lib pipeline::tests --offline -- --test-threads=1`: 116 passed, no failures.
- `cargo build --offline`: passed with existing warnings.
- Final `crystalline-lint . --format sarif`: no errors; existing owned warnings/notes are compared with the preceding sealed lint result, rather than claimed removed.
- `run_probes.py` exercises the existing CLI with an unavailable interpreter. SVG and PDF controls and unknown-axis inputs succeed; matching `wght: 700` inputs fail closed for both formats. An unknown-axis bundle PDF also succeeds. Exact environment, fixtures, executable and font hashes, and unchanged before/after source fingerprints are in `probe-green.json`.

The CLI uses the development build for correctness probes. Its build preceded the final metadata-only prompt/header seal; no executable behavior changed during that seal. No duration is a performance result. PDF byte identity, renderer equivalence against upstream and complete variable-font support are not established here. Bundle is covered by the post-correction CLI control and the existing pipeline suite, without a separate new bundle RED invocation.

The previous open questions about shaper language/script identity, TIFF density conversion, resource instancing costs for supported axes, FileId exhaustion, collection boundaries and causal pipeline reuse remain open. Follow a stable release measurement protocol before attributing speed gains. This correction prevents an unnecessary dependency rejection and centralizes its guard; it does not prove that font instancing accounts for the observed total compilation-time gap.
