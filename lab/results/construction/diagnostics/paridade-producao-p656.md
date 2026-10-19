# Relatório de Paridade — P656

**Passo:** 656  
**Data:** 2026-07-09  
**Foco:** Confirmar se `args.named.keys()` tem ordem determinística (suspeita levantada por P655 nos loops `never_loop` de `structural.rs`).  
**Dependências:** P655 (onde os casos foram classificados sem verificação de ordem).  
**Hash do commit com as alterações:** `5bac6977a`

---

## 1. Verificação

### 1.1 Tipo de `args.named`

`01_core/src/entities/args.rs:21`:

```rust
pub struct Args {
    /// Argumentos posicionais, em ordem.
    pub items: Vec<Value>,
    /// Argumentos nomeados (named args), preservando ordem de inserção.
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}
```

`args.named` é `IndexMap`, que preserva a ordem de inserção. A ordem de iteração é determinística e estável.

### 1.2 Teste directo no cristalino

```bash
cat > /tmp/p656-multi-invalido.typ <<'EOF'
#(3, 1, 2).sorted(reverse: true, stable: false, alpha: 1)
EOF
for i in 1 2 3 4 5; do
  ./target/release/typst /tmp/p656-multi-invalido.typ -o /tmp/p656-out.pdf 2>&1 | grep "unexpected argument"
done
```

Resultado (5 execuções):

```text
/tmp/p656-multi-invalido.typ:<detached>: error: unexpected argument: reverse
/tmp/p656-multi-invalido.typ:<detached>: error: unexpected argument: reverse
/tmp/p656-multi-invalido.typ:<detached>: error: unexpected argument: reverse
/tmp/p656-multi-invalido.typ:<detached>: error: unexpected argument: reverse
/tmp/p656-multi-invalido.typ:<detached>: error: unexpected argument: reverse
```

### 1.3 Comportamento do vanilla no mesmo caso

```bash
for i in 1 2 3; do
  lab/typst-original/target/release/typst compile /tmp/p656-multi-invalido.typ /tmp/p656-vanilla.pdf 2>&1 | grep "unexpected argument"
done
```

Resultado (3 execuções):

```text
error: unexpected argument: reverse
error: unexpected argument: reverse
error: unexpected argument: reverse
```

---

## 2. Decisão

A suspeita de P655 não se confirmou:

- `args.named` é `IndexMap`, com ordem de inserção determinística.
- O teste directo mostra que a mensagem de erro é idêntica em todas as execuções, tanto no cristalino como no vanilla.
- O primeiro argumento inválido reportado (`reverse`) é o primeiro na ordem de escrita do documento, alinhando com o vanilla.

Não houve alterações de código. A nota em `paridade-producao-p655.md` foi actualizada para registar esta confirmação.

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

---

## 4. Ligação a P655

Adicionada nota em `00_nucleo/diagnosticos/paridade-producao-p655.md`, secção 2.2, confirmando que a ordem de `args.named` é determinística e que a classificação "semântica intencional" dos loops `never_loop` se mantém.
