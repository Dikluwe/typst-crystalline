# L0 — compiler/stdlib/structural/math/matrix
Hash do Código: d5276fd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/matrix.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

At the uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz`, `01_core/src/compiler/stdlib/structural/math.rs:725-1042` contains `MathMatrixOptions`, relative-length and augment casts, named option application, row interpretation, rectangular padding and the `math.mat` native. Static binding occurs later in the parent. The `p1431` test filter passed 29 tests before movement. This is a structural and regression baseline, not a performance result or a new bilateral vanilla measurement.

## Language contract

Own `math.mat` with variadic positional rows and named `delim`, `align`, `augment`, `gap`, `row-gap` and `column-gap`. Unknown named and duplicate occurrence are errors; `rows:` is not accepted. Preserve validation order and the source spans and wording of each diagnostic. `gap` fills both axes first; specific row/column gaps override it. The defaults and explicit-presence bits are retained, including when a supplied value equals its default. Alignment must be horizontal. Relative length accepts Relative, Ratio and Length without losing its relative component. Delimiter conversion is called from the exclusive `compiler/stdlib/structural/math/cases.md` owner; this prompt does not duplicate that mapping.

Scalar positionals form one row. In array mode all positionals must be arrays; in delimited-row mode they must be supported parenthesized MathDelimited rows. Mixed incompatible shapes keep their existing errors. Convert each cell with the existing Content/LocatedContent/None/Symbol/value-to-content mapping. Pad shorter rows with Empty to the maximum width. Preserve row order, empty rows and comma splitting in delimited rows. Construct the complete semantic `Content::MathMatrix`; do not execute layout or separate `&`.

`augment` accepts none, an integer vertical offset or a dictionary with `hline`, `vline` and `stroke`. Offsets accept an integer or array of integers and are checked against dimensions under the same current source-span gate. The dictionary consumes these three recognized fields and tolerates other internal keys, matching the measured upstream asymmetry; this does not permit invalid types for known fields or unknown top-level named arguments. Stroke extraction uses the existing stdlib helper. Preserve the existing `MathMatrixExplicit` bits and `MathMatrixAugment` carrier.

The parent `math.rs` owns only static registration of the native binding. The structural hub reexports `MathMatrixOptions`, `math_matrix_augment` and `apply_math_matrix_option` directly from this child with unchanged visibility; eval and layout consumers keep their established bridge. No dynamic dispatch, public signature change, default, pipeline phase or compatibility break is authorized.

## Acceptance

A local regression checks rectangular padding and `gap` shorthand with a
specific `row-gap` override. The P1431 regression filter and full core suite
remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. Header ownership is 1:1. No performance gain is claimed without a separate measurement.
