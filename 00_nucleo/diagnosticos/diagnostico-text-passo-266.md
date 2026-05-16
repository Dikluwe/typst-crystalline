# Diagnóstico Text — Passo 266 (preparatório)

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico
(**não materializa código**)
**Estrutura**: duas fases registadas (precedente N=5 de
"auditoria condicional" P192A/P255/P257/P258/P259; **primeiro
consumo directo ADR-0084 + ADR-0085** EM VIGOR pós-P260).

**Análogo estrutural canónico directo**:
- P254A (Introspection actualizado) + P254B (Math) + P255
  (Math executado) — pattern auditoria.
- **P259 (Visualize Fase A)** — **template literal directo**
  deste passo. Visualize foi último audit pré-P260
  formalização; Text é primeiro audit pós-P260.

**Motivação**:
- Resumo cumulativo recente cita "Text ~52%" há vários
  passos sem audit empírico.
- DEBT-1 + DEBT-52 ENCERRADOS P142 — **mas DEBT-53 (shaping
  rustybuzz) continua aberto candidato XL**.
- Multiplos refinos pós-P142: P144 lang hyphenation; P146
  multi-font; P155 smart-quotes; P155 quote variant; ADR-0055/
  0056/0057.
- ADR-0084 + ADR-0085 formalizados P260; **Text seria
  primeiro audit empírico pós-formalização real**
  (P262/P264 produziram diagnósticos vanilla, não audits Fase
  A propriamente ditos).

---

## §1 — ADRs e DEBTs relevantes

### ADRs activos para Text

| ADR | Status | Relevância |
|-----|--------|------------|
| ADR-0019 | IMPLEMENTADO | `ttf-parser` + `rustybuzz` (autorização L3) |
| ADR-0027 | IMPLEMENTADO | CIDFont/Identity-H (Unicode completo) |
| ADR-0033 | EM VIGOR | Paridade observable vanilla |
| ADR-0034 | EM VIGOR | Diagnóstico canónico (estendido por ADR-0085) |
| ADR-0038 | EM VIGOR | Sistema styles L1 (`Content::Styled` + StyleChain) |
| ADR-0039 | EM VIGOR | TextStyle SR cache (preservado P261/P262/P264) |
| ADR-0052 | EM VIGOR | Lang tipo semântico 2-3 letras ASCII |
| ADR-0053 | EM VIGOR | `font` dict deferido (FontList agregador) |
| ADR-0054 | EM VIGOR | DEBT-1 fecho — perfil observacional graded |
| ADR-0055 | IMPLEMENTADO | Font consumer CIDFont (5 decisões; multi-font P146) |
| ADR-0057 | IMPLEMENTADO | Lang hyphenation via hypher |
| **ADR-0084** | EM VIGOR (P260) | **Auditoria condicional — este passo primeiro consumo directo** |
| **ADR-0085** | EM VIGOR (P260) | **Diagnóstico imutável — este passo primeiro consumo directo audit Fase A pós-P260** |
| ADR-0065 | EM VIGOR | Inventariar primeiro |
| ADR-0080 | EM VIGOR | L0 minimal refactors aditivos |

### DEBTs Text

| DEBT | Status | Relevância |
|------|--------|------------|
| DEBT-1 | ENCERRADO P142 | StyleChain + StyleDelta cumprido |
| DEBT-52 | ENCERRADO P142 | Rastreador shaping fechado |
| **DEBT-53** | **EM ABERTO** | **Shaping rustybuzz real (XL candidato)** — único Text DEBT activo |
| ADR-0055bis | candidato informal | Variant-aware font selection — não DEBT |
| ADR-0056 | candidato informal | Font subsetting — não DEBT |

### Materializações cumulativas pós-DEBT-1 fecho (P142+)

| Passo | Feature | Tipo |
|-------|---------|------|
| P144 | Lang hyphenation (`hypher` crate) | ADR-0057 + módulo `rules/layout/hyphenation.rs` |
| P146 | Multi-font per document | Extensão ADR-0055 decisão 5 |
| P155 | `Content::Quote` + smart-quotes lang-aware | ADR-0060 Fase 1 + módulo `rules/lang/quotes.rs` |
| P155 | `Content::Divider` + `Content::Terms` | ADR-0060 Fase 1 |

---

## §2 — Inventário declarado pré-P266 (não auditado formalmente)

Text é módulo grande sem inventário Tabela A explícito (ao
contrário de Math P192A-style, Model P154A, Visualize
parcial P259). Lista parcial baseada em referências
cumulativas cruzadas + ADRs conhecidos.

### Subsistema A — StyleChain + StyleDelta

| Campo StyleDelta | Estado | Origem |
|------------------|--------|--------|
| `bold` | implementado | P30 (DEBT-1 base) |
| `italic` | implementado | P30 |
| `size` | implementado | P30 |
| `fill` | implementado | P99 (ADR-0038) |
| `heading_level` | implementado | P99 |
| `weight` (numérico) | implementado | P126 |
| `weight` (simbólico) | implementado | P129 |
| `tracking` | implementado | P137 |
| `leading` | implementado | P138 |
| `font` (string) | implementado | P140B |
| `font` (array fallback) | implementado | P141 |
| `lang` | implementado⁺ parcial | P144 hyphenation + P155 smart-quotes; shaping ausente |

**Cobertura StyleChain**: 11/12 implementados; 1 parcial.
**Esperado: ≥85% estrutural**.

### Subsistema B — Markup text features

| Feature | Estado provável | Origem |
|---------|-----------------|--------|
| `Content::Text` | implementado | base |
| `Content::Strong` | implementado | base + ADR-0038 |
| `Content::Emph` | implementado | base + ADR-0038 |
| `Content::Heading` | implementado⁺ | P155 etc. |
| `Content::Quote` | implementado | P155 (4 atributos vanilla) |
| `Content::Raw` (code blocks ``…``) | **a confirmar Fase A** | provavelmente parcial — sem syntax highlighting |
| `Content::Link` | parcial per P154A | Model |
| `Content::Linebreak` | implementado provável | base |
| `Content::Parbreak` | implementado provável | base |
| Smart-quotes `"..."` lang-aware | implementado | P155 |
| Escape characters `\#` etc. | **a confirmar Fase A** | provavelmente implementado |
| Shorthands `~` NBSP, `--` em-dash | **a confirmar Fase A** | provavelmente parcial |

### Subsistema C — Font rendering

| Aspecto | Estado | Origem |
|---------|--------|--------|
| Helvetica fallback (WinAnsiEncoding) | implementado | P21 |
| CIDFont/Identity-H (Unicode completo) | implementado | P140B (ADR-0027) |
| Multi-font per document | implementado | P146 |
| Font fallback chain | implementado | P141 |
| Variant-aware (Bold/Italic font-file dedicado) | **ausente** | ADR-0055bis candidato |
| Font subsetting PDF | **ausente** | ADR-0056 candidato |
| Font caching `Arc::ptr_eq` | implementado | P140B/141 |

### Subsistema D — Lang features

| Feature | Estado | Origem |
|---------|--------|--------|
| Lang tipo semântico (2-3 letras ASCII) | implementado | ADR-0052 |
| Hyphenation (hypher; ~30+ idiomas) | implementado | P144 (ADR-0057) |
| Smart-quotes (6 idiomas + default) | implementado | P155 |
| Shaping features (ligatures/kern/bidi via rustybuzz) | **ausente** | DEBT-53 XL |
| Bidirectional text (RTL Arabic/Hebrew) | **ausente** | DEBT-53 |

### Subsistema E — Refino features

| Aspecto | Estado provável | Origem |
|---------|-----------------|--------|
| `faux-bold` stroke (weight 700) | implementado | P139 |
| Tracking PDF `Tc` operator | implementado | P137 |
| Leading line-height | implementado | P138 |
| Hyphenation greedy break | implementado | P144 |
| `text.font` direct | implementado | P140B |
| `text.fill` (não color directo) | preservado | ADR-0039 |

### Cobertura agregada pré-P266 (estimativa)

| Subsistema | Implementado | Pendente |
|------------|-------------|----------|
| A StyleChain + StyleDelta | 11/12 = 92% | 1 parcial (lang shaping) |
| B Markup text features | ~70-80% | Raw shaping, shorthands a confirmar |
| C Font rendering | 6/8 = 75% | Variant-aware + subsetting |
| D Lang features | 3/5 = 60% | Shaping + bidi (DEBT-53) |
| E Refino features | ~100% | — |

**Cobertura agregada estimativa**: **~80-85%** (sobe vs "52%
citado" há vários passos; o "52%" provavelmente era apenas
um subconjunto de features e não cobertura agregada Text).

**Hipótese auditável**: Text está **substancialmente mais
completo** que indicado pelo "52% citado". Audit empírico vai
confirmar ou refutar.

---

## §3 — Fase A: Inventário empírico (a executar)

Análogo a P255/P257/P258/P259 §1. Comandos `grep`/`view` que
produzem evidência factual antes de decisão.

**Primeiro audit Fase A pós-P260 ADR-0084 + ADR-0085 EM
VIGOR**.

### Bloco 1 — Vanilla Text módulo (estrutura literal)

```bash
# Listagem vanilla
ls lab/typst-original/crates/typst-library/src/text/
view lab/typst-original/crates/typst-library/src/text/mod.rs | head -60

# Variants TextElem/RawElem/StrongElem/EmphElem vanilla
grep -rn "^\s*#\[elem\]\|pub struct.*Elem\b" \
  lab/typst-original/crates/typst-library/src/text/ | head -30

# TextStyle/StyleChain vanilla
grep -rn "^\s*pub struct TextElem\|pub struct StyleChain" \
  lab/typst-original/crates/typst-library/src/ | head -10
```

**Output esperado**: lista variants vanilla Text relevantes
(TextElem, StrongElem, EmphElem, RawElem, LinkElem,
LinebreakElem, ParbreakElem, SmartQuoteElem, etc.).

### Bloco 2 — Variants Content text-related (cristalino)

```bash
# Variants Content text-related
grep -n "^\s*Text\|^\s*Strong\|^\s*Emph\|^\s*Heading\|^\s*Quote\|^\s*Raw\|^\s*Link\|^\s*Linebreak\|^\s*Parbreak" \
  01_core/src/entities/content.rs

# Total Content variants (post-P258 ~62)
grep -c "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs
```

**Critério**: confirmar variants Text presentes na cristalino.

### Bloco 3 — StyleChain + StyleDelta campos

```bash
# StyleDelta campos actuais (esperado: 12 campos)
grep -A 30 "^\s*pub struct StyleDelta" 01_core/src/entities/style_chain.rs

# Resolvers StyleChain (esperado: 12 funcs)
grep -n "pub fn .*\(&self\) ->" 01_core/src/entities/style_chain.rs | head -20
```

**Critério**: confirmar 11 implementados + 1 parcial (lang).

### Bloco 4 — Font rendering helpers L3

```bash
# CIDFont/Identity-H helpers
grep -n "fn export_pdf_with_font\|fn export_pdf_multifont\|fn build_cidfont" \
  03_infra/src/export.rs

# Resolução font fallback chain
grep -n "fn resolve_font\|fn resolve_fonts\|FontBook::select" \
  01_core/src/ 03_infra/src/

# Variant-aware? (esperado ausente)
grep -rn "FontVariant\|FontWeight::from_name\|variant_aware" \
  01_core/src/ 03_infra/src/ | head -10
```

**Critério**:
- 3 export_pdf paths confirmados.
- `FontVariant::default()` usado (variant-aware ausente).

### Bloco 5 — Lang features

```bash
# Hyphenation
ls 01_core/src/rules/layout/hyphenation.rs
grep -n "pub fn hyphenate\|hypher::" 01_core/src/rules/layout/hyphenation.rs

# Smart-quotes
ls 01_core/src/rules/lang/quotes.rs
grep -n "pub fn localize_quotes\|DEFAULT_QUOTES" 01_core/src/rules/lang/quotes.rs

# Shaping ausente?
grep -rn "rustybuzz::\|shape(" 01_core/src/ 03_infra/src/
```

**Critério**:
- Hyphenation + smart-quotes confirmados.
- Shaping zero hits (DEBT-53 confirmado em aberto).

### Bloco 6 — Refinos text features (markup secundários)

```bash
# Escape characters
grep -rn "SyntaxKind::Escape\|fn escape\|Content::Escape" \
  01_core/src/

# Shorthands (~, --, ...)
grep -rn "SyntaxKind::Shorthand\|Shorthand::LIST" 01_core/src/

# Raw blocks (code highlighting?)
grep -n "Content::Raw\|fn raw\|highlight" \
  01_core/src/entities/content.rs 01_core/src/rules/

# Linebreak / Parbreak
grep -n "Content::Linebreak\|Content::Parbreak" \
  01_core/src/entities/content.rs
```

**Critério**: confirmar quais features markup secundários
materializados vs ausentes.

### Bloco 7 — Inconsistências documentais

```bash
# L0 prompts text-related
ls 00_nucleo/prompts/entities/style_chain.md
ls 00_nucleo/prompts/rules/layout.md
ls 00_nucleo/prompts/rules/lang.md

# Hash drift verificar
grep "@prompt-hash" 01_core/src/entities/style_chain.rs
grep "@prompt-hash" 01_core/src/rules/layout/hyphenation.rs
```

**Esperado** (per precedente P255/P257/P258/P259):
- Inconsistências documentais prováveis em prompts L0 não
  actualizados desde refinos cumulativos pós-P155.

---

## §4 — Cenários Fase B

### Cenário B1: Cobertura ≥75% confirmada (provável)

Confirma hipótese §2. Text está substancialmente completo na
maioria dos subsistemas; shaping rustybuzz (DEBT-53) é o gap
maior conhecido e fora de scope ADR-0054.

Acção: relatório de fecho conceptual Text análogo a P258
(Model B1 ~73%). DEBTs activos preservados (DEBT-53 candidato
XL futuro).

### Cenário B2: Cobertura 55-70% (possível)

Implicaria gaps inesperados em Markup secundários (Raw,
shorthands) ou refinos não materializados que sub-estimei.

Acção: documentar estado factual; identificar 1-3 sub-passos
prioritários:
- **Opção 1**: ADR-0055bis variant-aware font selection (M).
- **Opção 2**: ADR-0056 font subsetting (M-L).
- **Opção 3**: Refinos Raw (syntax highlighting básico? — S+).
- **Opção 4**: Shaping pre-rustybuzz (basic kern/lig PDF
  hints — M-L).
- **Opção 5**: Refinos lang adicionais (mais idiomas
  smart-quotes, hyphenation extra — S+).

### Cenário B3: Cobertura ≤50% (improvável)

Implicaria que materializações cumulativas P140B+P141+P144+
P146+P155 estavam superestimadas. Pouco provável dado
DEBT-1+52 ENCERRADOS formalmente.

Acção: re-classificação conservadora; sub-passos de elevação
prioritários.

---

## §5 — Recomendação concreta

### Recomendação primária

**P266-aud — Fase A audit empírico** (XS-S; ~30-45 min de
leitura de código). Output: diagnóstico imutável análogo a
`diagnostico-math-fase-a-passo-255.md` /
`diagnostico-color-vanilla-passo-257.md` /
`diagnostico-model-fase-a-passo-258.md` /
`diagnostico-visualize-fase-a-passo-259.md` com classificação
literal de cada subsistema.

**Será primeiro audit Fase A pós-formalização P260
ADR-0084 + ADR-0085 EM VIGOR**. Consome ADRs directamente
(estrutura + criterios cobertura ambígua).

### Recomendação secundária (pós-audit)

Depende do cenário Fase B confirmado:

- **B1 provável**: fecho conceptual; passar a outro módulo
  (Footnote? Tiling? P-Gradient-Conic?).
- **B2 possível**: documentar Opções 1-5; humano escolhe
  qual materializar.
- **B3 improvável**: re-classificação conservadora primeiro.

### Não recomendado

- Atacar **DEBT-53 (rustybuzz shaping)** sem ADR explícita
  preliminar — magnitude XL declarada; **fora de scope
  ADR-0054**.
- Atacar **font subsetting** (ADR-0056 candidato) sem audit
  primeiro confirmar prioridade.

---

## §6 — Padrões metodológicos aplicados

### ADR-0084 + ADR-0085 primeiro consumo directo

**Marco metodológico**: P260 formalizou metodologia "auditoria
condicional" + "diagnóstico imutável" como ADRs EM VIGOR.
P266 é **primeiro audit Fase A executado sob essas ADRs
formais** (P262/P264 produziram diagnósticos vanilla per
ADR-0029, não audits Fase A propriamente ditos).

Validação retrospectiva ADR-0084 + 0085 cumprida.

### Subpadrão "auditoria condicional" N=5 → N=6

Cumulativo:
- N=1 P192A (M7 fixpoint).
- N=2 P255 (DEBT-8 Math).
- N=3 P257 (Color Fase A vanilla).
- N=4 P258 (Model Fase A).
- N=5 P259 (Visualize Fase A).
- **N=6 P266** (Text Fase A; este passo recomenda).

**Patamar N=6 excede limiar formalização clara**. Pattern
sólido confirmado retroactivamente.

### Subpadrão "diagnóstico imutável precedente à acção" N=6 → N=7

Cumulativo:
- N=1-4 P255/P257/P258/P259 (audit Fase A).
- N=5 P262 (diagnóstico Gradient Linear vanilla).
- N=6 P264 (diagnóstico Gradient Radial vanilla).
- **N=7 P266** (se Fase A materializar diagnóstico Text
  Fase A imutável).

**Patamar N=7 reforça pattern sólido**.

### Política "sem novas reservas"

Preservada. Recomendações §5 são para validação humana, não
compromissos.

---

## §7 — Limitações deste diagnóstico

1. **Cobertura agregada Text "~52% citada" sem origem clara
   no contexto** — pode ser feature-específico vs agregado;
   audit empírico produzirá número real.

2. **Tabela A entradas Text-específicas não existe**
   formalmente — P266.A vai precisar de derivar listagem
   vanilla a partir de leitura literal de
   `lab/typst-original/.../text/`.

3. **DEBT-53 fora de scope ADR-0054** — qualquer cenário B
   preserva DEBT-53 como candidato XL futuro; não
   bloqueante.

4. **Refinos markup secundários (Raw, shorthands, escape)
   incertos** — não há referência clara no contexto a
   materialização completa; pode haver gaps.

5. **Cross-references Text ↔ Math** não auditadas — equation
   inline em texto integra com Text shaping? — provavelmente
   fora de scope.

---

## §8 — Referências

- ADR-0019, ADR-0027, ADR-0033, ADR-0034, ADR-0038, ADR-0039,
  ADR-0052, ADR-0053, ADR-0054, ADR-0055, ADR-0057, ADR-0065,
  ADR-0080, ADR-0084, ADR-0085.
- DEBT-1 (fechado P142), DEBT-52 (fechado P142), DEBT-53 (em
  aberto XL).
- ADR-0055bis (candidato informal — variant-aware).
- ADR-0056 (candidato informal — font subsetting).
- P21 — Helvetica fallback.
- P30, P99 — DEBT-1 base.
- P100 — Layouter via StyleChain.
- P126, P127, P128, P129, P137, P138, P139 — DEBT-1 fases A/B.
- P140B, P141 — DEBT-1 fase C (font embedding CIDFont).
- P142 — DEBT-1 + DEBT-52 ENCERRADOS.
- P144 — hyphenation (ADR-0057).
- P146 — multi-font per document.
- P155 — Content::Quote + smart-quotes lang-aware.
- P192A — N=1 "auditoria condicional".
- P255 — DEBT-8 Math ENCERRADO (subpadrão N=2).
- P257 — Color paridade vanilla 8/8 (subpadrão N=3).
- P258 — Model fecho conceptual (subpadrão N=4).
- P259 — Visualize Fase A (subpadrão N=5).
- P260 — ADRs meta (formaliza ADR-0084/0085 consumidos
  directamente por este passo).
- P262 — Gradient Linear (diagnóstico vanilla).
- P264 — Gradient Radial (diagnóstico vanilla).
- ADR-0029 §enumeração — Text não é vanilla type per si;
  TextElem é elemento markup.
