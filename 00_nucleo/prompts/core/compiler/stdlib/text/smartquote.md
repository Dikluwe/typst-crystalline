# L0 — stdlib/text/smartquote
Hash do Código: b0bd08f3

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/smartquote.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c

## Evidence before specification

`01_core/src/compiler/stdlib/text/smartquote.rs:37` establishes exclusive L1 ownership, not new bilateral measurement/upstream intent. sanitization-parser-stdlib-history.md preserves prior provenance. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz preserves original oracle pin/binary SHA, HEAD/working tree/diffstat/UTC/sources/argv/cwd/environment/stdout-stderr. Markup eval loses SmartQuote fields and reused quotes retain English glyphs in German/French/disabled contexts; primes become quotes. Prior native-show/name correction passes independently. Oracle empty constructor has empty fields; omitted double is Unset, explicit false known, disabled remains SmartQuote. Filtered show/set-double have positive probes. Literal HTML receives Text, programmatic quote rejected; literal title versus script-string rejection are distinct controls. Source anchors markup_stream.rs:79-128/layout smartquote flags/constructor 112-117/observation 242-250 versus pinned markup.rs:141-146, inline collect 196-211/text smartquote 119-158.

Human ratification 2026-10-06 authorizes double bool→Option<bool>, enabled:Option<bool>, one pure helper/context and glyph selection eval→each target render, superseding only earlier SmartQuote conflicts. No unrelated ADR/nucleus/proposal is ratified by this scope.

## Exclusive current contract

Preserve native_smartquote signature/named-only validation. Zero positionals; retain exact `smartquote() não aceita argumentos posicionais (recebeu {n})`, `smartquote(): argumento nomeado inesperado '{name}'` and parameter-specific bool diagnostics. double/enabled absent→None, explicit Bool→Some(bool); alternative Bool default false and quotes Auto|string|array|dictionary retain helper validation. Always return semantic Content::SmartQuote including enabled:false, never disabled literal Text or Styled wrapper. Do not read engine/style/language to fill omitted fields or force location; fields exposes only explicit known values and public content.func identity stays smart.quote. Preserve absent versus explicit quotes:auto erasing inherited override. Set uses Styles; layout owns direct/set/default inheritance/glyph/open-close state.

Shared localized-quotes validation: string exactly two Unicode grapheme clusters, array exactly two items, dictionary only single/double each Auto/string/array. Preserve exact `expected 2 characters, found {n} character{s}`, `expected 2 quotes, found {n} quote{s}` and `unexpected key "{key}", valid keys are "double" and "single"`. Explicit quotes precedes alternative, absent/Auto members choose localized pairs. Constructor neither updates quoter nor resolves glyphs.

Semantic identity/fields remain in eval; paged and HTML resolve in their own render composition, HTML never paginated layout (ADR-0128). One pure L1 contextual helper/per-flow state, no I/O/global/vtable/property registry/upstream-memoization requirement. HTML escaping remains exporter-owned. No new modes/flags/unsafe/watch/math phase or contextual show evaluator. Known/omitted fields, glyphs/style/language are acceptance; representation/stack mechanics may differ. Do not infer unmeasured BCP-47-region/field coverage from repr transparency.

## Acceptance and limits

Require ratified RED→GREEN for explicit/omitted/disabled identity/fields/types/errors/spans/inheritance/localized/custom quotes, target boundaries and contextual glyphs, preserving prior correct outputs. Build/suites/lineage/pins and canonical ownership remain valid. This documentation operation changes no Rust or unrelated public contract; general bidi/locale/region parity and unrelated proposals remain unapproved by this ratification.
