# Relatório de Paridade — P649

**Passo:** 649  
**Data:** 2026-07-09  
**Foco:** Trocar correspondência de texto por tipo estruturado no filtro de erros de P648.  
**Dependências:** P648 (onde o filtro de texto foi introduzido).  
**Hash do commit com as alterações:** `a8efa674e`

---

## 1. Sumário

O filtro de propagação selectiva de erros de parser introduzido em P648 comparava o texto exacto da mensagem (`msg.starts_with("invalid hexadecimal number:")`). Este passo substitui essa comparação por um campo tipado `SyntaxErrorKind` em `SyntaxError`, tornando a decisão independente do texto da mensagem.

Nenhuma alteração de comportamento visível: as mensagens apresentadas ao utilizador continuam as mesmas; apenas o mecanismo de decisão mudou.

---

## 2. Implementação

### 2.1 `01_core/src/entities/syntax_node.rs`

Adicionado `SyntaxErrorKind`:

```rust
pub enum SyntaxErrorKind {
    Other,
    InvalidHexNumber,
    InvalidUnicodeCodepoint,
}
```

`SyntaxError` passou a ter um campo `kind: SyntaxErrorKind`. `SyntaxError::new` preenche `Other` por omissão; `SyntaxError::with_kind` permite especificar a categoria.

### 2.2 `01_core/src/engine/lexer/mod.rs`

Adicionado `Lexer::error_with_kind`, que cria um token de erro com categoria explicita. `Lexer::error` mantém o comportamento anterior (`Other`).

### 2.3 `01_core/src/engine/lexer/code.rs`

O ponto que gera `"invalid hexadecimal number: ..."` passou a usar `error_with_kind(..., SyntaxErrorKind::InvalidHexNumber)`.

### 2.4 `01_core/src/engine/lexer/markup.rs`

O ponto que gera `"invalid Unicode codepoint: ..."` passou a usar `error_with_kind(..., SyntaxErrorKind::InvalidUnicodeCodepoint)`.

### 2.5 `01_core/src/engine/parse/parser.rs`

O filtro de preservação de erros de lexer em `enter_modes` passou a usar `e.kind` (matches em `InvalidHexNumber` / `InvalidUnicodeCodepoint`) em vez de comparar o texto.

### 2.6 `01_core/src/engine/eval/mod.rs`

O filtro de propagação em `eval_with_full_error` passou a usar `e.kind` da mesma forma.

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram, sem falhas.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.1 Testes de P648

```bash
cargo test -p typst-core p648_ -- --nocapture
```

- `p648_parse_error_hex_literal_errors` — ok
- `p648_invalid_unicode_escape_markup_errors` — ok

### 3.2 Teste de fragilidade

Para provar que a propagação já não depende do texto exacto, as mensagens foram temporariamente alteradas para:

- `"TEST hexadecimal error: 0xZZ"`
- `"TEST unicode error: FFFFFFFF"`

A CLI foi executada com essas mensagens temporárias:

```bash
./target/release/typst /tmp/p649-0xzz.typ /tmp/p649-0xzz.pdf
./target/release/typst /tmp/p649-escape.typ /tmp/p649-escape.pdf
```

Ambos os comandos terminaram com `exit code 1` e apresentaram o erro na posição correcta, confirmando que a propagação continua a funcionar mesmo quando o texto da mensagem muda completamente. As mensagens foram revertidas para o texto original.

---

## 4. Decisão

- A comparação de strings para decidir comportamento crítico é frágil e foi eliminada.
- A categoria do erro é definida no ponto onde o erro é criado (lexer) e usada nos dois filtros subsequentes (`enter_modes` e `eval_with_full_error`).
- O texto da mensagem continua a ser o mesmo; o utilizador final não nota diferença.

---

## 5. Estado da sequência de falhas silenciosas

A sequência iniciada em P633 está fechada com um mecanismo de segurança que não repete, um nível abaixo, o mesmo tipo de fragilidade que a sequência inteira se propôs a eliminar.
