:warning: **Prompt L0 — `engine/layout/equation` — Layout de Equações**
Hash do Código: 323d4880

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
