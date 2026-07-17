# Relatório — P451 Heading Numbering

**Passo:** 451  
**Data:** 2026-06-24  
**Foco:** Heading numbering com patterns configuráveis (`"1."`, `"1.1"`, `"I."`, `"(a)"`, `"A."`) e função nativa `heading(level, body, numbering?)`.

---

## Sumário executivo

O P451 materializou a camada final de formatação de números de cabeçalho. A infraestrutura de contador hierárquico (`CounterRegistry`) e o gate de numeração na `StyleChain` já existiam; este passo acrescentou:

1. **Conversão de valores em strings formatadas** (`entities/counter_format.rs`).
2. **Transporte do pattern string** através da chain (`heading.numbering.pattern`).
3. **Layout sensível ao pattern** com fallback para formatação histórica.
4. **`native_heading(level, body, numbering:?)`** como função direta.

---

## Mudanças realizadas

### Código

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/counter_format.rs` | **Novo**. `format_counter(values, pattern)` com tokens `1`, `I`, `a`, `A`; romanos até 3999; letras com wrap `aa`/`AB`. |
| `01_core/src/entities/mod.rs` | Re-exporta `counter_format`. |
| `01_core/src/entities/content.rs` | `Content::heading_numbered_with_pattern(level, body, pattern)`; `heading_numbered` mantido como wrapper sem pattern. |
| `01_core/src/entities/introspector.rs` | Novo método `counter_values_at(key, loc)` no trait `Introspector` e em `TagIntrospector`. |
| `01_core/src/engine/eval/rules.rs` | `#set heading(numbering: "...")` agora empurra também `heading.numbering.pattern`. |
| `01_core/src/engine/layout/heading.rs` | Lê pattern da chain; formata via `format_counter`; fallback para `formatted_counter_at`. |
| `01_core/src/engine/stdlib/structural.rs` | `native_heading(level, body, numbering:?)` substitui o sentinel que retornava erro. |
| `01_core/src/engine/layout/tests.rs` | 3 tests E2E P451. |
| `01_core/src/engine/stdlib/structural.rs` | 4 tests L2 de `native_heading`. |
| `03_infra/src/measurements.rs` | Adiciona `counter_values_at` ao `CountingIntrospector` e actualiza contadores de métodos (24 → 25). |

### Specs L0

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/entities/counter_format.md` | **Novo**. API e semântica de `format_counter`. |
| `00_nucleo/prompts/engine/layout/heading.md` | **Novo**. Layout de heading com pattern. |
| `00_nucleo/prompts/engine/stdlib/structural.md` | Actualizada: `native_heading(level, body, numbering:?)`. |

---

## Critério de fecho

- [x] `Counter` implementado com `step` e `display` — equivalência via `CounterRegistry` + `format_counter`.
- [x] 5+ testes L1 de counter verdes (`counter_format`: 9 tests).
- [x] `native_heading` aceita `numbering: Option<EcoString>`.
- [x] `EvalContext` / chain mantém `heading.numbering` e propaga pattern para layout.
- [x] Layout renderiza prefixo numérico antes do body do heading.
- [x] 3 tests L3 de layout E2E verdes.
- [x] Spec L0 actualizada (3 prompts).
- [x] `cargo test --workspace` verde (excepto stack overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, confirmado independente deste passo).
- [x] `crystalline-lint` zero violações novas (apenas warnings órfãos pré-existentes).

---

## Notas técnicas

### Decisão arquitectónica: adaptar ao invés de reescrever

O P451 original propunha um `Counter` próprio no `EvalContext` e um campo `numbering` em `HeadingElem`. No entanto, o repositório já tinha evoluído para:

- `CounterRegistry` no `TagIntrospector` (history location-aware, P177/P184C).
- Gate de numeração via `StyleChain` (`heading.numbering`, F-5a de-bake P364).

Reescrever essa infraestrutura seria regresso e risco de regressões. Optou-se por:

- Reutilizar `CounterRegistry` e `StyleChain`.
- Adicionar `format_counter` como camada de apresentação.
- Transportar o pattern numa segunda chave custom (`heading.numbering.pattern`).

### Formatação com pattern

A função `format_counter` substitui tokens do pattern pelos valores hierárquicos correspondentes:

```rust
format_counter(&[1, 2], "1.1")  // "1.2"
format_counter(&[4], "I.")      // "IV."
format_counter(&[2], "(a)")     // "(b)"
format_counter(&[3], "A.")      // "C."
```

Se o pattern tiver mais tokens do que `values` disponíveis, retorna `None`; o layout faz fallback para `formatted_counter_at`, preservando o comportamento histórico com ponto final.

### `native_heading`

A função nativa passou de sentinel de erro para construtor direto:

```typst
heading(1, [Título])           // heading não numerado
heading(2, "Sub", numbering: "1.1")  // heading numerado com pattern
```

Mantém-se o registo no scope como selector para `#show heading: it => ...`.

---

## Testes adicionados

### L1 — `counter_format`

- `format_1_ponto`, `format_1_ponto_1`
- `format_romano`
- `format_letras_minusculas` / `format_letras_maiusculas`
- `format_hierarquico_misto`
- `valores_insuficientes_devolve_none`

### L2 — `native_heading`

- `native_heading_sem_numbering_emite_heading_simples`
- `native_heading_com_numbering_emite_styled`
- `native_heading_rejeita_level_fora_do_range`
- `native_heading_rejeita_body_invalido`

### L3 — layout E2E

- `p451_layout_heading_pattern_1_ponto`
- `p451_layout_heading_pattern_1_ponto_1`
- `p451_layout_heading_pattern_romano_reseta_inferior`

---

## Resultados de validação

```bash
$ cargo test --workspace --no-fail-fast -- --skip p350c_flag_on_nao_convergente_classifica
# 3212 passed; 0 failed (typst-core)
# 497 passed; 0 failed (typst-infra)
# 24 passed; 0 failed (typst-shell)
# 21 passed; 0 failed (04_wiring integration)
# 2 passed; 0 failed (crystalline_lint)

$ crystalline-lint .
# 0 drift / 0 errors
# apenas warnings órfãos pré-existentes (adr-stub-vs-fallback.md, show-regex.md)
```

O único teste que falha em `cargo test --workspace` é `p350c_flag_on_nao_convergente_classifica` (stack overflow), confirmado como pré-existente via `git stash`.

---

## Commits

- `P451 — Heading numbering: patterns configuráveis + native_heading(numbering?)`
