# Passo 1032 — Relatório: fatiamento de `stdlib::foundations`

> **Estado: COMPLETO.** `foundations.rs` monolítico fatiado; débito do nó `regex` resolvido.
> **Build:** `cargo build -p typst-core` ok (28 warnings pré-existentes, 0 erros).
> **Testes:** `cargo test --workspace` ok (~5855 passed, 0 failed).
> **Lint:** `crystalline-lint .` → 0 violations (3 warnings V7 pré-existentes).
> **Hash drift:** `crystalline-lint --fix-hashes .` → 0 drift warnings.

---

## Contexto

`01_core/src/compiler/stdlib/foundations.rs` era o maior hub stdlib por fatiar (~89 KB, 2403 linhas), agregando funções nativas fundamentais, constructors de cor, runtime state/counter e introspecção. O Passo 1022 deixou em aberto o débito do nó `regex`, então em `stdlib/text/regex.rs` mas com domínio real em `foundations/str.rs` no vanilla.

## Inventário medido

Inventário completo via `grep -nE '^(pub(\([a-z:) ]+\))? )?fn '`:

- 41 funções públicas + helpers privados em `foundations.rs`.
- 0 `trait`/interface (critério-zero: agregado plano).
- 4 módulos de teste unitário, total de **27 `#[test]`**.

## Aplicação dos 4 critérios

### Critério 3 — co-mudança histórica (decisivo)

Ferramenta: `tools/analysis/cochange_metrics.py 01_core/src/compiler/stdlib/foundations.rs '(rules|engine|compiler)/stdlib/foundations\.rs$'`.

| Cluster medido | Nó resultante |
|---|---|
| P256-257 + P365 — constructors de cor (`as_f32`, `native_cmyk`, `native_hsl`, `native_hsv`, `native_linear_rgb`, `native_luma`, `native_oklab`, `native_oklch` + mix de cores/lumas) | `foundations/color.rs` |
| P171 + P233-238 — state (`native_state`+`native_state_update`; `native_state_at`+`native_state_final`) | absorvido por `state.rs` |
| P1018 + P176-180 + P321 — counter (`native_counter_at`+`native_counter_final`; `native_counter_display`+`native_counter_step`+`native_metadata`+state) | absorvido por `counter.rs` |
| P207-215 + P844 — query/localização (`native_counter_step`+`native_here`+`native_locate`+`native_query`+`parse_selector_arg`; `element_kind_of_native_func`+`native_query`) | `foundations/query.rs` |

Ruído descontado: P797 (`cargo fmt` global), P663/P0f5 (renames `rules→engine→compiler`).

### Critério 2 — pureza vs estado

Classificação por uso real de `EvalContext`:

- **Sem contexto real**: `type`, `repr`, `len`, `str`, `str.from-unicode`, `int`, `float`, `range`, `bytes`, `datetime`, `symbol`, constructors de cor, `metadata`, `selector`.
- **Contexto real**: `counter_at`, `counter_final`, `state_final`, `state_at`, `query`, `locate`, `here`, `target`.

Não criou fronteiras novas, mas confirmou coerência com o critério 3.

### Critério 4 — vanilla

- `metadata`/`state`/`counter`/`query`/`locate`/`here`/`target` no vanilla vivem em `typst-library/src/introspection/`. No cristalino mantiveram-se sob `stdlib/` (`state.rs`, `counter.rs`, `foundations/query.rs`) para não violar os L0s vigentes de `state.md`/`counter.md`.
- **`regex` corrigiu a hipótese inicial**: estava em `stdlib/text/` por histórico; o vanilla coloca `Regex`/`regex()` em `foundations/str.rs`. Absorvido para `foundations/str.rs`.

### Critério 1 — isolamento de teste

Vácuo, como em todas as aplicações anteriores do método. Testes E2E por ficheiro Typst e unitários por módulo; a suite partilhada permaneceu no hub `stdlib/mod.rs`.

## Ficheiros criados

### Nós L1

```
01_core/src/compiler/stdlib/foundations/
├── mod.rs       # hub de reexportação
├── ty.rs        # native_type
├── repr.rs      # native_repr
├── len.rs       # native_len
├── str.rs       # native_str, native_str_from_unicode, native_regex
├── cast.rs      # native_int, native_float, native_range, native_bytes, native_datetime, native_symbol
├── color.rs     # native_rgb, native_luma, native_oklab, native_oklch, native_linear_rgb, native_cmyk, native_hsl, native_hsv
└── query.rs     # native_metadata, native_query, native_locate, native_here, native_target, native_selector
```

### Prompts L0

```
00_nucleo/prompts/compiler/stdlib/foundations/
├── ty.md
├── repr.md
├── len.md
├── str.md
├── cast.md
├── color.md
└── query.md
```

## Ficheiros modificados

- `01_core/src/compiler/stdlib/mod.rs` — reexports ajustadas; `native_regex` via `foundations`.
- `01_core/src/compiler/stdlib/text/mod.rs` — removido `mod regex;` e reexportação.
- `01_core/src/compiler/stdlib/state.rs` — absorvidas `native_state_update`, `native_state_update_with`, `native_state_display`, `native_state_final`, `native_state_at`.
- `01_core/src/compiler/stdlib/counter.rs` — absorvidas `native_counter_display`, `native_counter_at`, `native_counter_final`, `native_counter_step`.
- `00_nucleo/prompts/compiler/stdlib/foundations.md` — reescrito como L0 do hub.
- `00_nucleo/prompts/compiler/stdlib/state.md` — atualizado com nativas absorvidas.
- `00_nucleo/prompts/compiler/stdlib/counter.md` — atualizado com nativas absorvidas.
- `00_nucleo/prompts/compiler/stdlib/text/regex.md` — nota de redirecionamento para `foundations/str.md`.
- `01_core/src/entities/state.rs` — resselo de `@prompt-hash`.
- `crystalline.toml` — exceção de órfão para `text/regex.md` (redirecionamento histórico).

## Ficheiros apagados

- `01_core/src/compiler/stdlib/foundations.rs` (2403 linhas)
- `01_core/src/compiler/stdlib/text/regex.rs`

## Métricas

- `foundations.rs` original: **2403 linhas**.
- Novos nós `foundations/`: **~1950 linhas** (redução por eliminação de código morto e de duplicação).
- `state.rs`: 403 → **554 linhas**.
- `counter.rs`: 409 → **535 linhas**.
- Testes unitários movidos: **27** (`str.rs`: 2, `cast.rs`: 7, `color.rs`: 18).

## Decisões e correções de hipóteses

1. **`regex` não é de `text/`**: critério 3 em vácuo; critério 4 apontou para `foundations/str.rs`. Movido.
2. **`native_state` duplicada em `foundations.rs:805` era código morto** (compilador reportava "never used"). Removida; a ativa está em `state.rs`.
3. **Não se criou `stdlib/introspection/`**: os L0s vigentes de `state.md`/`counter.md` apontam para `stdlib/state.rs`/`counter.rs`. Mover para novo diretório violaria esses L0s sem ganho de fronteira.

## Guardas de passos anteriores

- **Passo 1018**: confirmado que `native_counter_at`/`native_counter_final` ainda estavam em `foundations.rs`. Não se ativou como bloqueio — foram absorvidas por `counter.rs` sem alterar semântica.
- **Passo 1022**: ativada e resolvida — `regex` absorvido por `foundations/str.rs`.

## Validação

```text
cargo build -p typst-core           -> ok (28 warnings pré-existentes, 0 erros)
cargo test --workspace              -> ok (~5855 passed, 0 failed)
crystalline-lint .                  -> 0 violations
crystalline-lint --fix-hashes .     -> 0 drift warnings
```

## Commits

- Estágio de análise e decisão: working tree deste passo.
- Materialização: working tree deste passo.
