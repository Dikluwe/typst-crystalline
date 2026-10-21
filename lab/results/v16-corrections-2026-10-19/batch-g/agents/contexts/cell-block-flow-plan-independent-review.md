# Independent standalone Cell composition review

The bounded plan has no architectural blocker. The frozen actual CLI reports exactly one required morphology failure: `bundle-cell-heading` emits `body/p/h2(H)` while the pinned oracle emits `body/h2(H)`. Exit, error/warning payloads and parsed diagnostic checks pass. The original semantic DOM policy remains binding.

The source cause is local composition: `render_body` wraps an unclassified nominal Cell in a paragraph, then `inline.rs:110` exposes its selected `standalone.visual` or body, already containing h2. Delegating the same selected borrowed child in block detection, direct composition and sequence accumulation corrects the existing HTML flow. This is ADR-0127 continuous internal parity work, without a new public contract, mode/default, phase or compatibility promise.

Keep Cell nominal during global projection and inside Table rows. The row renderer deliberately selects resolved visual first and retains td/th/span metadata; ordinary inline Cell selects standalone visual first. Retain lexical styles, the existing paragraph quote context and reset rules, unsupported-producer errors and explicit-root recognition. No global mapper or table renderer change is justified.

The four tests add two causal controls and two preservation controls; all prior source bytes remain an exact prefix. They cover direct h2, mixed paragraphs/rich heading, inline Strong and td/span boundaries. They do not independently certify conflicting snapshot precedence, Styled lexical changes or SmartQuote glyphs. Those remain source-review and existing-suite obligations, not new full coverage claims.

The owning L0 now states evidence before its bounded contract. Production has not changed in the reviewed snapshot. Actual functional RED/GREEN and all source-bound revalidation are still pending; the prior full12 PASS is historical after the required CLI failure. Full provenance and SHA-256 inputs are in the adjacent JSON.
