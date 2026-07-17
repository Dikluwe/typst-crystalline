---
# P652 — `array.sorted()` com tipos incompatíveis assume `Equal` em silêncio

> **Passo:** 652
> **Data:** 2026-07-09
> **Foco:** P650 confirmou que `(1, "a", 2).sorted()` compila sem erro, porque `value_cmp(...).unwrap_or(Ordering::Equal)` assume que elementos incomparáveis são iguais, em vez de reportar o problema. Isto é diferente dos casos anteriores desta sequência — não é ausência de erro sobre uma acção que não teve efeito, é um resultado **errado** apresentado como se fosse correcto. A ordenação produzida não é a ordenação real dos dados, e nada avisa disso.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P650 (caso confirmado).

---

## Sonda

### Confirmar o comportamento do vanilla

```bash
cat > /tmp/p652-sorted-misto.typ <<'EOF'
#(1, "a", 2).sorted()
EOF
lab/typst-original/target/release/typst compile /tmp/p652-sorted-misto.typ /tmp/p652-vanilla.pdf
echo "Exit code vanilla: $?"
```

Confirmar a mensagem de erro exacta do vanilla, se houver.

### Confirmar o local exacto no cristalino

```bash
sed -n '175,195p' 01_core/src/engine/stdlib/collections.rs
```

---

## Implementação

Substituir `unwrap_or(Ordering::Equal)` por propagação de erro quando `value_cmp` não consegue comparar dois elementos, com a mensagem confirmada pelo vanilla.

### Critério de fecho da implementação

- [ ] `(1, "a", 2).sorted()` produz erro, igual ao vanilla.
- [ ] `array.sorted()` com elementos do mesmo tipo comparável continua a funcionar sem regressão.
- [ ] `array.sorted(key: ...)` (se existir essa variante) testado separadamente, para confirmar que a correcção cobre também esse caminho.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

```bash
cat > /tmp/p652-sorted-valido.typ <<'EOF'
#(3, 1, 2).sorted()
#("banana", "apple", "cherry").sorted()
EOF
./target/release/typst /tmp/p652-sorted-valido.typ /tmp/p652-valido.pdf
```

---

## Critério de fecho do passo

- [ ] Sonda completa, mensagem do vanilla confirmada.
- [ ] Erro propagado para tipos incompatíveis.
- [ ] Uso correcto (mesmo tipo) sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p652.md`, com hash do commit.
