# L0 — compiler/eval/repr/selector
Hash do Código: 91182105

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/repr/selector.rs`. At HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` with a clean tree, `01_core/src/compiler/eval/repr.rs:1467-1515` contains the separated behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. The purpose is to make algorithms discoverable by behavior, with no presumed performance gains.

## Contract

Format Selector recursively, including ordered Element.where fields and Before/After with explicit false inclusive. Preserve function names, escaping and Value formatting.

The detailed contract and historical parity evidence remain in `00_nucleo/prompts/core/compiler/eval/repr.md`; extraction preserves output, public signatures and phase. No I/O or mutable global state is permitted.

## Acceptance

Preserve language morphology tests, build, and pass `crystalline-lint .` with a valid owner and hash.

## Ratified bare table.cell selector boundary

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/math-residuos-table-cell-2026-10-06/{before,table-additional-before,table-aliases-before}.json.gz`
retains UTC/diffstat/inputs/commands/source and executable hashes. Historical pin:
`9f2b6e8715237cb086899a42873660fe744622e8`. selector(table.cell) returns cell;
where() returns cell.where(:), comparison is false, and query/counter reject
nonlocatable cell. A genuine CodeBlock alias preserves those results. The
previous document-only alias fixture did not expose its variable to eval and
was not positive alias evidence; table-aliases-before corrects it.

Upstream foundations/selector.rs:309–321 distinguishes dict None from Some;
:421–450 validates locatability separately. Local entities/selector.rs:34–37
represents only a present Element group; stdlib/foundations/selector.rs:69–77,
348–354 omits native cell. d08-owner-audit.json retains other boundaries/hashes.
Form/identity/query rejection are language; the closed carrier is chosen
mechanics. Source explicitly distinguishes the states; do not infer intent from
output alone. The inference that the new variant covers selection/locatability
is refuted by a public absent-group/identity case it cannot represent. This does
not presume correct Cell realization.

### Ratification, responsibility and incomplete scope

The human confirmed the selector-only manifest
`f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c`;
receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-selector-aprovacao-2026-10-06.json`
at 2026-10-06T22:36:30.591466+00:00. Its original ADR-0127 gate is satisfied;
the archived proposal-only instruction is not a new pending approval.

Format NativeElement with the static catalog's canonical recognized element name (cell), without .where and without deriving identity from function.name(). Empty Element remains cell.where(:), including compositions, arrays and counters. Preserve all existing formats/signatures. Do not change global Func formatting to obtain Selector representation.

This selector package covers carrier/construction/repr/equality/composition/
locatability only. The broader Cell domain needs separately measured direct show,
body/field projection and explicit/implicit cell preparation, including positions,
padding/spans/header/footer/lexical scope. Its own measured L0 and public/phase
gates are not authorized by selector approval. Preserve negative controls and no
universal parity claim. Atomic promotion, language RED before code, static/safe
Rust, 1:1/V26 before restricted owner resealing, build/core suite/linter remain
required. Revalidate bare/where/alias, true/false equality, direct/composed
query/counter, homonymous function and partial/closure; preserve D05/D07/query.
