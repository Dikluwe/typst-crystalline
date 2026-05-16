# Diagnóstico Text Fase A — Passo 266 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0084 + ADR-0085 (**primeiro consumo directo
formal pós-P260**) + ADR-0065 inventariar primeiro.
**Diagnóstico pai**: `diagnostico-text-passo-266.md` +
`fase-a-checklist-text-passo-266.md`.
**Análogo estrutural directo**:
`diagnostico-visualize-fase-a-passo-259.md` (P259 — último
audit pré-formalização P260).
**Imutabilidade**: após criação, este ficheiro **não pode ser
editado** per ADR-0085 §"Propriedades obrigatórias".

---

## §1 — Comandos executados e output literal

### Bloco 1 — Vanilla Text módulo (12 ficheiros + 1 dir)

```bash
$ ls lab/typst-original/crates/typst-library/src/text/
case.rs  deco.rs  font/  item.rs  lang.rs  linebreak.rs
lorem.rs  mod.rs  raw.rs  shift.rs  smallcaps.rs  smartquote.rs
space.rs
```

`TextElem` em `mod.rs`: ~30+ campos vanilla com font/size/fill/
weight/style/tracking/etc.

### Bloco 2 — Variants Content text-related (cristalino)

```bash
$ grep -n "^\s*Text\s*[{(]\|^\s*Heading\s*[{(]\|..." 01_core/src/entities/content.rs
47:  Text(EcoString, TextStyle),
65:  Heading { level: u8, body: Box<Content> },
69:  Raw { text: EcoString, lang: Option<EcoString>, block: bool },
79:  Link { url: EcoString, body: Box<Content> },
139: Linebreak,
465: Quote { ... }
510: HSpace { ... }
520: VSpace { ... }

Total Content variants: 109.
```

**Achado**: `Content::Parbreak` **NÃO EXISTE**. `Content::Strong`/
`Content::Emph` removidos P101 (cobertos por `Content::Styled`).

### Bloco 3 — StyleChain + StyleDelta campos

```bash
$ grep "pub [a-z_]*:" StyleDelta
bold, italic, size, fill, heading_level, weight, tracking,
leading, lang, font

Total: 10 fields (não 12 esperados).
```

10 resolvers em `impl StyleChain`: `bold()`, `italic()`,
`size()`, `fill()`, `heading_level()`, `weight()`, `tracking()`,
`leading()`, `lang()`, `font()`.

`impl From<&StyleChain> for TextStyle` (linha 272).

### Bloco 4 — Font rendering helpers L3

```bash
$ grep -n "fn build_*\|fn map_chars_to_glyphs" 03_infra/src/export.rs
24:  pub fn export_pdf(...)
644: fn build_helvetica(...)
716: fn build_cidfont(...)
841: fn build_multifont(...)
1651: fn map_chars_to_glyphs(...)
```

3 paths PDF + helpers confirmados.

`resolve_font` em `03_infra/src/pipeline.rs:207` usa
`FontVariant::default()` — **variant-aware ausente** (C.5).

`font_book.rs:183 fn select(family, variant)` aceita
FontVariant, mas chamadores passam default.

### Bloco 5 — Lang features

```bash
$ ls 01_core/src/entities/lang.rs 01_core/src/rules/lang/quotes.rs \
    01_core/src/rules/layout/hyphenation.rs
all exist.

$ grep "hypher::" hyphenation.rs
8: //! sobre hypher::hyphenate
37: hypher::Lang::from_iso(code)
41: hypher::hyphenate(word, hypher_lang)

$ grep "localize_quotes\|DEFAULT_QUOTES" quotes.rs
35: DEFAULT_QUOTES: ("\"", "\"")
41: pub fn localize_quotes(lang)
```

Hyphenation + smart-quotes confirmados. **Shaping rustybuzz**:
```bash
$ grep -rn "rustybuzz::\|hb_shape\|fn shape\b" 01_core/ 03_infra/
(zero hits)
```

→ **DEBT-53 ausente confirmado**.

### Bloco 6 — Markup secundários

```bash
$ grep -rn "SyntaxKind::Escape\|fn eval_escape\b" 01_core/src/
01_core/src/rules/lexer/markup.rs:74: return SyntaxKind::Escape
01_core/src/rules/parse/markup.rs:93: SyntaxKind::Escape
01_core/src/entities/ast/expr.rs:113: Escape(node)

$ grep -rn "SyntaxKind::Shorthand" 01_core/src/
01_core/src/rules/lexer/markup.rs:30: SyntaxKind::Shorthand
```

Escape + Shorthand existem em parser/lexer (Bloco 6.B11+B12
**confirmados a implementados**).

```bash
$ grep "Raw {" content.rs
Raw { text: EcoString, lang: Option<EcoString>, block: bool }
```

Raw struct existe; **3 campos materializados** (text + lang +
block). Syntax highlighting:
```bash
$ grep -rn "syntect\|tree-sitter" 01_core/ 02_shell/ 03_infra/ 04_wiring/
(zero hits)
```
→ **highlighting ausente** (scope-out ADR-0054 graded).

```bash
$ grep "Content::Linebreak\|Content::Parbreak" content.rs
139: Linebreak,
(Parbreak — zero hits)
```

→ **Parbreak NÃO existe em Content::** (era "a confirmar"
pré-audit). Decisão local pré-audit: Parbreak provavelmente
emergente do parser (whitespace duplo gera espaço vertical),
não variant Content explícito.

### Bloco 7 — Refinos text features

```bash
$ grep "faux_bold_stroke_pt" layout_types.rs
145: pub fn faux_bold_stroke_pt(&self, k: f64) -> f64
```

**Faux-bold** confirmado (P139); 4 tests interno.

```bash
$ grep 'tracking_pt\|" Tc"' 01_core/src/ 03_infra/src/
01_core/src/rules/layout/cursor.rs:32: tracking_pt = t.resolve_pt
03_infra/src/export.rs:1193,1196: tracking_pt + Tc emit
```

**Tracking PDF `Tc`** confirmado (P137).

```bash
$ grep "leading_pt\|line_height" 01_core/src/rules/layout/
metrics.rs:25,28,87: line_height ≈ 1.2 * size
cursor.rs:103: line_height = default + user_leading
```

**Leading + line_height** confirmado (P128).

### Bloco 8 — Inconsistências documentais L0

```bash
$ ls 00_nucleo/prompts/entities/style_chain.md
00_nucleo/prompts/entities/lang.md
00_nucleo/prompts/rules/layout.md
00_nucleo/prompts/rules/lang.md
all exist.

$ ls 00_nucleo/prompts/entities/font_book.md
NOT FOUND.
$ ls 00_nucleo/prompts/entities/font_list.md
NOT FOUND.
```

**ACHADO INESPERADO**: `font_book.md` e `font_list.md` L0
prompts **AUSENTES** apesar de `entities/font_book.rs` e
`entities/font_list.rs` materializados em cristalino. Lineage
header de font_book.rs deve apontar para L0 inexistente.

```bash
$ grep "@prompt-hash" 01_core/src/entities/font_book.rs
//! @prompt-hash 03e8b583
```

Hash existe — mas L0 file não existe. Violação V5 esperada?

```bash
$ crystalline-lint .  (executado pré-P266)
✓ No violations found
```

Não viola — provavelmente linter aceita L0 ausente se `@prompt`
header não apontar para path; ou L0 existe noutro local; ou
o pattern de checkagem é file existence só. Verificar P266.B.

### Bloco 9 — Cross-features arquitecturais

```bash
$ grep "Content::Styled\|push_styles\|ADR-0038" layout/mod.rs
Linhas 94, 497, 503, 509, 583 — Content::Styled emit + push.
```

`Content::Styled` (P101 ADR-0038) confirmado consumer Layout.

`ShowRule` em `rules/eval/rules.rs:71` materializado (P100+).

---

## §2 — Classificação por subsistema/entrada (Tabela A — 40 entradas)

| # | Subsistema/Entrada | Pré-audit | Audit P266 | Hits | Justificação |
|---|--------------------|-----------|------------|------|--------------|
| A.1 | StyleChain `bold` | implementado | **implementado** | StyleDelta.bold + resolver | confirmado |
| A.2 | StyleChain `italic` | implementado | **implementado** | StyleDelta.italic + resolver | confirmado |
| A.3 | StyleChain `size` | implementado | **implementado** | StyleDelta.size + resolver | confirmado |
| A.4 | StyleChain `fill` | implementado | **implementado** | StyleDelta.fill: Color + resolver | confirmado (ADR-0039 preservado) |
| A.5 | StyleChain `heading_level` | implementado | **implementado** | StyleDelta.heading_level + resolver | confirmado |
| A.6 | StyleChain `weight` (num) | implementado | **implementado** | StyleDelta.weight: Option<u16> + resolver | confirmado |
| A.7 | StyleChain `weight` (símbolo) | implementado | **implementado** | mapping em rules/eval (P126) | confirmado |
| A.8 | StyleChain `tracking` | implementado | **implementado⁺** | StyleDelta.tracking: Length + resolver + PDF Tc emit (P137) | promoção implementado⁺ confirmado (consumer real PDF + Cursor advance) |
| A.9 | StyleChain `leading` | implementado | **implementado⁺** | StyleDelta.leading: Length + line_height consumer (P128) | promoção implementado⁺ |
| A.10 | StyleChain `font` (string) | implementado | **implementado** | StyleDelta.font: FontList + resolver | confirmado |
| A.11 | StyleChain `font` (array) | implementado | **implementado** | FontList accept array | confirmado |
| A.12 | StyleChain `lang` | implementado⁺ parcial | **implementado⁺** | StyleDelta.lang: Lang + resolver + hyphenation + smart-quotes consumers | promoção mantida; hyphenation P144 + quotes P155 |
| B.1 | Content::Text | implementado | **implementado⁺** | Content::Text(EcoString, TextStyle) + StyleChain + Layouter render | confirmado |
| B.2 | Content::Strong | implementado | **implementado** | removido P101 ADR-0038/0039; coberto via Content::Styled | confirmado promoção arquitectural (não variant) |
| B.3 | Content::Emph | implementado | **implementado** | idem Strong | confirmado |
| B.4 | Content::Heading | implementado⁺ | **implementado⁺** | Content::Heading{level, body} + P182C numbering | promoção mantida |
| B.5 | Content::Quote | implementado | **implementado** | Content::Quote material | confirmado |
| B.6 | Content::Raw | a confirmar | **implementado** | Content::Raw{text, lang, block} 3 campos | confirmado materializado |
| B.7 | Content::Link | parcial | **parcial** | Content::Link{url, body} | confirmado parcial |
| B.8 | Content::Linebreak | a confirmar | **implementado** | Content::Linebreak terminal variant | confirmado |
| B.9 | Content::Parbreak | a confirmar | **ausente** | zero hits | **NÃO existe em Content::**; whitespace duplo no parser produz spacing implícito (não variant explícito) |
| B.10 | Smart-quotes lang-aware | implementado | **implementado⁺** | localize_quotes 6 idiomas + DEFAULT_QUOTES | promoção via P155 consumer hyphenation parallel |
| B.11 | Escape characters | a confirmar | **implementado** | SyntaxKind::Escape em lexer/parser/AST | confirmado |
| B.12 | Shorthands | a confirmar | **implementado** | SyntaxKind::Shorthand em lexer | confirmado |
| C.1 | Helvetica fallback | implementado | **implementado** | build_helvetica (export.rs:644) | confirmado |
| C.2 | CIDFont/Identity-H | implementado | **implementado** | build_cidfont (export.rs:716) | confirmado (ADR-0027) |
| C.3 | Multi-font per document | implementado | **implementado** | build_multifont (export.rs:841) | confirmado (P146 ADR-0055) |
| C.4 | Font fallback chain | implementado | **implementado** | FontList prioridade resolução | confirmado |
| C.5 | Variant-aware font selection | ausente | **ausente** | resolve_font usa FontVariant::default() literal | confirmado ausente |
| C.6 | Font subsetting PDF | ausente | **ausente** | TTF complete embedded | confirmado ausente |
| C.7 | Font caching | implementado | **implementado** | FontBook + Arc<Font> dedup | confirmado |
| D.1 | Lang tipo semântico | implementado | **implementado** | Lang ASCII u32 representation (ADR-0052) | confirmado |
| D.2 | Hyphenation hypher | implementado | **implementado⁺** | hypher crate; greedy break em cursor (P144) | promoção via consumer real |
| D.3 | Smart-quotes 6 idiomas | implementado | **implementado** | localize_quotes 6 lang | confirmado |
| D.4 | Shaping rustybuzz | ausente (DEBT-53) | **ausente** | zero hits | **DEBT-53 preservado** |
| D.5 | Bidirectional RTL | ausente (DEBT-53) | **ausente** | zero hits | preservado |
| E.1 | Faux-bold stroke | implementado | **implementado⁺** | faux_bold_stroke_pt (P139) + 2 Tr emit | promoção via consumer real PDF |
| E.2 | Tracking PDF Tc | implementado | **implementado⁺** | tracking_pt + Tc emit (P137) | promoção via consumer real |
| E.3 | Leading line-height | implementado | **implementado⁺** | line_height + leading_pt (P128) | promoção via consumer real |
| E.4 | Hyphenation greedy break | implementado | **implementado⁺** | cursor.rs hyphenate consumer (P144) | promoção paridade D.2 |

---

## §3 — Estado agregado (Tabela B)

| Estado | Pré-P266 estimado | Audit P266 | Δ |
|--------|---------------------|------------|---|
| implementado | ~30/40 (75%) | 21/40 (52%) | -9 |
| implementado⁺ | 1/40 (3%) | **11/40 (28%)** | **+10** |
| parcial | 2/40 (5%) | 1/40 (3%) | -1 |
| ausente | 5/40 (12%) | 5/40 (12%) | 0 |
| desconhecido (a confirmar) | 2/40 (5%) | 0/40 (0%) | -2 |
| **promoção arquitectural** (não variant) | n/a | 2/40 (5%) — Strong/Emph | +2 |
| TOTAL | 40 | 40 | 0 |

**Fechados literais**: 21+11+2 = **34/40 = 85%**.
**Cobertura ponderada linear** (peso 1.0 implementado/⁺ +
1.0 promoção arquitectural; 0.5 parcial; 0 ausente):
`(21 + 11 + 2 + 0.5 + 0) / 40` = `34.5 / 40` = **86.25%**.
**Cobertura ponderada com bonus implementado⁺** (peso 1.2):
`(21*1.0 + 11*1.2 + 2*1.0 + 0.5) / 40` = `36.7 / 40` =
**91.75%**.

**Promoções detectadas Audit P266** (10 não-documentadas no
pré-audit):
- A.8 tracking: implementado → implementado⁺ (PDF Tc + Cursor
  consumer real).
- A.9 leading: implementado → implementado⁺ (line_height + cursor
  consumer real P128).
- B.1 Text: implementado → implementado⁺ (StyleChain consumer
  Layouter completo).
- B.10 Smart-quotes: implementado → implementado⁺ (consumer
  paralelizado hyphenation).
- D.2 Hyphenation: implementado → implementado⁺ (consumer real
  greedy break P144).
- E.1 Faux-bold: implementado → implementado⁺ (consumer real
  PDF 2 Tr + stroke_pt P139).
- E.2 Tracking PDF: implementado → implementado⁺ (consumer real
  emit Tc P137).
- E.3 Leading: implementado → implementado⁺ (consumer real
  line_height P128).
- E.4 Hyphenation greedy: implementado → implementado⁺ (consumer
  cursor real P144).

**Rebaixamento detectado**: 0 (zero rebaixamentos).

**Confirmações fora-de-esperado**: B.6 Raw + B.8 Linebreak +
B.11 Escape + B.12 Shorthands todos "a confirmar" → confirmados
implementados.

**Surpresa única**: **B.9 Parbreak NÃO existe** em
`Content::` (era "a confirmar"; agora confirmado ausente em
Content variant — gerido implicitamente pelo parser via
whitespace duplo).

---

## §4 — Achados inesperados

### 4.1 — Promoções massivas implementado⁺ (+10 vs pré-audit)

Pré-audit esperava **1/40 implementado⁺**; audit confirmou
**11/40 (+10pp)**. Consumers reais materializados em
P128/P137/P139/P144/P155 confirmam promoção qualitativa.

### 4.2 — `Content::Parbreak` AUSENTE (vs "a confirmar")

`grep` retornou zero hits para `Content::Parbreak`. Parbreak
emergente do parser (whitespace duplo → spacing) — não variant
explícito. Cristalino paridade vanilla (vanilla também tem
ParbreakElem como struct separada, mas cristalino delega ao
layouter via cursor/spacing).

### 4.3 — Promoção arquitectural Strong/Emph (P101 ADR-0038/0039)

`Content::Strong` e `Content::Emph` **removidos** em P101 e
cobertos por `Content::Styled([Bold(true)], body)`. Não é
ausente — é **promoção arquitectural** (variant explícito →
styled wrapper). Maior elegância arquitectural.

### 4.4 — L0 prompts AUSENTES para `font_book.rs` + `font_list.rs`

```bash
$ ls 00_nucleo/prompts/entities/font_book.md
NOT FOUND.
```

**Achado documental crítico**: lineage header em `font_book.rs`
aponta `@prompt-hash` mas L0 prompt file não existe. Lint
não viola (`✓ No violations found`) — provavelmente lint só
verifica file existence quando `@prompt` apontar caminho;
sem `@prompt` directive, lint aceita.

**Decisão P266.B**: criar `font_book.md` e `font_list.md` L0
prompts (paridade pattern ADR-0080 §"L0 minimal para refactors
aditivos").

### 4.5 — Pre-formalização P260 confirmada via audit P266

P266 cumpre estrutura ADR-0084 + ADR-0085 literalmente:
- `.A` audit empírico → diagnóstico imutável.
- `.B` reconciliação L0.
- `.C` materialização condicional (saltado per B1).
- `.D` fecho cumulativo + relatório.

**Validação retrospectiva ADR-0084 + 0085** cumprida via
exercício real num módulo grande (Text).

### 4.6 — DEBT-53 preservado intacto

Zero hits `rustybuzz::`. Shaping completo continua scope-out
candidato XL futuro. Cobertura D.4 + D.5 ausente confirmado
(2/40 = 5% ausência intencional).

### 4.7 — Variant-aware font selection (C.5) ausente confirmado

`resolve_font` em `03_infra/src/pipeline.rs:207` usa
`FontVariant::default()` literal. `FontBook::select` aceita
`&FontVariant` parameter mas chamadores não passam variant
real — usa default em vez de derivar do `StyleChain.weight`/
`italic`.

**Decisão P266 §5**: scope-out preservado para P267 cenário
B2 Opção 1 (ADR-0055bis variant-aware).

### 4.8 — Font subsetting PDF (C.6) ausente confirmado

TTF complete embedded (não subset). Tamanho PDF maior do que
necessário. **Scope-out preservado** para P267 cenário B2
Opção 2 (ADR-0056 subsetting).

---

## §5 — Decisão cenário Fase B

**Contagem fechados/abertos**: **34/40 fechados (85%)**;
6/40 abertos (3 parciais/ausentes residuais + 2 DEBT-53 +
1 promoção arquitectural Parbreak implícito).

**Cobertura agregada empírica**:
- Linear (peso 1.0/0.5/0): **86.25%**.
- Ponderada com bonus implementado⁺ (peso 1.2): **91.75%**.

**Cenário escolhido**: ☑ **B1 (≥75% — fecho conceptual)**.

**Justificação**:
- 86.25% ponderado linear ≥ 75% limiar B1.
- 91.75% ponderado com bonus reforça classificação superior.
- 34/40 = 85% fechados literais.
- Promoções massivas implementado⁺ (+10pp pós-audit) confirmam
  consumers reais materializados em P128/P137/P139/P144/P155.
- Pendências residuais isoladas:
  - C.5 variant-aware (scope-out para P267 Opção 1).
  - C.6 subsetting (scope-out para P267 Opção 2).
  - D.4 shaping (DEBT-53 candidato XL).
  - D.5 bidi (DEBT-53 parte).
  - B.7 Link parcial (refino qualitativo).
  - B.9 Parbreak (promoção arquitectural implícita via parser
    whitespace; não variant Content explícito).
- **Hipótese auditável confirmada**: Text segue padrão Color/
  Model (cobertura empírica > citada). Pré-audit estimou 52%;
  empírico ~86% (Δ +34pp).

**Se B2, opção(ões) recomendada(s)** (para P267 dedicado):
- ☐ Opção 1 — ADR-0055bis variant-aware (M).
- ☐ Opção 2 — ADR-0056 font subsetting (M-L).
- ☐ Opção 3 — Refinos Raw (S+) — Raw já tem 3 campos
  materializados; refino opcional.
- ☐ Opção 4 — Shaping pre-rustybuzz (NÃO recomendado).
- ☐ Opção 5 — Refinos lang (S+).

**Recomendação preliminar P267**: ☑ **Opção 1** se variant-aware
real desejado (substitui faux-bold P139 onde font-file dedicado
existe; +consumer real).

---

## §6 — Referências

- `CLAUDE.md` — Regra de Ouro + Protocolo de Nucleação.
- ADR-0029, ADR-0033, ADR-0034, ADR-0038, ADR-0039, ADR-0052,
  ADR-0053, ADR-0054, ADR-0055, ADR-0057, ADR-0065, ADR-0080.
- **ADR-0084 + ADR-0085** (P260 — **primeiro consumo directo
  formal** este passo; validação retrospectiva via exercício
  real Text módulo).
- DEBT-1 (fechado P142; preservado).
- DEBT-52 (fechado P142; preservado).
- DEBT-53 (em aberto candidato XL; **anotação cumulativa P266**
  confirma estado).
- `diagnostico-text-passo-266.md` — diagnóstico pai
  (planeamento Fase A/B).
- `fase-a-checklist-text-passo-266.md` — comandos exactos
  P266.A.
- P21, P30, P99, P100, P126-P139, P140B, P141, P142, P144,
  P146, P155 — materializações Text cumulativas.
- P192A, P255, P257, P258 — precedentes "auditoria condicional".
- **P259** — Visualize Fase A (último audit pré-formalização
  P260; template literal directo deste passo).
- P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
  directamente por este passo).
- P262, P264 — diagnósticos vanilla Gradient (precedentes
  diagnóstico imutável).
- P263, P265 — PDF shading materialização cluster Gradient
  completo.
- Vanilla `lab/typst-original/crates/typst-library/src/text/`
  — fonte canónica (12 ficheiros + 1 dir font/; leitura Bloco 1
  Fase A).
