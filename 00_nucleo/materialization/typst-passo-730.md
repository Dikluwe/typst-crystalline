---
# P730 — `Array.slice(start, end, count)`

> **Passo:** 730
> **Data:** 2026-07-10
> **Foco:** P728/P729 confirmaram que `Array` não tem `.slice()` implementado (só `Str.slice`, `01_core/src/engine/stdlib/collections.rs:79`). Consumidor real, pesado: `cetz` usa `array.slice` em pelo menos dez sítios (`draw/shapes.typ`, `anchor.typ`, `coordinate.typ`, `drawable.typ`). É o bloqueio actual, único conhecido, da reprodução completa de `cetz`.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S — mesmo padrão já usado para `Str.slice`, só a generalizar para `Array`.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P728 (onde o bloqueio foi encontrado com `file:line` exacto).

---

## Sonda

### Confirmar o comportamento exacto no vanilla

```bash
cat > /tmp/p730-slice.typ <<'EOF'
#((1,2,3,4).slice(1, 3))
#((1,2,3,4).slice(-2))
#((1,2,3,4).slice(1))
#((1,2,3,4).slice(0, count: 2))
#((1,2,3,4).slice(-3, -1))
#(().slice(0))
EOF
lab/typst-original/target/release/typst compile /tmp/p730-slice.typ /tmp/p730-vanilla.pdf
pdftotext /tmp/p730-vanilla.pdf -
```

Confirmar a assinatura completa (`start`, `end` opcional, `count:` nomeado opcional, índices negativos), e o comportamento com array vazio.

### Confirmar a fonte do vanilla

```bash
grep -n "fn slice" lab/typst-original/crates/typst-library/src/foundations/array.rs
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p730-slice.typ /tmp/p730-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Assinatura completa confirmada (posicionais, `count:`, índices negativos, array vazio).

---

## Implementação

Adicionar `Array.slice` ao dispatcher de métodos (`try_dispatch_collection_method`, mesmo mecanismo já usado para `Array.at`, P714), seguindo a mesma lógica de `locate_opt` já estabelecida.

### Critério de fecho da implementação

- [ ] `Array.slice` implementado, testado com todos os casos da sonda.
- [ ] `Str.slice` (já existente) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p730-slice.typ /tmp/p730-depois.pdf
pdftotext /tmp/p730-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Reprodução final de `cetz` — diff de pixels

```bash
cat > /tmp/p730-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p730-cetz.typ /tmp/p730-cetz.pdf
mutool draw -o /tmp/p730-cetz.png -r 150 /tmp/p730-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p730-cetz.typ /tmp/p730-cetz-vanilla.pdf
mutool draw -o /tmp/p730-cetz-vanilla.png -r 150 /tmp/p730-cetz-vanilla.pdf
```

Diff de pixels — se aproximar de zero, a cadeia P678-730 fecha de vez, com prova visual. Se não: registar o próximo bloqueio com os campos fixos habituais (tempo, exit code, `file:line`).

---

## Critério de fecho do passo

- [ ] Sonda completa, assinatura confirmada.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — diff de pixels final registado com número exacto, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p730.md`, com hash do commit.
- [ ] Item marcado como fechado em `achados-adiados-cetz.md`.
- [ ] Se a cadeia fechar de vez: resumo completo, número de passos, categorias de bugs corrigidos.
