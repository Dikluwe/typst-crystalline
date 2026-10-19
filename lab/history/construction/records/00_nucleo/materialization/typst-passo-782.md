---
# P782 — Splice de `#expr`/field-access bare em modo matemático

> **Passo:** 782
> **Data:** 2026-07-17
> **Foco:** P772y (§3.6.2) encontrou, e P780 reconfirmou como distinto do débito já fechado (`MathIdent` bare por nome), que interpolação `#expr` dentro de `$...$` e field-access bare (ex: `$sym.suit.heart$` sem `#`) produzem página vazia para o valor interpolado, sem erro — descartado silenciosamente em `eval_math_expr`, caindo no braço genérico `_ => Ok(Content::Empty)`. Casos confirmados por P772y: `$x #sym.suit.heart y$`, `$x #hc y$` (com `hc` vinculado a `Content`), e `$sym.suit.heart$` bare fora de qualquer `FuncCall`/`MathIdent`.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M — braço de match a adicionar em `eval_math_expr`, mecanismo de conversão já existe (`value_to_display_content`, criado por P780).
> **ADR-0108 EM VIGOR** — confirmar o mecanismo exato do vanilla antes de implementar.
> **Dependências:** P772y (achado original), P780 (confirmação de que é débito distinto, `value_to_display_content` já disponível para reutilizar).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "Expr::FieldAccess\|Expr::Hash\|splice" lab/typst-eval/src/math.rs 2>/dev/null
```

Confirmar:
1. Como o vanilla trata `#expr` dentro de `$...$` — é um nó de sintaxe distinto (`MathShorthand`/equivalente) que o parser já reconhece, ou é resolvido no avaliador?
2. Como o vanilla trata field-access bare (`sym.suit.heart` sem `#`) dentro de math — é tratado como uma sequência de `MathIdent`s encadeados (`sym`, `.`, `suit`, `.`, `heart`) resolvidos em cascata, ou como uma única expressão `FieldAccess` no AST?

```bash
cat > /tmp/p782-test1.typ <<'EOF'
$x #sym.suit.heart y$
EOF
cat > /tmp/p782-test2.typ <<'EOF'
$sym.suit.heart$
EOF
cat > /tmp/p782-test3.typ <<'EOF'
#let hc = sym.suit.heart
$x #hc y$
EOF
lab/typst-original/target/release/typst compile /tmp/p782-test1.typ 2>&1
lab/typst-original/target/release/typst compile /tmp/p782-test2.typ 2>&1
lab/typst-original/target/release/typst compile /tmp/p782-test3.typ 2>&1
```

Confirmar que os três compilam com sucesso no vanilla e renderizam o glifo/conteúdo esperado.

### Confirmar o estado atual do cristalino

```bash
grep -n "Expr::FieldAccess\|_ =>" 01_core/src/engine/eval/math.rs
```

Confirmar exatamente onde o braço genérico descarta o valor, e se `Expr::Hash`/interpolação já chega como um nó distinto no AST do cristalino ou se precisa de mudança no parser/lexer também (não só no avaliador).

---

## Implementação

1. Se `#expr` já chega como nó distinto ao avaliador: adicionar o braço em `eval_math_expr` que avalia a expressão interna normalmente (`eval_expr`, não `eval_math_expr`) e converte o resultado via `value_to_display_content` (já criado por P780), reutilizando sem duplicar.
2. Se field-access bare precisar de resolução em cascata: implementar a resolução de `Expr::FieldAccess` dentro de `eval_math_expr`, usando `scopes.get_local` (já criado por P780) para o identificador base e navegando os campos.
3. Confirmar se os dois casos (`#expr` e field-access bare) compartilham código ou são caminhos genuinamente distintos — não assumir que é o mesmo fix para os dois sem confirmar.

---

## Validação

```bash
./target/release/typst compile /tmp/p782-test1.typ 2>&1
./target/release/typst compile /tmp/p782-test2.typ 2>&1
./target/release/typst compile /tmp/p782-test3.typ 2>&1
```

Confirmar que os três renderizam o glifo `♥` corretamente (não página vazia).

```bash
# Não regressão — os casos já cobertos por P780 continuam funcionando
cat > /tmp/p782-nonregression.typ <<'EOF'
#let myvar123 = 5
$ myvar123 $
$ foobarbaz $
EOF
./target/release/typst compile /tmp/p782-nonregression.typ 2>&1
```

```bash
cargo test --workspace
crystalline-lint .
```

Reconfirmar testes de math existentes (P299-301, P765b, P772w, P772y, P780) sem regressão.

---

## Critério de fecho do passo

- [ ] Mecanismo exato do vanilla confirmado para `#expr` e field-access bare (podem ser mecanismos distintos).
- [ ] `#expr` splice implementado, reutilizando `value_to_display_content` de P780.
- [ ] Field-access bare implementado (resolução em cascata ou mecanismo confirmado pela sonda).
- [ ] Os três casos de teste de P772y renderizam corretamente.
- [ ] Sem regressão nos casos já corrigidos por P780.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de eval matemático atualizado antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p782.md`.

---

## Próximo passo

Com este fechado: fallback de fontes matemáticas (débito antigo de P772w) é o único item de débito conhecido ainda em aberto. Ou parar para um resumo da série P765a-P782.
