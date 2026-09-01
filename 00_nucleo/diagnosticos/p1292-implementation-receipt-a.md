# P1292 — recibo de implementação do lote A (`math.cancel`)

Data da medição: 2026-08-31T23:22:16-03:00
Estado: working tree não commitado sobre `0eb39f8ecb48930515f2cadb6a378450855b5a72`.

## Papel e fronteira

Implementação apenas do lote A, sob o regime segregado. Este recibo não é
ataque, atestação nem veredito independente. Não foi iniciada implementação de
underline, vec ou flush.

O conteúdo de `04_wiring/tests/p1292_contract.rs` e dos outputs
`/tmp/p1292-*-red.out` não foi aberto. O oráculo protegido não foi executado.
Seu checksum foi apenas conferido mecanicamente contra o manifesto.

## Selos de entrada

- manifesto final, incluindo as exceções mecânicas:
  `fd8159e05bb72ef8da8927f0cdf907f2a99a54e67f29e23605f89ba71d032661`;
- contrato canónico:
  `18a987cedb0808cab4473a5f578b1cd336e1830595cb694cd9d31cfbd1fa0554`;
- selo de contrato:
  `7359b85cda7cdaf85f12eeabb2725ee04452676b2448ffebaa32b8a4a635edbe`;
- recibo RED resealed:
  `343c2cfdd093e7382f8a856eae7a9ac725221fdbf7f4bdf5dfa5338b5ad8d547`;
- oráculo congelado, preservado:
  `3ced6be4556d5b8b7f79540cc0df12791f38d2e22e6d46a2992f38049043c2f5`.

## Implementação A

- `math.cancel` referencia a nativa canónica `native_cancel` antes do espelho
  `sym` → `math`; o binding global legado permanece.
- `MathCancelExplicit` transporta separadamente a presença de `length`,
  `inverted`, `cross`, `angle`, `stroke` e `background`.
- A nativa marca exatamente as chaves presentes, inclusive quando o valor
  explícito coincide com o default. `Content::math_cancel(body)` conserva os
  defaults omitidos e `Span::detached()`.
- Igualdade, hash e `map_content` preservam os bits; `span` continua fora da
  identidade.
- `repr` começa por `body:` e emite somente named explícitos, na ordem
  `length`, `inverted`, `cross`, `angle`, `stroke`, `background`, usando os
  formatadores públicos existentes e a quebra vertical canónica.
- `apply_math_default` preserva os bits ao reconstruir o nó.
- O runtime P1291 existente não foi reimplementado: `cross` continua a produzir
  duas requests independentes (`line=0` e `line=1`) em preorder, com o mesmo
  default automático positivo e o span original.

## Ficheiros e SHA-256

| Ficheiro | Antes | Depois |
|---|---|---|
| `01_core/src/entities/elements/math_cancel.rs` | `66a769eab84425696b7ed7d094f1138e4198f749e3fae6f011b2b45ceac431f5` | `59b6c1c92a803a5aa15c2202036128bc846e9c25d4757faae731f470dbadd020` |
| `01_core/src/entities/content.rs` | `fba0c7abbe850755dd29c97fa9c507e66d3c312e99912cb4aca195e204b7da7a` | `90fd651dbb41862de97e3ec20dfa6ce0daf95bdc0177d9c411ba73853fbe0c07` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `2672fef305aed5fbc192f365c752847b5993788cdabaa95e653349e7c6f70c53` | `9f5ecffba7b829110bf28a012117e46c16f887d592797cba70d47ba490cfc6c6` |
| `01_core/src/compiler/eval/repr.rs` | `eda7b6c2318f51ad7673279ee85069e51ab5a002167511a7bf7a29bcb937e7bc` | `8d36692a10edacc160f93570d5530f8b46a76723e6d7e144d1a008b3b0ea405f` |
| `01_core/src/compiler/math/layout/mod.rs` | `e3c6f586c18bbff8849d5b01f56dac7d891134f9d870ad37c726b89360ae3bab` | `8aef55e3c1b8d9a55af5d3eb2b7631725157a0fb6cb3c9e43d6967890a300e64` |
| `01_core/src/compiler/math/layout/cancel.rs` | `a51e81d2420c0e84eb90d303d803010027379d2f1f864912d426f584b01cb747` | `6e6035ce00f113eaffd0613baf4d3164d033aaa4001eaa84c20f319568df0611` |
| `01_core/src/compiler/math/layout/tests.rs` | `d6b5d455984571e6b6c25453e3170797be8fa552c5158a2089b409c9c8e0c28b` | `603cc01c15c77eba769ce1ad9c66820ca7166872c7b611e62c029d1521893b85` |
| `03_infra/src/layout.rs` | `0fa5031334161d5e220b49db206375235aeebcabb8a2d3ebdcec5203c15722c2` | `f42d72261a5b4fe1a603391c10d4b74f90f807449c28b3184a811880f1c44ded` |

As três alterações sob `#[cfg(test)]` foram exclusivamente mecânicas e
explicitamente autorizadas: `MathCancelExplicit::default()` nos literais já
existentes; em `03_infra/src/layout.rs`, o import foi ajustado para a mesma
adição. Assertions, casos e fixtures permaneceram intactos.

## Linhagem atualizada

Os headers dos owners A tocados apontam aos hashes congelados:

- `math_cancel.rs` → `cc164630`;
- `content.rs` → `48212970`;
- `structural/math.rs` → `087e24d0`;
- `eval/repr.rs` → `4d5247c2`;
- `math/layout/mod.rs` → `90378803`;
- `math/layout/cancel.rs` → `84b2c852`.

## Comandos e resultados

1. `rustfmt --edition 2021 --check <oito ficheiros allowlisted tocados>` —
   código `0` após ajustes locais; nenhum `cargo fmt` global foi executado.
2. `cargo check -p typst-core -p typst-infra --tests` — código `0`; warnings
   preexistentes. Uma execução anterior identificou somente o literal inline
   sem o novo campo; após o adendo específico do manifesto, a repetição ficou
   GREEN.
3. `cargo test -p typst-core math_cancel -- --nocapture` — GREEN, `4 passed`,
   `0 failed`, `5346 filtered out`.
4. `cargo test -p typst-core p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada -- --nocapture`
   — GREEN, `1 passed`, `0 failed`.
5. `cargo test -p typst-core p1291_identity_preorder_e_cross_duas_chamadas -- --nocapture`
   — GREEN, `1 passed`, `0 failed`.
6. `cargo test -p typst-core p1283_math_espelha_sym_sem_sobrescrever_funcoes -- --nocapture`
   — GREEN, `1 passed`, `0 failed`.
7. `cargo test -p typst-core compiler::eval::repr::tests -- --nocapture` —
   GREEN, `59 passed`, `0 failed`, `5291 filtered out`.
8. `cargo build --workspace` — código `0`; warnings preexistentes.
9. `git diff --check` — código `0`.

## Declaração de parada serial

O lote A termina neste recibo. Não foi executado o oráculo protegido, não foi
rodado `--fix-hashes`, e nenhum trabalho dos lotes B, C ou D foi iniciado.
