# Reflexão metodológica formal — cluster Gradient (P273.5-P273.17)

**Data**: 2026-05-18.
**Passo origem**: P273.17 (passo administrativo S+).
**Cluster**: Visualize / Gradient (encerrado definitivamente pós-P273.17).
**Tipo**: documento reflexão retrospectiva legível standalone; não-código.
**Magnitude**: ~500 linhas markdown.

---

## §1 — Trajectória factual P273.5-P273.16

Cluster Gradient atravessou **13 sub-passos consecutivos** (P273.5
a P273.17) — **caminho mais longo de sub-passos consecutivos do
mesmo cluster documentado no projecto cristalino**.

### Sequência cronológica completa

| Passo | Tipo | Magnitude real | Outcome | Sub-padrão N inaugurado/crescido |
|---|---|---|---|---|
| **P273.5** | Refino estrutural L1 | S (~30 LOC) | IMPLEMENTADO | DEBT-37 N=2 (consumer-pending) |
| **P273.6** | Refino estrutural L1+L3 | S+ (~80 LOC; cascade ~86 sites) | IMPLEMENTADO | DEBT-37 N=3 (consumer real); Template-passo N=1 inaugural |
| **P273.7** | Refino estrutural L1 | S menor (~20 LOC) | IMPLEMENTADO | Template-passo N=2 (Boxed paralelo Block) |
| **P273.8** | Cleanup XS | XS (~4 LOC `_`) | IMPLEMENTADO | Cleanup XS derivado N=1 inaugural |
| **P273.9** | Refino estrutural L1 | M (~58 LOC; Grid+Stack+Pad) | IMPLEMENTADO | Layout duplo arquitectural aceite N=1 inaugural; DEBT-37 N=4 |
| **P273.10** | Refino estrutural L3-only | S+ (~75 LOC) | IMPLEMENTADO | L3-only parent_bbox N=1 inaugural |
| **P273.11** | Cleanup intra-cluster | XS (-14 LOC net) | IMPLEMENTADO | Extract helper de replicação inline N=1 inaugural |
| **P273.12** | Refino arquitectural L3 | S (~85 LOC) | IMPLEMENTADO | Dedup Arc::as_ptr resources N=3 crossing; Bug arquitectural intencional N=1 |
| **P273.13** | Refino estrutural L3 (INSERIDO) | S (~35 LOC) | IMPLEMENTADO | L3-only parent_bbox N=2; Triplicação Group bbox N=1 |
| **P273.14** | Fase A go/no-go | 0 LOC | SCOPE-OUT-RECONFIRMED | Scope-out reconfirmado por Fase A N=1 inaugural (constraints externas) |
| **P273.15** | Fase A go/no-go | 0 LOC | SCOPE-OUT-RECONFIRMED | Scope-out reconfirmado por Fase A N=2 (constraints internas) |
| **P273.16** | Fase A go/no-go | 0 LOC | SCOPE-OUT-RECONFIRMED | Scope-out reconfirmado por Fase A N=3 crossing (estrutural aceita) |
| **P273.17** | Admin reflexão + 3 ADRs meta | 0 LOC; ~1500 linhas markdown | IMPLEMENTADO (admin) | Passo administrativo XS/S criar ADRs meta N=3 (não formalizado) |

**Totais**:
- 9 sub-passos materializados código (P273.5-P273.13).
- 3 sub-passos scope-out reconfirmados (P273.14-P273.16).
- 1 sub-passo administrativo S+ (P273.17 — este passo).
- ~300 LOC L1 + L3 acumulados (incluindo cascade ~86 sites P273.6).
- ~50 tests E2E novos cobrindo todas as variantes.
- 0 regressões em 2644 baseline (mantido bit-exact ao longo de
  P273.14-P273.17).

### Decisões inversas registadas

**P273.10 inserido por priorização B**: pós-P273.9, sequência prevista
era directa para P273.10 (CMYK-ICC pré-renumeração). Mas P273.13 spec
identificou pendência de `draw_item_local` Group gradient (caminho
emit real). Priorização B aceitou inserir P273.13 antes de P273.14
CMYK-ICC. Sequência renumerada: P273.10→P273.13 (que era P273.10)
no rascunho original; P273.13 (inserido) → P273.14 (era P273.10).

**Disciplina preservada**: cada inserção foi documentada
explicitamente (P273.10 §8 expôs P273.X-bis-group → tornou-se nada;
P273.12 §9 expôs draw_item_local issue → tornou-se P273.13 inserido).

---

## §2 — Sub-padrões emergentes inaugurados ou consolidados

| Sub-padrão | N pós-cluster | Inauguração | Reaplicações |
|---|---|---|---|
| **Anotação cumulativa em vez de ADR nova** | N=23 | Pré-cluster | P273.5-P273.16 todos (16 anotações ADR-0091 consecutivas) |
| **Reutilização literal helpers cross-passos** | N=17 | Pré-cluster | P273.5-P273.13 (helpers `apply_parent_transform`, `measure_content_constrained`, etc.) |
| **Cap LOC hard vs soft explícito** | N=16 | Pré-cluster | P273.5-P273.13 todos (estouros soft registados em ADR-0094 Pattern 1) |
| **Aplicação meta-ADR (ADR-0093)** | N=13 | P272 | P273.5-P273.17 todos |
| **Aplicação meta-ADR (ADR-0094)** | N=12 | P270.1 | P273.5-P273.13 (NO-GO P273.14-16 sem cap aplicado) |
| **Pattern DEBT-37 `cell_origin_*` replicado** | N=4 ⭐ | P84.6 | P273.5/6/9 — **formalizado ADR-0096** |
| **Template-passo replicado literal** | N=2 | P273.6 | P273.7 + P273.9 |
| **Sub-passos consecutivos do mesmo cluster** | N=13 | P273.5 (intra-cluster) | P273.6-P273.17 todos |
| **Layout duplo arquitectural aceite** | N=1 | P273.9 | (inaugural; aguardar reaplicação) |
| **L3-only parent_bbox** | N=2 | P273.10 | P273.13 |
| **Dedup `Arc::as_ptr` resources** | N=3 ⭐ | P73 | P263 + P273.12 — **formalizado ADR-0095** |
| **Bug arquitectural intencional corrigido** | N=1 | P273.12 | (inaugural) |
| **Triplicação Group bbox** | N=1 | P273.13 | (inaugural; candidato extract helper) |
| **Extract helper de replicação inline** | N=1 | P273.11 | (inaugural) |
| **Cleanup XS derivado** | N=1 | P273.8 | (inaugural; análogo aplicável noutros clusters) |
| **Scope-out reconfirmado por Fase A** | N=3 ⭐ | P273.14 | P273.15 + P273.16 — **formalizado ADR-0097** |
| **Diagnóstico imutável** | N=33 | Pré-cluster | P273.5-P273.17 todos (28 consumos directos cluster) |

⭐ — 3 sub-padrões atingiram limiar formalização N=3-4 e foram
formalizados via ADRs meta em P273.17.

---

## §3 — Limiares formalização atingidos — 3 sub-padrões → 3 ADRs

### ADR-0095: "Dedup `Arc::as_ptr` resources" (N=3 cumulativo)

- **P73** image_resources — inauguração.
- **P263** pattern_resources — reaplicação.
- **P273.12** pattern_resources bbox-aware via `DedupKey` —
  reaplicação consolidando.

**Mecânica formalizada**: `HashMap<usize, idx>` com `Arc::as_ptr(x)
as usize` como chave; estendida para `HashMap<DedupKey, idx>` quando
contexto matters.

### ADR-0096: "Pattern DEBT-37 `cell_origin_*` replicado" (N=4 cumulativo)

- **P84.6** Grid cell `cell_origin_x/y/w` — inauguração.
- **P273.5** `parent_bbox` consumer-pending — reaplicação inaugural
  cluster Gradient.
- **P273.6** consumer real Block + cascade ~86 sites — DEBT-37
  fechado.
- **P273.9** Grid cell paralelo — reaplicação cross-cluster.

**Mecânica formalizada**: campo `Option<T>` Layouter com `#[allow(dead_code)]`
consumer-pending; consumer activado em passo subsequente; DEBT
registado/fechado.

### ADR-0097: "Scope-out reconfirmado por Fase A" (N=3 cumulativo)

- **P273.14** CMYK-ICC — constraints externas (licensing + invariante L0).
- **P273.15** Bbox medido — constraints internas (custo perf + demanda).
- **P273.16** Bbox.y topo-exacto — bloqueador estrutural aceito
  (P156H + ADR-0078).

**Mecânica formalizada**: Fase A com decisão go/no-go binária;
NO-GO output documental (`trabalho-previo-externo.md`) é cumprimento
honesto.

---

## §4 — Descobertas metodológicas

### §4.1 — Caps soft sub-estimados sistematicamente

Múltiplos passos cluster Gradient registaram **estouros soft** de
cap LOC per ADR-0094 Pattern 1:

| Passo | Cap soft estimado | Real | Estouro |
|---|---|---|---|
| P273.6 | L1 soft 70 | ~80 | +14% (cascade ~86 sites) |
| P273.7 | L1 soft 20 | ~20 | limite |
| P273.9 | L1 soft 60 | ~58 | dentro |
| P273.10 | L3 soft 50 | ~75 | +50% (scope creep pattern_resources_for_page) |
| P273.12 | L3 soft 70 | ~85 | +21% (scope creep paralelo) |
| P273.13 | L3 soft 50 | ~35 | folga 30% |

**Lição**: caps soft útil mas raramente preciso pre-Fase A. Scope
creep arquitectural (P273.10 §A.7 e P273.12 §A.6) é causa principal
de estouros — descoberta pós-empírica que não cabe no cap nominal.

ADR-0094 Pattern 1 lidou bem com estouros via registo explícito
sem invalidar passos.

### §4.2 — Fase A factual prevalece sobre premissa documental

**Descoberta P273.16**: spec premissa "DEBT-56 EM ABERTO desde
2026-04-25" factualmente desactualizada. Verificação literal em
`DEBT.md:535` confirmou DEBT-56 **ENCERRADO P221 (2026-05-12)** —
6 dias antes da spec ser escrita.

**Conclusão NO-GO permaneceu correcta** via fundamentos diferentes
(P156H + ADR-0078 §sub-fase b em vez de DEBT-56). Demonstra
**padrão honesto de Fase A factual prevalece** quando premissa
documental divergir de realidade verificável.

Mecânica que se aplica recursivamente: o próprio processo de
verificar pode descobrir que o documento de referência está
desactualizado, e isso é parte legítima do output.

### §4.3 — Cleanups XS revelam dívidas latentes

**P273.8** (cleanup 4 warnings `parent_bbox_at_emit: _`) e **P273.11**
(extract Stack measurement helper) são cleanups XS que **emergiram
como pendências específicas registadas em passos materiais
anteriores**:

- P273.8 emergiu de P273.6 §9 ("4 warnings registados").
- P273.11 emergiu de P273.9 §9 segundo bullet ("Stack inline
  replication candidato extract helper").

Pattern: passos materiais documentam pendências XS detectadas
durante materialização; passos cleanup posteriores fecham-nas.
Disciplina preservada — sem cleanups XS, dívidas persistiriam.

### §4.4 — Bugs latentes corrigidos durante materialização

**P273.9** corrigiu bug latent em `translate_frame_item` (helpers.rs)
que descartava `parent_bbox_at_emit` do FrameItem::Shape durante
translate. Bug não-observable pré-P273.9 porque Grid cell era único
caller que emitia Shapes via translate, e Grid não populava
`parent_bbox` até P273.9.

**Sub-padrão "Bug latent corrigido em scope creep" N=1** emergente.

**P273.13** corrigiu bug latent em `draw_item_local`
(Group recursion) que usava solid color fallback em vez de consumir
pattern dict. Detectado durante P273.12 §9 quarto bullet.

**Disciplina**: bugs latentes detectados durante materialização são
corrigidos no mesmo cluster (não diferidos) quando estão dentro do
escopo arquitectural natural.

---

## §5 — Pendências residuais pós-cluster

### Pendências fora-cluster preservadas

Inalteradas:
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

### Pendências cluster Gradient

**3 scope-outs reconfirmados** (com trabalho prévio externo
documentado):
1. **P-Gradient-CMYK-ICC** — P273.14 NO-GO. 3 pré-requisitos:
   ADR sobre crate + profile + licença + decisão PDF size.
2. **P273.X-bis-bbox-medido-pos-layout** — P273.15 NO-GO.
   2 pré-requisitos: caso empírico + decisão custo perf.
3. **P273.X-bis2-bbox-y-topo-exacto-inline** — P273.16 NO-GO.
   3 pré-requisitos: caso empírico + refactor inline line_height
   + decisão dívida invisível.

**2 candidatos XS NÃO reservados**:
1. **P273.X-bis-helper-group-bbox** — extract helper
   `group_bbox_from_frame_item` partilhado entre
   `scan_all_gradients.walk` + `pattern_resources_for_page.walk` +
   `draw_item_local`. Sub-padrão "Extract helper de replicação
   inline" N=1 P273.11 precedente; reaplicação → N=2.
2. **P273.X-bis-content-md-debt56-update** — L0 `content.md:824`
   referência DEBT-56 fechado P221 desactualizada (~1 LOC L0).
   Descoberta empírica P273.16 §A.7. Sub-padrão "Cleanup XS derivado"
   N=1 P273.8 precedente.

### Pendência implícita fora cluster

- **P273.X-bis-draw-item-local-text-image** — Text + Image em Groups
  silenciosamente descartados em `draw_item_local`. Detectado P273.13
  §9 (comentário "_ => {} // Texto e outros tipos em grupos: adiado
  para passo futuro"). **Fora do cluster Gradient** (afecta
  Text/Image). NÃO reservado.

---

## §6 — Trade-offs aceitos

### Trade-off 1: `/DeviceCMYK` vs `/ICCBased`

**Decisão P270.2 + P273.14 NO-GO**: `/DeviceCMYK` directo preserved.
PDF/A compliance preserved como pendência inalterada.

**Razão**: ausência de profile CMYK royalty-free industry-recognized
+ invariante L0 "sem crates externas de PDF" + custo PDF size
profile embebido.

**Aceito por**: ADR-0091 §"Anotação cumulativa P270.2" preserved
+ ADR-0091 §"Anotação cumulativa P273.14" SCOPE-OUT-RECONFIRMED.

### Trade-off 2: 3γ.2.γ pré-layout vs 3γ.2.β medição

**Decisão P273.6 + P273.15 NO-GO**: 3γ.2.γ preserved para Block sem
dimensions literais; cae no fallback page_bbox L3 P273.5.

**Razão**: zero demanda empírica em 9 sub-passos + custo perf eager
O(N²) + impl walker desproporcional.

**Aceito por**: ADR-0091 §"Anotação cumulativa P273.6" preserved
+ ADR-0091 §"Anotação cumulativa P273.15" SCOPE-OUT-RECONFIRMED.

### Trade-off 3: bbox.y baseline-relative vs topo-exacto inline

**Decisão P273.7 + P273.16 NO-GO**: 3γ.2.γ-inline-baseline-y preserved
para Boxed inline. P156H limitação consciente coerente.

**Razão**: zero demanda em 9 sub-passos + Caminho 1 refactor inline
line_height fora de escopo S-M + Caminho 2 ad-hoc cria dívida
invisível.

**Aceito por**: ADR-0091 §"Anotação cumulativa P273.7" preserved
+ ADR-0091 §"Anotação cumulativa P273.16" SCOPE-OUT-RECONFIRMED.

### Trade-off comum: ADR-0054 graded como fundamento

Os 3 trade-offs partilham fundamento: **ADR-0054 graded — "menor
mudança suficiente"** preserved. Refinos qualitativos opcionais sem
demanda empírica concreta são over-engineering quando aplicados
indiscriminadamente. Disciplina anti-over-engineering preservada.

---

## §7 — Anti-padrões evitados

### Anti-padrão 1: Over-formalização sub-padrões emergentes

**Evitado**: P273.17 formaliza **apenas 3 sub-padrões com N≥3** —
não os 17+ sub-padrões emergentes cumulativos.

**Razão**: limiar formalização N=3-4 é critério rigoroso; sub-padrões
N=1-2 ficam preserved como emergentes aguardando reaplicação para
consolidação. Resistir à tentação de formalizar tudo é parte da
disciplina (anti-padrão explícito P273.17 spec §0).

**Sub-padrões NÃO formalizados** (preserved):
- L3-only parent_bbox (N=2; aguardar reaplicação cross-cluster).
- Template-passo replicado literal (N=2; idem).
- Layout duplo arquitectural aceite (N=1).
- Extract helper de replicação inline (N=1).
- Triplicação Group bbox (N=1).
- Bug arquitectural intencional corrigido (N=1).
- Bug latent corrigido em scope creep (N=1).
- Cleanup XS derivado (N=1).

### Anti-padrão 2: Scope creep cego

**Evitado**: scope creeps reconhecidos foram registados como
**scope creep arquitectural aceito** (não silencioso):

- **P273.10 §A.7** — `pattern_resources_for_page` recursão paralela
  ao `scan_all_gradients` recursão. Sem ele, P273.10 não produziria
  observable behavior.
- **P273.12 §A.6** — `emit_stroke_paint` ganha `effective_bbox` +
  3 build_page_stream signatures + 3 callsites. Sem ele, DedupKey
  lookup falharia em emit.
- **P273.13 §A.4** — arm Group novo em `draw_item_local` (corrige
  bug pre-existente de nested Groups silenciosamente descartados).

Cada scope creep documentado **com fundamento** — não foi creep
silencioso.

### Anti-padrão 3: NO-GO como cobertura para evitar trabalho

**Evitado**: NO-GO em P273.14/15/16 **foi cumprimento honesto** do
critério "verificar empíricamente". ADR-0097 §"Quando NÃO aplicar
NO-GO" formaliza limites:

- NO-GO ad-hoc sem Fase A formal → rejeitado.
- NO-GO para evitar trabalho viável → rejeitado.
- NO-GO sem 3 razões legítimas (externa/interna/estrutural) →
  rejeitado.

### Anti-padrão 4: Inserção de sub-passos não-documentada

**Evitado**: P273.13 inserido por priorização B foi **explicitamente
documentado** na spec P273.13 §9 e em todos os relatórios
subsequentes. Renumeração da sequência registada em P273.14 §9 +
P273.15 §9 + P273.16 §9.

---

## §8 — Reflexão final — cluster Gradient como caso de estudo metodológico

### Quantitativo

- **13 sub-passos consecutivos** (P273.5-P273.17) — caminho mais
  longo documentado no projecto.
- **9 sub-passos materializados** + **3 scope-out reconfirmados**
  + **1 admin reflexão**.
- **3 ADRs meta novas** (ADR-0095/0096/0097) — 81 → 84 vigentes.
- **17 anotações cumulativas ADR-0091** consecutivas (P273.5-P273.17).
- **17 sub-padrões emergentes** registados cumulativamente.
- **3 sub-padrões formalizados** via ADRs meta (N=3-4 crossing
  limiar).
- **~300 LOC L1+L3** acumulados; **~50 tests novos**; **2644
  baseline preservado** ao longo de P273.14-P273.17.
- **0 regressões** em todo o cluster.

### Qualitativo

Cluster Gradient começou em P262 como feature básica (Linear
único); cresceu organicamente para refino estrutural extensivo via
P273.5-P273.13 (Block + Boxed + Grid + Stack + Pad + Group);
consolidou-se com 3 scope-outs documentados (CMYK-ICC + Bbox
medido + Bbox.y topo-exacto) e 3 ADRs meta formalizando sub-padrões
empíricos.

**Output final**:
- Cluster funcional + feature-complete user-facing + qualitativo +
  refino estrutural + cleanup intra-cluster + dedup bbox-aware +
  render real Groups.
- **Dívida documentada** (3 scope-outs com trabalho prévio externo
  identificado + 2 candidatos XS NÃO reservados).
- **Sub-padrões consolidados** (3 ADRs meta novas) + sub-padrões
  emergentes preserved para reaplicação cross-cluster futura.

### Caso de estudo metodológico

Cluster Gradient torna-se **laboratório metodológico documentado
do projecto cristalino**. Lições transferíveis a futuros clusters:

1. **Refino estrutural incremental** (Pattern DEBT-37 ADR-0096)
   é sustentável quando dividido em passos S separados.
2. **Dedup Arc::as_ptr** (ADR-0095) é paradigma estável L3 export.
3. **NO-GO como output documental** (ADR-0097) é cumprimento
   honesto, não falha.
4. **Caps soft útil mas raramente preciso** — scope creep
   arquitectural é causa principal de estouros (P273.6/10/12).
5. **Fase A factual prevalece sobre premissa documental** —
   descoberta empírica pode actualizar spec (P273.16 DEBT-56).
6. **Cleanups XS revelam dívidas latentes** — pendências específicas
   detectadas durante materialização (P273.8 + P273.11).
7. **Bugs latentes corrigidos durante materialização** quando
   dentro do escopo arquitectural natural (P273.9 + P273.13).

### Sub-padrão meta-meta NÃO formalizado

P273.17 cria 3 ADRs meta simultaneamente — terceira aplicação do
sub-padrão **"Passo administrativo XS/S criar/promover ADRs meta"**
(N=3 cumulativo: P156K + P271 + P273.17). Padrão consolidado mas
**NÃO formalizado** nesta sessão (anti-padrão over-formalização
explícito).

Documentado aqui para futuro: se algum cluster futuro terminar com
mesmo padrão, então N=4 e candidato meta-ADR formalização.

---

## §9 — Conclusão

Cluster Gradient está **definitivamente encerrado pós-P273.17**.
Próximo passo natural: **sair do cluster Gradient para outro
cluster do projecto**. Pendências restantes (ADR-0055bis fonts,
P-Footnote-N, DEBT-33 Bézier, Stroke/Curve/Polygon, Tiling, outro
cluster) disponíveis.

A trajectória 13 sub-passos consecutivos demonstrou:
- **Refino estrutural extensivo é viável** com magnitude controlada
  por sub-passos pequenos.
- **NO-GO é output legítimo** quando empíricamente fundamentado.
- **Sub-padrões emergem naturalmente** durante materialização; ADRs
  meta formalizam apenas os que atingiram limiar.
- **Honestidade arquitectural prevalece** sobre completude
  prematura.

A disciplina é: **documentar honestamente o que existe**,
**formalizar apenas o que atinge limiar com valor metodológico
claro**, **resistir à tentação de formalizar tudo**.

---

## §10 — Referências

- **ADR-0091** — Gradient ColorSpace runtime + CMYK strategy
  (anotada cumulativa P273.5-P273.17; 17 anotações consecutivas).
- **ADR-0095** (criada P273.17) — Dedup `Arc::as_ptr` resources.
- **ADR-0096** (criada P273.17) — Pattern DEBT-37 campo Layouter
  consumer-pending.
- **ADR-0097** (criada P273.17) — Scope-out reconfirmado por Fase A.
- **ADR-0093** — Meta-metodologia evolução ADRs (Pattern 2 anotação
  cumulativa; aplicação prática N=13 cumulativo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC).
- **ADR-0054** — Critério fecho DEBT-1 graded (fundamento dos 3
  trade-offs aceitos).
- **ADR-0085** — Diagnóstico imutável (28 consumos directos cluster
  Gradient).
- **ADR-0029** — Pureza física L1 (preserved absoluto em todos os
  sub-passos).
- **DEBT.md DEBT-37** — Origem pattern (P84.6); fechado P273.6.
- **DEBT.md DEBT-56** — Column flow Fase 3 Layout (ENCERRADO P221;
  descoberta empírica P273.16).
- **3 documentos `trabalho-previo-externo.md`** (P273.14/15/16) —
  outputs legítimos NO-GO.
- **13 specs + 13 diagnósticos + 13 relatórios** P273.5-P273.17 —
  trajectória completa cluster Gradient.

---

*Documento reflexão imutável produzido em 2026-05-18 como output
legítimo do passo administrativo P273.17. Cluster Gradient laboratório
metodológico documentado do projecto cristalino — 13 sub-passos
consecutivos, 3 ADRs meta formalizadas, 17 sub-padrões emergentes
registados cumulativamente, 3 trade-offs aceitos via NO-GO honesto.*
