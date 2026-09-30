# L0 — compiler/eval/font_dict
Hash do Código: 54229e95

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/font_dict.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/font_dict.rs:28` — `fn parse_font_dict_named_fields`.
- `01_core/src/compiler/eval/font_dict.rs:164` — `fn parse_font_dict_legacy`.
- `01_core/src/compiler/eval/font_dict.rs:235` — `fn variants_from_value`.

## Contract

Own AST font-dictionary parsing and variants_from_value. The named form accepts
family/variant/weight/style/fallback with identifier keys, validates required
family and rejects unknown names. family accepts Str or Regex; variant/style are
Str, weight is Int converted to a string or Str, and fallback is Bool with default
true. Preserve existing type, missing-field and fallback diagnostic literals.
The named form produces one dictionary with name, an empty variants array and
optional variant/weight/style. The legacy form produces one dictionary per family
with name and variants; variants_from_value accepts Str or an array of Str and
rejects nonstring members at the supplied span.

The legacy form retains string/identifier keys and its existing regex-call key
forms and string/array values. Reject unsupported spreads and empty dictionaries
as currently specified. Evaluate field expressions once with current scopes,
context and Engine; expression side effects remain real and ordered.
fallback:false retains its current normalized-list behavior, not a new error.
Font resolution/layout is not owned here; preserve parse_font_dict_named_fields,
parse_font_dict_legacy and variants_from_value visibility and signatures.
