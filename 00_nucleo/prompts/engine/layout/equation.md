:warning: **Prompt L0 — `engine/layout/equation` — Layout de Equações**
Hash do Código: b8779a36

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
