---
# P725 — `Length * Int`/`Float` (as quatro combinações)

> **Passo:** 725
> **Data:** 2026-07-10
> **Foco:** P724 confirmou que `operators.rs` não tem nenhum braço `Mul` com `Length` — só `Length / Int|Float` (P713). Consumidor real: `canvas.typ:146-147,182-186` de `cetz` (`(x - offset) * length`, escala de coordenadas). As quatro combinações (`Length*Int`, `Int*Length`, `Length*Float`, `Float*Length`) confirmadas em falta e a funcionar no vanilla.
> **Tipo:** Sonda mínima + Implementação. Correcção pequena e bem isolada.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P724 (onde o bloqueio foi isolado com operando e `file:line` exactos), P713 (`Length / Int|Float`, padrão já estabelecido a espelhar).

---

## Sonda mínima

### Confirmar as quatro combinações e casos de borda

```bash
cat > /tmp/p725-mul.typ <<'EOF'
#(2.0 * 1pt)
#(1pt * 2.0)
#(2 * 1pt)
#(1pt * 2)
#(0 * 1pt)
#(-1 * 1pt)
EOF
lab/typst-original/target/release/typst compile /tmp/p725-mul.typ /tmp/p725-vanilla.pdf
pdftotext /tmp/p725-vanilla.pdf -
```

Confirmar overflow/NaN (multiplicar por um número muito grande) — erro, ou `inf`/comportamento silencioso?

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p725-mul.typ /tmp/p725-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Quatro combinações confirmadas, incluindo zero e negativo.

---

## Implementação

Adicionar os quatro braços em `eval_binary_op`, espelhando o padrão já estabelecido por `Length / Int|Float` (P713) — multiplicação directa sobre `Abs`/`Em`.

### Critério de fecho da implementação

- [ ] As quatro combinações implementadas e testadas.
- [ ] `Length / Int|Float` (P713) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p725-mul.typ /tmp/p725-depois.pdf
pdftotext /tmp/p725-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p725-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p725-cetz.typ /tmp/p725-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p725-cetz.png -r 150 /tmp/p725-cetz.pdf 2>/dev/null
```

Se produzir PDF: diff de pixels contra o vanilla, não só inspecção visual.

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`Mul`, `Length`) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p725.md`, com hash do commit.
