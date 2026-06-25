:warning: **Prompt L0 — `rules/layout/equation` — Layout de Equações**
Hash do Código: 7d4c8a1f

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/equation.rs`
**ADRs relevantes**: ADR-0037 (atomização), ADR-0068 (locatable), ADR-0114/0117 (sonda A.0)

---

## Contexto

Braço `Content::Equation` do `layout_content`. Responsável por renderizar
expressões matemáticas inline e de bloco, incluindo numeração automática de
equações de bloco quando activa.

## Regras de negócio

- Equações inline (`block: false`) renderizam no fluxo de texto, com eixo
  matemático alinhado à baseline do texto circundante.
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
