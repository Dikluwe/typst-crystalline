# Prompt L0 — Style and Styles
Hash do Código: 0796c486

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:c8570ec5f4f32c0805587230cafc796f35e5ee7c9f8e634e13d807db2afb6e00

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/style.rs`

## Language evidence and responsibility

Define pure typed Style vocabulary and the Styles facade for set/show and Content::Styled (ADR-0038). Historical upstream `e0e8ca4d:text/mod.rs:223–226,249–252,292,314,473` defines text.style (Normal; italic/oblique), weight (Regular; numeric/name), size 11pt, fill black and lang en. There is no text.bold/text.italic. A 2026-08-13 probe with #set text(bold:true) yielded `error: unexpected argument: bold` at column 10 in both historical oracle e0e8ca4d and local HEAD 4f64e4e69. The original labels were documentary errors; bool representations are an explicit simplification, with Weight covering numeric weight partly. Preserve size/fill/lang reference defaults; the historically escalated local lang divergence is not declared closed here.

## Style vocabulary

Preserve Debug/Clone/PartialEq without Copy, because FontList is not Copy. The original ten-variant count and claims about completed migration batches are historical, not limits on this declaration.

- Bold(bool): internal boolean weight projection, not a new language property.
- Italic(bool): internal style projection; it does not independently encode all oblique distinctions.
- Size(Length): full absolute+em text size under the ratified affine contract below.
- Fill(Color): text fill; HeadingLevel(u8): heading level.
- Lang(Lang): text language, also constructible through Styled/Styles, not only parsed set rules.
- Weight(u16): raw numeric weight channel (CSS/OpenType0–1000 vocabulary); no new validation is authorized by this data-owner slice. Programmatic Styled Weight uses the same faux-bold consumer as parsed input.
- Tracking(Length): absolute+em tracking, including programmatic 0.1em. Layout captures it in FrameItem::Text.style; PDF Tc emission reads that captured TextStyle, never a live style chain.
- Leading(Length): programmatic leading, including 0.65em. The documented per-line consumer peeks the last Text item before draining current_line, resolves leading against font size and advances cursor_y by line height plus leading. The original storage as text rather than upstream par is a declared compatibility scope, not a reason to rename the language property silently. Emit does not compute leading.
- Font(FontList): retain the non-Copy list and clone when folding; programmatic FontList::single("Inter") uses the same channel. Layout captures it; the exporter resolves the captured family through its font list and selects /F{i+1} Tf. No new live-chain dependency is introduced.
- Subscript(bool), Superscript(bool): text sub/super use styles, not new Content variants. Their separately owned layout path applies size*=0.6 and baseline offset.
- Highlight(Option<Color>): Some enables a run background, None explicitly disables inheritance. Layout emits a rectangular Shape behind runs with resolved highlight.

These consumers illustrate ADR-0098's shared resolved-style boundary; historic unchanged exporter hashes/migration counts are not current validation evidence.

## Single Styles backing

Styles wraps exactly one StyleDelta, with Debug/Clone/Default/PartialEq. This replaces the historical Vec<Style>/StyleDelta dual representation. Style remains constructor vocabulary; folding is ordered, last-write-wins per field except the explicitly owned size-chain composition outside this collection. No second backing, redundant conversion or duplicate fold.

Preserve new(), push(Style), inherent from_iter<I:IntoIterator<Item=Style>>, is_empty() delegating to StyleDelta, delta()->&StyleDelta, from_delta(StyleDelta)->Self, push_custom(key,value)->Self and is_semantically_empty()->bool. from_delta wraps an already folded show-set delta. push_custom appends/folds the existing (EcoString,Value) channel, mirroring the chain's mechanism without a new Style variant. is_semantically_empty tests absence of typed properties while ignoring custom, distinguishing semantic bold/italic wrappers from pure custom transport in morphology. iter()/len() over an old Style Vec remain removed.

StyleChain::push_styles clones the delta without reimplementing Style projection; show detection reads delta.bold/italic, not a variant iterator. Content::Styled's physical carrier follows its own current owner; the old Box tuple example is historical. The former backing/baked dual-path migration schedule does not authorize reintroducing a second backing. Keep no I/O, LazyHash dependency or custom proc macros.

## Ratified FromIterator and adapter evidence

The formatting package's base.json.gz freezes the four affected implementations/L0s/HEAD/tree/hashes/UTC/diffstat; Option parsers and domain next methods were classified separately. The duration probe's i128 correction is distinct. Owner approval on 2026-10-16, manifest `d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`, is in `lab/results/construction/diagnostics/clippy-enums-2026-10-16/approval-receipt.json`. It covers three Display impls, FromIterator and inherent-to-blanket ToString migrations plus T1–T5, no release. Generic trait users make rollback a compatibility concern.

Implement FromIterator<Style> with the same ordered fold into StyleDelta, preserving empty/custom/last-write-wins and all other APIs. Keep inherent from_iter as a complete adapter to <Self as FromIterator<Style>>::from_iter, with no additional clone/reorder/backing. Do not add FromStr, fields/variants/flags/defaults/phases/dependencies or unrelated suppressions.

The prediction that implementing the trait would remove the inherent adapter warning was refuted by Clippy/Rust1.92. first-validation/validation-clippy.log still reports should_implement_trait while four approved_traits tests pass (including empty collect and ordered Weight); traits-green-result.json and first-validation/validation-details.json preserve argv/UTC/cgroup/status/source hashes/HEAD/tree/diffstat. Keep the approved adapter unchanged and apply only `#[expect(clippy::should_implement_trait, reason="S005: adapter inerente de compatibilidade; FromIterator<Style> implementado e validado por collect")]` there. No suppression on trait/impl/module/crate or duplicate fold. Require unfulfilled_lint_expectations as error and retain both collect/inherent tests. This measured metadata correction supersedes only the no-expect prediction, without changing public approval.

## Full affine text-size evidence and contract

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/{before,before-measurement,morph-before-measurement}.json.gz` and audit.json preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat/source/L0/binaries/argv at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures0pt versus 24pt, affine3pt versus 27pt, nested over 18pt0pt versus 36pt. All six Size/Color/body permutations succeed upstream and fail locally; positional set18pt is ignored (12 versus 18). Upstream text/mod.rs:290,303,924–930,1136–1142 and args.rs:178–247 establish typed lookup/named preference/remaining body/composition. Local style_chain.rs:513–517 and constructor.rs:123 discard em; collapse:394–395,419–422 uses top-wins instead of composition. Restoration/Raw/heading/math/capture/show/duplicates/spans/zero/negative/metrics can refute the correction; no historic intent is inferred.

The equality probe refutes comparing declared size values: upstream text(size:2em)[A]==text(size:24pt)[A] is true and content/mod.rs:779–782 compares Styled child. This separate equality debt is not closed by the complete type. Owner approved TS1's two public contracts, manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`, on 2026-10-17, including S1–S16 under 12 GiB/swap 0, no release/merge/push. RG1 remains independent.

Style::Size carries the full Length; fold_into transfers it to StyleDelta.size. Preserve the single collection/backing, all other variants/fields/methods; no parallel size or fake em marker. Existing absolute constructors migrate explicitly to Length::pt(pt.val()). Reseal only after implementation and a valid ownership/nucleus graph.

## Acceptance

For traits, T1 generic Display+ToString and collect::<Styles> must be RED before missing traits and GREEN afterward. T2 preserves Decimal1.2300 zeros/negative/zero/Option rejection and Version empty/1.2.0. T3 preserves corrected Duration MIN/MAX/MIN+1/zero/negative boundaries through format!/ToString. T4 empty collect stays empty and Weight400 then 700 yields Some700 with all other fields unchanged; inherent calls remain compatible. T5 preserves language/casts/repr/show/CLI and full workspace/build/Clippy/expectations/Rustdoc/fmt/lineage,12 GiB/swap 0/one-job and unchanged expected results. For the adapter correction use L0 first/restricted reseal/full gates, without changing the approved interface. For Size retain the ratified S1–S16 scope and separate equality/remaining-size debts; representation alone is not language or global performance proof.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/style.md` preserves the original evidence and chronology.
