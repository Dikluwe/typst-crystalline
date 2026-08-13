# Prompt L0 — `stdlib/foundations` — hub de reexportação
Hash do Código: ffe0b6d2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/mod.rs`
**Origem**: Passo 1032 — fatiamento de `foundations.rs` monolítico conforme
`auditar-fatiamento.md` e absorção do nó `regex` de `text/regex.rs`.
**ADRs**: ADR-0037 (coesão por domínio), ADR-0107 (paridade linguagem),
ADR-0109 (atomização forma B), ADR-0127 (gate de L0).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Visão geral

O módulo `foundations` é um **hub de reexportação** sem lógica de negócio.
A lógica vive nos nós abaixo; o hub apenas declara os submódulos e reexporta
as funções para manter compatibilidade com os consumidores existentes
(`crate::compiler::stdlib::native_*`).

## 2. Nós

| Nó | Ficheiro | Funções |
|---|---|---|
| `ty` | `foundations/ty.rs` | `native_type` |
| `repr` | `foundations/repr.rs` | `native_repr` |
| `len` | `foundations/len.rs` | `native_len` |
| `str` | `foundations/str.rs` | `native_str`, `native_str_from_unicode`, `native_regex` |
| `cast` | `foundations/cast.rs` | `native_int`, `native_float`, `native_range`, `native_bytes`, `native_datetime`, `native_symbol` |
| `color` | `foundations/color.rs` | `native_rgb`, `native_luma`, `native_oklab`, `native_oklch`, `native_linear_rgb`, `native_cmyk`, `native_hsl`, `native_hsv` |
| `query` | `foundations/query.rs` | `native_metadata`, `native_query`, `native_locate`, `native_here`, `native_target`, `native_selector` |

As nativas de estado e counter que viviam em `foundations.rs`
(`native_state_update`, `native_state_update_with`, `native_state_display`,
`native_state_final`, `native_state_at`, `native_counter_display`,
`native_counter_at`, `native_counter_final`, `native_counter_step`) foram
absorvidas pelos respetivos módulos de valor de primeira classe:
`compiler/stdlib/state.rs` e `compiler/stdlib/counter.md`.

## 3. Restrições estruturais

- O hub **não contém lógica de negócio** — só `pub mod` e `pub use`.
- Cada nó tem o seu próprio prompt L0 em
  `00_nucleo/prompts/compiler/stdlib/foundations/<nó>.md`.
- A visibilidade e as assinaturas das funções são preservadas.

## 4. Critérios de verificação

- `cargo build -p typst-core` sem erros.
- `cargo test -p typst-core` mantém a contagem de `#[test]` do hub original.
- `crate::compiler::stdlib::native_*` continua a resolver para a função
  correspondente.
