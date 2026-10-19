---
# P724 — Parâmetros de closure com padrão de desestruturação (`((i, x)) => ...`)

> **Passo:** 724
> **Data:** 2026-07-10
> **Foco:** P723 confirmou que `eval_closure_expr` descarta silenciosamente parâmetros com padrão de desestruturação (`_ => None, // Placeholder, Destructuring — adiado`), criando uma closure de zero parâmetros que depois rejeita qualquer argumento. Consumidor real: `path-util.typ:453` de `cetz` (`segments.enumerate().filter(((i, segment)) => ...)`). Exige mudança no modelo `ClosureParam` (guardar o pattern completo, não só o nome) e bind via `destructure_let` (P715/P723) em `apply_closure`.
> **Tipo:** Sonda + Implementação. Mudança estrutural, cuidado redobrado.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mudança no modelo de `ClosureParam`, usado por todo o mecanismo de chamada de closures (P708).
> **Dependências:** P723 (onde o bloqueio foi isolado com `file:line`), P708 (`ClosureParam`/`apply_closure`, a modificar), P715/P723 (`destructure_let`, a reaproveitar).

---

## Sonda

### Confirmar o comportamento completo no vanilla

```bash
cat > /tmp/p724-destr-param.typ <<'EOF'
#let pairs = ((1, "a"), (2, "b"), (3, "c"))
#pairs.map(((n, s)) => str(n) + s)

#let f = ((a, b), c) => a + b + c
#f((1, 2), 3)

#let g(x, (a, b)) = x + a + b
#g(10, (1, 2))
EOF
lab/typst-original/target/release/typst compile /tmp/p724-destr-param.typ /tmp/p724-vanilla.pdf
pdftotext /tmp/p724-vanilla.pdf -
```

Confirmar: desestruturação em parâmetro de closure funciona misturada com parâmetros normais? Funciona em definições nomeadas (`#let f(...) = ...`), não só em closures anónimas (`=>`)?

### Confirmar o mecanismo exacto do vanilla

```bash
grep -n "struct Param\|ClosureParam\|fn eval.*closure" lab/typst-original/crates/typst-eval/src/binding.rs lab/typst-original/crates/typst-library/src/foundations/func.rs 2>/dev/null | head -20
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p724-destr-param.typ /tmp/p724-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Comportamento completo confirmado (closures anónimas, nomeadas, misturado com parâmetros normais).
- [ ] Mecanismo exacto do vanilla confirmado.

---

## Implementação

Estender `ClosureParam` (ou a estrutura equivalente) para guardar o `Pattern` completo, não só o nome de um `Ident`. Em `apply_closure`, para parâmetros com pattern de desestruturação, usar `destructure_let` (já reaproveitado por P723 no `for`) para ligar os nomes internos do padrão ao valor do argumento correspondente.

### Critério de fecho da implementação

- [ ] Desestruturação em parâmetro de closure funciona, testada com os casos da sonda.
- [ ] Parâmetros normais (sem desestruturação) sem regressão — todo o mecanismo de P708 (binding posicional/keyword-only) continua correcto.
- [ ] Misturar parâmetros normais e de desestruturação na mesma assinatura funciona.

---

## Validação

```bash
./target/release/typst /tmp/p724-destr-param.typ /tmp/p724-depois.pdf
pdftotext /tmp/p724-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado que isto toca `ClosureParam`/`apply_closure` (mecanismo central, já uma vez causador de bug silencioso em P708), correr o corpus de testes já existente com atenção redobrada a regressões silenciosas, não só a falhas explícitas.

### Campos fixos de progresso

```bash
cat > /tmp/p724-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p724-cetz.typ /tmp/p724-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p724-cetz.png -r 150 /tmp/p724-cetz.pdf 2>/dev/null
```

Se produzir PDF: diff de pixels contra o vanilla.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado, incluindo mistura de parâmetros normais e de desestruturação.
- [ ] Sem regressão em `cargo test --workspace`, com atenção a regressão silenciosa dado o alcance do mecanismo tocado.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`ClosureParam`, desestruturação) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p724.md`, com hash do commit.
