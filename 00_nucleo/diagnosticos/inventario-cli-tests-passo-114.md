# Passo 114.A — Inventário rápido (CLI tests)

**Data**: 2026-04-23

---

## Cargo

`04_wiring/Cargo.toml`:

```toml
[[bin]]
name = "typst"
path = "src/main.rs"

[dependencies]
typst-core  = { path = "../01_core" }
typst-shell = { path = "../02_shell" }
typst-infra = { path = "../03_infra" }
anyhow      = { workspace = true }
```

- **`[[bin]] name = "typst"`** ✓ — env var é `CARGO_BIN_EXE_typst`.
- **`[dev-dependencies]` ausente** — sem deps ergonómicas (nem
  pretendidas — ver ADR-0046 spec).

## Estrutura

- `04_wiring/` contém: `Cargo.toml`, `src/`.
- `04_wiring/tests/` **não existe** — será criada.

## Testes pré-existentes

Grep `#[test]` em `04_wiring/`: **zero matches**. Nenhum teste
no crate hoje.

## Strings em `main.rs`

| Linha | Literal |
|-------|---------|
| 38 | `"Usage: typst <input.typ> <output.pdf>"` |
| 53 | `"error: input path must have a file name: {}"` |
| 61 | `"error: {}"` (SystemWorldError) |
| 69 | `"error: failed to load source: {:?}"` |
| 86 | `"error: failed to write {}: {}"` |

Diagnósticos do eval/warnings usam formatter ADR-0045 (prefixos
literais `warning:` e `error:` em inglês).

## Decisões herdadas

- **`BIN = env!("CARGO_BIN_EXE_typst")`** — disponível em
  `tests/` porque `[[bin]]` existe.
- **Assert em strings inglesas** — "Usage", "warning:", "error:"
  todos literais em inglês.
- **`#variavel_desconhecida`** produz erro de eval ("unknown
  variable: ...") — confirmado em 113.D com
  `#unknown_variable_that_does_not_exist_anywhere`.

## Gate 114.A

**Não disparado**. Nenhuma tests/ pré-existente a preservar.
Nome do binário é `typst` (não `typst-wiring`). Zero ajustes
necessários ao esboço da spec.
