---
# P772x — Correção: decoração não propaga através de `layout_sub_frame`

> **Passo:** 772x
> **Data:** 2026-07-17
> **Foco:** P772w descobriu, investigando o comportamento por trás do símbolo mecânico `Inherit` (não procurando diretamente por isso), que `layout_sub_frame` (usado por `place()`, células de grid, `box()`, e mais 4 call-sites — 7 no total) faz o seu próprio flush manual da linha corrente sem chamar `flush_line()`, e é em `flush_line()` que o mecanismo de decoração wrap-aware (P284/P286) registra segmentos de sublinhado/tachado/overline em `decoration_lines_collector`. Resultado: qualquer conteúdo decorado que passe por um sub-frame perde a decoração silenciosamente. Repro já confirmado por P772w: `#underline[Some text #place(top+left)[explanation].]` — vanilla sublinha os dois trechos, cristalino só o primeiro.
> **Tipo:** Implementação directa (causa já identificada com precisão por P772w; este passo aplica a correção, com o cuidado adicional que P772w já sinalizou — 7 call-sites, risco de regressão visual).
> **Tamanho:** L — mexe em mecanismo compartilhado por múltiplos consumidores de `layout_sub_frame`.
> **ADR-0108 EM VIGOR** — validar cada um dos 7 call-sites individualmente, não assumir que a correção generaliza.
> **Regra 5 do handoff** — checklist de sub-layouts obrigatório, dado afetar `place`, `grid`, `box`, e outros.
> **Dependências:** P772w (achado, causa raiz identificada, commit a confirmar).

---

## Sonda — confirmar os 7 call-sites e o mecanismo exato

```bash
grep -rn "layout_sub_frame" 01_core/src/rules/layout/*.rs
```

Listar os 7 pontos exatos (P772w cita `placement.rs`, `place.rs`, `grid.rs`, `cursor.rs`, `mod.rs` — confirmar a lista completa e se há mais).

```bash
grep -n "decoration_lines_collector\|fn flush_line" 01_core/src/rules/layout/cursor.rs
```

Confirmar exatamente onde `flush_line()` registra os segmentos de decoração, e qual é a informação necessária para traduzir um `DecoSegment` gerado dentro de um sub-frame para o referencial do frame pai — P772w já apontou que é a mesma translação (`offset`/`origin_x/y`) já aplicada aos `FrameItem`s do sub-frame quando são colados de volta no pai. Confirmar isso por leitura de código, não assumir.

```bash
grep -n "fn layout_sub_frame\b" 01_core/src/rules/layout/sub_frame.rs
```

Confirmar se `layout_sub_frame` tem acesso a `decoration_lines_collector` do `Layouter` pai (é método do `Layouter`?) ou se precisa de um caminho novo para propagar os segmentos para fora do sub-frame antes de serem descartados.

---

## Implementação

1. Dentro de `layout_sub_frame`, no ponto onde a linha corrente é fechada manualmente (sem `flush_line()`), garantir que o mesmo registro de segmentos de decoração aconteça — reaproveitando a lógica de `flush_line()` (extrair para função compartilhada se `flush_line()` fizer mais do que só decoração, ou chamar a parte relevante diretamente).
2. Traduzir os `DecoSegment`s gerados dentro do sub-frame para o referencial do frame pai, usando a mesma translação já aplicada aos `FrameItem`s (confirmada na sonda).
3. Aplicar a correção nos 7 call-sites, um a um — não assumir que corrigir `layout_sub_frame` uma vez basta sem testar cada consumidor (a lógica de posicionamento de cada call-site é própria, conforme P772w já avisou).

---

## Validação — repro de P772w + checklist de sub-layouts

```bash
cat > /tmp/p772x-place.typ <<'EOF'
#underline[
  Some text #place(top+left)[explanation].
]
EOF
lab/typst-original/target/release/typst compile /tmp/p772x-place.typ /tmp/p772x-place-vanilla.pdf
./target/release/typst compile /tmp/p772x-place.typ /tmp/p772x-place-cristalino.pdf
mutool trace /tmp/p772x-place-vanilla.pdf | grep -c stroke_path
mutool trace /tmp/p772x-place-cristalino.pdf | grep -c stroke_path
```

Confirmar 4 `stroke_path` nos dois (conforme a medição original de P772w).

### Repetir para os outros consumidores de `layout_sub_frame` (regra 5, adaptada)

```bash
# grid
cat > /tmp/p772x-grid.typ <<'EOF'
#underline[
  #grid(columns: 1, [Cell text])
]
EOF

# box
cat > /tmp/p772x-box.typ <<'EOF'
#underline[
  #box[Boxed text]
]
EOF
```

Para cada um, comparar contagem de `stroke_path` vanilla vs cristalino, mesma disciplina.

### Testar strike/overline, não só underline

```bash
cat > /tmp/p772x-strike.typ <<'EOF'
#strike[
  Some text #place(top+left)[explanation].
]
EOF
```

Confirmar que a correção generaliza para os três tipos de decoração, não só underline.

### Não regressão — texto decorado fora de sub-frame

```bash
cat > /tmp/p772x-normal.typ <<'EOF'
#underline[Normal text, no sub-frame.]
EOF
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Lista completa dos 7 (ou mais) call-sites de `layout_sub_frame` confirmada.
- [ ] Mecanismo de tradução de `DecoSegment` do referencial do sub-frame para o do pai implementado, reaproveitando a translação já usada para `FrameItem`s.
- [ ] Repro original de P772w (`place()`) corrigido — 4 `stroke_path`, igual ao vanilla.
- [ ] Testado em pelo menos mais 2 consumidores de `layout_sub_frame` (`grid`, `box`).
- [ ] Testado com `strike`/`overline`, não só `underline`.
- [ ] Sem regressão em decoração fora de sub-frame.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `layout`/`sub_frame` atualizado antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772x.md`, com contagem de `stroke_path` antes/depois para cada caso testado.

---

## Próximo passo

Com este fechado: `image::pdf`, `math.class(...)`, ou fallback de fontes matemáticas — os três débitos restantes de P772w — ou encerrar a série P765a-P772x com um resumo final.
