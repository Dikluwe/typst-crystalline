# Prompt L0 — `stdlib/foundations/cast` — conversões e constructors
Hash do Código: dfcaa234

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/cast.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Funções

### `native_int` — `int(v)`

Aceita `Int`, `Bool`, `Str` (decimal ou `base:` 2–36). Rejeita `Float`.

### `native_float` — `float(v)`

Aceita `Float`, `Int`, `Str` parseável.

### `native_range` — `range(...)`

Algoritmo verbatim do vanilla `Array::range`: `inclusive:` e `step:`.
Direção incompatível → array vazio; `step: 0` → erro verbatim.

### `native_bytes` — `bytes(value)`

Aceita `Str` (UTF-8), `Array` de ints 0–255, `Bytes`. Int não aceite.

### `native_datetime` — `datetime(...)`

Argumentos nomeados `year`, `month`, `day`, `hour`, `minute`, `second`.
Data completa, hora completa ou ambas. Mensagens verbatim do vanilla.

### `native_symbol` — `symbol(...)`

Constrói `Value::Symbol` a partir de variantes (string de um grapheme ou
array `(modifiers, char)`).

## 2. Critérios de verificação

```
int(42)                    -> 42
int("-7")                  -> -7
int(3.7)                   -> Err "int() não converte float"
float("2.5")               -> 2.5
range(3)                   -> (0, 1, 2)
range(-5)                  -> ()
range(0, 10, step: 2)      -> (0, 2, 4, 6, 8)
range(0, 10, step: 0)      -> Err "number must not be zero"
bytes("α")                 -> bytes UTF-8 de "α"
bytes((0xFF,))             -> Err "number must be between 0 and 255"
datetime(year: 2026, month: 6, day: 25) -> Datetime
symbol("🖂")                -> Symbol com variante base
```
