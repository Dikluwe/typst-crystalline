# Fase A — Checklist empírico Text + Template Fase B

**Companheiro de**: `diagnostico-text-passo-266.md`
**Função**: lista executável de comandos `grep`/`view` para
produzir evidência factual sobre subsistemas Text.
**Análogo a**: `fase-a-checklist-visualize-passo-259.md`
(P259) e `fase-a-checklist-model-passo-256.md` (P258).

**Primeiro audit Fase A pós-formalização P260 ADR-0084 +
ADR-0085**.

---

## Comandos Fase A (executáveis em sequência)

### Bloco 1 — Vanilla Text módulo estrutura literal

```bash
ls lab/typst-original/crates/typst-library/src/text/
view lab/typst-original/crates/typst-library/src/text/mod.rs | head -100
grep -rn "^\s*#\[elem\]\|pub struct.*Elem\b" \
  lab/typst-original/crates/typst-library/src/text/
grep -A 50 "^\s*pub struct TextElem\b" \
  lab/typst-original/crates/typst-library/src/text/mod.rs
view lab/typst-original/crates/typst-library/src/text/raw.rs 2>/dev/null | head -50
view lab/typst-original/crates/typst-library/src/text/lang.rs 2>/dev/null | head -50
view lab/typst-original/crates/typst-library/src/text/font.rs 2>/dev/null | head -50
```

**Output**: lista variants vanilla Text + campos por elem.

### Bloco 2 — Variants Content text-related (cristalino)

```bash
grep -n "^\s*Text\b\|^\s*Strong\b\|^\s*Emph\b\|^\s*Heading\b\|^\s*Quote\b\|^\s*Raw\b\|^\s*Link\b" \
  01_core/src/entities/content.rs
grep -n "^\s*Linebreak\b\|^\s*Parbreak\b\|^\s*HSpace\b\|^\s*VSpace\b" \
  01_core/src/entities/content.rs
grep -c "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs
```

### Bloco 3 — StyleChain + StyleDelta campos

```bash
grep -A 30 "^\s*pub struct StyleDelta\b" \
  01_core/src/entities/style_chain.rs
grep -n "pub fn .*\(&self\) ->" \
  01_core/src/entities/style_chain.rs | head -20
grep -A 20 "^\s*impl From<&StyleChain> for TextStyle" \
  01_core/src/entities/style_chain.rs
```

**Critério**: 12 campos StyleDelta esperados; 12 resolvers.

### Bloco 4 — Font rendering helpers L3 (3 caminhos)

```bash
grep -n "fn export_pdf\b\|fn export_pdf_with_font\|fn export_pdf_multifont" \
  03_infra/src/export.rs
grep -n "fn build_cidfont\|fn build_helvetica\|fn map_chars_to_glyphs" \
  03_infra/src/export.rs
grep -n "fn resolve_font\|fn resolve_fonts\|fn collect_fonts_from_doc" \
  01_core/src/ 03_infra/src/
ls 01_core/src/entities/font_book.rs
grep -n "pub fn select\|FontVariant::default" \
  01_core/src/entities/font_book.rs
grep -rn "FontVariant::new\|variant_aware\|match.*FontVariant" \
  01_core/src/ 03_infra/src/ | head -10
```

**Critério**: 3 export paths confirmados; `FontVariant::default()`
usado (variant-aware ausente).

### Bloco 5 — Lang features

```bash
ls 01_core/src/rules/layout/hyphenation.rs
view 01_core/src/rules/layout/hyphenation.rs | head -30
grep -n "hypher::" 01_core/src/rules/layout/hyphenation.rs
ls 01_core/src/rules/lang/quotes.rs
view 01_core/src/rules/lang/quotes.rs | head -30
grep -n "pub fn localize_quotes\|DEFAULT_QUOTES" \
  01_core/src/rules/lang/quotes.rs
ls 01_core/src/entities/lang.rs
grep -n "pub fn from_str\|pub fn as_str" \
  01_core/src/entities/lang.rs
grep -rn "rustybuzz::\|hb_shape\|fn shape\b" 01_core/src/ 03_infra/src/
```

**Critério**: Hyphenation + smart-quotes confirmados; shaping
zero hits (DEBT-53).

### Bloco 6 — Markup secundários

```bash
grep -rn "SyntaxKind::Escape\|fn eval_escape\b" 01_core/src/
grep -rn "SyntaxKind::Shorthand\|Shorthand::LIST" 01_core/src/
grep -n "Content::Raw\|fn eval_raw\|fn layout_raw" \
  01_core/src/
grep -A 10 "Content::Raw\s*{" 01_core/src/entities/content.rs
grep -rn "syntect\|highlight\|tree-sitter" \
  01_core/ 02_shell/ 03_infra/ 04_wiring/
grep -n "Content::Linebreak\|Content::Parbreak\|eval_linebreak" \
  01_core/src/
```

**Critério**: Escape/shorthands/Raw a confirmar; highlighting
ausente; Linebreak/Parbreak implementados.

### Bloco 7 — Refinos text features

```bash
grep -n "faux_bold\|2 Tr\|stroke_pt" 01_core/src/entities/ 03_infra/src/
grep -rn "tracking_pt\|\"Tc\"" 01_core/src/ 03_infra/src/
grep -rn "leading_pt\|line_height" 01_core/src/rules/layout/
grep -n "fn layout_word\|hyphenate" 01_core/src/rules/layout/cursor.rs
```

### Bloco 8 — Inconsistências documentais

```bash
ls 00_nucleo/prompts/entities/style_chain.md
ls 00_nucleo/prompts/entities/font_book.md
ls 00_nucleo/prompts/rules/layout.md
ls 00_nucleo/prompts/rules/lang.md
grep "@prompt-hash" 01_core/src/entities/style_chain.rs
grep "@prompt-hash" 01_core/src/entities/font_book.rs
view 00_nucleo/prompts/entities/style_chain.md | head -50
view 00_nucleo/prompts/rules/lang.md | head -50
```

**Esperado** (precedente P255/P257/P258/P259): inconsistências
documentais prováveis em prompts L0.

### Bloco 9 — Cross-features arquitecturais

```bash
grep -n "Content::Styled\|ADR-0038\|push_styles" \
  01_core/src/rules/eval/ 01_core/src/rules/layout/
grep -n "eval_set_text\|SetText\|#set text" \
  01_core/src/rules/eval/ 01_core/src/entities/content.rs
grep -rn "ShowRule\|fn eval_show\b" 01_core/src/rules/eval/ | head -10
```

---

## Tabela de classificação Fase A (preencher após executar)

### Tabela A — Subsistemas Text + entradas (40 entradas)

| # | Subsistema/Entrada | Pré-audit | Audit P266 | Hits | Justificação |
|---|--------------------|-----------|------------|------|--------------|
| A.1 | StyleChain `bold` | implementado | _ | _ | _ |
| A.2 | StyleChain `italic` | implementado | _ | _ | _ |
| A.3 | StyleChain `size` | implementado | _ | _ | _ |
| A.4 | StyleChain `fill` | implementado | _ | _ | _ |
| A.5 | StyleChain `heading_level` | implementado | _ | _ | _ |
| A.6 | StyleChain `weight` (num) | implementado | _ | _ | _ |
| A.7 | StyleChain `weight` (símbolo) | implementado | _ | _ | _ |
| A.8 | StyleChain `tracking` | implementado | _ | _ | _ |
| A.9 | StyleChain `leading` | implementado | _ | _ | _ |
| A.10 | StyleChain `font` (string) | implementado | _ | _ | _ |
| A.11 | StyleChain `font` (array) | implementado | _ | _ | _ |
| A.12 | StyleChain `lang` | implementado⁺ parcial | _ | _ | _ |
| B.1 | Content::Text | implementado | _ | _ | _ |
| B.2 | Content::Strong | implementado | _ | _ | _ |
| B.3 | Content::Emph | implementado | _ | _ | _ |
| B.4 | Content::Heading | implementado⁺ | _ | _ | _ |
| B.5 | Content::Quote | implementado | _ | _ | _ |
| B.6 | Content::Raw | a confirmar | _ | _ | _ |
| B.7 | Content::Link | parcial | _ | _ | _ |
| B.8 | Content::Linebreak | a confirmar | _ | _ | _ |
| B.9 | Content::Parbreak | a confirmar | _ | _ | _ |
| B.10 | Smart-quotes lang-aware | implementado | _ | _ | _ |
| B.11 | Escape characters | a confirmar | _ | _ | _ |
| B.12 | Shorthands | a confirmar | _ | _ | _ |
| C.1 | Helvetica fallback | implementado | _ | _ | _ |
| C.2 | CIDFont/Identity-H | implementado | _ | _ | _ |
| C.3 | Multi-font per document | implementado | _ | _ | _ |
| C.4 | Font fallback chain | implementado | _ | _ | _ |
| C.5 | Variant-aware font selection | ausente | _ | _ | _ |
| C.6 | Font subsetting PDF | ausente | _ | _ | _ |
| C.7 | Font caching | implementado | _ | _ | _ |
| D.1 | Lang tipo semântico | implementado | _ | _ | _ |
| D.2 | Hyphenation hypher | implementado | _ | _ | _ |
| D.3 | Smart-quotes 6 idiomas | implementado | _ | _ | _ |
| D.4 | Shaping rustybuzz | ausente (DEBT-53) | _ | _ | _ |
| D.5 | Bidirectional RTL | ausente (DEBT-53) | _ | _ | _ |
| E.1 | Faux-bold stroke | implementado | _ | _ | _ |
| E.2 | Tracking PDF Tc | implementado | _ | _ | _ |
| E.3 | Leading line-height | implementado | _ | _ | _ |
| E.4 | Hyphenation greedy break | implementado | _ | _ | _ |

### Tabela B — Estado agregado

| Estado | Pré-P266 estimado | Audit P266 | Δ |
|--------|---------------------|------------|---|
| implementado | ~30/40 (75%) | _ | _ |
| implementado⁺ | 1/40 (3%) | _ | _ |
| parcial | 2/40 (5%) | _ | _ |
| ausente | 5/40 (12%) | _ | _ |
| TOTAL | 40 | _ | _ |
| Cobertura ponderada | ~80-85% (estimativa) | _% | _pp |

### Decisão cenário Fase B

☐ **B1** (≥75% — fecho conceptual). **PROVÁVEL**.
☐ **B2** (55-70% — sub-passos prioritários).
☐ **B3** (≤50% — re-classificação primeiro).

---

## Templates Fase B por cenário

### Cenário B1 — Fecho conceptual (provável)

Materializa:
1. DEBT-53 anotação cumulativa (em aberto candidato XL;
   cross-reference P266).
2. ADR-0054 anotação cumulativa (perfil graded vigente;
   cobertura Text empírica ~80-85%).
3. Actualizar L0 prompts obsoletos.
4. Relatório fecho conceptual Text.

**Magnitude**: XS-S documental.
**Sem ADR nova** (paridade P258 fecho conceptual Model).

### Cenário B2 — Sub-passos prioritários (improvável)

#### Opção 1 — ADR-0055bis variant-aware font selection

- ADR nova primeiro.
- Materialização: `resolve_font` com FontVariant explícito.
- Substitui faux-bold P139 onde font-file dedicado existe.
- Magnitude: M (~2-3h; +10-15 tests).

#### Opção 2 — ADR-0056 font subsetting

- ADR nova + crate `ttf-subset` ou similar (ADR-0018).
- PDF embeds só glyphs usados.
- Magnitude: M-L (~4-6h).

#### Opção 3 — Refinos Raw

- Confirmar Bloco 6 Raw shape actual.
- `Content::Raw { body, lang, block }` atributos completos.
- **Syntax highlighting scope-out** (preserved ADR-0054).
- Magnitude: S+ (~1-2h; +5 tests).

#### Opção 4 — Shaping pre-rustybuzz (não recomendado)

- DEBT-53 endereça shaping completo futuro.
- Aproximação visual sem rustybuzz seria fragmentação.

#### Opção 5 — Refinos lang (mais idiomas)

- Smart-quotes ja/zh/ar/he/etc.
- Magnitude S+ por idioma adicional.

### Cenário B3 — Re-classificação (improvável)

1. Re-classificar Tabela A conservadoramente.
2. Anotação ADR-0054 reflectir cobertura empírica revista.
3. Sub-passos elevação prioritários.

---

## Notas metodológicas

1. **Honesty rule** (precedente P255-P259): classificações
   literais (`grep` hits/no-hits), não interpretativas.

2. **Diagnóstico imutável**:
   `diagnostico-text-fase-a-passo-266.md` em
   `00_nucleo/diagnosticos/` marcado "Imutável após criação
   per ADR-0085". **Primeiro audit Fase A explicitamente sob
   ADR-0085 EM VIGOR pós-P260**.

3. **Pré-execução**: Claude Code lê CLAUDE.md primeiro.

4. **Primeiro audit ADR-0084 consumo directo**: estrutura
   cumpre literalmente (sub-passo .A audit + decisão B1/B2/B3
   + .B docs + .C condicional + .D relatório).

5. **Tempo estimado**:
   - Fase A audit: 30-45 min.
   - Cenário B1 fecho: 30-60 min documental.
   - Cenário B2 Opção 1: 2-3h + ADR-0055bis.
   - Cenário B2 Opção 2: 4-6h + ADR-0056.

---

## Comparação com audits prévios

| Audit | Módulo | Cobertura pré | Cobertura empírica | Δ |
|-------|--------|---------------|--------------------|----|
| P255 | Math DEBT-8 | parcial | fechado | + |
| P257 | Color | 25% (2/8) | 100% (8/8) | +75pp |
| P258 | Model | ~48% declarado | ~73% empírico | +25pp |
| P259 | Visualize | ~60-65% estim | ~52% factual | -8 a -13pp |
| **P266** | **Text** | **~52% citado** | **? (esperado ~80-85%)** | **? esperado +30pp** |

**Hipótese auditável**: Text segue padrão Model/Color
(cobertura empírica > cobertura citada).
