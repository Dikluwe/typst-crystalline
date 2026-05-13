# Relatório do passo P210A — Diagnóstico-primeiro Counter/State extras Q1=β

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-210A.md`.
**Tipo**: diagnóstico-primeiro reduzido (zero código tocado).
**Magnitude planeada**: S-M (~45 min). **Magnitude real**: S (~30min).
**Marco**: M9c (Bloco V — Counter/State extras forma minimal).

---

## §1 O que foi auditado

Mapeado empíricamente o gap entre Counter/State actual
cristalino e o target Q1=β (humano fixou em P207A C10:
materializar `counter.step()` / `counter.display(numbering)` /
`state.get()` here-aware como funcs stdlib separadas; **não**
rich types). Foco em 5 dimensões: counter actual, state actual,
vanilla methods, infra here-aware (P208B minimal), consumers
reais. Zero código tocado; 1 output (este).

---

## §2 Auditoria A1–A5

### A1 — `Counter` cristalino actual (CONFIRMADO)

Stdlib funcs counter em `01_core/src/rules/stdlib/foundations.rs`:

- `native_counter_at(key, label)` (P177) em linha 335 — retorna
  string formatada na Location associada ao label.
- `native_counter_final(key)` (P176) em linha 383 — retorna
  string formatada do estado final pós-walk.

Sub-store: `CounterRegistry` (per P207A A4) com métodos
`apply_hierarchical`, `apply_at`, `value_at`, `value_at_index`,
`format`.

**Sem** `counter_step()`, `counter_update()` ou `counter_display(numbering)`.

### A2 — `State` cristalino actual (CONFIRMADO)

Stdlib funcs state em `foundations.rs`:

- `native_state(key, init)` (P171) — registar state com init
  value.
- `native_state_update(key, value)` (P171) — emite update.
- `native_state_update_with(key, fn)` (P172) — variant
  callback (stub).

Sub-store: `StateRegistry` (per P207A A4) com métodos `init`,
`update`, `value_at`, `final_value`.

**Sem** `state_get()` here-aware.

### A3 — Vanilla `counter.step()` + `counter.display()` + `state.get()` (CONFIRMADO)

`lab/typst-original/.../introspection/counter.rs:486-513`:

```rust
#[func]
pub fn step(self, span: Span, #[named] level: NonZeroUsize) -> Content {
    self.update(span, CounterUpdate::Step(level))
}

#[func]
pub fn update(self, span: Span, update: CounterUpdate) -> Content { ... }
```

`counter.display(numbering)` em linha 379 — devolve `Content`
que durante layout resolve para string formatada na location
actual.

`state.get()` em `state.rs:255`:

```rust
#[func]
pub fn get(self, ...) -> SourceResult<Value> { ... }
```

Tracked com `engine.introspector.context.location()` —
**depende de current_location** para resolver "value at this
point".

**Crítico**: `counter.step()` **NÃO** depende de current_location
(emite `Content::CounterUpdate` que aplica em layout time);
`counter.display(numbering)` + `state.get()` **DEPENDEM** de
current_location (resolve "at this location").

### A4 — Cristalino infraestrutura here-aware (DIVERGÊNCIA)

`EvalContext.current_location: Option<Location>` field exposto
(P208B). Walk advance **não implementado** per P208B Opção i
minimal. `native_here()` retorna `Err` se None (P208B C2).

Implicação para Q1=β funcs:

- `counter.step()`: **NÃO** depende de current_location →
  materializável trivialmente sem infra adicional.
  `Content::CounterUpdate { key, action: CounterAction::Step }`
  variant existe em `entities/content.rs:201` desde P207-pre
  (per `apply_at`/`Step` pattern).
- `counter.display(numbering)`: **DEPENDE** de current_location
  → mock-testable só (paralelo a `native_here()`); zero
  consumers reais.
- `state.get()`: **DEPENDE** de current_location → mock-testable
  só; zero consumers reais.

### A5 — Consumers reais imediatos (CONFIRMADO)

Grep `counter.step`/`counter.display`/`state.get`/`native_counter_step`
em `01_core/src/`, `02_shell/src/`, `03_infra/src/`,
`04_wiring/src/`: **zero production matches** (1 match em
`introspect.rs:3449` é comentário sobre `state.get_flat`
método interno de `StateRegistry`, não o stdlib `state.get`
target).

Pattern consistente M9c: zero consumers reais imediatos
(P207D C1.1, P208B C1.3, P209D C1.3, agora P210A A5).

---

## §3 Decisões C1–C5

### C1 — Forma das funcs stdlib

Per Q1=β minimal:

- `native_counter_step(key)`: 1 arg `Value::Str(key)`; retorna
  `Value::Content(Content::CounterUpdate { key, action: Step })`.
- `native_counter_display(key, numbering)`: 2 args; depende
  de `ctx.current_location` para `formatted_counter_at`.
- `native_state_get(key)`: 1 arg; depende de
  `ctx.current_location` para `state.value_at`.

### C2 — Comportamento sem `current_location` populated

**Opção A** fixada — erro contextual paralelo a `here()`:
"`counter.display` chamado fora de contexto locatable —
current_location não populado (P208B minimal; captura
automática deferred)". Honest, simétrico a `here()`.
Aplicável a `counter.display` + `state.get`.

`counter.step()` **não cai neste caso** — não depende de
current_location.

### C3 — Caminho fixado: **Caminho 3 — subset minimal**

Justificação literal:

- **A5 zero consumers** → Caminho 2 full (3 funcs) tem
  custo M (~3h) sem benefício imediato.
- **A4 walk advance não implementado** → `counter.display` +
  `state.get` retornam só erros sem infra adicional;
  valor marginal (mock-testable apenas, paralelo a here()).
- **A3 counter.step trivial** → não depende de current_location;
  emite `Content::CounterUpdate` que aplica em layout time;
  ~30min materialização.

Decisão: materializar apenas `counter.step()`. Adiar
`counter.display`/`state.get` até walk advance estar
implementado (sub-passo dedicado pós-M9c quando consumer
real emergir, paralelo a `Content::Context` block deferred
em P208D).

**Pattern emergente "Caminho 3 — subset honest"**: 8ª
aplicação cumulativa do anti-inflação (acumulando P205D,
P207E, P208B C1, P208D, P209C-vazios, P209D C6, P209E C1.2,
**P210A C3**), mas distintamente: aceita materialização
parcial em vez de skip total.

### C4 — Plano P210B-C

**2 sub-passos** (não 4):

- **P210B** (S ~30min-1h): materializar `native_counter_step(key)`.
  +stdlib func + scope register + 3-4 tests (vazio, step
  básico, repeated step, dispatch error).
- **P210C** (S ~20-30min): encerramento série P210
  documental. ADR-0076 anotada (série P210 fechada com
  Caminho 3 subset; `counter.display`/`state.get` deferred
  com critério para reabrir); blueprint §3.0septies marca;
  relatório.

Sem P210D ou P210E. Série compacta.

### C5 — Magnitude agregada P210

**S-M (~1.5-2h)** total — 3 sub-passos (A + B + C).

Distribuição:
- P210A: S (~30min — concluído aqui).
- P210B: S (~30min-1h).
- P210C: S (~20-30min documental).

Total real estimado: ~1.5h. Abaixo do estimado P207A C5
(P210 série completa seria L); pattern Caminho 3 reduz
significativamente.

---

## §4 Magnitude agregada P210 série

**S-M (~1.5-2h estimado)**. 3 sub-passos.

Caveat: se P210B revelar que `Content::CounterUpdate`
emit não é trivial (alguma fricção com action variants
ou serialização), magnitude pode subir para M. Improvável
per A4 inspecção.

---

## §5 Plano P210B-C (resumo executável)

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P210B | Materializar `counter.step` | S (~30min-1h) | `native_counter_step(key)` em foundations.rs; scope register; 3-4 tests P210B. ADR-0076 §P210B anotado. Funcs display/get deferred documentadas. |
| P210C | Encerramento série P210 | S documental (~20-30min) | ADR-0076 série P210 transita "EM CURSO" → "✅ MATERIALIZADO"; bloco "Agregado série P210" com Caminho 3 e deferreds explícitos; blueprint §3.0septies marca; relatório resumo. |

**Pré-condições mantidas**:
- Trait `Introspector` mantém 26 métodos (P210 não estende
  trait — stdlib func).
- Regra empírica P207B §5 **não acionada**.
- Q4=β `query_count_before` continua adiado.

---

## §6 Próximo sub-passo

**P210B** — `native_counter_step(key)` materialização.

Pré-condição cumprida: P210A diagnóstico fechado; C1-C5
fixados; Caminho 3 subset.

Trabalho concreto P210B (preview):

- L1 `01_core/src/rules/stdlib/foundations.rs` — adicionar
  `pub fn native_counter_step(ctx, args, ...)` paralelo a
  `native_state_update`. Emite
  `Value::Content(Content::CounterUpdate { key, action:
  CounterAction::Step })`.
- L1 `stdlib/mod.rs` — +`native_counter_step` em re-exports;
  +3-4 tests.
- L1 `eval/mod.rs` — +import + `scope.define("counter_step",
  ...)`.

L0 prompts (eval.md/stdlib.md) **não modificados** —
convenção emergente P208B §3 (stdlib funcs P169+
inline-documentadas).

ADR-0076 mantém `PROPOSTO`. Estado M9c: 3 séries
fechadas (P207 + P208 + P209) + diagnóstico P210A
concluído. Pattern "Caminho 3 honest subset" formalizado.
