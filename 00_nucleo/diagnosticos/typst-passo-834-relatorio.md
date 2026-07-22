# Relatório — typst-passo-834: `image::svg` — suporte a SVG (achado #19, decisão de escopo)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal, sessão interactiva com o dono — prompt lido de `00_nucleo/materialization/typst-passo-834.md`).
**Proveniência:** commit HEAD `61239e9a2` (P833). Sem alterações de código neste passo — decisão de escopo + formalização de dívida. As medições do achado são de P831 (commit `3bcb695ea`, saídas literais no relatório de P831); a contagem de crates foi medida neste passo sobre os `Cargo.lock` do vanilla e do cristalino nesse commit.

**Decisão do dono (2026-07-22, nesta sessão): MANTER o scope-out e formalizá-lo como dívida.**

---

## Passo 1 — Levantamento

### 1.1 — Dívida formal: não existia (achado processual)

O scope-out de SVG existia desde P772k **só no código** (`01_core/src/engine/stdlib/figure_image.rs:167-173`, rejeição por extensão) **e no L0** (`00_nucleo/prompts/engine/stdlib/figure_image.md`). Não havia entrada em `00_nucleo/diagnosticos/debt/DEBT.md` nem ADR — DEBT-66 citava-o apenas como «precedente». Mesmo padrão de P807 (`pdf.attach`): scope-out nunca formalizado. **Corrigido neste passo**: criada a entrada **DEBT-67** em `00_nucleo/diagnosticos/debt/DEBT.md` (com a decisão, o levantamento de peso e o critério de reabertura), e registada no changelog do inventário (total abertos: 7 → 8).

### 1.2 — Peso medido de implementar SVG

- **Crates**: o vanilla usa `usvg 0.47.0` (dependência directa do `typst-library`). Análise do `Cargo.lock` vanilla vs cristalino: `usvg` arrasta **55 crates transitivas, das quais 17 novas** para o cristalino: `usvg`, `kurbo`, `svgtypes`, `tiny-skia-path`, `data-url`, `strict-num`, `simplecss`, `xmlwriter`, `euclid`, `polycool`, `zlib-rs`, `libz-rs-sys`, `arrayref`, `bytemuck_derive`, `float-cmp`, `pico-args`, `unicode-vo`. Já presentes: `fontdb`, `roxmltree`, `ttf-parser`, `rustybuzz`, `unicode-bidi`, `imagesize`, etc. A whitelist `l1_allowed_external` de `crystalline.toml` **não seria tocada** — a descodificação ficaria em L3, como PNG/GIF/WebP (P833).
- **Integração**: o vanilla renderiza SVG como **vector** no PDF (usvg tree → krilla). O exportador cristalino é hand-rolled: seria preciso um conversor usvg-tree → operadores PDF. Infra parcial existente: paths (`ShapeKind::Path`), fills/strokes, transforms/groups (`FrameItem::Group`), gradientes (P263/P272), clip-masks (P242). Trabalho adicional grande: **texto-em-SVG** (resolução de fontes dentro do SVG, `FontResolver` do vanilla em `svg.rs:212-234`), imagens linkadas, máscaras/filtros. **Estimativa: 3–6 passos para um subconjunto sólido**; paridade completa significativamente mais.
- Alternativa de rasterizar (resvg + tiny-skia) rejeitada por princípio: o vanilla é vectorial — raster diverge de propósito e é mais pesado (~30 crates extra).

### 1.3 — Sub-casos de erro

Medidos em P831 (saídas literais no relatório): SVG malformado → vanilla `error: failed to parse SVG (expected a whitespace not '<' at 1:94)`; imagem linkada em falta → vanilla `error: failed to load linked image imagem_que_nao_existe.png in SVG (file not found, ...)`; cristalino rejeita ambos com a mensagem genérica de scope-out. **Conclusão**: são consequência de a feature não existir — com `usvg` viriam essencialmente «de graça» (`format_usvg_error`, `svg.rs:160-169`). Não pesam na decisão de escopo; ficam registados como alvos de paridade para uma eventual reabertura.

## Passo 2 — Decisão (do dono)

Apresentado o levantamento acima. **Decisão do dono: manter o scope-out, formalizando a dívida** — alinhado com os precedentes P807 (DEBT-66) e P781 (PDF-como-imagem).

## Passo 3 — Formalização

- `00_nucleo/diagnosticos/debt/DEBT.md`: nova entrada **DEBT-67 — `#image()` com SVG rejeitado — ABERTO (scope-out formalizado em P834)**, com origem, achado processual, decisão datada, levantamento de peso, critério de reabertura (pedido explícito do dono ou caso de uso real em corpus; ponto de partida: `usvg` + conversor vectorial próprio) e referências. Changelog do inventário actualizado (7 → 8 abertos).

## Validação

Sem alterações de código — suíte não tocada (baseline verde de P833: typst-core 4524/0, typst-infra 678/0, lint exit 0). Únicos ficheiros alterados: `00_nucleo/diagnosticos/debt/DEBT.md` e este relatório.
