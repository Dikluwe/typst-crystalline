# Prompt L0 — `engine/layout/heading`
Hash do Código: 48634025

**Camada**: L1 · **Alvo**: `01_core/src/engine/layout/heading.rs`
**Criado em**: 2026-06-24 (P451 — heading numbering patterns)
**ADRs relevantes**: ADR-0109 (atomização), ADR-0033 (paridade vanilla)

---

## Contexto

Layout de `Content::Heading` (via `HeadingElem`). Responsável por aplicar estilo bold escalado por nível e, opcionalmente, prefixar o body com o número do counter hierárquico.

---

## Comportamento

1. Aplicar `TextStyle` com `bold=true`, `italic=false`, tamanho escalado por `heading_scale(level)`.
2. Se `cursor_x` já passou da margem, fazer `flush_line()`.
3. Se `heading.numbering` na `StyleChain` for `Bool(true)`:
   - Ler `heading.numbering.pattern` (opcional, `EcoString`).
   - Obter os valores brutos do counter via `Introspector::counter_values_at("heading", loc)`.
   - Se houver pattern, formatar com `format_counter(values, pattern)`.
   - Se não houver pattern, usar `Introspector::formatted_counter_at("heading", loc)` (forma legada `"1.2.3"`).
   - Renderizar o prefixo numérico como `Content::text("{num_str} ")` antes do body.
4. Renderizar o `body`.
5. Restaurar estilo anterior.

## Gate de numeração

- A numeração é activa quando `chain.custom("heading.numbering") == Some(Value::Bool(true))`.
- O pattern é transportado em `chain.custom("heading.numbering.pattern")` como `Value::Str`.

## Patterns suportados

Ver prompt `entities/counter_format.md`. Subset: `"1."`, `"1.1"`, `"I."`, `"(a)"`, `"A."`.

## Scope-outs

- Alinhamento customizado do número (left/center/right/hanging indent).
- Formatação de counter via `counter.display` callback (P241) — este layout usa formatação directa.

## Tests canónicos

- Heading sem numbering: sem prefixo.
- Heading numerado com pattern `"1."`.
- Heading nível 2 com pattern `"1.1"` produz `"1.1"`.
- Reset inferior: heading nível 1 após nível 2 volta a incrementar nível 1.
- Pattern romano `"I.I"` hierárquico.

---

## Resultado esperado

- `01_core/src/engine/layout/heading.rs` — free function `layout` + tests em `01_core/src/engine/layout/tests.rs`.
