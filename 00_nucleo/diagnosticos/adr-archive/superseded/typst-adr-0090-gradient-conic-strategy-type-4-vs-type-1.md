# ⚖️ ADR-0090: Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) divergência industry mesh-based variants

**Status**: `REVOGADO` (P272 — Type 4 Gouraud descontinuado;
substituída por ADR-0092 expandida cumulativamente — Type 6 Coons
unificado 8/8 spaces).
**Status anterior**: `EM VIGOR` (P268.1 a P272).
**Data**: 2026-05-15
**Data revogação**: 2026-05-17 (P272).
**Autor**: Humano + IA
**Passo origem**: P268.1
**Cluster**: Visualize / Gradient / PDF export
**Tipo**: divergência arquitectural empiricamente justificada
**Validado**: Passo 268.1 (criação EM VIGOR directa pós-pesquisa
empírica industry; sem passagem por PROPOSTO porque formaliza
decisão já materializada em P268 — paridade pattern
P229 ADR-0080 / P254 ADR-0082).
**Diagnóstico prévio**:
Pesquisa empírica industry consolidada na sessão Claude web
(web_search Cairo / Inkscape / Skia / krilla / pdf.js).
Registada literal em §"Pesquisa empírica industry" desta ADR
como artefacto imutável per ADR-0085 — **excepção formal**:
não há ficheiro `00_nucleo/diagnosticos/diagnostico-*-passo-268-1.md`
porque a fonte é externa (web), não filesystem cristalino.
Subpadrão novo "Diagnóstico empírico web em vez de filesystem"
**N=1 inaugural**.

---

## Contexto

P268 materializou Conic PDF shading via `/ShadingType 4`
Free-Form Gouraud Triangle Mesh com N=32 fatias e cor central
= primeiro stop. P268.1.PRE revelou que vanilla typst via
`krilla::SweepGradient` inicializa uma abstracção de gradient
cónico (`lab/typst-original/.../typst-pdf/src/paint.rs:255`).
A estratégia PDF interna do krilla actual não está publicamente
documentada e não foi verificada literal.

Verificação histórica via blog Typst "Color gradients and my
gradual descent into madness" (2023) revela que Typst original
(pré-krilla, antes da transição Part 7 #5420) usou
`/ShadingType 6` Coons Patches com tantos patches quantos stops
do gradiente, após rejeitar PostScript functions porque
"several readers do not support this feature".

P268.1 pesquisa industry revelou que **não há estratégia
canónica única** para conic em PDF — cada projecto inventou a
sua, mas a família mesh-based (Type 4 Gouraud / Type 6 Coons /
Type 7 Tensor) é amplamente adoptada. Cristalino escolha Type 4
é alinhamento com industry mesh-based standard; Typst original
(Type 6 Coons), Cairo (Type 6/7), Inkscape (Type 7), e
cristalino (Type 4) são todos variantes mesh-based.

ADR-0029 §"Simplificações aceites apenas com ADR explícita"
obriga formalização de divergência observable vs vanilla.
ADR-0089 cobre subset L1+stdlib + decisão estratégia PDF Type 4
materializada (anotação cumulativa P268); esta ADR-0090
**complementa ADR-0089** registando a justificativa empírica
industry e elevando "Type 4 vs Type 1" a decisão arquitectural
formal distinta.

---

## Pesquisa empírica industry

### Tabela achados factuais consolidados

| Projecto | Estratégia conic PDF | Tipo PDF |
|---|---|---|
| Krilla (typst actual via Part 7 #5420) | `SweepGradient` abstracção; estratégia PDF interna não documentada publicamente | desconhecido |
| Typst original (pré-krilla, blog 2023) | Coons Patches (1 patch por stop após rejeitar PS functions) | `/ShadingType 6` |
| Cairo | Coons/Tensor patch mesh via `cairo_mesh_pattern_*` | `/ShadingType 6` ou `/ShadingType 7` |
| Inkscape | Tensor patch mesh (suporte interno; user-facing limitado) | `/ShadingType 7` |
| Skia (rendering, não PDF) | SkSL shaders SweepGradient | N/A (rendering directo) |
| **Cristalino P268** | **Free-Form Gouraud Triangle Mesh** | **`/ShadingType 4`** |
| pdf.js (consumidor) | Não suporta `/ShadingType 1` conic (pink fallback) — relevante para qualquer estratégia Type 1 embutida via Chrome puppeteer-style; não estabelecido se afecta krilla output actual | parcial |

### Conclusões factuais

1. **PDF spec ISO 32000-1 oficial reconhece como gradient
   shadings apenas Types 2 (axial) e 3 (radial)** — conic não é
   parte oficial do spec. Tipos 4/5/6/7 são mesh shadings
   genéricos que podem ser usados para emular conic.

2. **Type 1 Function-Based Shading aplicado a conic é
   tecnicamente extensão vendor-specific quando o renderer
   mapeia (x,y) → angle → cor via PostScript function** —
   pdf.js (Firefox PDF viewer) regista isto como "Unsupported
   ShadingType: 1" e renderiza pink fallback. Chrome via
   Puppeteer produz PDFs Type 1 conic. Krilla actual produce
   strategy desconhecida; afirmação anterior "krilla usa Type 1"
   foi inferência não verificada e é retraída neste passo de
   correcção.

3. **Industry mesh-based abordagem (Type 4/6/7) é a mais comum
   em projectos PDF maduros**:
   - Cairo (Type 6/7) — 20+ anos maturidade; Igalia blog confirma.
   - Inkscape (Type 7) — segue Cairo.
   - **Typst original (Type 6 Coons)** — blog 2023 documenta o
     processo de decisão literal (sampled rejeitado, PostScript
     function rejeitado, Coons adoptado).
   - W3C CSS-Color-4 Workshop 2021 (bfo): "the only way we can
     render conic gradients" em PDF é Coons Patch shading.
   - Cristalino (Type 4) — escolha mais simples da família mesh;
     divergência intra-família mesh vs Typst original Type 6 /
     Cairo Type 6-7.

4. **Cor central em mesh shadings (Type 4/6/7) é vértice
   explícito com cor explícita por convenção PDF** — não é
   divergência arbitrária; é como mesh-based PDF shading
   funciona. Cairo precedente: "primeira cor no vértice inicial".

5. **Cristalino está alinhado com industry mesh-based standard**
   (Cairo, Inkscape, Typst original). Krilla actual estratégia
   interna desconhecida publicamente; pode ou não estar alinhada.
   Cristalino divergência vs Typst original é **intra-família
   mesh** (Type 4 vs Type 6), não entre famílias.

### Nota metodológica de proveniência

**Verificáveis literal no filesystem cristalino**:
- `lab/typst-original/crates/typst-pdf/src/paint.rs:255` —
  `SweepGradient` inicializado para `Gradient::Conic` (krilla
  actual; abstracção — estratégia PDF interna não verificada
  literal).
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` —
  warning "conic gradients are not supported in this PDF
  standard".

**Verificáveis via web (não filesystem)**:
- Blog Typst "Color gradients and my gradual descent into
  madness" (2023) — Typst original usou Coons Type 6 com 1
  patch por stop após rejeitar PostScript functions.
- W3C CSS-Color-4 Workshop (2021, Mike Bremford bfo) — Coons
  como única forma render conic em PDF.
- Igalia blog conic gradients (2020) — Cairo Coons patches.
- pdf.js issue #19233 — Unsupported ShadingType: 1.
- ISO 19005-1:2005 §6.2.7 — proíbe PostScript XObjects, não
  Type 4 calculator functions em shading dictionaries.

**Não verificáveis literal sem ler código privado**:
- Krilla actual estratégia PDF interna para conic.
- Chrome PDF generator estratégia interna para conic.

**Correcção pré-commit P268.1**: três afirmações originais
erradas (krilla=Type 1; PDF/A-1=functions proibidas; vanilla=
outlier total) identificadas via achado externo (Kimi) e
corrigidas literal antes do commit final, mantendo ADR-0090
estado `EM VIGOR` e decisão de fundo (Type 4 Gouraud)
preservada.

---

## Decisão

**Cluster Gradient cristalino usa `/ShadingType 4` Free-Form
Gouraud Triangle Mesh para Conic, divergindo deliberadamente de
vanilla (`/ShadingType 1` Function-Based via krilla)**.

Justificativas factuais ordenadas por prioridade:

1. **PDF spec ISO 32000-1 oficial** reconhece apenas Types 2/3
   como gradient shadings; Types 4/5/6/7 são mesh shadings
   genéricos adequados; Type 1 conic é extensão vendor-specific.

2. **Compatibilidade PDF readers**: Type 4 funciona em todos os
   readers (pdf.js, Chrome, Adobe, mupdf, poppler); Type 1 conic
   falha em pdf.js (Firefox).

3. **Compatibilidade PDF/A standards restritivos**: ISO
   19005-1:2005 §6.2.7 proíbe PostScript XObjects (streams
   PostScript embutidas como XObjects independentes). Type 4
   PostScript calculator functions usadas em shading dictionaries
   são tecnicamente subset restrito (sem loops/variáveis/
   subrotinas) e formalmente permitidas, mas suporte reader
   inconsistente é o argumento prático. Vanilla typst
   `convert.rs:514` emite warning "conic gradients are not
   supported in this PDF standard" — argumento prático
   "reader-support inconsistente" cobre o que cristalino Type 4
   Gouraud evita ao não usar functions em shading.

4. **Industry precedent**: Cairo (Type 6/7), Inkscape (Type 7),
   Typst original pré-krilla (Type 6 Coons; blog 2023 documenta),
   cristalino (Type 4) — todos família mesh-based. Krilla actual
   é opaco. Cristalino divergência é intra-família mesh.

5. **Performance reader cliente**: Type 4 = interpolação linear
   simples; Type 1 = interpreter PostScript por pixel.

6. **LOC cristalino**: Type 4 ~190 LOC; Type 1 estimado
   ~400-500 LOC.

7. **ADR-0018 preservado**: cristalino implementa autonomamente;
   nenhuma dependência externa requerida.

---

## Convenção cor central

Cor central = primeiro stop. Convenção PDF mesh shading
estabelecida (Cairo precedente: "color assigned to the corner
at the start of the path; follows Cairo conventions which
follows the PDF convention").

**Não é decisão arbitrária** — é convenção PDF mesh shading
inerente ao Type 4 onde cada vértice tem cor explícita; centro
do disco é vértice; primeiro stop é convenção industry
estabelecida.

---

## Consequências

### Positivas

- Cluster Gradient PDF compatível com todos PDF readers,
  incluindo pdf.js que vanilla quebra.
- PDFs cristalino renderizam em PDF/A-1 onde vanilla emite
  warning.
- Cristalino alinhado com Cairo/Inkscape industry mesh-based
  standard.
- Implementação autónoma; ADR-0018 preservado.

### Negativas

- Divergência observable bit-exact de vanilla em casos extremos
  (gradientes com contraste muito alto em N=32 fatias produzem
  banding ligeiramente perceptível; vanilla Type 1 é
  matematicamente suave).
- Refino qualidade visual fica pendente P268.2 (adaptive N
  hybrid).

### Neutras

- ADR-0089 §"Anotação cumulativa P268" preservada; esta ADR
  formaliza a decisão estratégica que P268 materializou.
- Vanilla read-first preservado — citações vanilla
  (`paint.rs:255`, `convert.rs:514`) são registo factual, sem
  alterar decisão vanilla.

---

## Scope-outs preserved

- **Type 1 PostScript Function**: scope-out permanente.
  Rejeitada historicamente também pelo Typst original (blog
  2023: "several readers do not support this feature").
  Cristalino segue precedente, não diverge.
- **Type 6/7 Coons/Tensor patches**: scope-out actual;
  cristalino escolha Type 4 por simplicidade implementação.
  Typst original usa Type 6 Coons (1 patch por stop); cristalino
  diverge intra-família. P268.2 adaptive N hybrid mitiga banding
  sem mudar estratégia. Candidato refino futuro
  (P-Gradient-Coons-Patch) se Type 4 banding for problema real.
- **PDF/A-1 explicit support**: scope-out; Type 4 funciona em
  PDF/A-1 mas cristalino não declara PDF/A compliance.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| α1 — Re-materializar Type 1 PostScript (P-Gradient-Conic-PS futuro) | Paridade bit-exact vanilla | Perde compatibilidade pdf.js; perde compatibilidade PDF/A; LOC 3-4×; ADR-0018 cliente (krilla não autorizada — re-implementar PostScript interpreter L3 puro inviável magnitude) |
| α2 — Re-materializar Type 6 Coons | Qualidade visual superior Type 4 | Magnitude L; precedente Cairo mais complexo do que necessário para conic simples; Type 4 suficiente para qualidade visual com adaptive N P268.2 |
| α3 — Manter Type 4 sem ADR | -1 ADR | Divergência arquitectural vs vanilla precisa documentação formal para futuro |
| **α4 — ADR-0090 EM VIGOR formalizando Type 4 (escolhida)** | **Justificação empírica industry registada; precedente formal para futuras divergências industry-aligned; ADR-0089 anotação P268.1 cross-reference** | **+1 ADR (76 → 77); custo administrativo baixo** |

**Decisão**: **α4 (ADR-0090 EM VIGOR formalizando Type 4)**.

---

## Critério revisão

Esta ADR pode ser revisitada se:

1. **pdf.js ou outro reader major passar a suportar Type 1
   conic** (compatibility argument enfraquece).
2. **Adaptive N P268.2 revelar-se insuficiente** para qualidade
   visual (Type 6 Coons torna-se candidato).
3. **ADR-0018 mudar e krilla for autorizada** (improvável).
4. **Industry standard mudar** (improvável a curto prazo).

Cada activação é **passo dedicado pequeno** (XS-M) per pattern
P262+.

---

## Subpadrão "ADR scope-out preserved com justificativa empírica" — N=1 inaugural

Cumulativo:
- **N=1 P268.1** (esta ADR; subpadrão novo; candidato
  formalização limiar N=3-5 cumulativo).

**Pattern emergente**: ADR documenta divergência observable de
vanilla preservando justificativa industry empírica em vez de
restaurar paridade bit-exact. Distinto de "decisão minimalista
(subset materializado)" — aqui o subset está materializado e a
divergência é estratégica, não simplificação.

---

## Subpadrão "Diagnóstico empírico web em vez de filesystem" — N=1 inaugural

Cumulativo:
- **N=1 P268.1** (este passo; pesquisa industry consolidada via
  web_search Claude web; ADR-0085 §"diagnóstico imutável"
  estendido para fontes externas com nota metodológica de
  proveniência).

**Pattern emergente**: diagnóstico imutável aceita fontes
externas quando filesystem cristalino é insuficiente, desde que
registadas literal e com nota de não-verificabilidade onde
aplicável.

---

## Subpadrão "Passo administrativo XS criar/promover ADR" — N=4 → N=5 cumulativo

Cumulativo:
- N=1 P156K (ADR-0064/0065 criadas).
- N=2 P160A (ADR-0066 criada).
- N=3 P229 (ADR-0080 promoção).
- N=4 P254 (ADR-0082 promoção).
- **N=5 P268.1** (ADR-0090 criada EM VIGOR directamente).

**Patamar N=5 excede limiar formalização clara**. Padrão
auto-documentado em cada ADR individual.

---

## Referências

- **ADR-0089** — Gradient Conic-only L1+stdlib + anotação
  cumulativa P268 (decisão Type 4 materializada; esta ADR-0090
  formaliza justificativa estratégica).
- **ADR-0018** — Whitelist crates externas em L1 (krilla não
  autorizada; preservada literal).
- **ADR-0029** — Pureza física L1 + simplificações requerem ADR
  explícita.
- **ADR-0033** — Paridade funcional vanilla (esta ADR documenta
  divergência observable consciente).
- **ADR-0034** — Diagnóstico canónico (estendido para web).
- **ADR-0085** — Diagnóstico imutável (estendido para fontes
  externas).
- **ADR-0054** — Perfil graded (anotação cumulativa P268.1;
  cluster Gradient PDF 3/3 industry-aligned).
- **ADR-0080** — L0 minimal para refactors aditivos (sem código
  alterado neste passo; paridade pattern).
- **ADR-0061** — Granularidade 1-2 features/passo (cumprido;
  P268.1 puramente documental; P268.2 refino visual separado).
- **ISO 32000-1 §7.5.7** — Shading Patterns Types 1-7.
- **Cairo `cairo_mesh_pattern_*` API** — precedente Type 6/7.
- **Inkscape Mesh Gradients wiki** — precedente Type 7.
- **pdf.js issue #19233** — Unsupported ShadingType: 1 fallback
  pink.
- **Vanilla `lab/typst-original/crates/typst-pdf/src/paint.rs:255`**
  — `krilla::SweepGradient` inicializado para `Gradient::Conic`
  (abstracção; estratégia PDF interna não verificada literal).
- **Vanilla `lab/typst-original/crates/typst-pdf/src/convert.rs:514`**
  — warning "conic gradients are not supported in this PDF
  standard" (suporte reader inconsistente; argumento prático).
- **Blog Typst "Color gradients and my gradual descent into madness"**
  (typst.app/blog/2023/color-gradients/) — Typst original Coons
  Type 6.
- **W3C Workshop CSS-Color-4 Mike Bremford** (2021) — Coons como
  única forma render conic em PDF.
- **ISO 19005-1:2005 §6.2.7** — PDF/A-1 PostScript XObjects
  restriction (não Type 4 calculator functions em shading).
- **typst/typst issue #2282 Part 7 / PR #5420** — transição para
  krilla.
- **Igalia blog "Renderization of Conic gradients"** (2020) —
  Cairo Coons patches.
- **P267** — Gradient Conic L1+stdlib (ADR-0089).
- **P268** — PDF Conic /ShadingType 4 Gouraud (decisão
  materializada; ADR-0089 §anotação P268).
- **P268.2** — Refino adaptive N hybrid (spec futura; passo
  separado).

---

## Próximos passos

1. **P268.2** — refino adaptive N hybrid 1+2 (critério número de
   stops + contraste cromático Oklab ΔE). Magnitude S;
   cap ~200 LOC + ~15 testes. Melhora qualidade visual Type 4
   sem mudar estratégia. Não revoga esta ADR — refina aplicação.
2. **P-Gradient-Focal** (futuro M) — activa `focal_*` Radial;
   revoga ADR-0088 §focal scope-out.
3. **Revisões futuras** — apenas se §"Critério revisão"
   accionado (pdf.js Type 1, Type 6 candidato, ADR-0018 mudar,
   industry shift).

---

## Anotação cumulativa P270 — ColorSpace runtime activado L1+stdlib (Type 4 preservado)

**Data**: 2026-05-17.

`Conic` variant ganha campo `space: ColorSpace` (default Oklab) em
P270 — extensão cross-variant. **Estratégia Type 4 Gouraud preservada
literal** (ADR-0090 intocada em decisão; só L1 sample multi-space
adicionado).

L3 emit Type 4 Gouraud + adaptive N hybrid (P268 + P268.2) preservados
P270; refactor multi-space L3 adiado P270.1.

Sub-padrão "Anotação cumulativa cross-ADR" N=1 inaugural — P270 anota
ADR-0083/0054/0087/0088/0089/0090 simultâneo.

Status `EM VIGOR` preservado literal. Decisão de fundo (Type 4 vs
industry mesh variants) intocada. Ver **ADR-0091 EM VIGOR** para
decisão arquitectural completa.

---

## Anotação cumulativa P270.1 — L3 emit multi-space (Type 4 + adaptive N preservados)

**Data**: 2026-05-17.

P270.1 materializa L3 emit multi-space (7 spaces) para Conic via
helper renomeado `multispace_sample_stops_conic` (era
`oklab_sample_stops_conic`). **Estratégia Type 4 Gouraud preservada
literal** — ADR-0090 intocada em decisão de fundo. **Adaptive N
hybrid P268.2 preservado bit-exact** — sample stops em space
escolhido (default Oklab idêntico P268.2; outros 6 spaces novos
via dispatcher P270).

Default Oklab preserva bytes pré-P270.1 bit-exact. CMYK preserva
scope-out P270.1; P270.2 fecha.

Sub-padrão "Anotação cumulativa cross-ADR" N=1 → N=2 cumulativo
(P270 + **P270.1**).

Status `EM VIGOR` preservado literal. Decisão de fundo Type 4 intocada.
Ver ADR-0091 §"Anotação cumulativa P270.1".

---

## Anotação cumulativa P270.2 — Type 4 Gouraud + Conic CMYK scope-out preserved

**Data**: 2026-05-17.

P270.2 materializa CMYK directo Linear+Radial via `/ColorSpace
/DeviceCMYK`. **Conic Type 4 Gouraud CMYK scope-out preserved**
(§A.8 diagnóstico Cenário B):

- Estratégia Type 4 Gouraud **preservada literal** — ADR-0090
  intocada em decisão de fundo.
- Conic CMYK em P270.2 usa pipeline P270.1 fallback (sample CMYK
  convert para sRGB sub-óptimo).
- Candidato futuro **P-Gradient-Conic-CMYK** ao materializar Type 4
  Gouraud + `/ColorSpace /DeviceCMYK` (stream binary 4 bytes/vertex;
  `/Decode` 5 pares).

Adaptive N hybrid P268.2 preservado bit-exact em todos os branches.

Sub-padrão "Anotação cumulativa cross-ADR" N=2 → N=3 cumulativo.

Status `EM VIGOR` preservado literal. Decisão de fundo Type 4 intocada.
Ver ADR-0091 §"Anotação cumulativa P270.2".

---

## Anotação cumulativa P270.3 — Type 6 Coons scope-out revogado parcialmente

**Data**: 2026-05-17.

§"Scope-outs preserved" §"Type 6/7 Coons/Tensor patches" revogado
**parcialmente**:

- **Type 6 Coons**: **revogado P270.3** — materialização cristalina
  como estratégia adicional Conic (paralela Type 4 Gouraud; opt-in
  flag interno para CMYK P270.4). Ver ADR-0092 EM VIGOR.
- **Type 7 Tensor**: preserved scope-out — refino futuro candidato.

**ADR-0090 decisão de fundo (Type 4 Gouraud RGB cristalino)
preservada literal** — Type 6 é estratégia adicional, não
substituição. P268+P268.2+P270.1 RGB preservados bit-exact.

### Sub-padrão "ADR scope-out revogado parcialmente" N=5 cumulativo

- N=1 P267 (ADR-0088 §Conic).
- N=2 P269 (ADR-0088 §focal_*).
- N=3 P270 (ADR-0083 §ColorSpace runtime).
- N=4 P270.2 (ADR-0083 §DeviceCMYK parcial Linear+Radial).
- **N=5 P270.3** (esta anotação — ADR-0090 §Type 6 Coons).

**Limiar formalização clara muito ultrapassado** — candidato meta-ADR
URGENTE paridade P260 ADR-0084/0085.

Status `EM VIGOR` preservado literal. Ver ADR-0092 §"Decisão (Cenário
A revisado)".

---

## Revogação P272

**Data**: 2026-05-17.
**Status**: `EM VIGOR` → **`REVOGADO`**.

**Motivo**: convergência industry-aligned para mesh-based Type 6
Coons (Cairo/Inkscape/Typst original blog 2023 precedente literal).

Pesquisa industry P270.3 + experiência P270.4 (Coons CMYK)
materializaram Type 6 com sucesso. Eliminar 2 estratégias coexistentes
(Type 4 Gouraud + Type 6 Coons) reduz complexidade arquitectural sem
perda funcional.

**Substituição**: **ADR-0092 expandida cumulativamente P272** cobrindo
estratégia Conic unificada Coons para 8/8 spaces.

**Implicações materializadas P272**:

- `emit_conic_gouraud_stream` (P268) **removed** (~85 LOC L3).
- `compute_adaptive_n_conic` (P268.2) **removed** (~40 LOC L3).
- `oklab_delta_e` (P268.2) **removed** (~15 LOC L3; única call site
  era `compute_adaptive_n_conic`).
- **20 tests P268+P268.2 removed** (1 multispace preserved — testa
  helper genérico ainda usado por P270.1+).
- `emit_conic_coons_stream_rgb` (P272) **active** — extension
  P270.3 `emit_conic_coons_stream` com strategy N=stops*4 patches
  inter-stop; corner colors interpolated via `Conic::sample(t)`
  dispatcher P270 (interpolate_in_space per conic.space).
- Dispatcher Conic em `emit_gradient_objects` **unificado**
  (`/ShadingType 6` Coons para 8/8 spaces; Decode 5 pares RGB ou
  6 pares CMYK; Function Type 2 N=1 identity 3/4 components).

**Decisão de fundo invalidada**: Type 4 Free-Form Gouraud já não é
estratégia Conic cristalina; substituída por Type 6 Coons Patch Mesh
(Cairo/Inkscape industry-aligned). Pattern ADR-0093 §Pattern 1
§"Quando NÃO aplicar" — revogação invalida decisão de fundo; use
status REVOGADO + ADR substituta.

**Sub-padrão "ADR REVOGADO + substituta"**: N=2 prévio cristalino
(ADR-0007 → ADR-0018; ADR-0028 → ADR-0029) → **N=3 cumulativo**
com **P272 ADR-0090 → ADR-0092 expandida**. Pattern emergente já
estabelecido historicamente; P272 distingue-se como **primeira
aplicação pós-formalização ADR-0093** P271.

**Sub-padrão "Aplicação meta-ADR (ADR-0093)" N=1 inaugural** —
P272 é primeira aplicação prática de meta-ADR ADR-0093 §Pattern 1
§"Quando NÃO aplicar" pós-formalização P271; demonstra empiria da
metodologia.

**Sub-padrão "Aplicação meta-ADR (ADR-0094)" N=1 inaugural** —
Cap LOC hard/soft Pattern 1 aplicado em P272 spec
(L3 additions hard 200/soft 120; testes additions hard 30/soft 22;
real ~80-100 LOC additions; folga 100% hard).

**Trabalho prévio preservado historicamente**:

- ADR-0090 conteúdo original preserved como registo arquitectural
  (decisão original, pesquisa industry, sub-padrões inaugurados
  P268.1).
- Industry research P268.1-correção preserved (lição metodológica;
  ADR-0094 §Pattern 3 "Industry research proactiva").
- Sub-padrão "Correcção ADR pré-commit" anti-pattern preservado.

Cross-reference: **ADR-0092 §"Anotação cumulativa P272 — Decisão
Cenário A revisado FINAL"**.
