# Relatório de Paridade — P654

**Passo:** 654  
**Data:** 2026-07-09  
**Foco:** Confirmar mensagens exactas do vanilla para `array.sorted()` rejeitado (positional e named arg desconhecida).  
**Dependências:** P653 (onde as mensagens foram escritas sem confirmação directa).  
**Hash do commit com as alterações:** `4e80255bc`

---

## 1. Sonda

### 1.1 Argumento posicional

```bash
cat > /tmp/p654-positional.typ <<'EOF'
#(3, 1, 2).sorted(x => -x)
EOF
lab/typst-original/target/release/typst compile /tmp/p654-positional.typ /tmp/p654-pos-vanilla.pdf
```

Mensagem do vanilla:

```text
error: unexpected argument
  ┌─ ../../../../../tmp/p654-positional.typ:1:18
  │
1 │ #(3, 1, 2).sorted(x => -x)
  │                   ^^^^^^^
```

### 1.2 Named argument desconhecido

```bash
cat > /tmp/p654-unknown.typ <<'EOF'
#(3, 1, 2).sorted(reverse: true)
EOF
lab/typst-original/target/release/typst compile /tmp/p654-unknown.typ /tmp/p654-unk-vanilla.pdf
```

Mensagem do vanilla:

```text
error: unexpected argument: reverse
  ┌─ ../../../../../tmp/p654-unknown.typ:1:18
  │
1 │ #(3, 1, 2).sorted(reverse: true)
  │                   ^^^^^^^^^^^^^
```

---

## 2. Ajuste

### 2.1 `01_core/src/engine/stdlib/collections.rs`

As mensagens do cristalino para os dois casos divergiam:

- Antes (positional): `array.sorted() does not accept positional arguments`
- Antes (unknown): `array.sorted() unexpected argument 'reverse'`

Alteradas para bater com o vanilla:

- Depois (positional): `unexpected argument`
- Depois (unknown): `unexpected argument: reverse`

### 2.2 `01_core/src/engine/eval/tests.rs`

Atualizado `p653_array_sorted_rejeita_positional` para verificar `unexpected argument` em vez da mensagem anterior.

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

Cristalino, positional:

```bash
./target/release/typst /tmp/p654-positional.typ -o /tmp/p654-pos-cristalino.pdf
```

Resultado:

```text
/tmp/p654-positional.typ:<detached>: error: unexpected argument
exit code: 1
```

Cristalino, unknown named arg:

```bash
./target/release/typst /tmp/p654-unknown.typ -o /tmp/p654-unk-cristalino.pdf
```

Resultado:

```text
/tmp/p654-unknown.typ:<detached>: error: unexpected argument: reverse
exit code: 1
```

Ambas batem com as mensagens do vanilla.

---

## 4. Decisão

- As mensagens de erro de `array.sorted()` para argumento posicional e named arg desconhecida foram alinhadas com o texto exacto do Typst vanilla.
- Nenhuma alteração semântica: a rejeição dos casos já funcionava; só o texto mudou.
