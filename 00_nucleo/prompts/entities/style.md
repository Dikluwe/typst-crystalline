# Prompt L0 — Style e Styles
Hash do Código: 4cb5f241

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

Adiadas (fields em `StyleDelta` sem variant `Style` correspondente):
`tracking`, `leading`, `font` (3/4 da assimetria residual P288 — `lang`
fechado P288, `weight` fechado P289). ADR-0026 como precedente de
divergência. Materialização individual em passos futuros (P289.1-3
candidatos não-reservados).

Derive: `Debug, Clone, Copy, PartialEq`. Lang é `Copy` (P131B) →
`Style` mantém `Copy` intacto pós-P288.

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
