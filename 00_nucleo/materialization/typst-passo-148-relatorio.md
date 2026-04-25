# Passo 148 — Relatório (inventário de cobertura: vanilla vs cristalino)

**Data**: 2026-04-24
**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas, revogadas ou
revisadas**. Único output: documento de inventário + relatório
+ actualização menor do §9 dos documentos de paridade.
**Precondição**: Passo 147 encerrado; documentos de paridade
actualizados; 1113 tests; zero violations; 57 ADRs; 10 DEBTs
abertos.

---

## 1. Sumário executivo

Inventário factual do gap de cobertura entre **vanilla Typst**
(snapshot `lab/typst-original/` em commit
`ba61529986e0a5a916cbf937c3c65117cd450683`) e **cristalino**
(Passo 146; 57 ADRs).

**Documento produzido**:
[`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`](../diagnosticos/typst-cobertura-vanilla-vs-cristalino.md)
— ~600 linhas com 3 tabelas (A user-facing, B arquitectural, C
cruzada), resumo agregado, top divergências.

**Reformulação adoptada na série paridade**: a pergunta
original "em que paridade estamos?" foi **transformada** em:

1. **(148, este passo)** "que features cristalino afirma
   cobrir?" — **inventário declarado**.
2. **(149+)** "do que cobre, X% bate observacionalmente com
   vanilla?" — **medição** com denominador honesto.

Sem inventário, percentagens de paridade contra a totalidade
do vanilla são ≈0% — sem informação útil. Com inventário,
paridade torna-se "do que afirmamos cobrir, X% bate".

**Tests**: inalterados em 1113. Zero código tocado.

---

## 2. Localizações canónicas confirmadas (sub-passo 148.1)

### 2.1 — User-facing (vanilla)

`lab/typst-original/crates/typst-library/src/` com 9
sub-módulos:

```
foundations/   — 35 ficheiros (Value, Args, Bytes, etc.)
introspection/ — 13 ficheiros (counter, locate, query, ...)
layout/        — 32 ficheiros (align, columns, grid, page, ...)
loading/       —  8 ficheiros (cbor, csv, json, ...)
math/          — 13 ficheiros (equation, frac, matrix, ...)
model/         — 22 ficheiros (heading, figure, table, ...)
pdf/           —  3 ficheiros
text/          — 14 ficheiros (raw, lang, smartquote, ...)
visualize/     — 11 ficheiros (color, shape, image, ...)
```

55 ficheiros usam macro `elem!` (definição de elementos
públicos) ou `#[elem]` derive. Total estimado: ~70 elementos
declarados via macro + funções stdlib + tipos foundations =
~138 features user-facing observadas.

### 2.2 — Arquitectural (vanilla)

`Value` enum em
`lab/typst-original/crates/typst-library/src/foundations/value.rs`
— **30 variants** (None, Auto, Bool, Int, Float, Length,
Angle, Ratio, Relative, Fraction, Color, Gradient, Tiling,
Symbol, Version, Str, Bytes, Label, Datetime, Decimal,
Duration, Content, Styles, Array, Dict, Func, Args, Type,
Module, Dyn).

`Content` em vanilla: vtable polimórfica via `elem!` macro;
~40+ elementos.

### 2.3 — Arquitectural (cristalino)

`01_core/src/entities/`:
- `Value` enum em `value.rs` — **18 variants** (sub-set
  estratégico).
- `Content` enum em `content.rs` — **38 variants** (enum
  fechado per ADR-0026/0026-R1; divergência intencional).
- `Style` enum em `style.rs` — **5 variants** (`Bold`,
  `Italic`, `Size`, `Fill`, `HeadingLevel`; ADR-0038).
- `StyleDelta` struct em `style_chain.rs` — **10 fields**
  (relatório 142 §3).
- `FrameItem` enum em `layout_types.rs` — **6 variants**
  (`Text`, `Line`, `Glyph`, `Image`, `Shape`, `Group`).

### 2.4 — Stdlib functions (cristalino)

`make_stdlib()` em `01_core/src/rules/eval/mod.rs:480` define
**29 funções nativas**: `type`, `len`, `range`, `rgb`, `luma`,
`str`, `int`, `float`, `heading`, `strong`, `emph`, `raw`,
`figure`, `image`, `rect`, `ellipse`, `circle`, `line`,
`polygon`, `grid`, `page`, `move`, `rotate`, `scale`, `align`,
`place`, `assert`, `upper`, `lower`, `replace` + módulo
`calc`. Plus 6 constantes de alinhamento (`left`, `center`,
`right`, `top`, `horizon`, `bottom`).

---

## 3. Tabela A — User-facing breakdown

138 entradas distribuídas em 9 categorias:

| Categoria | `implementado` | `implementado⁺` | `parcial` | `ausente` | `scope-out` | Total |
|-----------|----------------|-----------------|-----------|-----------|-------------|-------|
| Markup syntactic | 8 | 3 | 3 | 4 | 0 | 18 |
| `#let`/`#set`/`#show`/import | 7 | 1 | 4 | 1 | 0 | 13 |
| Text features | 7 | 5 | 1 | 8 | 2 | 23 |
| Math | 6 | 6 | 1 | 0 | 0 | 13 |
| Layout | 6 | 0 | 2 | 8 | 0 | 16 |
| Model (structural) | 4 | 4 | 5 | 8 | 0 | 21 |
| Visualize | 6 | 1 | 1 | 5 | 0 | 13 |
| Foundations stdlib | 9 | 1 | 4 | 1 | 0 | 15 |
| Introspection | 1 | 0 | 0 | 5 | 0 | 6 |
| **Total** | **54** | **21** | **21** | **40** | **2** | **138** |

**Cobertura user-facing** (impl + impl⁺): (54 + 21) / 138 =
**54%**.

**Itens scope-out** (2): `text.font` dict (gap 8 DEBT-52,
ADR-0054bis condicional); `text.lang` shaping features
(DEBT-53 candidato XL).

---

## 4. Tabela B — Arquitectural breakdown

105 itens distribuídos por tipo:

| Tipo | `implementado` | `implementado⁺` | `parcial` | `ausente` | `scope-out` | Total |
|------|----------------|-----------------|-----------|-----------|-------------|-------|
| `Value` variants | 18 | 0 | 4 | 9 | 0 | 31 |
| `Content` variants (cristalino) | 27 | 9 | 3 | 0 | 0 | 39 |
| `Content` variants extra ausentes (vanilla-only) | — | — | — | ~14 | — | ~14 |
| `Style` variants | 5 | 0 | 0 | 0 | 0 | 5 |
| `StyleDelta` fields | 7 | 2 | 0 | 0 | 1 | 10 |
| `FrameItem` variants | 6 | 0 | 0 | 0 | 0 | 6 |
| **Total** | **63** | **11** | **7** | **23** | **1** | **105** |

**Cobertura arquitectural** (impl + impl⁺): (63 + 11) / 105 =
**70%**.

`Value::Align` e `Content::Styled` são **divergências
intencionais favoráveis** (cristalino tem features que vanilla
não tem em forma de Value/enum); contadas como `implementado`
porque encerradas por ADRs (0028→0029, 0036, 0038, 0026).

---

## 5. Tabela C — Cruzada (excertos)

Para cada feature `parcial` ou `ausente` da Tabela A, a
documento lista o **bloqueante arquitectural** + ADR/DEBT/passo
canónica. Excertos representativos:

| Feature ausente | Bloqueante | Próximo |
|-----------------|------------|---------|
| `text.font` dict | `Value::Regex` ausente | gap 8 DEBT-52; ADR-0054bis |
| `text.weight` Bold dedicada | variant-aware selection | ADR-0055bis candidata |
| `cite`, `bibliography` | CSL parser; loading module | escopo XL |
| `here()`/`locate()`/`state()` | introspection runtime | ADR-0017 (adiada) |
| `table` | `Content::Table`; cell layout | escopo M; DEBT-34d/e abertos |
| `box`/`block`/`columns`/`stack` | `Content::*` ausentes | escopo M agregado |
| Show selectors regex/where | `regex` em L1 | ADR-0054bis condicional |
| OpenType features (lig/kern/bidi) | rustybuzz | DEBT-53 XL |

Lista completa (~40 entradas) no documento de inventário.

---

## 6. Resumo agregado

| Vista | Cobertura (impl + impl⁺) |
|-------|--------------------------|
| User-facing | **54%** (75 de 138) |
| Arquitectural | **70%** (74 de 105) |

**Esta é cobertura declarada, não paridade**. Paridade
observacional contra vanilla ainda exige medição, que vem em
Passo 149+.

---

## 7. Top divergências surpreendentes

10 entradas onde a classificação foi não-óbvia:

1. **`Value::Type` é parcial em cristalino** (string-based)
   enquanto vanilla tem `Type` como tipo dedicado. Divergência
   não documentada via ADR — candidato a clarificação futura.
2. **`Value::Args` não é variant** em cristalino — `Args` é
   tipo separado; passado para funções nativas. Divergência
   arquitectural não-formalizada.
3. **`Content::Heading` é `implementado⁺`, não `implementado`** —
   alguns atributos do `HeadingElem` vanilla
   (`numbering style`, `supplement`, `outline-position`) são
   parciais.
4. **`text.lang` mudou de scope-out total → `implementado⁺`**
   pós-Passo 144. Documentos pré-144 desactualizados.
5. **`paint.rs` em vanilla expõe `cmyk`/`oklab`** — cristalino só
   tem `rgb`/`luma`. Visual; afecta editoriais avançados.
6. **`figure` é `implementado⁺`** — DEBT-14/15 fechadas mas
   variantes do `kind` (table, equation figures) requerem
   trabalho adicional.
7. **~14 elementos `Content::*` vanilla ausentes** (Bibliography,
   Cite, Footnote, Quote, Terms, Table, Columns, Box, Block,
   Stack, Hide, Repeat, Pad, Stroke-object). Escopo XL agregado.
8. **Math layout aproximado (`implementado⁺`)** — sem métricas
   font math (OpenType MATH table). ADR-0033 perfil graded
   cobre.
9. **`Value::Align` é divergência cristalino** — vanilla
   resolve via HAlign+VAlign; cristalino tem Value variant.
   Resolução em DEBT-36.
10. **`Content::Styled(Box, Styles)` é divergência fundamental**
    — vanilla vtable; cristalino enum fechado. Mantida como
    escolha estrutural por ADR-0026/0026-R1.

---

## 8. Actualização §9 dos documentos de paridade

`00_nucleo/diagnosticos/typst-paridade-plano-medicao.md` §9
actualizado para reflectir reformulação da série:

- **Antes**: Passo 148 = "Implementar `frame_dto.rs`".
- **Depois**: Passo 148 = "Inventário de cobertura"; Passo 149
  = "frame_dto.rs". Passos 150/151 renumerados (eram 149/150).

Item 1 da lista renomeado; restantes (P150+ = ValueDTO/P2;
P151+ = pdf_compare/P4) renumerados consistentemente. Item
sobre corpus (P5 antes, P5 agora) clarifica filtro pelo
subconjunto declarado no inventário 148.

Aviso de reformulação adicionado no início do §9, citando o
inventário como pré-condição.

`typst-paridade-definicoes.md` **não tocado** — sem secção
equivalente; permanece factual.

---

## 9. Próximo passo: 149

**Passo 149** materializa a primeira matriz de paridade:

1. `lab/parity/src/frame_dto.rs` com `LayoutTolerance` e modo
   `text_content=true`.
2. `lab/parity/tests/layout_parity.rs` invocando o corpus
   filtrado pelo subconjunto "implementado" + "implementado⁺"
   + "parcial" do inventário.
3. `lab/parity/reports/latest.md` — primeira matriz real.
4. Decisão sobre corpus (oficial vs próprio vs ambos), com
   filtro por features.

A partir do Passo 149, o utilizador passa a ter o número
pedido — em formato de matriz, não de percentual único, com
denominador honesto.

---

## 10. Verificação final

| Item | Estado |
|------|--------|
| Documento `typst-cobertura-vanilla-vs-cristalino.md` criado em `00_nucleo/diagnosticos/` | ✅ |
| Tabela A user-facing (~138 entradas em 9 categorias) | ✅ |
| Tabela B arquitectural (~105 itens) | ✅ |
| Tabela C cruzada (~40 entradas com bloqueantes) | ✅ |
| Resumo agregado com contagens | ✅ |
| Top divergências (10 entradas) | ✅ |
| Cada entrada com referência canónica (ou "—" justificada) | ✅ |
| §9 dos documentos de paridade actualizado (P148 → inventário; P149 → materialização) | ✅ |
| `cargo test --workspace --lib` inalterado (1113) | ✅ |
| `crystalline-lint .` zero violations | ✅ |
| Nenhum ficheiro em `lab/parity/` tocado | ✅ |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ |
| Nenhuma ADR criada / revogada / revisada | ✅ |
| `DEBT.md` e `00_nucleo/adr/README.md` intactos | ✅ |
| Snapshot vanilla declarado (commit hash) | ✅ |
| Cobertura declarada quantificada (54% user-facing; 70% arquitectural) | ✅ |

**Pós-148**: inventário coerente; Passo 149 tem âncora
documental clara para materializar `frame_dto.rs` com corpus
filtrado pelo subconjunto declarado.
