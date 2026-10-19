---
# P649 — Trocar correspondência de texto por tipo estruturado no filtro de erros de P648

> **Passo:** 649
> **Data:** 2026-07-09
> **Foco:** P648 decide quais erros de parser propagar comparando o texto exacto da mensagem (`msg.starts_with("invalid hexadecimal number:")`). Se a mensagem for reescrita por qualquer razão no futuro, o filtro deixa de reconhecer o erro, e a propagação pára de funcionar sem aviso nenhum — a mesma classe de falha silenciosa que toda a sequência P633-P648 tentou eliminar, agora escondida no mecanismo de segurança em vez de no código de avaliação. Este passo troca a correspondência de texto por um tipo estruturado.
> **Tipo:** Implementação directa. Refactor, sem mudança de comportamento pretendida.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P648 (onde o filtro de texto foi introduzido).

---

## Contexto

Comparar strings para decidir comportamento é frágil de uma forma específica: o compilador Rust não avisa se a string deixar de bater certo — o programa continua a compilar, os testes que não passam pelo caminho exacto continuam a passar, e só um teste que force esse erro específico revelaria a quebra. É exactamente o tipo de problema difícil de apanhar que motivou a auditoria de P633.

---

## Implementação

### Adicionar uma categoria ao erro do lexer, não só uma mensagem

Confirmar a estrutura actual do erro produzido pelo lexer (`SyntaxError`, ou equivalente) e adicionar um campo que identifique a categoria do erro, não só o texto:

```rust
// Esboço, a confirmar contra a estrutura real:
pub enum LexErrorKind {
    InvalidHexNumber,
    InvalidUnicodeCodepoint,
    Other,
}
```

O lexer, ao criar o erro em cada um dos dois pontos já identificados por P648 (número hexadecimal inválido, escape unicode inválido), preenche `LexErrorKind` explicitamente, não deixa para inferir depois a partir do texto.

### Trocar o filtro em `eval_with_full_error`

```rust
// Em vez de comparar texto:
.filter(|e| matches!(e.kind, LexErrorKind::InvalidHexNumber | LexErrorKind::InvalidUnicodeCodepoint))
```

### Critério de fecho da implementação

- [ ] `LexErrorKind` (ou nome equivalente) criado, com pelo menos as duas categorias já identificadas por P648.
- [ ] Os dois pontos do lexer que geram estes erros preenchem a categoria explicitamente.
- [ ] O filtro em `eval_with_full_error` usa a categoria, não o texto da mensagem.
- [ ] O texto da mensagem continua a mesma (`"invalid hexadecimal number: {0}"`, etc.) — só o mecanismo de decisão muda, não a mensagem apresentada ao utilizador.

---

## Validação

Repetir exactamente os testes de P648, confirmando que continuam a passar com o novo mecanismo:

```bash
cargo test -p typst-core p648_ -- --nocapture
```

Confirmar também, de propósito, que mudar o texto da mensagem (só para teste, não para ficar) não quebra a propagação — uma forma de provar que a fragilidade anterior já não existe:

```bash
# Temporariamente, mudar a mensagem no lexer para outra frase,
# confirmar que o erro continua a propagar-se (porque agora depende da categoria, não do texto),
# depois reverter a mudança de teste.
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `LexErrorKind` (ou equivalente) criado e usado nos dois pontos já identificados.
- [ ] Filtro de `eval_with_full_error` usa tipo, não texto.
- [ ] Testado que mudar o texto da mensagem não quebra a propagação.
- [ ] Testes de P648 continuam a passar sem alteração de comportamento.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p649.md`, com hash do commit.

---

## Nota final

Com este passo, a sequência de falhas silenciosas iniciada em P633 fica fechada com um mecanismo de segurança que não repete, um nível abaixo, o mesmo tipo de fragilidade que a sequência inteira se propôs a eliminar.
