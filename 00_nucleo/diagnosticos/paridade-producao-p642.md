# Relatório de Paridade — P642

**Passo:** 642  
**Data:** 2026-07-09  
**Foco:** Propagar o erro do callback de `state.update(func)` em vez de o descartar silenciosamente.  
**Dependências:** P633 (caso 4 confirmado), P641 (fase e método de teste confirmados).

---

## 1. Sonda do vanilla

Comando:

```bash
cat > /tmp/p642-state-erro.typ <<'EOF'
#let s = state("x", 0)
#s.update(x => 1 / 0)
#context s.get()
EOF
lab/typst-original/target/release/typst compile /tmp/p642-state-erro.typ /tmp/p642-vanilla.pdf
```

Resultado:

```text
error: cannot divide by zero
  ┌─ ../../../../../tmp/p642-state-erro.typ:2:15
  │
2 │ #s.update(x => 1 / 0)
  │                ^^^^^

  while calling `get` at ../../../../../tmp/p642-state-erro.typ:3:9
    s.get()

Exit code: 1
```

**Conclusão:** o vanilla pára a compilação com erro. O cristalino deve propagar o erro, não emitir apenas um aviso.

---

## 2. Implementação

### 2.1 `01_core/src/rules/introspect/from_tags.rs`

- Adicionado `use crate::entities::source_result::SourceResult;`.
- Alterada a assinatura de `apply_state_funcs` para devolver `SourceResult<()>`.
- Substituído o `if let Ok(...)` por `match` que, em caso de `Err(diagnostics)`, devolve `Err(diagnostics)` imediatamente, interrompendo o processamento.
- Atualizados os testes unitários existentes para lidar com o novo tipo de retorno (`.expect(...)`).
- Adicionado teste unitário `func_eval_callback_erro_propaga_diagnostics` que confirma que o erro é propagado e que o estado não é actualizado.

### 2.2 `01_core/src/rules/introspect/fixpoint.rs`

A chamada a `apply_state_funcs` em `run_fixpoint` agora trata o erro:

```rust
apply_state_funcs(&tags, &mut introspector, engine, ctx)
    .map_err(FixpointError::Eval)?;
```

- Adicionado teste de integração `fixpoint_propaga_erro_state_update_callback` que constrói um `Content` com `state` + `state_update(StateUpdate::Func(...))` onde a callback devolve `Err`, e verifica que `run_fixpoint` devolve `FixpointError::Eval`.

### 2.3 `01_core/src/rules/introspect.rs`

Atualizados os três callers de `apply_state_funcs` nos testes `p173_cascade_engine_via_api_publica` e `p173_determinismo_func_eval` para propagar o resultado com `.expect("apply_state_funcs deve suceder")`.

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram** (3638 em `typst-core`, mais testes nos restantes crates).

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

---

## 4. Decisão

- O erro do callback de `state.update(func)` já não é descartado.
- A propagação segue o padrão já existente em `run_fixpoint` (`FixpointError::Eval`).
- O método de teste correcto é de integração/pipeline, conforme P641, porque o callback só é executado durante a introspecção.

---

## 5. Estado de fecho

- [x] Comportamento do vanilla confirmado — erro que pára a compilação.
- [x] Erro do callback propagado em `apply_state_funcs` (`from_tags.rs:64`).
- [x] Teste unitário para propagação de erro adicionado.
- [x] Teste de integração/pipeline via `run_fixpoint` adicionado.
- [x] Callers de `apply_state_funcs` atualizados.
- [x] `cargo test --workspace` sem regressões.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p642.md`.

---

## 6. Hash do commit

`f1353e1d2`
