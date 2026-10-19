---
# P624 — `grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs` já usam as funções únicas de P593?

> **Passo:** 624
> **Data:** 2026-07-05
> **Foco:** P593 consolidou o cálculo de largura em duas funções únicas (`text_width`, `line_content_right`) e confirmou que `cursor.rs`, `helpers.rs`, `layout_bidi.rs`, e `shaper.rs` já as chamavam. Não confirmou os quatro ficheiros tocados em P579/P580 (`grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`) — ficaram como pergunta em aberto desde então, nunca fechada. Este passo fecha essa pergunta, o último item da lista original de quatro "incertos".
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P593 (onde as funções únicas foram criadas e a pergunta ficou em aberto), P579/P580 (onde estes quatro ficheiros foram tocados pela primeira vez, por outra razão).

---

## Verificação

### Procurar cálculo de largura próprio, fora das funções únicas

```bash
grep -n "fn.*width\|advance(\|\.width()" 01_core/src/engine/layout/grid.rs 01_core/src/engine/layout/placement.rs 01_core/src/engine/layout/columns.rs 01_core/src/engine/layout/boxed.rs
```

Para cada ocorrência, confirmar:
1. Chama `text_width` ou `line_content_right` (as funções únicas de P593)?
2. Ou calcula largura de outra forma, por conta própria?

### Testar directamente com um caso que expõe divergência, se existir

Se algum dos quatro ficheiros tiver conta própria, o sintoma mais provável é o mesmo já visto antes nesta sequência: texto com script contextual (árabe, agora também devanágari desde P623) dentro de uma tabela, uma figura posicionada, colunas, ou uma caixa (`box`), com largura calculada errada.

```bash
cat > /tmp/p624-tabela-arabe.typ <<'EOF'
#set text(font: "DejaVu Sans")
#table(
  columns: 1,
  [#set text(lang: "ar", size: 30pt); الكتاب على الطاولة],
)
EOF
./target/release/typst /tmp/p624-tabela-arabe.typ /tmp/p624-tabela.pdf
mutool draw -o /tmp/p624-tabela.png -r 150 /tmp/p624-tabela.pdf
```

```bash
cat > /tmp/p624-colunas-arabe.typ <<'EOF'
#set page(columns: 2)
#set text(lang: "ar", size: 30pt)
الكتاب على الطاولة
EOF
./target/release/typst /tmp/p624-colunas-arabe.typ /tmp/p624-colunas.pdf
```

```bash
cat > /tmp/p624-box-arabe.typ <<'EOF'
#box(width: 150pt)[
  #set text(lang: "ar", size: 30pt)
  الكتاب على الطاولة
]
EOF
./target/release/typst /tmp/p624-box-arabe.typ /tmp/p624-box.pdf
mutool draw -o /tmp/p624-box.png -r 150 /tmp/p624-box.pdf
```

Confirmar visualmente, em cada caso, se o texto árabe dentro da estrutura (tabela, coluna, caixa) tem a largura calculada correctamente — sem sobreposição, sem quebra prematura.

### Critério de fecho

- [ ] Cada um dos quatro ficheiros revisto — chama as funções únicas, ou tem conta própria.
- [ ] Se algum tiver conta própria: testado com o caso correspondente (tabela, coluna, caixa) para confirmar se produz divergência real, ou se por coincidência dá o mesmo resultado.
- [ ] Se houver divergência: corrigido, chamando as funções únicas de P593.
- [ ] Se não houver divergência apesar de conta própria: decidir se ainda vale a pena unificar (por consistência, para evitar divergência futura) ou se fica registado como duplicação inofensiva, com razão.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os quatro ficheiros confirmados, um a um.
- [ ] Qualquer divergência real corrigida.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p624.md`, com hash do commit.
- [ ] Lista de disparidades actualizada — último item "incerto" da lista original fica fechado.
