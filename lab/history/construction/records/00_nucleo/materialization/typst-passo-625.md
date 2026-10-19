---
# P625 — Alinhamento RTL não se propaga para `box`, `columns`, `grid`, `placement`

> **Passo:** 625
> **Data:** 2026-07-05
> **Foco:** P624 encontrou, e dispensou como "fora do âmbito", que texto árabe dentro de `#box()` e dentro de `#set page(columns:)` não recebe o alinhamento à direita já corrigido para o fluxo principal de texto em P576/P592. É o mesmo padrão de bug já visto em P579/P580 — um mecanismo corrigido no fluxo principal, nunca propagado às mesmas quatro estruturas (`grid.rs`, `placement.rs`, `columns.rs`, `boxed.rs`). Este passo confirma o alcance e corrige.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** O mesmo padrão de bug, no mesmo sítio, não se despacha como "fora do âmbito" a segunda vez — precisa de resposta directa.

---

## Contexto

`align_current_line_rtl()`, criado em P576 e ajustado em P592, corrige a posição de uma linha RTL dentro do `Layouter` principal. Quando o mesmo texto entra por um caminho de sub-layout — `layout_sub_frame_with_width` (usado por `grid.rs`, `boxed.rs`), ou o cursor próprio de `columns.rs` — este mecanismo pode nunca ser chamado, deixando o texto alinhado à esquerda por defeito, mesmo sendo RTL.

---

## Sonda

### Confirmar exactamente onde `align_current_line_rtl` é chamado

```bash
grep -rn "align_current_line_rtl" 01_core/src/engine/layout/
```

Confirmar se `layout_sub_frame_with_width`, `columns.rs`, `grid.rs`, `placement.rs`, `boxed.rs` chamam esta função em algum ponto, ou se só o fluxo principal (`flush_line`, `finish`) a chama.

### Confirmar o alcance com os três casos já testados por P624

Reutilizar exactamente os documentos de teste de P624 (tabela, colunas, caixa), desta vez focando só no alinhamento, não na largura:

```bash
cat > /tmp/p625-box.typ <<'EOF'
#box(width: 150pt)[
  #set text(lang: "ar", dir: rtl, size: 30pt)
  الكتاب على الطاولة
]
EOF
./target/release/typst /tmp/p625-box.typ /tmp/p625-box.pdf
mutool draw -o /tmp/p625-box.png -r 150 /tmp/p625-box.pdf
lab/typst-original/target/release/typst compile /tmp/p625-box.typ /tmp/p625-box-vanilla.pdf
mutool draw -o /tmp/p625-box-vanilla.png -r 150 /tmp/p625-box-vanilla.pdf
```

Confirmar com `dir: rtl` explícito (não só `lang: "ar"`), para isolar se o problema é de detecção automática de direcção dentro do sub-layout, ou do próprio mecanismo de alinhamento nunca ser chamado mesmo com a direcção já conhecida.

### Testar também grid e placement, não só os dois já testados por P624

```bash
cat > /tmp/p625-grid.typ <<'EOF'
#grid(
  columns: 1,
  [#set text(lang: "ar", dir: rtl, size: 30pt); الكتاب على الطاولة],
)
EOF
./target/release/typst /tmp/p625-grid.typ /tmp/p625-grid.pdf
mutool draw -o /tmp/p625-grid.png -r 150 /tmp/p625-grid.pdf

cat > /tmp/p625-place.typ <<'EOF'
#place(top + left)[
  #set text(lang: "ar", dir: rtl, size: 30pt)
  الكتاب على الطاولة
]
EOF
./target/release/typst /tmp/p625-place.typ /tmp/p625-place.pdf
mutool draw -o /tmp/p625-place.png -r 150 /tmp/p625-place.pdf
```

### Critério de fecho da sonda

- [ ] Confirmado, com `file:line`, onde `align_current_line_rtl` é chamado e onde não é.
- [ ] Os quatro casos (box, columns, grid, place) testados com `dir: rtl` explícito.
- [ ] Confirmado se o problema é geral aos quatro, ou específico de alguns.

---

## Implementação

Depende da sonda. Provavelmente: `layout_sub_frame_with_width` e o cursor de `columns.rs` precisam de chamar `align_current_line_rtl()` (ou uma versão equivalente) antes de finalizar cada sub-frame, do mesmo modo que `flush_line`/`finish` já fazem no fluxo principal.

### Critério de fecho da implementação

- [ ] Texto RTL dentro de `box`, `grid`, `placement`, `columns` alinha à direita, batendo com o vanilla.
- [ ] Texto LTR dentro das mesmas estruturas sem regressão.
- [ ] Testado com os quatro casos, não só os que já falharam.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de posição já estabelecidos ao longo da sequência RTL (P563, P590, etc.) para confirmar que o fluxo principal continua correcto — esta correcção não deve alterar nada fora dos sub-layouts.

---

## Critério de fecho do passo

- [ ] Sonda completa, alcance confirmado nos quatro casos.
- [ ] Correcção aplicada, testada em todos os sub-layouts afectados.
- [ ] Fluxo principal de RTL sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p625.md`, com hash do commit.
- [ ] Nota adicionada ao padrão já identificado (P579/580, agora P625) — mecanismos corrigidos no fluxo principal precisam de checklist explícita de propagação aos quatro sub-layouts, para não se repetir uma terceira vez.
