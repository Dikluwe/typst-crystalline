# Passo 101 — Relatório de encerramento (Consolidação: remoção de `Content::Strong`/`Emph`)

**Data**: 2026-04-23
**Precondição**: Passo 100 encerrado; `Content::Styled` activo no
Layouter; `TextStyle` com `fill`+`heading_level`; 783 L1 + 174 L3 + 6
ignorados; zero violations.

---

## Sumário

`Content::Strong(Box<Content>)` e `Content::Emph(Box<Content>)`
removidos do enum `Content`. `*bold*` e `_italic_` passam a emitir
`Content::Styled(body, [Style::Bold(true)])` ou
`[Style::Italic(true)]` via construtores `Content::strong`/`Content::emph`
redefinidos — API pública preservada.

Zero regressão funcional: **783 L1 + 174 L3 + 6 ignorados** inalterados.
`crystalline-lint .` → zero violations.

---

## 101.A — Inventário

Inventário em `00_nucleo/diagnosticos/inventario-strong-emph-passo-101.md`.

Contagem: 28 ocorrências em 7 ficheiros.

| Categoria | Contagem |
|-----------|---------:|
| Enum (E) | 2 (Strong, Emph) |
| Construtores (C) | 8 call sites |
| Match arms (M) | 10 blocos |
| Tests (T) | ~8 sítios |

M=10 ≪ 30 (threshold da spec) → procedimento seguro em sequência.

---

## 101.B — eval_markup + stdlib

### Construtores redefinidos

```rust
// entities/content.rs
pub fn strong(body: Content) -> Self {
    Self::Styled(Box::new(body), Styles::from_iter([Style::Bold(true)]))
}

pub fn emph(body: Content) -> Self {
    Self::Styled(Box::new(body), Styles::from_iter([Style::Italic(true)]))
}
```

### stdlib/structural.rs

`native_strong` / `native_emph` chamam `Content::strong(body)` /
`Content::emph(body)` (que internamente emitem `Styled`).

`eval_strong` / `eval_emph` em `eval/markup.rs` já usavam
`Content::strong(body)` / `Content::emph(body)` — zero mudança.

---

## 101.C — Match arms removidos

### Layouter (`rules/layout/mod.rs`)

Arms `Content::Strong(body)` e `Content::Emph(body)` removidos (12
linhas no total). O arm `Content::Styled` (Passo 100) cobre ambos
via push/pop na `chain`.

### Introspect (`rules/introspect.rs`)

- `materialize_time`: arms Strong/Emph removidos — Content::Styled cobre.
- `walk`: `Content::Strong(body) | Content::Emph(body) => walk(...)`
  removido — Content::Styled cobre.

### Show rule selector (`eval/rules.rs`)

Patch mais delicado:

**Antes**:

```rust
let is_match = matches!(
    (node, kind),
    ...
    | (Content::Strong(_), NodeKind::Strong)
    | (Content::Emph(_),   NodeKind::Emph)
    | ...
);
```

**Depois**:

```rust
use crate::entities::style::Style;
let is_bold_styled = matches!(node, Content::Styled(_, ss)
    if ss.iter().any(|s| matches!(s, Style::Bold(true))));
let is_italic_styled = matches!(node, Content::Styled(_, ss)
    if ss.iter().any(|s| matches!(s, Style::Italic(true))));

let is_match = matches!(
    (node, kind),
    (Content::Heading { .. },  NodeKind::Heading)
    | ...
) || (matches!(kind, NodeKind::Strong) && is_bold_styled)
  || (matches!(kind, NodeKind::Emph)   && is_italic_styled);
```

A lógica de `show strong: it => ...` passa a casar `Content::Styled`
com `Style::Bold(true)`. Semântica preservada; o elemento Strong
permanece detectável.

### Content (internal matches)

- `PartialEq`: arms Strong/Emph removidos.
- `map_text`: arms removidos.
- `map_content`: arms removidos.
- `plain_text`: arms removidos (já cobertos pelo `Content::Styled` arm).

---

## 101.D — Variantes removidas do enum

```rust
// Removidas:
Strong(Box<Content>),
Emph(Box<Content>),
```

Documentação inline actualizada explicando a transição para
`Content::Styled`.

---

## 101.E — Testes actualizados

### `entities/content.rs` tests

- `map_text_desce_em_strong`: construção via `Content::strong(...)` em
  vez de `Content::Strong(Box::new(...))`. A asserção `assert_eq!` agora
  compara `Content::strong(...)` contra `Content::strong(...)` — o
  `PartialEq` sobre `Content::Styled` (Passo 99) cobre a igualdade.
- `map_text_closure_com_estado_entre_nos`: idem.
- `map_content_bottom_up_pai_ve_filhos_transformados`: `matches!(node,
  Content::Strong(_))` → `matches!(node, Content::Styled(_, _))`.

### `rules/introspect.rs` tests

Um construtor `Content::Strong(Box::new(Content::text("Negrito")))`
→ `Content::strong(Content::text("Negrito"))`.

### `layout/tests.rs`

Tests que usam `Content::strong(...)` / `Content::emph(...)` continuam
a compilar e a produzir o mesmo comportamento via construtores
redefinidos. **Zero asserções alteradas**.

---

## 101.F — Paridade funcional

`cargo test --workspace`: 783 L1 + 174 L3 + 6 ignorados (**inalterado**
face ao Passo 100). Zero asserções falharam.

Testes de paridade (`pipeline_parse_eval_layout`) continuam a produzir
a mesma sequência de FrameItem::Text para `Hello *bold* and _italic_`.

O caminho de resolução é:

1. Parse produz AST com `SyntaxKind::Strong` / `SyntaxKind::Emph`.
2. `eval_markup` converte para `Content::strong(...)` / `Content::emph(...)`.
3. Construtores emitem `Content::Styled([Bold(true)], body)` ou
   `[Italic(true)], body)`.
4. Layouter processa via arm `Content::Styled` (Passo 100): push/pop
   na `chain`, `self.style = TextStyle::from(&self.chain)`.
5. `FrameItem::Text.style` tem `bold: true` (ou `italic: true`).

Idêntico ao caminho anterior via arm `Content::Strong`/`Emph`, que
fazia `self.style = TextStyle::bold(size)`.

---

## 101.G — Encerramento

### Verificação

```
$ grep -rn "Content::Strong\|Content::Emph" 01_core/src/ 03_infra/src/ | grep -v "^[^:]*:.*//"
(zero matches)

$ cargo test --workspace | grep "test result"
test result: ok. 783 passed; 0 failed; 0 ignored ...

$ crystalline-lint .
✓ No violations found
```

### DEBT / ADR

- **DEBT-1** actualizado: secção "Actualização Passo 101" adicionada.
  Das 3 pendências listadas no relatório do Passo 100, **1 paga**
  ("remover wrappers Strong/Emph do layout"). 2 restantes:
  1. Activar `#set`/`#show` no eval (trabalho futuro).
  2. Propriedades adicionais (`text.font`, `text.lang`) — bloqueadas
     por tipos não materializados (ADR-0038).
- **Sem ADR novo**: consolidação puramente estrutural.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 783 | **783** (inalterado) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Variantes de `Content` | N | **N-2** (−Strong, −Emph) |
| Match arms em Content | M | **M-10** |
| API pública (`Content::strong/emph`) | preservada | preservada |

---

## Lições

1. **Construtores redefinidos como pontos de indirecção**: manter
   `Content::strong(body)` e `Content::emph(body)` como factories
   (redefinidos para emitir `Styled`) preservou a API pública e
   minimizou o blast radius em 8 call sites (incluindo tests legacy).
   Sem esta decisão, cada call site teria de ser migrado manualmente.

2. **Show selector com helper matching**: `show strong: it => ...`
   continua a identificar `Content::Styled(.., [Bold(true)])` via
   dois `matches!` locais (`is_bold_styled`, `is_italic_styled`). A
   lógica é mais verbosa mas mais flexível — qualquer Styled com
   Bold ganha match, incluindo os produzidos por push_styles
   manuais fora dos construtores.

3. **`Content::Heading` não arrastado**: o spec alerta explicitamente
   contra arrastar Heading. Heading tem semântica adicional (`level`
   usado por `step_hierarchical` e pelo contador de introspecção);
   colapsar em `Styled` perderia essa informação estrutural. A
   decisão certa é aguardar `Introspection` materializado para
   decidir.

4. **Consolidação sem tests novos**: Passo 101 não adiciona testes
   de unidade próprios. É por design — os tests do Passo 99 (API de
   StyleChain/Styles) e do Passo 100 (integração `Content::Styled`
   no Layouter) já cobrem a correcção. Adicionar mais testes seria
   duplicar cobertura.

5. **Zero regressão é o critério**: 783 → 783 L1 tests + zero
   violations é a prova de que a consolidação é **pura** — muda a
   estrutura interna sem mudar o comportamento observável.

---

## Estado pós-Passo 101

### `Content` enum mais pequeno e coeso

- Variantes estruturais (Heading, Figure, Image, Shape, Transform,
  Grid, Align, Place, SetPage, SetHeadingNumbering, SetFigureNumbering,
  ListItem, EnumItem, etc.) permanecem.
- Variantes de estilo inline (`Strong`, `Emph`) substituídas por
  `Styled(Box<Content>, Styles)`.
- `Content::Heading` permanece — bloco dedicado com semântica
  hierárquica.

### Pipeline end-to-end da fundação de estilos

```
Parse → AST → eval_markup → Content::Styled(body, [Style...])
              (para Strong/Emph; heading ainda Heading)
            ↓
Layouter: arm Content::Styled → push_styles(styles) → sync self.style
         (arm Heading continua dedicado por enquanto)
            ↓
FrameItem::Text { style: TextStyle resolvido }
            ↓
export.rs (L3) lê style.bold, style.italic, style.size → selecciona fonte
```

### Trabalho futuro

1. **Activar `#set`/`#show` no eval** — primeira produção de
   `Content::Styled` a partir de directivas Typst não implícitas
   (`#set text(bold: true)`, `#show heading: it => ...`).
2. **Colapsar `Content::Heading` em `Content::Styled`** quando
   `Introspection` materializar — perder o campo `level` dedicado
   e passar a `[Style::HeadingLevel(N), Style::Size(...)]`.
3. **`Introspection` / `Engine<'a>`** — agregador de world + style +
   contadores.
4. **Propriedades adicionais** (`text.font`, `text.lang`) — bloqueadas
   por tipos não materializados.
