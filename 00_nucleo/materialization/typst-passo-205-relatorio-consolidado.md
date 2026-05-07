# Relatório consolidado da série P205 — F3 fechado completo

**Escopo**: P205A–E (2026-05-07).
**Tema**: F3 — Layouter sub-stores trackable (sealing
post-iteração).
**Output 2 de 4 do passo P205E** (paralelo a P204H §C4).
**Estado final**: **ADR-0074 ACEITE final** (P205E
2026-05-07); F3 minimal **fechado completo**; 7/7
condições do plano de validação CUMPRIDAS.

---

## §1 Trajectória da série

P205A–E executou-se em sessão única 2026-05-07
(tom contínuo M8 → F3) com 5 sub-passos
diagnóstico-primeiro + implementação progressiva +
encerramento administrativo.

### §1.1 P205A — Diagnóstico-primeiro de F3

Magnitude M (real ~30 min).

P205A iniciou a série após M8 estruturalmente fechado
em P204H. Auditoria empírica A1–A14 cobriu 5 blocos
arquitecturais: 21 fields ortogonais do Layouter (per
snapshot 2026-05-05 §6); padrão `#[comemo::track]` em
vanilla (8 sub-stores tracked); arquitectura vanilla
(Engine + N Layouters); compatibilidade com fixpoint;
Position + §C6a.

Diagnóstico C1–C11 fixou:

- **Apenas 3 sub-fields são Categoria B** (runtime
  introspecção): `runtime.label_pages`,
  `runtime.known_page_numbers`, `runtime.positions`. Os
  18 restantes são Categoria A/C/D — fora-de-escopo F3.
- **Vanilla não tem Layouter monolítico** —
  arquitectura cristalino vs vanilla é fundamentalmente
  assimétrica. F3 não pode ser paridade vanilla
  literal. Registado como `P205A.div-1`.
- **Vanilla trackeia apenas post-sealing** —
  `PagedIntrospector::new(&pages)` constrói sub-stores
  immutáveis post-layout. Cristalino diverge
  intencionalmente (single-pass populates
  `runtime.positions`). Sealing point inevitável —
  registado como `P205A.div-2`.

ADR-0074 PROPOSTO produzido com 5 alternativas
consideradas e rejeitadas explicitamente (B/C/D/E/F),
plano de validação com 7 condições, plano de
materialização com 5 sub-passos.

### §1.2 P205B — Sealing infrastructure + SealedPositions

Magnitude S–M (real ~30 min).

P205B materializou sealing infrastructure conforme
ADR-0074 §Decisão:

- `pub struct SealedPositions { positions: HashMap<
  Location, Position> }` em
  `01_core/src/entities/sealed_positions.rs`. **Sem
  `Arc`** (decisão D1 de P205B; coerência com pattern
  `BibStore`/`MetadataStore`; clone O(n) único por
  iteração).
- `#[comemo::track] impl SealedPositions { fn position_of
  (&self, loc: Location) -> Option<Position> }`.
- `Layouter::finish` produz
  `doc.extracted_positions: SealedPositions` via
  `from_runtime(self.runtime.positions)` — sealing
  point literal per ADR-0074 C4.
- L0 prompt em
  `00_nucleo/prompts/entities/sealed-positions.md`
  (hash `94c68ba8`).

4 tests novos: 2 sentinelas (struct existe + impl
Track) + 2 unit (empty devolve None; from_runtime
preserva mappings). 1852 → 1856 verdes; 0 violations.

P205B teve **zero divergências** durante implementação
— ADR-0074 fixou decisão completa em P205A; P205B
executou directamente.

### §1.3 P205C — `position_of` impl real + consumer migration

Magnitude S–M (real ~25 min).

P205C activou impl real de `Introspector::position_of`
via Caminho A (`TagIntrospector` enriquecido em vez de
`PagedTagIntrospector` wrapper):

- `TagIntrospector` ganha campo `pub positions:
  SealedPositions` (default empty).
- `pub fn inject_positions(&mut self, sealed)` —
  caller pós-layout activa lookup real.
- `Introspector::position_of` impl delega
  `self.positions.position_of(location)`. Pre-injecção
  devolve `None` (preservando semântica P204D §C6a);
  pós-injecção devolve `Some(Position)` real.

4 tests novos: 3 unit (`p205c_*` em
`introspector.rs::tests`) + 1 E2E em `layout/tests.rs`
(`p205c_pipeline_layout_seal_inject_query_devolve_some`)
exercendo pipeline completo layout → seal → inject →
query. 1856 → 1860 verdes; 0 violations.

Caminho B (wrapper `PagedTagIntrospector`) rejeitado
em C2: cristalino tem única impl `Introspector`;
wrapper exigiria delegar 19 métodos só para 1 especial.
Vanilla precisa porque tem múltiplas impls
(paged/html/...); cristalino não. Per `P205A.div-1`.

Caminho C (adiar) rejeitado: ADR-0074 §"Decisão" fixou
explicitamente que F3 minimal fecha pendência ADR-0073
§C6a; adiar contradiria ADR PROPOSTA (mesmo que C1.1
mostre zero consumers de produção, infraestrutura
completa é pré-requisito para futuros
`here()`/`locate()` em P204F.div-1 expansion).

### §1.4 P205D — `label_pages` trackable (condicional) — DEFERIDO

Magnitude S documental (real ~15 min).

P205D auditou empíricamente em C1 (6 sub-secções) e
fixou Caminho B (adiar) em C2:

- C1.1: zero consumers de produção lêem
  `runtime.label_pages` directamente; `outline.rs:48`
  lê `runtime.known_page_numbers` (snapshot fixpoint
  anterior, distinto).
- C1.2: trait `Introspector` cristalino não tem
  `label_to_page`; vanilla também não — rota
  label→page é via `query_label` + `position(loc).page`.
- C1.4: `doc.extracted_label_pages` consumido apenas
  por convergência fixpoint (HashMap equality
  não-tracked) em `mod.rs:1575,1580`.
- C1.5: tracking de label_pages seria duplicação de
  info já tracked por `SealedPositions` + label
  registry.
- C1.6: aliasing confirmado — `runtime.label_pages`
  move-se para `doc.extracted_label_pages` em
  `finish` (paralelo literal a P205B).

Caminho B fixado por evidência. ADR-0074 §P205D anotada
`✅ DEFERIDO 2026-05-07` com fundamento empírico em 5
pontos. Sem alterações de código. 1860 mantém-se; 0
violations mantém-se.

### §1.5 P205E — Encerramento

Magnitude S documental (real ~25 min).

P205E auditou as 7 condições do plano de validação
ADR-0074 (todas CUMPRIDAS), fixou forma de fecho
(**Completo**) e transitou ADR-0074 PROPOSTO →
**ACEITE final**. Anotação cirúrgica em ADR-0066
(cross-reference §C6a fechada por F3) per C3
afirmativa. Blueprint actualizado com marca [P205E].

---

## §2 Divergências detectadas e absorvidas

### §2.1 `P205A.div-1` — Arquitectura vanilla diferente

**Origem**: P205A A4–A6 + B2.

**Causa**: vanilla typst não tem Layouter monolítico —
usa `Engine<'a>` + N Layouters especializados (Composer,
Distributor, Work, Collector, StackLayouter,
GridLayouter). Cristalino tem único `Layouter<M, S>`
agregando 21 fields ortogonais (per P204A snapshot).

**Impacto**: F3 não pode ser paridade vanilla literal.
Decisão arquitectural F3 é **cristalino-only** com
divergência intencional registada.

**Resolução**: ADR-0074 reconhece a divergência em
"Mecanismo (per P205A C3)" e "Sealing point (per P205A
C4)". P205C C2 rejeitou Caminho B
(`PagedTagIntrospector` wrapper vanilla-style) por
inflação desproporcionada. P205D C2 confirmou que
vanilla também não trackeia label_pages directamente —
divergência arquitectónica não impede paridade
funcional.

### §2.2 `P205A.div-2` — Categoria B reduzida a 3 sub-fields

**Origem**: P205A A2 + C1.

**Causa**: Categoria B (runtime introspecção) em
cristalino é apenas 3 sub-fields:
`runtime.label_pages`, `runtime.known_page_numbers`,
`runtime.positions`. Os 18 fields restantes são
Categoria A (runtime puro) ou C (config) ou D
(ambígua).

**Impacto**: F3 escopo "mínimo" e "médio" coincidem na
prática — Alternativa B (médio) rejeitada em ADR-0074.

**Resolução**: ADR-0074 fixou **F3 minimal** com 2
sub-stores (SealedPositions + SealedLabelPages
opcional). P205D deferiu o segundo por ausência de
benefício. Final: **1 sub-store** (SealedPositions);
escopo cristalino ainda menor que F3 minimal projectado
— mas suficiente para fechar §C6a.

---

## §3 Outputs concretos por sub-passo

### §3.1 Tabela de referência

| Sub-passo | Magnitude real | Outputs principais |
|-----------|----------------|---------------------|
| P205A | M (~30 min) | ADR-0074 PROPOSTO; 2 diagnósticos; relatório P205A |
| P205B | S–M (~30 min) | `sealed_positions.rs` + L0 prompt; 4 tests; ADR §P205B; relatório P205B |
| P205C | S–M (~25 min) | `introspector.rs` field+método+impl; `layout/tests.rs` E2E; 4 tests; ADR §P205C; relatório P205C |
| P205D | S documental (~15 min) | Inventário 6 sub-secções; relatório P205D; ADR §P205D anotado DEFERIDO |
| P205E | S documental (~25 min) | Inventário; consolidado (este); relatório P205E; ADR-0074 ACEITE; ADR-0066 anotada; blueprint [P205E] |

**Magnitude agregada real**: M + S-M + S-M + S + S
documental ≈ **M agregado** (menor que P204 L
cross-modular; F3 escopo mínimo per ADR-0074).

### §3.2 Ficheiros novos por camada

| Camada | Ficheiros novos | Origem |
|--------|------------------|--------|
| L0 prompts | `00_nucleo/prompts/entities/sealed-positions.md` | P205B |
| L1 código | `01_core/src/entities/sealed_positions.rs` | P205B |
| L2/L3/L4 | 0 | — |
| Diagnósticos | `typst-passo-205A-auditoria-f3.md`, `typst-passo-205A-diagnostico.md`, `typst-passo-205B-inventario.md`, `typst-passo-205C-inventario.md`, `typst-passo-205D-inventario.md`, `typst-passo-205E-inventario.md` | P205A–E |
| Materialização | 5 relatórios individuais + 1 consolidado (este) | P205A–E |
| ADR | 0074 (novo, PROPOSTO P205A → ACEITE P205E); 0066 anotada | P205A, P205E |

### §3.3 Ficheiros modificados por sub-passo

| Ficheiro | Sub-passos | Tipo |
|----------|------------|------|
| `01_core/src/entities/mod.rs` | P205B | export `pub mod sealed_positions;` |
| `01_core/src/entities/layout_types.rs` | P205B | campo `extracted_positions` em `PagedDocument` |
| `01_core/src/entities/introspector.rs` | P205C | campo `positions` + método `inject_positions` + impl `position_of` |
| `01_core/src/rules/layout/mod.rs` | P205B | `Layouter::finish` produz `extracted_positions` |
| `01_core/src/rules/layout/tests.rs` | P205C | 1 test E2E novo |
| `00_nucleo/adr/typst-adr-0074-...` | P205A, P205B, P205C, P205D, P205E | criação + 4 anotações sucessivas |
| `00_nucleo/adr/typst-adr-0066-...` | P205E | anotação cirúrgica F3 fecho §C6a |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | P205E | marca §3.0bis [P205E] F3 fechado |

---

## §4 Achados consolidados

### §4.1 Pendência ADR-0073 §C6a fechada estruturalmente

**Problema herdado**: P204D §C6a deixou
`TagIntrospector::position_of` retornando `None` como
solução temporária por `TagIntrospector` ser construído
pre-layout sem acesso a Layouter runtime.

**Resolução por F3**: P205B (sealing infrastructure)
+ P205C (`inject_positions` activa impl real). Caller
pós-layout faz `intr.inject_positions(doc.
extracted_positions.clone())`; `position_of` devolve
`Some(Position)` real. Pre-injecção (default empty)
preserva semântica `None` para consumers ainda não
migrados.

ADR-0073 §C6a permanece textualmente como registo
histórico do estado intermédio; F3 fecha-a sem alterar
ADR-0073 (per padrão de preservação histórica
estabelecido em P200/P204H). ADR-0066 anotada
cirúrgicamente em P205E para auditor futuro entender
chain-of-custody.

### §4.2 Divergência arquitectónica cristalino vs vanilla documentada

`P205A.div-1` regista que vanilla typst tem `Engine<'a>`
+ N Layouters especializados; cristalino tem único
Layouter monolítico. F3 é solução cristalina específica
— ADR-0074 explicita esta divergência em "Decisão" e
"Alternativa C rejeitada". Documentação preserva o
contexto para futuras decisões (ex: refactor
hipotético do Layouter monolítico em P210+ exigiria
revisitar F3).

### §4.3 Sealing point reproduzível

`Layouter::finish` produz `extracted_positions: SealedPositions`
1× por iteração (incluindo iterações fixpoint TOC).
Pattern `from_runtime` consolidado é
reutilizável para futuros sub-stores sealed (ex: caso
hipotético de re-materializar P205D por consumer real
emergir).

### §4.4 P205D condicional honesto

Decisão Caminho B (adiar) em P205D não foi cedência —
foi resposta empírica honesta a "zero benefício
observável detectado". ADR-0074 declarou P205D
condicional explicitamente; honrar a condicionalidade
afirmativamente ou negativamente não é ramo, é
resposta. Spec §8 anteviu esta hipótese como provável;
auditoria confirmou-a.

---

## §5 Métricas agregadas

### §5.1 Tests

| Sub-passo | Tests antes | Tests depois | ∆ |
|-----------|--------------|---------------|---|
| P205A | 1852 | 1852 | 0 (diagnóstico) |
| P205B | 1852 | 1856 | +4 (2 sentinelas + 2 unit) |
| P205C | 1856 | 1860 | +4 (3 unit P205C + 1 E2E) |
| P205D | 1860 | 1860 | 0 (deferred) |
| P205E | 1860 | 1860 | 0 (documental) |
| **Total série** | **1852** | **1860** | **+8** |

Estimativa do plano de validação: +10 a +18 tests.
Real: +8 (subestimou ligeiramente porque P205D deferred
eliminou ~4-6 tests previstos para `SealedLabelPages`).

### §5.2 LOC

| Tipo | LOC |
|------|-----|
| Código produção (sealed_positions.rs novo) | ~70 |
| Código produção (introspector.rs +impl P205C) | ~30 |
| Código produção (layout_types.rs + mod.rs P205B) | ~10 |
| Tests | ~150 (4+4+E2E pipeline) |
| Documental (ADRs novas/anotadas + diagnósticos + relatórios + blueprint) | ~1500+ |

### §5.3 ADRs

| ADR | Estado série P205 | Sub-passos |
|-----|-------------------|------------|
| ADR-0074 | NOVA — PROPOSTO (P205A) → ACEITE (P205E) | A → E |
| ADR-0066 | Anotação cirúrgica em P205E (cross-reference F3) | E |
| ADR-0073 | Não alterada (§C6a fechada por F3 sem editar) | (referenciada) |
| ADR-0072 | Não alterada (loops fixpoint preservados) | (referenciada) |

### §5.4 Sentinelas activas

- 19 sentinelas M8 preservadas (ADR-0073).
- +2 sentinelas P205B (`p205b_sealed_positions_struct_existe`,
  `p205b_sealed_positions_e_track`).
- +3 sentinelas P205C (`p205c_position_of_pre_injecao_devolve_none`,
  `p205c_inject_positions_activa_lookup_real`,
  `p205c_inject_positions_e_idempotente_para_reinjecao`)
  — funcionam como sentinelas estruturais.
- **Total sentinelas/quase-sentinelas activas**: 24 (19
  M8 + 5 F3).

---

## §6 Divergências da série

| Divergência | Origem | Impacto | Resolução |
|-------------|--------|---------|-----------|
| `P205A.div-1` | A4–A6, B2 | F3 cristalino-only; sem paridade vanilla literal possível | ADR-0074 reconhece; P205C rejeita Caminho B (wrapper); P205D usa para fundamentar Caminho B (adiar) |
| `P205A.div-2` | A2, C1 | Categoria B = 3 sub-fields; F3 mínimo coincide com médio | ADR-0074 fixa minimal; P205D deferred reduz ainda mais |

Sem novas divergências surgindo durante P205B–E.

---

## §7 Padrão demonstrado

### §7.1 Diagnóstico-primeiro de profundidade média (P205A)

14 cláusulas A1–A14 cobrindo 5 blocos arquitecturais —
menos que M8 (16 cláusulas em P204A) porque F3 escopo é
menor. Inventário empírico literal antes de fixar
decisões. Dois divergências detectadas e absorvidas em
ADR-0074.

### §7.2 Implementação directa quando ADR fixa decisão (P205B–C)

P205B e P205C executaram implementações directamente
após ADR-0074 fixar Decisão + Mecanismo + Sealing point
em P205A. Zero divergências mid-execution. Pattern
estabelecido: ADR-fixado obrigatório → implementação
não revisita decisões.

### §7.3 Condicional honrado afirmativamente ou negativamente (P205D)

P205D distinguiu correctamente "ADR-fixado obrigatório"
(P205C — fechar §C6a) vs "ADR-fixado condicional"
(P205D — sub-store opcional). Spec C2 fixou Caminho B
(adiar) por evidência empírica. ADR-0074 cond 3 do
plano de validação aceitou ambas as branches
explicitamente — honrar condicionalidade negativamente
não é falha.

### §7.4 Encerramento administrativo após auditoria empírica (P205E)

P205E auditou as 7 condições antes de fixar forma de
fecho. Decisão "Completo" baseou-se em evidência
literal: ADR-0074 declarou P205D condicional; cond 3
aceita deferral; logo P205D deferred é dentro do escopo
declarado. Anti-padrão evitado: inflar para
"Completo" sem honestidade ou inflar para
"estruturalmente fechado" sem necessidade.

### §7.5 Pattern `from_runtime` consolidado para sealing

`SealedPositions::from_runtime(self.runtime.positions)`
em `Layouter::finish` é literal-replicável para
futuros sub-stores sealed. Caminho A hipotético de
P205D usaria pattern análogo:
`SealedLabelPages::from_runtime(self.runtime.label_pages)`.
Pattern reutilizável e verificável por sentinelas
(struct existe + impl Track).

---

## §8 Estado pós-série face ao snapshot 2026-05-05

Snapshot 2026-05-05 §6 declarou: "alguns dos 21 fields
ortogonais do Layouter são candidatos a migrar para
sub-stores trackable se isto reduzir aliasing entre
estado de layout e estado de introspecção".

Após P205A–E:

- **Categoria B (runtime introspecção)**: 3 sub-fields
  identificados em P205A; **1 materializado**
  (`runtime.positions` → `SealedPositions`); 2
  permanecem como `HashMap` directo
  (`runtime.label_pages` deferred per P205D Caminho B;
  `runtime.known_page_numbers` é snapshot fixpoint,
  não consumer de tracking).
- **Categoria A (runtime puro), C (config), D
  (ambígua)**: 18 fields **inalterados** —
  fora-de-escopo F3 per ADR-0074. Candidatos a P210+
  se necessidade emergir.

Snapshot 2026-05-05 §6 hipótese "redução de aliasing"
**confirmada parcialmente**: `Introspector::position_of`
agora tem path tracked exclusivo via `inject_positions`
(P205C) — consumers podem migrar do dual path
`layouter.runtime.positions` direct access para path
Tracked exclusivo. Migração actual: zero consumers de
produção (P204F SKIP); infraestrutura disponível para
futuros consumers `here()`/`locate()` (DEBT-53/54
expansion).

---

## §9 Convenções consolidadas pela série

### §9.1 Lição P205C: "honestidade" considera contexto pré-existente

Spec P205C abriu Caminho C (adiar) como honestidade,
mas ADR-0074 já tinha fixado materialização. Caminho C
foi rejeitado correctamente por contradizer ADR
PROPOSTO. Honestidade ≠ adiar sempre que possível —
honestidade considera contexto: ADR pré-existente +
auditoria empírica + consumer real.

### §9.2 Lição P205D: condicional permite adiar sem contradição

ADR-0074 declarou P205D condicional explicitamente.
Spec P205D §8 anteviu Caminho B como hipótese mais
provável. Caminho B fixado em P205D C2 não contradiz
ADR — honra a condicionalidade negativamente. Pattern:
**ramo condicional na ADR ≠ ramo na spec do passo**.
Spec C13 reforça: "C2 fixa uma alternativa; Caminho B é
resposta empírica, não ramo".

### §9.3 Pattern `from_runtime` para sealing post-iteração

`SealedPositions::from_runtime(hashmap)` é constructor
canónico para sealing. Move semântico (não clone) do
HashMap interno; sealing point literal em
`Layouter::finish`. Pattern reaplicável a futuros
sub-stores sealed (P205D hipotético, outros futuros).

### §9.4 Distinção fecho estrutural vs final mantida

P205E fixou "Completo" porque P205D deferred é dentro
do escopo declarado (cond 3 aceita ambas as branches).
Contraste com P204H que fixou "estruturalmente
fechado" porque cond 9 PARCIAL exigia justificação
externa (DEBT-53/54). Distinção não é cosmética —
reflecte se as condições da ADR foram cumpridas
literalmente ou só estruturalmente.

### §9.5 Anotação cirúrgica em ADR superseded preserva chain-of-custody

ADR-0066 já SUPERSEDED-BY 0073 (P204H); P205E adicionou
anotação cirúrgica "Pendência §C6a fechada por F3
(P205B+C 2026-05-07)" no início. Conteúdo histórico
**não reescrito** (per padrão P201/P202). Anotação
estende o ponto final do chain-of-custody que
ADR-0066 originou: introspection runtime adiada → M8
adoptou comemo → F3 fechou §C6a.

---

## §10 Não-objectivos respeitados

P205A–E **não**:

- Reorganizou Categoria D (`page_config`, `locator`,
  `current_location`) — fora-de-escopo F3.
- Consolidou Categoria A (cursor as Point, cell_* as
  CellRect) — candidato a P210+.
- Restruturou o Layouter monolítico para emular
  arquitectura vanilla — magnitude XL+, fora-de-escopo
  (per `P205A.div-1`).
- Materializou `here()` / `locate()` stdlib —
  consumers futuros do impl real, não trabalho de F3
  (per P204F SKIP).
- Endereçou vanilla integration (DEBT-53/54) — pre-existing,
  fica para série dedicada P206+.
- Tocou em loop fixpoint TOC — hash convergence
  preservado (per ADR-0072).
- Modificou `runtime.label_pages` populated single-pass
  ou `extracted_label_pages` em `PagedDocument` — per
  P205D não-objectivos.
- Substituiu `SealedPositions` em P205C por wrapper
  `PagedTagIntrospector` — per `P205A.div-1`.

---

## §11 Sugestão para próximo marco arquitectónico

**Não-vinculativa** — depende de prioridades do humano.

Caminhos plausíveis após P205E:

1. **P206+ — Vanilla integration (DEBT-53/54)** —
   caminho identificado em P204H §6 e P204F.div-1.
   Magnitude XL+. Permitiria expandir corpus paridade
   introspection com `here()` / `locate()` reais
   testados via vanilla typst CLI. F3 infraestrutura
   (`SealedPositions` + `inject_positions`) é
   pré-requisito desbloqueado.

2. **P210+ — Refactor Categoria A/C/D do Layouter** —
   18 fields restantes do snapshot 2026-05-05. Ex:
   consolidar `cell_*` em `CellRect`, cursor em
   `Point`. Magnitude variável (S por consolidação,
   M agregado). Cristalino-only.

3. **Próximo marco arquitectónico não catalogado** —
   depende de mapa estratégico (Model Fase 2,
   table/figure-kinds/bibliography per blueprint
   §3.2 OPÇÃO A). Magnitude M+/XL.

4. **Pausa estratégica** — F3 fechou pendência de M8
   herdada; ponto natural para re-avaliar prioridades.

P205E **não decide** — reporta. Próximo passo é
escolha humana com base em retorno estratégico vs
custo de cada caminho.

---

## §12 Cross-references

- **Spec do passo P205E**:
  `00_nucleo/materialization/typst-passo-205E.md`.
- **ADR materializada**:
  `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
  (ACEITE final 2026-05-07).
- **ADR anotada**:
  `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (anotação F3 fecho §C6a 2026-05-07).
- **Diagnósticos da série**:
  - `00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md`.
  - `00_nucleo/diagnosticos/typst-passo-205A-diagnostico.md`.
  - `00_nucleo/diagnosticos/typst-passo-205B-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-205C-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-205D-inventario.md`.
  - `00_nucleo/diagnosticos/typst-passo-205E-inventario.md`.
- **Relatórios individuais**:
  - `00_nucleo/materialization/typst-passo-205A-relatorio.md`.
  - `00_nucleo/materialization/typst-passo-205B-relatorio.md`.
  - `00_nucleo/materialization/typst-passo-205C-relatorio.md`.
  - `00_nucleo/materialization/typst-passo-205D-relatorio.md`.
  - `00_nucleo/materialization/typst-passo-205E-relatorio.md`.
- **Blueprint actualizado**:
  `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0bis
  [P205E].
- **Predecessor série**: P204H (M8 estruturalmente
  fechado).
- **Pattern referência**: P204 consolidado
  (`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`).

---

## §13 Resumo executivo

F3 — Layouter sub-stores trackable com sealing
post-iteração — **fechado completo** em 2026-05-07 per
ADR-0074 ACEITE final. Materialização cirúrgica:
1 sub-store (`SealedPositions`) + injection mechanism
(`TagIntrospector::inject_positions`). Pendência
ADR-0073 §C6a fechada estruturalmente. P205D
(`SealedLabelPages` opcional) deferred com fundamento
empírico — vanilla também não trackeia label_pages
directamente; cristalino já tem rota equivalente via
`SealedPositions` + label registry. Tests workspace
1852 → 1860 (+8); 0 violations preservadas. 5
sub-passos sem inflação retórica.
