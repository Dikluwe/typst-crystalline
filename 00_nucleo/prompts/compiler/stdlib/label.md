# Prompt L0 — `rules/stdlib/label` — `native_label`
Hash do Código: 00000000

**Camada**: L1 · **Alvo**: `01_core/src/compiler/stdlib/structural.rs`
**Origem**: P460.

---

## Função nativa

```rust
pub fn native_label(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value>;
```

Registada no stdlib como `"label"` (`eval/mod.rs`).

## Assinatura Typst

```typst
label(name, body)
```

- 1º argumento posicional `name`: `Str` (não vazio).
- 2º argumento posicional `body`: `Content` (opcional; default `Content::Empty`).
- Named args não suportados.

## Emissão

Retorna `Value::Content(Content::label(name, body))`.

## Erros

- Nome vazio.
- `name` ou `body` com tipos inválidos.
- Named args inesperados.

## Notas

A sintaxe sugar `#label<sec1>` do vanilla é scope-out; neste passo usa-se
`label("sec1", body)` explicitamente.
