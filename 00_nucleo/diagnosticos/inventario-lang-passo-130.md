# Passo 130.A — Inventário `text.lang` (DEBT-1 subset)

**Data**: 2026-04-24

---

## Parte 1 — Vanilla `lang`

**Ficheiro**: `lab/typst-original/crates/typst-library/src/text/mod.rs:440`
e `text/lang.rs:123+`.

```rust
pub lang: Lang,                                // TextElem
pub struct Lang { /* ... */ }                  // BCP 47 wrapper
impl Lang {
    pub const ENGLISH: Self;
    pub const fn new(lang: Lang, region: Option<Region>) -> Self;
    pub fn as_str(&self) -> &str;
}
fn lang_str(lang: Lang, region: Option<Region>) -> EcoString;
```

Vanilla usa **tipo próprio `Lang`** com parser BCP 47,
validação, e constantes. `EcoString` é usado apenas para
serialização (output).

## Parte 2 — `Value::Str` em L1

**Ficheiro**: `01_core/src/entities/value.rs:29`.

```rust
Str(EcoString),
```

Directo. `if let Value::Str(s) = val` liga `s: EcoString`
sem cast.

## Parte 3 — `EcoString` em L1

**Não importado** em `entities/style_chain.rs` nem
`rules/eval/rules.rs` hoje. `Value::Str(EcoString)` vive em
`entities/value.rs`. Precisa `use ecow::EcoString;`.

ADR-0024 autoriza `EcoString` em L1 (Value::Str especialmente).
CLAUDE.md nota L1 pode usar `ecow` via ADR-0024.

## Parte 4 — DEBT-1 em `StyleDelta` actual

**Ficheiro**: `01_core/src/entities/style_chain.rs:28-50`.

Campos actuais: `bold, italic, size, fill, heading_level,
weight, tracking, leading` — 8 campos. Adicionar `lang` =
9 campos.

## Parte 5 — DEBT-49 L3 pool

**Ficheiro**: `03_infra/src/integration_tests.rs`.

Dois tests afectados pela activação de `lang`:

1. **`debt49_set_text_lang_emite_warning` (2203)**: input
   `"#set text(lang: \"pt\")"`; asserta 1 warning com `'lang'`.
   **Quebra** quando `lang` for capturado → precisa rotar para
   outra propriedade desconhecida.

2. **`debt49_set_text_multiplas_propriedades_desconhecidas`
   (2222)**: input `"font: \"A\", lang: \"pt\", stroke: 1pt"`;
   asserta 3 warnings. **Quebra** se `lang` sai → precisa
   substituir `lang` por outra desconhecida.

### Pool de propriedades ainda desconhecidas

Após 130 (lang capturado):
- `font` (ainda unknown)
- `stroke`, `alignment`, `hyphenate`, `justify`, `first-line-indent`,
  `dir`, `region`, `spacing`, `discretionary-ligatures`,
  `number-type`, `number-width`, `slashed-zero`, `fractions`,
  `overhang`, `top-edge`, `bottom-edge`, `kerning`,
  `stylistic-set`...

Pool saudável (> 15 candidatas desconhecidas). Substituição
por `alignment` (vanilla: `align: Alignment`) é natural — nome
ainda desconhecido em cristalino.

## Parte 6 — `Value::Str` precisa de cast?

Match `Value::Str(s)` → `s: EcoString`. `delta.lang` será
`Option<EcoString>`. Arm é:

```rust
"lang" => {
    if let Value::Str(s) = val {
        delta.lang = Some(s);   // move directo
    }
}
```

Sem `.clone()` — `val` consumido pelo match.

---

## Decisões

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Tipo em StyleDelta | `Option<EcoString>` | `Value::Str` já é `EcoString` — zero cast; clone O(1) |
| Validação BCP 47 | **Não** | Vanilla tem `Lang` struct com validação; cristalino captura raw, defer validação para consumer futuro |
| Region / script / dir | **Fora escopo** | Passo separado quando chegar |
| Arm pattern | Variante 1 (primitivo simples) | `Value::Str` → `EcoString` move directo |
| ADR-0038 | **Sem anotação** | Quinta aplicação literal; variante 1 (primitivo); ADR já tem 3 notas cobrindo o espaço |
| Rotação DEBT-49 | **Obrigatória** em 2 tests L3 | `lang` sai do pool |

### Divergência vanilla

Cristalino **não valida BCP 47**. Aceita `#set text(lang: "xx-invalid-nonsense")`
sem erro. Categoria ADR-0033: **semântica suave** — consumer
futuro (shaping, hyphenation) valida/normaliza ao usar.

## Gate 130.A

**Passa**. 3 ficheiros tocados:
- `entities/style_chain.rs` (+1 campo, +import).
- `rules/eval/rules.rs` (+1 arm).
- `integration_tests.rs` (rotação em 2 testes — substituir
  `lang` por `alignment`).

Tests +3 em L1. Zero ripple em L2/L4.

---

## Rotação DEBT-49

- `debt49_set_text_lang_emite_warning` → renomear para
  `debt49_set_text_alignment_emite_warning`; input
  `#set text(alignment: center)`.
- `debt49_set_text_multiplas_propriedades_desconhecidas`
  input `"font: \"A\", lang: \"pt\", stroke: 1pt"` → trocar
  `lang: "pt"` por `alignment: center`. Actualizar
  assertions.

(Manter teste estrutural em `alignment` → próximo passo de
rotação será quando `alignment` sair do pool.)
