# typst-passo-268.1 — ADR-0090 Gradient Conic Type 4 cristalino vs Type 1 vanilla (divergência arquitectural empírica)

**Magnitude**: XS (cap: ≤ 0 LOC L1/L3/stdlib; só ADR-0090 documento ~150 linhas + anotações cumulativas + README).
**Cluster**: Visualize / Gradient / PDF export (documental).
**Tipo**: passo administrativo XS numerado .1. Pattern análogo P156K/P160A/P260; sub-padrão "Passo administrativo XS criar/promover ADR" **N=4 → N=5 cumulativo**.
**Origem**: pesquisa industry (Cairo/Inkscape/Skia/krilla/pdf.js) revela que decisão P268 Type 4 Gouraud não é divergência arbitrária de vanilla — é alinhamento com industry standard mesh-based; vanilla via krilla Type 1 é o outlier.
**Sequência**: P268 (Type 4 Gouraud manual) → **P268.1 (ADR-0090 EM VIGOR formalizando decisão arquitectural)** → P268.2 (refino adaptive N hybrid; spec dedicada futura).
**Status decisão**: utilizador escolheu Op 1 + (d) — ADR scope-out empiricamente justificada + refino qualidade visual em passo separado.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md preservada literal** — sem código L1/L3/stdlib alterado. Apenas documentação. **Ordem reduzida**: pesquisa industry (concluída na sessão Claude web) → ADR-0090 criação → anotações cumulativas → README.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). **Sem Fase A nova** — diagnóstico empírico é a pesquisa industry consolidada na sessão Claude web (este passo). Excepcionalmente, **não cria ficheiro `diagnostico-*-passo-268-1.md`** porque pesquisa industry foi feita via web_search Claude web, não via filesystem cristalino. Decisão consciente: registar achados industry **na própria ADR-0090** como secção §"Pesquisa empírica industry" imutável.

3. **ADR-0018** (whitelist crates externas) **preservada literal**. ADR-0090 confirma que krilla permanece não autorizada; cristalino mantém implementação autónoma.

4. **ADR-0084** (auditoria condicional) **não aplicável** — passo é puramente documental; sem materialização condicional.

5. **ADR-0089 anotação cumulativa P268.1** — cross-reference para ADR-0090 nova. Sub-padrão "Anotação cumulativa em vez de ADR nova" **NÃO se aplica aqui** — neste passo cria-se ADR nova distinta (ADR-0090) porque divergência arquitectural Type 4 vs Type 1 é decisão estrutural distinta da decisão minimalista P267/P268.

6. **ADR-0054 perfil graded DEBT-1** — anotação cumulativa P268.1: divergência arquitectural empírica documentada; cluster Gradient PDF mantém-se 3/3 em estratégia conservadora industry-aligned.

7. **ADR-0087/ADR-0088/ADR-0089 preservadas** — Linear/Radial/Conic L1 intocados.

8. **ADR-0039 preservado** — TextStyle intocado.

9. **Crystalline-lint zero violations** obrigatório (apenas hash drift L0 documental).

10. **Vanilla read-first preservado** — secção §"Pesquisa empírica industry" da ADR-0090 cita literal o que vanilla typst usa (`paint.rs:255` krilla SweepGradient) sem alterar essa decisão; é registo factual.

---

## §1 — Sub-passo P268.1.A — Achados industry consolidados (substitui Fase A filesystem)

**Não há Fase A filesystem** — a "Fase A" deste passo é a pesquisa industry consolidada na sessão Claude web, registada literal em ADR-0090 §"Pesquisa empírica industry".

### Achados factuais consolidados (a serem citados literal em ADR-0090)

| Projecto | Estratégia conic PDF | Tipo PDF |
|---|---|---|
| Krilla (typst usa) | PostScript Function-Based Shading | `/ShadingType 1` (vendor-specific) |
| Cairo | Coons/Tensor patch mesh via `cairo_mesh_pattern_*` | `/ShadingType 6` ou `/ShadingType 7` |
| Inkscape | Tensor patch mesh (suporte interno; user-facing limitado) | `/ShadingType 7` |
| Skia (rendering, não PDF) | SkSL shaders SweepGradient | N/A (rendering directo) |
| **Cristalino P268** | **Free-Form Gouraud Triangle Mesh** | **`/ShadingType 4`** |
| pdf.js (consumidor) | Não suporta `/ShadingType 1` (renderiza pink fallback) | — |

### Conclusões factuais

1. **PDF spec ISO 32000-1 oficial reconhece como gradient shadings apenas Types 2 (axial) e 3 (radial)** — conic não é parte oficial do spec. Tipos 4/5/6/7 são mesh shadings genéricos que podem ser usados para emular conic.

2. **Type 1 Function-Based Shading usado por krilla/Chrome é tecnicamente extensão vendor-specific quando aplicado a conic** — pdf.js (Firefox PDF viewer) regista isto explicitamente como "Unsupported ShadingType: 1" e renderiza pink fallback.

3. **Industry mesh-based abordagem (Type 4/6/7) é a mais comum em projectos PDF maduros**:
   - Cairo (Type 6/7) — 20+ anos maturidade.
   - Inkscape (Type 7) — segue Cairo.
   - Cristalino (Type 4) — escolha mais simples da família mesh.

4. **Cor central em mesh shadings (Type 4/6/7) é vértice explícito com cor explícita por convenção PDF** — não é divergência arbitrária; é como mesh-based PDF shading funciona. Cairo precedente: "primeira cor no vértice inicial".

5. **Krilla/vanilla typst é o outlier no industry**, não cristalino. Cristalino está alinhado com Cairo/Inkscape industry standard mesh-based.

---

## §2 — Sub-passo P268.1.B — ADR-0090 criação EM VIGOR

Ficheiro novo `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`.

### Estrutura ADR-0090

```
# ADR-0090 — Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) vs Type 1 PostScript (vanilla)

**Status**: EM VIGOR
**Data**: 2026-05-15
**Passo origem**: P268.1
**Cluster**: Visualize / Gradient / PDF export
**Tipo**: divergência arquitectural empiricamente justificada

## Contexto

P268 materializou Conic PDF shading via /ShadingType 4 Free-Form Gouraud
Triangle Mesh com N=32 fatias e cor central = primeiro stop. P268.1.PRE
revelou que vanilla typst via krilla::SweepGradient usa /ShadingType 1
Function-Based Shading com PostScript function `atan2`.

P268.1 pesquisa industry (Cairo, Inkscape, Skia, pdf.js, krilla) revelou
que **não há estratégia canónica única** para conic em PDF — cada
projecto inventou a sua. Cristalino escolha Type 4 é alinhamento com
industry mesh-based standard, não divergência arbitrária de vanilla.

## Pesquisa empírica industry

[Tabela achados §1.A literal]

## Decisão

**Cluster Gradient cristalino usa /ShadingType 4 Free-Form Gouraud
Triangle Mesh para Conic, divergindo deliberadamente de vanilla
(/ShadingType 1 Function-Based via krilla)**.

Justificativas factuais ordenadas por prioridade:

1. **PDF spec ISO 32000-1 oficial** reconhece apenas Types 2/3 como
   gradient shadings; Types 4/5/6/7 são mesh shadings genéricos
   adequados; Type 1 conic é extensão vendor-specific.

2. **Compatibilidade PDF readers**: Type 4 funciona em todos os
   readers (pdf.js, Chrome, Adobe, mupdf, poppler); Type 1 conic
   falha em pdf.js (Firefox).

3. **Compatibilidade PDF/A standards restritivos**: Type 1
   Function-Based proibido em PDF/A-1 (PostScript não permitido);
   Type 4 permitido. Vanilla typst `convert.rs:514` emite warning
   "conic gradients are not supported in this PDF standard"
   precisamente porque Type 1 falha em PDF/A — Type 4 funcionaria.

4. **Industry precedent**: Cairo (Type 6/7), Inkscape (Type 7),
   cristalino (Type 4) — todos família mesh-based; krilla Type 1 é
   outlier.

5. **Performance reader cliente**: Type 4 = interpolação linear
   simples; Type 1 = interpreter PostScript por pixel.

6. **LOC cristalino**: Type 4 ~190 LOC; Type 1 estimado ~400-500 LOC.

7. **ADR-0018 preservado**: cristalino implementa autonomamente;
   nenhuma dependência externa requerida.

## Convenção cor central

Cor central = primeiro stop. Convenção PDF mesh shading estabelecida
(Cairo precedente: "color assigned to the corner at the start of the
path; follows Cairo conventions which follows the PDF convention").

**Não é decisão arbitrária** — é convenção PDF mesh shading inerente
ao Type 4 onde cada vértice tem cor explícita; centro do disco é
vértice; primeiro stop é convenção industry estabelecida.

## Consequências

+ Cluster Gradient PDF compatível com todos PDF readers, incluindo
  pdf.js que vanilla quebra.
+ PDFs cristalino renderizam em PDF/A-1 onde vanilla emite warning.
+ Cristalino alinhado com Cairo/Inkscape industry mesh-based
  standard.
+ Implementação autónoma; ADR-0018 preservado.
- Divergência observable bit-exact de vanilla em casos extremos
  (gradientes com contraste muito alto em N=32 fatias produzem
  banding ligeiramente perceptível; vanilla Type 1 é matematicamente
  suave).
- Refino qualidade visual fica pendente P268.2 (adaptive N hybrid).

## Scope-outs preserved

- **Type 1 PostScript Function**: scope-out permanente; vanilla
  outlier; cristalino industry-aligned.
- **Type 6/7 Coons/Tensor patches**: scope-out actual; cristalino
  escolha Type 4 por simplicidade implementação; candidato refino
  futuro se Type 4 banding for problema real (improvável dado
  hybrid adaptive N P268.2).
- **PDF/A-1 explicit support**: scope-out; Type 4 funciona em
  PDF/A-1 mas cristalino não declara PDF/A compliance.

## Alternativas consideradas

- **Re-materializar Type 1 PostScript** (P-Gradient-Conic-PS futuro):
  rejeitada — perde compatibilidade pdf.js; perde compatibilidade
  PDF/A; LOC 3-4x; ADR-0018 cliente.
- **Re-materializar Type 6 Coons**: rejeitada — magnitude L; precedent
  Cairo mais complexo do que necessário para conic simples; Type 4
  suficiente para qualidade visual com adaptive N P268.2.
- **Manter Type 4 sem ADR**: rejeitada — divergência arquitectural
  vs vanilla precisa documentação formal para futuro.

## Critério revisão

Esta ADR pode ser revisitada se:
- pdf.js ou outro reader major passar a suportar Type 1 conic
  (compatibility argument enfraquece).
- Adaptive N P268.2 revelar-se insuficiente para qualidade visual
  (Type 6 Coons torna-se candidato).
- ADR-0018 mudar e krilla for autorizada (improvável).
- Industry standard mudar (improvável a curto prazo).

## Subpadrões aplicados

- **Diagnóstico imutável**: pesquisa empírica industry (web_search
  Claude web) consolidada §"Pesquisa empírica industry"; sem ficheiro
  filesystem porque fonte é externa.
- **Passo administrativo XS criar/promover ADR**: P268.1 N=5
  cumulativo.
- **ADR scope-out preserved com justificativa empírica**: N=1 inaugural
  (subpadrão novo).

## Referências cross-passos

- P267 — Gradient Conic L1+stdlib (ADR-0089).
- P268 — PDF Conic /ShadingType 4 Gouraud (decisão materializada;
  ADR-0089 §anotação P268).
- P268.2 — Refino adaptive N hybrid (spec futura; passo separado).
- ADR-0018 — Whitelist crates (krilla não autorizada; preservado).
- ADR-0089 — Gradient Conic-only (anotação cumulativa P268.1
  cross-reference esta ADR).
- ADR-0054 — Perfil graded (anotação cumulativa P268.1).
- ISO 32000-1 §7.5.7 — Shading Patterns Types 1-7.
- Cairo `cairo_mesh_pattern_*` API — precedente Type 6/7.
- Inkscape Mesh Gradients wiki — precedente Type 7.
- pdf.js issue #19233 — Unsupported ShadingType: 1 fallback pink.
- Vanilla `lab/typst-original/.../typst-pdf/src/paint.rs:255` —
  krilla::SweepGradient origem.
```

---

## §3 — Sub-passo P268.1.C — Anotações cumulativas

### C.1 — ADR-0089 anotação cumulativa P268.1

Adicionar após secção "Anotação cumulativa P268":

```
## Anotação cumulativa P268.1 — Cross-reference ADR-0090

**Data**: 2026-05-15.
**Motivo**: Divergência arquitectural Type 4 cristalino vs Type 1
vanilla formalizada em ADR-0090 dedicada (EM VIGOR) após pesquisa
empírica industry (Cairo/Inkscape/Skia/pdf.js/krilla).

**Conclusão factual**: cristalino Type 4 alinhado com Cairo/Inkscape
industry mesh-based standard; vanilla via krilla Type 1 é outlier.

Ver ADR-0090 §"Pesquisa empírica industry" + §"Decisão" + §"Convenção
cor central" para justificação completa.

**Cor central = primeiro stop** confirmada como convenção PDF mesh
shading (não decisão arbitrária P268).

**Refino qualidade visual** pendente P268.2 (adaptive N hybrid).
```

### C.2 — ADR-0054 anotação cumulativa P268.1

```
P268.1 — divergência arquitectural Conic PDF Type 4 vs Type 1 vanilla
formalizada via ADR-0090 EM VIGOR; cluster Gradient PDF mantém-se 3/3
em estratégia conservadora industry-aligned (Cairo/Inkscape precedent).
ADR-0018 preservado.
```

### C.3 — L0 prompt `entities/gradient.md` anotação P268.1

Adicionar à secção P268 existente (após linha "PDF emit P268: Type 4
Gouraud Manual"):

```
**Anotação P268.1**: divergência arquitectural Type 4 cristalino vs
Type 1 vanilla formalizada em ADR-0090 EM VIGOR; convenção cor central
= primeiro stop confirmada como industry standard Cairo/PDF.
```

### C.4 — Hashes propagados

`crystalline-lint --fix-hashes` propaga hash em
`entities/gradient.md`. Zero violations.

---

## §4 — Sub-passo P268.1.D — README + relatório

### D.1 — README ADRs

- **Tabela**: entrada ADR-0090 adicionada com status `EM VIGOR`.
- **Distribuição actualizada**: PROPOSTO 11; **EM VIGOR 32 → 33**;
  IMPLEMENTADO 29; **total 76 → 77**.
- **Passos-chave**: entrada P268.1 ~20-30 linhas (passo administrativo
  XS; menor que entrada P268 ou P267).
- **Cross-reference ADR-0089 + ADR-0054** anotações cumulativas P268.1.

### D.2 — Relatório

`00_nucleo/materialization/typst-passo-268-1-relatorio.md`:

- §1 Sumário executivo (ADR-0090 EM VIGOR criada; pesquisa industry
  consolidada; sem código alterado).
- §2 ADR-0090 estrutura literal.
- §3 Anotações cumulativas ADR-0089/ADR-0054/L0.
- §4 README distribuição actualizada.
- §5 Métricas finais (testes preservados 2413; hash drift propagado
  documental; lint zero).
- §6 Sub-padrões aplicados + N cumulativo.
- §7 Pesquisa industry consolidada (cópia §1.A spec).
- §8 Pendência P268.2 reservada (refino adaptive N hybrid).
- §9 Critério aceitação checklist.
- §10 Referências.

---

## §política de paragem

Claude Code para e pergunta se qualquer das seguintes condições ocorrer:

1. **ADR-0090 estrutura ambígua** — passo ADR-0089 anotação ou ADR-0054
   anotação revela conflito com texto ADR-0090 que requer arbitragem.

2. **Crystalline-lint reporta violations não-triviais** após anotações
   L0 (`entities/gradient.md` hash propagation).

3. **README distribuição inconsistente** — total ADRs antes/depois não
   bate (esperado: 76 → 77; PROPOSTO 11 preservado; EM VIGOR 32 → 33;
   IMPLEMENTADO 29 preservado).

4. **Tests workspace regressão** — testes que estavam verdes pré-P268.1
   (2413) ficam vermelhos. **Não deveria acontecer** porque sem código
   alterado; se acontecer, indica problema de build cache ou hash drift
   inesperado.

5. **Referência cruzada quebrada** — ADR-0090 cita ficheiro/linha
   vanilla (`paint.rs:255`) que não corresponde literal ao filesystem
   `lab/typst-original/` actual.

6. **Cap LOC ameaçado** — passo é XS por definição (cap 0 LOC L1/L3/
   stdlib); qualquer alteração de código indica scope creep e dispara
   esta condição.

7. **Industry research §1.A factualmente incorrecta** — Claude Code, ao
   verificar literal contra filesystem disponível, detecta que algum
   dos achados industry consolidados não é verificável ou está errado.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N após P268.1 | Nota |
|---|---|---|
| Passo administrativo XS criar/promover ADR | **N=4 → N=5** | + P268.1 (P156K/P160A/P260/ADR-0062-create/**P268.1**) |
| Auto-aplicação ADR-0065 inline | N=8 | + P268.1 |
| ADR scope-out preserved com justificativa empírica | **N=1 inaugural** | P268.1 (subpadrão novo; candidato formalização limiar N=3-5 cumulativo) |
| Diagnóstico imutável (pesquisa industry como fonte) | **N=10 estendido** | P268.1 é primeiro consumo de pesquisa empírica web em vez de filesystem |

### Marco arquitectural P268.1

**Primeira ADR explicitamente justificada por pesquisa industry
empírica** (web_search Claude web em vez de leitura filesystem vanilla
ou krilla). Estabelece precedente para futuras decisões arquitecturais
onde paridade vanilla bit-exact não é prioridade absoluta — alinhamento
industry standard pode ser justificativa válida para divergência.

### Sequência pós-P268.1

- **P268.2** — refino adaptive N hybrid 1+2 (critério número de stops
  + contraste cromático Oklab ΔE). Magnitude S; cap ~200 LOC + ~15
  testes. Melhora qualidade visual Type 4 sem mudar estratégia.

- **P-Gradient-Focal** (M) — activa focal_* Radial; revoga ADR-0088
  §focal scope-out.

- **ADR-0055bis variant-aware fonts** (M) — refino Text.

- **P-Footnote-N** (M) — Model pendência.

- **DEBT-33 Bézier bbox + outros Visualize** (S+M).

---

## §referências cross-passos

- **P268** — PDF Conic shading Type 4 Gouraud (decisão materializada
  formalizada por este passo).
- **P267** — Gradient Conic L1+stdlib (ADR-0089).
- **P268.2** — Refino adaptive N hybrid (spec futura; passo separado).
- ADR-0018 — Whitelist crates (preservado; krilla não autorizada).
- ADR-0089 — Gradient Conic-only (anotação cumulativa P268.1).
- ADR-0054 — Perfil graded (anotação cumulativa P268.1).
- **ADR-0090** — Esta ADR (criada EM VIGOR neste passo).

---

## §0.1 — Notas de execução para Claude Code

- **Sem alteração de código L1/L3/stdlib**. Cap 0 LOC. Se qualquer
  alteração de código for tentada, §política condição 6 dispara.
- **ADR-0090 estrutura literal** segue §2 spec; texto é praticamente
  pronto a copiar para o ficheiro ADR.
- **Anotações cumulativas** (ADR-0089 + ADR-0054 + L0 gradient.md)
  são edições pequenas (~5-10 linhas cada).
- **Hash drift** esperado em 1 ficheiro L0 (`entities/gradient.md`).
- **Distribuição ADRs**: total 76 → 77 (ADR-0090 criada EM VIGOR
  directamente; sem passagem por PROPOSTO porque é formalização de
  decisão já tomada em P268, não decisão nova).
- **Tests workspace**: 2413 preservados; sem testes novos; sem
  regressão esperada.
- **Pesquisa industry §1.A** pode ser verificada literal contra
  `lab/typst-original/.../typst-pdf/src/paint.rs:242-267` (krilla
  SweepGradient) e `convert.rs:514` (PDF/A warning); demais citações
  industry são reproduções da pesquisa web Claude web e não
  verificáveis literal no filesystem cristalino (registar isto na
  ADR-0090 §"Pesquisa empírica industry" como nota metodológica).
- **Relatório final esperado**: 2413 testes verdes preservados; hash
  drift 1 ficheiro propagado; lint zero violations; ADRs 76 → 77;
  zero código alterado.
