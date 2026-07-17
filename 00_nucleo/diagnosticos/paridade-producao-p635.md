# Relatório de Paridade — P635

**Passo:** 635  
**Data:** 2026-07-09  
**Foco:** Implementar o mecanismo `FlowEvent` para `#break`, `#continue` e `#return` funcionarem de facto no cristalino.  
**Hash do commit:** b61c4bb0b

---

## Resumo

O cristalino tinha sintaxe para `#break`, `#continue` e `#return`, mas nunca implementou o mecanismo que faz estas instruções afectarem o fluxo de execução. Até ao Passo 634, qualquer ocorrência no dispatcher `eval_expr` era convertida num erro (fora de contexto) ou, antes disso, silenciosamente ignorada. Este passo introduz o `FlowEvent` equivalente ao vanilla (`typst-eval/src/flow.rs:14-22`) e propaga-o pelos ciclos (`#for`/`#while`), funções (`apply_closure`), blocos de código e o entrypoint `eval`.

---

## Sonda

### Mecanismo do vanilla

- `FlowEvent` definido em `lab/typst-original/crates/typst-eval/src/flow.rs:14-22`:
  - `Break(Span)`
  - `Continue(Span)`
  - `Return(Span, Option<Value>, bool)`
- Transporte: campo `vm.flow: Option<FlowEvent>` em `lab/typst-original/crates/typst-eval/src/vm.rs:20`.
- Consumo em ciclos: `flow.rs:88-96` (while) e `flow.rs:134-142` (for).
- Consumo em funções: `call.rs:698-706`.
- Conversão de evento residual em erro: `lib.rs:88-91`.

### Estado do cristalino antes da implementação

- `01_core/src/engine/eval/control_flow.rs:41-142`: `eval_while` e `eval_for` avaliam o corpo sem verificar sinalização de paragem.
- `01_core/src/engine/eval/closures.rs:106-179`: `apply_closure` avalia o corpo da closure sem verificar `Return`.
- `01_core/src/engine/eval/mod.rs:826-841` (P634): `LoopBreak`/`LoopContinue`/`FuncReturn` produzem erros byte-idênticos, mas uso legítimo dentro de ciclos/funções também falha.

### Testes directos antes da implementação

Usando o binário `target/release/typst`:

| Caso | Esperado | Obtido |
|------|----------|--------|
| `#for i in range(10) { if i == 3 { break } str(i) }` | `012` | `0123456789` |
| `#for i in range(5) { if i == 2 { continue } str(i) }` | `0134` | `01234` |
| `#let f(x) = { if x < 0 { return "negativo" } "positivo" } #f(-5) #f(5)` | `negativo positivo` | `positivo positivo` |

Confirmação: o mecanismo estava completamente ausente.

### Corpus

Nenhum documento em `lab/parity/corpus/` usa as keywords `#break`/`#continue`/`#return`. As ocorrências de `break`/`continue`/`return` são palavras normais de texto ou `#colbreak()`; portanto, não há risco de regressão nos testes de corpus existentes.

---

## Implementação

### Ficheiros alterados

1. **`01_core/src/engine/eval/flow.rs`** (novo): tipo `FlowEvent` + `forbidden()`.
2. **`01_core/src/engine/eval/mod.rs`**:
   - declara `mod flow; pub use flow::FlowEvent;`
   - adiciona `pub flow: Option<FlowEvent>` a `EvalContext`
   - `LoopBreak`/`LoopContinue`/`FuncReturn` definem `ctx.flow` em vez de erro
   - `CodeBlock` interrompe iteração se `ctx.flow` ficar `Some`
   - `eval_with_full_error` converte evento residual em erro
3. **`01_core/src/engine/eval/control_flow.rs`**:
   - `eval_while` e `eval_for` consomem `Break`/`Continue`/`Return`
   - `eval_conditional` marca `Return` como `conditional`
4. **`01_core/src/engine/eval/closures.rs`**:
   - `apply_closure` consome `Return` e devolve o valor; `Break`/`Continue` dentro de função produzem `forbidden()`
5. **`01_core/src/engine/eval/tests.rs`**: 9 novos testes P635.
6. **`00_nucleo/prompts/engine/eval.md`**: secção §P635 adicionada; hash atualizado para `ea93fd36`.

### Decisão técnica: limpeza de `Return` em `apply_closure`

O vanilla não limpa `vm.flow` após `Return` em `call.rs:698-706`; o evento sobe até ao consumidor externo. No cristalino, `apply_closure` limpa `ctx.flow` ao consumir `Return` para evitar que o evento seja re-interpretado como "fora de função" pelo contexto de chamada (por exemplo, `#let f() = { return 1 } #f()` no topo do documento). Esta é uma divergência mecânica consciente; o observable — return dentro de função funciona e return no topo dá erro — mantém-se.

### Decisão técnica: `while` sem teste de `continue` iterativo

O cristalino ainda não implementa assignment mutável em `while` (`i = i + 1` dá erro), pelo que não foi possível construir um teste iterativo de `#continue` num `while`. O mecanismo `Continue` é testado no `#for`; o consumidor no `eval_while` é idêntico ao do `eval_for`.

---

## Validação

### Testes unitários (P635)

```
running 9 tests
test rules::eval::tests::tests::p635_break_inside_function_is_forbidden ... ok
test rules::eval::tests::tests::p635_break_in_for_stops_loop ... ok
test rules::eval::tests::tests::p635_break_in_while_stops_loop ... ok
test rules::eval::tests::tests::p635_continue_inside_function_is_forbidden ... ok
test rules::eval::tests::tests::p635_continue_in_for_skips_iteration ... ok
test rules::eval::tests::tests::p635_nested_loops_break_only_inner ... ok
test rules::eval::tests::tests::p635_return_from_function_with_value ... ok
test rules::eval::tests::tests::p635_return_stops_function_body ... ok
test rules::eval::tests::tests::p635_return_without_value ... ok
```

### Testes directos após implementação

| Caso | Esperado | Obtido |
|------|----------|--------|
| `#for i in range(10) { if i == 3 { break } str(i) }` | `012` | `012` ✅ |
| `#for i in range(5) { if i == 2 { continue } str(i) }` | `0134` | `0134` ✅ |
| `#let f(x) = { if x < 0 { return "negativo" } "positivo" } #f(-5) #f(5)` | `negativo positivo` | `negativo positivo` ✅ |
| `#while true { break str(1) }` | (vazio) | (vazio) ✅ |
| `#break` no topo | erro | `cannot break outside of loop` ✅ |
| `#continue` no topo | erro | `cannot continue outside of loop` ✅ |
| `#return 1` no topo | erro | `cannot return outside of function` ✅ |

### Suite completa

```
cargo test --workspace
```

Resultado: todos os testes passaram (workspace completo; typst-core: 3613 passed; 0 failed).

### Linter

```
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## Estado de fecho

- [x] Sonda completa, mecanismo do vanilla e estado do cristalino confirmados.
- [x] `FlowEvent` implementado.
- [x] `#break`, `#continue`, `#return` funcionam de facto, testados com números concretos.
- [x] Ciclos aninhados testados.
- [x] Casos fora de contexto (P634) sem regressão.
- [x] Corpus verificado — nenhum documento usava as keywords.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p635.md`.
