# Relatório — typst-passo-813: equação em bloco não centrada + espaçamento vertical apertado (achado #16, de P808)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-813.md`; único ficheiro acedido em `materialization/`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** em todas as medições (`git diff HEAD --stat` no início: 48 ficheiros, +3776/-403 — passos P823…P826, incluindo o P826 reportado entretanto; ao fechar, 2026-07-22T02:57 (-03:00), os ficheiros deste passo somam alterações adicionais listadas no Passo 2). Horas: sonda ANTES 02:03–02:10, DEPOIS 02:41–02:57. Binário cristalino rebuildado (`cargo build --release`, 17.67s) após a última alteração e antes da medição final.
**Binários:** `./target/release/typst` (cristalino — `typst <input> <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Fixtures em `temp/p813/`. Medição com `mutool trace <pdf>`; no cristalino o trace vem com `transform="1 0 0 -1 0 841.89"`, pelo que `baseline_pdf = 841.89 − y_trace`.

**Âmbito (do prompt):** equação em **bloco** — (1) centragem horizontal na região e (2) espaçamento vertical 1.2em acima/abaixo — batendo com o vanilla. Inline (P800) fora de âmbito; avanços/kerns de glyphs math (P809/P811/P812) fora de âmbito.

---

## Passo 1 — Sonda (ANTES, literal)

Documento de P808 reproduzido (`temp/p813/doc.typ`): `Antes $ x^2 $ Depois`.

Comandos exactos:
`./target/release/typst temp/p813/doc.typ temp/p813/cristalino.pdf` e
`lab/typst-original/target/release/typst compile temp/p813/doc.typ temp/p813/vanilla.pdf`,
seguidos de `mutool trace <pdf>`.

**O achado de P808 reproduz-se tal qual** (reconfirmado do zero após P809/P811/P812, como o prompt mandava):

| Elemento | Cristalino ANTES (medido) | Vanilla (medido) |
|---|---|---|
| "Antes" baseline | 78.105 | 78.104 |
| math `x` baseline | 92.493 (Δ +14.4) | 100.410 (Δ +22.3) |
| "Depois" baseline | 104.736 (Δ +12.2) | 120.969 (Δ +20.6) |
| math `x` horizontal | 70.867 (margem esq.) | 291.993 (**centrado**) |

Medições auxiliares vanilla (decomposição das componentes):
- `$ x^2 $` sozinho (`d1.typ`): math baseline = **79.972** = margin(70.866) + ascent_frame(9.106) — **sem spacing acima no topo da página**.
- Do Δ antes→math (22.306): 22.306 − 13.2(spacing) = 9.106 = ascent do frame math; do Δ math→depois (20.559): 20.559 − 13.2 − 7.237(top-edge texto) = 0.122 = descent do frame math.
- `#set align(right)` (`align.typ`): math `x` continua em **291.993** — a centragem **não** vem do `align` do utilizador; é forçada pelo elemento.

### Pontos localizados (registados antes de tocar em código)

| Ponto | Vanilla | Cristalino (ANTES) |
|---|---|---|
| centragem | ShowSet de `EquationElem`: `out.set(AlignElem::alignment, Alignment::CENTER)` para `block` — `lab/typst-original/crates/typst-library/src/math/equation.rs:186-197` (l. 190); aplicada no flow por `align.x.position(size.x − frame.width())` — `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:589` | inexistente — `layout_equation` coloca os items em `offset_x = cursor_x` (margem esquerda) sem qualquer alinhamento (`01_core/src/engine/layout/equation.rs:61-68`) |
| wrapping em bloco | `EQUATION_RULE` embrulha a equação de bloco em `BlockElem::multi_layouter` — `lab/typst-original/crates/typst-layout/src/rules.rs:805-808` | sem equivalente — a equação é só uma "linha" no cursor |
| spacing vertical | `BlockElem::above/below` default `Smart::Custom(Em::new(1.2))` — `lab/typst-original/crates/typst-library/src/layout/container.rs:342` (parse fallback l. 346-354); aplicado como `Child::Rel` no flow — `lab/typst-original/crates/typst-layout/src/flow/collect.rs:246-253` | inexistente — só o avanço normal de `flush_line` (top-edge + leading), daí o aperto |
| ascent/descent do frame math | bounding boxes reais dos glyphs (ink) — `lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs` | trait `FontMetrics` sem capacidade de bbox de glyph (só `vertical_metrics`/`cap_height`/`text_edges` — `01_core/src/engine/layout/metrics.rs`) |

## Passo 2 — Implementação

Modelo implementado (medições da sonda + fonte vanilla): equação de bloco fica **centrada** em `x = margin + (largura_região − 2·margin − largura_eq)/2` (sem clamp — equação larga sangra centrada como o vanilla, medido); baseline da equação = `baseline_anterior + spacing(1.2em) + ascent_tinta`; baseline seguinte = `baseline_eq + descent_tinta + spacing + top_edge_texto`; no topo da página/região o spacing acima é **suprimido** (baseline = margin + ascent_tinta). A extensão (width/ascent/descent) é medida dos **mesmos items** do `layout_equation`, com tinta real via novo `FontMetrics::text_ink_bounds` (bboxes dos glyphs — paridade mecânica do frame math vanilla).

Ficheiros alterados (além dos headers `@prompt-hash` sincronizados por `--fix-hashes`):

- **`01_core/src/engine/layout/equation.rs`** — o fix principal: caminho de bloco passa a usar `layout_equation_measured`, calcula `offset_x` centrado, posiciona a baseline pelo modelo acima (3 casos: topo de página via `initial_baseline_pending`; linha em curso → captura `prev_baseline` antes do `flush_line`; linha já fechada → `cursor_y − last_flush_advance`), e após o flush da linha math repõe `cursor_y = baseline_eq + descent + spacing + top_edge` (guardado contra quebra de página; actualiza `last_flush_advance` para coerência com um bloco seguinte). Inline intocado.
- **`01_core/src/engine/math/layout/mod.rs`** — `pub struct EquationExtent { width, ascent, descent }` + `MathLayouter::layout_equation_measured` (extent calculado dos mesmos items: `pos.x + advance` para largura; `text_ink_bounds` para tinta; `cap_height` como aproximação documentada para `FrameItem::Glyph` de delimitadores extensíveis).
- **`01_core/src/engine/layout/metrics.rs`** — novo método do trait `FontMetrics::text_ink_bounds(text, size, style) -> (Pt, Pt)` com default conservador `(cap_height, 0)` (mantém `FixedMetrics` e stubs de teste a compilar sem alteração).
- **`01_core/src/engine/layout/cursor.rs`** — `flush_line` regista `last_flush_advance` (o avanço aplicado, quando há items); reset em `new_page`.
- **`01_core/src/engine/layout/mod.rs`** — campo `last_flush_advance: f64` no `Layouter` (+ init).
- **`01_core/src/engine/layout/sub_frame.rs`** — save/restore de `last_flush_advance` (mesmo padrão de `initial_baseline_pending`).
- **`03_infra/src/font_metrics.rs`** — override real de `text_ink_bounds` nas duas implementações: `FontBookMetrics` (face única: `glyph_index` + `glyph_bounding_box`) e `FallbackFontMetrics` (por char via `resolve_primary` + `covering`, **espelhando a cadeia de fallback math P784** — sem isto, chars math como 𝑥/U+1D465 resolviam para uma face arbitrária do FontBook em vez da NewCMMath usada no render; medido: o descent deixou de ser 0 e o Δ abaixo ficou exacto).
- **L0 (3 prompts, antes do código, com `--fix-hashes` no fim):**
  - `00_nucleo/prompts/engine/layout/equation.md` — regras P813 (centragem ShowSet, spacing 1.2em, modelo de baselines, supressão no topo, scope-outs);
  - `00_nucleo/prompts/engine/layout.md` — `text_ink_bounds` na secção `FontMetrics` + nota `last_flush_advance` na secção `flush_line`;
  - `00_nucleo/prompts/engine/math/layout/_comum.md` — `layout_equation_measured`/`EquationExtent`.
- **Testes:** +5 em `01_core/src/engine/layout/tests.rs` (`mod p813_equacao_bloco`), +3 em `01_core/src/engine/math/layout/tests.rs` (extent), +1 em `03_infra/src/font_metrics.rs` (tinta real em fixture NimbusSans). 1 teste pré-existente actualizado (ver Passo 3).

Pureza L1 mantida: só aritmética de cursor e chamadas ao trait `FontMetrics`; sem I/O, relógio, env ou estado global (verificado — nenhum `std::fs`/`std::env`/`SystemTime`/static novo em `01_core`).

## Passo 3 — Validação (DEPOIS, literal)

### Testes novos confirmados a falhar ANTES da implementação

`cargo test -p typst-core p813` antes da implementação → **erro de compilação E0599** (`layout_equation_measured` inexistente) nos 3 testes de extent; os 5 de layout falhariam por geometria (x na margem, baselines apertadas). Após implementação: `8 passed; 0 failed; 4466 filtered out`.

### DEPOIS — binário rebuildado, mesmo comando da sonda

`./target/release/typst temp/p813/doc.typ temp/p813/final.pdf && mutool trace temp/p813/final.pdf`

```text
<g unicode="A" ... x="70.867" y="763.785"/>     → baseline "Antes"  = 78.105   (vanilla 78.104)
<g unicode="𝑥" ... x="292.55" y="741.906"/>     → baseline math     = 99.984   (vanilla 100.410)
<g unicode="2" ... x="299.15" y="745.888"/>
<g unicode="D" ... x="70.867" y="721.347"/>     → baseline "Depois" = 120.543  (vanilla 120.969)
```

| Número | Vanilla | Cristalino ANTES | Cristalino DEPOIS | Estado |
|---|---|---|---|---|
| "Antes" baseline | 78.104 | 78.105 | 78.105 | ✓ |
| math `x` horizontal | 291.993 (centrado) | 70.867 (margem) | **292.55 (centrado)** | ✓ mecanismo correcto; Δ 0.557pt por largura da equação (scope-out, ver abaixo) |
| Δ acima (math − "Antes") | 22.306 | 14.388 | **21.879** | ✓ spacing 1.2em aplicado; Δ 0.427pt por tinta do dígito do sup (scope-out) |
| Δ abaixo ("Depois" − math) | 20.559 | 12.243 | **20.559** | ✓ **exacto** |

Verificação do mecanismo de centragem: x = margin + (453.547 − w)/2 com w = largura da equação medida dos items (10.18pt) → 292.55, exactamente o observado; o Δ de 0.557pt face ao vanilla vem **inteiramente** da largura da equação (cristalino 10.18 vs vanilla 11.29 — kern de attach/correcção itálica e avanços math, domínio P809/P811/P812), não do centrado.

### Casos extra (Passo 3.2 do prompt)

- **Numeração** (`num.typ`: `#set math.equation(numbering: "(1)")\nAntes $ x^2 $ Depois`): cristalino DEPOIS — equação centrada em x=292.55 (inalterada) e `(1)` em x=512.742 na **mesma baseline da equação** (99.984), limite direito 524.41 = margem direita ✓. Vanilla: equação em 291.993 (a numeração **não** muda a centragem neste caso), número em 510.351/100.410. A centragem não quebra a numeração ✓.
- **Parágrafo antes da equação** (`par.typ`: `Antes\n\n$ x^2 $\n\nDepois`): cristalino DEPOIS — baselines 78.105 / 99.984 / 120.543, idênticas ao caso inline (o spacing de bloco substitui o avanço de parágrafo, como no vanilla: 78.104 / 100.410 / 120.969) ✓ — caminho `last_flush_advance` verificado.
- **Duas equações consecutivas** (`duas.typ`: `$ a = b $ $ c = d $`): vanilla baselines 78.500 / 99.455; cristalino DEPOIS 78.501 / 99.335 — **sem duplicação de spacing** (max colapsado via `last_flush_advance` coerente); Δ residual 0.120pt (ascent do frame vanilla para runs com operadores não é pura tinta — scope-out).
- **Equação mais larga que a região** (`larga.typ`, 32 termos): vanilla sangra centrado para x **negativo** (−11.46); cristalino DEPOIS sangra centrado para x=22.49 (ambos à esquerda da margem 70.867, **sem clamp** — mesma classe de comportamento; a diferença de magnitude vem da largura da equação, fora de âmbito) ✓.
- **Equação no topo do documento** (`d1.typ` no cristalino): baseline = margin + ascent_tinta, sem spacing acima (paridade com o vanilla medido: 79.972 = margin + 9.106).

### Suítes (comando + contagem ANTES/DEPOIS)

| Suíte | ANTES | DEPOIS |
|---|---|---|
| `cargo test -p typst-core` | 4464 passed; 0 failed; 2 ignored | **4472 passed; 0 failed; 2 ignored** |
| `cargo test -p typst-infra` | 667 passed; 0 failed; 5 ignored | **668 passed; 0 failed; 5 ignored** |

4464 + 8 testes novos (5 layout + 3 extent) = 4472 ✓; 667 + 1 (tinta real) = 668 ✓ — as contagens batem com os testes declarados.

**Teste pré-existente actualizado (1):** `sum_block_limites_empilhados_verticalmente` (`01_core/src/engine/layout/tests.rs:1394`) — o threshold antigo (`min_y < 70.0`) codificava a geometria **antiga e incorrecta**: equação no topo com baseline = margin + cap-height do texto (79.7), deixando o sup empilhado invadir a margem superior (y ≈ 65.5 < margin 72). Com a baseline = margin + ascent_tinta (P813), o sup fica dentro da margem (y ≈ 76.3) — comportamento mais próximo do vanilla. Asserção reescrita para o invariante novo (empilhamento acima da base + tinta dentro da margem + baseline deslocada), com o motivo registado em comentário.

### Lint

`~/.cargo/bin/crystalline-lint --fix-hashes .` (após os 3 L0) → hashes sincronizados (`equation.rs` → `fc7e1a63`, layout.md → `783fab31`, math/layout → `dcdb8f29`), 0 drift warnings. `~/.cargo/bin/crystalline-lint .` → **6 warnings, todos V7 "prompt órfão" pré-existentes** da working tree P823…P826 (`eval/field-access.md`, `layout/enum_item.md`, `model/document.md`, `stdlib/layout.md`, `stdlib/structural.md`, `infra/package_version_resolution.md`) — o mesmo conjunto já reportado em P822/P826, nenhum em ficheiro tocado por este passo, nenhum V3/V4/V5/V13/V14. **Zero violations novas.**

## Scope-outs e débitos (confirmados, não tentados)

- **Largura da equação math** (kern de attach / correcção itálica / avanços): cristalino 10.18pt vs vanilla 11.29pt para `x^2` — causa total do Δx residual de 0.557pt no centrado. Domínio P809/P811/P812 (layout math interno), não de P813.
- **Fonte dos dígitos de sup/sub**: o cristalino renderiza o `2` sobrescrito em Libertinus Serif (a primária cobre ASCII) enquanto o vanilla usa NewCMMath — divergência pré-existente da cadeia de fontes (P784), causa total do Δ de 0.427pt no Δ acima (a tinta medida é consistente com os glyphs realmente renderizados; a geometria é honesta face ao render).
- **Ascent do frame vanilla para runs com operadores** não é pura tinta de glyphs (Δ residual de 0.120pt em equações consecutivas e ~1.3pt na baseline de equações largas com `+`): o vanilla inclui componentes adicionais no ascent do run; modelar isso exigiria paridade fina de `MathRun`/fragmentos, fora deste passo.
- **Colapso P250 entre `Content::Block` e equações**: o estado `prev_block_below_pending`/`block_chain_active` não integra equações de bloco (o colapso equação↔equação funciona via `last_flush_advance`; equação↔Block explícito não colapsa). Registado no L0 `equation.md`.
- **Calha do número** (`NUMBER_GUTTER` vanilla, `math/mod.rs:219-220`): a centragem de equações numeradas não reserva a largura do número (no caso medido o vanilla também centra na largura total, logo sem divergência observável; casos com `number-align` à esquerda não foram medidos — a feature `number_align` não existe ainda no cristalino).
- **`Content::Align` não reutilizado**: o prompt sugeria-o "se existir"; existe, mas opera sobre sub-frames de conteúdo, e o caminho de equação do cristalino é cursor-based (os items math integram a `current_line`). A centragem directa por `offset_x` é a forma mínima e idiomática do codebase (idem `emit_deferred_float`, `cursor.rs:587`).
