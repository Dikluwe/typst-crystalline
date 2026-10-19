# L0 — Effective inline text style
Hash do Código: e917cbea

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f

Layer: L1. Exclusive consumer: `01_core/src/compiler/layout/text/style.rs`.

## Evidence before specification

`01_core/src/compiler/layout/text.rs:29-222` resolves TextStyle through typed/custom chain precedence, including script scaling. Space previously used only cached style at `01_core/src/compiler/layout/space.rs:14`, which does not prove effective font at a lexical boundary. The approved carrier requires actual font identity. No new parity measurement is claimed.

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`, before-measurement.json.gz, morph-before-measurement.json.gz and audit.json preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC, status/diffstat, sources/L0/binaries/argv and historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. named-em A measures 0pt versus 24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. Pin accepts all six Size/Color/body permutations but Tekt rejects them. Positional set 18pt is ignored (12 versus 18pt). Pinned `text/mod.rs:290,303,924–930,1136–1142` and `args.rs:178–247` establish typed search, named preference, remaining body and composition. `style_chain.rs:513–517` and `constructor.rs:123` discard em; collapse at `style_chain.rs:394–395,419–422` uses top-wins where size needs folding. Size/scope are language observables; Rust representation is internal. Infer no historical intent. Restoration, Raw, heading, math, capture/show, duplicates/spans, zero/negative or metrics divergence can refute the correction. Equality probe refuted comparing declared values: in pin, text(size:2em)[A] == text(size:24pt)[A] is true; pinned StyledElem::eq at `content/mod.rs:779–782` compares child. That equality defect needs a separate correction; full type alone does not close it.

## Ownership and contract

Own pure effective-style resolution extracted unchanged from Text. Preserve field-specific precedence, typed size, custom font override, folded variations, semantic strong/emph provenance, math fields, script scaling 0.65, explicit script size and baseline offsets. Do not mutate Layouter, perform I/O, choose installed fonts or emit text. Text invokes once before its temporary style scope. Markup/Newline separators invoke once at current chain; Literal separators already within the resolved scope reuse that style to avoid double scaling. This internal descendant function introduces no public data/trait contract.

Use the already-resolved Pt size from active style; remove dead abs-only custom text.size projection without replacing deliberate heading/math/script sizes with exterior style. Preserve all other folds, metrics and emission.

## Approval, acceptance and remaining scope

The human ratified full text size (TS1) and both public contracts in proposal manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba` on 2026-10-17. Implementation/tests S1–S16 operate under 12GiB/zero swap; no release/merge/push. Reseal only after implementation and valid graph. RG1 remains independent. The approved cross-owner public changes are boundary criteria, not extra ownership of those consumers; language equality remains separately unresolved.


## Batch D V16 measurement (2026-10-19)

Evidence precedes the decisions below. The exact HEAD, uncommitted file list,
UTC, source bytes, argv, source/output/binary SHA-256 and pinned oracle recipe
are recorded in `lab/results/v16-corrections-2026-10-19/batch-d/agents/measurement/probes/png-before.json`.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
SVG geometry is evidence of occupied layout space, not a byte-equality contract.
These observations establish behavior; they do not infer upstream intent.
`text/style.rs:71-137` decodes literal family entries and dictionaries with
name Str/Regex plus required variants array; malformed entries are skipped.
`eval/rules/set/text.rs:220-315,423` and `eval/font_dict.rs:30-240` normalize valid
source schemas to that transport. `FontList::new` returns None for an empty
result and effective style then retains the ambient font. Direct custom chains
can contain invalid Values; source validation is not a proof they are impossible.

### Decision and refuters

At the two selected matches, explicitly distinguish Str/Regex dictionary names
from all invalid name variants, and Str/Dict family entries from all invalid
entries. Keep order, required variants and optional variant/weight/style fields,
custom-font precedence, and every unrelated TextStyle field. Do not select fonts,
reparse source or diagnose malformed raw transport here. Missing/invalid entries
retain existing filtering; an empty decoded list retains ambient font. The
misleading comments describing these schemas as nontext Content projections must
be removed. A valid writer emitting another name/entry kind refutes this schema;
the writer's validation remains outside this owner. Each new Value variant must
force a decision at both exact matches. ADR-0127 continuous internal flow applies.
