# Prompt L0 — `compiler/eval/call_dispatch` — dispatch de chamadas de função
Hash do Código: d4bfa9ef

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/call_dispatch.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/mod.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: dispatch de chamadas com intercepção de métodos especiais

---

## Contexto

Este nó contém o **dispatch de chamadas de função** do eval: dada uma expressão de chamada (`Expr::FuncCall`), avalia o callee, os argumentos, e aplica a função resultante. Inclui intercepções sintácticas para métodos especiais (`where`, `and`/`or`, `within`, mutating methods, métodos de coleção, `with`, `to-absolute`, `measure`, `layout`, métodos de `state`/`counter`/`color`/`version`/`args`/`content`, etc.) antes de cair no caminho genérico.

Extraído de `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade).

---

## Instrução

### 1. Contrato público

```rust
pub fn apply_func(
    func: Func,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;

pub(super) fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Args>;

pub(super) fn eval_func_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;
```

Funções privadas do nó:

- `trace_call` — envolve erros de uma chamada com `Tracepoint::Call` quando o span da chamada não contém o erro.
- `merge_with_args(pre: &Args, new: Args) -> Args` — funde args pré-ligados por `.with(...)` com os da chamada final.
- `call_plugin(p: &PluginFunc, args: &Args) -> SourceResult<Value>` — aplica um export de plugin WASM.
- `eval_location_method(loc, method, ctx) -> SourceResult<Value>` — resolve métodos de instância de `Value::Location`.

### 2. Comportamento

Manter exatamente o comportamento actual:

- `eval_func_call` avalia chamadas sintácticas, aplicando intercepções na ordem correcta (method calls especiais antes do caminho genérico).
- `apply_func` despacha sobre `FuncRepr`: `Closure` via `closures::apply_closure`, `Element` via ctor, `Plugin` via `call_plugin`, `With` via merge recursivo, `Native`/`NativeWithEngine` via fn-ptr.
- `eval_args` avalia argumentos posicionais/nomeados/spread, espelhando `ast::Args::eval` do vanilla.
- `trace_call` só adiciona trace quando o erro está fora do span da chamada.
- `call_plugin` rejeita named args e exige `Value::Bytes` posicionais.
- `eval_location_method` lê do `ctx.introspector`, com defaults seguros quando a posição ainda não está disponível.

### 3. Gatilhos de reabertura

- Novo `FuncRepr` ou novo tipo de função chamável.
- Nova intercepção de method call especial.
- Mudança de fase (eval ↔ layout) no processamento de chamadas.

---

## Critérios de verificação

```
Dado #let f(x) = x + 1; f(2) → 3
Dado #let g = f.with(1); g() → 2
Dado #calc.max(1, 2) → 2
Dado plugin("foo")(bytes) → bytes de retorno
Dado #loc.page() → int (via introspector)
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.
