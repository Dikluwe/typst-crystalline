# Prompt L0 — Style e Styles
Hash do Código: eb0d8fd9

## Módulo
`01_core/src/entities/style.rs`

## Propósito

Definir o vocabulário tipado de propriedades de estilo para o motor
Typst cristalino em L1. Fundação para `#set`/`#show` e para
`Content::Styled` (ADR-0038).

## Contrato

### Enum `Style`

Variantes obrigatórias (Passo 99.A + **P288**):

- `Bold(bool)` — propriedade `text.bold`
- `Italic(bool)` — propriedade `text.italic`
- `Size(Pt)` — propriedade `text.size` em pontos tipográficos
- `Fill(Color)` — propriedade `text.fill` (forward-compat)
- `HeadingLevel(u8)` — propriedade `heading.level` (forward-compat)
- **`Lang(Lang)`** — propriedade `text.lang` (Passo 288 — fecha
  assimetria Tabela B.3 vs B.4; `StyleDelta.lang` existia desde
  P130/P131B/P144 mas escrito apenas por parse-driven `eval_set_rule`).
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Lang(Lang::PT)]))` — desbloqueia testes lang-aware adiados em
  P287. Diagnóstico `diagnostico-style-lang-passo-288.md`.
- **`Weight(u16)`** — propriedade `text.weight` (Passo 289 — fecha
  1/4 da assimetria residual P288 §7 risco terciário; paralelo
  arquitectural absoluto a `HeadingLevel(u8)` e `Lang(Lang)`).
  `StyleDelta.weight: Option<u16>` existia desde P126/P129 mas escrito
  apenas por parse-driven `eval_set_rule` (`#set text(weight: 700)` ou
  nome simbólico via `FontWeight::from_name`). Storage raw `u16` (range
  0-1000 CSS/OpenType; sem validação per spec não-objectivo). Activa 2ª
  fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Weight(700)]))`. Consumer faux-bold P139
  (`TextStyle::faux_bold_stroke_pt`) reusado sem alteração — **primeira
  aplicação directa de ADR-0098** (single source of truth como invariante
  anti-bug; hash `export.rs 66cb8ac3` preservado pelo 6º passo
  consecutivo). Diagnóstico `diagnostico-style-weight-passo-289.md`.
- **`Tracking(Length)`** — propriedade `text.tracking` (Passo 290 — fecha
  1/3 da assimetria residual P289 §5.6; paralelo absoluto a P288/P289).
  `StyleDelta.tracking: Option<Length>` existia desde P127/P137 mas
  escrito apenas por parse-driven `eval_set_rule` em `eval/rules.rs:374`.
  `Length` preserva `abs + em` (P127). Activa 2ª fonte de entrada via
  `Content::Styled(body, Styles::from_iter(
  [Style::Tracking(Length::em(0.1))]))`. **Distinção crítica vs P288/P289**:
  consumer P137 inclui emit real (`Tc` operator em `export.rs:2139-2146`)
  — mas via paradigma `FrameItem::Text.style.tracking` capturado pelo
  Layouter (`TextStyle::from(&chain)`), **não via `chain.tracking()`
  directo**. P290 confirma **ADR-0098 robusta mesmo com emit consumer
  real**: o critério não é "ausência total em emit" mas "via
  `FrameItem.style` capturado". Hash `export.rs 66cb8ac3` preservado
  pelo **7º passo consecutivo**. Diagnóstico
  `diagnostico-style-tracking-passo-290.md` (5 secções A.0-A.5 com
  A.0 não-trivial).
- **`Leading(Length)`** — propriedade `text.leading` (Passo 291 — fecha
  1/2 da assimetria residual P290 §5.6; resta apenas `font`).
  `StyleDelta.leading: Option<Length>` existia desde P128/P138 mas
  escrito apenas por parse-driven `eval_set_rule` em `eval/rules.rs:298`.
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Leading(Length::em(0.65))]))`. **Paradigma consumer distinto
  vs Tracking** (per-line, não per-glyph): `cursor.rs:119-128` em
  `flush_line` faz peek do último `FrameItem::Text` da `current_line`
  (`iter().rev().find_map`) — lê `style.leading.resolve_pt(font_size)`
  e adiciona `line_height + leading_pt` a `cursor_y` antes do drain.
  `export.rs` zero hits para leading — ADR-0098 vigente; hash
  `66cb8ac3` preservado pelo **8º passo consecutivo**.
  **Divergência arquitectural consciente**: vanilla typst tem `leading`
  em `par`; cristalino captura em `text` por conveniência temporária
  (Tabela A.3 linha 70 + 184; sem `Content::Par` propriamente). P291
  preserva esta divergência.
  Diagnóstico `diagnostico-style-leading-passo-291.md` com **6 secções**
  A.0-A.5 + A.5' anti-reflexão (mitigação activa do risco "sequência
  reflexa" P290 §7 — elemento estructuralmente novo identificado:
  paradigma peek `current_line` per-line distinto vs tracking per-glyph).
- **`Font(FontList)`** — propriedade `text.font` (Passo 292 — **fecha
  1/1 final da assimetria residual P289 §5.6; pós-P292 série cirúrgica
  P288-P292 termina naturalmente — assimetria 5/5 fechada**).
  `StyleDelta.font: Option<FontList>` existia desde P140B/P141/P146.
  Activa 2ª fonte de entrada via `Content::Styled(body, Styles::from_iter(
  [Style::Font(FontList::single("Inter"))]))`.
  **Distintivo arquitectural vs P288-P291**: `FontList` é
  `pub struct FontList(Vec<FontFamily>)` — **não é `Copy`**. P292 escolhe
  opção (a) `Font(FontList)` apesar disso — **`Style` enum perde `Copy`
  derive** (A.2.0 inventário literal confirma 0 call sites dependem de
  `*style`; perda inofensiva). Cascade arm usa `f.clone()` em vez de
  `*f` por necessidade material.
  **Paradigma consumer "indirect resolution via FontBook"** (2 layers):
  `FrameItem::Text.style.font` capturado pelo Layouter; emit multifont
  (`export.rs:2169-2174`) consulta `style.font.as_ref()` + `fonts.iter()
  .position(|f| match name)` → `/F{i+1} Tf` (PDF font select). ADR-0098
  vigente. Hash `export.rs 66cb8ac3` preservado pelo **9º passo
  consecutivo**.
  Diagnóstico `diagnostico-style-font-passo-292.md` (6 secções A.0-A.5
  + A.5' N=2 anti-reflexão).

**Marco arquitectural P292**: pós-P292, **série cirúrgica P288-P292
termina naturalmente** — não há mais campos `StyleDelta` sem variant
`Style` correspondente (assimetria 5/5 fechada). Próximo passo será
ortogonal por construção (não há reaplicação cumulativa possível).

Derive pós-P292: `Debug, Clone, PartialEq`. **`Copy` removido por
`FontList: !Copy`** (`Style::Font(FontList)`). Inventário A.2.0
confirma 0 call sites afectados — perda inofensiva. Clone preserved
para `cascade.push_styles(&Styles)` walk.

### Struct `Styles`

- `Styles(Vec<Style>)` — colecção de deltas de estilo.
- Métodos mínimos: `new()`, `push()`, `iter()`, `is_empty()`, `len()`,
  `from_iter<I: IntoIterator<Item = Style>>(iter)`.
- Ordem preservada (a ordem de inserção importa para resolução).

Derive: `Debug, Clone, Default, PartialEq`.

## Invariantes

- Sem I/O (pureza L1).
- Sem dependência de `LazyHash` (ADR-0016 preservada).
- Sem proc macros custom (ADR-0026 como precedente).

## Consumidores

- `Content::Styled(Box<Content>, Styles)` — variante de `Content`.
- `StyleChain::push_styles(&Styles)` — projecção em `StyleDelta`.
- Pipeline futuro de `#set`/`#show` — activação fora do Passo 99.
