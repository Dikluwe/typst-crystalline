# Passo 226 — ADR meta documental "L0 minimal para refactors" N=7 + diagnóstico amplo Fase 5 Layout "completar Layout" (4 categorias A+B+C+D) + ADR-0079 PROPOSTO

**Série**: 226 (décimo-segundo sub-passo Layout pós-M9c;
**primeiro passo Fase 5 Layout candidata diagnóstico**;
abre série β "completar Layout" cumulativa per decisão
humana literal — escopo Tudo A+B+C+D incluindo reabertura
de decisões arquitecturais; paridade estrutural P215 para
Fase 3 com agregação Caminho 1 + 2 P225 §8).
**Marco**: nenhum (décimo-quarto passo pós-M9c; primeiro
Fase 5 — paridade estrutural P156B para Fase 1 Layout;
pattern "diagnóstico amplo + ADR PROPOSTO + roadmap"
N=4 → 5 cumulativo).
**Tipo**: passo administrativo + diagnóstico amplo
agregado num único passo per decisão humana literal.
ADR meta documental nova (formaliza pattern emergente
N=7) + diagnóstico Fase 5 Layout + ADR-0079 PROPOSTO
nova (column flow Fase 5 ou similar conforme estrutura
decidida em §1).
**Magnitude**: M+ (~3-4h cumulativo; XS ADR meta + M
diagnóstico amplo).
**Pré-condição**: P225 concluído (Fase 4 Layout candidata
fechada formalmente; série α "terminar Layout" cumprida;
2039 tests verdes; 0 violations; cobertura Layout 89%
real Opção γ refresh; Layout em estado terminal estructural
reconhecido oficialmente); humano fixou em duas decisões
literais P225 pós-discussão:
- **Decisão H1**: "Completar Layout" = Tudo A+B+C+D
  (incluir runtime queries; reabre ADR-0066 +
  arquitectura single-pass).
- **Decisão H2**: P226 = ADR meta documental + diagnóstico
  Fase 5 agregado (passo administrativo + diagnóstico
  amplo único).
- Pattern "L0 minimal para refactors" N=7 cumulativo
  (P217+P218+P219+P220+P222+P223+P224 todos Opção γ);
  promoção formal candidato sólido P221 §8 Caminho 4
  diferido até P225 §8 Caminho 1.
**Output**: 1 ficheiro relatório curto + 1 ficheiro ADR
novo (ADR-0067 meta documental ou número disponível) +
1 ficheiro ADR novo (ADR-0079 Fase 5 Layout PROPOSTO) +
1 ficheiro diagnóstico novo (`diagnostico-layout-fase-5-completar.md`)
+ blueprint anotação §3.0quaterdecies (marca abertura série
β) + inventário 148 anotação cumulativa (sem reclassificação).

---

## §1 Trabalho

P226 cumpre **3 trabalhos agregados** per Decisão H2:

### Trabalho 1 — ADR meta documental "L0 minimal para refactors" N=7

Pattern emergente N=7 cumulativo (P217+P218+P219+P220+P222
+P223+P224 todos Opção γ "sem extensão L0 formal para
refactors") consolida-se como **prática estabelecida pós-M9c**.
P224 divergência consciente vs spec C6 Opção α reforçou
em vez de suspender o pattern. **N≥6 patamar formalização
sólido** atingido pós-P223; P224 confirmou. **Promoção
formal a ADR meta documental** justificada per critérios:
- N=7 patamar empírico **muito sólido** (limiar formalização
  N=3-4 amplamente ultrapassado).
- 7 sub-passos sequenciais sem reformulação.
- Divergência consciente face a spec C6 P224 documenta
  decisão empírica formal preservada.
- Múltiplas séries cumulativas pós-M9c (Fase 3 sub-fase b
  + Fase 4 candidata).

**ADR meta documental nova** (paridade estrutural ADR-0065
"inventariar primeiro" + ADR-0034 "diagnóstico obrigatório"):
- **Número**: ADR-0067 (próximo disponível após ADR-0066;
  audit em C1 para confirmar — ADR-0078 column flow já
  IMPLEMENTADO; números 0067-0077 disponíveis).
- **Título**: "L0 minimal para refactors aditivos pós-M9c".
- **Status**: `PROPOSTO` (paridade ADR-0034; pode transitar
  imediatamente a `EM VIGOR` se humano priorizar — passo
  administrativo separado).
- **Conteúdo**: regra emergente codificada + 7 aplicações
  cumulativas listadas + critério de aceitação ("refactor
  aditivo a variant existente OU stdlib expose OU stdlib
  aditiva trivial → L0 não tocado por defeito; documentação
  inline-doc no código").

### Trabalho 2 — Diagnóstico amplo Fase 5 Layout "completar Layout" (4 categorias A+B+C+D)

Paridade estrutural P215 (diagnóstico Fase 3 + ADR-0078
PROPOSTO). P226 cobre 4 categorias per Decisão H1:

#### Categoria A — Cosméticos (sem reabrir decisões)

Atributos visuais não-estruturais; refinos paridade
P156G+H+I scope-outs:
- **A.1** — `stroke` Grid (paridade vanilla `GridStroke`;
  inherits Table per delegate). Magnitude S+ a M.
- **A.2** — `fill` Grid (paridade vanilla `GridFill`; inherits
  Table). Magnitude S+ a M.
- **A.3** — `stroke`/`fill` em GridCell per-cell. Magnitude
  M (precisa precedence rules).
- Provável outros refinos cosméticos: `outset`/`radius`/`clip`
  para Block/Boxed (P156G+H scope-outs reabertos).

#### Categoria B — Algorítmicos isolados (sem reabrir decisões)

Refinos algorítmicos sem reabrir Opção B P219 ou ADR-0066:
- **B.1** — DEBT-34d Auto track sizing ("Auto não encolhe
  antes de matar fr"). Magnitude M (algorítmico isolado;
  refactor passo 3 do `layout_grid`).
- **B.2** — Consumer geometric integration P224.C
  (`place_cells` algorítmico → Layouter geometric). Magnitude
  M (integra placement com `layout_grid`).
- **B.3** — Per-cell GridCell atributos (`align`/`inset`/
  `breakable`) — paridade P157B subset. Magnitude M.

#### Categoria C — Estruturais reabrindo decisões (maior risco)

Reabertura de decisões arquitecturais maiores:
- **C.1** — Place `float` real (flow contorna). **Reabre
  Opção B P219 graded**; precisa multi-region flow real ou
  pelo menos parcial (flow secundário para topo/fundo).
  Magnitude L+.
- **C.2** — Opção A multi-region completa para columns/
  colbreak real. **Reabre P216B `Regions { current }` minimal**
  + DEBT-56 ENCERRADA (P221) — semântica de "reabertura
  pós-fecho" precisa nota arquitectural explícita.
  Magnitude L+ a XL.

#### Categoria D — Runtime queries (reabertura arquitectural maior)

Reabre ADR-0066 PROPOSTO + arquitectura single-pass:
- **D.1** — `state(key, init)` runtime mutable (P160B
  candidato existente). Magnitude M.
- **D.2** — `metadata(value)` attaching (P160C candidato).
  Magnitude S+.
- **D.3** — `here()` / `locate()` location-aware (P160D
  candidato). Magnitude M.
- **D.4** — `query(target)` runtime introspection (P160E
  candidato). Magnitude M+.
- **D.5** — `position(target)` location-aware (P160F
  candidato).
- **Promoção ADR-0066** PROPOSTO → IMPLEMENTADO após D.1
  materializa (per ADR-0066 §"Plano promoção" 3 condições).

#### Roadmap total e magnitude cumulativa

| Categoria | Sub-passos | Magnitude cumulativa | Reabertura |
|-----------|------------|----------------------|------------|
| A | A.1+A.2+A.3 | M-L (~4-6h) | não |
| B | B.1+B.2+B.3 | M+ a L (~6-9h) | não |
| C | C.1+C.2 | L+ a XL (~10-20h) | **sim** (Opção B P219; P216B; DEBT-56) |
| D | D.1+D.2+D.3+D.4+D.5 + promoção | L+ a XL (~10-15h) | **sim** (ADR-0066; single-pass) |
| **Total** | **~13-14 sub-passos** | **L+ a XL (~30-50h cumulativo)** | sim |

### Trabalho 3 — ADR-0079 Fase 5 Layout PROPOSTO

Paridade estrutural ADR-0078 column flow PROPOSTO em P215.
**Nome provisório**: "Layout Fase 5 roadmap — completar
Layout (Tudo A+B+C+D)".

Cobertura:
- Decisão arquitectural macro: completar Layout cobre 4
  categorias cumulativas.
- Reaberturas de decisões registadas explicitamente
  (Opção B P219; P216B; DEBT-56 fechada; ADR-0066 PROPOSTO).
- Roadmap 13-14 sub-passos identificados mas NÃO reservados
  per política P158.
- Trade-off cumulativo (~30-50h L+ a XL) registado.
- Critério de promoção PROPOSTO → IMPLEMENTADO: completar
  Layout cumprido em N=13-14 sub-passos OU **decisão
  humana de scope-out parcial formal** (e.g. categorias
  A+B materializadas; C+D scope-out formal).

**Decisão arquitectural central — 5 decisões fixadas**:

### Decisão 1 — ADR meta única vs múltipla

P226 promove APENAS pattern "L0 minimal para refactors"
N=7 a ADR meta documental. Outros patterns emergentes
(Field semantic adiada N=5; div-N N=2; encerramento Fase
pós-M9c N=2) permanecem registados em §3.0terdecies P225
sem promoção formal **em P226** (política P158).

Promoção dos restantes a ADRs meta separadas fica como
candidato futuro (sub-passos administrativos XS dedicados
se humano priorizar).

**Decisão fixada — Opção α (ADR meta única L0 minimal N=7)**.

### Decisão 2 — Estrutura diagnóstico Fase 5

Paridade pattern P215 (Layout Fase 3 diagnóstico + ADR
PROPOSTO + roadmap sub-passos). Diagnóstico em ficheiro
novo `diagnostico-layout-fase-5-completar.md` cobrindo 4
categorias A+B+C+D com matriz de dependências entre
sub-passos.

**Decisão fixada — paridade P215 estrutura**.

### Decisão 3 — ADR-0079 Fase 5 nova vs anotação ADR-0061

ADR-0061 está IMPLEMENTADO desde P221. Anotar reabertura
de decisões arquitecturais em ADR já encerrada é
estruturalmente incorrecto.

**3 opções consideradas**:

| Opção | Acção | Trade-off |
|-------|-------|-----------|
| **α** | ADR-0079 Fase 5 Layout PROPOSTO nova | Paridade ADR-0078 column flow; estruturalmente correcto |
| β | Anotação ADR-0061 IMPLEMENTADO + nota de reabertura | Viola semantic "IMPLEMENTADO" |
| γ | Reabrir ADR-0061 IMPLEMENTADO → PROPOSTO retroactiva | Viola política de não-retroactividade |

**Decisão fixada — Opção α (ADR-0079 nova)** porque
estruturalmente correcta + paridade ADR-0078 + preserva
ADR-0061 IMPLEMENTADO histórico.

### Decisão 4 — Política "sem novas reservas" P158 preservada

P226 identifica 13-14 sub-passos mas **NÃO reserva nenhum**.
ADR-0079 PROPOSTO documenta roadmap; sub-passos
materialização ficam abertos para decisão humana
caso-a-caso.

**Decisão fixada — preservar P158 literal**: roadmap
identificado mas não-reservado.

### Decisão 5 — Magnitude P226 + atomização interna

P226 cumpre 3 trabalhos agregados (ADR meta + diagnóstico
amplo + ADR-0079 PROPOSTO). Atomização interna em
cláusulas dedicadas:
- C2 — ADR meta L0 minimal N=7 (XS).
- C3-C6 — Diagnóstico amplo 4 categorias A/B/C/D (M).
- C7 — ADR-0079 PROPOSTO (S).

Magnitude cumulativa: **M+ (~3-4h)**. Paridade P215
diagnóstico Fase 3 (M~2h real); P226 ligeiramente mais
amplo por 4 categorias vs 1 área columns.

**Decisão fixada — atomização interna A/B/C/D + ADR meta
+ ADR-0079; spec único agregado**.

Reuso de dados (sem recolha nova):

- 7 aplicações cumulativas pattern "L0 minimal" P217-P224.
- Patterns secundários P225 §3.0terdecies (Field semantic
  adiada N=5; div-N N=2; encerramento Fase N=2).
- ADR-0066 §"Plano promoção" 3 condições não-satisfeitas
  (state, 2-pass, E2E observable).
- ADR-0078 estrutura precedente para ADR-0079.
- ADR-0061 IMPLEMENTADO + ADR-0078 IMPLEMENTADO + ADR-0066
  PROPOSTO histórico.
- DEBT-34d preservado per `P224.div-1`.
- DEBT-56 fechada P221.
- DEBT-37 §"Divergência" fechada P223.
- P156B inventário 148 §A.5 Layout baseline (Categorias
  A+B candidatos identificados desde 2026-04-25).
- P160 diagnóstico Introspection (Categoria D candidatos
  identificados desde 2026-04-26).
- P215 estrutura diagnóstico Fase 3 (precedente formal).

---

## §2 Cláusulas (10)

### C1 — Inventário pré-P226: confirmar estado factual

Auditoria empírica:

```
ls 00_nucleo/adr/ | wc -l
grep -c "^- \*\*ADR-00" 00_nucleo/adr/README.md
grep -n "ADR-007[0-9]" 00_nucleo/adr/README.md | head -5
grep -n "P224.div-1\|DEBT-34d" 00_nucleo/DEBT.md
```

Hipótese:
- **Total ADRs**: 65 actual (per ADR-0066 P160A documenta);
  pós-P221 transições 65 inalterado (+0; transições não
  criam ADRs).
- **Próximo ADR disponível**: ADR-0067 (slot 0067-0077 livre;
  audit confirmará).
- **ADR-0079 disponível**: confirmar slot livre (audit; per
  precedente "número próximo disponível" P160A ADR-0066).
- **ADR-0078 IMPLEMENTADO** (P221 transição) preservado.

Se divergência: registar `P226.div-1`.

**Audit de patterns N=7 reais**:
- P217 L0 não tocado: ✓.
- P218 L0 não tocado (spec C6 propôs Opção α; decisão
  empírica Opção γ): ✓.
- P219 L0 não tocado (spec C7 propôs Opção α; decisão
  empírica Opção γ): ✓.
- P220 L0 não tocado (decisão Opção γ): ✓.
- P222 L0 não tocado (decisão Opção γ pattern N=4 → 5): ✓.
- P223 L0 não tocado (decisão Opção γ pattern N=5 → 6): ✓.
- P224 L0 não tocado (spec C6 propôs Opção α; decisão
  empírica Opção γ pattern N=6 → 7): ✓.

**N=7 cumulativo confirmado**.

### C2 — ADR meta documental "L0 minimal para refactors"
N=7

Criar ficheiro novo:
`00_nucleo/adr/typst-adr-0067-l0-minimal-para-refactors.md`
(número per audit C1).

Estrutura paridade ADR-0034 / ADR-0065 (ADRs meta
documentais cristalinos):

```markdown
# ⚖️ ADR-0067: L0 minimal para refactors aditivos pós-M9c

**Status**: `PROPOSTO`
**Data**: 2026-05-13
**Validado**: 7 aplicações cumulativas pós-M9c
(P217+P218+P219+P220+P222+P223+P224; N=7 patamar empírico
sólido).

---

## Contexto

Entre P217 e P224 (cumulativamente 7 sub-passos pós-M9c
da série Layout Fase 3 sub-fase b + Fase 4 candidata),
emergiu prática empírica não-formalizada: refactors
aditivos a variants Content existentes ou novas stdlib
funcs aditivas NÃO actualizam L0 prompts em
`00_nucleo/prompts/`; em vez disso, documentam decisões
em inline-doc no código + footnotes em inventário 148 +
anotações em ADRs.

Vários passos (P218 spec C6 propôs Opção α "linha minimal
em tabela"; P219 spec C7 propôs Opção α "secção dedicada
retroactiva refinada"; P224 spec C6 propôs Opção α
"secção dedicada"; todos foram Opção γ na materialização)
divergiram conscientemente face às specs em favor da
prática empírica emergente.

Pattern N=7 atinge limiar formalização (N=3-4 mínimo)
amplamente ultrapassado.

## Decisão

**Refactors aditivos pós-M9c NÃO actualizam L0 prompts**
por defeito. Documentação fica em:
1. Inline-doc no código Rust (`/// ...` sobre fields,
   functions, variants).
2. Footnote em inventário 148
   (`typst-cobertura-vanilla-vs-cristalino.md`).
3. Anotação em ADR relevante (PROPOSTO ou IMPLEMENTADO).
4. Marca cirúrgica em blueprint §3.0... se for fecho de
   série/Fase.

## Escopo

"Refactor aditivo" significa qualquer dos seguintes:
- Variant Content novo (sem alterar variants existentes).
- Field novo a variant Content existente (refino aditivo
  paridade P223).
- Stdlib func nova aditiva (sem alterar stdlib funcs
  existentes).
- Stdlib func refinada com named args novos (sem alterar
  semantic de args existentes).
- Helper privado novo ou promoção de visibility (paridade
  P222 measure_content).
- Módulo L1 novo (paridade P224 `grid_placement.rs`).

"Refactor não-aditivo" requer L0 actualizada
(retroactividade explícita):
- Mudança de signature de função L0-documentada.
- Mudança de variant existente sem manter paridade
  observable.
- Refactor estrutural de tipo (e.g. enum → struct;
  Vec → Arc<[T]>).
- Novas regras de validação que mudem rejeição de input.

## 7 aplicações cumulativas (validação empírica)

| Passo | Refactor | L0 acção | Observação |
|-------|----------|----------|------------|
| P217 | Content::Columns variant novo | L0 não tocado | Decisão empírica nova |
| P218 | native_columns stdlib | L0 não tocado | Spec C6 propôs Opção α |
| P219 | Layouter arm refactor | L0 não tocado | Spec C7 propôs Opção α retroactiva |
| P220 | Content::Colbreak agregado | L0 não tocado | Convenção consolidada |
| P222 | native_measure stdlib + visibility | L0 não tocado | Pattern N=4 → 5 |
| P223 | Content::Place +float +clearance | L0 não tocado | Pattern N=5 → 6 |
| P224 | Content::Grid refino substantivo + 3 variants + módulo | L0 não tocado | **Divergência consciente vs spec C6 Opção α** |

## Consequências

**Positivas**:
- Reduz overhead documental de refactors aditivos
  frequentes.
- Mantém L0 como documentação semantic estável (não
  histórico de evolução).
- Acelera materialização pós-M9c (precedente N=7).
- Preserva ADR-0033 paridade observable (L0 documenta
  semantic; refactors aditivos preservam semantic).

**Negativas**:
- L0 não reflecte estado real cumulativo de variants/
  stdlib (que cresceram +6 stdlib funcs + 5 variants
  pós-M9c sem L0 actualização correspondente).
- Auditor externo precisa cruzar L0 + inventário 148 +
  ADRs para estado completo.

**Trade-off aceite**: documentação distribuída vs
overhead actualização cumulativa.

## Alternativas consideradas

### Alt A — Actualizar L0 em todos refactors
- Pro: L0 sempre reflecte estado real.
- Con: overhead inflacionário; ADR-0061 P221 tinha 10+
  variants novas sem L0 actualizada (pattern já estabelecido
  pré-formalização).
- **Rejeitada**: precedente empírico forte (N=7).

### Alt B — Actualizar L0 apenas para variants Content novas
- Pro: refinos aditivos a variants existentes não
  inflacionam.
- Con: P217+P220+P224 introduziram 5 variants Content
  novas sem L0 → pattern N=4 viola Alt B.
- **Rejeitada**: pattern empírico N=4 já viola.

## Cross-references

- P217 + P218 + P219 + P220 + P222 + P223 + P224 — 7
  aplicações cumulativas.
- P224 spec C6 — divergência consciente Opção α → γ.
- §3.0terdecies P225 — pattern N=7 registado pré-promoção
  formal.
- ADR-0033 — paridade observable preservada.
- ADR-0034 — diagnóstico obrigatório precedente metadocumental.
- ADR-0065 — inventariar primeiro precedente metadocumental.

## Promoção

ADR-0067 transita PROPOSTO → EM VIGOR quando:
- N=8+ aplicação cumulativa **sem decisão explícita
  contrária** (i.e., humano não fixou Opção α em sub-passo
  futuro).
- OU passo administrativo XS dedicado para promoção
  (humano fixa).

**Status PROPOSTO** — autorização arquitectural concedida
em princípio; promoção EM VIGOR em passo futuro.
```

**Magnitude isolada C2**: XS (~30min).

### C3 — Diagnóstico amplo Fase 5 — Categoria A (Cosméticos)

Criar ficheiro novo:
`00_nucleo/diagnosticos/diagnostico-layout-fase-5-completar.md`.

§"Categoria A — Cosméticos (sem reabrir decisões)":

3 sub-passos identificados não-reservados:
- **A.1 — `stroke` Grid + Table inheritance**: paridade
  vanilla `GridStroke`. Aditivo a `Content::Grid`. Layouter
  emite `FrameItem::Line` per cell border. Magnitude S+
  a M.
- **A.2 — `fill` Grid + Table**: paridade vanilla `GridFill`.
  Aditivo a `Content::Grid`. Layouter emite `FrameItem::Shape::Rect`
  com fill colour per cell background. Magnitude S+ a M.
- **A.3 — `stroke`/`fill` em GridCell per-cell**: paridade
  vanilla per-cell precedence. Magnitude M (precisa
  precedence rules).

Refinos cosméticos adicionais identificados:
- Block/Boxed `outset`/`radius`/`clip` (P156G+H scope-outs).
- Place per-cell alignment override.

Total Categoria A: **3-6 sub-passos** identificados não-reservados;
magnitude cumulativa **M-L (~4-6h)**.

Dependências cross-passo: A.3 depende de A.1 + A.2 (precedência
per-cell vs Grid-level).

### C4 — Diagnóstico amplo Fase 5 — Categoria B (Algorítmicos isolados)

§"Categoria B — Algorítmicos isolados (sem reabrir
decisões)":

3 sub-passos identificados não-reservados:
- **B.1 — DEBT-34d Auto track sizing**: "Auto não encolhe
  antes de matar fr" — refactor passo 3 do `layout_grid`.
  Algorítmico isolado em L1. Fecha DEBT-34d preservado per
  `P224.div-1`. Magnitude M.
- **B.2 — Consumer geometric integration P224.C**:
  `place_cells` algorítmico → Layouter geometric. Integra
  PlacedCell com `layout_grid`. Magnitude M.
- **B.3 — Per-cell GridCell atributos** (`align`/`inset`/
  `breakable`): paridade P157B subset. Magnitude M.

Total Categoria B: **3 sub-passos**; magnitude cumulativa
**M+ a L (~6-9h)**.

Dependências cross-passo: B.2 depende de B.1 (track sizing
afecta geometric); B.3 depende de B.2 (per-cell precisa
integration).

### C5 — Diagnóstico amplo Fase 5 — Categoria C (Estruturais reabrindo)

§"Categoria C — Estruturais reabrindo decisões (maior
risco)":

2 sub-passos identificados não-reservados; **alta
complexidade**:
- **C.1 — Place `float` real (flow contorna)**: reabre
  Opção B P219 graded. Precisa multi-region flow real ou
  parcial (flow secundário topo/fundo página). Reabertura
  registada explicitamente em ADR-0079 §"Reaberturas".
  Magnitude L+.
- **C.2 — Opção A multi-region completa** (columns/colbreak
  real flow entre colunas): **reabre P216B `Regions {
  current }` minimal** + DEBT-56 ENCERRADA (P221). Semantic
  "reabertura pós-fecho" precisa nota arquitectural
  explícita em ADR-0079. Magnitude L+ a XL.

Total Categoria C: **2 sub-passos**; magnitude cumulativa
**L+ a XL (~10-20h)**.

Dependências cross-passo: C.1 e C.2 ortogonais (Place
float pode ser implementado sem Opção A multi-region;
mas Opção A facilita Place float real).

**Reabertura de DEBT-56 fechada**:
- DEBT-56 fechada P221 (CLOSED via materialização Opção
  B graded).
- C.2 = materialização Opção A real (não graded).
- Decisão arquitectural: criar **DEBT-56b** novo (refino
  Opção A pós-fecho DEBT-56) ou anotar reabertura em
  DEBT-56 ENCERRADO.
- **Recomendação subjectiva diagnóstico**: DEBT-56b novo
  preserva semantic "DEBT-56 fechada literal" + abre
  rastreamento explícito reabertura.

### C6 — Diagnóstico amplo Fase 5 — Categoria D (Runtime queries)

§"Categoria D — Runtime queries (reabertura ADR-0066)":

5+1 sub-passos identificados não-reservados (P160B-F
candidatos pré-existentes desde 2026-04-26):
- **D.1 — `state(key, init)` runtime mutable** (P160B):
  primeira feature runtime queries genuína. Magnitude M.
  **Promoção ADR-0066 PROPOSTO → IMPLEMENTADO ocorre após
  D.1** (per ADR-0066 §"Plano promoção" 3 condições
  satisfeitas: state materializada; pipeline introspect
  extendido 2-pass; tests E2E observable).
- **D.2 — `metadata(value)` attaching** (P160C). Magnitude
  S+.
- **D.3 — `here()` / `locate()` location-aware** (P160D).
  Magnitude M.
- **D.4 — `query(target)` runtime introspection** (P160E).
  Magnitude M+.
- **D.5 — `position(target)` location-aware** (P160F).
  Magnitude S+.
- **D.6** (Bloco C cross-módulo continuação) — cross-document
  cite refs (depende multi-document pipeline; fora de
  Layout puro).

Total Categoria D: **5-6 sub-passos**; magnitude cumulativa
**L+ a XL (~10-15h)**.

Dependências cross-passo: D.1 desbloqueia D.2-D.5; D.6
ortogonal (cross-document).

Cobertura Introspection esperada pós-D.5: **17% → ~50%**
(per ADR-0066 §"Subset minimal" estimativa).

### C7 — ADR-0079 Fase 5 Layout PROPOSTO

Criar ficheiro novo:
`00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
(número per audit C1; provavelmente disponível pós-ADR-0078).

Estrutura paridade ADR-0078 (column flow Fase 3 roadmap):

```markdown
# ⚖️ ADR-0079: Layout Fase 5 roadmap — "completar Layout" (Tudo A+B+C+D)

**Status**: `PROPOSTO`
**Data**: 2026-05-13
**Validado**: diagnóstico amplo P226
(`diagnostico-layout-fase-5-completar.md`); decisão humana
literal P225 §8 "completar Layout" escopo A+B+C+D.

## Contexto

Layout pós-P225 está em estado terminal estructural pós-Fase
3+4 cumulativas (cobertura 89% real; 0 ausentes §A.5;
DEBT-56 fechada; DEBT-34e fechada; DEBT-37 §"Divergência"
fechada). Refinos remanescentes 4 categorias identificados
mas não-materializados.

Decisão humana literal P225 pós-discussão: **"completar
Layout" = Tudo A+B+C+D** incluindo reabertura ADR-0066 +
arquitectura single-pass.

## Decisão

Materializar 13-14 sub-passos cumulativos cobrindo 4
categorias A+B+C+D conforme diagnóstico amplo P226.
Roadmap identificado mas **NÃO reservado** per política
P158 — sub-passos materialização ficam abertos para
decisão humana caso-a-caso.

Reaberturas de decisões arquitecturais registadas
explicitamente:
- **Categoria C.1**: reabre Opção B P219 graded (Place
  float real).
- **Categoria C.2**: reabre P216B `Regions { current }`
  minimal + DEBT-56 ENCERRADA (CLOSED preservada literal;
  novo DEBT-56b criado para refino Opção A multi-region
  pós-fecho).
- **Categoria D**: reabre ADR-0066 PROPOSTO → IMPLEMENTADO
  via subset minimal P160B-F.

## Trade-off cumulativo

- Magnitude cumulativa: **L+ a XL (~30-50h em 13-14
  sub-passos)**.
- Cobertura Layout pós-completar: 89% → **100% literal**
  (todas 18 entradas §A.5 → impl puro ou impl⁺).
- Cobertura Introspection pós-D.5: 17% → ~50% bonus.
- Reaberturas arquitecturais 2-3 explícitas (C.1, C.2,
  D categoria).

## Critério de promoção

ADR-0079 transita PROPOSTO → IMPLEMENTADO quando:
- Todos 13-14 sub-passos identificados materializados, OU
- **Decisão humana de scope-out parcial formal** (e.g.,
  categorias A+B materializadas; C+D scope-out formal por
  trade-off magnitude/risco).

ADR-0079 transita PROPOSTO → REJEITADA se:
- Decisão humana literal "abandonar completar Layout".

## Cross-references

- ADR-0061 IMPLEMENTADO — Layout Fases 1+2+3 + Fase 4
  candidata.
- ADR-0078 IMPLEMENTADO — Column flow Opção B graded.
- ADR-0066 PROPOSTO — Introspection runtime adiada (D
  reabre via promoção).
- ADR-0054 — Perfil graded (A+B preservam; C+D promovem
  além graded).
- P156B + P215 — precedentes diagnósticos amplos Layout.
- P226 (este passo) — diagnóstico amplo Fase 5 +
  ADR-0079 PROPOSTO.
- Diagnóstico amplo:
  `diagnostico-layout-fase-5-completar.md`.

## Próximos passos

Sub-passos identificados:
- **A** (cosméticos): 3-6 sub-passos M-L cumulativos.
- **B** (algorítmicos isolados): 3 sub-passos M+ a L.
- **C** (estruturais reabrindo): 2 sub-passos L+ a XL.
- **D** (runtime queries): 5-6 sub-passos L+ a XL.

Decisão humana fixa ordem materialização caso-a-caso.
Pattern P156C-J + P217-P220 + P222-P224 sugere ordem
"baixo risco → alto risco" (A → B → C → D); mas
dependências cross-passo podem alterar (e.g., D.1 desbloqueia
D.2-D.5; B.2 facilita C.2).
```

**Magnitude isolada C7**: S (~30min).

### C8 — Verificação tests workspace

Critério: **2039 verdes preservados** (P226 é documental
puro; zero código tocado).

```
cargo test --workspace 2>&1 | tail -3
```

Erro tolerado: zero. Qualquer red indica problema externo
(não causado por P226).

### C9 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Sem hashes propagados — P226 só
edita/cria ficheiros documentais (ADRs novos + diagnóstico
novo + blueprint marca); não toca L0 prompts em
`00_nucleo/prompts/`.

"Nothing to fix" esperado.

### C10 — Blueprint §3.0quaterdecies marca + inventário 148
anotação cumulativa

**Blueprint §3.0quaterdecies** marca de actualização —
[P226] **Abertura série β "completar Layout" Fase 5
candidata (ADR-0067 + ADR-0079 PROPOSTAS + diagnóstico
amplo 4 categorias)** adicionada após §3.0terdecies (P225)
e antes de §3.1:

- Distinção qualitativa face a §3.0terdecies P225
  (encerramento Fase 4) vs §3.0quaterdecies P226 (abertura
  Fase 5 candidata).
- 2 ADRs novas PROPOSTAS (ADR-0067 meta + ADR-0079 Fase
  5 roadmap).
- 1 diagnóstico amplo novo (`diagnostico-layout-fase-5-completar.md`).
- 4 categorias A+B+C+D identificadas com 13-14 sub-passos
  potenciais.
- Reaberturas arquitecturais 2-3 registadas explicitamente.
- Política "sem novas reservas" P158 preservada.
- Pattern emergente "ADR meta + diagnóstico amplo agregado"
  N=1 inaugurado P226 (Decisão H2 humana).

**Inventário 148**: sem reclassificações em P226
(diagnóstico puro; não materializa código). Anotação
cumulativa em footnote ⁴⁷ provável (ou skip per política
minimalista).

**Distribuição ADRs pós-P226**:
- PROPOSTO: 11 → **13** (+2: ADR-0067 + ADR-0079).
- IMPLEMENTADO: 21 (preservado).
- Total: 65 → **67**.

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-226-relatorio.md`.

Estrutura (~8-10 KB; magnitude M+ justifica) com 8 §s:

- §1 O que foi feito (sumário 5-8 linhas).
- §2 Auditoria pré-P226 + audit pattern N=7 (C1).
- §3 ADR-0067 meta documental L0 minimal N=7 (C2).
- §4 Diagnóstico amplo Fase 5 — Categoria A + B (C3+C4).
- §5 Diagnóstico amplo Fase 5 — Categoria C + D (C5+C6).
- §6 ADR-0079 PROPOSTO + reaberturas registadas (C7).
- §7 Resultados verificação (C8+C9; tests + lint).
- §8 Blueprint marca + próximo sub-passo (C10; sub-passo
  decidido humano).

Código alterado: **zero** (passo documental puro).

Ficheiros canónicos editados/criados:
- **Novo**: `00_nucleo/adr/typst-adr-0067-l0-minimal-para-refactors.md`
  (ADR meta documental PROPOSTO).
- **Novo**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (Fase 5 Layout PROPOSTO).
- **Novo**: `00_nucleo/diagnosticos/diagnostico-layout-fase-5-completar.md`
  (diagnóstico amplo 4 categorias A+B+C+D).
- **Editado**: `00_nucleo/adr/README.md` (+ 2 entradas
  novas ADR-0067 + ADR-0079; distribuição PROPOSTO
  11 → 13).
- **Editado**: `00_nucleo/diagnosticos/blueprint-projecto.md`
  (§3.0quaterdecies marca abertura série β).
- **Possivelmente editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁴⁷ cumulativa; opcional per política minimalista).

**3 ficheiros novos**; resto editado.

---

## §4 Não-objectivos

- Materializar qualquer sub-passo A/B/C/D em P226 —
  P226 é diagnóstico puro; materialização fica para
  sub-passos futuros.
- Promover ADR-0067 PROPOSTO → EM VIGOR — paridade
  ADR-0034/0065 PROPOSTO inicial; promoção a EM VIGOR fica
  para passo administrativo XS dedicado (se humano
  priorizar).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — paridade
  ADR-0078 que transitou só após série materialização
  completa P217-P220 + P221 encerramento.
- Reabrir DEBT-56 ENCERRADA P221 — preservada literal;
  reabertura via novo DEBT-56b se C.2 materializar (decisão
  diferida ao próprio C.2).
- Reabrir DEBT-34d preservado per `P224.div-1` — preservado
  literal; fecho via B.1 se materializar.
- Reservar sub-passos A/B/C/D — política P158 preservada
  literal.
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  materializa.
- Promover outros patterns emergentes a ADRs meta separadas
  (Field semantic adiada N=5; div-N N=2; encerramento Fase
  N=2) — Caminhos administrativos XS separados se humano
  priorizar.
- Tocar em código `.rs` — P226 é documental puro.
- Tocar em L0 prompts — ADR-0067 PROPOSTO formaliza
  precisamente o pattern "L0 não tocado para refactors".

---

## §5 Riscos a evitar

1. **ADR-0067 número conflito**: tentação de usar ADR-0067
   sem auditar slot. Mitigação: C1 audit empírico.
2. **ADR-0079 número conflito**: idem; auditar slot 0079
   livre.
3. **Promover ADR-0067 directamente a EM VIGOR**: tentação
   de pular PROPOSTO. Rejeitada — paridade ADR-0034/0065
   PROPOSTO inicial preserva pattern administrativo
   incremental.
4. **Promover ADR-0079 a IMPLEMENTADO**: tentação — P226 é
   diagnóstico; ADR-0079 PROPOSTO até materialização
   completa Fase 5.
5. **Reservar sub-passos A/B/C/D explicitamente**: tentação
   de "fixar próximo é A.1". Rejeitada — política P158
   preservada literal.
6. **Reabrir DEBT-56 ENCERRADA prematuramente**: tentação
   se "C.2 está nas considerações". Rejeitada — DEBT-56
   preservada literal; reabertura via DEBT-56b se/quando
   C.2 materializar.
7. **Magnitude P226 inflar para L**: tentação de detalhar
   cada sub-passo A/B/C/D ao nível de spec individual.
   Rejeitada — diagnóstico identifica + caracteriza
   sub-passos; specs individuais ficam para materialização.
8. **Inflação inventário 148 footnote ⁴⁷**: tentação de
   anotar pormenores Fase 5 candidata em inventário.
   Rejeitada — política minimalista (sem reclassificações
   em P226 puro diagnóstico).
9. **Pattern emergente N=7 promovido sem critério explícito**:
   ADR-0067 deve declarar 7 aplicações cumulativas
   explicitamente em §"Validação".
10. **Reaberturas arquitecturais sem nota explícita**:
    ADR-0079 deve registar reaberturas C.1 (Opção B P219)
    + C.2 (P216B + DEBT-56) + D (ADR-0066) explicitamente.
    Mitigação: §"Reaberturas" dedicada em ADR-0079.
11. **Skip ADR meta L0 minimal por "óbvio"**: tentação de
    "todos sabem; não precisa ADR". Rejeitada — N=7 patamar
    sólido + Decisão H2 humana explícita.
12. **Diagnóstico amplo sem matriz dependências**: paridade
    P215 + P156B inclui matriz cross-passo. Manter em
    §"Dependências cross-passo" do diagnóstico novo.

---

## §6 Hipótese provável

C1 confirmará ADR-0067 e ADR-0079 slots livres
(provavelmente); N=7 pattern empírico confirmado em audit
(7 aplicações cumulativas).

C2 criará ADR-0067 meta documental PROPOSTO ~150-200
linhas (paridade ADR-0034/0065 estrutura).

C3-C6 estructurarão diagnóstico amplo `diagnostico-layout-fase-5-completar.md`
em 4 categorias A+B+C+D com matriz dependências cross-passo
+ magnitudes estimadas + reaberturas registadas
explicitamente. ~300-500 linhas total.

C7 criará ADR-0079 PROPOSTO ~150-200 linhas (paridade
ADR-0078 estrutura).

C8 reportará 2039 verdes preservados.

C9 reportará 0 violations; "Nothing to fix".

C10 actualizará blueprint §3.0quaterdecies + README ADRs
+ skip footnote ⁴⁷ inventário 148 per política minimalista.

Custo real: **M+ (~3-4h)**. Maior parcela em C3-C6
(diagnóstico amplo 4 categorias com matriz dependências
+ estimativas cumulativas).

Mas é hipótese, não decisão. C1-C10 fixam-se
empíricamente.

---

## §7 Particularidade P226

P226 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro passo Fase 5 Layout candidata diagnóstico** —
  paridade estrutural P215 Fase 3 (P156B Fase 1 também
  precedente). Pattern "diagnóstico amplo + ADR PROPOSTO
  + roadmap" N=4 → **5 cumulativo** (P156B + P159B + P160
  + P215 + **P226**).
- **Decisão humana literal "completar Layout" = Tudo
  A+B+C+D** — primeira aplicação pós-M9c de escopo
  arquitectural maior reabrindo decisões fechadas (Opção
  B P219; P216B; ADR-0066 PROPOSTO; arquitectura
  single-pass).
- **Decisão humana literal P226 = ADR meta + diagnóstico
  agregado** — pattern "ADR meta + diagnóstico agregado"
  N=1 inaugurado P226 (distinto de P215 que foi diagnóstico
  + ADR PROPOSTO sem componente ADR meta).
- **Pattern "ADR PROPOSTO com materialização parcial graded"
  expandido**: ADR-0067 PROPOSTO + ADR-0079 PROPOSTO +
  ADR-0066 PROPOSTO (pre-existente). 3 ADRs PROPOSTAS
  cumulativas com semantic distinta.
- **2 reaberturas arquiteturais maiores registadas
  explicitamente** em ADR-0079 §"Reaberturas":
  - Reabertura Opção B P219 graded (Categoria C.1 Place
    float real).
  - Reabertura P216B + DEBT-56 ENCERRADA (Categoria C.2
    Opção A multi-region; criação prevista DEBT-56b se/quando
    C.2 materializar).
  - Reabertura ADR-0066 PROPOSTO + arquitectura single-pass
    (Categoria D runtime queries).
- **Magnitude P226 M+ (~3-4h)** — maior que P215 (M ~2h)
  por 4 categorias vs 1 área.
- **Distribuição ADRs muda**: PROPOSTO 11 → **13** (+2:
  ADR-0067 + ADR-0079); IMPLEMENTADO 21 preservado; total
  65 → **67**.
- **Layout em estado terminal estructural reconhecido P225
  agora tem roadmap "completar Layout" formalizado P226**.
  Trade-off ~30-50h cumulativo é trabalho substantivo;
  decisão humana caso-a-caso preserva flexibilidade.
- **Política "sem novas reservas" P158 preservada literal**
  — 13-14 sub-passos identificados mas NÃO reservados.

Por isso §5 risco 6 (reabrir DEBT-56 prematuramente) é o
mais provável. Tentação óbvia é "C.2 reabre Opção A;
reabrir DEBT-56 ENCERRADO". Defesa: DEBT-56 preservada
literal; novo DEBT-56b candidato se/quando C.2 materializar
(decisão diferida).

**Critério de aceitação P226**:
- ADR-0067 meta documental PROPOSTO criada.
- ADR-0079 Fase 5 Layout PROPOSTO criada.
- Diagnóstico amplo `diagnostico-layout-fase-5-completar.md`
  criado com 4 categorias + matriz dependências.
- Blueprint §3.0quaterdecies marca abertura série β
  adicionada.
- README ADRs actualizado (PROPOSTO 11 → 13; total 65 → 67).
- 2039 tests verdes preservados (zero código tocado).
- 0 violations preservadas.
- "Nothing to fix" lint.
- Política "sem novas reservas" P158 preservada literal.

**Estado pós-P226 esperado**:
- Tests workspace: 2039 verdes preservados.
- Content variants: 59 preservado.
- Stdlib funcs: 59 preservado.
- ADR-0061 IMPLEMENTADO; ADR-0078 IMPLEMENTADO; ADR-0066
  PROPOSTO + **ADR-0067 PROPOSTO + ADR-0079 PROPOSTO**.
- Distribuição ADRs: PROPOSTO **13** (+2); IMPLEMENTADO
  21; total **67** (+2).
- Saldo DEBTs: 12 preservado (DEBT-34d preservado per
  `P224.div-1`).
- Fase 5 Layout candidata diagnosticada + roadmap registado
  em ADR-0079 PROPOSTO.
- Pattern "L0 minimal para refactors" N=7 formalizado em
  ADR-0067 PROPOSTO.
- Pattern "diagnóstico amplo + ADR PROPOSTO + roadmap"
  N=4 → 5 cumulativo.
- Pattern "ADR meta + diagnóstico agregado" N=1
  inaugurado P226.
- 18 aplicações cumulativas anti-inflação pós-P205D
  (P226 documental puro preserva política).
- **Trajectória aberta pós-P226**: sub-passos A/B/C/D
  materialização caso-a-caso (Categoria A.1 stroke Grid
  candidato baixo risco se humano priorizar; ou outra
  ordem conforme dependências cross-passo identificadas).
