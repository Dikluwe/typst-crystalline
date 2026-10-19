# Static diagnostic field audit

Current inputs and oracle provenance are in `warning-additional-fields.json` and
`warning-additional-fields-controls.json`. These are language-warning diagnostics;
shared visitor recursion is implementation evidence, not a parity contract.

The existing typed Content/Option<Content>/Vec<Content> fields were compared with
`entities/content/traversal.rs:35–340` using `rg` over all element files. Static
bodies in ordinary containers, math nodes, Figure caption, FigureCaption separator,
Cite supplement, FootnoteEntry note/separator, Grid/Table children, Outline/Entry,
Document body/title have existing visitor arms. Ref.supplement is the concrete
missing static typed field. Known native RawLineElem.body is also omitted because
it is carried in Dynamic; native producer is `elements/raw.rs:81–113`. Unknown
Dynamic extensions remain opaque without a public-field enumeration contract.
State.init and Image/Shape dimension Values are not generic content children;
Metadata.value needs the already measured Content/Array-specific scan.

PageRun is the concrete overtraversal: static body is relevant, but header/footer,
supplement/background/foreground are deferred and all measured oracle negatives.
SetPage is already a leaf in the shared visitor. Constructor Heading supplement
lives in Styles rather than the Heading struct; the existing marker alone is insufficient after merging a numbering-only
constructor with set styles, as warning-heading-inherited.json demonstrates.
A per-field private explicit-argument snapshot is now specified in the shared
heading-constructor-supplement nucleus. Generic Style values must remain opaque. Figure producer discards supplement; Equation's constructor and set-rule
styles use the same key without a constructor marker. Those producer dependencies
are explicitly retained in the current owning L0, with exact owner paths and a
refuter, and no complete field-parity claim is made.

Potential extra differences in derived Outline/Table payloads and arbitrary
extensions are unmeasured. Do not use this bounded audit as proof that all shared
visitor children equal upstream public fields. No shared traversal or producer
was modified by this diagnostic correction.
