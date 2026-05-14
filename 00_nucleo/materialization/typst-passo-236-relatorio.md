# Relatório do passo P236 — `state_final(key)` stdlib func refino aditivo (Fase 5 Categoria D 1/? — pós `P236.div-1` divergência factual material)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-236.md`.
**Tipo**: refino aditivo subset pós-divergência factual material.
**Magnitude planeada**: M+ (~3-4h). **Magnitude real**: S (~45min
— audit C1 + 1 stdlib func + 6 tests + docs).
**Marco**: **`P236.div-1` divergência factual material
registada** (state runtime já materializado pre-P236 P171+
M9+M9c; ADR-0066 SUPERSEDED-BY 0073 desde 2026-05-07);
**decisão humana via questionário 4-opções → Opção 2**
(refino aditivo subset); pattern "L0 minimal para refactors"
N=6 → **7 cumulativo aplicação automática ADR-0080 EM VIGOR**;
pattern emergente "Divergência factual material registada
via Pxxx.div-1 + decisão humana pós-divergência" N=1
inaugurado P236.

---

## §1 O que foi feito

P236 limita-se a refino aditivo verdadeiro pós-divergência:
- **`native_state_final(key)` em `foundations.rs`** —
  stdlib func nova; reuso `Introspector::state_final_value`
  baseline P171; paralelo absoluto a `counter_final` P176.
- **Registo scope** `state_final` em `eval/mod.rs`.
- **L0 NÃO tocado** — ADR-0080 §"Escopo" Opção γ aplicação
  automática N=7 cumulativo.
- **ADR-0066 NÃO tocado** — status SUPERSEDED-BY 0073 preservado.
- 6 unit tests novos (cenários canónicos); workspace
  **2137 → 2143 verdes** (+6); 0 adaptações intencionais;
  0 regressões reais; 0 violations.

---

## §2 Auditoria pré-P236 + `P236.div-1` divergência factual material (C1)

**Audit C1 obrigatório CRÍTICO revelou contradição com
spec**:

**ADR-0066 status real**:
- Cadeia chronológica: PROPOSTO (2026-04-27) → ACEITE
  (P192B 2026-05-05 — "intermediário até M8") → **SUPERSEDED-BY
  0073** (P204H 2026-05-07 — M8 adoptou comemo) → F3
  fechou §C6a (ADR-0074 ACEITE P205B+C+E 2026-05-07).
- Spec P236 (linha 8): "transição ADR-0066 PROPOSTO →
  IMPLEMENTADO". **Impossível** — status SUPERSEDED é
  terminal.

**State runtime já materializado pre-P236 (M9+M9c)**:
- `Content::State { key, init }` (P171, M9 sub-passo 3).
- `Content::StateUpdate { key, update: StateUpdate }`
  (P171/P172).
- `entities/state_registry.rs` (P171) — `HashMap<String,
  Vec<(Location, Value)>>` completo.
- `entities/state_update.rs` (P171/P172) — `enum
  StateUpdate { Set(Value), Func(Func) }`.
- `entities/layouter_runtime_state.rs` (P190C/D).
- 3 stdlib funcs em `foundations.rs`:
  - `native_state(key, init)`.
  - `native_state_update(key, value)`.
  - `native_state_update_with(key, fn)`.
- Pipeline activo: `Introspector::state_final_value` +
  `state_value(key, location)` lookup; from_tags walk
  aplica updates.

**`P236.div-1` registada** — divergência factual material
entre spec e estado real. Spec assumia state runtime
ausente; audit revelou substancialmente materializado
P171+M9+M9c.

---

## §3 Decisão humana pós-divergência (Opção 2 do questionário)

Questionário 4-opções enviado ao humano:
- Opção 1: P236.div-1 + skip materialization.
- **Opção 2: Refino aditivo subset (4ª stdlib func)** ✓.
- Opção 3: Reescrever spec P236.
- Opção 4: Forçar materialização literal.

**Humano escolheu Opção 2** — adicionar UMA stdlib func
que materializa parte específica de D.1 não coberta pelo
M9 baseline.

**Escolha entre `state_at(key, location)` e `state_final(key)`**:
- `state_at` requer Location explícita (mais complexo).
- **`state_final` paralelo absoluto a `counter_final` P176**
  — mais imediato, semantic óbvia.
- Decisão: `state_final` primeiro; `state_at` candidato
  refino futuro.

---

## §4 Implementação `native_state_final(key)`

`01_core/src/rules/stdlib/foundations.rs`:

```rust
pub fn native_state_final(
    ctx:                &mut EvalContext,
    args:               &Args,
    _world:             &dyn crate::contracts::world::World,
    _current_file:      FileId,
    _figure_numbering:  Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key)] => {
            let value = ctx
                .introspector
                .state_final_value(key.as_str())
                .cloned()
                .unwrap_or(Value::None);
            Ok(value)
        }
        [other] => err(format!(
            "state_final() requer string como argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_final() requer 1 argumento (key), recebeu {}",
            args.items.len()
        )),
    }
}
```

**Reuso `Introspector::state_final_value`** baseline P171
(`entities/introspector.rs:415`) — wrapper trivial; sem
nova lógica algorítmica.

**Registo scope** em `01_core/src/rules/eval/mod.rs:601`:

```rust
scope.define("state_final", Value::Func(Func::native("state_final", native_state_final)));
```

**Imports** atualizados em `mod.rs:37` e `eval/mod.rs:559`.

---

## §5 8 decisões fixadas pós-divergência

- **Decisão 1** — Opção 2 humana (refino aditivo subset;
  rejeitar Opções 1/3/4).
- **Decisão 2** — `state_final` escolhido sobre `state_at`
  (mais imediato; paralelo `counter_final` directo).
- **Decisão 3** — Paralelo absoluto pattern `counter_final`
  P176 (Introspector wrapper trivial).
- **Decisão 4** — `Value::None` retornado se key inexistente
  (semantic distinto Value::Str("") `counter_final` —
  state pode ter qualquer Value type).
- **Decisão 5** — Iter 0 fixpoint (introspector vazio)
  retorna None.
- **Decisão 6** — 6 unit tests subset minimal cenários
  canónicos (sem layout E2E pois state não-renderiza).
- **Decisão 7** — **ADR-0066 NÃO tocado** (status
  SUPERSEDED-BY 0073 preservado; promoção impossível).
- **Decisão 8** — **Opção γ L0 NÃO tocado** — sétima
  aplicação automática ADR-0080 EM VIGOR (refino aditivo
  verdadeiro qualifica per ADR-0080 §"Escopo" line 66
  "stdlib func nova aditiva"; **NÃO excepção** como spec
  original sugeria).

**Spec original Decisão 8 invertida**: spec assumia P236
seria "primeira excepção justificada à aplicação automática
ADR-0080 EM VIGOR pós-P229" (não-incrementa N=6 → 6).
Audit C1 confirmou que refino aditivo verdadeiro qualifica
Opção γ ADR-0080 §"Escopo" literal — N=6 → **7 cumulativo
preservado**, NÃO excepção.

**Anti-inflação 28ª aplicação cumulativa** pós-P205D —
Opção 2 humana + `state_final` apenas (não `state_at`) +
sem `Value::State` + sem módulos novos + sem promoção
ADR-0066 + sem L0 tocado + sem Layouter field + sem
Content::StateUpdate refactor (já existe).

---

## §6 Resultados verificação + tests (C7+C9)

| Critério | Esperado spec | Real pós-divergência |
|----------|---------------|----------------------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2154 verdes | **2143 verdes** (1854+242+24+2+21) ✓ (6 novos vs 15-17 spec — refino aditivo subset) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" L0 não tocado | **"Nothing to fix"** ✓ (L0 automático N=7) |
| Adaptações pre-existentes | N=0-2 | **N=0** ✓ |
| Value variants | 55 → 56 spec | **55 preservado** (Value::State NÃO criado pós-divergência) |
| Content variants | 59 → 60 spec | **60 já-existente** (Content::StateUpdate baseline P171) |
| Stdlib funcs | 60 → 64 spec | **60 → 61** (+state_final apenas; +state/+state_update/+state_update_with já-existentes P171) |
| Layouter +1 field | sim spec | **NÃO** (state já em StateRegistry/LayouterRuntimeState pre-P190C/D) |
| ADR-0066 PROPOSTO → IMPLEMENTADO | sim spec | **NÃO** (SUPERSEDED-BY 0073 preservado) |
| L0 tocado partial | sim spec | **NÃO** (Opção γ aplicação automática) |
| Regressões reais | 0 | **0** ✓ |

**Tests P236** (6 unit; sem layout E2E):
- `p236_state_final_introspector_vazio_retorna_none` —
  iter 0 fixpoint Value::None.
- `p236_state_final_apos_init_retorna_init_value` —
  populate via `StateRegistry::init` → final = init.
- `p236_state_final_apos_updates_retorna_ultimo_valor` —
  3 updates sequenciais → último vence.
- `p236_state_final_key_inexistente_retorna_none` —
  outras keys populadas; key consultada não retorna None.
- `p236_state_final_arg_nao_string_retorna_err` —
  validação tipo arg.
- `p236_state_final_zero_args_retorna_err` — validação
  arity.

---

## §7 Inventário 148 footnote ⁵⁵ + ADR-0079 anotação Categoria D 1/? + ADR-0066 NÃO tocado (C11)

**Inventário 148**:
- Footnote ⁵⁵ adicionada (~165 linhas) documentando
  P236.div-1 + state runtime já-materializado pre-P236 +
  Opção 2 humana + `state_final` paralelo `counter_final`
  + 8 decisões pós-divergência + 5 patterns
  consolidados/inaugurados + ADR-0066 SUPERSEDED preservado.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco `### P236
  anotação — Categoria D sub-passo 1 (state runtime);
  P236.div-1 divergência factual material; ADR-0066
  SUPERSEDED; refino aditivo state_final pós decisão
  humana`.
- Status ADR-0079 mantido PROPOSTO (9/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓ +
  Categoria D 1/? + Categoria C 0/?**).

**ADR-0066 NÃO tocado** — status SUPERSEDED-BY 0073
preservado (P204H 2026-05-07 promoção SUPERSEDED é terminal;
chain ADR-0066 → ADR-0073 → ADR-0074 fechado P205B+C+E).

**ADR-0080 NÃO tocado** — refino aditivo qualifica Opção
γ §"Escopo" literal line 66 ("stdlib func nova aditiva");
N=6 → 7 cumulativo aplicação automática.

---

## §8 Próximo sub-passo

P236 fecha D.1 retroactivamente (state runtime materializado
pre-P236 + refino aditivo `state_final`). Próxima sessão
candidatos:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **D.1 refino estendido** | `state_at(key, label)` paralelo `counter_at` P177 (paridade vanilla `state.at(location)`) | XS-S (~30min-1h) | média (completa paralelo state↔counter) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b | L+ a XL (~10-20h) | baixa |
| **ADR meta admin XS** | Promoção formal pattern `.or()` resolution N=3 | XS (~30min) | média (consolidação meta paridade P229) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **D.1 refino estendido
state_at** (XS-S ~30min-1h) — completa paralelo absoluto
state↔counter; paridade vanilla `state.at(location)`;
extension trivial paralela a `state_final`. Alternativa:
**ADR meta admin XS** se humano priorizar consolidação meta.

**Decisão humana fica em aberto literal** pós-P236.

**Estado pós-P236**:
- Tests workspace: 2137 → **2143 verdes** (+6 P236).
- Content variants: 60 preservado (StateUpdate já-existia).
- Value variants: 55 preservado (State NÃO criado).
- **Stdlib funcs: 60 → 61** (+state_final).
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados (state runtime via
  StateRegistry/LayouterRuntimeState pre-P190C/D).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**
  (D.1 é Introspection refino, não Layout).
- Cobertura user-facing total: 67% preservada (refino
  qualitativo marginal).
- **ADRs preservadas**: PROPOSTO 12; EM VIGOR 29 (ADR-0080);
  IMPLEMENTADO 21; total 67. **ADR-0066 SUPERSEDED-BY
  0073 preservado** (não conta nos 67 — terminal).
- **Saldo DEBTs: 11 preservado**.
- **28 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=6 → 7 cumulativo** (P230+P231+P232+P233+
  P234+P235+**P236**) — pattern extremamente sólido
  empíricamente (sete consecutivas sem excepção).
- **Pattern "stdlib func runtime para final value lookup"
  N=1 → 2 cumulativo** (counter_final P176; **state_final
  P236**).
- **Pattern "Divergência factual material registada via
  Pxxx.div-1 + decisão humana pós-divergência" N=1
  inaugurado P236**.
- **Pattern "Spec materializada como refino aditivo
  subset pós-divergência factual" N=1 inaugurado P236**.
- **Pattern "State runtime materializado pre-P236
  reconhecido retrospectivamente como cumprimento ADR-0066
  via chain ADR-0073/ADR-0074"** — documentação corretiva.
- **Categoria D Fase 5 Layout: 1/? sub-passos materializados
  pós-divergência** (D.1 ✓ refino aditivo P236; D.2/D.3/D.4
  candidatos).
- **Fase 5 Layout candidata: 9/13-15 sub-passos
  materializados** (~60-69% cumulativo; **Categoria A
  100% + Categoria B 100% + Categoria D 1/?**).
