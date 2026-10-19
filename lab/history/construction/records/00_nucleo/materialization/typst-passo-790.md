---
# P790 — Show rule por string (`#show "texto": ...`) aceita e não aplicada em silêncio

> **Passo:** 790
> **Data:** 2026-07-20
> **Foco:** P786 confirmou em `eval::rules` que `#show "world": [W]` compila com exit 0 no cristalino, mas a substituição nunca acontece — o texto original ("Hello world.") renderiza sem alteração, quando deveria mostrar "Hello W.". Nenhum erro, nenhum aviso — a regra é aceita e silenciosamente ignorada. O mesmo módulo também confirmou `page`/`par` como alvo de `#show` gerando erro `unknown variable` no cristalino, quando o vanilla aceita com um warning específico (`` `show page` is not supported and has no effect ``).
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar o mecanismo de show-by-string do vanilla antes de implementar.
> **Prioridade:** Alta — "aceita em silêncio quando deveria ter efeito" é a categoria mais grave já vista nesta série (nem sequer um erro, é uma regra que o usuário escreveu e nunca é executada).
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/c_rules_showpage.typ`, `c_rules_showpar.typ`, `c_rules_probe_show.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn.*show.*string\|Selector::Text\|show.*is not supported" lab/typst-original/crates/typst-library/src/foundations/*.rs lab/typst-eval/src/*.rs 2>/dev/null | head -30
```

Confirmar:
1. Como `#show "texto": ...` é resolvido — é um `Selector` que casa substring de texto, aplicado durante a realização do documento?
2. A mensagem exata do warning para `page`/`par` como alvo de show (`` `show page` is not supported and has no effect ``, confirmar texto completo e se há hint).

```bash
cat > /tmp/p790-showstring.typ <<'EOF'
Hello world.
#show "world": [W]
EOF
lab/typst-original/target/release/typst compile /tmp/p790-showstring.typ 2>&1

cat > /tmp/p790-showpage.typ <<'EOF'
#show page: it => [WRAPPED: #it]
Hello.
EOF
lab/typst-original/target/release/typst compile /tmp/p790-showpage.typ 2>&1
```

### Estado atual do cristalino

```bash
grep -n "fn.*show\|Selector::" 01_core/src/rules/eval/*.rs 01_core/src/entities/*.rs 2>/dev/null | grep -i "text\|string"
```

Confirmar se existe algum mecanismo de `Selector` baseado em texto, mesmo que não conectado à realização — ou se falta desde a raiz.

---

## Implementação

1. Implementar o `Selector` de texto (substring match) e conectar ao mecanismo de realização de show rules, para que `#show "texto": ...` de fato substitua as ocorrências.
2. Para `page`/`par` como alvo de show: em vez de erro `unknown variable`, aceitar como identificador especial reconhecido e emitir o warning específico do vanilla, sem abortar a compilação.

---

## Validação

```bash
./target/release/typst compile /tmp/p790-showstring.typ 2>&1
```

Confirmar que o texto renderizado mudou (verificar via `pdftotext`, não só exit code).

```bash
./target/release/typst compile /tmp/p790-showpage.typ 2>&1
```

Confirmar warning idêntico ao vanilla, exit 0, compilação prossegue.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo de show-by-string do vanilla confirmado.
- [ ] Mensagem de warning para `page`/`par` confirmada palavra por palavra.
- [ ] `#show "texto": ...` de fato substitui o conteúdo (confirmado por `pdftotext`, não só ausência de erro).
- [ ] `#show page`/`#show par` aceito com warning, não erro fatal.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p790.md`.

---

## Próximo passo

Próximos candidatos da lista de P786 §5: selectors por label (`#show <lbl>: ...`, já parcialmente relacionado a este passo, confirmar sobreposição), context/layout eval (`#layout`, `text.lang`, `here().position()`), numbering, smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes.
