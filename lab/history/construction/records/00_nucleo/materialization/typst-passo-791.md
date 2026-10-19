---
# P791 — Show rule por seletor de label (`#show <lbl>: ...`)

> **Passo:** 791
> **Data:** 2026-07-20
> **Foco:** P786 confirmou em `foundations::selector` que `#show <sp>: it => [LBL=#it]` (label como seletor de show rule) é rejeitado no cristalino (`error: selector inválido para show rule: label`), enquanto o vanilla aceita normalmente (exit 0, "HEAD=Alpha [origLBL=] found=1"). Diferente de `#show heading: ...` (funciona) e `#context query(<a1>).len()` (funciona) — o gap é especificamente `Selector::Label` como alvo de `#show`. Este passo confirma se compartilha mecanismo com P790 (splice/aplicação de show rules) ou é um caminho de validação de seletor independente.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar sobreposição com P790 antes de reimplementar.
> **Prioridade:** Alta — funcionalidade rejeitada com erro (não silenciosa, mas ainda assim bloqueia um padrão comum de uso de `#show`).
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/b_selector_show.typ`), P790 (mecanismo de show rules já reforçado, `eval_show_rule`/`apply_show_rules`).

---

## Sonda — mecanismo exato do vanilla e estado atual do cristalino

```bash
grep -n "Selector::Label\|fn.*show.*selector" lab/typst-original/crates/typst-library/src/foundations/selector.rs 2>/dev/null
```

Confirmar como o vanilla resolve um `Selector::Label` dentro de show rule — mesmo mecanismo de `Selector::Elem` (aplica quando o elemento com aquele label é encontrado durante a realização)?

```bash
grep -n "selector inválido para show rule\|Selector::" 01_core/src/rules/eval/rules.rs 2>/dev/null
```

Confirmar onde exatamente o cristalino rejeita `Selector::Label` — é uma validação explícita que só aceita alguns tipos de seletor, faltando o caso de label?

```bash
cat > /tmp/p791-test.typ <<'EOF'
= Alpha <sp>
#show <sp>: it => [LBL=#it]
EOF
lab/typst-original/target/release/typst compile /tmp/p791-test.typ 2>&1
```

---

## Implementação

Adicionar `Selector::Label` como caso válido no ponto de validação/aplicação de `#show`, reutilizando o mecanismo de aplicação já existente para `Selector::Elem` (a diferença deve ser só o critério de match — label do elemento, não seu tipo).

---

## Validação

```bash
./target/release/typst compile /tmp/p791-test.typ 2>&1
```

Confirmar exit 0, comportamento idêntico ao vanilla.

```bash
# Não regressão — show por tipo de elemento e query por label continuam funcionando
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo do vanilla para `Selector::Label` em show rule confirmado.
- [ ] Confirmado se compartilha código com P790 ou é caminho separado.
- [ ] `#show <lbl>: ...` funciona, comportamento idêntico ao vanilla.
- [ ] Sem regressão em `#show <tipo>: ...` nem em `query(<lbl>)`.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p791.md`.

---

## Próximo passo

Context/layout eval (`#layout`, `text.lang`, `here().position()`), numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
