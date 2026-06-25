# Prompt L0 — `stdlib/collections` — métodos de instância de array, dict e str

**Camada**: L1  
**Ficheiro alvo**: `01_core/src/rules/stdlib/collections.rs`  
**Criado em**: 2026-06-25 (Passo P466)  
**ADRs**: ADR-0037 (coesão por domínio), ADR-0117 Cláusula 4 (métodos de tipos
existentes; não propõe estrutura em elementos).

---

## 1. Visão geral

`collections.rs` implementa métodos de instância para os tipos de coleção do
Typst: `array`, `dict` e `str`. Os métodos são despachados pela sintaxe de
chamada de método (ex.: `(1, 2, 3).first()`, `"ab".repeat(3)`) em
`rules/eval/closures.rs::eval_func_call`, antes do dispatch genérico de
funções.

A assinatura do dispatcher é:

```rust
pub(crate) fn try_dispatch_collection_method(
    target: Value,
    method: &str,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> Option<SourceResult<Value>>
```

Retorna `Some(Result)` se o método for reconhecido; `None` caso contrário,
permitindo fallback para o dispatch genérico.

---

## 2. Métodos de `array`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `first` | `array.first() -> any` | Primeiro elemento ou `none`. |
| `last` | `array.last() -> any` | Último elemento ou `none`. |
| `rev` | `array.rev() -> array` | Array invertido. |
| `sum` | `array.sum() -> int \| float` | Soma numérica; rejeita tipos não-numéricos. Mistura int/float produz float. |
| `sorted` | `array.sorted(key: function?) -> array` | Ordenação; `key` opcional. Compara `int`, `float` e `str`. |
| `filter` | `array.filter(pred: function) -> array` | Filtra por predicado. |
| `map` | `array.map(func: function) -> array` | Mapeia por função. |
| `find` | `array.find(pred: function) -> any` | Primeiro elemento que satisfaz ou `none`. |
| `any` | `array.any(pred: function) -> bool` | Algum satisfaz? |
| `all` | `array.all(pred: function) -> bool` | Todos satisfazem? |
| `zip` | `array.zip(other: array) -> array` | Pares `(a_i, b_i)`; trunca no menor tamanho. |
| `enumerate` | `array.enumerate() -> array` | Pares `(index, value)`. |

---

## 3. Métodos de `dict`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `pairs` | `dict.pairs() -> array` | Array de pares `(key, value)`. |
| `remove` | `dict.remove(key: str) -> any` | Valor removido ou `none`. **Nota**: no cristalino o dict original não é mutado (dispatch por valor). |
| `update` | `dict.update(other: dict) -> dict` | Mescla com outro dict. |

---

## 4. Métodos de `str`

| Método | Assinatura | Semântica |
|--------|-----------|-----------|
| `contains` | `str.contains(substr: str) -> bool` | Contém substring? |
| `starts-with` | `str.starts-with(prefix: str) -> bool` | Começa com prefixo? |
| `ends-with` | `str.ends-with(suffix: str) -> bool` | Termina com sufixo? |
| `find` | `str.find(substr: str) -> int \| none` | Índice da primeira ocorrência. |
| `replace` | `str.replace(old: str, new: str) -> str` | Substitui substring literal. |
| `trim` | `str.trim() -> str` | Remove whitespace dos extremos. |
| `split` | `str.split(sep: str) -> array` | Divide por separador. |
| `repeat` | `str.repeat(n: int) -> str` | Repete `n` vezes; `n >= 0`. |

---

## 5. Convenções

- Usar `SourceDiagnostic::error(Span::detached(), ...)` para erros de tipo ou
  aridade.
- Métodos que recebem closures (`filter`, `map`, `find`, `any`, `all`, `sorted`
  com `key`) aplicam a função via `apply_func` existente em
  `rules/eval/closures.rs`.
- `Value::truthy()` (definido em `entities/value.rs`) é usado para avaliar o
  resultado dos predicados.

---

## 6. Scope-outs

- `array.dedup()`, `array.min()`, `array.max()`, `array.flatten()`.
- `str.rev()`.
- `str.replace` com regex.
- Métodos com argumento `default`.
- Unicode avançado (grapheme clusters); índices de `str.find` são byte/char.
- Mutação do dict original em `.remove()`.

---

## 7. Testes canônicos

Ver `01_core/src/rules/eval/tests.rs`, secção P466, com 23 testes E2E (um por
método).
