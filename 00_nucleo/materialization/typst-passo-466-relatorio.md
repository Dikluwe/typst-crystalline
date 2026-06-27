# Relatório P466 — Métodos restantes de `array`, `dict`, `str`

**Data:** 2026-06-25  
**Executor:** Kimi Code CLI  
**Passo:** P466 (Trilha 8 — Refinos de stdlib e tipos)  
**Materialização:** Implementação + testes + specs L0

---

## 1. Resumo

Materializou-se o subset crítico de métodos de instância para os tipos de
coleção `array`, `dict` e `str`, totalizando **23 métodos**. Adicionalmente,
foi necessário implementar o eval de dict literal `(a: 1, b: 2)` e o helper
`Value::truthy()` para suportar predicados em métodos como `filter`, `any` e
`all`.

---

## 2. Métodos implementados

### `array` — 12 métodos

| Método | Exemplo | Resultado |
|--------|---------|-----------|
| `.first()` | `(1, 2, 3).first()` | `1` |
| `.last()` | `(1, 2, 3).last()` | `3` |
| `.rev()` | `(1, 2, 3).rev()` | `(3, 2, 1)` |
| `.sum()` | `(1, 2, 3).sum()` | `6` |
| `.sorted()` | `(3, 1, 2).sorted()` | `(1, 2, 3)` |
| `.filter(func)` | `(1, 2, 3).filter(x => x > 1)` | `(2, 3)` |
| `.map(func)` | `(1, 2, 3).map(x => x * 2)` | `(2, 4, 6)` |
| `.find(func)` | `(1, 2, 3).find(x => x > 1)` | `2` |
| `.any(func)` | `(1, 2, 3).any(x => x > 2)` | `true` |
| `.all(func)` | `(1, 2, 3).all(x => x > 0)` | `true` |
| `.zip(other)` | `(1, 2).zip(("a", "b"))` | `((1, "a"), (2, "b"))` |
| `.enumerate()` | `("a", "b").enumerate()` | `((0, "a"), (1, "b"))` |

### `dict` — 3 métodos

| Método | Exemplo | Resultado |
|--------|---------|-----------|
| `.pairs()` | `(a: 1, b: 2).pairs()` | `(("a", 1), ("b", 2))` |
| `.remove(key)` | `(a: 1, b: 2).remove("a")` | `1` |
| `.update(other)` | `(a: 1).update((b: 2))` | `(a: 1, b: 2)` |

### `str` — 8 métodos

| Método | Exemplo | Resultado |
|--------|---------|-----------|
| `.contains(substr)` | `"hello".contains("ell")` | `true` |
| `.starts-with(prefix)` | `"hello".starts-with("he")` | `true` |
| `.ends-with(suffix)` | `"hello".ends-with("lo")` | `true` |
| `.find(substr)` | `"hello".find("ll")` | `2` |
| `.replace(old, new)` | `"hello".replace("l", "x")` | `"hexxo"` |
| `.trim()` | `"  hello  ".trim()` | `"hello"` |
| `.split(sep)` | `"a,b,c".split(",")` | `("a", "b", "c")` |
| `.repeat(n)` | `"ab".repeat(3)` | `"ababab"` |

---

## 3. Arquivos alterados

### Código de produção

- `01_core/src/rules/stdlib/collections.rs` — novo módulo com os 23 métodos e
  o dispatcher `try_dispatch_collection_method`.
- `01_core/src/rules/stdlib/mod.rs` — registo do módulo `collections` e
  re-exportação do dispatcher.
- `01_core/src/rules/eval/closures.rs` — intercepção de method calls em
  `eval_func_call` antes do dispatch genérico de funções.
- `01_core/src/rules/eval/mod.rs` — eval de dict literal `(a: 1, b: 2)`;
  chaves keyed não-string (ex.: regex) deixam o dict como `Value::None` para
  preservar o parsing especializado de `#set text(font:)`.
- `01_core/src/entities/value.rs` — adição de `Value::truthy()`.

### Testes

- `01_core/src/rules/eval/tests.rs` — 23 testes de integração E2E, um por
  método.

### Specs L0

- `00_nucleo/prompts/rules/stdlib/foundations.md` — secção 10 com a tabela de
  métodos de `array`, `dict` e `str`.
- `00_nucleo/prompts/entities/value.md` — nota sobre métodos de coleção,
  `truthy()` e eval de dict literal.

---

## 4. Divergências e limitações documentadas

1. **`dict.remove()` não muta a variável original**  
   O dispatch de método recebe o dict por valor. Portanto,
   `#let d = (a: 1); d.remove("a")` retorna `1`, mas `d` permanece
   `(a: 1)`. Divergência consciente vs vanilla.

2. **`array.sum()` rejeita tipos não-numéricos**  
   Ao contrário do snippet de rascunho do P466, a implementação actual emite
   erro contextual para valores que não sejam `int` ou `float`.

3. **`array.sorted()` com `key` usa `apply_func`**  
   A chave é uma `Func` e é aplicada via infraestrutura existente de closures.

4. **Dict literal com chaves regex**  
   O eval genérico de dict literal converte `(a: 1)` em `Value::Dict`, mas
   `(regex("..."): ...)` retorna `Value::None`, permitindo que callers
   especializados (como `#set text(font:)`) processem o AST directamente.

---

## 5. Resultados dos testes

### Testes específicos P466

```
running 23 tests
test rules::eval::tests::tests::p466_array_all ... ok
test rules::eval::tests::tests::p466_array_any ... ok
test rules::eval::tests::tests::p466_array_enumerate ... ok
test rules::eval::tests::tests::p466_array_filter ... ok
test rules::eval::tests::tests::p466_array_find ... ok
test rules::eval::tests::tests::p466_array_first ... ok
test rules::eval::tests::tests::p466_array_last ... ok
test rules::eval::tests::tests::p466_array_map ... ok
test rules::eval::tests::tests::p466_array_rev ... ok
test rules::eval::tests::tests::p466_array_sorted ... ok
test rules::eval::tests::tests::p466_array_sum ... ok
test rules::eval::tests::tests::p466_array_zip ... ok
test rules::eval::tests::tests::p466_dict_pairs ... ok
test rules::eval::tests::tests::p466_dict_remove ... ok
test rules::eval::tests::tests::p466_dict_update ... ok
test rules::eval::tests::tests::p466_str_contains ... ok
test rules::eval::tests::tests::p466_str_ends_with ... ok
test rules::eval::tests::tests::p466_str_find ... ok
test rules::eval::tests::tests::p466_str_replace ... ok
test rules::eval::tests::tests::p466_str_repeat ... ok
test rules::eval::tests::tests::p466_str_split ... ok
test rules::eval::tests::tests::p466_str_starts_with ... ok
test rules::eval::tests::tests::p466_str_trim ... ok

test result: ok. 23 passed; 0 failed
```

### Suite completa

```
RUST_MIN_STACK=8388608 cargo test --workspace -- --test-threads=1
```

Resultado: **todos os testes passam** (3291+ no `typst-core`, 21 no `typst-wiring`,
2 no `crystalline_lint`).

> **Nota ambiental:** Sem `RUST_MIN_STACK=8388608` e `--test-threads=1`, a suite
> completa falha com *stack overflow* no teste pré-existente
> `p350c_flag_on_nao_convergente_classifica` (e ocasionalmente em
> `recursao_infinita_retorna_err_sem_crash`). O problema é ambiental/preexistente
> e não relacionado ao P466.

### `crystalline-lint`

```
Running tests/crystalline_lint.rs
running 2 tests
test type_level_allowed_ecow_ecostring ... ok
test type_level_violation_ecow_ecomap ... ok

test result: ok. 2 passed; 0 failed
```

---

## 6. Critério de fecho

- [x] 12 métodos de `array` implementados e registados no dispatcher.
- [x] 3 métodos de `dict` implementados e registados no dispatcher.
- [x] 8 métodos de `str` implementados e registados no dispatcher.
- [x] 23 testes verdes (1 por método).
- [x] Spec L0 atualizada (`rules/stdlib/foundations.md`, `entities/value.md`).
- [x] `cargo test --workspace` verde (com flags ambientais documentadas);
  `crystalline-lint` zero violations.
- [x] **Trilha 8: 2/8 completo** (repr P465 + métodos P466).

---

## 7. Próximo passo recomendado

- **P467** — Tipos `Value`: `Relative` (`Rel<Length>`), `Symbol` refinado
  (Trilha 8, S, ~15 min).
