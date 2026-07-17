---
# P653 — `array.sorted(key: ...)` é aceite mas nunca implementado

> **Passo:** 653
> **Data:** 2026-07-09
> **Foco:** P652 revelou que o argumento nomeado `key:` de `array.sorted(...)` é aceite pela sintaxe mas nunca usado — a ordenação compara sempre os valores em bruto, ignorando a função de chave. Antes de P652, isto produzia resultado errado sem aviso quando os valores não eram comparáveis; depois de P652, produz erro nesse caso, mas continua a ordenar pelo valor errado quando os valores por acaso são comparáveis sem a chave. É o mesmo tipo de situação do `FlowEvent` (P635) — sintaxe que existe, funcionalidade que não.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P652 (onde o gap foi encontrado), P635 (precedente do mesmo tipo de situação — sintaxe aceite, semântica ausente).

---

## Sonda

### Confirmar o comportamento do vanilla

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

Confirmar a ordem esperada (Alice, Bob, Carol, pela chave `name`).

### Confirmar o alcance completo do problema no cristalino

```bash
grep -n "\"key\"\|named.*key\|fn.*sorted" 01_core/src/engine/stdlib/collections.rs | head -20
```

Confirmar se `key` é sequer lido dos argumentos nomeados, ou se nem chega a ser extraído — se a sintaxe aceita `key:` sem erro de "argumento desconhecido", o parsing dos argumentos já reconhece o nome, só a lógica de aplicação está em falta.

### Testar directamente com valores comparáveis, para confirmar o resultado errado

Usar um exemplo onde a diferença entre "key aplicado" e "key ignorado" seja visível sem ambiguidade:

```bash
cat > /tmp/p653-sorted-key-visivel.typ <<'EOF'
#(3, -10, 2).sorted(key: x => calc.abs(x))
EOF
./target/release/typst /tmp/p653-sorted-key-visivel.typ /tmp/p653-visivel.pdf
pdftotext /tmp/p653-visivel.pdf -
```

Esperado, com `key` a funcionar: `(2, 3, -10)` (por valor absoluto: 2, 3, 10). Se `key` for ignorado: `(-10, 2, 3)` (ordem numérica em bruto). Os dois resultados são visivelmente diferentes, confirmando sem ambiguidade se `key` tem efeito.

### Critério de fecho da sonda

- [ ] Comportamento do vanilla confirmado.
- [ ] Confirmado, com exemplo que distingue os dois casos sem ambiguidade, que `key` não tem efeito hoje no cristalino.
- [ ] Confirmado onde `key` é (ou não) extraído dos argumentos nomeados.

---

## Implementação

Aplicar a função `key` a cada elemento antes de comparar, em vez de comparar os elementos directamente. A comparação usa o resultado de `key(elemento)`, não o elemento em si.

### Critério de fecho da implementação

- [ ] `array.sorted(key: ...)` ordena pelo resultado da função, não pelo valor em bruto.
- [ ] Testado com o exemplo que distingue os dois casos, confirmando o resultado certo.
- [ ] `array.sorted()` sem `key` continua a funcionar como já corrigido em P652.
- [ ] Erro de P652 (tipos incompatíveis) continua a aplicar-se aos resultados de `key`, não aos elementos originais — se a função `key` devolver tipos incompatíveis entre si, o erro deve referir-se a isso.

---

## Validação

```bash
./target/release/typst /tmp/p653-sorted-key-visivel.typ /tmp/p653-depois.pdf
pdftotext /tmp/p653-depois.pdf -
```

Confirmar `(2, 3, -10)`, batendo com o vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, com exemplo que distingue sem ambiguidade os dois comportamentos possíveis.
- [ ] `key` implementado e a funcionar, testado contra o vanilla.
- [ ] Sem regressão em `array.sorted()` sem `key`.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p653.md`, com hash do commit.
