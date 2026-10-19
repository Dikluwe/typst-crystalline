# FG1 readiness and explicit Batch F boundary

Status: **not ready for approval or implementation**. V16-055 remains open.
`design-draft.md` is a frozen candidate for review, not an active L0 or a settled
architecture. Its proposal to reuse typed Selector counter keys is incomplete;
its one-pass preparation wording is superseded by the findings below. No active
Figure owner or productive Figure code was changed in Batch F.

## Measured blockers

1. **Constructor carrier loses language identity and presence.**
   `entities/elements/figure.rs` retains `kind: Option<String>`;
   `stdlib/figure_image.rs` accepts element functions but discards them into
   constructor-time body inference. Existing Figure fields do not expose kind.
   Geometry's `../agents/geometry/baseline-query/receipt.json` records ten realized
   queries rejected by the product while the pinned oracle returns typed kind.
   An enum arm list cannot repair that carrier loss. The candidate FigureKind,
   lexical omitted/auto/explicit state and occurrence resolution are public-field
   changes under ADR-0127; they have not been approved.

2. **Selection and final preparation are distinct observations.**
   Pinned `typst-realize/src/lib.rs:446–456` pre-synthesizes a clone for selector
   matching; `:557–575` applies matched show-set styles before final synthesis and
   materialization. Current `compiler/realize/recipes.rs:410–444` filters rules
   before `compiler/eval/rules/apply.rs:600–624` accumulates show-set styles.
   `oracle-boundaries.json` confirms a figure inferred as table matches
   `figure.where(kind: table)` setting kind:raw and the show callback sees raw.
   A final contract must specify a pure selection preview followed by final
   occurrence resolution, preserving aliases, guards and hidden-parent effects.
   Simply preparing once before or after selection is insufficient.

3. **Semantic inference is not the existing visual/static visitor.**
   Pinned `foundations/content/mod.rs:408–464` visits public Content-valued fields
   and recursively nested Arrays, but not Dictionaries. The new oracle receipt
   confirms Metadata containing Table directly or inside Arrays infers table;
   a Dictionary containing the same Table is opaque. First-field order changes
   the choice relative to a following Raw. Context-produced and later show-produced
   Table remain image in these observations. Existing
   `entities/content/traversal.rs::visit_content_static` intentionally treats
   Metadata/Value and callback records as opaque and uses active visual views.
   Reusing it would silently lose measured semantics. The field/child partition
   and its ownership must be completed before an inference implementation is
   authorized; no currently unreviewed container may be labelled a fake leaf.

4. **Typed key replacement alone breaks caption/reference counting.**
   The independent review in `../agents/adapters/fg1-review.json` describes the
   graph. Filtered counter callback actions are resolved by
   `compiler/introspect/from_tags/counter_updates.rs::resolve_filtered_counter`;
   `apply_counter_funcs` skips Element-containing filtered keys. Manual Set/Step
   are already materialized by populate; the earlier claim that all manual
   updates are skipped was incorrect. Caption/reference readers use registry
   snapshots and figure_number_at_index, whose event index must not be confused
   with the index of a Figure occurrence when manual events are interleaved.
   Current `default_supplement_for_key` also does not map Selector::Element.
   A future typed-key contract must identify one coherent history and resolve
   nth-Figure Location before prefix lookup, without adding automatic events or
   invoking callbacks twice.

5. **Counter history demand cannot be made eager for every Figure.**
   The oracle receipt's callback-with-caption panics through the filtered counter
   update; no-caption, numbering:none and show-hidden controls do not. A generic
   pre-layout fold for every realized kind would change observable effects.
   Any shared materialization design must preserve demand, causal order, complete
   demanded-history validation, Context::none callback context and reuse/replay.
   That phase/effect contract and its full owner graph are not finished here.

6. **Custom names require an explicit supplement boundary.**
   Name("table") is not the table element. The oracle requires a supplement for
   the custom name under default numbering; an explicit supplement or
   numbering:none makes the same typed name valid. `figure_supplement`'s current
   name-based lookup cannot establish element identity. The candidate must either
   include measured supplement transport/synthesis with its public fields and
   callback phase, or explicitly retain that behavior as unresolved without
   asserting full FG1 parity. Caption omission/none, position/separator presence,
   numbering/supplement callbacks and figure_location remain separately incomplete.

## Concrete next work before a permission question

Retain the candidate typed FigureKind and lexical/occurrence distinction only as
an option under review. Complete an exact source/owner manifest covering the
constructor, entity, reconstruction, reflection, reuse, realization, payload,
introspection, counters, layout/reference and L3 trait implementors. Revise the
candidate L0 amendments around selection preview versus final resolution and
one demand-preserving counter history; measure negative controls before deciding.
A different public design must be reviewed against these same observations.
Only then produce a hashed review package and ask the human to approve the exact
public fields, trait signature, compatibility and phase changes. Existing Context
approval does not authorize Figure changes.

This is intentionally a blocked Figure subset, not a request for vague permission.
The Batch F repairs outside Figure proceed independently with their own gates.

## Provenance and limits

`oracle-boundaries.json` records exact source/argv/UTC, HEAD, uncommitted diffstat,
runner and binary hashes for 18 oracle-only probes against commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
It is not a new product comparison. `counter-update-prefix` returned an empty
query because its label fixture did not attach to the emitted Metadata; it is
not evidence about numbering and must be corrected before reuse. All other
reported findings identify their concrete cases or source boundaries.
