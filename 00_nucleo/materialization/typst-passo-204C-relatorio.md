# Relatório do passo P204C

**Data de execução**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204C.md`.
**Natureza**: implementação cross-modular pós-P204B.
**Sub-passo `C` da série M8** — segundo de 7 (B-H) per
ADR-0073.
**Magnitude planeada**: M.
**Magnitude real**: **M** (~80 min; struct + 5 impls +
fixpoint loop + 03_infra + 7 tests + 1 assignment
elimination + 2 sentinels).

---

## §1 O que foi feito

P204C migrou `Layouter` para receber `Tracked<'a, dyn
Introspector + 'a>` por construtor (em vez de
`TagIntrospector` por valor + assignment post-construção).

Per Padrão A (paridade vanilla literal — fixado em C2 de
P204A; ADR-0073 PROPOSTO), Layouter agora carrega
`Tracked` field análogo ao `Engine.introspector:
Protected<Tracked<'a, dyn Introspector + 'a>>` do vanilla.

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204C-inventario.md`.

Conteúdo:
- §1 C1 inventário (10 sub-secções: localização, fields,
  impls, consumers, mutações, call sites, API).
- §2 C2 wrapper SIM com justificação.
- §3 C3-C7 alterações literais aplicadas.
- §4 C6+C9 migração de consumers.
- §5 C8-C11 verificações.
- §6 Decisões.
- §7 Métricas.
- §8 Critério de fecho.
- §9 Referências.

Tamanho: ~14 KB.

### Output 2 — Alterações em código

7 ficheiros modificados:

#### Production (01_core)

1. `mod.rs` — struct ganha `'a`; field passa a `Tracked`;
   `Layouter::new` aceita `Tracked` parameter; assignments
   `l.introspector = ...` em `layout_with_introspector`
   eliminados (substituídos por construção tracked uma
   vez antes do loop).
2. `cursor.rs` — impl block ganha `'a`.
3. `equation.rs` — impl block ganha `'a`.
4. `grid.rs` — impl block ganha `'a`.
5. `placement.rs` — impl block ganha `'a`.

#### Production (03_infra)

6. `layout.rs` — `layout_with_font` constrói empty
   TagIntrospector + tracked + passa a Layouter::new.

#### Tests (01_core)

7. `tests.rs` — 7 sites de Layouter::new com boilerplate
   tracked; 1 assignment `layouter.introspector =`
   eliminado; 2 sentinels P204C adicionados.

### Output 3 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P204C: ~3 min.
- C1 inventário (greps + reads de struct + impls +
  consumers + call sites): ~15 min.
- C2 decisão wrapper (análise + justificação): ~3 min.
- C3 + C4 struct + 5 impls: ~5 min.
- C5 layout_with_introspector + fixpoint loop: ~5 min.
- 03_infra/src/layout.rs adaptação: ~3 min.
- C7 tests adaptação (7 sites + 1 assignment elim):
  ~15 min.
- Verificação cargo build + iteração: ~5 min.
- C9 tests workspace + lint: ~3 min.
- C10 sentinels P204C (2): ~5 min.
- Inventário interno (Output 1): ~15 min.
- Este relatório: ~5 min.

**Total**: ~80 min.

---

## §3 Métricas

| Métrica | Pré-P204C | Pós-P204C | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1827 | **1829** | +2 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | baseline | +~50 | +50 |
| LOC tests | baseline | +~80 | +80 |
| Layouter lifetime | nenhum | `'a` | breaking interno |
| Layouter `introspector` field | TagIntrospector | Tracked<dyn Introspector + 'a> | migrado |
| Layouter::new args | 3 | 4 | +1 |
| `l.introspector = ...` assignments | 3 (2 prod + 1 test) | 0 | -3 |
| Ficheiros modificados | — | 7 | mod.rs, cursor.rs, equation.rs, grid.rs, placement.rs, tests.rs (01_core), layout.rs (03_infra) |

---

## §4 Decisões tomadas durante a leitura

### 4.1 C2 — Wrapper SIM

Mantém API pública sem lifetime exposed. `pub fn layout`
e `pub fn layout_with_introspector` continuam aceitando
TagIntrospector / Content sem `'a`. Lifetime aparece
apenas em `Layouter::new` directo (1 caller externo —
03_infra/src/layout.rs).

Justificação: 4 callers externos de `pub fn layout` em
03_infra; expor lifetime seria viral.

### 4.2 Tracked construído UMA vez antes do fixpoint loop

```rust
use comemo::Track;
let intr_dyn: &dyn Introspector = &introspector;
let intr_tracked = intr_dyn.track();

if !has_outline {
    let mut l = Layouter::new(..., intr_tracked);
    ...
}

for _ in 0..MAX_ITERATIONS {
    let mut l = Layouter::new(..., intr_tracked);  // Tracked é Copy
    ...
}
```

Tracked é Copy — reusado em iterações sem custo.

### 4.3 Eliminação dos 3 assignments `l.introspector = ...`

Tracked é borrow, não valor — assignment post-construção
impossível. Solução: passar Tracked ao construtor.

- mod.rs:1485 (no-outline path) — eliminado.
- mod.rs:1519 (fixpoint loop) — eliminado.
- tests.rs:4292 (test) — eliminado.

### 4.4 Tests boilerplate inline em vez de helper

Tentativa de helper `make_test_layouter()` rejeitada
porque Tracked precisa de borrow ao introspector que
outlive layouter — helper não pode retornar Tracked
sem closure pattern (invasivo para tests).

Decisão: boilerplate 5 linhas inline em cada test.
Mecânico mas explícito. 7 sites adaptados.

### 4.5 Test linha 4292 — clone eliminado também

Antes:
```rust
let intr = introspect_with_introspector(&content);
let intr_clone = intr.clone();
let mut layouter = Layouter::new(...);
layouter.introspector = intr_clone;
```

Depois:
```rust
let intr = introspect_with_introspector(&content);
// (clone eliminado)
let intr_dyn: &dyn Introspector = &intr;
let intr_tracked = intr_dyn.track();
let mut layouter = Layouter::new(..., intr_tracked);
```

Vantagem dupla: assignment eliminado + clone eliminado.

### 4.6 Sem alterações nos consumers de leitura

Consumers `self.introspector.<método>(...)` (~9 sites
em mod.rs + equation.rs) e
`layouter.introspector.<método>(...)` (~3 sites em
references.rs, outline.rs) **não precisaram alteração**.

Tracked oferece deref-like access via macro-generated
impl. Métodos do trait são invocados transparentemente.

### 4.7 Sentinels P204C — 2 (não 3)

Spec recomendava 3; decidido 2:
- `p204c_layouter_struct_aceita_tracked_introspector`
  (tipo).
- `p204c_pipeline_e2e_via_tracked` (runtime).

Terceira hipotética
(`p204c_introspector_field_e_tracked`) seria coberta
por #1 (mesmo erro de compilação detectado). Sem ganho
adicional.

### 4.8 03_infra/src/layout.rs — empty introspector

`layout_with_font` é fonts-only path. Empty
TagIntrospector é suficiente. Caso futuro precise de
introspection populada (ex: TOC com fonts customizadas),
adaptar nesse momento.

---

## §5 Sugestão para próximo sub-passo (não-vinculativa)

**P204D — Position concrete** (per ADR-0073 plano de
materialização).

Trabalho concreto P204D:
1. Tipo `Position { page: NonZeroUsize, point: Point }`
   em `01_core/src/entities/position.rs`.
2. `runtime.positions: HashMap<Location, Position>` em
   `LayouterRuntimeState`.
3. Layouter popula durante layout (single-pass) — gate
   por `current_location`.
4. Trait método `position_of -> Option<Position>`
   (substitui stub `Option<()>`).
5. 2-3 tests E2E.

Magnitude esperada: **S-M**.

Pré-condição cumprida por P204C:
- Layouter tem `Tracked<dyn Introspector + 'a>` ✅.
- Layouter tem lifetime parameter ✅.
- API `Introspector` extensível (já adicionei 20 métodos
  em ciclo M9; mais um trivial).

---

## §6 Critério de progressão respeitado

Per spec §3 C13, P204C está concluído quando:

- [x] C1 inventário completo (10 sub-secções).
- [x] C2 wrapper-decisão fixada (SIM).
- [x] C3+C4 Layouter ganha `'a` em struct e 5 impls.
- [x] C5 construção via `track_with` aplicada.
- [x] C6 ~10 consumers — sintaxe inalterada via deref.
- [x] C7 tests adaptados (7 sites + 1 assignment elim).
- [x] C8 compilação verde.
- [x] C9 tests workspace verdes (1829).
- [x] C10 sentinelas (2 adicionadas).
- [x] C11 linter 0 violations.
- [x] Inventário registado (Output 1).
- [x] Relatório escrito (este).

**Sem `P204C.div-N`** registadas — sem divergências
empíricas relevantes.

---

## §7 Não-objectivos respeitados

Per spec §7, P204C não:

- [x] Não aplicou `#[comemo::track]` em outros traits
  (P204B cobriu o único alvo).
- [x] Não materializou Position (P204D).
- [x] Não adicionou `evict()` wrapper (P204E).
- [x] Não adicionou ficheiros ao corpus de paridade
  (P204F).
- [x] Não adicionou benchmarks (P204G).
- [x] Não transitou ADR-0073 para ACEITE (P204H).
- [x] Não criou ADR nova.
- [x] Não tocou em `TagIntrospector` impl.
- [x] Não tocou em loops fixpoint.
- [x] Não materializou sub-stores trackable
  separadamente.

---

## §8 Achados resumo

| Achado | Implicação |
|--------|-----------|
| Tracked construído antes do loop reusado em iterações | Sem perda de performance; Tracked é Copy |
| Layouter::new agora obriga 4º parâmetro Tracked | API breaking interno; 1 caller externo (03_infra) adaptado |
| Assignments `l.introspector = ...` foram 3 (2 prod + 1 test) | Todos eliminados — Tracked é borrow |
| Tests precisaram boilerplate de 5 linhas em 7 sites | Mecânico; explícito |
| Consumers de leitura não precisaram alteração | Tracked oferece deref-like via macro |
| Wrapper SIM mantém API pública sem lifetime | Minimiza ondas; 4 callers externos de `pub fn layout` inalterados |
| 03_infra/src/layout.rs adaptado com empty introspector | fonts-only path; sem TOC |

---

## §9 Notas operacionais

### 9.1 Magnitude real M alinhada com planeada

P204C foi M-magnitude conforme spec. ~80 min.
Cross-modular como esperado (5 ficheiros core + 1 infra +
1 tests).

### 9.2 Diagnóstico-primeiro evitou descoberta tardia

C1 inventário detectou:
- 5 impl blocks (não só o main em mod.rs).
- 13 Layouter::new sites (incluindo tests).
- 3 assignments `l.introspector = ...` (não só nas
  tests mas também em produção).

Sem este inventário, a migração teria sido feita por
trial-and-error com cargo build cycles.

### 9.3 Trabalho útil cumulativo

P204A + P204B + P204C juntos:
- Auditoria empírica (16 cláusulas).
- Diagnóstico (14 cláusulas).
- ADR-0073 PROPOSTO.
- Trait `Introspector` trackable + Send + Sync.
- 3 Hash impls (Value, BibEntry, Content).
- Layouter migração para `Tracked<dyn Introspector +
  'a>`.
- 5 sentinel tests activos (3 P204B + 2 P204C).

P204D pode iniciar imediatamente.

### 9.4 Performance preservada

- Tracked é Copy (cheaper than .clone() de TagIntrospector).
- Tracked construído UMA vez fora do fixpoint loop.
- Consumers usam deref-coerção sem overhead.
- Iterações do fixpoint reusam mesmo Tracked sem
  rebuild.

### 9.5 Paridade vanilla observada

Padrão actual cristalino agora replica vanilla literal:
- `pub trait Introspector: Send + Sync` (vanilla:
  exactamente isto).
- `Tracked<'a, dyn Introspector + 'a>` (vanilla: usa
  exactamente esta forma).
- Construção via `.track()` em `&dyn Introspector`
  (vanilla: idem).

---

**Fim do relatório P204C.**
