# Prompt L0 — `stdlib/text` — smartquote, decoração textual, lorem e smallcaps
Hash do Código: 0a3f4916

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/text.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`.
**Nota de deriva (F4)**: `text.rs` também define `native_upper`/`native_lower`/
`native_replace`, não specados em `stdlib.md`; candidatos a spec dedicada.

---

## `smartquote(double?, enabled?)` — Passo 287

Função stdlib paralela ao markup `"foo"`/`'bar'` (P155). Emite
`Content::SmartQuote { double }` que o Layouter resolve lang-aware via
`localize_quotes` + state per-document (`Layouter.smartquote_*_open`).
Diagnóstico: `diagnostico-smartquote-passo-287.md`. Assinatura
`native_smartquote(_ctx, args, _world, _current_file, _figure_numbering)`.

**Argumentos**: `double: bool = true` · `enabled: bool = true` (quando `false`,
emite `Content::Text(glyph)` ASCII literal directo). **Não aceita posicionais**.

**Scope-out per ADR-0054 graded** (erro educacional citando ADR): `alternative:
bool` (DE/FR); `quotes: Smart<SmartQuoteDict>`.

```
native_smartquote() → Ok(Content::SmartQuote { double:true })
native_smartquote(double:false) → Ok(Content::SmartQuote { double:false })
native_smartquote(enabled:false) → Ok(Content::text("\""))
native_smartquote(enabled:false,double:false) → Ok(Content::text("'"))
native_smartquote(alternative:true) → Err;  native_smartquote(quotes:"()") → Err;  native_smartquote(Bool(true)) → Err
```

Layouter consumer (glyph lang-aware): ver `content.md` "Variant
Content::SmartQuote — Passo 287".

## `underline / strike / overline(body, stroke?, offset?, extent?)` — Passo 284 (ADR-0054 graded)

Decoração textual paralela vanilla `text/deco.rs`. Três funções com `body`
posicional obrigatório (Content ou Str) + cosméticos opcionais. Emit reusa
`FrameItem::Line`; offsets Y default por kind (em-units no espaço Layouter):

| Função | offset_em default | Posição |
|--------|---:|---|
| `underline` | `+0.10` | logo abaixo do baseline |
| `strike` | `-0.25` | atravessa o x-height |
| `overline` | `-0.80` | acima do cap-height |

Assinaturas idênticas `native_underline`/`native_strike`/`native_overline`
(`_ctx, args, _world, _current_file, _figure_numbering`).

**Named aceites**: `stroke: Color | none` (**apenas Color**; objecto `Stroke`
rico vanilla é scope-out, Tabela A.7 linha 201; `none` desactiva) · `offset:
Length | none` (override; `Length::resolve_pt(font_size_pt)`; Int/Float pt) ·
`extent: Length | none` (extende horizontalmente; `None`=0pt).

**Scope-out (erro citando ADR-0054)**: `evade: bool` (descender skipping);
`background: bool` (z-order).

```
native_underline([Content(c)]) → Ok(Content::Underline { body:c, stroke:None, offset:None, extent:None })
native_underline([Str("x")]) → Ok(Content::Underline { body:text("x"), ... })
native_underline([],stroke:Color) → Err;  native_underline([Content(c)],evade:true) → Err
```
Idem `native_strike` (sem `evade` em vanilla) e `native_overline`.

## `lorem(n)` — Passo 391

Helper puro de texto dummy. Entrada `Int` ≥ 0, saída `Value::Str` com `n`
palavras de Lorem Ipsum. Zero tipo novo; zero I/O; zero layout.

**Argumentos**: `n: Int` (posicional obrigatório). `n < 0` → erro. Não aceita
argumentos nomeados.

**Implementação**: vocabulário Lorem Ipsum fixo embeddado; cicla/repete até
atingir `n` palavras. O texto exacto não precisa de ser byte-identical ao
vanilla (paridade semântica ADR-0107).

```
native_lorem(Int(0))  → Ok(Str(""))
native_lorem(Int(5))  → Ok(Str("Lorem ipsum dolor sit amet"))
native_lorem(Int(-1)) → Err
native_lorem(Str("x")) → Err
native_lorem(Int(5), foo:Int(1)) → Err
```

## `smallcaps(body)` — Passo 408 + Passo 446

Elemento de texto vanilla `SmallcapsElem` que transforma o body em small
capitals. No cristalino o variant `Content::SmallCaps { body }` existe desde
o Passo 408; o **Passo 446 materializa o consumer real** com fallback por
scaling (paridade visual vanilla quando a fonte não disponibiliza small caps
OpenType `smcp`/`c2sc`).

**Argumentos**: 1 posicional `Content | Str`. Zero named args.

**Implementação**:
- `Content::SmallCaps { body: Box<Content> }`.
- `plain_text` delega a `body`; `map_content` recursa em `body`; `is_empty`
  delega a `body`; `PartialEq` por `body`.
- Show rule: `NodeKind::Smallcaps` casa `Content::SmallCaps { .. }`.
- Layouter flag `smallcaps: bool`:
  - Arm `Content::SmallCaps { body }` activa o flag e faz `layout_content(body)`.
  - No arm `Content::Text`, quando o flag está activo, cada palavra é
    segmentada em runs de minúsculas vs outros caracteres:
    - minúsculas → maiúsculas (`to_uppercase`) renderizadas a `0.8×` do
      tamanho actual;
    - maiúsculas e não-letras mantêm o tamanho actual.
  - O flag é herdado por conteúdo aninhado (`strong`, `emph`, `Styled`, etc.).

**Scope-out (ADR-0054 graded)**: shaping OpenType `smcp`/`c2sc` nativo da
fonte continua scope-out; o fallback por scaling é funcionalmente equivalente
para a maioria das fontes.

```
native_smallcaps([Content(c)]) → Ok(Content::SmallCaps { body:c })
native_smallcaps([Str("x")])   → Ok(Content::SmallCaps { body:text("x") })
native_smallcaps()             → Err
native_smallcaps([Content(c)], foo:Int(1)) → Err
layout(smallcaps([Hello]))     → "HELLO" com minúsculas a 0.8×
```

## `sub(body)` / `super(body)` — Passo 448

Elementos de texto vanilla `SubElem` / `SuperElem`. No cristalino modelam-se
via `Content::Styled` + `Style::Subscript` / `Style::Superscript`, reaproveitando
a cadeia de estilos existente.

**Argumentos**: 1 posicional `Content | Str`. Zero named args.

**Implementação**:
- `Content::sub(body)` emite `Content::Styled(body, [Style::Subscript(true)])`.
- `Content::superscript(body)` emite `Content::Styled(body, [Style::Superscript(true)])`
  (o construtor chama-se `superscript` porque `super` é keyword de Rust).
- Show rule: `NodeKind::Subscript` / `NodeKind::Superscript` casam
  `Content::Styled` com o respectivo flag activo.
- Layouter consumer em `layout/text.rs`:
  - `sub`: reduz o tamanho para `0.6×` e desloca a baseline para `-0.2em`.
  - `super`: reduz o tamanho para `0.6×` e desloca a baseline para `+0.3em`.
- `TextStyle.baseline_offset` (Length) transporta o offset; `cursor.rs` aplica-o
  ao posicionamento Y de cada `FrameItem::Text`.

**Scope-out (ADR-0054 graded)**: `offset` e `size` configuráveis mantêm-se
fora de escopo; usam-se os valores vanilla padrão.

```
native_subscript([Content(c)])   → Ok(Content::Styled(c, [Subscript(true)]))
native_subscript([Str("x")])     → Ok(Content::Styled(text("x"), [Subscript(true)]))
native_subscript()               → Err
native_superscript([Content(c)]) → Ok(Content::Styled(c, [Superscript(true)]))
native_superscript([Str("x")])   → Ok(Content::Styled(text("x"), [Superscript(true)]))
native_superscript()             → Err
layout(sequence([a, sub(b), c])) → "abc", "b" 0.6× e abaixo da baseline
layout(sequence([a, super(b), c])) → "abc", "b" 0.6× e acima da baseline
```

## `highlight(body, fill?)` — Passo 449

Elemento de texto vanilla `HighlightElem`. No cristalino modela-se via
`Content::Styled` + `Style::Highlight(Option<Color>)`, reaproveitando a
cadeia de estilos existente.

**Argumentos**: 1 posicional `Content | Str`. Named opcional `fill: Color | none`
(default amarelo Typst `rgb("#ff236")` / `rgba(255, 242, 54, 255)`).

**Implementação**:
- `Content::highlight(body, fill)` emite `Content::Styled(body, [Style::Highlight(fill)])`.
- `native_highlight` valida `fill`; quando omitido usa o amarelo padrão; quando
  `none` emite `Style::Highlight(None)` (desactiva herança).
- Show rule: `NodeKind::Highlight` casa `Content::Styled` com `highlight` definido.
- Layouter consumer em `layout/text.rs` + `cursor.rs`:
  - Ao posicionar um run de texto com `TextStyle.highlight = Some(color)`, emite
    primeiro um `FrameItem::Shape` rectangular (`ShapeKind::Rect`) com o
    preenchimento, cobrindo a altura da linha, e depois o `FrameItem::Text`.
  - `fill: none` resulta em `TextStyle.highlight = None` e não emite shape.
- Export PDF: reusa `FrameItem::Shape` com `fill` (zero alterações no export).

**Scope-out (ADR-0054 graded)**: `extent`, `top-edge`, `bottom-edge`; gradient
fill; math mode.

```
native_highlight([Content(c)])              → Ok(Content::Styled(c, [Highlight(Some(yellow))]))
native_highlight([Content(c)], fill: red)   → Ok(Content::Styled(c, [Highlight(Some(red))]))
native_highlight([Content(c)], fill: none)  → Ok(Content::Styled(c, [Highlight(None)]))
native_highlight()                          → Err
layout(sequence([a, highlight(b), c]))      → "abc", "b" com rect amarelo por detrás
```
