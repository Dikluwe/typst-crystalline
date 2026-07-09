# Relatório de Paridade — P653

**Passo:** 653  
**Data:** 2026-07-09  
**Foco:** `array.sorted(key: ...)` é aceite pela sintaxe mas ignorado — ordenação compara sempre os valores em bruto.  
**Dependências:** P652 (onde o gap foi detectado).  
**Hash do commit com as alterações:** `c72ea9b2e`

---

## 1. Sonda

### 1.1 Comportamento do vanilla

```bash
cat > /tmp/p653-sorted-key.typ <<'EOF'
#(
  (name: "Bob", age: 30),
  (name: "Alice", age: 25),
  (name: "Carol", age: 35),
).sorted(key: x => x.name)
EOF
lab/typst-original/target/release/typst compile /tmp/p653-sorted-key.typ /tmp/p653-vanilla.pdf
pdftotext /tmp/p653-vanilla.pdf -
```

Resultado:

```text
(
(name: "Alice", age: 25),
(name: "Bob", age: 30),
(name: "Carol", age: 35),
)
```

### 1.2 Alcance do problema no cristalino

`01_core/src/rules/stdlib/collections.rs:147`: `array_sorted` lia `args.items`, tratando um eventual argumento como posicional. Como o vanilla exige `key:` como named arg, `args.items` estava sempre vazio e a função `key` nunca era aplicada.

Exemplo que distingue os dois comportamentos sem ambiguidade:

```bash
cat > /tmp/p653-sorted-key-visivel.typ <<'EOF'
#(3, -10, 2).sorted(key: x => calc.abs(x))
EOF
./target/release/typst /tmp/p653-sorted-key-visivel.typ -o /tmp/p653-antes.pdf
pdftotext /tmp/p653-antes.pdf -
```

Antes da correção:

```text
(-10, 2, 3)
```

A ordenação ignorava `key` e usava a ordem numérica em bruto.

---

## 2. Implementação

### 2.1 `01_core/src/rules/stdlib/collections.rs`

- `array_sorted` passou a validar argumentos de acordo com o vanilla:
  - Rejeita argumentos posicionais.
  - Rejeita named args desconhecidas (só `key` é permitida).
  - Extrai `key` de `args.named` em vez de `args.items`.
- Quando `key` está presente, aplica a função a cada elemento e compara os valores resultantes com `value_cmp`.
- Quando `key` está ausente, mantém o comportamento corrigido em P652.
- Erros de comparação de tipos incompatíveis aplicam-se aos resultados de `key`, não aos elementos originais.

### 2.2 `01_core/src/rules/eval/tests.rs`

Adicionados quatro testes:

- `p653_array_sorted_key_funciona` — `(3, -10, 2).sorted(key: x => calc.abs(x))` produz `(2, 3, -10)`.
- `p653_array_sorted_key_em_dicionarios` — ordena dicionários pelo campo `name`.
- `p653_array_sorted_rejeita_positional` — `sorted(x => -x)` é rejeitado.
- `p653_array_sorted_key_tipo_incompativel_propaga` — o erro de comparação refere-se aos valores produzidos por `key`.

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.1 CLI

Caso `key` visível:

```bash
./target/release/typst /tmp/p653-sorted-key-visivel.typ -o /tmp/p653-depois.pdf
pdftotext /tmp/p653-depois.pdf -
```

Resultado:

```text
(2, 3, -10)
```

Caso dicionários:

```bash
./target/release/typst /tmp/p653-sorted-key-dicts.typ -o /tmp/p653-dicts.pdf
pdftotext /tmp/p653-dicts.pdf -
```

Resultado:

```text
((name: "Alice", age: 25), (name: "Bob", age: 30), (name: "Carol", age: 35))
```

Regressão `sorted()` sem `key`:

```bash
./target/release/typst /tmp/p653-sem-key.typ -o /tmp/p653-sem-key.pdf
```

Resultado: `(1, 2, 3)`, exit code 0.

Regressão erro de tipos incompatíveis:

```bash
./target/release/typst /tmp/p653-erro-tipos.typ -o /tmp/p653-erro-tipos.pdf
```

Resultado: `error: cannot compare str and int`, exit code 1.

Rejeição de positional:

```bash
./target/release/typst /tmp/p653-positional.typ -o /tmp/p653-positional.pdf
```

Resultado: `error: array.sorted() does not accept positional arguments`, exit code 1.

---

## 4. Decisão

- `array.sorted(key: ...)` passou a ordenar pelo resultado da função chave, alinhando com o vanilla.
- `array.sorted()` sem `key` permanece inalterado face a P652.
- A interface rejeita argumentos posicionais e named args desconhecidas, como o vanilla.
