# Relatório correcção pré-commit P268.1 — Correcção factual ADR-0090 + anotações dependentes

**Data**: 2026-05-15.
**Magnitude**: XS (cap 0 LOC L1/L3/stdlib; só edições documentais).
**Cluster**: Visualize / Gradient / PDF export (documental).
**Tipo**: correcção pré-commit. **Não é sub-passo materializado** — edição directa do output P268.1 antes de commit final. Sem numeração .N ordinal.
**Spec**: `00_nucleo/materialization/typst-passo-268.1-correcao.md`.

---

## §1 — Sumário executivo

Correcção pré-commit a ADR-0090 + 3 anotações dependentes (ADR-0089,
ADR-0054, L0 `entities/gradient.md`) + entrada README ADRs passo
268.1, retractando **3 afirmações factuais erradas** identificadas
via pesquisa externa (Kimi corroborada por web_search literal
blog Typst 2023 + W3C Workshop 2021 + Cairo/Igalia + ISO 19005-1).

### Afirmações erradas retractadas

1. **"Krilla (typst usa) PostScript Function-Based Shading
   `/ShadingType 1` (vendor-specific)"** → **ERRADA / não verificável**.
   Krilla actual produce strategy é opaco; afirmação era inferência
   não verificada. Typst original pré-krilla (blog 2023) usou
   `/ShadingType 6` Coons Patches.

2. **"PDF/A-1 proíbe functions (PostScript não permitido)"** →
   **IMPRECISA**. ISO 19005-1:2005 §6.2.7 proíbe PostScript XObjects
   (streams PostScript embutidas como XObjects independentes), **não**
   Type 4 calculator functions usadas em shading dictionaries. Type 4
   functions são subset restrito (sem loops/variáveis/subrotinas) e
   formalmente permitidas. Argumento prático "reader-support
   inconsistente" prevalece.

3. **"Type 1 Function-Based usado por krilla/Chrome é vendor-specific"**
   → **PARCIALMENTE ERRADA**. Chrome aspect confirmado (pdf.js issue
   #19233). Krilla aspect não verificável. Typst original aspect: usou
   Coons Type 6 explicitamente per blog 2023.

### Pré-condição verificada

Status commit confirmado via `git log --oneline`:

- `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`:
  **untracked** (zero commits) — pré-condição §política 2 satisfeita.
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`: última commit
  `792cfbaa0 Passo 267`; anotações P268/P268.1/P268.2 estão **todas em
  working tree não commitadas** — pré-condição §política 1 satisfeita.
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md`: última commit
  `a737f7200 Passo 266`; anotações P268.1+P268.2 **em working tree**.
- `00_nucleo/prompts/entities/gradient.md`: última commit
  `792cfbaa0 Passo 267`; anotações P268+P268.1+P268.2 **em working
  tree**.

**Conclusão**: edição literal directa viável; **§política condições
1 e 2 NÃO accionadas**.

### Decisão de fundo ADR-0090 preservada

**Cristalino Type 4 Gouraud é mantido como decisão arquitectural
vigente**. Status `EM VIGOR` preservado literal. A correcção só
ajusta as **justificativas** (não a decisão); cristalino continua
alinhado com industry mesh-based standard, mas a divergência é
agora caracterizada como **intra-família mesh** (Type 4 vs Type 6
Typst original/Cairo) em vez de "outlier industry" (que era
afirmação infundada).

### Marco arquitectural

**Primeira correcção factual de ADR `EM VIGOR` pré-commit** baseada
em verificação empírica externa. Estabelece **prática** (não pattern
formal) de que ADRs ainda não commitadas podem ser editadas literal
quando achados factuais novos invalidam justificativas, preservando
decisão de fundo se intacta. Subpadrão "Correcção ADR pré-commit"
**deliberadamente NÃO formalizado** — anti-pattern; normalizar
incentivaria editar ADRs livremente após criação.

---

## §2 — Diff afirmações ADR-0090 antes/depois

### §2.A — Título

| Antes | Depois |
|---|---|
| `# ⚖️ ADR-0090: Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) vs Type 1 PostScript (vanilla)` | `# ⚖️ ADR-0090: Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) divergência industry mesh-based variants` |

Motivo: título antigo assumia incorrectamente que vanilla usa Type 1.

### §2.B — Contexto (parágrafo P268.1.PRE)

**Antes**: "P268.1.PRE revelou que vanilla typst via
`krilla::SweepGradient` usa `/ShadingType 1` Function-Based Shading
com PostScript function `atan2`."

**Depois**: "P268.1.PRE revelou que vanilla typst via
`krilla::SweepGradient` inicializa uma abstracção de gradient cónico.
A estratégia PDF interna do krilla actual não está publicamente
documentada e não foi verificada literal. Verificação histórica via
blog Typst 2023 revela que Typst original (pré-krilla, antes da
transição Part 7 #5420) usou `/ShadingType 6` Coons Patches com
tantos patches quantos stops do gradiente."

### §2.C — Tabela "Pesquisa empírica industry"

**Linha krilla antes**: "PostScript Function-Based Shading
`/ShadingType 1` (vendor-specific)"
**Depois**: "Krilla (typst actual via Part 7 #5420) | `SweepGradient`
abstracção; estratégia PDF interna não documentada publicamente |
desconhecido"

**Linha nova adicionada**: "Typst original (pré-krilla, blog 2023) |
Coons Patches (1 patch por stop após rejeitar PS functions) |
`/ShadingType 6`"

**Linha pdf.js depois**: refinada para distinguir "Type 1 conic
embutido via Chrome puppeteer-style" de "krilla output actual"
(este último não estabelecido).

### §2.D — Conclusões factuais

- **Ponto 2**: "Krilla/Chrome vendor-specific" → "krilla afirmação
  retraída neste passo de correcção; Chrome via Puppeteer aspect
  preservado".
- **Ponto 3**: adicionada bullet "**Typst original (Type 6 Coons)**
  — blog 2023 documenta o processo de decisão literal" + W3C
  Workshop bfo citation.
- **Ponto 5**: "Krilla/vanilla typst é outlier" → "Krilla actual
  estratégia interna desconhecida publicamente; pode ou não estar
  alinhada. Cristalino divergência vs Typst original é
  **intra-família mesh** (Type 4 vs Type 6)".

### §2.E — Decisão justificativa 3 (PDF/A)

**Antes**: "Type 1 Function-Based proibido em PDF/A-1 (PostScript não
permitido); Type 4 permitido."

**Depois**: "ISO 19005-1:2005 §6.2.7 proíbe PostScript XObjects
(streams PostScript embutidas como XObjects independentes). Type 4
PostScript calculator functions usadas em shading dictionaries são
tecnicamente subset restrito (sem loops/variáveis/subrotinas) e
formalmente permitidas, mas suporte reader inconsistente é o
argumento prático."

### §2.F — Decisão justificativa 4 (Industry precedent)

**Antes**: "Cairo (Type 6/7), Inkscape (Type 7), cristalino (Type 4)
— todos família mesh-based; krilla Type 1 é outlier."

**Depois**: "Cairo (Type 6/7), Inkscape (Type 7), Typst original
pré-krilla (Type 6 Coons; blog 2023 documenta), cristalino (Type 4)
— todos família mesh-based. Krilla actual é opaco. Cristalino
divergência é intra-família mesh."

### §2.G — Scope-outs Type 1 + Type 6/7

- **Type 1**: "scope-out permanente; vanilla outlier; cristalino
  industry-aligned" → "scope-out permanente. Rejeitada historicamente
  também pelo Typst original (blog 2023: 'several readers do not
  support this feature'). Cristalino segue precedente, não diverge."
- **Type 6/7**: adicionado "Typst original usa Type 6 Coons (1 patch
  por stop); cristalino diverge intra-família" + candidato refino
  futuro nomeado "P-Gradient-Coons-Patch".

### §2.H — Nota metodológica

Reorganizada em **3 secções** distintas: "Verificáveis literal no
filesystem cristalino" (paint.rs:255 + convert.rs:514, com claims
factuais corrigidos) + "Verificáveis via web (não filesystem)"
(blog Typst 2023, W3C Workshop, Igalia, pdf.js issue, ISO 19005-1)
+ "Não verificáveis literal sem ler código privado" (krilla,
Chrome).

Bloco final "Correcção pré-commit P268.1" registra os 3 retract
factuais e a preservação da decisão de fundo.

### §2.I — Referências

5 entradas novas adicionadas (blog Typst 2023, W3C Workshop bfo,
ISO 19005-1:2005, typst/typst PR #5420 Part 7, Igalia blog). 2
entradas existentes (paint.rs:255, convert.rs:514) corrigidas para
remover claims factuais retractados ("confirma /ShadingType 1
Function-Based via krilla", "confirma Type 1 incompatibilidade
PDF/A-1").

---

## §3 — Verificação anotações dependentes (status commit)

### §3.1 — ADR-0089 anotação P268.1 — corrigida literal

Working tree, não commitada. Edição literal aplicada. Mudou:
- "Type 4 cristalino vs Type 1 vanilla" → "Type 4 cristalino vs
  estratégia vanilla actual desconhecida (krilla `SweepGradient`
  interno opaco; Typst original pré-krilla era Type 6 Coons per
  blog 2023)".
- "cristalino Type 4 alinhado com Cairo/Inkscape industry mesh-based
  standard; vanilla via krilla Type 1 é outlier" → "cristalino Type 4
  alinhado com industry mesh-based standard (Cairo Type 6/7,
  Inkscape Type 7, Typst original Type 6 Coons). Divergência
  intra-família mesh (Type 4 vs Type 6), não entre famílias. Krilla
  actual produce strategy desconhecida."
- Cross-reference adicionada para §"Nota metodológica de proveniência".

Status `IMPLEMENTADO` preservado literal.

### §3.2 — ADR-0054 anotação P268.1 — corrigida literal

Working tree, não commitada. Edição literal aplicada. Mudou:
- "divergência arquitectural Conic PDF Type 4 vs Type 1 vanilla"
  → "Type 4 cristalino vs estratégia vanilla actual desconhecida
  (Typst original pré-krilla era Type 6 Coons)".
- "divergente bit-exact de vanilla" → "divergente intra-família
  mesh (Type 4 vs Type 6 Typst original / Type 6-7 Cairo)".
- Justificação "compatibilidade pdf.js/PDF-A" → "simplicidade
  implementação Type 4 vs Type 6; compatibilidade reader".

Status `EM VIGOR` preservado literal.

### §3.3 — L0 `entities/gradient.md` anotação P268.1 — corrigida literal

Working tree, não commitada. Edição literal aplicada. Mudou:
- "Type 4 cristalino vs Type 1 vanilla" → "Type 4 cristalino vs
  estratégia vanilla actual desconhecida (krilla `SweepGradient`
  interno opaco; Typst original pré-krilla era Type 6 Coons per
  blog 2023 — Part 7 #5420 transitou para krilla)".
- "Vanilla via krilla `SweepGradient` (`/ShadingType 1`
  Function-Based) é outlier industry — pdf.js renderiza pink
  fallback; PDF/A-1 proíbe Type 1" → removido (afirmação retractada).
- "alinhado com Cairo/Inkscape mesh-based standard" → "alinhado
  com Cairo (Type 6/7) / Inkscape (Type 7) / Typst original
  (Type 6 Coons) — todos família mesh-based; divergência
  intra-família mesh (Type 4 vs Type 6)".

**Hash propagado**: edição literal ao L0 alterou o hash do prompt;
`crystalline-lint --fix-hashes` propagou o novo hash (`5cf78d81`)
para `01_core/src/entities/gradient.rs` header (única ocorrência;
zero violations pós-propagação).

### §3.4 — README ADRs entrada P268.1 — corrigida literal

Working tree, não commitada. Edição literal aplicada:
- Tabela ADR-0090 linha: título e descrição actualizados +
  parágrafo "correcção pré-commit P268.1" registando retract.
- Tabela ADR-0089 linha (segmento anotação P268.1): factualização
  corrigida + adicionado "anotação corrigida pré-commit P268.1".
- §"Passo 268.1" passos-chave: substituído "Type 4 vs Type 1
  vanilla" por "Type 4 vs industry mesh variants"; "Krilla/vanilla
  typst Type 1 é outlier" retraído; "intra-família mesh (Type 4 vs
  Type 6 Typst original/Cairo)" adicionado; novo parágrafo
  **Marco correcção pré-commit P268.1**.
- Linha "Distribuição" parêntese "Type 4 vs Type 1" → "Type 4 vs
  industry mesh variants".

Total ADRs **77 preservado**. Distribuição preservada.

---

## §4 — Sub-padrões + N cumulativo

| Subpadrão | N pós-correcção | Nota |
|---|---|---|
| Diagnóstico empírico web em vez de filesystem | **N=2 → N=3** | + correcção P268.1 (P268.1 inaugural + P268.2 + **correcção P268.1** — verificação Kimi via web_search blog Typst + W3C + ISO 19005-1) |
| Descoberta empírica que recalibra spec autor | **N=1 → N=2** | + correcção P268.1 (P268.2 inaugurou factor_delta CIELab → Oklab; **correcção P268.1** estende para retracts factuais em ADR EM VIGOR pré-commit) |
| Auto-aplicação ADR-0065 inline | N=9 preservado | correcção documental |
| Auditoria condicional (ADR-0084) | N=10 preservado | sem nova Fase A |
| **Correcção ADR pré-commit** | **N=1; deliberadamente NÃO formalizado** | anti-pattern; normalizar incentivaria editar ADRs livremente. Este caso é excepção justificada (ADR ainda não commitada + achado factual externo verificado), não pattern reutilizável. |

---

## §5 — Métricas finais

| Métrica | Pré-correcção | Pós-correcção | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2428 | 2428 | **0 (cap 0 LOC)** |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 | **+1** (L0 `entities/gradient.md` anotação P268.1 corrigida → hash do prompt mudou; propagado para `01_core/src/entities/gradient.rs` header via `crystalline-lint --fix-hashes`; ADR-0090 e ADRs editadas são documentação pura sem hashes) |
| ADRs totais | 77 | 77 | **0 (sem ADR nova)** |
| ADRs EM VIGOR | 33 | 33 | 0 |
| ADRs IMPLEMENTADO | 29 | 29 | 0 |
| Ficheiros documentais editados | — | 5 | ADR-0090, ADR-0089, ADR-0054, L0 gradient.md, README ADRs |
| Ficheiros documentais criados | — | 1 | este relatório |
| LOC L1/L3/stdlib alterado | — | — | **0 (cap exacto)** |
| Afirmações factuais retractadas | — | 3 | krilla=Type 1 (não verificada); PDF/A-1=functions proibidas (imprecisa); vanilla=outlier total (parcialmente errada) |

### §política condições verificadas

- 1 (anotações dependentes não commitadas — edição literal viável). ✓
- 2 (ADR-0090 não commitada — edição literal viável). ✓
- 3 (sem trecho ambíguo — todas as edições foram substituições literais). ✓
- 4 (lint zero violations confirmado). ✓
- 5 (tests workspace 2428 preservados; sem código alterado). ✓
- 6 (README distribuição 77 preservado). ✓
- 7 (cap 0 LOC respeitado). ✓

---

## §6 — Pesquisa Kimi + web_search consolidada

### §6.1 — Achado central Kimi (corroborado web)

Blog Typst "Color gradients and my gradual descent into madness"
(typst.app/blog/2023/color-gradients/) lista as três técnicas
testadas pelo autor original (Typst pré-krilla):

1. **Sampled pattern** (imagem pré-renderizada): rejeitada por
   bloat e pixelização.
2. **PostScript function**: rejeitada porque "several readers do
   not support this feature".
3. **Coons patch method (Type 6)**: a que acabaram por adotar.
   Citação literal: *"we can still use Coons patches, but we
   need to create at least as many patches as there are stops
   in the gradient."*

Devido a Apple PDF reader não suportar shading function em Coons
patches.

### §6.2 — W3C CSS-Color-4 Workshop 2021 (Mike Bremford, bfo)

> "These are implemented in PDF as Coons Patch shading. The concept
> isn't directly a part of CSS, but they're the only way we can
> render conic gradients."

Reforça que Coons é industry-standard para conic em PDF.

### §6.3 — Transição typst → krilla

Tracking issue typst/typst #2282 (Part 7) regista "Switch PDF backend
to krilla #5420". Sem documentação pública sobre estratégia interna
krilla para conic. `lab/typst-original/.../typst-pdf/src/paint.rs:255`
mostra `krilla::SweepGradient` sendo inicializada para
`Gradient::Conic` — mas `SweepGradient` é abstracção; o PDF emitido
pode usar internamente Type 4/6/7 ou outra estratégia. **Não
verificável literal sem ler código krilla**.

### §6.4 — Cairo confirmado mesh-based

Igalia/Cairo blog: "A Coons patch comes very handy to paint a conical
gradient." Cairo usa Type 6/7 mesh patches, **não Type 4 Gouraud
puro** — cristalino diverge intra-família mesh.

### §6.5 — ISO 19005-1:2005 §6.2.7

Proíbe PostScript XObjects (streams PostScript embutidas como
XObjects independentes); **não** proíbe Type 4 calculator functions
usadas em shading dictionaries. Type 4 functions são subset restrito
(sem loops/variáveis/subrotinas) e formalmente permitidas. Suporte
reader inconsistente é o argumento prático real, não proibição
formal PDF/A-1.

---

## §7 — Referências

### Cross-passos

- **P268** — PDF Conic Type 4 Gouraud materializado.
- **P268.1** — ADR-0090 criada (com erros factuais agora corrigidos
  por este passo de correcção pré-commit).
- **P268.2** — Refino adaptive N hybrid (já executado; preservado
  literal; não afectado pela correcção factual).

### ADRs editadas

- **ADR-0090** — `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`
  — editada literal §2.A-§2.I; status `EM VIGOR` preservado.
- **ADR-0089** — `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md`
  — anotação cumulativa P268.1 corrigida literal; status
  `IMPLEMENTADO` preservado.
- **ADR-0054** — `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md`
  — anotação cumulativa P268.1 corrigida literal; status `EM VIGOR`
  preservado.

### Documentos cristalinos editados (não-ADR)

- `00_nucleo/prompts/entities/gradient.md` — anotação P268.1 corrigida
  literal (sem hash drift propagável; L0 prompt édito mas conteúdo
  factual; hashes recalculados se necessário pelo lint).
- `00_nucleo/adr/README.md` — tabela ADR-0090 + tabela ADR-0089
  segmento P268.1 + §"Passo 268.1" passos-chave + parêntese
  Distribuição corrigidos literal.

### Documentos criados

- `00_nucleo/materialization/typst-passo-268.1-correcao-relatorio.md`
  — este relatório.

### Fontes empíricas (verificáveis via web)

- **Blog Typst "Color gradients and my gradual descent into madness"**
  (typst.app/blog/2023/color-gradients/) — Typst original Coons Type 6.
- **W3C Workshop CSS-Color-4 Mike Bremford** (2021, bfo) — Coons como
  única forma render conic em PDF.
- **ISO 19005-1:2005 §6.2.7** — PDF/A-1 PostScript XObjects restriction.
- **typst/typst issue #2282 Part 7 / PR #5420** — transição para
  krilla.
- **Igalia blog "Renderization of Conic gradients"** (2020) — Cairo
  Coons patches.
- **pdf.js issue #19233** — Unsupported ShadingType: 1.

### Vanilla literal (verificável filesystem cristalino)

- `lab/typst-original/crates/typst-pdf/src/paint.rs:255` —
  `SweepGradient` inicializado para `Gradient::Conic` (abstracção
  apenas; estratégia PDF interna não verificada literal).
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` —
  warning "conic gradients are not supported in this PDF standard".

### ADRs referenciados (não editados)

- **ADR-0018** — Whitelist crates externas (krilla não autorizada;
  preservada literal).
- **ADR-0085** — Diagnóstico imutável (estendido pela correcção
  para fontes externas).
- **ADR-0034** — Diagnóstico canónico.
- **ADR-0033** — Paridade funcional vanilla (esta correcção
  documenta que a divergência vs Typst original é **intra-família
  mesh**, não entre famílias).
- **ADR-0029** — Pureza física L1 + simplificações requerem ADR
  explícita.
