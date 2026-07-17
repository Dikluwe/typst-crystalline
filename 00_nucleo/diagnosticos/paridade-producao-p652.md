# Relatório de Paridade — P652

**Passo:** 652  
**Data:** 2026-07-09  
**Foco:** `array.sorted()` com tipos incompatíveis assume `Ordering::Equal` em silêncio.  
**Dependências:** P650 (caso confirmado).  
**Hash do commit com as alterações:** `63836cbac`

---

## 1. Sonda

### 1.1 Comportamento do vanilla

```bash
cat > /tmp/p652-sorted-misto.typ <<'EOF'
#(1, "a", 2).sorted()
EOF
lab/typst-original/target/release/typst compile /tmp/p652-sorted-misto.typ /tmp/p652-vanilla.pdf
```

Resultado:

```text
error: cannot compare string and integer
  ┌─ /tmp/p652-sorted-misto.typ:1:1
  │
1 │ #(1, "a", 2).sorted()
  │  ^^^^^^^^^^^^^^^^^^^^
  │
  = hint: consider choosing a `key` or defining the comparison with `by`
```

Exit code: 1.

### 1.2 Local no cristalino

`01_core/src/engine/stdlib/collections.rs:184,187`:

```rust
keyed.sort_by(|a, b| value_cmp(&a.0, &b.0).unwrap_or(Ordering::Equal));
sorted.sort_by(|a, b| value_cmp(a, b).unwrap_or(Ordering::Equal));
```

`value_cmp` devolvia `None` para tipos incompatíveis, e o sort assumia `Equal`.

---

## 2. Implementação

### 2.1 `01_core/src/engine/stdlib/collections.rs`

- `value_cmp` passou a devolver `Result<Ordering, EcoString>`:
  - Tipos incompatíveis → `Err("cannot compare {type_a} and {type_b}")`.
  - `NaN` em floats → `Err("cannot compare floats")` / etc.
  - Int/float e strings continuam comparáveis.
- `array_sorted` captura o primeiro erro de comparação durante o `sort_by` (via flag externa) e propaga-o como `SourceDiagnostic::error`.
- A lógica aplica-se tanto ao caminho sem `key` como ao caminho com `key`.

### 2.2 `01_core/src/engine/eval/tests.rs`

Adicionado `p652_array_sorted_tipo_incompativel_errors`, que confirma que `(1, "a", 2).sorted()` agora produz erro com a mensagem esperada.

---

## 3. Validação

```bash
cargo test --workspace
```

Resultado: todos os crates passaram (`3646 passed` em `typst-core`).

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.1 CLI

Caso inválido:

```bash
cat > /tmp/p652-sorted-misto.typ <<'EOF'
#(1, "a", 2).sorted()
EOF
./target/release/typst /tmp/p652-sorted-misto.typ /tmp/p652-misto.pdf
```

Saída:

```text
/tmp/p652-sorted-misto.typ:<detached>: error: cannot compare str and int
exit code: 1
```

Caso válido:

```bash
cat > /tmp/p652-sorted-valido.typ <<'EOF'
#(3, 1, 2).sorted()
#("banana", "apple", "cherry").sorted()
EOF
./target/release/typst /tmp/p652-sorted-valido.typ /tmp/p652-valido.pdf
```

Saída: exit code 0, sem erros.

### 3.2 Nota sobre `sorted(key: ...)`

O cristalino ainda não suporta a variante named `key` de `array.sorted()` (o argumento `key` é ignorado). O teste com `((name: "bob"), (name: "alice")).sorted(key: x => x.name)` continua a comparar os dicionários inteiros; com esta alteração, passou a produzir erro em vez de resultado silenciosamente incorrecto. Suporte a `sorted(key: ...)` é trabalho futuro, fora do scope deste passo.

---

## 4. Decisão

- `value_cmp` já não retorna `Option<Ordering>`; retorna `Result<Ordering, EcoString>` e deixa o caller decidir.
- `array_sorted` propaga o erro, alinhando-se com o vanilla para tipos incompatíveis.
- O resultado de ordenação incorrecto para tipos mistos deixou de ser apresentado como se fosse válido.
