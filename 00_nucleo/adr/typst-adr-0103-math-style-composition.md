# ⚖️ ADR-0103: Composição math style — outer-wins via `Option::or`

**Status**: `EM VIGOR`
**Data**: 2026-05-20
**Passo promotor**: P311b.4 (refinamento empírico)
**Categoria**: Arquitectural / Regras de composição cross-variant
**Cross-ref**: ADR-0102 (mecanismo MathStyled; esta ADR é
              complemento), ADR-0033 (paridade observable)

---

## Contexto

Wraps math style podem aninhar arbitrariamente em código user:
`bb(cal(x))`, `bold(italic(x))`, `upright(italic(x))`, `script(sscript
(x))`, etc. A composição precisa de regras determinísticas para que
o output seja previsível e paritário com vanilla.

Diagnóstico P311a §3.3 propôs regras compostas (variant outer-wins,
bold ortogonal bitwise, size multiplicativo). P311b.4
materialização empírica refinou:

- Bold/italic: ambos são `Option<bool>` em `Content::MathStyled`;
  composição uniforme via `Option::or` é mais simples que distinção
  "ortogonal bitwise" vs "outer-wins".
- Size: vanilla `EquationElem.size` é set pelo último wrap aplicado
  (outer-wins), **não multiplicativo**. Diagnóstico §3.5 estava
  incorreto; P311b.4 alinha com vanilla.

---

## Decisão

`apply_math_style` aplica uma única regra universal de composição:
**`Option::or` em cada field** (kind/bold/italic). Outer ganha se
set; inner herda se outer é `None`.

```rust
fn apply_math_style(
    body: &Content,
    kind: Option<MathStyleKind>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> Content {
    match body {
        Content::MathStyled { kind: ik, bold: ib, italic: ii, body: ibody, .. } =>
            apply_math_style(
                ibody,
                kind.or(*ik),       // outer-wins
                bold.or(*ib),       // outer-wins (em vez de bitwise OR)
                italic.or(*ii),     // outer-wins
            ),
        // ... casos folha (MathIdent/MathText) aplicam map_glyph
        // com kind.unwrap_or(Plain), bold.unwrap_or(false), italic.unwrap_or(false)
    }
}
```

---

## Regras de composição (4 casos canónicos)

### Regra 1 — Variant glyph outer-wins

`bb(cal(x))`:
- Inner `cal(x)`: `MathStyled { kind: Some(Chancery), ... body: x }`.
- Outer `bb(...)`: `MathStyled { kind: Some(DoubleStruck), body: inner }`.
- Composição: outer `kind = Some(DS).or(Some(Chancery)) = Some(DS)`.
- Resultado: `MathIdent("𝕩")` (DS x = U+1D569).

**Paridade vanilla**: `body.set(EquationElem::variant, DS)` chama
após `body.set(variant, Chancery)`; vanilla last-set-wins =
outer-wins (porque outer é aplicado por último no top-down stack
de wraps).

### Regra 2 — Bold flag outer-wins

`bold(bb(x))`:
- Inner `bb(x)`: `MathStyled { kind: Some(DS), bold: None, ... body: x }`.
- Outer `bold(...)`: `MathStyled { kind: None, bold: Some(true), body: inner }`.
- Composição:
  - `kind = None.or(Some(DS)) = Some(DS)` — outer não set, inner ganha.
  - `bold = Some(true).or(None) = Some(true)` — outer ganha.
- Resultado: DS lowercase plane (DS x; vanilla DS não tem variant
  Bold separado em Unicode plane).

**Nota**: a regra "ortogonal" mencionada no diagnóstico §3.3 é
preservada pela natureza separada dos fields (`kind` vs `bold`).
Não há conflito real entre os dois — `Option::or` em cada um basta.

### Regra 3 — Italic flag outer-wins

`upright(italic(x))`:
- Inner `italic(x)`: `MathStyled { italic: Some(true), ... }`.
- Outer `upright(...)`: `MathStyled { italic: Some(false), body: inner }`.
- Composição: `italic = Some(false).or(Some(true)) = Some(false)`
  — outer wins.
- Resultado: 'x' literal (Plain + italic=false; map_glyph passa-
  through).

### Regra 4 — Size variant outer-wins

`script(sscript(x))`:
- Inner `sscript(x)`: `MathStyled { kind: Some(SScript), ... }`.
- Outer `script(...)`: `MathStyled { kind: Some(Script), body: inner }`.
- Composição: `kind = Some(Script).or(Some(SScript)) = Some(Script)`
  — outer wins.
- Resultado: `style.size *= 0.7` (factor de Script, NÃO 0.7 × 0.5).

**Refuta** diagnóstico P311a §3.5 que propôs composição
multiplicativa. P311b.4 empírico confirma vanilla outer-wins
(consistente com Regra 1).

---

## Casos especiais

### Operadores texto (`MathOp`) passa-through

`bb(op("sin"))`:
- `MathOp` (operador texto vanilla `sin`/`lim`/etc.) não recebe
  variant glyph. Função `apply_math_style` arm `Content::MathOp =>
  body.clone()` (passa-through).
- Resultado: `MathOp { text: "sin" }` inalterado mesmo dentro de
  `bb(...)` wrap. Paridade vanilla (`sin` é semanticamente palavra,
  não símbolo).

### Auto-itálico em `MathIdent` suprimido

Pré-P311b.4: `MathIdent` handler em `layout_node` aplica auto-
itálico via `is_single_letter_var`. Pós-P311b.4: wraps `MathStyled`
SUPRIMEM auto-itálico via `math_style.italic = false` antes de
descer ao body transformado. Itálico explícito (`italic(x)`)
honrado via codepoint Bold/Italic plane já encoded em
`apply_math_style`.

Sem este override, `bb(x)` produziria 𝕩 + auto-itálico da fonte
(possivelmente fallback porque DS plane não tem variant italic).

### `cramped` propaga

`script(x)`/`sscript(x)` setam `cramped: Some(true)` em
`Content::MathStyled`. Field propagado para sub-elementos via
`apply_math_style`; consumer downstream (script-specific kerning
em `attach.rs`) ainda não materializado (sub-passo P311b.X
candidato se cobertura empírica exigir).

---

## Consequências

### Imediatas (P311b)

1. Composição determinística e simples (uma só regra `Option::or`).
2. Paridade vanilla preservada para 5 casos canónicos testados em
   P311b.5.
3. Auto-itálico suprimido em wraps math style — sem regressão em
   tests `MathIdent` sem wrap (caminho default não-MathStyled
   preserva comportamento original).

### Futuras (pós-P311b)

- Se vanilla revelar regra mais complexa para um caso de borda
  específico (e.g. `bold + italic` combinatória especial em Unicode
  Bold Italic plane), refinamento aditivo aplicado em
  `apply_math_style` sem mudar arquitectura.
- Se materialização StyleChain real (DEBT-1) for prosseguida,
  composição via Style merge pode substituir `apply_math_style`;
  regras conceptualmente idênticas (outer-wins é semântica natural
  de Style merge top-down).

---

## Refutações ao diagnóstico P311a §3.3

| Proposta diagnóstico | Refutação P311b.4 |
|---|---|
| "Bold ortogonal bitwise OR" | `Option::or` uniforme é mais simples; bold é field separado de kind, sem conflito |
| "Size compõe multiplicativamente" | Vanilla outer-wins; tests P311b.4 confirmam |
| "Italic outer-wins Option<bool>" | **Preservado** — única regra do diagnóstico mantida literal |
| "Variant glyph outer-wins" | **Preservado** |

Refinamentos refletidos em L0 `entities/content.md` §"Composição
cross-variant" + ADR-0103.

---

## Não-objectivos desta ADR

- Não trata mapping Unicode (ADR-0102 + `entities/math_style.md`).
- Não documenta semântica de `cramped` consumer downstream (sub-
  passo futuro).
- Não cobre interacções com `Content::Equation` block/inline
  (ortogonais; size variant não muda block style).

---

## Referências

- **ADR-0102** — Mecanismo Math-Style (variant `Content::MathStyled`;
  esta ADR é complemento).
- **ADR-0033** — Paridade observable.
- **Diagnóstico P311a §3.3** — proposta original (parcialmente
  refinada por P311b.4 empírico).
- **P311b.4** — implementação `apply_math_style` + handler.
- **P311b.5** — tests E2E que confirmam regras (6 tests).
- `01_core/src/rules/math/layout/mod.rs:apply_math_style` —
  implementação canónica.
