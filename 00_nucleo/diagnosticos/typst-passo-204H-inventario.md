# P204H — Inventário empírico

**Data**: 2026-05-07.
**Cláusula**: C1–C3 do passo P204H.
**Pré-condição confirmada**: P204A–G concluídos; ADR-0073
mantém PROPOSTO; ADR-0066 ACEITE com nota "intermediário
até M8"; tests 1852 verdes; 0 violations.

---

## §1 C1 — Auditoria das 9 condições de ADR-0073

Listadas literalmente per ADR-0073 §"Plano de validação"
linhas 184–207.

| # | Condição | Estado | Evidência empírica |
|---|----------|--------|---------------------|
| 1 | **P204B materializado**: `#[comemo::track]` aplicado ao trait `Introspector`. Compila sem erros. `Send + Sync` bounds verificados. | ✅ **CUMPRIDA** | `01_core/src/entities/introspector.rs:40` (`#[comemo::track] pub trait Introspector: Send + Sync`). Sentinelas `p204b_trait_e_send_sync`, `p204b_dyn_trait_implementa_track`, `p204b_tagintrospector_pode_ser_tracked_via_dyn` (3 sentinelas P204B). Relatório `typst-passo-204B-relatorio.md`. |
| 2 | **P204C materializado**: Layouter ganha lifetime parameter; field `introspector` é `Tracked<'a, dyn Introspector + 'a>`. Consumers migrados (10 sites Layouter). | ✅ **CUMPRIDA** | `01_core/src/rules/layout/mod.rs:67` (struct Layouter com `'a` lifetime + Tracked introspector field). 2 sentinelas P204C. Relatório `typst-passo-204C-relatorio.md`. |
| 3 | **P204D materializado**: tipo `Position` em L1; `runtime.positions` populated; `position_of` retorna `Option<Position>`. | ✅ **CUMPRIDA** | `01_core/src/entities/position.rs` (struct Position com page+point). `01_core/src/entities/introspector.rs:70` (`fn position_of(&self, location: Location) -> Option<Position>`). 2 sentinelas P204D. Relatório `typst-passo-204D-relatorio.md`. |
| 4 | **P204E materializado**: `crystalline_evict(n)` wrapper em L4. | ✅ **CUMPRIDA** | `04_wiring/src/eviction.rs` (`pub fn crystalline_evict(max_age: usize)`). L0 prompt `00_nucleo/prompts/wiring/eviction.md` (hash `7ac7b48b`). 2 sentinelas P204E. Relatório `typst-passo-204E-relatorio.md`. ADR-0073 anota ✅ MATERIALIZADO 2026-05-06. |
| 5 | **P204F materializado**: 5-7 ficheiros corpus paridade novos cobrindo introspection features. | ✅ **CUMPRIDA** | 6 ficheiros em `lab/parity/corpus/visual/`: `outline-toc.typ`, `counter-heading.typ`, `figure-ref.typ`, `equation-ref.typ`, `cite-bibliography.typ` (+ `refs.yaml`), `query-metadata.typ`. 6 smoke tests `p204f_corpus_*_compila` em `03_infra/src/integration_tests.rs`. Relatório `typst-passo-204F-relatorio.md`. ADR-0073 anota ✅ MATERIALIZADO 2026-05-06. |
| 6 | **P204G materializado**: measurements internos com logging hits/misses. | ✅ **CUMPRIDA** | `03_infra/src/measurements.rs` (módulo L3 com `cache_stats`, `introspector_call_counts`, `reset`, `record_evict`, `CountingIntrospector`). L0 prompt `00_nucleo/prompts/infra/measurements.md` (hash `c89617ca`). 2 sentinelas P204G + 6 tests cláusula-C6. `04_wiring/src/main.rs:106` (logging opt-in `CRYSTALLINE_MEASUREMENTS=1`). Relatório `typst-passo-204G-relatorio.md`. ADR-0073 anota ✅ MATERIALIZADO 2026-05-07. |
| 7 | **Tests workspace verdes**: estimativa 1824 → 1830-1840. | ✅ **CUMPRIDA** (excedida) | Real: 1824 → 1852 (+28). Trajectória per sub-passo: B+3, C+2, D+7, E+2, F+6, G+8. Estimativa P204A foi conservadora; real superou sem regressão. `cargo test --workspace` 1852 passed; 6 ignored (pre-existing). |
| 8 | **Crystalline-lint 0 violations**. | ✅ **CUMPRIDA** | `crystalline-lint .` exit=0; "✓ No violations found". Mantido em todos os 6 sub-passos. |
| 9 | **Saída cristalino sanity-check vs vanilla nos 5-7 ficheiros corpus paridade — sem regressões observable**. | ⚠️ **PARCIAL** | Cristalino compila os 6 ficheiros corpus (6 smoke tests verdes). **Comparação vanilla deferred** per `P204F.div-1`: lab/parity harness vanilla não funcional (DEBT-53/54 pre-existing — não criado por M8). Spec P204A C13.1 anteviu observable harness; realidade pós-P204F é cristalino-only baseline. |

**Resumo**: 8/9 CUMPRIDAS, 1 PARCIAL (condição 9 — vanilla
comparison observable).

**Nota auditor**: condição 9 não é bloqueante para a decisão
arquitectural de ADR-0073. As 6 condições estruturais
(1–6) e as 2 quantitativas (7–8) confirmam que a adopção
de `#[comemo::track]` é viável, compatível, e estável.
Condição 9 valida *paridade externa observable*, distinta
de viabilidade arquitectural.

---

## §2 C2 — Forma de fecho fixada

**Etiqueta**: **"Estruturalmente fechado"**.

Justificação:

1. 8/9 condições CUMPRIDAS — todos os 6 sub-passos
   estruturais (P204B–G) materializados; tests verdes
   (excedidos vs estimativa); lint zero.
2. 1/9 condição PARCIAL (condição 9 — vanilla
   comparison) por bloqueio empírico pre-existente
   (DEBT-53/54).
3. Padrão estabelecido: M7 fechou como "estruturalmente
   fechado" em P192B (per ADR-0072) com modelo análogo —
   loops fixpoint funcionais + tests E2E + decisão
   intermédia documentada. M8 segue esse padrão.
4. "Fechado completo" exigiria todas as 9 condições
   integralmente. Inflar a etiqueta para "fechado
   completo" sem materializar a condição 9 seria
   acomodação cosmética — viola o "Erro a não repetir"
   do passo P204H §8.
5. "Estruturalmente fechado" é honesto: a estrutura
   arquitectural está fechada (paridade vanilla literal
   no padrão `#[comemo::track]`); a validação observable
   não foi consumada por bloqueio externo.

C2 fixa **uma** etiqueta: **"Estruturalmente fechado"**.

---

## §3 C3 — Caminho de resolução fixado

**Caminho A — Aceitar parcialmente**.

Critério aplicado:

- **Bloqueante?** NÃO. A condição 9 valida paridade
  externa observable; M8 mantém valor estrutural sem ela.
  As 6 features observable (track aplicado, Layouter
  Tracked, Position, evict, corpus cristalino,
  measurements) foram todas materializadas e testadas.
- **Justificável?** SIM. P204F.div-1 documentou que
  vanilla integration foi deferred per DEBT-53/54
  pre-existing. P204A C9 não pré-fixou "validação
  reduzida", mas P204F encontrou empíricamente que o
  harness vanilla está não-funcional desde antes da
  série P204. Não foi M8 que criou o gap.
- **Decisão consciente em P204A?** Parcialmente. P204A
  C13.1 assumiu observable harness disponível; realidade
  divergiu. Spec não pré-fixou alternativa, mas P204F
  registou divergência (`P204F.div-1`) com fundamento
  empírico. Caminho A respeita isso explicitando a
  excepção.

A clarificação inicial fixou transições "completas"
(ADR-0073 ACEITE final + ADR-0066 SUPERSEDED-BY).
Caminho A respeita isso documentando excepção sem
acomodação cosmética.

C3 fixa **uma** alternativa: **Caminho A**.

### Tensão registada explicitamente

A clarificação inicial deixou duas decisões que poderiam
contradizer-se:

- "Forma do fecho decidida no inventário" → eventualmente
  "estruturalmente fechado".
- "Transições completas" → ACEITE final + SUPERSEDED-BY.

Caminho A resolve a tensão **honestamente**:

- ACEITE final é viável porque ADR-0073 valida a
  *adopção da decisão arquitectural*, não a *paridade
  externa absoluta*. As 8 condições estruturais
  cumpridas suficiem para ACEITE.
- A excepção (condição 9 PARCIAL) é documentada no bloco
  "Validação P204A–G" da ADR-0073 com referência cruzada
  a `P204F.div-1`.
- ADR-0066 transita SUPERSEDED-BY 0073 porque a sua
  promessa (adopção comemo em M8) foi cumprida pelo
  caminho A — adoptado, com excepção registada.
- M8 fica "estruturalmente fechado" no blueprint, sem
  inflar para "completo".

Esta resolução cumpre a forma das transições da
clarificação **e** respeita a empírica do estado do
projecto. Não é compromisso entre as duas — é coerência
honest.

---

## §4 Decisões durante a leitura

### D1 — Trajectória de tests excedeu estimativa (sem regressão)

P204A estimou 1824 → 1830-1840 (+6 a +16). Real: 1824 →
1852 (+28). Não é regressão; é cobertura mais densa que
o esperado:

| Sub-passo | Estimativa P204A | Real |
|-----------|------------------|------|
| P204B | +2 a +3 | +3 |
| P204C | +0 a +2 | +2 |
| P204D | +2 a +4 | +7 |
| P204E | +1 a +2 | +2 |
| P204F | +5 a +7 | +6 |
| P204G | +0 a +5 | +8 |
| **Total** | **+10 a +23** | **+28** |

Não há regressão — todos os 1852 verdes; 6 ignored são
pre-existing pre-P204A.

### D2 — `P204F.div-1` é DEBT-53/54 herdada, não regressão M8

DEBT-53 (vanilla integration bloqueada por DEBT-54) e
DEBT-54 (vanilla workspace setup) foram registadas em
P151/P152 — antes de M8 começar. P204A C9 assumiu, sem
verificar empíricamente o estado actual do harness, que
a observação cristalino vs vanilla seria viável. P204F
descobriu empíricamente que não — registou a divergência
e prosseguiu cristalino-only.

Caminho A documenta isto sem inflar a obrigação de M8.
M8 não fechou a paridade vanilla porque a paridade
vanilla está bloqueada por DEBT pre-existing — fechá-la
é trabalho separado (sub-passo dedicado, sugerido em
P204G §6).

### D3 — Sentinelas "17 activas" preservadas

Per spec C8: 3 (P204B) + 2 (P204C) + 2 (P204D) + 2
(P204E) + 6 (P204F) + 2 (P204G) = **17**. Todas as 17
permanecem verdes em `cargo test --workspace`. P204H não
adiciona nem remove sentinelas (encerramento documental).

### D4 — Blueprint pré-data 2026-04-25 (~12 dias atrás)

`00_nucleo/diagnosticos/blueprint-projecto.md` está
datado 2026-04-25 com 1145 tests. Várias actualizações
desde então (P155→P204G; M5/M6/M7/M8/M9 progrediram).
P204H actualiza apenas a marca M8 (per C7); reescrita
ampla é fora-de-escopo (per §7 não-objectivos).

### D5 — Localização do blueprint

Encontrado em `00_nucleo/diagnosticos/blueprint-projecto.md`
(não em `00_nucleo/projecto/` como sugerido na spec
P204H §3 C7). Edição cirúrgica aplicada na localização
real.
