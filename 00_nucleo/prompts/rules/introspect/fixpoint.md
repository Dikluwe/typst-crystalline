# Prompt L0 — `rules/introspect/fixpoint`
Hash do Código: 03cec7ed

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/introspect/fixpoint.rs`
**Criado em**: 2026-04-29 (P174 sub-passo .D — mecanismo LOOP_EXTERNAL para fixpoint)
**ADRs relevantes**: ADR-0066 (Introspection runtime)

---

## Contexto

`run_fixpoint` é o helper que orquestra um loop de fixpoint até convergência:

1. Coloca `TagIntrospector` da iter anterior em `EvalContext.introspector`.
2. Chama closure `eval_step` para produzir `Content`.
3. Walk + `from_tags` produzem `(state, introspector)` da iter actual.
4. Compara hash de tags vs iter anterior — se igual, converge; senão, repete.
5. Hard cap em `MAX_FIXPOINT_ITERATIONS = 5`.

**P174 entrega mecanismo sem clientes.** Caller actual (`introspect()` legacy + Layouter pipeline) não usa fixpoint. Adopção é P175+ quando primeira feature stdlib que depende de introspector ser materializada (`query`, `here`, `counter.at`).

Vanilla equivalente: `comemo::analyze::analyze` + memoization. Cristalino simplifica: loop explícito sem memoization (M7+ refino com comemo).

---

## Restrições Estruturais

- Camada **L1**: closure-based, sem I/O directa.
- `eval_step` é `FnMut` que recebe `&mut Engine + &mut EvalContext` e retorna `SourceResult<Content>`. Caller decide como evaluar (parse + eval, ou Content pré-construído para tests).
- Walk em `introspect.rs::walk` **NÃO modificado**.
- `introspect_with_introspector` signature **inalterada**.
- Determinístico: dado mesmo input, fixpoint converge no mesmo número de iters com mesmo resultado.

---

## Interface pública

```rust
use crate::entities::content::Content;
use crate::entities::counter_state_legacy::CounterStateLegacy;
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::rules::eval::EvalContext;

/// Hard cap de iterações. Vanilla usa 5; cristalino segue paridade.
pub const MAX_FIXPOINT_ITERATIONS: usize = 5;

/// Erro do loop de fixpoint.
#[derive(Debug)]
pub enum FixpointError {
    /// Loop excedeu `MAX_FIXPOINT_ITERATIONS` sem convergir.
    NotConverged,
    /// Closure `eval_step` retornou erro.
    Eval(Vec<SourceDiagnostic>),
}

pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
    eval_step: F,
) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;

/// **P175** — entry point semanticamente claro para introspecção com
/// fixpoint. Wrapper directo sobre `run_fixpoint`. Adopção opt-in
/// quando feature stdlib (e.g. `query`) requer introspector populado
/// de iter anterior.
pub fn introspect_to_fixpoint<F>(
    engine:    &mut Engine<'_>,
    ctx:       &mut EvalContext,
    eval_step: F,
) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;
```

---

## Semântica

- **Iteração 0**: `ctx.introspector = TagIntrospector::empty()` (default). `eval_step` é chamado; produz `content_0`. Walk + `from_tags` produzem `(state_0, introspector_0)`. `prev_tags_hash = Some(hash(tags_0))`.
- **Iteração N (N ≥ 1)**: `ctx.introspector = introspector_{N-1}.clone()`. `eval_step` produz `content_N`. Walk + `from_tags` produzem `(state_N, introspector_N)`. Se `hash(tags_N) == prev_tags_hash`, converge e retorna `(state_N, introspector_N)`. Senão, `prev_tags_hash = Some(hash(tags_N))` e repete.
- **Não-convergência**: ao atingir `MAX_FIXPOINT_ITERATIONS` sem hash igual ao anterior, retorna `Err(NotConverged)`.
- **Erro de eval**: closure retornar `Err(diagnostics)` propaga como `Err(Eval(diagnostics))` no primeiro tick que erra.
- **Doc sem queries**: closure retorna sempre mesmo Content → converge em 2 iter (1 produção + 1 confirmação).

---

## Algoritmo

```rust
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx:    &mut EvalContext,
    mut eval_step: F,
) -> Result<(CounterStateLegacy, TagIntrospector), FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>,
{
    let mut prev_introspector = TagIntrospector::empty();
    let mut prev_tags_hash: Option<u64> = None;

    for _iteration in 0..MAX_FIXPOINT_ITERATIONS {
        ctx.introspector = prev_introspector.clone();

        let content = eval_step(engine, ctx).map_err(FixpointError::Eval)?;

        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(&content, &mut state, &mut locator, &mut tags, None);

        let curr_hash = compute_tags_hash(&tags);
        let introspector = from_tags(&tags, Some(engine), Some(ctx));

        if let Some(prev_hash) = prev_tags_hash {
            if prev_hash == curr_hash {
                return Ok((state, introspector));
            }
        }

        prev_tags_hash = Some(curr_hash);
        prev_introspector = introspector;
    }

    Err(FixpointError::NotConverged)
}
```

**Nota convergência**: requer **dois** ciclos consecutivos com mesmo hash. Primeira iter nunca converge (sem `prev`). Doc trivial converge em 2 iter (iter 0 produz tags; iter 1 confirma).

---

## Tests obrigatórios

- `fixpoint_converge_em_doc_estavel`: closure retorna Content fixo → convergência em 2 iter.
- `fixpoint_excede_cap_oscilatorio`: closure depende de contador interno e oscila → `NotConverged`.
- `fixpoint_propaga_erro_eval`: closure retorna `Err` → `Eval(_)`.
- `fixpoint_introspector_actualiza_entre_iters`: closure observa `ctx.introspector` e regista — iter 1 vê iter 0 populado.

---

## Consumers

Nenhum em P174 (mecanismo sem clientes). Adopção planeada em P175+:
- `query()` / `here()` / `counter.at` — features stdlib que dependem de introspector populado.
- Layouter / pipeline migra para usar `run_fixpoint` quando features acima existirem.

---

## Sobre paridade

Vanilla orquestra fixpoint via `comemo::analyze::analyze`. Cristalino simplifica: closure + loop linear sem memoization. Custo: 2× eval no caso típico; até 5× para docs com queries instáveis. M7+ pode introduzir comemo se justificado.

**Determinismo**: walk + from_tags determinísticos (P163 + P173). Eval determinístico (Funcs sem side effects, vanilla proibition). Fixpoint herda determinismo.

---

## Resultado Esperado

- `01_core/src/rules/introspect/fixpoint.rs` — `MAX_FIXPOINT_ITERATIONS` const + `FixpointError` enum + `run_fixpoint` fn + 4 tests.
- Re-export em `01_core/src/rules/introspect.rs` (`pub mod fixpoint`).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-29 | P174 sub-passo .D: helper LOOP_EXTERNAL para fixpoint runtime | `fixpoint.rs`, `fixpoint.md` |
| 2026-04-29 | P175 sub-passo .E: entry point opt-in `introspect_to_fixpoint` | `fixpoint.rs`, `fixpoint.md` |
