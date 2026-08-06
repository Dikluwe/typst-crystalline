:warning: **Prompt L0 — `engine/layout/equation` — Layout de Equações**
Hash do Código: 4e95546e

**Camada**: L1 · **Alvo**: `01_core/src/engine/layout/equation.rs`
**ADRs relevantes**: ADR-0037 (atomização), ADR-0068 (locatable), ADR-0114/0117 (sonda A.0)

---

## Contexto

Braço `Content::Equation` do `layout_content`. Responsável por renderizar
expressões matemáticas inline e de bloco, incluindo numeração automática de
equações de bloco quando activa.

## Regras de negócio

- Equações inline (`block: false`) renderizam no fluxo de texto, com a
  **baseline da equação alinhada à baseline do texto circundante** (P800 —
  paridade vanilla medida por `mutool trace`: texto e math partilham a mesma
  baseline; o eixo matemático fica `axis_height` ACIMA da baseline e só
  governa o centrado interno de fracções/delimitadores via
  `apply_axis_offset`). Os items do `MathLayouter::layout_equation` vêm com
  posições relativas à baseline da fórmula (y = 0), pelo que a integração
  soma simplesmente `cursor_y` (`offset_y = cursor_y`). A regra anterior
  (Passo 48: "eixo matemático alinhado à baseline do texto", implementada
  como `offset_y = cursor_y - axis_pt`) foi **refutada por medição** —
  deslocava toda a fórmula inline `axis_pt` (~0.5em) para cima do texto.
- Equações de bloco (`block: true`) dão `flush_line()` antes e depois,
  ocupando a sua própria linha.
- **P813 — equações de bloco são centradas horizontalmente na região**
  (paridade vanilla: ShowSet `align(center)` para equações de bloco —
  `lab/typst-original/crates/typst-library/src/math/equation.rs:190`;
  centrado dentro do bloco pelo flow —
  `lab/typst-original/crates/typst-layout/src/flow/distribute.rs:589`).
  O `offset_x` dos items é `margin + (largura_util - largura_equacao) / 2`
  (largura_util = `regions.current.width - 2 * margin`), sem clamp — uma
  equação mais larga que a região sangra centrada, como no vanilla.
- **P813 — espaçamento vertical de bloco 1.2em acima e abaixo**
  (paridade vanilla: `BlockElem::above/below` default
  `Smart::Custom(Em::new(1.2))` —
  `lab/typst-original/crates/typst-library/src/layout/container.rs:342`;
  o wrapping da equação em BlockElem —
  `lab/typst-original/crates/typst-layout/src/rules.rs:807`). Modelo de
  baselines (medido por `mutool trace` em P813):
  - baseline da equação = `baseline_anterior + spacing + ascent_ink` —
    o vanilla empilha `descent_prev + spacing + ascent_frame` e o
    `descent` da linha de texto é 0 (bottom-edge default `"baseline"`);
  - baseline seguinte = `baseline_equacao + descent_ink + spacing +
    top_edge_texto`;
  - **no topo da página/região** (`initial_baseline_pending`), o spacing
    acima é suprimido: baseline = `margin + ascent_ink` (medido: `$x^2$`
    sozinho → baseline = margin + ascent, sem 1.2em);
  - `ascent_ink`/`descent_ink`/`width` vêm de
    `MathLayouter::layout_equation_measured` (extent calculado dos items
    com `FontMetrics::text_ink_bounds` — paridade com o vanilla, cujo
    frame math usa as bounding boxes dos glyphs, não as métricas globais
    da fonte).
  - Para recuperar a baseline da linha anterior quando a equação entra
    com a linha já fechada (ex.: após `Parbreak`), o Layouter regista
    `last_flush_advance` em `flush_line()` (ver `engine/layout.md`).
  - Scope-out P813: o colapso `max(prev.below, above)` entre blocos
    adjacentes (P250) não se aplica ainda a equações consecutivas; a
    centragem de equações numeradas não reserva a calha do número
    (`NUMBER_GUTTER` vanilla).
- Numeração automática:
  - O gate é lido da chain via `custom("equation.numbering")`.
  - O valor deve ser `Value::Str(pattern)` (ex: `"(1)"`, `"[I]"`, `"(a)"`).
  - Apenas equações de bloco (`block: true`) com pattern presente são numeradas.
  - O número é obtido via `Introspector::flat_counter_at("equation", loc)`.
  - O número é formatado por `format_counter(&[n], pattern)`, com fallback
    arábico se o pattern for inválido.
  - O número é renderizado como `FrameItem::Text` à **direita** da página,
    alinhado verticalmente com a baseline da equação.
  - Equações inline ignoram o gate (não são numeradas).

## Patterns suportados

Mesmo subset de `format_counter` (P451): `"1"`, `"1."`, `"I."`, `"(a)"`,
`"A."`, `"[1]"`, etc.

## Critérios de verificação

- Equação de bloco com `equation.numbering = "(1)"` → número `(1)` visível.
- Equação de bloco com `equation.numbering = "[I]"` → número `[I]` visível.
- Duas equações de bloco numeradas sequenciais → `(1)` e `(2)`.
- Equação inline com gate activo → sem número.
- `plain_text` continua a incluir o número formatado.

## P784 — `TextStyle.math` marcado no ponto de entrada

Antes de chamar `MathLayouter::layout_equation`, constrói-se
`math_style = TextStyle { math: true, ..self.style.clone() }` e passa-se
`&math_style` (não `&self.style`). Ponto **único** onde `math: true` é
definido — herdado por toda a árvore de layout math via `..style.clone()`.
Consumido em L3 (`shaper.rs`, ver `infra/shaper.md` §P784) para engatar
sempre a cadeia de fallback de fontes matemáticas, independentemente de a
fonte de corpo por omissão ter ou não tabela MATH OpenType própria (não
tem — verificação visual real com glifo `⨿`/U+2A3F confirmou que a
condição anterior, só `primary_has_math`, nunca disparava no caso comum e
o cristalino embutia glifo errado de fonte de sistema aleatória). Ver
`entities/layout_types.md` §P784 para o campo em si.

## P893 — `MathLayouter::new` passa a receber `math_style`

`math_layouter = math::layout::MathLayouter::new(&self.metrics, block)` passa a
`MathLayouter::new(&self.metrics, block, &math_style)` — `math_style` já estava construído na linha
imediatamente anterior (P784, acima), só não era passado. Motivo: `MathLayouter::new` computa
`constants = metrics.math_constants(style)` (ver `engine/layout.md` §P893, `math/layout/_comum.md`
§P893) — sem `style`, `FallbackFontMetrics::math_constants` não tem como resolver qual face da
cadeia de fallback fornece as constantes MATH reais, caindo sempre em `MathConstants::fallback()`
(achado colateral de P891, medido e quantificado em `typst-passo-893-relatorio.md`).

## P895 — centragem/numeração de equação de bloco guardadas contra `width: auto`

**Achado** (`typst-passo-894-relatorio.md`, Prioridade 1): `#set page(width: auto)` deixa
`page_config.width`/`regions.current.width` em `f64::INFINITY` até `compute_page_width()` resolver
o valor final (a partir do conteúdo já colocado — só acontece em `finish()`/`new_page()`, ver
`engine/layout.md`). A centragem horizontal de equações de bloco (P813, acima) e o posicionamento do
número da equação (`layout_equation`, secção "Acrescentar número") liam
`self.regions.current.width` **directamente**, antes dessa resolução — com `width: auto`,
`offset_x`/`right_x` ficavam `INFINITY`, propagando para `compute_page_width()` (via
`line_content_right`) e daí para a `MediaBox` exportada como o literal inválido `"inf"` (PDF
malformado; leitores como MuPDF/poppler substituem silenciosamente um tamanho de página fallback,
mascarando o bug como "conteúdo desaparecido").

**Correcção**: ambos os pontos passam a verificar `self.regions.current.width.is_finite()` antes de
usar o valor:
- **Centragem** (P813): quando infinito, `offset_x` fica no valor já inicializado
  (`self.regions.current.cursor_x`, a margem) — sem aplicar a fórmula de centragem. Paridade com o
  comportamento observado no vanilla para uma única equação de bloco com `width: auto` (medido:
  vanilla também posiciona a equação exactamente na margem nesse caso — a página auto-ajustada faz
  `usable == largura da equação`, a centragem degenera para offset zero de qualquer forma).
- **Numeração**: quando infinito, o número da equação **não é posicionado** (scope-out deliberado —
  não há margem direita bem definida contra a qual alinhar; caso raro, não exercitado pelo ficheiro
  que expôs o achado original).

**Fora de âmbito, à data de P895** (superado por P896, abaixo): esta correcção não replicava a
centragem cruzada do vanilla entre múltiplas equações de larguras diferentes na mesma página
`width: auto`.

## P896 — centragem/numeração corrigidas via correcção adiada (superam o "fora de âmbito" de P895)

**Achado** (`typst-passo-896-relatorio.md`, Fase A): lido o código-fonte real do vanilla
(`typst-layout/src/flow/distribute.rs`) — o mecanismo real **não** é "duas passagens completas de
layout" (não recalcula nada); é **posicionamento diferido**: cada filho do flow é acumulado como
`Item::Frame(frame, align)` (frame já medida, posição ainda por resolver) numa estrutura de
trabalho, e só **uma** função (`finalize`) resolve todas as posições, chamada depois de toda a
região estar recolhida, quando a dimensão final (mesmo que `auto`) já é conhecida.

**Correcção** (Opção (c) do relatório, escopo mínimo — só equação, decisão confirmada pelo dono):
`layout_equation` já não tenta resolver a posição correcta quando `regions.current.width` está
infinito — regista os dados necessários (`Layouter::pending_equation_centering`/
`pending_equation_numbering`, `engine/layout.md` §P896) e `Layouter::apply_pending_equation_fixups`
(chamado por `finish()`/`new_page()`, já com `page_width` finito) corrige as posições antes da
`Page` ser fechada. Ver `engine/layout.md` §P896 para o mecanismo completo (campos, método,
`helpers::shift_frame_item_x`).

**Ainda fora de âmbito** (decisão explícita do dono, per `typst-passo-896-relatorio.md`): o mesmo
bug em `Content::Align`/`resolve_alignment` (`#align(center)[...]` fora de modo math, confirmado na
Fase A a sofrer exactamente a mesma classe de erro — `available_width()` também devolve `infinito`)
**não foi corrigido**. Candidato a um passo dedicado futuro que estenda o mesmo mecanismo de
diferimento a `resolve_alignment` em geral.

## P944 — `EquationElem::show_set` de fonte: equações usam `New Computer Modern Math`

**Medição** (proveniência: commit `8c8fb3c97`, working tree com `eprintln!` de diagnóstico
temporários em `attach.rs`/`stretchy.rs`, revertidos antes do fecho da Fase A; artefactos em
`temp/p944/`; relatório completo em `00_nucleo/diagnosticos/typst-passo-944-relatorio.md`):

- Com `#set text(font: "New Computer Modern")`, `$ mat(1,2;3,4) $` produz
  `vertical_glyph_variants('(') = []` e `vertical_glyph_assembly('(') = 0 partes` →
  `layout_stretchy_delimiter` cai no fallback de glifo base → delimitador curto, colado à
  última linha da matriz (defeito visual confirmado pelo dono nas secções 5/21 do documento
  de 30 secções).
- `$ lim_(x -> ∞) 1/x $` produz `math_constants().lower_limit_gap_min = 0` →
  `y_sub = base_descent + 0 + sb.ascent` (ascent de tinta, P921): 4.41pt para `x→∞` vs 5.86pt
  para `x→0` → o subscrito do segundo `lim` fica ~3pt acima da posição vanilla e colide com a
  base (defeito visual confirmado na secção 4).
- Causa raiz medida (fontTools): `NewCM10-Regular.otf` (a fonte de **texto** "New Computer
  Modern", primeira primária) **tem tabela MATH, mas stub** — `LowerLimitGapMin = 0`,
  `LowerLimitBaselineDropMin = 0`, `MathVariants` ausente. `NewCMMath-*.otf` tem os valores
  reais (`LowerLimitGapMin = 167`, `LowerLimitBaselineDropMin = 600`, `(` com 8 variantes
  verticais 997–2991du + assembly de 3 partes). O primeiro passe de `covering` (P912) e
  `math_constants` (P893) escolhem "a primeira primária com tabela MATH" — e essa é o stub de
  texto, não a fonte MATH real.
- Vanilla (`lab/typst-original/crates/typst-library/src/math/equation.rs:197-201`):
  `EquationElem::show_set` fixa **`TextElem::weight = 450`** e
  **`TextElem::font = FontList(["New Computer Modern Math"])`** para **toda** a equação
  (inline e bloco) — o `#set text(font:)` do documento **não** se aplica dentro de equações.
- Sondagem end-to-end (sem código novo): o mesmo `.typ` com
  `#set text(font: "New Computer Modern Math")` produz variantes povoadas, assembly de 3
  partes, `lower_gap = 1.837pt` e render limpo dos dois casos (`temp/p944/min-mathfont.png`).

**Decisão**: em `layout_equation` — ponto único, tal como P784 para `math: true` — o
`math_style` passa a fixar `font = FontList(["New Computer Modern Math"])` e
`weight = Some(450)`, replicando o show-set do vanilla citado acima. **Nenhuma** alteração em
`covering`/`math_constants`/`select_variant` em L3 — o mecanismo está correcto; a fonte
primária é que estava errada. Nós `MathStyled` (`mono`/`serif`/`sans`/`upright`/`bold`)
não tocam `style.font` (actuam por transformação de codepoints Unicode e factores de
tamanho), pelo que o override não os afecta. O **número de equações numeradas** também é
composto com `math_style` (emenda da revisão cética de P944): no vanilla, o número é
layoutado com a chain que inclui o show-set
(`lab/typst-original/crates/typst-layout/src/math/mod.rs:217`,
`layout_frame(engine, &counter, …, styles)`). A mudança é de
comportamento amplo por desígnio (toda a matemática passa a usar a fonte MATH real — é a
semântica da língua, ADR-0107); a revalidação visual do documento de 30 secções (Fase C do
passo) é o gate.

**Scope-out** (medidos em P944, registados para passo dedicado — não corrigidos aqui):

- `attach.rs` (limites) não implementa o termo `max(lower_limit_baseline_drop_min, …)` da
  fórmula do vanilla (`lab/typst-original/crates/typst-layout/src/math/scripts.rs:306-310`),
  e `MathConstants` não tem `lower_limit_baseline_drop_min`/`upper_limit_baseline_rise_min` —
  residual sub-ponto na posição de limites após esta correcção.
- `sb.ascent` de tinta (P921) vs ascent de frame por métricas da fonte do vanilla — diferença
  residual na mesma casa decimal.
- Fonte custom dentro de equações (`#show math.equation: set text(font: …)`): sem evidência
  de suporte actual no cristalino; o override é incondicional (comportamento do vanilla na
  ausência desse show).

**Critérios de aceitação** (nível da língua — geometria medida, ADR-0107/0108):

- `$ mat(1,2;3,4) $` e `$ mat(1,2,3;4,5,6;7,8,9) $` com `#set text(font: "New Computer
  Modern")`: delimitadores cobrem todas as linhas da matriz (prova visual/`mutool trace`
  contra o vanilla).
- `$ lim_(x -> ∞) 1/x $`: subscrito `x→∞` abaixo de `lim`, sem colisão de glifos; gap ≥
  `LowerLimitGapMin` (167du = 1.837pt a 11pt).
- Suite completa verde e `crystalline-lint .` com zero violations.
- Documento de 30 secções revalidado visualmente na íntegra (Fase C), com registo de
  qualquer achado novo.

## P945 — entrada fixa `math_size: Display | Text`

No mesmo ponto único de P784/P944 (construção do `math_style`), a entrada passa
a fixar também `math_size: MathSize::Display` (equação de bloco) ou
`MathSize::Text` (inline) — paridade com o vanilla, que fixa
`EquationElem::size` = Display/Text conforme `block`
(`lab/typst-original/crates/typst-library/src/math/equation.rs:189-195`).
Semântica e consumidores do campo: `entities/layout_types.md` §P945.

## P952 — espaçamento equação→equação inclui `descent_ink` da anterior

**Medição** (`typst-passo-952` Fase A, decomposição exata do modelo em
`layout_equation` + medição etiqueta-a-etiqueta das 44 equações do documento):
o vanilla posiciona blocos de flow **aresta-a-aresta** — fundo do frame
anterior + `max(above, below)` colapsado (1.2em,
`flow/collect.rs:253-278` + `flow/distribute.rs:185-199`) + topo do frame
seguinte. O cristalino (P813) computava a baseline da equação seguinte como
`baseline_anterior + 1.2em + ascent_ink(seguinte)` — **sem o `descent_ink` da
equação anterior** — encostando equações consecutivas com conteúdo profundo
(défice = `descent_ink` da anterior; medido: matrizes +12.3 a +12.9pt/gap,
frações +4.8 a +8.5pt/gap, linhas simples ~0). O caso texto→equação estava
correcto porque `bottom-edge` do texto é a baseline (descent = 0) — e o caso
equação→texto já incluía a descent (P813, linha "baseline seguinte =
baseline_equacao + descent_ink + spacing + top_edge").

**Correcção**: no cálculo de `prev_baseline` (ramo de recuperação,
`cursor_y - last_flush_advance`), somar a `descent_ink` da equação de bloco
imediatamente anterior, registada num novo campo do Layouter
`prev_block_equation_descent: f64` (default 0.0) — escrito no epílogo do
bloco (`Some(ext.descent)`) e reposto a 0.0 nos mesmos pontos onde
`last_flush_advance` é actualizado/resetado (`cursor.rs` — flush de linha de
texto e reset, `sub_frame.rs` — save/restore). Sem efeito quando o conteúdo
anterior não é equação de bloco (campo 0 → comportamento P813 inalterado,
guardado pelos testes P813 existentes).


## P967 — cursor após equação inline inclui o espaçamento interno de classe (extent, não soma de advances)

**Medição** (`typst-passo-967` Fase A; auditoria externa 2026-08-05 secção
8.2 — espaço entre math inline e a palavra seguinte: 0.0-0.06pt no
cristalino vs 3.65-4.85pt no vanilla): caso mínimo `a $3x + y = 9$ dado b`
— no cristalino, "dado" é colocado a `x = cursor_x` computado como a
**soma dos `metrics.advance(text)`** dos items da equação, que não inclui
o espaçamento de classe embutido nos `pos.x` (THICK após relações, etc.
de `compute_gaps`, P772y) — o cursor ficava ~11pt antes do fim real da
equação e "dado" renderizava SOBRE o "9". No vanilla, a equação inline
avança pela largura total do run (fragments com gaps incluídos,
`math/mod.rs:64-72`).

**Correcção**: no braço inline de `layout_equation`, o cursor passa a
acompanhar o **extent real** dos items — por item,
`cursor_x = max(cursor_x, offset_x + pos.x + advance)` (Text/TextShaped) e
`offset_x + pos.x + x_advance` (Glyph) — a mesma fórmula de largura de
`EquationExtent` (P813), partilhada, em vez da soma simples de advances.
Efeito: o cursor final = fim real da equação (incluindo gaps internos); o
espaço literal do `.typ` entre `$…$` e a palavra seguinte é colocado a
partir daí, sem sobreposição.

## P987 — número de equação acompanha o conteúdo (calha `NUMBER_GUTTER`) sob `width: auto`

**Medição** (achado §8.7 da auditoria 2026-08-06 — o mais grave da ronda): no
vanilla, `(1)` fica a ~6pt do fim do conteúdo da SUA equação; no cristalino,
a ~206pt (x=496.47, quase na margem direita da página auto-width do doc de 30
secções). Repro mínimo (`temp/p987/min.typ`): vanilla `(1)` em x=96.48 com
conteúdo a acabar em 90.36 (gap = 0.5em); cristalino `(1)`/`(2)` ambos em
x=56.75 — SOBREPOSTOS ao conteúdo, porque a página auto-width encolhe ao
conteúdo (a reserva do número não entra na largura) e o número era alinhado à
direita dessa largura computada.

**Leitura do vanilla** (`typst-layout/src/math/mod.rs:209-330`,
`add_equation_number` + `resize_equation`): para largura de região INFINITA,
a linha da equação numerada tem largura `W = eq_width + 2 × full_number_width`
com `full_number_width = number_width + NUMBER_GUTTER` (`Em::new(0.5)`,
`mod.rs:217`); o conteúdo é centrado na linha e o número fica no fim da linha.
Invariante resultante (válida com ou sem recentragem da linha na região):
`number_x = content_end_x + gutter`. Para largura FINITA (página fixa), o
número vai para o fim da região — igual ao comportamento actual do cristalino
nesse caso (não se mexe).

**Correcção** (mecanismo em `engine/layout.md` §P987): o número pendente
passa a registar `eq_width`/`applied_offset`; a largura da página auto inclui
a linha de cada equação numerada (`eq_width + 2 × (number_width + gutter)`);
o fixup posiciona o número em `content_end + gutter` (conteúdo centrado pela
mesma fórmula do fixup de centragem P896). **Revoga o scope-out de P813**
("a centragem de equações numeradas não reserva a calha do número") registado
acima.

**Critério**: doc com `width: auto` e 2+ equações numeradas de larguras
diferentes → cada número a `content_end + 0.5em` da sua equação; página fixa
→ número na margem direita (inalterado); página auto com uma equação larga
não numerada + numerada estreita → número da estreita junto a ela, não à
largura da larga.
