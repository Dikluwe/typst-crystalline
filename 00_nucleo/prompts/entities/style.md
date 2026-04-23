# Prompt L0 — Style e Styles
Hash do Código: 37404a23

## Módulo
`01_core/src/entities/style.rs`

## Propósito

Definir o vocabulário tipado de propriedades de estilo para o motor
Typst cristalino em L1. Fundação para `#set`/`#show` e para
`Content::Styled` (ADR-0038).

## Contrato

### Enum `Style`

Variantes obrigatórias (Passo 99.A):

- `Bold(bool)` — propriedade `text.bold`
- `Italic(bool)` — propriedade `text.italic`
- `Size(Pt)` — propriedade `text.size` em pontos tipográficos
- `Fill(Color)` — propriedade `text.fill` (forward-compat)
- `HeadingLevel(u8)` — propriedade `heading.level` (forward-compat)

Adiadas: `text.font`, `text.lang`, `par.leading`, propriedades
derivadas de proc macro `#[elem]` do vanilla — ADR-0026 como precedente
de divergência.

Derive: `Debug, Clone, Copy, PartialEq`.

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
