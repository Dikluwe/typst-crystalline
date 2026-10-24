# L0 — compiler/eval/rules/textual/matching/projection
Hash do Código: b600c8ee


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/rules/textual/matching/projection.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3


## Measurement before decision

At the working tree captured in `lab/results/construction/diagnostics/atomizacao-textual-projecao-2026-09-30-before.json.gz`, `01_core/src/compiler/eval/rules/textual/matching.rs:186-223` contains unmatched-range projection and flattening of replacement sequences. The existing `p1494` filter passed 17 tests after the space/matching movement, before this projection movement. These helpers use content and ranges only; engine, selector and style state are absent.

## Decision and behavior

Own reconstruction of an unmatched textual range from its original content nodes. Preserve complete nodes, including explicit and inferred spaces and style, when wholly included. Slice only partially included Text into a new Text node. Flatten replacement sequences recursively in their original order. Keep UTF-8 byte ranges supplied by the matching owner and do not change selector matching or error behavior. The matching owner calls these helpers; it retains rule precedence, guard state, engine restoration and transform invocation. No public signature, phase, default or compatibility changes.

The existing unmatched-range projection test belongs to this module. No dynamic dispatch or additional I/O is introduced.

## Acceptance

The projection test, existing `p1494` filter and full core suite remain green. `cargo build` and `crystalline-lint .` have no new blocking violations. No speedup is claimed without measurement. Header ownership is 1:1.

## Causal realization evidence and ratification

`lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz`
retains HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working
tree, UTC/status/diffstat/sources/argv/binary hashes and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. before-sources.json.gz contains
complete current sources/L0. No candidate code was implemented at measurement.
matching.rs:118–150 applies reverse regex then literals in declaration order;
:194–207 exposes products to later passes. textual.rs:102–123 captures full
snapshot. realize/mod.rs:177–217 visits recipes bottom-up. apply.rs:620–648/
matching.rs:218–225 call functions without capturing children.
expression_dispatch.rs:125–129 already distinguishes insertion_rules from local
state.

Measured pairs: identical literals C/B; literal/regex tie R/L; overlap X/YB;
first effect LATER/FIRST; inverse chained function BBBB/BB; separate product
AB/X; product+neighbor X/AB; Strong hiding child gives panic HIDDEN/Q;
captured global followed by new global L/R, captured local L/L; local CJK
水\n果 gives 水果/X. Pinned foundations/selector.rs:110–125 unifies escaped
literal/regex; typst-realize/src/lib.rs:1328–1405 chooses start and carries
revocation into product. These are language/effects, not ratification of Rust
mechanics/buffer equality/PDF bytes. Scope separation and earlier child visits
are inferred causes, refutable by acceptance.json, equality/fields/style/query
and error controls.

Human confirmation of manifest
`47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`
ratified the pinned textual-recipe-frontier nucleus and this order. Within this
scope it supersedes provisional ContentBlock show execution and pending-recipe
bottom-up visits. Ordinary expression evaluation order stays. Do not reevaluate
the program, advance State/Counter, bake render into morphology or create a
public Rust contract.


## Current exclusive causal projection

Project intervals from original nodes/causal boundaries, preserving wholly covered
nodes. Partial Text keeps Source/origin/correct UTF-8 positions. Do not cross
completed product/effective style/incompatible snapshot when rebuilding run.
Flatten only private product scaffolding while its boundary remains represented;
never erase authored Sequence/explicit Empty/provenance. Pure owner: no Engine/
callback/normalization.

Require acceptance.json families, R2-S/R3 and single-evaluation controls RED→GREEN.
Validate PDF/HTML separately; build/workspace/doctests/strict Clippy-Rustdoc/fmt/
V1/V5/V15/V26/reverse/differential SARIF without new error-warning. Use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. No RAM gain without
paired experiment or suppression to clear gate; no commit/push/release.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

Slice an unmatched Symbol substring as Symbol with its complete byte-valid Unicode substring, preserving origin; converted non-math text remains Text.
