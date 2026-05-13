# Passo 210B — `native_counter_step(key)` stdlib

**Série**: 210 (sub-passo `B`).
**Marco**: M9c (Bloco V — Counter/State extras forma
minimal; subset Caminho 3 fixado em P210A C3).
**Tipo**: implementação stdlib trivial.
**Magnitude**: S (~30min-1h).
**Pré-condição**: P210A concluído; Caminho 3 subset
fixado; `counter.step()` materializável sem
current_location (per P210A A3); `Content::CounterUpdate`
variant pré-existe (per P210A A4 — confirmar em C1);
trait 26 métodos; stdlib funcs ~52; tests 1935 verdes;
0 violations.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Materializar `native_counter_step(key)` stdlib func.

Per P210A A3: `counter.step()` emite Content que aplica
em layout time. **Não depende de `current_location`**.
Distinto qualitativamente de `counter.display` +
`state.get` (deferred).

Reuso de dados P210A + trajectória M9c:

- Pattern stdlib `native_X(ctx, args, world, current_file,
  figure_numbering)`.
- `native_state_update(key, value)` (P171) em
  `foundations.rs` como template — emite Content
  análogo.
- Convenção emergente P208B §3: stdlib funcs P169+
  inline-documentadas; sem L0 separado.
- Scope register em `eval/mod.rs` per P208B + P208C
  pattern.

---

## §2 Cláusulas (4)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. **`Content::CounterUpdate` variant**: confirmar
   estrutura literal em
   `01_core/src/entities/content.rs`. Esperado:
   `CounterUpdate { key: ..., action: CounterAction }`.
2. **`CounterAction::Step` variant**: confirmar
   existência em `CounterAction` enum. P210A A4
   mencionou existência; verificar literal.
   - Se ausente, **fallback**: usar `CounterAction::Add(1)`
     ou similar; registar `P210B.div-N` documentando
     adaptação.
3. **`native_state_update` template**: confirmar
   localização + assinatura em
   `01_core/src/rules/stdlib/foundations.rs`.
4. **`Value::Content` variant**: confirmar que `Value`
   enum tem variant para Content (P179 pattern).

Se C1.2 falhar, registar `P210B.div-N` e fixar fallback.

### C2 — Materializar `native_counter_step`

**L0**: não modificar (convenção emergente P208B §3 —
stdlib funcs P169+ inline-documentadas).

**L1** — `01_core/src/rules/stdlib/foundations.rs`:

```text
pub fn native_counter_step(
    ctx:              &mut EvalContext,
    args:             &Args,
    _world:           &dyn World,
    _current_file:    FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    let key = /* args[0] espera Value::Str */;
    let content = Content::CounterUpdate {
        key:    key.into(),
        action: CounterAction::Step,
    };
    Ok(Value::Content(content))
}
```

Detalhes (parsing args[0], error messages, key tipo
exacto) decididos durante implementação.

**L1** — `01_core/src/rules/stdlib/mod.rs`:
- `+native_counter_step` em re-exports.
- +3-4 tests.

**L1** — `01_core/src/rules/eval/mod.rs`:
- +`native_counter_step` em import block.
- +`scope.define("counter_step", Value::Func(Func::native("counter_step", native_counter_step)))`.

### C3 — Tests

Tests dedicados (~3-4):

- `p210b_counter_step_basico` — `counter_step("foo")`
  retorna `Value::Content(Content::CounterUpdate { key: "foo", action: Step })`.
- `p210b_counter_step_arg_invalido` — `counter_step(42)`
  retorna `Err` (não é `Value::Str`).
- `p210b_counter_step_sem_args` — `counter_step()`
  retorna `Err`.
- `p210b_counter_step_multipla_invocacao` — duas
  invocações com mesma key produzem Content
  estructuralmente igual (sem state shared no nível
  stdlib func).

### C4 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1938+ verdes (1935 + 3+); 0 violations.

**Regra empírica P207B §5 não accionada** —
`counter_step` é stdlib func, não trait method. Trait
mantém 26 métodos.

Anotar ADR-0076 §P210B: `✅ MATERIALIZADO {data}` +
sumário (counter.step subset; display/get deferred per
P210A C3).

---

## §3 Output

1 ficheiro:
`00_nucleo/materialization/typst-passo-210B-relatorio.md`.

Estrutura conciso (~3-5 KB) com 6 §s padrão.

---

## §4 Não-objectivos

- `counter.display(numbering)` (deferred até walk advance
  per P210A C3).
- `state.get()` here-aware (deferred idem).
- Rich Counter/State types (Q1=β excluiu).
- Walk advance automático (deferred P208B).
- `query_count_before` (Q4=β deferred).
- Trait method extensions.
- L0 prompt novo (convenção emergente).

---

## §5 Riscos a evitar

1. **`CounterAction::Step` ausente**: P210A A4
   mencionou pre-existência mas inspecção real pode
   revelar nuance (e.g. variants `Set`/`Add`/`None`
   apenas). C1.2 verifica; fallback via div-N
   adapta assinatura à variant disponível.
2. **Key tipo**: vanilla `counter.step` usa
   `CounterKey` (struct ou enum); cristalino
   `Content::CounterUpdate` pode usar tipo distinto
   (String, EcoString, key handle). Adaptar
   empíricamente — não inventar tipo novo.
3. **Esquecer scope register**: stdlib func sem
   register não invocável. C2 nota explicitamente.
4. **Inflar com display/get**: P210A C3 fixou subset.
   P210B materializa só step. Display/get ficam para
   sub-passo futuro pós walk advance.
5. **Convenção L0**: manter sem L0 separado per P208B
   convenção emergente. Se humano pediu correção em
   sessão futura, pattern muda; por agora preserva.
