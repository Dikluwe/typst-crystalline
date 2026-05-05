# ADR-0072 — M7 fixpoint runtime estruturalmente fechado

**Estado**: ACEITE
**Data**: 2026-05-05
**Sub-passo**: P192B
**Pré-condição**: M5 universal completo (P200B) + M6 fechado completo (P190I) + M9 11/11.

---

## Contexto

M7 (loop fixpoint runtime) era marco arquitectural pendente para
resolver dependências reverse em introspection — page numbers em
TOC (forward refs); queries runtime stdlib (`query()`,
`counter.at()`, `here()`) que dependem de introspector populado de
iteração anterior.

**P192A diagnóstico revelou** que M7 estava **estruturalmente
fechado** por força de sequência incremental P174 → P175-P179 → M9
→ P190 série → P191 série, **sem ADR explícita**. P192A audita
estado existente; P192B (este passo) declara formalmente.

---

## Decisão

**M7 fechado estruturalmente** via dois loops fixpoint
complementares — não redundantes:

### 1. TOC fixpoint (`layout/mod.rs:1515`)

- Resolve forward refs em page numbers (DEBT-12 cobertura).
- Activo em produção quando `intr.kind_index.contains_key(&ElementKind::Outline)`.
- `MAX_ITERATIONS = 5` (paridade vanilla `MAX_ITERS = 5`).
- Convergência via `doc.extracted_label_pages == known_page_numbers`
  (page numbers map idempotente entre iterações).
- Documentos sem TOC: short-circuit 1 passagem única.

### 2. `run_fixpoint` (`introspect/fixpoint.rs:65`)

- Mecanismo opt-in para stdlib features (P175-P179 / M9).
- 626 LOC; `pub fn run_fixpoint`, `pub fn introspect_to_fixpoint`.
- `MAX_FIXPOINT_ITERATIONS = 5`.
- Convergência via `compute_tags_hash(&tags) == prev_tags_hash` (2
  hashes consecutivos iguais).
- Estruturalmente pronto; sem clientes runtime em produção (per
  docs P174 internas: "Mecanismo sem clientes em P174. Adopção
  planeada para P175+").
- 13+ tests E2E exercitam mecanismo + features stdlib.

### Distinção arquitectural — categorias de dependências reverse

| Loop | Categoria de dependência |
|------|--------------------------|
| TOC fixpoint | Page numbers reverse-deps (layout-time → introspection result) |
| `run_fixpoint` | Stdlib queries reverse-deps (eval-time → introspection result) |

**Não redundantes**. Os dois loops cobrem categorias distintas em
fases distintas do pipeline (eval / layout).

---

## Distinção crítica — fechamento estrutural

**M7 fechado estruturalmente, não arquiteturalmente definitivo.**

Hash-based convergence é **decisão intermédia viável**:
- Funciona empíricamente — tests E2E verdes.
- 2 loops complementares cobrem categorias distintas.
- Cap `MAX_ITERATIONS = 5` paridade nominal vanilla.

**Mas paridade vanilla typst exige `comemo::Track`** para:
- **Memoização cross-iteration**: evitar re-walk full por iteração;
  invalidação granular permite re-walks parciais.
- **Tracking granular de dependências**: cada query rastreia que
  sub-stores observou; mudança em sub-store não-observado não
  invalida resultado.
- **Performance comparável**: re-walk full × 5 iterações é caro;
  comemo permite cache válido em maioria dos casos.

**M8 introduzirá comemo** — adopção planeada como próximo passo
natural após M7 estrutural.

---

## Consequências

### Positivas

- **M7 declarado estruturalmente fechado em P192B**.
- Cristalino atinge **consolidação arquitectural intermédia**:
  M5 universal completo + M6 fechado + M7 estruturalmente fechado +
  M9 11/11.
- Pattern **"auditoria sobre estado existente"** documentado (1ª
  aplicação completa).
- M8 definido como adopção `comemo` para paridade vanilla.

### Neutras

- Performance hash-based convergence é aceitável para cristalino
  actual (tests verdes; sem queixas observáveis).
- Paridade vanilla típica imediata via comemo é desejável mas não
  bloqueante.

### Riscos

- **Risco baixo**: Se cristalino expandir features stdlib que
  exercitam `run_fixpoint` em produção real (não-tests),
  performance hash-based pode tornar-se gargalo. Mitigação: M8.

---

## Alternativas avaliadas

### Comemo imediato em M7 (rejeitado)

ADR-0066 já tinha decidido adiar comemo. M7 fecha hash-based como
mecanismo intermédio viável. Comemo planeado para M8.

**Razão para adiar**: comemo invariantes complexos; M5 universal
completo + M6 + M7 estruturalmente fechados primeiro permitem M8
enfocar exclusivamente em comemo adopção.

### Sem fixpoint (rejeitado)

Dependências reverse (TOC page numbers, stdlib queries) exigem
convergência iterativa. Sem fixpoint, forward refs ficam não
resolvidas.

### Loop único combinado (rejeitado)

Combinar TOC fixpoint + `run_fixpoint` num único loop adicionaria
complexidade. Os dois resolvem categorias distintas em fases
distintas (layout vs eval); separação é mais clara.

---

## Cross-references

- **P174** — `run_fixpoint` mecanismo (sub-passo M7 inicial).
- **P175-P179** — features stdlib via fixpoint (`query()`,
  `counter.at()`, etc.).
- **M9 11/11** — features stdlib finalizadas.
- **P190I** — M6 fechado completo; ADR-0070 ACEITE.
- **P191C** — ADR-0071 ACEITE (walk pipeline com Introspector
  accessible — pré-condição location-aware queries).
- **ADR-0066** — ACEITE em P192B com nota "intermediário até M8".
- **ADR-0068** — ACEITE (location-aware Layouter; pré-condição
  queries runtime location-aware).
- **M8** — adopção `comemo::Track` para paridade vanilla typst
  (próximo passo natural; ADR dedicada futura).

---

## Pattern emergente

### "Auditoria sobre estado existente vs planeamento de trabalho futuro"

P192A é **primeira instância documentada** deste pattern. Distinção
das auditorias planeadoras anteriores:

| Auditoria planeadora (P190A, P191A, P195A, etc.) | Auditoria sobre estado existente (P192A) |
|--------------------------------------------------|------------------------------------------|
| Audita para identificar trabalho a fazer | Audita estado já materializado |
| Produz ADR PROPOSTO + plano implementação | Produz declaração formal + ADR ACEITE retrospectiva |
| Sub-passos subsequentes implementam | Sub-passos subsequentes documentam |
| Magnitude inicial S-M; total M+/L | Magnitude inicial S-M; total S-M agregado |

**Pattern reaproveitável** quando trabalho cumulativo incremental
atinge fechamento estrutural sem ADR explícita. Aplicações futuras
prováveis: marco fechado por agregação de séries pequenas que
resolvem o trabalho em conjunto.

---

## Métricas marco P192

- **2 sub-passos** materializados (A diagnóstico + B declaração).
- **0 LOC produção** (P192A + P192B; passo declarativo).
- **0 LOC tests** (testes inalterados).
- **2 ADRs novas / transitadas**: ADR-0072 (NOVA ACEITE);
  ADR-0066 (PROPOSTO → ACEITE).
- **2 relatórios**: P192A relatório + P192-relatório-consolidado.
- **1 diagnóstico**: P192A.
- **Magnitude agregada**: S-M.
- **Tests workspace**: 1.802 (inalterados).

---

## Estado pós-P192B

- M5 universal completo (P200B): ✅
- M6 fechado completo (P190I): ✅
- **M7 estruturalmente fechado (P192B)**: ✅ — esta ADR.
- M9 11/11: ✅ (snapshot pré-P190).
- M8 (comemo): pendente — próximo passo natural.

**7 ADRs ACEITES no ciclo M5/M6/M7**: ADR-0066 (com nota
intermediário), 0067, 0068, 0069, 0070, 0071, 0072.

**Padrão diagnóstico-primeiro**: 33ª aplicação consecutiva.
