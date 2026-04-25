# ⚖️ ADR-0040: Activação de `#set` em eval

**Status**: `EM VIGOR`
**Validado**: Passo 102.E — 790 testes L1; +7 integração/unitários `#set`; zero violations.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 102

---

## Contexto

A directiva `#set` do Typst permite aplicar propriedades a todo o
conteúdo subsequente no mesmo bloco. Sintaxe:

```typst
#set text(size: 18pt)
Texto em 18pt
```

No cristalino, `#set text(...)` está **activo desde o Passo 30**. A
análise do Passo 102.A revelou a arquitectura actual:

1. `eval_set_rule` empilha `StyleDelta` em `*styles: &mut StyleChain`
   (propagado por ADR-0036).
2. Subsequentes `Content::Text(s, style)` capturam `TextStyle::from(&*styles)`
   no momento da produção — **bake-in**.
3. O Layouter lê o `TextStyle` baked-in; sem necessidade de
   `Content::Styled` wrapping.

Em paralelo, os Passos 99/100/101 introduziram um caminho alternativo
via `Content::Styled(body, Styles)`. Ambos os caminhos produzem o
mesmo output para texto inline.

Este ADR formaliza a decisão de **manter a arquitectura bake-in como
principal** para `#set text`, e abrir caminho futuro para o wrapping.

## Decisão

### Arquitectura do `#set` em vigor

| Target | Arquitectura | Mecanismo |
|--------|-------------|-----------|
| `#set text(bold/italic/size)` | **bake-in** | `StyleDelta` empilhado em `*styles`; `TextStyle::from(&*styles)` capturado em cada `Content::Text` produzido |
| `#set heading(numbering: ...)` | **variante dedicada** | `Content::SetHeadingNumbering { active }` (Passo 57) |
| `#set page(width/height/margin: ...)` | **variante dedicada** | `Content::SetPage { ... }` (Passo 81) |
| `#set figure(numbering: ...)` | **parâmetro explícito** | Muta `*figure_numbering: &mut Option<String>` (Passo 75, parametrizado no Passo 98) |

### Catálogo de propriedades suportadas em `#set text`

| Propriedade Typst | Tipo | `Style`/`TextStyle` |
|-------------------|------|---------------------|
| `bold` | `Value::Bool` | `TextStyle.bold` / `Style::Bold` |
| `italic` | `Value::Bool` | `TextStyle.italic` / `Style::Italic` |
| `size` | `Value::Length` → `Pt` | `TextStyle.size` / `Style::Size` |
| `fill` | `Value::Color` | `TextStyle.fill` / `Style::Fill` — **activado no Passo 102** |

### Propriedades adiadas

| Propriedade | Razão |
|------------|-------|
| `text.font` | Requer `Font` real — stub em L1 |
| `text.lang`, `text.region` | Vocabulário de localização não materializado |
| `text.weight` como string ("bold"/"regular") | Mapping string→bool adiado |
| `par.leading`, `par.spacing` | Sistema de parágrafo não materializado |

### Coexistência bake-in vs. `Content::Styled` wrapping

O Passo 100 criou o arm `Content::Styled` no Layouter e no
`Content::Text` merge com a chain do Layouter. Isto significa que:

- `Content::Styled(body, [Style::Bold(true)])` **também** funciona —
  o Layouter aplica o bold via push/pop na sua `chain`.
- `*bold*` (Passo 101) emite `Content::Styled([Bold(true)], body)`.
- `#set text(bold: true)` empilha em `*styles` (do eval); o próximo
  `Content::Text` captura a vista resolvida.

Ambos os caminhos convergem em `FrameItem::Text { style: TextStyle { bold: true, ... } }`.

**Decisão**: não fazer refactorização deste passo para substituir
bake-in por wrapping. Razão:

1. **Zero regressão**: a arquitectura bake-in tem 6+ testes
   dependentes. Substituir cria risco sem ganho funcional imediato.
2. **Domínio complementar**: wrapping via `Content::Styled` é
   benéfico para conteúdo **não-Text** (shapes, images, grids).
   Hoje o Layouter tem arms dedicados; migrar esses para passar
   pela chain exigiria refactoring extenso.
3. **Evolução natural**: quando `Introspection` materializar e
   `Content::Heading` for colapsado em `Content::Styled`, o
   wrapping torna-se a arquitectura dominante. Adia-se a
   unificação para esse momento.

### O que esta ADR não decide

- **`#show`**: activação do show rule com selector baseado em
  `Content::Styled`. Dívida latente identificada no Passo 101
  (selector match actual usa `matches!(...if ss.iter().any(...))`).
  Este ADR **não** activa `#show`; decisão fica para passo dedicado.
- **Substituição de bake-in por wrapping**: adiada para passo
  dedicado, eventualmente ligado à materialização de Introspection.
- **Warnings para propriedades não suportadas**: `#set text(font:
  "Arial")` é silenciado. Quando `Sink` materializar, deverá emitir
  warning. Abrir DEBT.

## Alternativas consideradas

1. **Refactorizar para `Content::Styled` wrapping no 102**:
   substituir o bake-in por `Content::Styled([Bold(true)],
   following_content)` emitido no eval. Rejeitado por risco de
   regressão (ponto 1 da decisão) e ganho funcional zero para
   Text inline.

2. **Adicionar `Value::Styles` ao enum `Value`**: permitir que
   `#set text` produza um valor primeira-classe. Rejeitado — abre
   porta para decisões estruturais que merecem ADR própria. A
   propagação via `*styles: &mut StyleChain` já cobre o caso.

3. **Remover bake-in completamente**: forçar wrapping em todos os
   casos. Rejeitado — ver pontos 1 e 2 da decisão.

## Relação com outros ADRs

- **ADR-0033** (paridade funcional): preservada. `#set text(size: 14pt)`
  produz output identic ao vanilla.
- **ADR-0036** (atomização): `styles` já é `&mut StyleChain`
  parâmetro desde Passo 94 (segunda aplicação). Este ADR não mexe
  na atomização.
- **ADR-0037** (coesão): `eval_set_rule` em `eval/rules.rs` segue a
  coesão por domínio.
- **ADR-0038** (sistema de estilos em L1): `Style::Fill(Color)` foi
  adicionado; este ADR activa o seu consumo via `#set text(fill: ...)`.
- **ADR-0039** (forma de estilo no FrameItem): `TextStyle.fill:
  Option<Color>` é preenchido pelo `#set text(fill: ...)` daqui em
  diante.

## Consequências

### Positivas

- `#set text(fill: ...)` passa a funcionar — a propriedade `fill` do
  enum `Style` (ADR-0038) tem agora consumidor real.
- Documentação escrita; futuros refactorings têm baseline clara.
- Zero regressão funcional.

### Negativas

- Duplicação de paths (bake-in + wrapping) continua. Dívida
  arquitectural, não funcional.
- Propriedades não suportadas continuam silenciadas. UX sub-óptimo,
  DEBT aberto.

### Neutras

- `eval_set_rule` ganha uma linha de `match "fill" =>`. Sem
  restruturação da função.

---

## Referências

- `00_nucleo/diagnosticos/inventario-set-rule-passo-102.md`
- `00_nucleo/materialization/typst-passo-102.md`
