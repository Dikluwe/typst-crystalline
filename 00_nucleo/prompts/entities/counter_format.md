# Prompt L0 — `entities/counter_format`
Hash do Código: 98d2d013

**Camada**: L1 · **Alvo**: `01_core/src/entities/counter_format.rs`
**Criado em**: 2026-06-24 (P451 — heading numbering patterns)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0109 (counters hierárquicos)

---

## Contexto

`CounterRegistry` fornece valores hierárquicos de counters (ex.: `[1, 2, 1]` para headings). Este módulo aplica **patterns de formatação** a esses valores, permitindo renderizar prefixos como `"1."`, `"I."`, `"(a)"` ou `"A."`.

---

## Interface pública

```rust
/// Formata um vector de valores hierárquicos segundo um pattern.
pub fn format_counter(values: &[usize], pattern: &str) -> Option<String>;
```

## Semântica

- Cada caractere do pattern que for um **token** (`1`, `I`, `a`, `A`) é substituído pela representação do valor do nível correspondente.
- Caracteres não-token são copiados literalmente (prefixos, sufixos, separadores).
- Tokens são consumidos em ordem: o primeiro token usa `values[0]`, o segundo `values[1]`, etc.
- Se `values` estiver vazio, ou se o pattern não contiver nenhum token reconhecido, retorna `None`.
- Se houver mais tokens do que `values` disponíveis, retorna `None`.

## Tokens suportados

| Token | Significado | Exemplos |
|-------|-------------|----------|
| `1` | Algarismos arábicos | `1`, `2`, `10` |
| `I` | Romanos maiúsculos (até 3999; fallback arábico fora do range) | `I`, `IV`, `XLIX` |
| `a` | Letras minúsculas (a=1, ..., z=26, aa=27, ...) | `a`, `z`, `aa` |
| `A` | Letras maiúsculas | `A`, `Z`, `AA` |

## Exemplos

```
[1] + "1."      → "1."
[1, 2] + "1.1"  → "1.2"
[4] + "I."      → "IV."
[2] + "(a)"     → "(b)"
[3] + "A."      → "C."
[2, 3] + "I.1"  → "II.3"
```

## Scope-outs

- Círculos numerados (`"①"`), kanji (`"一"`), etc.
- Patterns com repetição/posicionamento não-linear dos tokens.

## Tests obrigatórios

- Pattern `"1."` com nível 1 e 2.
- Pattern `"1.1"` hierárquico.
- Pattern `"I."` para romanos.
- Pattern `"(a)"` para letras minúsculas (incluindo wrap `aa`).
- Pattern `"A."` para letras maiúsculas.
- Valores insuficientes retornam `None`.

---

## Resultado esperado

- `01_core/src/entities/counter_format.rs` — função pura + tests.
- Re-export em `01_core/src/entities/mod.rs`.
