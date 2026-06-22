# Prompt L0 — `stdlib/text` — smartquote, decoração textual, lorem e smallcaps
Hash do Código: 44dffb26

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

## `smallcaps(body)` — Passo 408

Elemento de texto vanilla `SmallcapsElem` que transforma o body em small
capitals. No cristalino o **consumer é stub transparente**: o shaping OpenType
(`smcp` / `c2sc`) está scope-out (DEBT-53, XL futuro). A feature existe no
pipeline (parse, eval, Content variant) sem fallback software incorreto.

**Argumentos**: 1 posicional `Content | Str`. Zero named args.

**Implementação**:
- `Content::SmallCaps { body: Box<Content> }`.
- `plain_text` delega a `body`; `map_content` recursa em `body`; `is_empty`
  delega a `body`; `PartialEq` por `body`.
- Layouter: `Content::SmallCaps { body } => self.layout_content(body)`.

**Scope-out (ADR-0054 graded)**: small caps real requer shaping; não implementar
fallback software (uppercase + scale) porque não é paridade vanilla.

```
native_smallcaps([Content(c)]) → Ok(Content::SmallCaps { body:c })
native_smallcaps([Str("x")])   → Ok(Content::SmallCaps { body:text("x") })
native_smallcaps()             → Err
native_smallcaps([Content(c)], foo:Int(1)) → Err
layout(smallcaps([Hello]))     → identical a layout([Hello]) (stub)
```
