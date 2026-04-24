# Diagnóstico do estado actual de shaping — Passo 135

**Data**: 2026-04-24
**Motivação**: reavaliação do critério de fecho de DEBT-1.
`StyleDelta` está totalmente capturado pós-134 (`font`, `lang`,
`par.leading`, `weight`, `tracking`, outros) mas valores são
inertes — layout não os consome. Paridade ADR-0033 lido literal
exige output observacional equivalente ao vanilla, não só
captura.

---

## 1. Estado do consumer actual

### 1.1 Mapa `StyleDelta` → efeito observável

| Campo | Capturado em | Consumer em layout | Efeito no PDF |
|-------|--------------|--------------------|---------------|
| `bold` | Passo 30 | `TextStyle.bold` → `FrameItem::Text.style.bold` → export selects `F2` (Helvetica-Bold) | **Activo** |
| `italic` | Passo 30 | `TextStyle.italic` → export selects `F3` (Helvetica-Oblique) | **Activo** |
| `size` | Passo 30 | `TextStyle.size` → export `Tf` operator | **Activo** |
| `fill` | Passo 102 | `TextStyle.fill` — `FrameItem::Text.style.fill` | **Parcial** (lido pelo export mas não propagado como `rg`/`RG` em todos os sítios) |
| `heading_level` | Passo 99 | `TextStyle.heading_level` — usado para scaling de size em heading | **Activo indirecto** |
| `weight` | Passo 126/129 | **Nenhum** | **Inerte** |
| `tracking` | Passo 127 | **Nenhum** | **Inerte** |
| `leading` | Passo 128/134 | **Nenhum** | **Inerte** |
| `lang` | Passo 130/131B | **Nenhum** | **Inerte** |
| `font` | Passo 132B | **Nenhum** | **Inerte** |

**5 campos activos (bold/italic/size/fill/heading_level), 5 campos
inertes (weight/tracking/leading/lang/font)**. Inertes são
exactamente os capturados em 126–132B.

### 1.2 `TextStyle` como ponte

Ficheiro: `01_core/src/entities/layout_types.rs:104`.

```rust
pub struct TextStyle {
    pub bold:          bool,
    pub italic:        bool,
    pub size:          Pt,
    pub fill:          Option<Color>,
    pub heading_level: Option<u8>,
}
```

**5 campos**. Apenas subset de `StyleDelta`. `From<&StyleChain>`
em `style_chain.rs:218` mapeia só estes 5.

**DEBT-48 está ENCERRADO** (Passo 100) — `FrameItem::Text.style`
ainda é `TextStyle` (não `StyleChain`). A ponte é aceite como
definitiva na sua forma actual; extensão de `TextStyle` com os
novos campos é o caminho futuro (não substituição por
`StyleChain`).

### 1.3 `FontBook` em L1

Ficheiro: `01_core/src/entities/font_book.rs`.

- **Existe**: `struct FontBook { infos: Vec<FontInfo> }`.
- **`select`**: sim — `fn select(family, variant) -> Option<usize>`.
- **Integração com `FontWeight`/`FontStretch`**: sim — usam na
  selecção de variante mais próxima.
- **Integração com `FontList` (132B)**: **nula**. `FontList` foi
  materializado em 132B mas nenhum consumer faz `book.select(...)`
  para cada FontFamily ainda.
- **Dependência externa**: `ttf_parser` em L3 (`fonts.rs`,
  `font_metrics.rs`) para parse; L1 só conhece metadata.

### 1.4 `rustybuzz` e shaping pipeline

`grep rustybuzz 01_core/ 03_infra/`: **zero hits**.

**Não há shaping engine integrado**. Text layout usa:
- `metrics.advance(word, size)` — largura por caractere, sem
  shaping OpenType.
- `metrics.rs:75` comentário: "Métricas fixas monoespaçadas —
  para layout sem FontBook real".

Vanilla `typst-library` usa `rustybuzz::shape(...)` em
`text/shape.rs`; cristalino **não tem esse caminho**. Substituir
por integração real é trabalho grande.

### 1.5 Pipeline completo Typst → PDF (estado actual)

```
texto Typst
  → parse          (01_core/src/rules/parse/)
  → AST            (entities/ast/)
  → eval           (01_core/src/rules/eval/rules.rs)
     — captura #set text/#set par/#set heading/#set figure/#set page
     — produz Content + StyleDelta pushed on StyleChain
  → Content        (entities/content.rs)
  → introspect     (rules/introspect.rs) — counters, labels
  → layout         (rules/layout/mod.rs)
     — resolve StyleChain → TextStyle (bold/italic/size/fill/heading_level)
     — text processing: metrics.advance() per word
     — produz Frame(FrameItem::Text { pos, text, style })
  → export         (03_infra/src/export.rs)
     — text: `BT /F{1|2|3} {size} Tf {x} {y} Td ({text}) Tj ET`
     — F1 Helvetica, F2 Helvetica-Bold, F3 Helvetica-Oblique
     — size usado; bold/italic → selecção PDF font ref
  → PDF bytes
```

**Gap estrutural**: entre StyleChain e FrameItem::Text, apenas 5
propriedades atravessam (via TextStyle). Resto fica em
`StyleDelta` sem caminho para o frame.

---

## 2. DEBTs existentes relacionados

### 2.1 DEBTs abertos relevantes para shaping

| DEBT | Título | Relação com shaping |
|------|--------|---------------------|
| DEBT-1 | StyleChain — parcialmente resolvido | **Central**: fecho depende de consumer integral |
| DEBT-2 | Closures eager vs lazy | Não relacionado |
| DEBT-8 | Motor de equações | Tangencial: math layout usa TextStyle própria |
| DEBT-9 | Cobertura paridade | Tracking, não bloqueio |
| DEBT-33 | Bézier bbox | Não relacionado |
| DEBT-34d/e | Grid auto/colspan | Layout mas não shaping |
| DEBT-35b | Cache available_width | Layout mas não shaping |
| DEBT-42 | `get_unchecked` scanner | Não relacionado |
| DEBT-43 | Linter whitelist | Não relacionado |
| DEBT-50 | Show selector Strong/Emph | Lateral |

**Apenas DEBT-1 é central**. Nenhum DEBT dedicado a consumer
de shaping existe.

### 2.2 DEBT-48 — análise

**ENCERRADO (Passo 100)**. A decisão final manteve `TextStyle`
como ponte plana; extensão vs substituição foi deferida.

Para este passo: **não reabrir**. A via de menos resistência é
**estender `TextStyle`** com novos campos
(`weight/tracking/leading/lang/font`) — trabalho de extensão
do pattern, não refactor da ponte.

### 2.3 Candidatos registados em relatórios 126-134

| Registo | Origem |
|---------|--------|
| Consumer `weight` em layout (selecção variante ou faux-bold) | 126/129 relatórios |
| Consumer `tracking` em layout (offset inter-glyph) | 127 |
| Consumer `leading` em layout (vertical) | 128/134 |
| Consumer `font` em layout (lookup via FontBook + fallback) | 132B |
| Consumer `font` dict (autorizar `regex` + `Covers` concreto) | 132B |
| Materializar `Region` (para hint "put region in region parameter") | 131B |
| Materializar `Dir` + `Lang::dir()` | 131B |
| Expandir constantes `Lang::*` on-demand | 131B |
| Extract helper `eval_with_warnings` em L1 test harness | 127 |

---

## 3. Gap analysis

### 3.1 Tabela de gaps

| Gap | Dependências | Estimativa | Bloqueios |
|-----|--------------|-----------:|-----------|
| Estender `TextStyle` com `weight/tracking/leading/lang/font` | — | XS | — |
| Propagar novos campos de StyleChain para TextStyle em `From` | — | XS | — |
| Consumer `tracking` em `metrics.advance` (offset inter-word/glyph) | `TextStyle.tracking` | S | — |
| Consumer `leading` em layout vertical (espaçamento entre linhas) | `TextStyle.leading`; medir lines | S | — |
| Consumer `weight` faux-bold OR selecção variante | `FontBook::select` para variante; ou `T*` stroke PDF | S (faux) / M (variante) | Fonte PDF fixa (F1/F2/F3) |
| Consumer `font` string simples — selecção por nome | `FontBook::select` + integração export PDF (múltiplos fonts); PDF font embedding | M | Sistema de fontes L3 (hoje hardcode F1-F3) |
| Consumer `font` array (fallback chain) | Shaping per-codepoint com tentativa sequencial | M | Requer `font` string simples primeiro |
| Consumer `font` dict (covers) | `Covers` concreto + `regex` autorizado; ADR-0054 | M-L | **ADR-0054 (proposta futura)** |
| Consumer `lang` shaping features | `rustybuzz` integrado em L1 ou L3 | **L** | Integração stack shaping (fundacional) |
| Consumer `lang` hyphenation | Crate hifenização (ex: `hyphenation`) autorizada | M | Autorização crate nova |
| Embedding PDF fonts reais | L3 font loader + font subsetting | L | Infra PDF |
| Shaping engine (rustybuzz) | Autorização crate + integração L1/L3 | **XL** | Decisão arquitectural grande |

### 3.2 Breakdown por dificuldade

- **XS (2)**: estender TextStyle, propagar via From.
- **S (3)**: tracking, leading, weight faux-bold.
- **M (4)**: font string, font array, lang hyphenation, font dict com covers.
- **L (1)**: lang shaping features (requer rustybuzz integrado).
- **XL (1)**: shaping engine completo.
- **M (infra)**: embedding PDF fonts reais.

**Ordem lógica de ataque**:
1. XS first (TextStyle extension) — habilita todos os outros.
2. S batch (tracking, leading, weight-faux-bold) — efeito visível
   incremental.
3. M batch (font string + array + hyphenation).
4. Decisão L/XL: shaping engine é refactor fundacional —
   provavelmente separar em série dedicada.
5. Font dict depende de ADR-0054 (autorizar regex).

### 3.3 Observações

- **Infra PDF é limitação concreta**: F1/F2/F3 Helvetica hardcoded
  no export. Font real requires PDF font embedding + CMap —
  trabalho de infra L3 independente do consumer L1.
- **rustybuzz ausente** é o gap maior. Shaping features
  (ligatures, kern, substitution, bidi) exigem. Alternativa
  simples: layout caracter-a-caracter continua (cristalino
  actual) sem features — paridade limitada mas functional.
- **Paridade ADR-0033 graded**: paridade "output visível"
  (tracking, leading, weight, size, fill, font nome) é
  alcançável sem rustybuzz. Paridade "shaping real" (features,
  script-aware) requer o XL.

---

## 4. Roadmap revisto para fecho de DEBT-1

### Fase A — Extensão de TextStyle (XS, 1 passo)

**136**: estender `TextStyle` com `weight/tracking/leading/lang/font`
(ausente `Option<u16>/Length/Length/Lang/FontList`) +
propagar via `From<&StyleChain>`. Zero efeito visível, infra
pronta.

### Fase B — Consumers S (3 passos)

**137**: consumer `tracking` — offset em `metrics.advance` ou
item space em export.

**138**: consumer `leading` — aumentar `line_height` em
layout vertical.

**139**: consumer `weight` faux-bold — PDF `2 Tr` (stroke) ou
offset double-print. Variante real fica para fase C.

### Fase C — Consumer M (4-5 passos, opcional ordem)

**140**: consumer `font` string — `FontBook::select(name)`.
Requer infra PDF font embedding (passo adjacente ou pré-req).

**141**: consumer `font` array — fallback chain.

**142** (potencial): consumer `lang` hyphenation — crate
`hyphenation` autorizada + ADR.

**143** (dependente 142): aplicar hyphenation na layout.

### Fase D — Opcional (L/XL; decidível no fim da fase C)

- ADR-0054 autorizar `regex` + `Covers` concreto para
  font dict.
- Integração rustybuzz (XL) — série dedicada.

### Fecho DEBT-1

**Quando fase A + B + C encerrarem** com paridade observável
para os inputs documentados. Gaps L/XL não bloqueiam se
registados explicitamente como "escopo reduzido" no fecho.

**Estimativa**: **4-8 passos** entre XS/S, mais eventuais
M/L. Fase A+B = 4 passos ≈ 4-6h cumulativo. Fase C = 4-5
passos ≈ 6-10h. Total: **10-16h** para paridade observável
razoável.

---

## 5. Decisão sobre ADR-0033

**Opção escolhida: (b)** — criar **ADR-0054** "Critério de fecho
de DEBT-1 inclui consumo integral".

**Razão**:
- Mudança de critério de fecho de um DEBT central é decisão
  própria, não nota marginal.
- Precedente: ADR-0052 e ADR-0053 formalizaram materializações
  específicas como decisões próprias.
- ADR-0033 permanece intacta como princípio geral; ADR-0054
  aplica-o a DEBT-1 com especificidade.

---

## 6. DEBT novo proposto — DEBT-52

**Nome**: Consumer integral de `StyleDelta` em layout.

**Escopo**: rastreador dos gaps identificados em 3.1.

**Não é trabalho per se** — é o registo que permite ver o
estado global. Fecha quando todos os gaps forem atacados ou
explicitamente escopados-out.

**Aberto como resultado deste passo**. Número 52 (próximo livre;
último atribuído foi 51).

---

## 7. Resumo executivo

- **5 de 10 campos de StyleDelta são inertes**. Infra de
  consumer falta.
- **TextStyle é a ponte viável** (DEBT-48 encerrado com
  forma actual). Extensão > substituição.
- **rustybuzz ausente** — shaping real é XL, deferir.
- **Paridade observável limitada é alcançável em ~4-8 passos**
  (fase A+B+C sem rustybuzz).
- **Decisão ADR-0033 (opção b)**: ADR-0054 nova formaliza
  critério de fecho.
- **DEBT-52 novo** rastreia gaps.
- **Próximo passo sugerido (136)**: estender TextStyle
  (Fase A, XS).
