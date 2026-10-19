# Prompt L0 — `Symbol` — named Unicode grapheme, modifiers and construction
Hash do Código: 1caf8923

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/symbol.rs`
**Governance:** ADR-0017, ADR-0107, ADR-0108, ADR-0029.

## Evidence before specification

Historical ratified pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`, foundations/symbol.rs:19–37,51–129,176–196,219–310,339–400, documents symbolic field access, custom constructor, sym names without prefix in math, and emoji. Probes accept one extended grapheme including woman-technologist, skin-tone and flag sequences; empty string/ab reject with `variant value must be exactly one grapheme cluster`. emoji.heart renders U+2764 U+FE0F but differs in equality from runtime symbol("❤️") despite equal codepoints; two equal runtime constructions compare equal. Concatenation retains the full cluster. Identity and variant lists are language data, not reducible to visible text. Upstream SymbolInner/Arc/static-runtime representation is mechanics.

At 2026-09-14T17:24:42-03:00, clean HEAD `762e95025a76cb8dec4b58097ca57aa878e05243`, local baseline SHA-256 `50c137a42d8a0da4ba43213c2a04e0f8896478a8fc05e25da7e9fee9247a873c`, oracle `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` at that pin, serial 512 MiB probes found excess escaping for apostrophe/double quote and abbreviated NUL. Backslash/newline/tab/carriage return/VS/ZWJ controls already agreed. Upstream symbol.rs:339–390 delegates to string repr; str.rs:718–753 defines character escaping. repr.rs:170–223 specifies pretty_array_like(...,false), byte width 50 and multiline two-space indentation/trailing commas. These are historical measurements, not current-pin certification.

## Carrier and constructors

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: EcoString,
    pub name: EcoString,
    pub variants: Vec<(EcoString, EcoString)>,
    pub applied: Vec<EcoString>,
}
```

value and every variant value contain exactly one nonempty extended grapheme, possibly multiple scalars. name is the canonical base name, empty for runtime symbols. variants pairs dotted modifiers with values; the base has empty modifiers. applied retains field-access modifiers. EcoString preserves clusters, is permitted pure L1 and clones O(1). Reject Scalar|Cluster semantic splitting, upstream static/runtime enum copying, and char-plus-suffix fragmented representation.

new(value,name) accepts value:impl Into<EcoString> and creates a simple empty-modifier variant; with_variants(value,name,variants) serves native sym/emoji tables; runtime(variants) serves the language constructor. Internal inputs are validated/static; runtime diagnostic ownership belongs to the language constructor. Value::Symbol stores this Symbol, reports type_name `symbol`, supports From<Symbol>, and retains its specified Debug-format hash integration.

modified(&str)->Option<Symbol> rejects simple symbols, appends the modifier, filters variants containing every applied modifier, chooses the fewest extra modifiers and returns updated value without normalization or loss of variation selectors/ZWJ. Bindings field access returns the resulting Value::Symbol or `unknown symbol modifier`. Global symbol is Value::Type(Type::Symbol); static type dispatch calls native_symbol, whose positional base strings or (modifiers,value) arrays must contain exactly one nonempty grapheme. Markup emits Content::Text(value.clone()); string/content/math conversion never truncates with chars().next().

## Representation

repr emits symbol(inner) from variants compatible with applied modifiers, removing those modifiers from displayed variant keys. Escape every internal string uniformly, including simple/base/variant values: apostrophe stays literal, double quote is `\"`, NUL is `\u{0}`, backslash/newline/tab/carriage return are `\\`, `\n`, `\t`, `\r`. Preserve canonical `\u{fe0f}` and `\u{200d}` without normalization. Do not special-case witness names/modules/modifiers/glyphs. The historical repair authorizes only private repr helpers/local tests, without constructor/grapheme validation/modifier/table/equality/layout/other-type repr changes.

Use compact output when the byte sum of pieces/separators fits 50; otherwise one variant per line, two-space indentation, trailing comma. emoji.heart is multiline while repr(emoji.heart.arrow) is `symbol("💘")`; short runtime symbols stay compact. Textual syntax is observable; helper placement is mechanics.

## Integration and unresolved scope

Keep SYM_SIMPLE and the measured arrow variant subset; sym_lookup("arrow.r.filled") applies modifiers sequentially. Value, bindings, closures, repr and stdlib sym/foundations are consumers with their own owners, not additional consumers of this L0. The original missing math-prefix/emoji support belongs to historical subset scope and must not erase later native constructors. A full upstream variant catalog, deprecation warnings and Symbol::func callable accents remain outside this owner slice. Preserve supplied codepoints; NFC/NFD requires new measurement and decision.

## Acceptance

```
sym.arrow.r → Value::Symbol with value="→"
sym.arrow.r.filled → Value::Symbol with value="➡"
symbol("🖂", ("stamped", "🖃")).stamped → "🖃"
symbol("♥️") → value U+2665 U+FE0F, without truncation
symbol("❤️") → value U+2764 U+FE0F, without truncation
symbol("👩‍💻"), symbol("👍🏽"), symbol("🇧🇷") → accepted
symbol(""), symbol("ab") → exactly-one-grapheme-cluster error
repr(symbol(("bold", "α"), ("italic", "α"))) == "symbol((\"bold\", \"α\"), (\"italic\", \"α\"))"
repr(symbol("'")) → symbol("'")
repr(symbol("\"")) → symbol("\"")
repr(symbol("\u{0}")) → symbol("\u{0}")
repr(symbol("\\")) → symbol("\\")
repr(symbol("\n")) → symbol("\n")
repr(symbol("\t")) → symbol("\t")
repr(symbol("\r")) → symbol("\r")
repr(symbol("👩‍💻")) → symbol("👩\u{200d}💻")
```

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/symbol.md` preserves the original evidence and chronology.
