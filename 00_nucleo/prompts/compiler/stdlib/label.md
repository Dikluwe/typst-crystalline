# Prompt L0 — `compiler/stdlib/label` — `native_label`
Hash do Código: 00000000

**Camada**: L1 · **Alvo**: `01_core/src/compiler/stdlib/label.rs`
**Origem**: P460; contrato corrigido por P1140.2.
**ADRs**: ADR-0107, ADR-0108, ADR-0127.

## Medição anterior à decisão

Vanilla ratificado `a51e02804`, `foundations/label.rs:74-89`, declara
`label(name: Str) -> Label`. Probes de 2026-08-23:

```text
repr(type(label))       -> "type"
repr(label("x"))        -> "<x>"
type(label("x"))        -> label
label("x") == <x>       -> true
label("x", [body])      -> erro unexpected argument
```

O cristalino anterior aceita `label(name, body)` e devolve
`Value::Content(Content::Label)`. Não foi encontrado consumidor `.typ` dessa
extensão; os usos Rust de `Content::label` são mecanismos internos.

## Função nativa

```rust
pub fn native_label(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>;
```

Registada no stdlib como o construtor de `Value::Type(Type::Label)`.

## Assinatura e emissão

```typst
label(name)
```

- exatamente um posicional `name: Str`, não vazio;
- named args rejeitados;
- devolve `Value::Label(Label(name))`, igual ao valor `<name>` em código;
- zero, dois ou mais posicionais, tipo diferente e nome vazio são erros.

A forma pública `label(name, body)` é removida por incompatibilidade com a
linguagem. `Content::label` e `Content::label_auto` permanecem API Rust interna
para associação/introspecção. Em markup, `<name>` continua anexando a label
ao elemento precedente por `Content::label_auto`.

## Critérios

```text
label("x")             -> Value::Label(Label("x"))
label("")              -> Err
label()                -> Err
label(1)               -> Err
label("x", content)    -> Err
label(name: "x")       -> Err
```
