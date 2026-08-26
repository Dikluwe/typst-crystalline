# Prompt L0 — `stdlib/foundations/ty` — `type(v)`
Hash do Código: 7d9c475b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/ty.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Função

### `native_type` — `type(v)`

**Assinatura**: `type(v: any) -> type`

**Semântica**: Devolve o **valor-tipo** do argumento — `Value::Type(v.type_of())`.
Paridade vanilla P685: `type(1) == int`, `type(int) == type`, `repr(type(1)) == "int"`.

**Testes canônicos**:
```
type(1)             -> Value::Type(Type::Int)
type("abc")         -> Value::Type(Type::Str)
type(none)          -> Value::Type(Type::None)
type(int)           -> Value::Type(Type::Type)
type()              -> Err "type() requer 1 argumento"
type(1, 2)          -> Err "type() requer 1 argumento"
```
