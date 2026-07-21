# P787 — CSV: rigor de parsing e correção de `row-type`

> **Passo:** 787
> **Data:** 2026-07-20, medições entre ~18:25Z e ~18:50Z
> **Commit-base:** `0774275fe1b340d823e624958f66e5b6b344a51a` + working tree não commitado (P786a + P787)
> **Ficheiros alterados neste passo (`git diff HEAD --stat`):** `01_core/src/engine/stdlib/loading.rs` (+117), `01_core/src/engine/eval/tests.rs` (7 testes eval-level), `00_nucleo/prompts/engine/stdlib/loading.md` (§3.2 §P787); + linha `@prompt-hash` por `--fix-hashes`
> **Binários:** vanilla `lab/typst-original/target/release/typst` (0.15.0, rev `969087ec`); cristalino `./target/release/typst` (rebuild pós-correção)
> **ADRs:** ADR-0107 (mensagens de erro são observáveis), ADR-0108 (mensagens/API confirmadas por execução antes de implementar).

---

## Resumo em uma linha

**Os três achados P786 de `loading::csv_` corrigidos com mensagens idênticas ao vanilla: linha malformada agora rejeitada (`failed to parse CSV (found 3 instead of 2 fields in line 2)`), delimitador não-ASCII com a razão certa (`delimiter must be an ASCII character`), e `row-type:` aceita o tipo `dictionary`/`array` (a string é rejeitada com `expected type, found string`).**

---

## 1. Sonda — contrato do vanilla medido por execução (não só leitura)

Todos os casos executados contra o vanilla 0.15.0 (ficheiros em `/tmp/p787/`, `--root /`):

| Caso | Vanilla 0.15.0 |
|---|---|
| `csv("bad.csv")` (`a,b` + `1,2,3`) | exit 1: `failed to parse CSV (found 3 instead of 2 fields in line 2)` (span no ficheiro csv) |
| `delimiter: "é"` | exit 1: `delimiter must be an ASCII character` |
| `delimiter: "ab"` | exit 1: `expected exactly one character` |
| `row-type: dictionary` / `array` / `delimiter: ";"` | exit 0, `((a: "1", b: "2"),)` |
| `row-type: "dictionary"` (string) | exit 1: `expected type, found string` |
| `row-type: 5` | exit 1: `expected type, found integer` |
| `row-type: str` | exit 1: `` expected `array` or `dictionary` `` |

Casts confirmados também por leitura (`loading/csv.rs:103-135`): `Delimiter` = char ASCII; `RowType` = `Type` array/dict.

## 2. Implementação (`01_core/src/engine/stdlib/loading.rs`)

1. **`decode_csv`**: removido `.flexible(true)` — a crate `csv` rejeita linhas com nº de campos divergente; `UnequalLengths` mapeado para o formato exacto do vanilla, com `line` do `Position` do próprio erro (não inventada). Outros erros → `failed to parse CSV ({e})`.
2. **`delimiter:`**: validação por char — 1 char ≠ → `expected exactly one character`; não-ASCII → `delimiter must be an ASCII character`.
3. **`row-type:`**: aceita `Value::Type` (`Type::Array`/`Type::Dictionary`); tipo errado → `` expected `array` or `dictionary` ``; valor não-tipo → `expected type, found {string|integer|...}` (helper `vanilla_type_name`, formas longas do vanilla).
4. **L0 primeiro** (Protocolo de Nucleação): `loading.md` §3.2 §P787 + `--fix-hashes`.

## 3. Testes (escritos primeiro; 8 falharam antes, 10/10 depois)

- Unitários (`loading.rs`): linha malformada array/dictionary rejeitada com a mensagem exacta; CSV válido sem regressão.
- Eval-level (`tests.rs`, `MockWorld` + `add_file`): os 7 casos da tabela acima via `#csv(...)`.

## 4. Validação CLI (release, `/tmp/p787/`)

```text
r1  exit 1  error: failed to parse CSV (found 3 instead of 2 fields in line 2)   ✓ = vanilla
r2  exit 1  error: delimiter must be an ASCII character                          ✓ = vanilla
r3  exit 0  row-type: dictionary → ((a: "1", b: "2"))                            ✓ = vanilla
r3c exit 1  error: expected type, found string                                   ✓ = vanilla
r4  exit 0  CSV válido sem regressão                                             ✓
r5  exit 0  delimiter ";" + dictionary                                           ✓
r6  exit 1  error: expected `array` or `dictionary`                              ✓ = vanilla
```

- `cargo test --workspace`: **verde**, exit 0.
- `crystalline-lint .`: **zero violações** (2 warnings V7 pré-existentes).

## 5. Registos para passos futuros

1. **Span `<detached>` nos erros de `csv()`** (r1/r2/r3c/r6): a linha do ficheiro CSV viaja no texto da mensagem (paridade de conteúdo), mas o vanilla aponta o diagnóstico para o ficheiro `.csv` (e o chamador para o call site). Mesma família dos spans de loading em geral — agrupar com T6 da reverificação P785.
2. **Erro de leitura vs parse**: o cristalino lê o ficheiro inteiro antes de parsear; o vanilla reporta `while calling `csv`` como trace. Diferença de formato de diagnóstico (single-line cristalino vs fancy vanilla) — convenção do projecto, sem acção.
