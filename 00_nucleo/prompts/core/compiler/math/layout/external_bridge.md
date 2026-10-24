# L0 — compiler/math/layout/external_bridge
Hash do Código: 6df29002

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/math/layout/external_bridge.rs`. The original behavior was in `01_core/src/compiler/math/layout/mod.rs:1325-1580` at HEAD `6fc546566`, clean tree, 2026-10-01 01:04:22 UTC. This is internal extraction, not a new parity or performance measurement.

## Contract

Lay out external content inside math and recursively traverse vertical ink. Preserve injected metrics, local style chain, baseline anchor, flattened top-level items, textual fallback for an empty frame and Group/Link traversal. Preserve the public contract, pipeline phase, defaults, compatibility and L1 purity.

## Acceptance

The L1 suite, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.

## Ratified bidi carrier adaptation

Before decision, `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
identifies this unit's references and exclusive owners. Manifest 1ae7513e's
contract was confirmed. Adapt uncertified constructors with None and read patterns
with `..`, preserving existing language observables. Transformations follow the
pinned origin nucleus without expanding phase/API.

## Ratified complete text size

### Evidence before decision

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC, status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures A at 0pt versus
24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. All six
Size/Color/body permutations are accepted by the pin and rejected by Tekt.
Positional set 18pt is ignored (12 versus 18pt). Pinned text/mod.rs:290,303,
924–930,1136–1142 and args.rs:178–247 establish typed search, named preference,
remaining body and composition. style_chain.rs:513–517 and constructor.rs:123
discard em; collapse at style_chain.rs:394–395,419–422 uses top-wins where size
needs a fold. Size/scope is observable language; Rust representation is an
internal choice. Infer no historical intent. Restoration, Raw, heading, math,
capture/show, duplicates/spans, zero/negative or metric failures can refute it.
The equality probe refuted comparing declared values: the pin returns true for
text(size:2em)[A] == text(size:24pt)[A]. Pinned StyledElem::eq at
content/mod.rs:779–782 compares child. That equality defect requires independent
correction; a complete size type does not close that axis.

### Ratification and limits

The human approved the complete-size package and its two public contracts in
manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Require implementation and S1–S16 tests under 12 GiB/zero swap; no release/merge/
push. Reseal only after implementation and a valid graph. RG1 remains independent.

### Exclusive bridge responsibility

The bridge adapter supplies Style::Size with absolute Length converted from
resolved Pt. Preserve origin, box, glyphs and effective external-bridge size.


## Planned direct Equation source bridge

### Measurement before correction

`lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json`
records the source-bound diagnostic11 product and the pinned vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Actual public syntax
`math.equation(metadata("equation-child"))` and
`math.equation(figure([EQUATION CHILD], caption: none))` both succeed in the
oracle and fail source-plan verification in the product. Ordinary `$a + b$`
succeeds with equal typed observations in both. Exact before source/L0 bytes,
hashes, UTC and working-tree provenance are archived in
`agents/layout/equation-bridge-before.json.gz` under that implementation root.

The source plan already declares EquationBody and recursively plans the direct
body according to the existing semantic partition. The equation painter calls
MathLayouter without that cursor; its external bridge creates an isolated
unplanned Layouter. Therefore required direct-body facts cannot be admitted by
actual painting. This is a missing internal handoff within the human-approved
physical-page phase, not authorization to open terminal MathSequence subtrees.

### Exclusive correction

Factor the existing typed TextStyle projection and external-frame composition.
The planned path consumes the frame painted once by the parent Layouter using its
actual resources and transaction; it never creates a second temporary painter.
The existing unplanned temporary-painter path remains unchanged. Retain real
FrameItems for final position projection, but exclude authenticated private
position markers from the planned frame's ink/width/empty-paint calculations.
A marker-only zero-paint action retains zero extent and all markers; markers are
not visible shapes. Preserve existing plain-text fallback and baseline/axis
anchoring. This does not authorize new math descendant coverage or public APIs.
