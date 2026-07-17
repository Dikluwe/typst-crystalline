---
# P733 — Argumento nomeado extra sem parâmetro (`f(1, z: 2)`) aceite silenciosamente

> **Passo:** 733
> **Data:** 2026-07-10
> **Foco:** P708 corrigiu o binding de argumentos posicionais extra (agora produz erro, como o vanilla). O caso irmão — um argumento **nomeado** que não corresponde a nenhum parâmetro da função — continua a ser aceite silenciosamente, sem erro. Mesma categoria de bug ("aceitação silenciosa incorrecta"), código provavelmente diferente (validação de `args.named`, não `args.items`).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P708 (onde o caso posicional foi corrigido, padrão a espelhar para o caso nomeado).

---

## Sonda

### Confirmar o comportamento exacto no vanilla

```bash
cat > /tmp/p733-named-extra.typ <<'EOF'
#let f(a) = a
#f(1, z: 2)
EOF
lab/typst-original/target/release/typst compile /tmp/p733-named-extra.typ /tmp/p733-vanilla.pdf
```

Confirmar a mensagem de erro exacta.

```bash
cat > /tmp/p733-named-ok.typ <<'EOF'
#let f(a, named: 10) = (a, named)
#f(1, named: 20)
EOF
lab/typst-original/target/release/typst compile /tmp/p733-named-ok.typ /tmp/p733-ok-vanilla.pdf
pdftotext /tmp/p733-ok-vanilla.pdf -
```

Confirmar que argumentos nomeados válidos continuam a funcionar (caso de não-regressão).

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p733-named-extra.typ /tmp/p733-cristalino.pdf
echo "Exit code: $?"
```

### Localizar o código exacto

```bash
grep -n "args.named\|fn apply_closure" 01_core/src/rules/eval/closures.rs
```

Confirmar onde `args.named` é consultado durante o binding, e onde falta a verificação de "sobrou algum nome não consumido".

### Critério de fecho da sonda

- [ ] Mensagem de erro exacta do vanilla confirmada.
- [ ] Caso de não-regressão (nomeados válidos) confirmado.
- [ ] Localização exacta no código confirmada.

---

## Implementação

Depois do binding de todos os parâmetros (incluindo keyword-only), verificar se sobrou algum nome em `args.named` não consumido — se sim e não houver sink (`..args`), erro.

### Critério de fecho da implementação

- [ ] Argumento nomeado extra sem parâmetro produz erro, mensagem igual à do vanilla.
- [ ] Sink (`..args`) continua a absorver nomeados extra sem erro, se for esse o comportamento do vanilla — confirmar.
- [ ] Argumentos nomeados válidos sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p733-named-extra.typ /tmp/p733-depois.pdf
echo "Exit code: $?"
./target/release/typst /tmp/p733-named-ok.typ /tmp/p733-ok-depois.pdf
pdftotext /tmp/p733-ok-depois.pdf -
```

Comparar com o vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

Dado que isto toca o mesmo mecanismo central de P708, atenção a regressão silenciosa — confirmar que os testes de P708/P715/P724 continuam verdes.

---

## Critério de fecho do passo

- [ ] Sonda completa, mensagem de erro e caso de não-regressão confirmados.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`, testes de P708/P715/P724 confirmados intactos.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p733.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
