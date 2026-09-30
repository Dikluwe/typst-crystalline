# Entities, contracts and wiring documentation sanitation history

## Status and provenance

This is a diagnostic, not a materializable L0 or an approval to change code.
The sanitation is limited to entities, contracts and the wiring hub. It does
not establish completion of the global documentation sanitation. The file
manifest below distinguishes edited documents from documents still requiring
individual semantic review; unchanged English prompts are not automatically
declared reviewed.

The inspected checkout is a pre-existing dirty working tree based on HEAD
`6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5` (verify the exact HEAD in the
verification record below). The manifest records each edited document's exact
pre-edit SHA-256, including pre-existing uncommitted changes. No clean-commit
provenance or new bilateral parity measurement is claimed. The current upstream
reference remains `9f2b6e8715237cb086899a42873660fe744622e8`.
Older `e0e8ca4d`, `a51e02804` and `586e1bd43` observations are historical only.
Version labels alone are not oracle identities.

Repository instructions and ADR-0107/0108/0109/0127/0129/0130 govern the cleanup.
Exclusive ownership was resolved from actual Rust `@prompt` headers, rather
than from proposed target names or orphan exceptions. Hash metadata and existing
nucleus pins are retained; no global reseal, formatting or productive changes
are part of this task.

## Contract history retained outside L0

### World and external service contracts

World's earlier staged descriptions mixed incompatible `read_bytes` signatures,
an integer-based mock clock and a claim that tracking was generated directly
from World. Later requirements specify optional Duration offsets and rooted
path identity. Current World already exposes those types. Tracking is a separate
contract/facade, not a generated supertrait; ADR-0005's tracked boundary remains
distinct. Legacy helpers are still required to compose supported rooted
resolution once. Their default error implementations do not demonstrate that
composition; this is a remaining adapter obligation, not a reason to remove it.
Historical source anchors include `lib.rs:92-99`,
`foundations/path.rs:186-214` and `03_infra/src/world.rs:493-531`.
The earlier clock probes using offsets 0, 2, -3 and a two-hour Duration were
snapshot-specific; they are not remeasurement against the current upstream pin.
The staged execution references P1141/P1146 and the earlier numbered World
integration stages are logistics, not additional normative owners.

PluginHost's read/runtime/injection/validation stages P696–P700 and transition
P819 were execution history. Its old note deferring Send + Sync contradicted
its own declared bounds and the current trait. Both bounds remain preserved.
Calls, export names and errors retain their contract. Transition isolates linear
memory/page data, not arbitrary WASM globals; derived instances do not mutate
the original and siblings receive separate identities. Retained evidence is
`plugin.rs:366-380`, `328-352`, `177-182`, `420-426` and `522-545`.
Package downloading remains an injected L1 contract, not authorization for
domain filesystem or network access.

### Element boundary, registration and hubs

The pilot and migration-lot narrative P316–P330, implementation inventories and
old terminal/recursive tables described extraction logistics. The lasting
decision is model D's thin closed Content enum with static native delegation,
plus the sole E1 dynamic extension boundary. No enum elimination, vtable or
reverse entity-to-compiler dependency is authorized. The old speculative
absorption of ElementKind/ElementPayload is not a new requirement.
Historical dispatcher anchors were `content.rs:1258,2566,2671,3015,3058,3308`,
and locatability evidence `locatable.rs:217,10-11` with `layout/mod.rs:993-994`.
They are extraction-era coordinates, not all current locations.
The trait owner specifies its existing hooks, including resolve_settable;
element fields and implementations remain with exclusive child owners.
Dynamic bridging delegates to that trait instead of restating its full methods.
Registration notes still marking flush or later elements as future work are
obsolete execution states, not defaults. Debug-based Rust hashing is a mechanical
contract where already specified, not evidence of language equality.

### Geometry, page carriers and realized views

Page geometry's earlier gate P1140.20.1 and claims that the public page surface
was absent describe staging, not the current contract. The domain owns Paper
and PageBinding. The old rounded A4 literals `595.28`/`841.89` at
`layout_types.rs:694-695` are not the normative origin; conversion remains
millimetres times 72/25.4. Historical provenance was HEAD
`45b547073d7686cdd5d3e3030c82de3e22ec395f` with uncommitted changes, recorded in
`00_nucleo/diagnosticos/typst-p1140.20-propriedades-page.md`.

Removed cross-owner geometry duplication remains a handoff constraint:
PageMarginSpec has four optional components and an optional two_sided mode;
specific side overrides axis, axis overrides rest; physical and logical sides
must not mix; fold applies only supplied components/mode. PageMargins is a
physical snapshot resolved after binding and physical page parity. Default is
A4, unflipped, automatic binding and automatic margin. Paper fills only axes
not overridden in the same delta, including explicit Auto. Flip follows axis
resolution. Auto margin uses `(2.5 / 21) * min(width,height)` on finite oriented
axes; one infinite axis uses the other and two use A4 width. Auto binding uses
effective root direction and inside/outside is resolved per physical page.
Evidence remains `page.rs:613-718`, `pages/run.rs:105-130` and
`layout/page.rs:724-756`. Reconcile these with layout_types and geometry owners;
they are not waived by exclusive ownership sanitation.

PageCanvas staging P1140.20.2 concerned materialization, not a current gate.
Bleed remains relative/domain transport, and Auto/None/Paint fill states remain
distinct. PDF transparency versus SVG white belongs to exporters, not the carrier.

Running-matter history describes marginal resolution, auto-numbering selection,
explicit Content/None precedence, isolated header/footer layout and body
composition. That algorithm belongs to the existing layout/page_running owner
and the pinned shared nucleus, not the entity. Rich callback Content must not
be replaced by plain text; the existing alignment-width estimate is a limitation,
not a new intrinsic-width parity claim. Page-runs restore lexical configuration
in LIFO order even when empty, nested or multipage. Repeated visual marginal
artifacts must not duplicate body plain text, query identities or accessible
structure/MCIDs. Physical page indices are not substitutes for logical counters.
These cross-owner obligations must remain represented by their owners/nucleus.

The supplement prompt previously claimed ownership of reference layout and
sealing. Its domain now owns only Auto/None/explicit Content and localization.
The former reference contract remains a handoff constraint: RefForm is Normal
or Page, with Normal default and only the corresponding strings accepted.
Page references use the target's page, numbering and supplement, not the
reference's page configuration. Explicit content overrides supplement, explicit
None suppresses it, and omitted uses the target page. Nonempty supplement joins
the number with NBSP; empty has no leading space. Missing numbering requires
`cannot reference without page numbering` with a hint to enable
`#set page(numbering: "1")`. Internal destinations and forward-reference
fixpoint behavior remain required. Callback supplements require their own
contextual contract, never silent string conversion. The canonical reference
owner is `00_nucleo/prompts/compiler/layout_references.md`, not the formerly
written nonexistent `compiler/layout/references.md` path.

PageStore's P207D minimal schema used EcoString patterns, no per-Page metadata
and pending capture. Existing P1157/P1159 amendments supersede that schema with
raw Numbering, logical values, spans and separately realized marginal/reference
views. The historical logical-counter probe produced 7/8/9 with final 9,
distinct from physical 1/2/3; it justifies separate views, not a new runtime probe.
The compatibility constructor remains without realized views. The old no-PartialEq
restriction was never explicitly reconciled with the current public derive.

### Fonts and mathematical data

FontVariations' earlier wght=99999 probe in `temp/p836/e5_range.typ` exited
successfully. The July 2026 version label lacks the complete currently required
oracle identity; it cannot close current-pin parity. No range clamp is added to
this domain. Tag normalization, later-wins deduplication, fold precedence and
ordered diagnostic hints are preserved. The old claim that unicode-segmentation
was absent from the L1 whitelist is stale; the actual character-versus-grapheme
diagnostic limitation remains, independent of that false explanation.
ShapedGlyph moved to a separate exclusive owner; its u16 glyph identifier is
preserved, not silently widened to match ADR-0120's u32 wording. Run-level UPM
must remain the conversion basis, not a glyph-local substitute.

### Document features, metadata and output options

CompilerFeatures history includes HTML's extracted type ownership and Bundle
activation stages P1288/P1484/P1485. Empty defaults remain independent of
targets; the explicit html() helper is not default feature activation.
Compatibility through the HTML reexport refers to identical types, not a wrapper.
Evidence tied to baseline `bde47...` and upstream `586e1bd43` is historical only.

DocumentInfo's earlier metadata stages P536/P1031 preserve a reduced carrier
with title, author and keywords. The evaluated comma-separated author-array
conversion is not performed by the carrier. Metadata is embedded, not visible
body text. Upstream date/description and richer Content/multi-author fields were
scope-outs, not newly approved API. PDF/UA title evidence is not full compliance.

DocumentFormatOptions' staged SVG/PNG/HTML/PDF/tagging/pages/standards sequence
P1462–P1469 and dated approvals are execution history. The enduring decision is
resolved document carriers without CLI overrides, PNG in pixels per inch,
tri-state tagging, optional physical page ranges and structured PDF profile
conflicts. The reduced PDF standards scope validates language names/profile
compatibility and effective header version; it does not implement full PDF/A or
PDF/UA compliance. Historical oracle SHA-256 was
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
for the old `586e1bd43` build, not the current pinned oracle.
The earlier `(1, "x")` failure concerned PageRange validation and must not be
misattributed as PdfStandards evidence. Existing conflict data/order remains
normative; diagnostic text formatting belongs to evaluator/export consumers.

### Semantic text elements and terminal commands

Emph/Strong's P1339 history records baseline
`2f42d64253547734564513a1159ee6b584c1c4b4`, where the existing Element default
returned no payload (`elements/mod.rs:173`; `emph.rs:34-65`;
`strong.rs:38-69`). The later decision requires the own occurrence's
NativeElement payload, even with empty body, not payload for visual descendants.
Functional identity and causal snapshots remain compiler responsibilities.
Align's model-D lot and inline-to-Arc migration were extraction history.
Its alignment/body equality, recursive maps, default nonempty wrapper and manual
Debug-based Rust Hash remain preserved, not new language-equivalence criteria.

Divider's pilot proved a fieldless structural singleton. Its old 0.5pt line
clause belongs to layout and remains a handoff obligation. Flush's P1292
namespace/flow history does not alter its terminal identity. The earlier
cross-owner contract remains: place.flush accepts zero arguments; positional
arguments produce `unexpected argument`, named ones `unexpected argument: <name>`,
and repr is `flush()`. It forces preceding floats, not later ones, without text,
a visual item or an introspection identity.

Linebreak's P1140.10–P1140.12 staging is not normative chronology. Presence
preserves omitted versus explicit false for repr/reflection. The old layout
handoff requires true to expand the preceding line's opportunities from current
width; false/markup omission preserve ordinary breaks. Keep the explicit
boundary through bidi reflow and equivalent RTL true/false behavior without
empirical formula constants. Colbreak's P1140.9 amendment preserves weak and
weak_explicit: the existing constructor treats false as omitted and true as
explicit. Both remain nonempty terminal commands despite empty text projections.

### Bytes, signed intervals and allocation identity

Bytes' P398 migration and P395 enum gate are execution history. The P1031
source correction rejects unsupported opaque language semantics: bytes are
iterable and convertible; Vec is mechanics. The old restricted cast list and
upstream array-conversion evidence need reconciliation in constructor/cast
owners, not a silent choice here. Value::Bytes keeps the bytes type name;
the old `bytes({len})`/no-binary-dump repr obligation belongs to repr.

The prior byte-method handoff remains explicit: len counts octets; at accepts
negative-from-end indices, returns integers 0–255 and uses a supplied default or
the required out-of-bounds diagnostic. Slice admits len boundaries and negative
indices; end below start yields empty; supplied end wins over count according
to the historical P1214 probe. First/last were absent from the measured public
byte scope. Slices return Bytes, not arrays. Concatenation, EcoVec refinement and
sophisticated encoding detection were scope-outs, not newly approved features.

Duration's P400/P850 migration established signed intervals, not datetimes.
The P1140.1-B probe distinguishes a callable global type from an ordinary
function: type(duration) is type; constructed values keep the duration type
name. That is an eval binding/dispatcher obligation, not storage.
The old cast handoff accepts identity, Int as signed nanoseconds and Float as
signed seconds times 1e9 with overflow rejection; string parsing was out of
scope, other casts return None. Signed add/sub require overflow errors; scalar
multiply/divide and duration/duration Float division require zero-divisor errors.
Comparison is signed total ordering. Language seconds/minutes/hours/days fields
return signed Float. Repr uses named nonzero week/day/hour/minute/second
components, negative components for negative intervals, and duration() for zero.
Compact domain to_string does not replace any of these language obligations.

PtrEqArc's DEBT-26 closure and image-size anecdote are not acceptance metrics.
Shared-allocation equality/hash is a mechanical resource contract; image-data
immutability remains a consumer lifecycle obligation, not a type-level guarantee
or proof of general language equality.

## Discrepancies requiring reconciliation, not automatic code promotion

- CounterDisplay's L0 declares kind: String; the consumer uses CounterKey.
  Resolve the public type through its owner before any code/API change.
- Axes' facade reexport obligation is absent from the actual facade. Do not
  treat its absence as permission to delete the obligation.
- ShapedGlyph u16 conflicts with ADR-0120's u32 description; no widening or
  narrowing approval is inferred from current code.
- Location's production-identity obligation uses Locator. A legacy native
  fallback at `01_core/src/compiler/stdlib/counter.rs:692` constructs
  `Location::from_raw(0)` outside it; unrelated cfg(test) uses are not violations
  of that production-only requirement.
- PageStore derives PartialEq despite the earlier explicit restriction.
- PageSupplement's current table covers only English/Portuguese/Spanish/French/
  German and aliases. The obligation covers all languages already supported by
  the localization owner; a smaller current table does not repeal it.
- World rooted helpers are available but their legacy default errors do not
  establish the mandated compatible composition.
- The State always-unequal rule currently matches the absent equality arm but
  belongs to the Content equality owner; its cross-owner handoff remains pending.
- Old blanket Hash derives on Content-bearing elements must not be confused
  with supported Rust derives or language observational equality.
- The reference layout owner already records physical-pattern numbering versus
  the older logical-store promise as unresolved. This task does not settle it.

## Read-only inventory of documents directly in prompts/

Classification checks used actual consumers, crystalline.toml exceptions and
inbound references. No candidate was moved or edited as part of this inventory.

| Document | Classification evidence | Recommended disposition |
| --- | --- | --- |
| auditar-fatiamento.md | Workflow; no productive consumer; process exception at crystalline.toml:212; cited by compiler/stdlib/foundations.md:7 | Workflow under materialization/workflows; diagnostic case inventories separately under diagnosticos |
| auditar-spec.md | Workflow; no consumer; exception at crystalline.toml:213; template reference at auditar-spec.md:21 | materialization/workflows |
| template-prompts.md | Editorial template; no consumer; explicit exception at crystalline.toml:198 | materialization/templates |
| package-spec-dto.md | Unmaterialized proposal; no consumer; future exception at crystalline.toml:203 | Architectural review before archive under materialization or diagnosticos; do not call it a pure diagnostic solely because it is orphaned |
| passo-537b-set-page-columns.md | Actual exclusive owner of 01_core/src/compiler/layout/set_page.rs despite tactical filename; referenced by entities/elements/columns.md:57 | Retain L0 role; future authorized canonical rename to compiler/layout/set_page.md with header/reference migration; history to diagnostics |
| core.md | Actual owner of 01_core/src/lib.rs | Retain L0; sanitize its own historical content separately |
| infra.md | Actual owner of 03_infra/src/lib.rs; config's future-crate explanation is stale | Retain L0; reconcile stale exception separately |
| shell.md | Actual owner of 02_shell/src/lib.rs | Retain L0; separate historical content, do not move wholesale |
| wiring.md | Actual owner of 04_wiring/src/main.rs | Retain L0; hub sanitation is in the assigned scope |
| world-types.md | Actual owner of 01_core/src/entities/world_types.rs | Retain L0, not a process document |
| world.md | Actual owner of 01_core/src/contracts/mod.rs, but duplicates World/TrackedWorld clauses | Retain facade owner; reconcile duplicate trait ownership in a separately authorized task |

Further inbound evidence includes
`00_nucleo/materialization/typst-passo-1032.md:17`,
`00_nucleo/materialization/typst-passo-1021.md:32`,
`00_nucleo/materialization/auditoria-l0.md:143`, and
`00_nucleo/emenda-auditar-fatiamento-constantes-geometricas.md:1`.
`00_nucleo/diagnosticos/typst-passo-1001-relatorio.md:209` records that the proposed
L3 PackageSpec DTO was never implemented while L1 PackageSpec::from_str existed.
An old diagnostic labeling passo-537b as process does not override its live
productive ownership. Config/reference/header migrations are outside this task.

## Verification record

Read-only V15/V26 checks passed before the final review. V5 drift is expected
after edits because productive headers may not be changed in this task. No
`--fix-hashes`, `--update-snapshot`, global formatter or behavioral test run
is claimed. The final verification appendix records fresh results.

### Final checks for this partial delivery

The actual HEAD remains `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
`crystalline-lint . --checks v15,v26 --fail-on warning --format sarif`
exited successfully with no findings. The separate V5/V6 run exited with
warnings: scoped findings were V5 prompt drift; no V6 findings were emitted.
Outside-scope V5 warnings also remain and were not repaired. Rust headers were
deliberately not resealed because only assigned documentation may be edited.
The comparison to the pre-edit baseline found no changes to protected Rust
files or nucleus TOML files, and no change to Hash do Código metadata or pins.
Canonical prompt references introduced in edited files resolve to existing files.
The scoped `git diff --check` passed. No behavioral build/test/parity claim follows
from these documentation and lineage checks. Completing the unchanged corpus
and reconciling the recorded contract ambiguities remain pending.

## Edited-document manifest

Each entry records the baseline SHA-256 and actual exclusive consumer. It does
not certify all remaining prompts or replace canonical lineage metadata.

- `00_nucleo/prompts/contracts/package_downloader.md` — baseline `e157da50f7779be482998f707c554a59659e1f8e1f163371570c2517622f5b06`; consumer `01_core/src/contracts/package_downloader.rs`.
- `00_nucleo/prompts/contracts/plugin_host.md` — baseline `2f1ad06495067f883e79c062058b9b19c4320b3ef3b48f0f131df49d4182d9fc`; consumer `01_core/src/contracts/plugin_host.rs`.
- `00_nucleo/prompts/contracts/world.md` — baseline `cd2b0d775bf3dd759908f3440eb90758cfef8f0ed8d8529f0cce18be9ef12078`; consumer `01_core/src/contracts/world.rs`.
- `00_nucleo/prompts/entities/axes.md` — baseline `43e202850e4a046a96e6a7d8b02900ce6dfc2576dfa9ec5e0900a00790286065`; consumer `01_core/src/entities/axes.rs`.
- `00_nucleo/prompts/entities/bytes.md` — baseline `bd6199460444e5c82754a5d7050733bd0a5c36bf58d907855287fd1b6b539a7f`; consumer `01_core/src/entities/bytes.rs`.
- `00_nucleo/prompts/entities/compiler_features.md` — baseline `112cf905311abafa57a848068184b2819794364e5bc9bfbc9346825e35659449`; consumer `01_core/src/entities/compiler_features.rs`.
- `00_nucleo/prompts/entities/corners.md` — baseline `e8b96de0cdc0e5c921c99cd63790084fa86b5f2d2a567cd81223bb925e18baed`; consumer `01_core/src/entities/corners.rs`.
- `00_nucleo/prompts/entities/dir.md` — baseline `b9cd0f95837004169ec06f16ab6c9fb7b138f1407969e3da42e3587d72ae4aba`; consumer `01_core/src/entities/dir.rs`.
- `00_nucleo/prompts/entities/document_format_options.md` — baseline `6f330ae7cde1365a5222d7f3cf18dc27712a00a6052eadebaa2d25e1c874b343`; consumer `01_core/src/entities/document_format_options.rs`.
- `00_nucleo/prompts/entities/document_info.md` — baseline `51581971833af09a41b5621d5434228d83d0640e178315a5e681b410e141c60d`; consumer `01_core/src/entities/document_info.rs`.
- `00_nucleo/prompts/entities/duration.md` — baseline `b16ff16253af6ad0cd119679880a429f401bd3815cebb7077f70179bc1c6ae2e`; consumer `01_core/src/entities/duration.rs`.
- `00_nucleo/prompts/entities/element_registry.md` — baseline `e44721d0d074681a960f28311d681b6602c3d1edfc42e083c3ad99f88c5bbcf7`; consumer `01_core/src/entities/element_registry.rs`.
- `00_nucleo/prompts/entities/elements/_comum.md` — baseline `c8fa55a0a8439198532950b979e31f37007485b487fbe3181e7388cab30001b1`; consumer `01_core/src/entities/elements/mod.rs`.
- `00_nucleo/prompts/entities/elements/align.md` — baseline `835498f375803c5faed3021d5b465dd5ca857499c093546dd19942375e2279b4`; consumer `01_core/src/entities/elements/align.rs`.
- `00_nucleo/prompts/entities/elements/colbreak.md` — baseline `ae2d0563d735a147b0503eaa9d250ffe13a9f81b85efc0fda5cdefdcd4718409`; consumer `01_core/src/entities/elements/colbreak.rs`.
- `00_nucleo/prompts/entities/elements/context_block.md` — baseline `be82544e35e1bc9dbbbb40f9f16aae2ae80e4ea2cc6a77124a80c9bc7d03c4ae`; consumer `01_core/src/entities/elements/context_block.rs`.
- `00_nucleo/prompts/entities/elements/counter_display.md` — baseline `e3f2e1e7d1c970f692ac5824a1f4b4af2dbb68db3ef7fedb9aec2253384ab58a`; consumer `01_core/src/entities/elements/counter_display.rs`.
- `00_nucleo/prompts/entities/elements/divider.md` — baseline `8608f08f73692f03fcae33d7a7cf31d79533aa79699de8d4b8342e3d4e884122`; consumer `01_core/src/entities/elements/divider.rs`.
- `00_nucleo/prompts/entities/elements/dynamic.md` — baseline `c66a9e8367412c4dacf3bb551ceeeefc9bbfc598d16ea3a4e9b5600cb6406559`; consumer `01_core/src/entities/elements/dynamic.rs`.
- `00_nucleo/prompts/entities/elements/emph.md` — baseline `3c58a4b386503759b8dfa57b69185ea94f5ba3d41a674fb1a9447b602da85332`; consumer `01_core/src/entities/elements/emph.rs`.
- `00_nucleo/prompts/entities/elements/flush.md` — baseline `8662789565be6e2b5045f46677798d7563f4a5ee0f0115d476eccbec9be84929`; consumer `01_core/src/entities/elements/flush.rs`.
- `00_nucleo/prompts/entities/elements/grid_footer.md` — baseline `072a469b3aea70af1b40d81cc5d246da0c61fbcf3a6f5f6a7a558e3022f3b908`; consumer `01_core/src/entities/elements/grid_footer.rs`.
- `00_nucleo/prompts/entities/elements/grid_header.md` — baseline `071d7d249d373a6e0c06ff4fd5144bc2261acc081153010461641e205af30000`; consumer `01_core/src/entities/elements/grid_header.rs`.
- `00_nucleo/prompts/entities/elements/hide.md` — baseline `294b8335fd5e61612ce99598715fcc567f07260ad78aaa99a25ad7790aabc035`; consumer `01_core/src/entities/elements/hide.rs`.
- `00_nucleo/prompts/entities/elements/linebreak.md` — baseline `36cc971b1ed88ee10f5a4d60c9904692f34e4cc6c5961127b84057049c11f51a`; consumer `01_core/src/entities/elements/linebreak.rs`.
- `00_nucleo/prompts/entities/elements/pad.md` — baseline `9991ee1fce18da8664849eb2d9cc3ec9a7a1220ff7a9421246bb124b3552be1c`; consumer `01_core/src/entities/elements/pad.rs`.
- `00_nucleo/prompts/entities/elements/state.md` — baseline `227ab4c9447b1794963c6382bff1bef5773741148b1b973ab37599f62c516224`; consumer `01_core/src/entities/elements/state.rs`.
- `00_nucleo/prompts/entities/elements/state_display.md` — baseline `1c05e8e1996689fc27fb6a453c560a4f0f4b701e17c5c7d7a34a27034b708bf3`; consumer `01_core/src/entities/elements/state_display.rs`.
- `00_nucleo/prompts/entities/elements/strong.md` — baseline `58cf23d177073af1cae57303538bf5fcc93b0dfeacf2d3a2db465463acd5826b`; consumer `01_core/src/entities/elements/strong.rs`.
- `00_nucleo/prompts/entities/elements/table_footer.md` — baseline `e94fa330201d5c6d3388b476fb37f0ac33e08eeea968f75be7a17d814333cbc2`; consumer `01_core/src/entities/elements/table_footer.rs`.
- `00_nucleo/prompts/entities/elements/table_header.md` — baseline `b1bcc39b6919161353310e1c79325697b5450d27d119eaaca136752fcd315bb6`; consumer `01_core/src/entities/elements/table_header.rs`.
- `00_nucleo/prompts/entities/elements/terms.md` — baseline `ab48d0f2837310b6cde01a30efb5149e9aece29e1f9acc2af462fdebd2a9dcbd`; consumer `01_core/src/entities/elements/terms.rs`.
- `00_nucleo/prompts/entities/elements/test_callout.md` — baseline `41d7c741fcf9770ecda996e82a53f0ce491f980510a39b2cf454270f7d41cce0`; consumer `01_core/src/entities/elements/test_callout.rs`.
- `00_nucleo/prompts/entities/elements/title.md` — baseline `02ec0db4250149e2f788b5bb4637b4390ed345044eb9cc2fb7f2572e2f305d3a`; consumer `01_core/src/entities/elements/title.rs`.
- `00_nucleo/prompts/entities/font_variations.md` — baseline `4bcc503925b4bd83845d533100e2b1384a79c0cb740d8560ae51a910033d528b`; consumer `01_core/src/entities/font_variations.rs`.
- `00_nucleo/prompts/entities/label_kind.md` — baseline `d7e7f245928355978fc70b47029fe5402bd2a9365e0bc2eb97794acca22b1cd9`; consumer `01_core/src/entities/label_kind.rs`.
- `00_nucleo/prompts/entities/location.md` — baseline `4628ee19327ebe7eb60b13fe010c526e378b11efc76728b878de643d0a188883`; consumer `01_core/src/entities/location.rs`.
- `00_nucleo/prompts/entities/mod.md` — baseline `e6e17688d2be0f639e42be1eb2f15ff1ee507d7be32b14f29e0f025d0d64772a`; consumer `01_core/src/entities/mod.rs`.
- `00_nucleo/prompts/entities/page_canvas.md` — baseline `4a5059bf176c563b2775983d7e6b3ff73789c4e64e77449536b670f53aac0c77`; consumer `01_core/src/entities/page_canvas.rs`.
- `00_nucleo/prompts/entities/page_geometry.md` — baseline `5552b42f58308f1a19828fa2c330959f1199bfcd68c5b29594f7d7ce43b7ed9e`; consumer `01_core/src/entities/page_geometry.rs`.
- `00_nucleo/prompts/entities/page_running.md` — baseline `ac91674d6227f6044a7d1954d993b1411643147c1692ccf4e17815c5d3bc20a5`; consumer `01_core/src/entities/page_running.rs`.
- `00_nucleo/prompts/entities/page_store.md` — baseline `86d5e5a7ecbb1d8a026e2d6632d4a3d06d7c28f46916c484f603691766f4ad52`; consumer `01_core/src/entities/page_store.rs`.
- `00_nucleo/prompts/entities/page_supplement.md` — baseline `90d98353ec086ad27f3c519b2a61827b0e93d40d735a48d9642bff27690087ca`; consumer `01_core/src/entities/page_supplement.rs`.
- `00_nucleo/prompts/entities/parity.md` — baseline `070d818f5c200ccc6edf9fd8dab0be968b880cdcaa8177580a8a7dc26f742f84`; consumer `01_core/src/entities/parity.rs`.
- `00_nucleo/prompts/entities/ptr-eq-arc.md` — baseline `357edd9fa7e5e461f9541fd3141715b13c14fd84cd523d4a2c0fae134ba60f6b`; consumer `01_core/src/entities/ptr_eq_arc.rs`.
- `00_nucleo/prompts/entities/rel.md` — baseline `4593726fcf6393bf4f3d1440f5c783a4a5cf21166d5f8df655501c88fb5284ad`; consumer `01_core/src/entities/rel.rs`.
- `00_nucleo/prompts/entities/shaped_glyph.md` — baseline `addb5837fd6b8b46bc374c20869cb0251206351c4ed36b5dc988de8bdeca12ab`; consumer `01_core/src/entities/shaped_glyph.rs`.
- `00_nucleo/prompts/entities/state.md` — baseline `466e16e018de4bd1cc043e1cf79b92bf5c4b375b178a65ec85228e373bdecec4`; consumer `01_core/src/entities/state.rs`.
- `00_nucleo/prompts/wiring.md` — baseline `5fbfb19c8a062926fd768d058bc122eedfa686f10cd52ed3159ca9a71ccdb5d1`; consumer `04_wiring/src/main.rs`.

## Retained evidence index from edited baselines

These exact reference tokens are an index into the old evidence, not a claim
that every historical coordinate still identifies the same declaration today.

- `00_nucleo/prompts/contracts/plugin_host.md`: `plugin.rs:366-380`, `plugin.rs:177-182,420-426,522-545`, `lab/typst-original/.../foundations/plugin.rs:328-352`.
- `00_nucleo/prompts/contracts/world.md`: `crates/typst-library/src/lib.rs:92-99`, `foundations/path.rs:186-214`, `03_infra/src/world.rs:493-531`.
- `00_nucleo/prompts/entities/bytes.md`: `crates/typst-library/src/foundations/bytes.rs:16-28`.
- `00_nucleo/prompts/entities/compiler_features.md`: `01_core/src/entities/html.rs:12-42`, `01_core/src/entities/mod.rs:59`, `compiler/eval/mod.rs:119`, `02_shell/src/cli.rs:82-91`, `03_infra/src/pipeline.rs:77-82`, `04_wiring/src/main.rs:374`, `lab/typst-original/crates/typst-library/src/lib.rs:272-305`, `01_core/src/entities/compiler_features.rs:7-32`, `lab/typst-original/crates/typst-library/src/lib.rs:283-306`, `02_shell/src/cli.rs:182`.
- `00_nucleo/prompts/entities/dir.md`: `crates/typst-library/src/layout/dir.rs:6-20`.
- `00_nucleo/prompts/entities/document_format_options.md`: `lab/typst-original/crates/typst-library/src/layout/page.rs:760-869`, `typst-pdf/src/convert.rs:883-898`, `typst-library/src/layout/page.rs:806-847`, `02_shell/src/cli.rs:377-378`, `typst-pdf/src/format.rs:552-719`, `krilla/src/configure/validate.rs:993-1013,1187-1201`, `krilla/src/configure/mod.rs:68-96`.
- `00_nucleo/prompts/entities/document_info.md`: `crates/typst-library/src/model/document.rs:14-34`, `document.rs:157-179`, `document.rs:168-178`, `document.rs:151-153`.
- `00_nucleo/prompts/entities/duration.md`: `duration.rs:9-12`, `duration.rs:38-42`.
- `00_nucleo/prompts/entities/elements/_comum.md`: `01_core/src/compiler/introspect/locatable.rs:217`, `locatable.rs:10-11`, `01_core/src/compiler/layout/mod.rs:993-994`, `content.rs:1258`, `content.rs:2566`, `content.rs:2671`, `content.rs:3015`, `content.rs:3058`, `content.rs:3308`.
- `00_nucleo/prompts/entities/elements/align.md`: `content.rs:1798`, `content.rs:2356`, `content.rs:2640`, `content.rs:1958`.
- `00_nucleo/prompts/entities/elements/colbreak.md`: `content.rs:1790`, `content.rs:1617`.
- `00_nucleo/prompts/entities/elements/counter_display.md`: `content.rs:1703`, `content.rs:1867`.
- `00_nucleo/prompts/entities/elements/divider.md`: `content.rs:1726`, `content.rs:1512`.
- `00_nucleo/prompts/entities/elements/emph.md`: `entities/elements/emph.rs:34-65`, `entities/elements/mod.rs:173`, `model/emph.rs:26`.
- `00_nucleo/prompts/entities/elements/grid_footer.md`: `content.rs:1720`, `content.rs:1564`, `content.rs:2279`, `content.rs:2584`, `content.rs:1890`.
- `00_nucleo/prompts/entities/elements/grid_header.md`: `content.rs:1719`, `content.rs:1563`, `content.rs:2275`, `content.rs:2580`, `content.rs:1888`.
- `00_nucleo/prompts/entities/elements/hide.md`: `content.rs:1812`, `content.rs:1640`, `content.rs:2155`, `content.rs:2445`, `content.rs:1985`.
- `00_nucleo/prompts/entities/elements/linebreak.md`: `content.rs:1683`.
- `00_nucleo/prompts/entities/elements/pad.md`: `content.rs:1843`, `content.rs:1677`, `content.rs:2148`, `content.rs:2395`, `content.rs:1995`.
- `00_nucleo/prompts/entities/elements/state.md`: `content.rs:1657`, `extract_payload.rs:43`.
- `00_nucleo/prompts/entities/elements/state_display.md`: `content.rs:1659`, `extract_payload.rs:58`, `content.rs:2003`.
- `00_nucleo/prompts/entities/elements/strong.md`: `entities/elements/strong.rs:38-69`, `entities/elements/mod.rs:173`, `model/strong.rs:21`.
- `00_nucleo/prompts/entities/elements/table_footer.md`: `content.rs:1740`, `content.rs:1579`, `content.rs:2331`, `content.rs:2631`, `content.rs:1920`.
- `00_nucleo/prompts/entities/elements/table_header.md`: `content.rs:1739`, `content.rs:1578`, `content.rs:2327`, `content.rs:2627`, `content.rs:1917`.
- `00_nucleo/prompts/entities/elements/terms.md`: `content.rs:1754`, `content.rs:1570`, `content.rs:2114`, `content.rs:2450`, `content.rs:1934`.
- `00_nucleo/prompts/entities/font_variations.md`: `text/mod.rs:846-850`, `tag.rs:22-27`, `variations.rs:234`, `variations.rs:210-214`, `tag.rs:91-97`, `tag.rs:99-104`, `tag.rs:106-112`, `variations.rs:196-208`.
- `00_nucleo/prompts/entities/page_canvas.md`: `layout/page.rs:170-219`, `typst-layout/pages/run.rs:132-138`, `run.rs:218-244`, `page.rs:257-278`, `document.rs:88-121`, `typst-pdf/convert.rs:108-123`.
- `00_nucleo/prompts/entities/page_geometry.md`: `layout/page.rs:54-103`, `typst-layout/src/pages/run.rs:105-110`, `run.rs:149-155`, `layout/page.rs:724-756`, `layout/page.rs:818-1032`, `layout_types.rs:694-695`, `page.rs:613-718`, `pages/run.rs:112-130`, `page.rs:886-1032`.
- `00_nucleo/prompts/entities/page_running.md`: `typst-library/src/layout/page.rs:315-450`, `typst-layout/src/pages/run.rs:141-233`.
- `00_nucleo/prompts/entities/page_store.md`: `lab/typst-original/crates/typst-layout/src/introspect.rs:38-58`, `lab/typst-original/crates/typst-layout/src/introspect.rs:22-145`.
- `00_nucleo/prompts/entities/page_supplement.md`: `typst-library/src/layout/page.rs:315-329`, `typst-layout/src/pages/run.rs:145-149,233-235`, `typst-layout/src/document.rs:98-103`, `typst-layout/src/introspect.rs:29-56,122-129`, `typst-library/src/model/reference.rs:135-257,334-355`, `translations/en.txt:8`, `translations/pt.txt:8`, `translations/es.txt:8`, `translations/fr.txt:8`, `translations/de.txt:8`.
- `00_nucleo/prompts/wiring.md`: `04_wiring/src/main.rs:22`, `04_wiring/src/main.rs:27`, `04_wiring/src/main.rs:43`.

## Documents still requiring individual completion review

The following unchanged documents are not claimed sanitized by this turn. Some
were already English or already edited by the user; their pre-existing state is
preserved. Remaining normative Portuguese, chronology and ownership issues must
be reviewed against each consumer before completion can be declared.

- `00_nucleo/prompts/entities/args.md`.
- `00_nucleo/prompts/entities/ast/code.md`.
- `00_nucleo/prompts/entities/ast/expr.md`.
- `00_nucleo/prompts/entities/ast/markup.md`.
- `00_nucleo/prompts/entities/ast/math.md`.
- `00_nucleo/prompts/entities/ast/mod.md`.
- `00_nucleo/prompts/entities/bib_entry.md`.
- `00_nucleo/prompts/entities/bib_store.md`.
- `00_nucleo/prompts/entities/citation_form.md`.
- `00_nucleo/prompts/entities/citation_style.md`.
- `00_nucleo/prompts/entities/color.md`.
- `00_nucleo/prompts/entities/content.md`.
- `00_nucleo/prompts/entities/content/construction.md`.
- `00_nucleo/prompts/entities/content/emptiness.md`.
- `00_nucleo/prompts/entities/content/equality.md`.
- `00_nucleo/prompts/entities/content/fields.md`.
- `00_nucleo/prompts/entities/content/identity.md`.
- `00_nucleo/prompts/entities/content/morphology.md`.
- `00_nucleo/prompts/entities/content/page-structure.md`.
- `00_nucleo/prompts/entities/content/plain-text.md`.
- `00_nucleo/prompts/entities/content/rust-hash.md`.
- `00_nucleo/prompts/entities/content/sequence.md`.
- `00_nucleo/prompts/entities/content/tests.md`.
- `00_nucleo/prompts/entities/content/tests/morphology.md`.
- `00_nucleo/prompts/entities/content/tests/projections.md`.
- `00_nucleo/prompts/entities/content/tests/representation.md`.
- `00_nucleo/prompts/entities/content/tests/sequence.md`.
- `00_nucleo/prompts/entities/content/tests/traversal.md`.
- `00_nucleo/prompts/entities/content/traversal.md`.
- `00_nucleo/prompts/entities/content_hash.md`.
- `00_nucleo/prompts/entities/counter.md`.
- `00_nucleo/prompts/entities/counter_format.md`.
- `00_nucleo/prompts/entities/counter_registry.md`.
- `00_nucleo/prompts/entities/counter_state_legacy.md`.
- `00_nucleo/prompts/entities/counter_update.md`.
- `00_nucleo/prompts/entities/decimal.md`.
- `00_nucleo/prompts/entities/element_info.md`.
- `00_nucleo/prompts/entities/element_kind.md`.
- `00_nucleo/prompts/entities/element_payload.md`.
- `00_nucleo/prompts/entities/elements/bibliography.md`.
- `00_nucleo/prompts/entities/elements/block.md`.
- `00_nucleo/prompts/entities/elements/boxed.md`.
- `00_nucleo/prompts/entities/elements/cite.md`.
- `00_nucleo/prompts/entities/elements/columns.md`.
- `00_nucleo/prompts/entities/elements/counter_display_callback.md`.
- `00_nucleo/prompts/entities/elements/counter_update.md`.
- `00_nucleo/prompts/entities/elements/curve.md`.
- `00_nucleo/prompts/entities/elements/enum_item.md`.
- `00_nucleo/prompts/entities/elements/equation.md`.
- `00_nucleo/prompts/entities/elements/figure.md`.
- `00_nucleo/prompts/entities/elements/figure_caption.md`.
- `00_nucleo/prompts/entities/elements/footnote.md`.
- `00_nucleo/prompts/entities/elements/footnote_entry.md`.
- `00_nucleo/prompts/entities/elements/grid.md`.
- `00_nucleo/prompts/entities/elements/grid_cell.md`.
- `00_nucleo/prompts/entities/elements/grid_hline.md`.
- `00_nucleo/prompts/entities/elements/grid_vline.md`.
- `00_nucleo/prompts/entities/elements/h_space.md`.
- `00_nucleo/prompts/entities/elements/heading.md`.
- `00_nucleo/prompts/entities/elements/image.md`.
- `00_nucleo/prompts/entities/elements/label.md`.
- `00_nucleo/prompts/entities/elements/link.md`.
- `00_nucleo/prompts/entities/elements/list_item.md`.
- `00_nucleo/prompts/entities/elements/math_accent.md`.
- `00_nucleo/prompts/entities/elements/math_align_point.md`.
- `00_nucleo/prompts/entities/elements/math_attach.md`.
- `00_nucleo/prompts/entities/elements/math_cancel.md`.
- `00_nucleo/prompts/entities/elements/math_cases.md`.
- `00_nucleo/prompts/entities/elements/math_class_override.md`.
- `00_nucleo/prompts/entities/elements/math_delimited.md`.
- `00_nucleo/prompts/entities/elements/math_frac.md`.
- `00_nucleo/prompts/entities/elements/math_limits_override.md`.
- `00_nucleo/prompts/entities/elements/math_matrix.md`.
- `00_nucleo/prompts/entities/elements/math_op.md`.
- `00_nucleo/prompts/entities/elements/math_root.md`.
- `00_nucleo/prompts/entities/elements/math_stretch.md`.
- `00_nucleo/prompts/entities/elements/math_styled.md`.
- `00_nucleo/prompts/entities/elements/math_underline.md`.
- `00_nucleo/prompts/entities/elements/math_underover.md`.
- `00_nucleo/prompts/entities/elements/math_vec.md`.
- `00_nucleo/prompts/entities/elements/metadata.md`.
- `00_nucleo/prompts/entities/elements/outline.md`.
- `00_nucleo/prompts/entities/elements/outline_entry.md`.
- `00_nucleo/prompts/entities/elements/overline.md`.
- `00_nucleo/prompts/entities/elements/p1429-math-accent-tests.md`.
- `00_nucleo/prompts/entities/elements/p1430-math-cases-tests.md`.
- `00_nucleo/prompts/entities/elements/p1431-math-matrix-tests.md`.
- `00_nucleo/prompts/entities/elements/page_run.md`.
- `00_nucleo/prompts/entities/elements/pagebreak.md`.
- `00_nucleo/prompts/entities/elements/pdf_artifact.md`.
- `00_nucleo/prompts/entities/elements/pdf_attach.md`.
- `00_nucleo/prompts/entities/elements/place.md`.
- `00_nucleo/prompts/entities/elements/quote.md`.
- `00_nucleo/prompts/entities/elements/raw.md`.
- `00_nucleo/prompts/entities/elements/ref.md`.
- `00_nucleo/prompts/entities/elements/repeat.md`.
- `00_nucleo/prompts/entities/elements/shape.md`.
- `00_nucleo/prompts/entities/elements/smartquote.md`.
- `00_nucleo/prompts/entities/elements/stack.md`.
- `00_nucleo/prompts/entities/elements/state_update.md`.
- `00_nucleo/prompts/entities/elements/strike.md`.
- `00_nucleo/prompts/entities/elements/table.md`.
- `00_nucleo/prompts/entities/elements/table_cell.md`.
- `00_nucleo/prompts/entities/elements/table_hline.md`.
- `00_nucleo/prompts/entities/elements/table_vline.md`.
- `00_nucleo/prompts/entities/elements/term_item.md`.
- `00_nucleo/prompts/entities/elements/transform.md`.
- `00_nucleo/prompts/entities/elements/underline.md`.
- `00_nucleo/prompts/entities/elements/v_space.md`.
- `00_nucleo/prompts/entities/engine.md`.
- `00_nucleo/prompts/entities/enum_numbering.md`.
- `00_nucleo/prompts/entities/file-id.md`.
- `00_nucleo/prompts/entities/font-list.md`.
- `00_nucleo/prompts/entities/font_book.md`.
- `00_nucleo/prompts/entities/func.md`.
- `00_nucleo/prompts/entities/geometry.md`.
- `00_nucleo/prompts/entities/glyph_variants.md`.
- `00_nucleo/prompts/entities/gradient.md`.
- `00_nucleo/prompts/entities/html.md`.
- `00_nucleo/prompts/entities/image-format.md`.
- `00_nucleo/prompts/entities/image-sizer.md`.
- `00_nucleo/prompts/entities/introspector.md`.
- `00_nucleo/prompts/entities/label.md`.
- `00_nucleo/prompts/entities/label_registry.md`.
- `00_nucleo/prompts/entities/lang.md`.
- `00_nucleo/prompts/entities/layout_types.md`.
- `00_nucleo/prompts/entities/layouter_runtime_state.md`.
- `00_nucleo/prompts/entities/list_marker.md`.
- `00_nucleo/prompts/entities/locator.md`.
- `00_nucleo/prompts/entities/math-class.md`.
- `00_nucleo/prompts/entities/math_constants.md`.
- `00_nucleo/prompts/entities/math_style.md`.
- `00_nucleo/prompts/entities/metadata_store.md`.
- `00_nucleo/prompts/entities/module.md`.
- `00_nucleo/prompts/entities/numbering.md`.
- `00_nucleo/prompts/entities/operators.md`.
- `00_nucleo/prompts/entities/p1431-line-cap-tests.md`.
- `00_nucleo/prompts/entities/package-spec.md`.
- `00_nucleo/prompts/entities/paint.md`.
- `00_nucleo/prompts/entities/path.md`.
- `00_nucleo/prompts/entities/plugin_func.md`.
- `00_nucleo/prompts/entities/position.md`.
- `00_nucleo/prompts/entities/regex.md`.
- `00_nucleo/prompts/entities/region.md`.
- `00_nucleo/prompts/entities/resolved_label_store.md`.
- `00_nucleo/prompts/entities/scope.md`.
- `00_nucleo/prompts/entities/sealed-positions.md`.
- `00_nucleo/prompts/entities/selector.md`.
- `00_nucleo/prompts/entities/show.md`.
- `00_nucleo/prompts/entities/sides.md`.
- `00_nucleo/prompts/entities/sink.md`.
- `00_nucleo/prompts/entities/source-result.md`.
- `00_nucleo/prompts/entities/source.md`.
- `00_nucleo/prompts/entities/span.md`.
- `00_nucleo/prompts/entities/state_registry.md`.
- `00_nucleo/prompts/entities/state_update.md`.
- `00_nucleo/prompts/entities/style.md`.
- `00_nucleo/prompts/entities/style_chain.md`.
- `00_nucleo/prompts/entities/symbol.md`.
- `00_nucleo/prompts/entities/syntax-kind.md`.
- `00_nucleo/prompts/entities/syntax-mode.md`.
- `00_nucleo/prompts/entities/syntax-node.md`.
- `00_nucleo/prompts/entities/syntax-node/errors.md`.
- `00_nucleo/prompts/entities/syntax-node/linked.md`.
- `00_nucleo/prompts/entities/syntax-node/representation.md`.
- `00_nucleo/prompts/entities/syntax-set.md`.
- `00_nucleo/prompts/entities/syntax-text.md`.
- `00_nucleo/prompts/entities/tag.md`.
- `00_nucleo/prompts/entities/tiling.md`.
- `00_nucleo/prompts/entities/value.md`.
- `00_nucleo/prompts/entities/version.md`.
