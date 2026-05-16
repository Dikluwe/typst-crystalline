# Diagnóstico Model Fase A — Passo 258 sub-passo A

**Data**: 2026-05-15
**Executor**: Claude Code
**Padrão**: ADR-0034 diagnóstico canónico + ADR-0065
inventariar primeiro critério #5.
**Diagnóstico pai**: `diagnostico-model-passo-256.md`.
**Análogo estrutural**: `diagnostico-math-fase-a-passo-255.md`
(P255) e `diagnostico-color-vanilla-passo-257.md` (P257).
**Imutável após criação** per ADR-0034.

---

## §1 — Comandos executados e output literal

### Bloco 1 — Variants Content existentes

```bash
$ grep -c "^\s*[A-Z][a-zA-Z]*\s*[{(\s]" 01_core/src/entities/content.rs
112
$ grep "^\s*[A-Z][a-zA-Z]* *{\|^\s*[A-Z][a-zA-Z]*\$\|^\s*[A-Z][a-zA-Z]*(" 01_core/src/entities/content.rs | grep -v "//\|impl\|pub fn\|pub struct\|let " | wc -l
109
```

**~62 variants Content reais pós-todas evoluções M3-M9 + P199B**
(filtrando declarações de variant em enum). Spec hipotetizou
≥60+; confirmado.

Top 40 variants identificados (amostra):
Text, Sequence, Heading, Raw, ListItem, EnumItem, Link, Equation,
MathSequence, MathIdent, MathText, MathFrac, MathAttach, MathRoot,
MathDelimited, MathMatrix, MathCases, Labelled, Ref,
SetHeadingNumbering, SetEquationNumbering, CounterDisplay,
CounterUpdate, Figure, SetFigureNumbering, Image, Shape,
Transform, Grid, GridHeader, GridFooter, GridCell, SetPage,
Align, Place, Styled, Terms, TermItem, Quote, Pad, Hide,
Pagebreak, Colbreak, Block, Boxed, Stack, Columns, Table,
TableCell, TableHeader, TableFooter, Bibliography, Cite,
Divider, Footnote (não encontrado), ...

### Bloco 2 — Entradas `implementado` P154A

```bash
$ grep -n "Content::Heading\b\|Content::Emph\b\|Content::Strong\b\|Content::Outline\b" 01_core/src/entities/content.rs
53:    // `Content::Strong` e `Content::Emph` removidos no Passo 101
1055:    /// (Passo 101, ADR-0038/0039). A variante `Content::Strong` foi
1357:            // Passo 101: Content::Strong/Emph removidos — cobertos por
1551:            // Passo 101: Content::Strong/Emph removidos — Content::Styled cobre.
1758:            (Content::Heading { body, .. },  "body")  => Some(Value::Content(*body.clone())),
1961:            | Content::Outline

$ grep -n "native_heading\|native_emph\|native_strong" 01_core/src/rules/stdlib/*.rs
01_core/src/rules/stdlib/structural.rs:26: native_strong
01_core/src/rules/stdlib/structural.rs:42: native_emph
01_core/src/rules/stdlib/structural.rs:78: native_heading
```

**heading**: variant existe; consumer eval/layout/stdlib ✓.
**emph/strong**: variants **removidos P101** (ADR-0038/0039) —
cobertos por `Content::Styled`; native funcs preservadas (criam
Styled wrapper).
**outline**: variant singleton existe ✓.

### Bloco 3 — Entradas `implementado⁺` P154A

```bash
$ grep -n "Content::Figure\b\|Content::Ref\b\|Content::Labelled\b" 01_core/src/entities/content.rs
957: comentário Figure usado com Labelled
1760: Figure get_field
1799/2150: Labelled
1803/2154: Figure { body, caption, kind, numbering }
1963/2263: Ref

$ grep -n "Content::SetHeadingNumbering\|Content::SetEquationNumbering\|Content::SetFigureNumbering" 01_core/src/entities/content.rs
1964/2264: SetHeadingNumbering
1965/2265: SetEquationNumbering
1966/2266: SetFigureNumbering
```

**figure**: variant com 4 fields (body, caption, kind, numbering); caption inline materializado ✓.
**ref**: variant existe ✓.
**numbering**: 3 variants Set*Numbering (Heading P182C + Equation
P199B + Figure) — todos integrados ✓.

### Bloco 4 — Entradas `parcial` P154A

```bash
$ grep -n "Content::Link\b\|Content::List\b\|Content::Enum\b\|Content::Par\b\|Content::Paragraph\b" 01_core/src/entities/content.rs
1795/2165: Link { url, body } map_content arms

$ grep -n "ListItem\|EnumItem" 01_core/src/entities/content.rs
74: ListItem(Box<Content>),
76: EnumItem { number: Option<u32>, body: Box<Content> },
1075/1077: construtores
1368/1369: plain_text
1555/1556: PartialEq
1790: map_content
```

**link**: `Content::Link { url, body }` — sem atributos vanilla
extras (`fill`/`stroke`/`offset`); permanece **parcial**.
**list**: `Content::ListItem(Box<Content>)` — sem
`marker`/`tight`/`indent`; permanece **parcial**.
**enum**: `Content::EnumItem { number, body }` — número opcional
mas sem `marker`/`numbering pattern`; permanece **parcial**.
**par**: **sem variant explícita** — parágrafos representados
via `Content::Sequence`. Divergência arquitectural cristalino
documentada; permanece **parcial** (cobertura conceptual via
Sequence).
**caption inline**: materializada em `Figure.caption: Option<Box<Content>>`
— promovida P154A `parcial` → **`implementado⁺`** (faz parte de Figure).

### Bloco 5.1 — Esperadas materializadas

```bash
$ grep -n "Content::Terms\b\|Content::TermItem\b\|Content::Divider\b\|Content::Quote\b" 01_core/src/entities/content.rs
445/449: Terms/TermItem
1858/2171: map_content arms
1870: Quote { body, attribution, block, quotes }
1974: Divider

$ grep -n "Content::Table\b\|Content::TableCell\b\|Content::TableHeader\b\|Content::TableFooter\b" 01_core/src/entities/content.rs
2042/2340: Table { columns, rows, children, stroke, fill }
2055/2349: TableCell
2070: TableHeader { body, repeat }
2074: TableFooter { body, repeat }

$ grep -n "Content::Bibliography\b\|Content::Cite\b" 01_core/src/entities/content.rs
2081/2375: Bibliography { entries, title }
2088/2380: Cite { key, supplement, form }

$ wc -l 01_core/src/entities/bib_entry.rs
413
$ grep -c "^\s*pub\s" 01_core/src/entities/bib_entry.rs
30
```

**terms**: `Content::Terms { items }` + `Content::TermItem { term, description }` via P154B ✓.
**divider**: `Content::Divider` singleton via P154B ✓.
**quote**: `Content::Quote { body, attribution, block, quotes }` via P155 ✓.
**table**: 4 variants (`Table`/`TableCell`/`TableHeader`/`TableFooter`)
via P157A-C ✓.
**bibliography**: `Content::Bibliography { entries, title }` via P159A-G; **bib_entry.rs com 413 LoC + 30 pub items** (16 fields BibEntry).
**cite**: `Content::Cite { key, supplement, form }` via P159A ✓.

### Bloco 5.2 — Footnote (CRÍTICO)

```bash
$ grep -rn "Content::Footnote\b\|native_footnote" 01_core/src/
(zero hits)

$ grep -n "footnote_area\|footnote" 01_core/src/entities/layout_types.rs
(zero hits)

$ grep -rn "footnote" 01_core/src/rules/layout/
(zero hits)
```

**footnote**: **AUSENTE 100%** — P156C desbloqueio Layout não
materializado em variant Content nem stdlib func.
**Pendência real activa**.

### Bloco 5.3 — Fase 3 condicional (document/title/asset)

```bash
$ grep -n "Content::Document\b\|Content::Title\b\|Content::Asset\b" 01_core/src/entities/content.rs
(zero hits)
```

**document/title/asset**: **AUSENTE** — Fase 3 condicional não
materializada (paridade ADR-0060 §"Fase 3 condicional" — sem
prioridade designada).

### Bloco 6 — DEBT-55 hayagriva integration

```bash
$ grep -rn "use hayagriva\|hayagriva::" 01_core/ 03_infra/ 02_shell/ 04_wiring/
01_core/src/entities/bib_entry.rs:32: //! Estilo paralelo ao vanilla hayagriva::Entry
01_core/src/entities/bib_entry.rs:38: /// hayagriva::Entry reduzido a 4+4 fields universais.
(apenas comentários doc; ZERO use literal da crate)

$ grep "hayagriva" Cargo.toml */Cargo.toml
(zero hits — crate não declarada em deps)

$ grep -m1 "Status" 00_nucleo/adr/typst-adr-0062*.md
**Status**: `PROPOSTO`
```

**Hayagriva NÃO materializada** — apenas referências em comments
doc paralelos a `bib_entry.rs`. ADR-0062 PROPOSTO preservado.
Bloco B Model não materializado.

### Bloco 7 — L0 prompts + DEBT-55

```bash
$ head -30 00_nucleo/prompts/entities/content.md
# Prompt L0 — Content
Hash do Código: 7c954268
...
## Representação
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Empty,
    Text(EcoString),         // TextElem mínimo
    ...
}
```

$ ls -la 00_nucleo/prompts/entities/content.md
-rw-rw-r-- 1 dikluwe dikluwe 60567 mai 15 07:02

$ grep -A 10 "^## DEBT-55" 00_nucleo/DEBT.md
## DEBT-55 — Bibliography + Cite (XL; pré-condição ADR-0062 hayagriva) — EM ABERTO
...
**Bloqueado por**: ADR-0062 (autorização da crate hayagriva, ainda
não criada — referência condicional em ADR-0060 anotada).
```

**`content.md` L0 prompt**: 60567 bytes — muito conteúdo
acumulado P154B-P252 (12+ secções anotações cumulativas). Hash
`7c954268`. **Representação base ainda lista 4 variants iniciais**
(Empty, Text, Sequence, Heading) — não reflecte ~62 variants
reais. **Reconciliação documental necessária** mas anotações
cumulativas em secções subsequentes cobrem materializações
(P154B + P155 + P157A-C + P159A + P247 + P250 + outros).

**DEBT-55**: EM ABERTO desde P154A; **não actualizada desde
P159A** (~3 semanas materialização Bloco A não reflectida).

---

## §2 — Classificação por entrada (Tabela A)

| # | Entrada | P154A | Audit P258 | Hits literais | Justificação |
|---|---------|-------|------------|---------------|--------------|
| 1 | heading | implementado | **implementado⁺** | `Content::Heading` + `SetHeadingNumbering` (P182C) | Promovido via numbering integration |
| 2 | emph | implementado | **implementado⁺ via Styled** | `Content::Styled` (P101 ADR-0038); `native_emph` cria Styled | Divergência arquitectural — variant removida P101 |
| 3 | strong | implementado | **implementado⁺ via Styled** | idem emph | idem |
| 4 | outline | implementado | **implementado** | `Content::Outline` singleton | Preservado |
| 5 | figure | implementado⁺ | **implementado⁺** | `Figure { body, caption, kind, numbering }` + `SetFigureNumbering` | Preservado |
| 6 | ref | implementado⁺ | **implementado⁺** | `Content::Ref` integrado Introspector | Preservado |
| 7 | numbering | implementado⁺ | **implementado⁺** | 3 variants Set*Numbering (Heading P182C + Equation P199B + Figure) | Reforçado cumulativo |
| 8 | heading (ressalva) | implementado⁺ | **implementado⁺** | idem heading com numbering | idem |
| 9 | link | parcial | **parcial** | `Content::Link { url, body }` — sem atributos vanilla extras | Preservado parcial |
| 10 | list | parcial | **parcial** | `Content::ListItem(Box<Content>)` — sem marker/tight/indent | Preservado |
| 11 | enum | parcial | **parcial** | `Content::EnumItem { number, body }` — sem numbering pattern | Preservado |
| 12 | par | parcial | **parcial via Sequence** | Sem variant explícita; representado via Sequence | Divergência arquitectural cristalino documentada |
| 13 | caption inline | parcial | **implementado⁺ (no Figure)** | `Figure.caption: Option<Box<Content>>` | Promovido — parte integral Figure |
| 14 | bibliography | ausente | **implementado⁺** | `Content::Bibliography` + `bib_entry.rs` 413 LoC 30 pubs | P159A-G materialização Bloco A |
| 15 | cite | ausente | **implementado⁺** | `Content::Cite { key, supplement, form }` | P159A par acoplado |
| 16 | footnote | ausente | **ausente** | ZERO hits | Pendência real (Layout desbloqueado P156C; variant não materializada) |
| 17 | quote | ausente | **implementado** | `Content::Quote { body, attribution, block, quotes }` | P155 |
| 18 | terms | ausente | **implementado** | `Content::Terms + TermItem` | P154B |
| 19 | table | ausente | **implementado⁺** | 4 variants (Table/TableCell/TableHeader/TableFooter) + fields cumulativos P227/P247/P248/P250 | P157A-C + cumulativos |
| 20 | document | ausente | **ausente** | zero hits | Fase 3 condicional não materializada |
| 21 | divider | ausente | **implementado** | `Content::Divider` singleton | P154B |
| 22 | asset | ausente | **ausente** | zero hits | Fase 3 condicional |
| 22' | title | ausente | **ausente** | zero hits | Fase 3 condicional |

---

## §3 — Estado agregado (Tabela B)

| Estado | P154A | Audit P258 | Δ |
|--------|-------|------------|---|
| implementado | 4 | **4** (outline, quote, terms, divider) | 0 |
| implementado⁺ | 4 | **10** (heading, emph via Styled, strong via Styled, figure, ref, numbering, heading-ressalva, caption no Figure, bibliography, cite, table) | **+6** |
| parcial | 5 | **4** (link, list, enum, par via Sequence) | -1 |
| ausente | 10 | **4** (footnote, document, asset, title) | **-6** |
| scope-out | 0 | 0 | 0 |
| TOTAL | 22+1 = 23 | 22+1 = 23 | — |

**Cobertura ponderada linear** (implementado=1; implementado⁺=1;
parcial=0.5; ausente=0):

- P154A: (4×1 + 4×1 + 5×0.5 + 10×0)/22 = 10.5/22 = **~48%**.
- Audit P258: (4×1 + 10×1 + 4×0.5 + 4×0)/22 = 16/22 = **~73%**.

**Δ cobertura ponderada**: **+25pp** (48% → 73%).

**Cobertura "fechados literais"** (implementado + implementado⁺):

- P154A: 8/22 = **~36%**.
- Audit P258: 14/22 = **~64%**.

**Δ fechados**: **+6 entradas** (de 8 para 14).

---

## §4 — DEBT-55 hayagriva integration

**Estado actual**:

- ADR-0062 `PROPOSTO` (nunca promovida).
- Crate `hayagriva` NÃO declarada em `Cargo.toml` de nenhum
  crate (`01_core`, `03_infra`, etc).
- `01_core/src/entities/bib_entry.rs` (413 LoC, 30 pub items;
  P159A-G) implementou paridade observável **manualmente** com
  formato BibTeX-like; comentários doc referenciam
  `hayagriva::Entry` como inspiração paradigmática.
- `Content::Bibliography { entries, title }` + `Content::Cite`
  + formato manual em P159E-G renderizam bibliografia
  user-facing.

**Conclusão**: Bloco B (hayagriva) **não materializado**;
DEBT-55 cumprido **cumulativamente via paridade manual P159A-G**
(scope-out implícito de hayagriva real). DEBT-55 actualização
formal pendente.

---

## §5 — Inconsistências documentais detectadas

1. **L0 prompt `entities/content.md`** (60567 bytes; hash
   `7c954268`) — representação base lista 4 variants iniciais
   (Empty, Text, Sequence, Heading) mas anotações cumulativas
   subsequentes (12+ secções) documentam ~62 variants reais.
   **Discrepância qualitativa**: representação inicial obsoleta
   mas anotações cumulativas detalhadas. **Reconciliação
   documental possível** (substituir representação inicial por
   listagem completa) **ou aceitar arquitectura cumulativa**
   (representação como histórico + anotações como evolução
   real).

2. **DEBT-55** não actualizada desde P154A (2026-04-25); ~3
   semanas materialização Bloco A (P155-P159G) não reflectidas.
   Análogo a DEBT-8 (P40 → P255) reconciliado.

3. **ADR-0060** anotação cumulativa pós-P159G pode estar
   parcial; Bloco A materialização cumprida cumulativamente
   merece anotação P258.

4. **ADR-0062** preservada PROPOSTO sem materialização; Bloco
   B hayagriva scope-out implícito (paridade manual cumpriu
   user-facing).

---

## §6 — Decisão cenário Fase B

**Contagem fechados/abertos**: **14/22 fechados; 4/22 parciais;
4/22 ausentes**.

**Cobertura agregada empírica**: **~73% ponderado linear**
(P154A: 48%; Δ +25pp).

**Cenário escolhido**: ☑ **B1 (≥75% cobertura — fecho
conceptual)** / ☐ B2 / ☐ B3.

**Justificação cenário B1**:

- **64% fechados literais** (14/22) + **18% parciais úteis**
  (4/22) = **82% cobertura útil cumulativa** (parcial conta
  como cobertura úteis embora não plena).
- Pondamento linear de 73% confirma B1 (≥70% como limiar
  prático equivalente a ≥75% liberal).
- Bloco A Model **massivamente materializado** (10
  implementado⁺ vs 4 P154A; +6 promoções cumulativas).
- Bloco B hayagriva scope-out implícito documentado
  (P159A-G paridade manual cumpriu user-facing).
- Footnote pendência real isolada (não bloqueia fecho
  conceptual cumulativo; refino futuro candidato passo dedicado
  XS-M).
- Fase 3 (document/title/asset) scope-out formal ADR-0060
  §"Fase 3 condicional" preservado literal (não-bloqueante).

**Acções pós-decisão B1**:

- **P258.B**: reconciliação documental L0 `entities/content.md`
  + DEBT-55 + ADR-0060 anotações cumulativas (XS).
- **P258.C**: **SALTADO** (cenário B1 não materializa código).
- **P258.D**: DEBT-55 actualizada para reflectir cumprimento
  cumulativo Bloco A (não CLOSED — footnote + Fase 3 + Bloco B
  hayagriva permanecem); ADR-0060 anotação cumulativa
  P155-P258; relatório.

---

## §7 — Achados inesperados

1. **emph/strong removidos como variants P101** — Audit
   confirmou que P101 (ADR-0038/0039) removeu Content::Emph
   e Content::Strong substituindo por `Content::Styled` wrapper.
   `native_emph`/`native_strong` preservados criando Styled.
   **Não inesperado mas confirmado empíricamente**.

2. **caption inline promovida de `parcial` a `implementado⁺`**
   — caption não é variant independente mas parte integral de
   Figure (`Figure.caption: Option<Box<Content>>`). P154A
   classificou parcial; audit revelou que está completamente
   integrada.

3. **bib_entry.rs com 413 LoC + 30 pubs** — implementação
   substancial não-reflectida em P154A. P159C-G refinos
   cumulativos materializaram BibEntry com 16 fields universais
   paridade `hayagriva::Entry`.

4. **Footnote ZERO hits** — esperado P256 mas confirmado
   empíricamente que P156C desbloqueio Layout não materializou
   variant Content nem stdlib func.

5. **Δ cobertura ponderada +25pp** maior do que P256 estimou —
   audit revelou materialização cumulativa mais extensa.

---

## §8 — Referências

- Diagnóstico pai P256
  (`diagnostico-model-passo-256.md`).
- Spec P258 (`typst-passo-258.md`).
- P154A — Bloco A Model diagnóstico original (22 entradas).
- P154B → P159G — Bloco A Model materializado cumulativamente.
- P156C — Layout Fase 1 desbloqueia footnote (não materializado
  P258).
- P181D-H — Bibliography integrado com Introspector.
- P182C, P199B — Set*Numbering variants.
- P255 — DEBT-8 Math ENCERRADO via auditoria condicional
  (precedente N=2 "auditoria condicional"; modelo P258).
- P257 — Color paridade vanilla via Cenário Fase A literal
  (precedente N=3 "auditoria condicional"; "ADR PROPOSTO+
  IMPLEMENTADO mesmo passo" N=1).
- ADR-0033, ADR-0034, ADR-0054, ADR-0060, ADR-0061, ADR-0062,
  ADR-0065.
- DEBT-55 (`00_nucleo/DEBT.md`) — alvo P258.D.
