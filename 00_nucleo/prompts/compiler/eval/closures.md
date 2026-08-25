# Prompt L0 — `compiler/eval/closures` — criação e aplicação de closures
Hash do Código: 5fb7121c

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/eval/closures.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/eval.md` (dono de `compiler/eval/mod.rs`)
**ADRs relevantes**: ADR-0107 (paridade língua), ADR-0108 (medir antes de decidir), ADR-0109 (atomização)
**Técnica**: aplicação de closures com captura de scope e binding de parâmetros

---

## Contexto

Este nó contém a **criação e aplicação de closures** do eval:

- `eval_closure_expr` constrói um `Value::Func` a partir de uma expressão de closure, capturando o scope actual.
- `apply_closure` aplica uma closure a argumentos: cria um scope filho do captured, injeta auto-referência para recursão, liga parâmetros (posicionais/nomeados/default/patterns/sink), avalia o body num engine local, e consome `FlowEvent` ao sair.

Extraído do monólito `compiler/eval/closures.rs` no Passo 1012 conforme ADR-0109 (atomização — forma B, free function no arquivo da unidade). O dispatch geral de chamadas (`eval_func_call`, `apply_func`, avaliação de args, etc.) mudou-se para o nó `compiler/eval/call_dispatch.rs`.

---

## Instrução

### 1. Contrato público

```rust
pub(super) fn apply_closure(
    closure: &ClosureRepr,
    func: &Func,
    args: Args,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;

pub(super) fn eval_closure_expr(
    closure_expr: ClosureNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;
```

### 2. Comportamento

Manter exatamente o comportamento actual:

- `eval_closure_expr` faz captura eager do scope por snapshot (`scopes.snapshot()`), extrai parâmetros (posicionais, nomeados com default, patterns, sink spread), e constrói um `ClosureRepr`.
- `apply_closure` verifica profundidade de chamada (`MAX_CALL_DEPTH = 80`), cria scope filho do captured via `Arc::clone`, injeta auto-referência se a closure tiver nome, liga parâmetros com named-sobre-posicional e regras P708/P724/P733/P504, avalia o body num `Engine` local com route/styles/sink próprios, e consome `FlowEvent::Return`.

### 3. Gatilhos de reabertura

- Mudança de semântica de captura (lazy vs eager, scope vs environment).
- Mudança no binding de parâmetros (ordem, default, sink, patterns).
- Mudança de fase (eval ↔ layout) na execução de closures.

---

## Critérios de verificação

```
Dado #let f(x) = x * 2; f(3) → 6
Dado #let g(a, b: 1) = a + b; g(2) → 3
Dado #let fib(n) = if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }; fib(6) → 8
Dado #let h(..args) = args.pos().len(); h(1, 2, 3) → 3
```

Aplicação final: `cargo build && crystalline-lint .` — zero violations.

## P1160 — parâmetro posicional obrigatório ausente

Medição no vanilla ratificado `a51e02804`: aplicar dois argumentos a
`(a, b, c) => ...` falha com `missing argument: c`; o parâmetro não recebe
`none` implicitamente. `apply_closure` deve emitir esse diagnóstico no binding
do primeiro positional obrigatório ausente. Defaults nomeados e sinks mantêm
as regras vigentes; argumentos excedentes continuam `unexpected argument`.
