# Relatório — typst-passo-881: tipos chamáveis como funções de ordem superior

**Data:** 2026-07-24T00:02:45Z  
**Executor:** Kimi Code  
**Commit base:** `3f15cc50ec1dc40e852b41bc92e9ee2895ecaec6` (HEAD após P880)  
**Ramo:** `Tekt`  
**L0 afetado:** `00_nucleo/prompts/engine/stdlib/collections.md`

---

## 1. Resumo

P880 descobriu que `..range(50).map(str)` falhava no cristalino com `array.map() espera função, recebeu type`, enquanto funcionava no vanilla 0.15.0. P881 corrige isso: métodos de ordem superior de `array` (e `dict`) agora aceitam tipos chamáveis (`str`, `int`, `float`, `type`, `counter`, `state`, `symbol`, `bytes`, `datetime`) como funções, replicando o comportamento do vanilla.

O `05-tables.typ` original voltou a compilar sem o ajuste de sintaxe exigido por P880, tornando a comparação de benchmark com P872 estritamente maçã-com-maçã de novo.

---

## 2. O que foi implementado

### 2.1 Causa raiz

Em `01_core/src/engine/eval/mod.rs:1488-1490`, os identificadores `str`, `int`, `float` são definidos no escopo global como `Value::Type(...)`. A chamada direta `str(42)` funciona porque `eval_call` despacha `Value::Type` para o construtor nativo correspondente. No entanto, métodos como `array.map()` esperam `Value::Func` e rejeitavam `Value::Type`.

### 2.2 Correção

`01_core/src/engine/stdlib/collections.rs`:
- Adicionado helper `type_as_callable(Type) -> Option<Func>` que mapeia cada tipo chamável para a `Func` nativa correspondente (`native_str`, `native_int`, `native_float`, `native_type`, `native_counter`, `native_state`, `native_symbol`, `native_bytes`, `native_datetime`).
- `expect_one_func` agora aceita `Value::Type(t)` quando `type_as_callable(t)` devolve `Some`, e rejeita com mensagem apropriada quando devolve `None`.

Isto cobre `array.map`, `array.filter`, `array.find`, `array.any`, `array.all`, `array.sorted` (via `key`), `dict.map` e `dict.filter` — todos usam `expect_one_func`.

### 2.3 L0 atualizado

`00_nucleo/prompts/engine/stdlib/collections.md` secção 5 (Convenções) foi atualizada para documentar que tipos chamáveis são aceites como closures.

---

## 3. Validação

### 3.1 Testes

```text
$ cargo test --workspace

test result: ok. 4693 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out  (typst_core)
test result: ok. 729 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out   (typst_infra)
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst_shell)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (typst_wiring unit)
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out    (typst_wiring integration)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out     (typst_wiring crystalline_lint)
```

**Total: 5504 passed** (+7 relativamente a P880: 4 testes unitários em `collections.rs`, 3 testes E2E em `eval/tests.rs`).

### 3.2 Linter

```text
$ crystalline-lint .
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' não é referenciado por nenhum arquivo em L1–L4. Materializar ou remover. [V7]
```

Zero violations. O warning V7 é pré-existente.

### 3.3 Testes de regressão novos

**Unitários (`01_core/src/engine/stdlib/collections.rs`):**
- `p881_type_as_callable_mapeia_tipos_construtores`: confirma que `str`, `int`, `float`, `type`, `counter`, `state`, `symbol`, `bytes`, `datetime` são mapeados e que `bool`, `array`, `length` não são.
- `p881_expect_one_func_aceita_tipo_chamavel`: `Value::Type(Type::Str)` é aceite como função.
- `p881_expect_one_func_rejeita_tipo_nao_chamavel`: `Value::Type(Type::Bool)` é rejeitado.
- `p881_expect_one_func_rejeita_int`: valores não-função continuam rejeitados.

**E2E (`01_core/src/engine/eval/tests.rs`):**
- `p881_array_map_str_tipo_chamavel`: `(0, 1, 2).map(str)` → `("0", "1", "2")`.
- `p881_array_map_int_tipo_chamavel`: `("5", "6").map(int)` → `(5, 6)`.
- `p881_array_filter_type_tipo_chamavel`: `("a", 1, "b").filter(x => type(x) == str)` → `("a", "b")`.

### 3.4 Verificação manual contra vanilla

```text
$ lab/typst-original/target/release/typst compile p881-str.typ /dev/null --format pdf
  → sucesso
$ target/release/typst p881-str.typ /dev/null
  → sucesso
```

Testado para `str`, `int`, `float`, `type`.

---

## 4. Impacto no benchmark P880/P872

### 4.1 05-tables original volta a funcionar

Com a correção, o documento original de P872:

```typst
#for i in range(20) {
  table(columns: 5, rows: 10, ..range(50).map(str))
}
```

compila em ambos os binários. O benchmark foi re-executado com a sintaxe original:

| Cenário | Vanilla média (s) | Cristalino média (s) | Razão C/V |
|---|---|---|---|
| 05-tables (original `.map(str)`) | 0.3024 | 0.1153 | **0.38×** |

Este valor é idêntico, dentro da margem de erro, ao valor P880 medido com a expressão lambda ajustada (0.3014 / 0.1149 = 0.38×). A alteração sintática que P880 precisou fazer não distorcia a comparação, mas agora a comparação é formalmente maçã-com-maçã com P872/P879/P880.

---

## 5. Ficheiros alterados

- `00_nucleo/prompts/engine/stdlib/collections.md`
- `01_core/src/engine/stdlib/collections.rs`
- `01_core/src/engine/eval/tests.rs`

---

## 6. Procedimento de reprodução

```text
cargo build --workspace --release
cd /tmp/p872-bench
hyperfine --warmup 1 --min-runs 10 \
  '/home/dikluwe/Documentos/Antigravity/typst-crystalline/lab/typst-original/target/release/typst compile 05-tables.typ /dev/null --format pdf' \
  '/home/dikluwe/Documentos/Antigravity/typst-crystalline/target/release/typst 05-tables.typ /dev/null'
```

---

## 7. Conclusão

A regressão semântica que obrigou P880 a alterar `05-tables.typ` está corrigida. Tipos chamáveis (`str`, `int`, `float`, etc.) agora funcionam como valores de primeira classe em métodos de ordem superior, replicando o vanilla. O benchmark P872 pode ser reproduzido sem adaptações sintáticas, e o valor de 05-tables permanece 0.38×.
