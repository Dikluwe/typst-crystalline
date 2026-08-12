# Prompt L0 — `compiler/layout/heading`
Hash do Código: 43d603a5

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/heading.rs`
**Criado em**: 2026-06-24 (P451 — heading numbering patterns)
**ADRs relevantes**: ADR-0109 (atomização), ADR-0033 (paridade vanilla)

---

## Contexto

Layout de `Content::Heading` (via `HeadingElem`). Responsável por aplicar estilo bold escalado por nível e, opcionalmente, prefixar o body com o número do counter hierárquico.

---

## Comportamento

1. Aplicar `TextStyle` com `bold=true`, `italic=false`, tamanho escalado por `heading_scale(level)`.
2. Se `cursor_x` já passou da margem, fazer `flush_line()`.
3. Se `heading.numbering` na `StyleChain` for `Bool(true)`:
   - Ler `heading.numbering.pattern` (opcional, `EcoString`).
   - Obter os valores brutos do counter via `Introspector::counter_values_at("heading", loc)`.
   - Se houver pattern, formatar com `format_counter(values, pattern)`.
   - Se não houver pattern, usar `Introspector::formatted_counter_at("heading", loc)` (forma legada `"1.2.3"`).
   - Renderizar o prefixo numérico como `Content::text("{num_str} ")` antes do body.
4. Renderizar o `body`.
5. Restaurar estilo anterior.

## Gate de numeração

- A numeração é activa quando `chain.custom("heading.numbering") == Some(Value::Bool(true))`.
- O pattern é transportado em `chain.custom("heading.numbering.pattern")` como `Value::Str`.

## Patterns suportados

Ver prompt `entities/counter_format.md`. Subset: `"1."`, `"1.1"`, `"I."`, `"(a)"`, `"A."`.

## Scope-outs

- Alinhamento customizado do número (left/center/right/hanging indent).
- Formatação de counter via `counter.display` callback (P241) — este layout usa formatação directa.

## Tests canónicos

- Heading sem numbering: sem prefixo.
- Heading numerado com pattern `"1."`.
- Heading nível 2 com pattern `"1.1"` produz `"1.1"`.
- Reset inferior: heading nível 1 após nível 2 volta a incrementar nível 1.
- Pattern romano `"I.I"` hierárquico.

---

## Resultado esperado

- `01_core/src/compiler/layout/heading.rs` — free function `layout` + tests em `01_core/src/compiler/layout/tests.rs`.

## P978 — escala de heading do vanilla (1.4/1.2/1.0 em relativo) e composição com `#set text`

**Data:** 2026-08-05

**Medição que motiva** (achado incidental de P975 — as letras de maior
|dx| nas secções 4/25/28 eram de headings): duas divergências —

1. **Factores**: `heading_scale` (`layout/helpers.rs`) usava
   2.0/1.667/1.333/1.167 (escala tipo HTML). O vanilla
   (`lab/typst-original/crates/typst-library/src/model/heading.rs:281-285`)
   usa **1.4 para nível 1, 1.2 para nível 2, 1.0 para nível 3+** — medido:
   15.4/13.2/11.0pt sobre corpo de 11pt.
2. **Composição**: o vanilla define o tamanho do heading como estilo de
   chain em **em relativo** (`TextElem::size = Em(scale)`), que compõe com
   `#set text(size:)` (multiplica o tamanho corrente — medido: 9pt set →
   heading L2 a 10.8pt). O cristalino calculava o tamanho em
   `heading.rs` (`layouter.style.size × scale`) mas o merge de
   `layout/text.rs` (`ns_size.unwrap_or(layouter.style.size)`) deixava o
   canal custom do `#set` **sobrepor-se** ao tamanho deliberado do heading
   — com `#set text(size:)` o heading saía ao tamanho do corpo (medido:
   11.0pt em vez de 13.2pt com set de 11pt; 9.0 em vez de 10.8 com set de
   9pt). A regra documentada do merge é "a chain tipada (heading) vence" —
   estava invertida para `size`.

**Decisão**: `heading_scale` passa a 1.4/1.2/1.0 (nível 1/2/3+) e o merge
de tamanho em `text.rs` passa a usar `layouter.style.size` (que já inclui
o `#set` via sync da chain — verificado por instrumentação) — o tamanho
deliberado de features de layout (heading, super/subscrito) volta a
vencer, como a regra documentada manda. **Nota**: o merge também afecta
super/subscritos de texto (`text.rs` escala `layouter.style.size`
directamente) — ficam igualmente corrigidos pela inversão.

**Residual registado** (fora do escopo deste passo): o espaçamento
acima/abaixo do heading no vanilla é `1.8em` (L1) / `1.44em` (L2+) ÷
escala acima e `0.75em` ÷ escala abaixo (mesmo sítio, heading.rs:288-289)
— não medido nem portado aqui.
