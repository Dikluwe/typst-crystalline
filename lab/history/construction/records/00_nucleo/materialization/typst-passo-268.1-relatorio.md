# Relatório P268.1 — ADR-0090 EM VIGOR (divergência arquitectural Conic PDF Type 4 vs Type 1 vanilla industry-aligned)

**Data**: 2026-05-15.
**Magnitude**: XS (cap 0 LOC L1/L3/stdlib; documental).
**Cluster**: Visualize / Gradient / PDF export (documental).
**Tipo**: passo administrativo XS numerado `.1`.
**Origem**: pesquisa industry consolidada na sessão Claude web revelou que P268 Type 4 Gouraud não é divergência arbitrária — é alinhamento com industry mesh-based standard; vanilla via krilla Type 1 é outlier.
**Spec**: `00_nucleo/materialization/typst-passo-268.1.md`.

---

## §1 — Sumário executivo

Passo administrativo XS puramente documental que formaliza a
**divergência arquitectural Conic PDF Type 4 cristalino vs
Type 1 vanilla** via ADR-0090 nova directamente em estado
`EM VIGOR`. Sem alteração de código L1/L3/stdlib.

**Decisão materializada**:
- **ADR-0090 criada `EM VIGOR`** complementando ADR-0089
  §anotação P268 com justificativa empírica industry registada
  literal em §"Pesquisa empírica industry" (Cairo/Inkscape/
  Skia/pdf.js/krilla).
- **ADR-0089 anotada P268.1** cross-reference ADR-0090.
- **ADR-0054 anotada P268.1** cluster Gradient PDF
  industry-aligned.
- **L0 `entities/gradient.md` anotado P268.1** após anotação
  P268.
- **README ADRs**: total **76 → 77**; EM VIGOR **32 → 33**;
  PROPOSTO 11 + IMPLEMENTADO 29 preservados.

**Subpadrões inaugurados**:
- "ADR scope-out preserved com justificativa empírica" **N=1
  inaugural** (candidato formalização limiar N=3-5 cumulativo).
- "Diagnóstico empírico web em vez de filesystem" **N=1
  inaugural** (pesquisa web_search Claude web; ADR-0085
  estendido para fontes externas com nota metodológica).

**Subpadrão consolidado**:
- "Passo administrativo XS criar/promover ADR" **N=4 → N=5
  cumulativo** (P156K/P160A/P229/P254/**P268.1**).

**Marco arquitectural P268.1**: **primeira ADR explicitamente
justificada por pesquisa industry empírica** (web_search em vez
de leitura filesystem vanilla ou krilla). Estabelece precedente
para futuras decisões arquitecturais onde paridade vanilla
bit-exact não é prioridade absoluta — alinhamento industry
standard pode ser justificativa válida para divergência.

---

## §2 — ADR-0090 estrutura literal

Ficheiro: `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`.

**Status**: `EM VIGOR`.
**Data**: 2026-05-15.
**Passo origem**: P268.1.
**Cluster**: Visualize / Gradient / PDF export.
**Tipo**: divergência arquitectural empiricamente justificada.

### Secções

1. **Contexto** — P268 Type 4 materializado; P268.1.PRE revelou
   vanilla krilla Type 1; sem estratégia canónica única no
   industry.
2. **Pesquisa empírica industry** (tabela 6 projectos +
   conclusões factuais 1-5 + **nota metodológica** sobre
   verificabilidade vanilla vs web).
3. **Decisão** — Type 4 cristalino com 7 justificativas factuais
   prioritizadas (PDF spec oficial + pdf.js + PDF/A-1 + industry
   precedent + performance + LOC + ADR-0018 preservado).
4. **Convenção cor central = primeiro stop** (convenção PDF
   mesh shading; Cairo precedente literal).
5. **Consequências** (positivas/negativas/neutras).
6. **Scope-outs preserved** (Type 1 PostScript permanente; Type
   6/7 actual; PDF/A-1 explicit).
7. **Alternativas consideradas** (α1-α4 com α4 escolhida).
8. **Critério revisão** (4 condições para reabrir).
9. **Subpadrão N=1 inaugural** "ADR scope-out preserved com
   justificativa empírica".
10. **Subpadrão N=1 inaugural** "Diagnóstico empírico web em
    vez de filesystem".
11. **Subpadrão N=4 → N=5 cumulativo** "Passo administrativo XS
    criar/promover ADR".
12. **Referências** (12+ entries: ADRs cross-reference + ISO
    32000-1 + Cairo/Inkscape/pdf.js + vanilla literal lines).
13. **Próximos passos** (P268.2 refino + P-Gradient-Focal +
    revisões condicionais).

### Justificativas factuais ordenadas

1. PDF spec ISO 32000-1 oficial: Types 2/3 gradient; Types 4-7
   mesh genéricos adequados; Type 1 conic vendor-specific.
2. Compatibilidade pdf.js (Firefox): Type 4 sim; Type 1 não.
3. Compatibilidade PDF/A-1: Type 4 sim; Type 1 proibido
   (vanilla emite warning `convert.rs:514`).
4. Industry precedent: Cairo/Inkscape/cristalino mesh; krilla
   outlier.
5. Performance reader: Type 4 interpolação linear; Type 1
   interpreter PostScript.
6. LOC: Type 4 ~190; Type 1 estimado ~400-500.
7. ADR-0018 preservado: implementação autónoma.

---

## §3 — Anotações cumulativas

### §3.1 — ADR-0089 anotação cumulativa P268.1

Adicionada após §"Anotação cumulativa P268" existente.

**Conteúdo essencial**: cross-reference ADR-0090; conclusão
factual cristalino Type 4 alinhado industry mesh-based; cor
central = primeiro stop confirmada como convenção PDF mesh
shading; refino qualidade visual pendente P268.2.

Status `IMPLEMENTADO` preservado literal (ADR-0089 cobre
L1+stdlib + decisão estratégia PDF materializada; ADR-0090
nova **complementa** formalizando justificativa empírica;
sem revogação).

### §3.2 — ADR-0054 anotação cumulativa P268.1

Adicionada como nova secção `## Anotação cumulativa P268.1`
após anotação P266 existente.

**Conteúdo essencial**: divergência arquitectural Conic PDF
formalizada via ADR-0090 EM VIGOR; cluster Gradient PDF
mantém-se 3/3 em estratégia conservadora industry-aligned;
perfil graded ADR-0054 válido para Visualize/Gradient porque
divergência é estratégica (compatibilidade pdf.js/PDF-A), não
simplificação per se; ADR-0018 preservado.

Status `EM VIGOR` preservado literal.

### §3.3 — L0 `entities/gradient.md` anotação P268.1

Adicionada após anotação P268 existente no fim do ficheiro.

**Conteúdo essencial**: divergência arquitectural Type 4
cristalino vs Type 1 vanilla formalizada em ADR-0090 EM VIGOR;
convenção cor central = primeiro stop confirmada como industry
standard Cairo/PDF; vanilla krilla `SweepGradient` é outlier
industry (pdf.js pink fallback; PDF/A-1 proíbe Type 1);
cristalino Type 4 alinhado Cairo/Inkscape mesh-based standard;
ADR-0018 preservado; refino qualidade visual pendente P268.2.

Hash do código L0 propagado via `crystalline-lint --fix-hashes`
(1 ficheiro afectado).

---

## §4 — README ADRs distribuição actualizada

### Tabela "Estado por ADR"

- **Nova linha** ADR-0090 `EM VIGOR` adicionada após ADR-0089.
- **Linha ADR-0089 estendida** com "+ anotação cumulativa
  P268.1" antes do total.

### Total ADRs

**76 → 77** (+ADR-0090 EM VIGOR P268.1).

### Distribuição

| Status | Pré-P268.1 | Pós-P268.1 | Delta |
|---|---|---|---|
| `PROPOSTO` | 11 | 11 | 0 |
| `IDEIA` | 2 | 2 | 0 |
| **`EM VIGOR`** | **32** | **33** | **+1 (ADR-0090)** |
| `IMPLEMENTADO` | 29 | 29 | 0 |
| `REVOGADO` | 2 | 2 | 0 |
| `ADIADO` | 1 | 1 | 0 |
| **Total** | **76** | **77** | **+1** |

### Passos-chave

Nova entrada `- **Passo 268.1**` adicionada após Passo 268.
~50 linhas (passo administrativo XS; menor que entrada P268
ou P267 conforme spec §4.D.1).

---

## §5 — Métricas finais

| Métrica | Pré-P268.1 | Pós-P268.1 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2413 | 2413 | 0 (cap 0 LOC) |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados | — | 1 (L0 gradient.md) | +1 |
| ADRs totais | 76 | 77 | +1 |
| ADRs EM VIGOR | 32 | 33 | +1 |
| LOC L1/L3/stdlib | — | — | **0 (preservado literal)** |
| Ficheiros documentais editados | — | 4 | +4 (ADR-0089, ADR-0054, L0 gradient.md, README ADRs) |
| Ficheiros documentais criados | — | 2 | +2 (ADR-0090, relatório P268.1) |

**Cap LOC respeitado**: 0 LOC L1/L3/stdlib alterado (§política
condição 6 não accionada).

**Tests preservados**: 2413 verdes (cap 0 LOC; sem testes
novos; zero regressões — §política condição 4 não accionada).

---

## §6 — Subpadrões aplicados + N cumulativo

| Subpadrão | N pós-P268.1 | Nota |
|---|---|---|
| Passo administrativo XS criar/promover ADR | **N=4 → N=5** | + P268.1 (P156K/P160A/P229/P254/**P268.1**) |
| ADR scope-out preserved com justificativa empírica | **N=1 inaugural** | P268.1 (subpadrão novo; candidato formalização limiar N=3-5 cumulativo) |
| Diagnóstico empírico web em vez de filesystem | **N=1 inaugural** | P268.1 (primeiro consumo pesquisa empírica web em vez de filesystem; ADR-0085 estendido com nota metodológica) |
| Auto-aplicação ADR-0065 inline | N=8 (preservado) | P268.1 não afecta contagem (puramente documental) |
| Diagnóstico imutável (fonte estendida web) | **N=10 estendido** | P268.1 estende ADR-0085 para fontes externas |

### Marco arquitectural P268.1

**Primeira ADR explicitamente justificada por pesquisa industry
empírica** (web_search Claude web em vez de leitura filesystem
vanilla ou krilla). Estabelece precedente para futuras decisões
arquitecturais onde paridade vanilla bit-exact não é prioridade
absoluta — alinhamento industry standard pode ser justificativa
válida para divergência.

---

## §7 — Pesquisa industry consolidada (cópia §1.A spec)

### Tabela achados factuais

| Projecto | Estratégia conic PDF | Tipo PDF |
|---|---|---|
| Krilla (typst usa) | PostScript Function-Based Shading | `/ShadingType 1` (vendor-specific) |
| Cairo | Coons/Tensor patch mesh via `cairo_mesh_pattern_*` | `/ShadingType 6` ou `/ShadingType 7` |
| Inkscape | Tensor patch mesh (suporte interno; user-facing limitado) | `/ShadingType 7` |
| Skia (rendering, não PDF) | SkSL shaders SweepGradient | N/A (rendering directo) |
| **Cristalino P268** | **Free-Form Gouraud Triangle Mesh** | **`/ShadingType 4`** |
| pdf.js (consumidor) | Não suporta `/ShadingType 1` (renderiza pink fallback) | — |

### Conclusões factuais

1. PDF spec ISO 32000-1 oficial reconhece como gradient shadings
   apenas Types 2 (axial) e 3 (radial); Types 4-7 são mesh
   shadings genéricos que podem ser usados para emular conic.
2. Type 1 Function-Based Shading usado por krilla/Chrome é
   tecnicamente extensão vendor-specific quando aplicado a
   conic — pdf.js (Firefox PDF viewer) regista isto
   explicitamente como "Unsupported ShadingType: 1" e renderiza
   pink fallback.
3. Industry mesh-based abordagem (Type 4/6/7) é a mais comum em
   projectos PDF maduros: Cairo (Type 6/7) 20+ anos maturidade;
   Inkscape (Type 7) segue Cairo; cristalino (Type 4) escolha
   mais simples da família mesh.
4. Cor central em mesh shadings é vértice explícito com cor
   explícita por convenção PDF — não é divergência arbitrária.
   Cairo precedente: "color assigned to the corner at the start
   of the path".
5. Krilla/vanilla typst é o outlier no industry, não cristalino.

### Nota metodológica de proveniência

Verificáveis literal no filesystem cristalino:
- `lab/typst-original/crates/typst-pdf/src/paint.rs:255` —
  `SweepGradient` inicializado para `Gradient::Conic`.
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` —
  hint "conic gradients are not supported in this PDF standard".

Não verificáveis literal no filesystem cristalino (reproduções
pesquisa web Claude web): Cairo `cairo_mesh_pattern_*`,
Inkscape Mesh Gradients wiki, pdf.js issue Unsupported
ShadingType: 1.

Registadas em ADR-0090 §"Pesquisa empírica industry" + §"Nota
metodológica" per ADR-0085 §"diagnóstico imutável" estendido.

---

## §8 — Pendência P268.2 reservada (refino adaptive N hybrid)

**P268.2 — Refino qualidade visual Type 4 via adaptive N
hybrid** (spec dedicada futura; não materializada neste passo).

- **Magnitude**: S (~200 LOC + ~15 testes).
- **Estratégia hybrid 1+2**: N adaptive baseado em
  - critério (1) número de stops (mais stops → mais fatias);
  - critério (2) contraste cromático Oklab ΔE entre stops
    consecutivos (ΔE alto → mais fatias para reduzir banding).
- **Não revoga ADR-0090** — refina aplicação Type 4 sem mudar
  estratégia.
- **Cap**: respeita ADR-0018 (krilla não autorizada); helpers
  Oklab P262 reutilizados literal (subpadrão "Reutilização
  literal helpers cross-passos" potencial N=3 → N=4).

P268.2 fica em aberto para activação humana posterior.

---

## §9 — Critério aceitação checklist

- [x] **ADR-0090 criada** com estrutura §2 spec (cabeçalho
      canónico + 13 secções + status `EM VIGOR` directo).
- [x] **ADR-0089 anotada P268.1** após §"Anotação cumulativa
      P268"; status `IMPLEMENTADO` preservado literal.
- [x] **ADR-0054 anotada P268.1** após anotação P266; status
      `EM VIGOR` preservado literal.
- [x] **L0 `entities/gradient.md` anotado P268.1** após
      anotação P268; hash propagado.
- [x] **README ADRs**: tabela + total 76 → 77; distribuição
      EM VIGOR 32 → 33; entrada §"Passo 268.1" passos-chave.
- [x] **Relatório P268.1** criado (este ficheiro) com 10
      secções per §4.D.2 spec.
- [x] **Cap 0 LOC** respeitado (zero código L1/L3/stdlib
      alterado).
- [x] **Tests workspace** 2413 preservados.
- [x] **Lint zero violations** após `crystalline-lint
      --fix-hashes`.
- [x] **Pesquisa industry §1.A factualmente verificada**
      literal contra `paint.rs:255` (`SweepGradient` confirmado)
      + `convert.rs:514` (warning PDF/A Conic confirmado);
      demais citações industry registadas com nota metodológica
      de proveniência em ADR-0090.

**§política condições NÃO accionadas**: 1 (sem ambiguidade), 2
(lint zero), 3 (distribuição consistente 76 → 77), 4 (tests
preservados), 5 (referências cruzadas verificadas), 6 (cap LOC
respeitado), 7 (pesquisa industry verificada).

---

## §10 — Referências

### Cross-passos

- **P267** — Gradient Conic L1+stdlib (ADR-0089).
- **P268** — PDF Conic /ShadingType 4 Gouraud (decisão
  materializada formalizada por este passo).
- **P268.2** — Refino adaptive N hybrid (spec futura; passo
  separado).

### ADRs

- **ADR-0090** — Esta ADR (criada EM VIGOR neste passo).
- **ADR-0089** — Gradient Conic-only L1+stdlib (anotação
  cumulativa P268.1 cross-reference ADR-0090; status
  `IMPLEMENTADO` preservado).
- **ADR-0054** — Perfil graded DEBT-1 (anotação cumulativa
  P268.1; status `EM VIGOR` preservado).
- **ADR-0018** — Whitelist crates externas (krilla não
  autorizada; preservada literal).
- **ADR-0085** — Diagnóstico imutável (estendido para fontes
  externas com nota metodológica em P268.1).
- **ADR-0034** — Diagnóstico canónico (estendido para web).
- **ADR-0033** — Paridade funcional vanilla (esta ADR-0090
  documenta divergência observable consciente).
- **ADR-0029** — Pureza física L1 + simplificações requerem
  ADR explícita.
- **ADR-0080** — L0 minimal refactor aditivo (paridade pattern
  P229/P254/P268.1; sem código alterado).
- **ADR-0061** — Granularidade 1-2 features/passo (P268.1
  documental; P268.2 refino separado).

### Documentos cristalinos editados

- `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md` (criado).
- `00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (anotado §P268.1).
- `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` (anotado §P268.1).
- `00_nucleo/adr/README.md` (tabela + distribuição + passos-chave).
- `00_nucleo/prompts/entities/gradient.md` (anotado P268.1; hash propagado).
- `00_nucleo/materialization/typst-passo-268-1-relatorio.md` (este relatório).

### Vanilla literal (verificável)

- `lab/typst-original/crates/typst-pdf/src/paint.rs:255` —
  `SweepGradient` (krilla; `/ShadingType 1`).
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` —
  warning "conic gradients are not supported in this PDF
  standard".

### Industry (web; não verificável literal filesystem cristalino)

- ISO 32000-1 §7.5.7 — Shading Patterns Types 1-7.
- Cairo `cairo_mesh_pattern_*` API — precedente Type 6/7.
- Inkscape Mesh Gradients wiki — precedente Type 7.
- pdf.js issue #19233 — Unsupported ShadingType: 1 pink
  fallback.
