# Relatório — Passo 862

**Data:** 2026-07-23T12:18:38-03:00  
**Objectivo:** Separar texto e espaços em tokens distintos no lexer de markup (`SyntaxKind::Text` vs `SyntaxKind::Space`), atingindo paridade de granularidade interna com o vanilla Typst (`[hello world]` → `Text("hello")`, `Space`, `Text("world")`).

---

## 1. Medição antes

O lexer de markup em `01_core/src/engine/lexer/markup.rs:346` fundia texto e espaços num único token `Text`. A causa estava no caso de continuação do loop `text()`:

```rust
// 01_core/src/engine/lexer/markup.rs:372
Some(' ') if s.at(char::is_alphanumeric) => {}
```

Este ramo fazia com que um espaço seguido de alfanumérico fosse absorvido pelo token `Text` em vez de terminar o token. O resultado era:

- `[hello world]` → um único `SyntaxKind::Text("hello world")`.
- O comentário em `01_core/src/engine/eval/tests.rs:9434` registava explicitamente esta divergência como "fora do escopo de F2".

---

## 2. Código identificado

- **L0:** `00_nucleo/prompts/engine/lexer/mod.md` — secção de critérios de verificação do `text()`.
- **Lexer:** `01_core/src/engine/lexer/markup.rs` — função `text()` (linhas 346–385).
- **Loop principal:** `01_core/src/engine/lexer/mod.rs:121` — já encaminha whitespace para `whitespace()`, que emite `Space`/`Parbreak`.
- **Testes lexer:** `01_core/src/engine/lexer/mod.rs`.
- **Testes eval:** `01_core/src/engine/eval/tests.rs` — `p421_repr_sequence` e helpers `eval_doc`/`p421_eval_plain_text`.

---

## 3. Diff resumido

### 3.1 L0

`00_nucleo/prompts/engine/lexer/mod.md`:

- Atualizado o hash do código (`e6675153` → `4caae056`).
- Adicionado critério:
  ```
  Lexer::new("hello world", Markup) → Text("hello"), Space(" "), Text("world")
  ```

### 3.2 Código

`01_core/src/engine/lexer/markup.rs`:

- Removido o ramo `Some(' ') if s.at(char::is_alphanumeric) => {}` do `match` de continuação em `text()`.
- Adicionado comentário explicando que espaços terminam o token `Text`; o loop principal emite `Space`/`Parbreak`.

### 3.3 Testes

`01_core/src/engine/lexer/mod.rs`:

- `lex_markup_text_splits_on_space`: verifica que `"hello world"` em Markup produz `Text`, `Space`, `Text`, `End` e que os textos dos nós são `"hello"`, `" "`, `"world"`.

`01_core/src/engine/eval/tests.rs`:

- Removido o comentário obsoleto em `p421_repr_sequence` que afirmava que `[hello world]` permanecia como `Text` único.
- `p862_repr_plain_text_splits_on_space`: `repr([hello world])` → `"sequence([hello], [ ], [world])"`.
- `p862_content_tree_splits_plain_text_on_space`: inspecção directa da árvore `Content` para `"hello world"` → `Content::Sequence([Text("hello"), Space, Text("world")])`.

---

## 4. Medição depois

### 4.1 Build

```text
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.96s
```

Build sem erros (apenas warnings pré-existentes).

### 4.2 Testes novos

```text
$ cargo test -p typst-core lex_markup_text_splits_on_space -- --nocapture
running 1 test
test engine::lexer::tests::lex_markup_text_splits_on_space ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4674 filtered out

$ cargo test -p typst-core p862 -- --nocapture
running 2 tests
test engine::eval::tests::tests::p862_content_tree_splits_plain_text_on_space ... ok
test engine::eval::tests::tests::p862_repr_plain_text_splits_on_space ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 4673 filtered out
```

### 4.3 Suite completa

```text
$ cargo test --workspace
... (output omitido) ...
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out (cli)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out (crystalline_lint)
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out (doc-tests typst_core)
```

Todos os testes do workspace passam.

### 4.4 Linter

```text
$ crystalline-lint --fix-hashes .
Fixed 4 files:
  ./01_core/src/engine/lexer/code.rs            → 9f8b8648
  ./01_core/src/engine/lexer/markup.rs          → 9f8b8648
  ./01_core/src/engine/lexer/math.rs            → 9f8b8648
  ./01_core/src/engine/lexer/mod.rs             → 9f8b8648

Re-running analysis... ✅ 0 drift warnings remaining

$ crystalline-lint .
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
   --> 00_nucleo/prompts/infra/package_version_resolution.md:0
```

Zero violations (V7 pré-existente, autorizado na descrição do passo).

---

## 5. Contagem de testes por crate

As contagens refletem o estado da working tree no momento do relatório. O passo 862 adicionou **1 teste no lexer** (`01_core/src/engine/lexer/mod.rs`) e **2 testes em eval** (`01_core/src/engine/eval/tests.rs`). O working tree continha também alterações de passos adjacentes, pelo que a diferença total é superior.

| Crate     | HEAD  | Working tree | Delta (total) |
|-----------|-------|--------------|---------------|
| 01_core   | 4657  | 4675         | +18           |
| 02_shell  | 36    | 41           | +5            |
| 03_infra  | 722   | 722          | 0             |
| 04_wiring | 35    | 40           | +5            |

> **Nota:** o delta total inclui alterações de outros passos presentes na working tree (ex.: P865, P867). A contribuição específica do passo 862 é de **+3 testes** em `01_core`.

---

## 6. Estado de fecho

- ✅ L0 atualizado e hash corrigido.
- ✅ Código alterado: espaços terminam tokens `Text` em Markup.
- ✅ Testes novos passam.
- ✅ `cargo build` sem erros.
- ✅ `cargo test --workspace` passa.
- ✅ `crystalline-lint .` sem violations (exceto V7 pré-existente).
