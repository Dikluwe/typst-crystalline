# Historiograma dos passos do projecto Typst Cristalino

**Data de produção**: 2026-05-05.
**Geração**: Claude Code (LLM externa) executando o passo P201,
lendo: `00_nucleo/materialization/` (~534 ficheiros), ADRs em
`00_nucleo/adr/` (0001-0072 menos 0063), diagnósticos em
`00_nucleo/diagnosticos/`, historiograma anterior P156A (backup
em `00_nucleo/historiograma-passos.P156A.md`).
**Escopo**: passos P0 a P200 inclusive variantes (-v2/-v3 forks
P4-9; sub-passos P81.5/83.5/83.6/84.x/91_5/96.x/131A-156L/
157A-C/158A-C/159A-G/160A-B/181A-J/182A-F/183A-D/184A-F/
185A-E/186A-F/187A-B/188A-B/189A-B/190A-I/191A-C/192A-B/
193A-B/194A-B/195A-E/196A-C/197A-C/198A-D/199A-C/200A-C).
**Natureza**: meta-documento descritivo. Não é prescritivo
absoluto. Onde a evidência é fraca, mantém-se qualificador.
**Coexistência**: complementa `blueprint-projecto.md`
(snapshot estático "onde estamos") e
`auditoria-delta-P156A-P200.md` (delta focado P157+).
**Versão anterior**: P156A (cobriu P0-P155).

---

## Sumário executivo

Em ~5 meses (Janeiro 2026 a Maio 2026), 200 passos numerados
(mais ~80 sub-passos/variantes) materializaram um compilador
Typst cristalino com **1823 testes verdes**, **70 ADRs**
(60 + 10 novas no ciclo P156-P200; ADR-0063 não existe),
**0 violations** (`crystalline-lint .`), 9 marcos
arquitecturais fechados (M1-M7, M9, F1) e o roadmap Layout
Fase 2 a 72%.

A trajectória pós-P156A foi marcada por **três eixos
simultâneos**:

1. **Roadmap Model Fase 2** (P156B-P156L: Layout cobertura
   38%→72%; P157-P159G: Table foundations + Bibliography +
   figure-kinds).
2. **Pipeline Introspection runtime** (P160-P200): da
   abertura M1 (`CounterStateLegacy` paralelo) ao fecho M5
   universal (P200B), M6 (P190I) e M7 (P192B).
3. **Governança meta-arquitectural** (P156K, P185A, P191A,
   P195A, P192B): 6 ADRs novas ACEITES (0066, 0068, 0069,
   0070, 0071, 0072) + 2 EM VIGOR (0064, 0065) + 1 ADR
   reservada (0062 hayagriva).

Padrões emergentes principais do ciclo P156B-P200:

1. **Diagnóstico-primeiro consolidou-se como norma** —
   ~33 aplicações consecutivas declaradas (vs 7 em P156A);
   nenhum `*A` produziu materialização sem revelar informação
   relevante.
2. **Pattern ADR-0069 (post-recursion tag emission) com 5
   variantes operacionais** identificadas explicitamente:
   target não-locatable (P195D), content locatable directo
   (P196B), cenário α sem Tag pós-recursão (P197B), promote
   completo β (P198C), α por construção (P199B). P200B é
   "trabalho híbrido sem nova variante" — combinação de
   variantes existentes.
3. **Pattern ADR-0070 (eliminação write paralelo)** aplicado
   8× consecutivos na série P190B-I; struct
   `CounterStateLegacy` (16 fields → 0; eliminada).
4. **Pattern ADR-0071 (walk pipeline com Introspector
   acessível)** resolveu barreira arquitectural P190F (walk
   fn sem acesso a Introspector); from_tags::from_tags
   eliminado (-969 LOC).
5. **Pattern Layouter-runtime → struct dedicada** — aplicado
   2×: P190C (page tracking), P190D (is_readonly).
6. **Pattern auditoria sobre estado existente** (P192A) —
   auditoria revela que M7 já estava estruturalmente fechado
   por sequência incremental (P174→P179→M9→P190→P191), sem
   ADR explícita; P192B declara formalmente.
7. **Pattern substitution-with-fallback (M4-residual)** —
   aplicado 7× consecutivos (P168, P181G, P182D, P184D,
   P187B, P188B, P194B), sintaxe canónica
   `introspector.<primitive>().or_else(|| legacy.<primitive>())`.

Comparação directa com P156A:

| Métrica | P156A (2026-04-25) | P200 (2026-05-05) | Δ |
|---------|:------------------:|:-----------------:|:--:|
| Testes workspace | 1145 | **1823** | +678 |
| ADRs total | 60 | **70** (-1 slot 0063) | +10 |
| Violations | 0 | **0** | = |
| Marcos fechados | até Fase 1 Model | M1-M7, M9, F1, F3 parcial | +9 |
| Aplicações diagnóstico-primeiro | 7 | **~33** | +26 |
| DEBTs novos abertos no ciclo | — | 1 (DEBT-56 column flow) | — |
| ADRs revogadas no ciclo | — | 0 | — |

---

## §1 — Linha temporal completa

### §1.1 Tabela cronológica compacta — Fase Pré-P156 (extracto P156A)

Para detalhe completo P0-P155 ver
`00_nucleo/historiograma-passos.P156A.md` §1. Resumo agregado:

| Cluster | Range | Padrão dominante | Resultado |
|---------|-------|------------------|-----------|
| Fundação inicial | P0-P3 | tudo-num-passo | World, PackageSpec, tipos domínio |
| Turbulência | P4-P9 (+forks v2/v3) | reformulação | 5 passos com forks; estabiliza após P10 |
| Pipeline eval+visual | P10-P25 | diagnóstico-primeiro emergente | Module/Value/Frame/PDF; DEBT-1 declarada P22 |
| Cluster math | P26-P50 | diag-primeiro | Motor matemático completo |
| Introspection+show base | P51-P67 | misto | Counters/Labels/Refs/TOC/show base |
| Features visuais | P68-P80 | cascata DEBTs | Show, imagens, gráficos, grid |
| Page+auditorias | P81-P83.6 | gate stress | DEBT-38; auditoria 83.5 |
| Governança ADRs | P84.x (14 sub-passos) | governança | ADR-0032/33/34; convenção `-RN`; vocab |
| Comemo+atomização | P85-P95 | TDD+atomização | Route real; ADR-0036 |
| Reestruturação coesão | P96-P97 (12 sub) | governança→aplicação | ADR-0037 promovida; eval/parse/stdlib reorg |
| Sink+Engine+CLI | P98-P120 | descoberta empírica | ADR-0042/44/49; Engine; CLI completa |
| CLI flags+DEBT-1 | P121-P141 | sucessivo + diag-primeiro | 5 subsets XS; ADR-0052/53/55 |
| Fecho DEBT-1+paridade+Model | P142-P155 | fecho-DEBT + diag | DEBT-1/52 fecham; Fase 1 Model fecha |
| **Historiograma P156A** | P156A | meta-administrativo | Primeiro historiograma do projecto |

### §1.2 Tabela cronológica compacta — Ciclo P156B–P200 (delta novo)

Convenções: `S/M/L` = magnitude (Small ~1-2h, Medium ~2-4h,
Large ~4-8h+); `α/β` = cenários ADR-0069. Padrões abreviados:
`diag` = diagnóstico-primeiro; `mat-A` = materialização-pós-A;
`subst-fallback` = substitution-with-fallback;
`elim-paralelo` = ADR-0070 eliminação write paralelo;
`hibrido` = trabalho híbrido sem nova variante.

#### Layout F1+F2+F3 (P156B-P156L) — granular 1-2 features/passo

| # | Data | Mag | Marco | Padrão | Output | Notas |
|---|------|-----|-------|--------|--------|-------|
| 156B | 2026-04-25 | L0-puro | Layout admin | diag (8ª) | ADR-0061 PROPOSTO; DEBT-56 | Recálculo 38%→22% empírico; renumeração P157→P158 |
| 156C | 2026-04-25 | S agreg. | F1.1 | mat-A | Content::{Pad,Hide}; Sides<T> +27 tests | 1ª aplicação concreta ADR-0061 |
| 156D | 2026-04-25 | S agreg. | F1.2 | mat-A | Content::{HSpace,VSpace}; helper +20 tests | 33%→44% |
| 156E | 2026-04-25 | S+ agreg. | F1.3 | mat-A | Content::Pagebreak +22 tests | Halfway F1 |
| 156F | 2026-04-25 | S | F1.4 | inv-1º + diverg. | TransformMatrix::skew +16 tests | Reduzido S+→S; já existia desde P78 |
| 156G | 2026-04-25 | M+ | F2.1 | inv-1º | Content::Block +20 tests | 4 opções avaliadas; 56%→61% |
| 156H | 2026-04-25 | M | F2.2 | reaplicação | Content::Boxed +21 tests | Naming evita std::Box; 61%→67% |
| 156I | 2026-04-26 | M | F2.3 (fecha F2) | inv-1º | Content::Stack; entities/dir.rs +25 tests | Target 72% atingido |
| 156J | 2026-04-26 | S | F3.1 | diag (9ª) | Content::Repeat +7 tests | Algoritmo runtime out-of-scope |
| 156K | 2026-04-26 | meta L0-puro | governança | meta-ADR | ADR-0064 + ADR-0065 EM VIGOR | N=6/N=5 evidência |
| 156L | 2026-04-26 | M | refino F1 | refino-via-0065 | Content::Pad sides individualizadas | 10ª diag; divergência factual spec |

#### Model Fase 2 — Table+Figure-kinds+Bibliography (P157-P159G)

| # | Data | Mag | Marco | Padrão | Output | Notas |
|---|------|-----|-------|--------|--------|-------|
| 157 | ~04-26 | L0-puro | Model F2 admin | diag (11ª) | diagnóstico table foundations | 1ª aplicação ADR-0065 #5 |
| 157A | ~04-26 | M+ | Model F2.1 | mat-A | Content::Table; #table | 10ª materialização consecutiva |
| 157B | ~04-26 | M | Model F2.2 | mat-A | Content::TableCell | 1º ADR-0064 Caso A em Model |
| 157C | ~04-26 | M | Model F2.3 (fecha) | mat-A | Content::{TableHeader,TableFooter} | Saturação cross-domínio ADR-0064 |
| 158 | ~04-27 | L0-puro | figure-kinds admin | diag (13ª) | diagnóstico figure-kinds | Política nova: sem reservas pré-passo |
| 158A | ~04-27 | S | F-kind.1 | refino qual. | auto-detecção #figure | Subset minimal; recursão Sequence |
| 158B | ~04-27 | S | F-kind.2 | mat | rules/lang/figure_supplement.rs; state.lang | Fallback PT (não EN) |
| 158C | ~04-27 | S | F-kind.3 | refactor | Figure.kind Option<String> | 1º Caso A "estrito" refactor |
| 159 | ~04-27 | L0-puro | Bib admin | diag (14ª) | diagnóstico bibliography+cite | XL→Estrutura A adaptada |
| 159A | ~04-27 | M+ | Bib.1 | par acopl. | Content::{Bibliography,Cite}; entities/bib_entry.rs | 1ª aplicação ADR-0065 #2 isolada |
| 159B | ~04-27 | M- | família admin | diag amplo | diagnóstico expansão multi-feature | Análogo P156B; 4ª ADR-0065 #5 |
| 159C | ~04-27 | S | Bib.2 | refino enum | enum CitationForm; Cite.form | Caso A em refactor variant |
| 159D | ~04-28 | S | Bib.3 | refino entity | BibEntry +4 fields; builder | Pattern fluente |
| 159E | ~04-28 | S | Bib.4 | refino entity | BibEntry +2 fields (url/doi) | 20ª materialização consecutiva |
| 159F | ~04-28 | S | Bib.5 | refino comport. | bib_numbers; numbering [N] | Subpadrão #15 N=3 |
| 159G | ~04-28 | S | Bib.6 (fecha família) | refino entity | BibEntry +6 fields | 16 fields total; subpadrão #16 N=3 |

#### Pipeline Introspection runtime (P160-P200) — M1→M2→M3→M4→M5→M6→M7→M9

| # | Data | Mag | Marco | Padrão | Output | Notas |
|---|------|-----|-------|--------|--------|-------|
| 160 | ~04-29 | L0-puro | Introsp. admin | diag (24ª) | diagnóstico introspection | Cross-domínio Model→Introsp |
| 160A | ~04-29 | XS | governança | promote ADR | ADR-0066 PROPOSTO | ADR-0017 ocupado; usar 0066 |
| 160B | ~04-29 | — | descartado | — | (sem relatório) | Refactor antes de features (P161 declara) |
| 161 | 2026-04-30 | M | M1 (1/3) | preparação | rename CounterState→Legacy; 7 tipos novos | Não toca walk |
| 162 | 2026-04-30 | M | M1 (2/3) | refactor paralelo | walk emite Tags + extract_payload | walk 5 params; tags descartadas |
| 163 | 2026-04-30 | S | M1 (3/3) FECHA | validação E2E | 7 tests E2E; m1-lacunas-captura.md | M1 fecha; 3 lacunas iniciais |
| 164 | 2026-04-30 | S | M2 FECHA | extracção fn | is_locatable() exaust. 56 variants | Invariante is_locatable↔extract_payload |
| 165 | 2026-04-30 | M | M3 FECHA | sub-stores+trait | trait Introspector + TagIntrospector + 4 stores + from_tags | 4 ficheiros L1 + 4 L0 novos |
| 166 | 2026-04-30 | S | M4 FECHA | API expose | introspect_with_introspector público | 38 call-sites preservados |
| 167 | 2026-04-30 | L0-puro | M5 (1) | inventariar | inventario-consumers-counter-state-legacy.md | 6 consumers; +4 lacunas (#4-#7) |
| 168 | 2026-04-30 | M | M5 (2) | subst-fallback (1ª) | figure-ref via Introspector; layout_with_introspector | Pattern subst-fallback estabelecido |
| 169 | 2026-04-30 | M | M9 (1) | feature full-stack | Content::Metadata; #metadata; MetadataStore | Cobre ciclo completo |
| 170 | 2026-04-30 | S | M9 (2) | refactor sub-store | CounterRegistry hierárquico | Lacuna #5 fecha |
| 171 | 2026-04-30 | M | M9 (3) | feature full-stack | Content::{State,StateUpdate}; StateRegistry; #state | Func variant adiada |
| 172 | 2026-04-29 | S | M9 (4) | stub adiado | StateUpdate::Func stub no-op | Divergência da spec: stub vs cascade |
| 173 | 2026-04-29 | M | M9 (5) correctivo | cascade Engine | Engine + EvalContext em introspect_with_*; eval real Func | Walk preservado puro |
| 174 | 2026-04-29 | M | M7 (1) | mecanismo | run_fixpoint; FixpointError; MAX=5 | Sem clientes — adopção P175+ |
| 175 | 2026-04-29 | M | M9 (6) fixpoint use | feature stdlib | Selector::Kind; query(); introspect_to_fixpoint | Lacuna #7 NÃO fecha (Outline não-payload) |
| 176 | 2026-04-29 | S | M9 (7) | reuso | counter_final via formatted_counter | Sem trait method novo |
| 177 | 2026-04-29 | M | M9 (8) | location-aware | CounterRegistry.history; counter_at | Backwards compat preservada |
| 178 | 2026-04-29 | S | refino | promote locatable | ElementKind/Payload::Outline | Lacuna #7 FECHA |
| 179 | 2026-04-29 | S | M9 (9) | upgrade | Value::Location; query()→Vec<Location> | 3 stdlib tests adaptados |
| 180 | 2026-04-29 | L0-puro | M5 (3) | inventariar | inventario-bib-state.md | Recomendação Caminho A |
| 181A | 2026-05-01 | L0-puro | M9 lac.#6 (A) | diag | 6 cláusulas + plano P181B-J | — |
| 181B | 2026-05-01 | S | M9 lac.#6 (B) | sub-store | BibStore field | +8 tests |
| 181C | 2026-05-01 | S | M9 lac.#6 (C) | infra payload | ElementKind/Payload::Bibliography | +6 tests |
| 181D | 2026-05-01 | S | M9 lac.#6 (D) | promote | is_locatable(Bibliography) | +4 tests |
| 181E | 2026-05-01 | S | M9 lac.#6 (E) | from_tags | from_tags arm popula BibStore | +4 tests |
| 181F | 2026-05-01 | S | M9 lac.#6 (F) | trait API | bib_entry_for_key + bib_number_for_key | +3 tests |
| 181G | 2026-05-01 | M | M9 lac.#6 (G) | subst-fallback (2ª) | Layouter cite-arm | +6 tests |
| 181H | 2026-05-02 | S | M9 lac.#6 (H) | walk puro | walk arm Bibliography puro; layout() re-walks | Invariante walk puro restaurada |
| 181I | 2026-05-02 | S | M9 lac.#6 (I) FECHA | validação E2E | 5 tests E2E; lac.#6 FECHA | M9 atinge 10/11 |
| 181J | 2026-05-02 | — | encerramento | documental | consolidado P181 | — |
| 182A | 2026-05-02 | S | M9 lac.#4 (A) | diag | 6 cláusulas; vanilla sem numbering_active | Divergência arquitectural consciente |
| 182B | 2026-05-02 | S | M9 lac.#4 (B) | trait API | is_numbering_active(key) | +5 tests |
| 182C | 2026-05-02 | S | M9 lac.#4 (C) | extract+auto-init | SetHeadingNumbering arm; auto-init | Gate trivial inesperado |
| 182D | 2026-05-02 | S | M9 lac.#4 (D) | subst-fallback (3ª) | 2 consumers Layouter (heading+equation) | Equation fallback é caminho activo |
| 182E | 2026-05-02 | S | M9 lac.#4 (E) | validação E2E | 5 tests E2E heading_numbering | Re-update revela fallback funcional |
| 182F | 2026-05-02 | S doc | M9 lac.#4 (F) FECHA | documental | consolidado; lac.#4 FECHA | **M9 11/11 completo** |
| 183A | 2026-05-02 | L0-puro | M4-residual | diag | 12 read-sites; 5 áreas C1-C5 | Plano P183B-F; ajusta para granular |
| 183B | 2026-05-02 | S* | C1 attempt | subst-fallback | C1 heading prefix tentativa | (failed; documentada em P183C) |
| 183C | 2026-05-03 | bloq. | C2 attempt | auditoria-2-eixos | (zero código) | Bloqueio gate substancial em 2 eixos |
| 183D | 2026-05-03 | bloq. | C3 attempt | auditoria-2-eixos | (zero código) | Eixo 1 OK, eixo 2 falha (chave global) |
| 184A | 2026-05-03 | S doc | M5 desbloq. C3 | diag | 6 cláusulas + plano P184B-F | C3 mais barato individualmente |
| 184B | 2026-05-03 | S | C3 (B) | refinamento arm | from_tags Figure popula `figure:{kind}` | Convenção `kind:value` |
| 184C | 2026-05-03 | S | C3 (C) | trait+helper | figure_number_at_index; value_at_index | +8 tests |
| 184D | 2026-05-03 | S | C3 (D) | subst-fallback (4ª) | consumer C3 migrado | Heurística defensiva |
| 184E | 2026-05-03 | S | C3 (E) | validação E2E | 5 tests E2E | +5 tests |
| 184F | 2026-05-03 | S doc | C3 (F) FECHA | documental | consolidado | C3 funcional via Introspector |
| 185A | 2026-05-03 | M | M3 location-aware | diag+ADR | ADR-0068 PROPOSTO; 6 cláusulas | M3 escolhido vs M2 |
| 185B | 2026-05-03 | S | M3 (B) | trait methods | is_numbering_active_at; flat_counter_at | +10 tests |
| 185C | 2026-05-03 | M | M3 (C) | Layouter struct | current_location; Locator dedicado | +37 LOC vs 30 estimados |
| 185D | 2026-05-03 | S | M3 (D) | validação E2E | 4 tests sync Layouter↔walk | Sincronização literal validada |
| 185E | 2026-05-03 | S doc | M3 FECHA | governança | ADR-0068 ACEITE | Critério §3 cumprido |
| 186A | 2026-05-03 | S | Equation locatable | diag | 6 cláusulas + plano | Resolve eixo 2 do bloqueio P183C |
| 186B | 2026-05-03 | S | E (B) | infra payload | ElementPayload::Equation | +7 tests |
| 186C | 2026-05-03 | S | E (C) | extract | extract arm Equation | +3 tests |
| 186D | 2026-05-03 | S | E (D) | promote | is_locatable(Equation) | Δ tests = 0 |
| 186E | 2026-05-03 | S | E (E) | from_tags gated | gate location-aware | +4 tests; gate dormente |
| 186F | 2026-05-03 | S | E FECHA | validação+doc | consolidado + 4 tests E2E | Diferença P184: dormente em produção |
| 187A | 2026-05-03 | S | C1 (A) | diag | 6 cláusulas; site real mod.rs:345 | P183B aprendizado validado |
| 187B | 2026-05-04 | S | C1 (B) FECHA C1 | subst-fallback (5ª) | Layouter Heading via formatted_counter_at | Primeiro consumer C1 funcional |
| 188A | 2026-05-04 | S | C2 (A) | diag | 7 cláusulas; legacy get_flat | Forma migração espelha P187B |
| 188B | 2026-05-04 | S | C2 (B) FECHA M4-resid. | subst-fallback (6ª) | Layouter equation.rs:97 via flat_counter_at | Introspector dormente; fallback permanente |
| 189A | 2026-05-04 | S | M5 universal incrm. | diag | 7 cláusulas; 6 arms não-puros | Cadeia 4 pré-requisitos |
| 189B | 2026-05-04 | S | M5 — Outline migrável | walk puro parcial | Content::Outline migrado; 6 excepções E1-E6 | 1/6 arms puro |
| 190A | (s/data) | S-M | M6 diag | diag | 9 cláusulas plano β incremental | Reescrita do zero |
| 190B | (P190 série) | M | M6 (B) | elim-paralelo (1ª) | bib_entries+bib_numbers fields elim | -2 tests |
| 190C | (P190 série) | M | M6 (C) | LR→struct (1ª) | LayouterRuntimeState (page tracking) | 2 fields elim |
| 190D | (P190 série) | M | M6 (D) | extensão | is_readonly→LRS; has_outline elim | 2 fields elim |
| 190E | (P190 série) | S | M6 (E) | defer | (numbering_active defer) | Caso 1 parcial |
| 190F | (P190 série) | S | M6 — pause | barreira | (zero código) | Walk fn sem acesso Introspector; abre P191 |
| 191A | 2026-05-05 | S-M | walk redesign | diag+ADR | ADR-0071 PROPOSTO; 9 cláusulas | 4 opções; Opção A escolhida |
| 191B | 2026-05-05 | M+ | ADR-0071 (B) | walk redesign — prova | walk sig +TagIntrospector; from_tags::from_tags ELIMINADO | -699 LOC líquido; 25 call-sites |
| 191C | 2026-05-05 | S-M | ADR-0071 FECHA | walk redesign — completar | compute_labelled migrado | ADR-0071 ACEITE |
| 190G | (pós-191) | M+ | M6 (G) | retomar M6 | resolved_labels/headings_for_toc/auto_label_counter/numbering_active elim | 4 fields elim |
| 190H | (pós-191) | M | M6 (H) | extensão | figure_*/local_figure_counters elim; helper compute_figure orphan elim | 3 fields elim |
| 190I | 2026-05-05 | L | M6 FECHA | API breaking | hierarchical/flat/lang elim; **struct ELIMINADA**; Layouter counter elim; 4 helpers elim | ADR-0070 ACEITE; -10 tests; LOC líq -990; **F1 fechado** |
| 192A | 2026-05-05 | S-M | M7 auditoria | auditoria-existente | diagnóstico M7; 7 cláusulas → Estado A | 32ª diag-1º; M7 estruturalmente fechado |
| 192B | 2026-05-05 | S | M7 FECHA | declaração formal | ADR-0072 NOVA ACEITE; ADR-0066 PROPOSTO→ACEITE | Δ tests = 0 |
| 193A | 2026-05-04 | S | seq.§9 P189 (1A) | diag | 8 cláusulas | Passo 1 de 7 sequência |
| 193B | 2026-05-04 | S | seq.§9 P189 (1B) FECHA | sub-store | ResolvedLabelStore + trait method | +6 tests; replica P181B |
| 194A | 2026-05-04 | S | seq.§9 P189 (2A) | diag | 6 cláusulas | Site C4 confirmado |
| 194B | 2026-05-04 | S | seq.§9 P189 (2B) FECHA | subst-fallback (7ª) | Layouter references.rs C4 | +4 tests; sub-store P193B vazio até P195+ |
| 195A | 2026-05-04 | S | E4 (A) | diag+ADR | ADR-0069 PROPOSTO; 7 cláusulas+Locator | Bloqueador: extract puro sem state |
| 195B | 2026-05-04 | S | E4 (B) | infra variant | ElementPayload::Labelled | +5 tests |
| 195C | 2026-05-04 | S | E4 (C) | from_tags | from_tags arm Labelled popula 2 stores | +4 tests |
| 195D | 2026-05-04 | M | E4 (D) | walk arm pós-rec | helper compute_labelled (1º family); snapshot+find_map | **Variante operacional 1**: target não-locatable |
| 195E | 2026-05-04 | S doc | E4 FECHA | governança | ADR-0069 ACEITE | E4 fecha estruturalmente |
| 196A | 2026-05-03 | S | E2 (A) | diag | 7 cláusulas | E2 maior; 4 mutações |
| 196B | 2026-05-03 | M | E2 (B) | walk arm pós-rec | helper compute_heading_auto_toc (2º family); emitted_loc directo | **Variante operacional 2**: content locatable |
| 196C | 2026-05-03 | S | E2 — fecho parcial | documental | consolidado; 3/4 mutações fechadas | E2-residuo declarado (lacuna #3) |
| 197A | 2026-05-04 | S | E3 (A) | diag | 7 cláusulas; cenário α | C3 já migrado P184; refactor estilístico |
| 197B | 2026-05-04 | S/M | E3 (B) | helper extraction | helper compute_figure (3º family) | **Variante operacional 3**: cenário α |
| 197C | 2026-05-04 | S doc | E3 FECHA | documental | consolidado | E3 fecha estruturalmente |
| 198A | 2026-05-04 | S | E5+E6 (A) | diag | 9 cláusulas | 2 variantes na mesma série |
| 198B | 2026-05-04 | S | E5 (B) | trivial 1-linha | walk arm SetHeadingNumbering Tag emit | Cenário α activo desde P182C |
| 198C | 2026-05-04 | M | E6 (C) | promote completo | ElementPayload/Kind::CounterUpdate | **Variante operacional 4**: cenário β-promote |
| 198D | 2026-05-04 | S doc | E5+E6 FECHA | documental | consolidado | E1+E2-residuo restantes |
| 199A | 2026-05-04 | S | E1 (A) | diag | 7 cláusulas | Reserva 1 desde P189B |
| 199B | 2026-05-04 | M | E1 (B) | mat α por construção | Content::SetEquationNumbering + arms + Layouter | **Variante operacional 5**: α por construção; 0 excepções activas |
| 199C | 2026-05-04 | S doc | E1 FECHA | documental | consolidado | M5 universal a 1 passo do fecho |
| 200A | 2026-05-04 | S | E2-resid+lac.#3 (A) | diag | 9 cláusulas; correcção `usize` (não u8) | Trabalho híbrido |
| 200B | 2026-05-04 | M+ | E2-resid+lac.#3 (B) | hibrido | sub-store HeadingsForTocStore + Payload::HeadingForToc + walk arm + helper compute_heading_for_toc (4º family) + trait method (19→20) + consumer outline | **MARCO M5 universal completo**; auditor #1 pattern |
| 200C | 2026-05-04 | S doc | M5 universal FECHA | documental | consolidado | 0 excepções, 0 residuos, 0 pré-requisitos |

**Buracos de numeração detectados**:
- P160B: spec existe sem relatório (descartado por P161 com
  decisão arquitectural).
- P183B: relatório individual ausente; documentação retroactiva
  em P183C absorve.
- P190B-H: relatórios individuais não publicados como ficheiros
  separados; consolidado P190 absorve.

### §1.3 Narrativa cronológica por cluster — ciclo P156B-P200

**P156B-P156L (Layout cluster — 11 sub-passos, 2 dias)**:
recálculo empírico de cobertura (38%→22% em P156B); cadeia
granular P156C-I cobre F1+F2 com 1-2 features por passo;
P156J abre F3 (Repeat); P156K formaliza ADRs meta (0064
Smart→Option N=6, 0065 inventariar primeiro N=5); P156L
refino tardio. **Decisão humana 2026-04-25**: ADR-0061 mantém
PROPOSTO mesmo após F2 fechar.

**P157-P159G (Model Fase 2 cluster — 14 sub-passos, ~3 dias)**:
- Table foundations P157-P157C (saturação cross-domínio
  ADR-0064 com 4 casos canónicos atingida em P157C).
- Figure-kinds P158-P158C (auto-detect, supplement i18n,
  Option<String> refactor).
- Bibliography+Cite família P159-P159G (entity full-spec com
  16 fields; 6 sub-passos refino-entity replicados).

**P160-P166 (Pipeline Introspection — fundação, 7 passos)**:
P160 abre cross-domínio Model→Introspection; P161-P163 fecham
**M1** (walk emite tags + extract_payload); P164 fecha **M2**
(is_locatable função pública); P165 fecha **M3** (trait
+ TagIntrospector + 4 sub-stores + from_tags); P166 fecha
**M4** (introspect_with_introspector exposto).

**P167-P179 (M5 incremental + M9 Introspection vanilla, 13
passos)**: P167 inventário consumers (6 identificados; +4
lacunas #4-#7); P168 1ª aplicação **substitution-with-fallback**
(figure-ref); P169-P171 features full-stack (Metadata, State,
StateUpdate); P172-P173 cascade Engine correctivo; P174 cria
**mecanismo M7** (run_fixpoint MAX=5); P175-P179 features
M9 que usam fixpoint (query, counter_at, Outline, Value::Location);
P175 abre lacuna #7, P178 fecha-a.

**P180-P182F (M9 lacunas residuais — 17 sub-passos)**:
- P180 inventário bib-state.
- P181A-J série bibliography full-pipeline (10 sub-passos
  granulares; **fecha lacuna #6**).
- P182A-F série numbering_active (6 sub-passos; **fecha
  lacuna #4**; **M9 11/11 completo**).

**P183-P188B (M4-residual via Introspector — 17 sub-passos)**:
- P183A inventário 5 áreas C1-C5; P183B-D 3 tentativas
  iniciais (B retroactivamente "failed"; C/D bloqueios).
- P184A-F desbloqueio C3 (figure auto-number per kind) —
  **fecha C3**.
- P185A-E mecanismo M3 location-aware (ADR-0068 ACEITE) —
  resolve eixo 1 do bloqueio.
- P186A-F Equation locatable — resolve eixo 2 do bloqueio.
- P187A-B desbloqueio C1 (heading prefix) — **fecha C1**.
- P188A-B desbloqueio C2 (equation counter; Introspector
  dormente; fallback permanente) — **fecha M4-residual
  funcionalmente**.

**P189-P200C (M5 universal + M6 + M7 — sequência §9 P189,
~8 passos chave + M6 e M7 paralelos)**:
- P189A-B abre sequência; declara 6 excepções E1-E6.
- P190 série abre M6 (P190A-F primeiro lote; **pause em
  P190F** por barreira arquitectural).
- P191A-C ramo paralelo (ADR-0071 ACEITE) resolve barreira;
  walk fn ganha `&mut TagIntrospector`; from_tags eliminado.
- P190G-I retomam M6; **P190I fecha M6** (struct
  CounterStateLegacy ELIMINADA; ADR-0070 ACEITE; F1 fechado).
- P192A-B fecham M7 declaratoriamente (ADR-0072 NOVA ACEITE;
  ADR-0066 PROPOSTO→ACEITE).
- P193-P200B sequência §9 P189 fecha excepções E1-E6 +
  E2-residuo + lacuna #3:
  - P193 ResolvedLabelStore (passo 1).
  - P194 consumer C4 (passo 2).
  - P195 E4 Labelled — **ADR-0069 ACEITE**; **variante
    operacional 1**.
  - P196 E2 Heading auto-toc — **variante 2**.
  - P197 E3 Figure — **variante 3**.
  - P198 E5+E6 — **variante 4** (CounterUpdate promote).
  - P199 E1 SetEquationNumbering — **variante 5** (α por
    construção).
  - P200B fecha **M5 universal completo**; trabalho híbrido
    sem 6ª variante.

---

## §2 — Padrões agregados

### §2.1 Diagnóstico-primeiro (sufixo `A` ou equivalente)

**Lista cumulativa P0-P200** (consolidado P156A 1-7;
ciclo P156B-P200 8-33+):

| # | Sub-passo | Distância par | Descoberta |
|---|---|---|---|
| 1 | 131A | 0 | Lang precisa tipo dedicado |
| 2 | 132A | 0 | regex bloqueia FontList |
| 3 | 140A | 1 | Infra CIDFont já existia |
| 4 | 148 | — | Cobertura empírica vs declarada |
| 5 | 154A | 1 | Cobertura Model 38%→32-36% |
| 6 | 156A | — | Historiograma do projecto |
| 7 | 156B | 0 | Recálculo Layout 38%→22% empírico |
| 8 | 156J | 0 | Repeat algoritmo runtime out-of-scope |
| 9 | 156L | — | Pad já implementado puro desde P156C |
| 10 | 157 | 0 | Table foundations Model F2 |
| 11 | 158 | 0 | Figure-kinds política nova |
| 12 | 159 | 0 | Bibliography XL→Estrutura A |
| 13 | 159B | 0 | Família 159 multi-feature |
| 14 | 160 | 0 | Cross-domínio Introspection |
| 15 | 167 | — | Inventário 6 consumers + 4 lacunas |
| 16 | 180 | — | Bib-state inventário |
| 17 | 181A | 0 | Plano BibStore P181B-J |
| 18 | 182A | 0 | Vanilla sem numbering_active StyleChain |
| 19 | 183A | 0 | M4-residual 12 read-sites; 5 áreas |
| 20 | 184A | 0 | C3 desbloqueio mais barato |
| 21 | 185A | 0 | M3 location-aware ADR-0068 |
| 22 | 186A | 0 | Equation locatable resolve eixo 2 |
| 23 | 187A | 0 | C1 site real mod.rs:345 |
| 24 | 188A | 0 | C2 legacy get_flat |
| 25 | 189A | 0 | M5 universal 6 arms não-puros |
| 26 | 190A | 0 | M6 plano β incremental |
| 27 | 191A | 0 | Walk pipeline ADR-0071 |
| 28 | 192A | 0 | M7 estruturalmente fechado |
| 29 | 193A | 0 | ResolvedLabelStore |
| 30 | 194A | 0 | C4 site Layouter references |
| 31 | 195A | 0 | E4 Labelled bloqueador |
| 32 | 196A | 0 | E2 Heading 4 mutações |
| 33 | 197A | 0 | E3 Figure cenário α |
| 34 | 198A | 0 | E5+E6 dois cenários |
| 35 | 199A | 0 | E1 SetEquationNumbering reserva |
| 36 | 200A | 0 | E2-residuo+lac.#3 correcção `usize` |

**Estatísticas cumulativas**:
- **35 aplicações** com diagnóstico explícito formal
  (P156B-P200A); 7 prévias (P131A a P156A).
- Em **35 de 35 aplicações pós-P156A**, descobriu-se
  informação que alterou (ou validou definitivamente) a
  materialização planeada — **probabilidade observada de
  retorno alto: 100% (N=35)**, vs 100% (N=6) em P156A.
- A spec P201 declara "33 aplicações consecutivas". Discrepância
  marginal; pode incluir/excluir P156L (spec ambígua entre
  diag formal e refino-via-ADR-0065).
- **Em 27 de 35 aplicações**, o diagnóstico foi consumido
  pelo materialização-pós-A no mesmo dia (distância 0 ou 1).
- Pattern emergente: o sufixo `A` convergiu para "diagnóstico
  L0 + cláusulas + plano" como forma canónica.

### §2.2 ADR-0069 — Post-recursion tag emission

**5 variantes operacionais** (ciclo P195-P199; consolidação
P200B híbrido):

| Variante | Cenário | 1ª aplicação | Característica chave |
|----------|---------|--------------|----------------------|
| 1 | target não-locatable | P195D (Labelled) | snapshot+find_map para Location do recurse |
| 2 | content locatable | P196B (Heading auto-toc) | emitted_loc directo do walk top |
| 3 | cenário α (caminho activo desde P184/P182) | P197B (Figure) | declaração formal; helper-stylesheet sem Tag pós-recursão |
| 4 | cenário β-promote | P198C (CounterUpdate) | nova variant Payload + Kind + 2 arms |
| 5 | α por construção | P199B (SetEquationNumbering) | infra downstream pré-planeada activa imediatamente |

**Família ADR-0069 stylesheet** (4 helpers privados paralelos):
`compute_labelled` (P195D), `compute_heading_auto_toc`
(P196B), `compute_figure` (P197B), `compute_heading_for_toc`
(P200B). Mesmo formato; pattern reutilizável.

**P200B é trabalho híbrido sem nova variante**: combina
sub-store novo (P193B-style) + variante 2 (P196B) +
substitution-with-fallback (P184D). Sinaliza maturidade —
combinatória de variantes existentes cobre novos casos sem
expansão da família.

### §2.3 ADR-0070 — Eliminação write paralelo

**8 aplicações sequenciais** (série P190B-I):
P190B (bib_*), P190C (page_*), P190D (is_readonly/has_outline),
P190E (numbering_active defer), P190G
(resolved_labels/headings_for_toc/auto_label_counter/numbering_active),
P190H (figure_*/local_figure_counters), P190I
(hierarchical/flat/lang/struct + Layouter `counter` field).

**Pré-condição**: caminho Introspector activo. Forma:
substituir mutação legacy pela leitura via Introspector path;
fallback eliminado quando legacy é dead code factual
confirmado. **Resultado cumulativo**: struct
`CounterStateLegacy` 16 fields → 0 (eliminada); LOC líquido
série -990; -10 tests.

ACEITE em P190I (2026-05-05).

### §2.4 ADR-0071 — Walk pipeline com Introspector

**1 aplicação** (P191A-C). Resolveu **barreira arquitectural
P190F** (walk fn não tinha acesso a Introspector; helpers
walk-readers e walk-arm-gates bloqueados).

Mecanismo: `walk` fn ganha `&mut TagIntrospector` parameter;
`populate_intr_from_tag_start` helper centralizado;
`from_tags::from_tags` **eliminado** (969 LOC removidos);
25 call-sites adaptados.

PROPOSTO P191A; ACEITE P191C; pré-condição cumprida para
retomar P190G+.

### §2.5 Pattern Layouter-runtime → struct dedicada

**2 aplicações** (P190 série):
- **P190C** (1ª): cria `LayouterRuntimeState` com 2 fields
  page tracking (`label_pages`, `known_page_numbers`).
- **P190D** (extensão): movido `is_readonly`; `lang` defer
  para P190I.

Pattern emergente para extrair estado runtime do Layouter
para struct própria, mantendo Layouter slim. Field
`Layouter.runtime: LayouterRuntimeState` é o ponto de
agregação.

### §2.6 Pattern auditoria sobre estado existente (P192A)

**Variante reflexiva do diagnóstico-primeiro**: o passo `A`
audita o sistema *como ele está*, e o diagnóstico revela que
o marco **já está estruturalmente fechado** sem ADR explícita.
Resultado: passo `B` é declarativo (ADR formal + nota
intermediário).

Aplicado em P192 (M7); Δ tests = 0 em ambos os sub-passos.

Cláusula central: "M7 fechado pela sequência incremental
P174 → P175-P179 → M9 → P190 série → P191 série".

### §2.7 Pattern substitution-with-fallback (M4/M5-residual)

Estabelecido em **P168** (figure-ref); replicado consistentemente:

| # | Passo | Consumer | Notas |
|---|-------|----------|-------|
| 1 | P168 | figure-ref / C3 antecipado | Pattern estabelecido |
| 2 | P181G | cite-arm | Bibliography full-stack |
| 3 | P182D | heading + equation numbering | 2 consumers em 1 passo |
| 4 | P184D | C3 figure auto-number per kind | Heurística defensiva |
| 5 | P187B | C1 heading prefix | 1º consumer C1 funcional |
| 6 | P188B | C2 equation counter | Introspector dormente; fallback funcional permanente |
| 7 | P194B | C4 resolved label | Sub-store P193B vazio em produção até P195+ |

**Forma canónica**: `introspector.<primitive>().or_else(|| legacy.<primitive>())`.
Variações sintácticas: `unwrap_or_else` (P188B; legacy retorna
`usize`), `match { Some/None }` (P194B; `Option<&str>`
propagado).

### §2.8 Pattern auditor #1 — ajustar fixture vs violar restrição

Documentado explicitamente em P185D, P191B, P200B. Quando
uma restrição (ex: ADR-0068 sincronização Locator) seria
violada por adaptação simples de teste, o pattern auditor #1
prefere **ajustar o fixture** (ex: 5 tests P196B → 4 tests
por filtro `hash != 0` em P200B). Inverso (alterar produção
para acomodar test) seria violar restrição.

### §2.9 Pattern administrativo XS — promover ADR PROPOSTO de reserva

**N=2 explicitamente declarado** (P160A):
- P160A: ADR-0066 PROPOSTO (Introspection runtime adiada;
  reserva conceptual ADR-0017 preexistente).
- ADR-0062-create administrativo: `hayagriva` reserva → ADR
  formal.

Patamar formalização N=3-4 não atingido na janela.

### §2.10 Subpadrão #15 — Infraestrutura state lookup

Adições cumulativas a `CounterState`/`TagIntrospector`:
- N=1: state.lang (P158B)
- N=2: state.bib_entries (P159A)
- N=3: state.bib_numbers (P159F)

### §2.11 Subpadrão #16 — Refino entity por replicação

Pattern P159D (BibEntry +4 fields opcionais + builder)
replicado em P159E (+2) e P159G (+6). Mesma estrutura.
N=3 atingido.

### §2.12 Padrão "extract_length helper reusado"

**N=6 cumulativo** declarado em P156K (motivou ADR-0064
Smart→Option). Promoção a helper público diferida.

### §2.13 Análise comparativa face ao historiograma de P156A

**Padrões mantidos** (ainda activos):

| Padrão | Estado |
|--------|--------|
| Diagnóstico-primeiro formal | **Mantido + acelerado** (7→33+ aplicações) |
| Auditoria periódica como pivô | **Mantido** (P156B, P167, P180, P183A análogos a P83.5/P84.7/P125) |
| Pares A→B emparelhados | **Mantido + estendido** (séries A-Z em P156, P181, P182, P184, P185, P186, P190, P195-P200) |
| Cluster temático denso | **Mantido** (Layout P156B-L, Model P157-9G, Pipeline Introsp P160-200) |
| Promoção empírica de ADR | **Mantido** (ADR-0066 PROPOSTO→ACEITE em P192B; ADR-0068 PROPOSTO→ACEITE em P185E; etc.) |
| Spec partir de premissa errada | **Reduzido** (raríssimo no ciclo; P172→P173 é único caso pós-P156A) |
| Erro de camada arquitectural | **Não observado** no ciclo |
| Cascata de DEBTs sem fecho imediato | **Reduzido** (apenas DEBT-56 abre; sem fecho imediato; aceitável) |

**Padrões evoluídos**:

- **Diagnóstico-primeiro**: forma evoluiu de "diagnóstico
  embutido em passo substantivo" (P148, P154A) para "passo
  L0-puro dedicado com cláusulas numeradas" (P156B+) →
  agora canónico.
- **Inserção correctiva mid-série**: em P156A era P96.2 e
  P96.6; ciclo P156B-P200 não exibe inserção correctiva
  (séries fluem sem renumerar — e.g. P181A-J 10 sub-passos
  sem inserção; P190 série 9 sub-passos com pause-resume
  documentado mas sem renumeração).
- **Reformulação de série** (P148-P153 paridade suspensa em
  P156A): no ciclo emerge **pause-resume** documentado
  (P190F→P191→P190G) sem suspensão; ramo paralelo abre e
  fecha; série retoma. Forma mais sofisticada.

**Padrões novos emergentes (não-antecipados por P156A)**:

- ADR-0069 com 5 variantes operacionais (§2.2).
- ADR-0070 eliminação write paralelo (§2.3).
- ADR-0071 walk pipeline com Introspector (§2.4).
- Layouter-runtime → struct dedicada (§2.5).
- Auditoria sobre estado existente (§2.6).
- Substitution-with-fallback (7 aplicações; §2.7).
- Auditor #1 fixture vs restrição (§2.8).
- Trabalho híbrido sem nova variante (P200B; §2.2 nota).

**Padrões abandonados / não aplicáveis**:

- **Forks v2/v3**: continuam ausentes (P4-P9 fenómeno único).
- **DEBTs implícitos não-formalizados** (P156A §3.6 #4):
  ciclo formaliza explícitamente todas as decisões.
- **Cascata de DEBTs sem fecho** (P156A §3.6 #5):
  ciclo não exibe (DEBT-56 único; aceito como F3 long-term).

---

## §3 — Recuos e reformulações

### §3.1 Cancelamentos parciais / re-orientações

- **P160B descartado** por P161 ("refactor antes de features"):
  spec existe sem relatório; trabalho pivotou para o pipeline
  Introspection runtime via M1.
- **P172→P173 cascade adiado e retomado**: P172 spec
  especificava "Resolution + Engine cascade"; execução
  divergiu para stub; P173 é "continuação correctiva" e
  remove explicitamente test stub P172 ("codificava
  invariante incorrecto").
- **P183B documentado retroactivamente como "primitiva
  inadequada"**: P183C declara que `formatted_counter`
  (snapshot-final P170) não suportava re-update sequences;
  P185 série fornece primitiva location-aware; P187B fecha
  C1 com aprendizado validado.

### §3.2 Bloqueios formais (não recuos)

- **P183C, P183D**: bloquearam em `.B` com zero código
  tocado. Não são recuos — são bloqueios formais que
  motivam ramificações arquiteturais (P184/P185/P186).
- **P190F pause**: barreira arquitectural detectada;
  ramo paralelo P191A-C abre; **retomar P190G após P191C**.
  Reorganização de ordem cronológica documentada;
  tracker pause-resume em P191A.

### §3.3 Divergências da spec (consciente vs factual)

- **P156F divergência consciente** (S+→S; reduziu por
  inventário revelar unificação preexistente desde P78).
- **P156L divergência factual** (cobertura quantitativa
  não actualizada; pad já implementado puro desde P156C;
  refino além do mínimo).
- **P189B decisão CounterUpdate** (deferida intencionalmente
  para sub-passo seguinte; resolução: E6 excepção δ).
- **P192B correcção interpretativa**: P192A §4.3 inicialmente
  declarava "divergência arquitectural intencional sem
  comemo"; P192B corrige — divergência é intermédia, não
  permanente; comemo virá em M8.
- **P200A correcção `usize` vs `u8`**: spec inicial sugeria
  `Vec<(Label, Content, u8)>`; P200A `.1.4` corrige para
  `usize` empíricamente.
- **P200B correcção "helper sempre Some"**: P200A §2 cláusula
  5 sugeriu gate por `is_numbering_active("heading")`;
  P200B `.7` corrigiu — mutação 4 legacy é incondicional
  (push sempre); helper segue paridade.

### §3.4 ADRs revogadas / substituídas

**Nenhuma revogação no ciclo P156B-P200**. Todas as ADRs
novas (0061-0072) foram aditivas.

ADR-0017 mencionada como "reserva conceptual" sem ficheiro
físico; em P160A o slot 0017 é considerado ocupado e usa-se
0066 — não é revogação; é uso de número diferente para evitar
ambiguidade.

ADR-0063 não existe (slot reservado conceptualmente para
column flow per P160A; sem ficheiro físico).

---

## §4 — Cadeias de dependência

### §4.1 Cadeias declaradas (relatório posterior cita anterior)

**Pipeline Introspection runtime** (cadeia principal do
ciclo):

1. **M1**: P161 → P162 → P163 (declarado em cabeçalho
   P161 "M1 sub-passo 1/3").
2. **M3**: P165 (criação Introspector) → P166 (M4 expose) —
   capitaliza directamente sub-stores P165.
3. **M5**: P167 (inventário) → P168 (1ª migração C3) →
   P181G (cite via padrão P168).
4. **M9 lacuna #6**: P180 → P181A → P181B-J (cadeia
   interna; P181F estabelece API pattern para P182).
5. **M9 lacuna #4**: P181F (pattern API) → P182B; P182D
   capitaliza P171 + P173 + P185.
6. **M3 location-aware**: P185A (PROPOSTO) → P185B → P185C
   → P185D → P185E (ACEITE).
7. **C1 unblock**: P183B (tentativa falha) → P185 série →
   P187B (fecho C1; valida retroactivamente P183B).
8. **C2 unblock**: P183C → P186A-F + P185 série → P188B.
9. **C3 unblock**: P183D → P184A-F → P184D consumer.

**Sequência §9 P189 (M5 universal)**:
10. P189A → P189B → P193A-B → P194A-B → P195A-E (E4) →
    P196A-C (E2) → P197A-C (E3) → P198A-D (E5+E6) →
    P199A-C (E1) → P200A-C (E2-residuo + lac.#3 → fecha
    M5 universal).

**M6 série com pause-resume**:
11. P190A → P190B-F → **pause** → P191A → P191B → P191C
    (ADR-0071 ACEITE) → **retomar** P190G → P190H →
    P190I (ADR-0070 ACEITE; M6 fecha).

**M7 confirmação retroactiva**:
12. P174 (mecanismo) → P175-P179 (M9 features) → P190G/H/I
    (consumers) → P192A (auditoria revela fechado) →
    P192B (declaração ADR-0072 ACEITE).

**Layout F2/F3**:
13. P156B → P156C-I (Fase 1+2 cadeia granular) → P156J
    (1ª F3) → P156L (refino F1).

**Bibliography**:
14. P159 → P159A → P159B → P159C-G (cadeia).

### §4.2 Cadeias transversais ADR-0069

**P195D → P196B → P197B → P198C → P199B → P200B**: cadeia
de variantes operacionais ADR-0069 (1→2→3→4→5; P200B
híbrido sem 6ª).

### §4.3 Aprendizados retroactivos

**P183B aprendizado** atravessa P184/P185/P187: tentativa
falha em P183B fica documentada (sem relatório individual)
e é resolvida por correcção arquitectural (P185 fornece
primitiva location-aware; P187B fecha C1 com aprendizado
validado retroactivamente).

---

## §5 — Métricas cumulativas

### §5.1 Totais P0-P200

| Métrica | Valor | Fonte |
|---------|------:|-------|
| Total passos numerados (P0-P200) | 201 | inventário materialization/ |
| Total sub-passos/variantes adicionais | ~85 | sufixos a-l, .1-.10, A-J |
| Total ADRs criadas | 70 | 60 (P0-P155) + 10 (ciclo P156B-P200) |
| ADRs total número (com slot 0063 vazio) | 71 | inclui slot reservado |
| Slot ADR-0063 | (sem ficheiro) | reserva conceptual column flow |
| ADRs ACEITES (status formal) | 6 no ciclo | 0066, 0068, 0069, 0070, 0071, 0072 |
| ADRs EM VIGOR no ciclo | 2 | 0064, 0065 |
| ADRs PROPOSTAS pendentes | 3+ | 0061, 0062, 0067 |
| ADRs revogadas no ciclo | 0 | nenhuma |
| Tests workspace baseline P156A | 1145 | snapshot P155 |
| Tests workspace P200B | 1823 | empírico 2026-05-05 |
| Δ tests cumulativo no ciclo | +678 | crescimento monotónico |
| Aplicações diagnóstico-primeiro | ~35 | 7 prévias + 26-28 ciclo |
| Marcos fechados no ciclo | 9 | M1, M2, M3, M4, M5, M6, M7, M9, F1 |
| Crystalline-lint violations | 0 | empírico |
| LOC líquido série P190 | -990 | consolidado P190 |
| LOC eliminados from_tags::from_tags | -969 | consolidado P191 |

### §5.2 Trajectória de testes (cumulativa)

```
P3   ~ 69   testes
P25  ~ 368
P50  ~ 553
P75  ~ 720  (estimado)
P100 ~ 783
P125 ~ 870
P150 ~ 1095
P155 1145   (P156A baseline)
P156I 1296  (P156A nota cumulativa pós-Layout F2)
P181I ~1500 (estimativa)
P186F ~1801
P195E ~1838
P200B 1823  (estabilizou pós M6 cleanup; sentinelas legacy
             redundantes removidas)
```

Crescimento monotónico salvo pequeno Δ-15 ao fim do ciclo
(M6 cleanup eliminou tests sentinela já verificados via
caminho Introspector).

### §5.3 Trajectória ADRs cumulativa

```
P0   1 ADR
P25  ~28
P50  ~32
P95  ~37
P109 ~44
P120 ~51
P135 ~54
P155 60 ADRs (P156A baseline)
P156K 62
P160A 63
P185E 64
P191C 65
P192B 66
ciclo total: 70 ADRs (60 + 10 novas; 0063 não existe)
```

### §5.4 ADRs novas no ciclo (delta P156B-P200)

| ADR | Tópico | Proposta | Aceite/EM VIGOR |
|-----|--------|----------|-----------------|
| 0061 | Layout Fase X roadmap | P156B | mantém PROPOSTO (decisão humana 2026-04-25) |
| 0062 | hayagriva (reserva) | P156B nota | promovida em "ADR-0062-create" administrativo |
| 0063 | (não existe; slot reservado conceptual) | — | — |
| 0064 | Smart→Option/default (4 casos) | P156K | EM VIGOR imediato |
| 0065 | Inventariar primeiro (6 critérios) | P156K | EM VIGOR imediato |
| 0066 | Introspection runtime adiada | P160A | ACEITE P192B (com nota) |
| 0067 | Attribute grammar scoping | (pré-janela?) | PROPOSTO |
| 0068 | Layouter location-aware (M3) | P185A | ACEITE P185E |
| 0069 | Post-recursion tag emission | P195A | ACEITE P195E |
| 0070 | Eliminação CounterStateLegacy | P190 série | ACEITE P190I |
| 0071 | Walk pipeline com Introspector | P191A | ACEITE P191C |
| 0072 | M7 fixpoint runtime fechado | P192B (NOVA ACEITE) | ACEITE P192B |

---

## §6 — Convenções operacionais

### §6.1 Convenções activas (estabelecidas/confirmadas no ciclo)

| Convenção | Estabelecida | Estado |
|-----------|--------------|--------|
| Sub-passo `.A` = auditoria L0 / diagnóstico | pré-P156A (P131A); confirmada toda janela | activa |
| Sub-passos `*A` = diagnóstico-primeiro formal | pré-P156A; canónica P156B+ | activa |
| Sub-passos `*B+` = implementação sem condicionais | P181A+ | activa |
| 3 outputs padrão por passo (spec, relatório, diag) | P156A+ | activa |
| Sem código Rust nas specs | pré-P156A | activa |
| Distinção fecho estrutural vs arquitectural | P195E+, P196C+, P200C+ | activa |
| Preservação histórica de relatórios | P83.6+ | activa |
| Ficheiros relatório-consolidado por série | P181+ | activa |
| Numeração de ADRs sem reuso de slots | desde P84.8h; ADR-0063 slot vazio confirmado | activa |
| Pattern auditoria-2-eixos para gates substanciais | P183C/D | activa |
| Pattern pause-resume documentado | P190F→P191→P190G | activa |
| Trabalho híbrido sem promover variante nova | P200B | activa (1 caso) |
| Inventariar primeiro pré-decisão (ADR-0065) | P156K | activa (formalizada) |
| Smart→Option (ADR-0064) | P156K | activa (formalizada) |
| Cobertura empírica vs declarada (recálculo periódico) | P156B | activa |

### §6.2 Palavras banidas / inflação retórica (lista actual)

Termos a evitar nas specs e relatórios (per spec P201 §7):
- "patamar"
- "limiar"
- "consolidação"
- "deriva"
- "subpadrão" (como bandeira retórica)
- "cumulativo" (como bandeira retórica)
- "cross-domínio" (como bandeira retórica)
- "paridade observable" (como bandeira retórica)

### §6.3 Convenções abandonadas

**Nenhuma convenção foi explicitamente abandonada no ciclo
P156B-P200**. As convenções de P156A mantêm-se válidas e
foram estendidas.

---

## §7 — Coexistência e actualização

Este documento **complementa**:

- `00_nucleo/diagnosticos/blueprint-projecto.md` (snapshot
  estático "onde estamos").
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  (delta focado P157-P200; leitura rápida).
- `00_nucleo/historiograma-passos.P156A.md` (versão anterior;
  cobre P0-P155 com mais detalhe que esta).

**Princípio de actualização** (mantido de P156A):
regenerar quando passos significativos fechem (não a cada
passo). Frequência observada: ~45 passos entre P156A e P201
(2026-04-25 a 2026-05-05; ~10 dias). Frequência futura
sugerida: ao fechar M8 (ou Fase 3 do roadmap Layout).

**Próximas perguntas que este documento deveria responder em
revisões futuras**:
- Como M8 reorganiza o stack runtime (substituir hash-based
  convergence por comemo intermediário→permanente).
- Padrões emergentes de M8 (com N suficiente).
- Estado pós-fechamento de DEBT-56 column flow.
- Promoção (ou não) de ADR-0061 Layout Fase X.

---

## §8 — Referências

- `00_nucleo/diagnosticos/blueprint-projecto.md`
- `00_nucleo/historiograma-passos.P156A.md` (backup
  histórico).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
- `00_nucleo/adr/README.md` (índice canónico ADRs).
- `00_nucleo/DEBT.md` (autoritário para DEBTs).
- `00_nucleo/materialization/typst-passo-201.md` (spec deste
  passo).
- `00_nucleo/materialization/typst-passo-201-relatorio.md`
  (relatório da execução).
- Consolidados ciclo (`typst-passo-{181,182,184-200}-relatorio-consolidado.md`).
- ADRs do ciclo: `typst-adr-{0061,0062,0064-0072}-*.md`.
- Diagnósticos do ciclo: `00_nucleo/diagnosticos/diagnostico-*-passo-1[5-9][0-9]*.md`.
