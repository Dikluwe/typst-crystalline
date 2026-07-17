# P737 — Paridade de produção: `counter`/`state` como valores-tipo

## Proveniência da medição (regra 2026-07-05)

- **Commit base:** `4c1812f2de6fd3244f86d3759144e1825deb49bb` ("P736: preenche hash do commit no relatório")
- **Commit da implementação:** `07110541e638e85eb8a61f819b1d539987370a77` ("P737: counter/state como valores-tipo (paridade vanilla)")
- **Estado na medição final:** working tree não commitado; `git diff HEAD --stat`: 9 ficheiros — `01_core/src/engine/eval/{closures,mod,tests}.rs`, `01_core/src/engine/stdlib/{counter,state}.rs` + `01_core/src/entities/{counter,state}.rs` (hash headers sincronizados) + L0 `counter.md`/`state.md` (+98/−8).
- **Hora da validação final:** 2026-07-13 ~21:06 (-03)
- **Binário vanilla de referência:** `lab/typst-original/target/release/typst`
- **Binário cristalino:** `./target/release/typst` (rebuild 16.8s após a implementação)

## Sonda (medida antes de decidir — ADR-0108)

| Sondagem | Vanilla | Cristalino (pré-P737) |
|---|---|---|
| `#type(counter)` / `#type(state)` | `type` / `type` | `function` / `function` |
| `#(type(counter("x")) == counter)` | `true` | `false` |
| `#(type(state("y", 0)) == state)` | `true` | `false` |
| `#repr(counter)` / `#repr(state)` | `counter` / `state` | `#counter` / `#state` |

Nota: as instâncias (`Value::Counter`/`Value::State`) já mapeavam para `Type::Counter`/`Type::State` em `type_of` (`entities/value.rs:360-361`) — a divergência era só no binding global e na chamabilidade do tipo.

## Decisão

- `counter`/`state` passam de `Value::Func` a `Value::Type(Type::Counter/Type::State)` no scope global.
- Chamabilidade mantida via o despacho de tipos chamáveis de P685 (`eval/closures.rs`): novos braços `Type::Counter → native_counter` e `Type::State → native_state`.
- Métodos de instância (P506, braço em `eval_func_call` sobre `Value::Counter`/`Value::State`) não são afetados — operam sobre as instâncias, não sobre o binding.

L0 atualizados: `counter.md`, `state.md` (hashes via fix-hashes; entidades `counter.rs`/`state.rs` sincronizadas).

## Testes (fail-first confirmado: 3/5 falhavam; os 2 de não-regressão passavam)

5 testes `p737_*` em `eval/tests.rs`: tipo `type`, igualdade `type(instância) == counter/state`, chamabilidade (`counter("x")` → `Value::Counter`, `state("y", 0)` → `Value::State`), repr verbatim, métodos de instância sem regressão (`c.step()`/`s.update(5)` → Content).

## Validação

| Verificação | Resultado |
|---|---|
| `cargo test -p typst-core p737` | **5 passed**, 0 failed |
| `cargo test --workspace` | **4755 passed**, 0 failed (pré-P737: 4750; +5) |
| `crystalline-lint .` | 0 violations, 0 drift |
| `cargo build --release` | OK (16.8s) |
| E2E (ficheiro do passo) | `type type true true counter state` — **idêntico ao vanilla** |
| cetz (150 dpi, mesmo ficheiro de P734/P736) | diff **0.1477%**, 1478/1535 px — números idênticos a P736, sem regressão |

## Achados

- Item "`counter` e `state` expostos como `function`" (aberto desde P731) **fechado**.
- Sem novos achados neste passo.
