---
# P729 — Auditoria: `join` sequencial em todos os corpos de bloco

> **Passo:** 729
> **Data:** 2026-07-10
> **Foco:** P728 corrigiu `join` sequencial só em `Expr::CodeBlock`. Mas `if`/`else`, corpos de `for`/`while`, e corpos de função podem ter as suas próprias implementações separadas do mesmo mecanismo ("sequência de expressões → um valor"), com o mesmo bug de "só a última expressão" ainda por corrigir — o mesmo padrão de duplicação já visto em P715 (`let`/atribuição), P723 (`for`), P724 (parâmetros de closure). Este passo é uma auditoria sistemática, não a perseguição de mais um bloqueio de `cetz`.
> **Tipo:** Sonda ampla + Implementação. Prioridade alta — mesma categoria dos bugs de P728.
> **Tamanho:** M-L, dependendo de quantos sítios distintos existirem.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mecanismo central de avaliação.
> **Dependências:** P728 (onde o bug de `join` foi encontrado e corrigido para `Expr::CodeBlock`).

---

## Sonda

### Confirmar, no vanilla, que o mesmo `join` sequencial se aplica a todos os corpos

```bash
cat > /tmp/p729-join-todos.typ <<'EOF'
#if true { (1,); (2,) }
#if false { (9,) } else { (1,); (2,) }
#let f() = { (1,); (2,) }
#f()
#for i in (1,) { (1,); (2,) }
#let i = 0
#while i < 1 { i += 1; (1,); (2,) }
EOF
lab/typst-original/target/release/typst compile /tmp/p729-join-todos.typ /tmp/p729-vanilla.pdf
pdftotext /tmp/p729-vanilla.pdf -
```

Confirmar que todos produzem `(1, 2)` (join), não só o último valor.

### Confirmar o estado actual do cristalino, um a um

```bash
./target/release/typst /tmp/p729-join-todos.typ /tmp/p729-cristalino.pdf
pdftotext /tmp/p729-cristalino.pdf -
```

Comparar linha a linha com o vanilla — identificar exactamente quais construtos já estão correctos (por delegarem em `Expr::CodeBlock`, já corrigido por P728) e quais têm implementação própria, ainda por corrigir.

### Localizar cada implementação separada no código

```bash
grep -n "fn eval_if\|fn eval_for\|fn eval_while\|fn eval_closure\|last = \|output = " 01_core/src/engine/eval/*.rs | grep -v test
```

Para cada construto que a sonda anterior mostrar como ainda errado, localizar a implementação exacta e confirmar se seria corrigido reaproveitando `operators::join` (já criado por P728), ou se precisa de lógica própria.

### Critério de fecho da sonda

- [ ] Todos os construtos testados contra o vanilla, um a um, não assumidos.
- [ ] Lista completa de construtos com bug confirmado, com `file:line` de cada implementação separada.
- [ ] Confirmado se `operators::join` (P728) é directamente reaproveitável em cada caso.

---

## Implementação

Para cada construto confirmado com o bug, aplicar `operators::join` (P728), seguindo o mesmo padrão já estabelecido.

### Critério de fecho da implementação

- [ ] Todos os construtos confirmados corrigidos.
- [ ] `Expr::CodeBlock` (já corrigido por P728) sem regressão.
- [ ] Construtos já correctos (se algum já delegava correctamente) confirmados sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p729-join-todos.typ /tmp/p729-depois.pdf
pdftotext /tmp/p729-depois.pdf -
```

Comparar com o vanilla já obtido na sonda — todos os seis casos devem coincidir.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance (mesma categoria de P728, mecanismo central), correr o corpus completo com atenção a regressão silenciosa.

### Repetir a reprodução de `cetz`

```bash
cat > /tmp/p729-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p729-cetz.typ /tmp/p729-cetz.pdf
echo "Exit code: $?"
```

Confirmar se este passo, por acidente, avança `cetz` para lá do bloqueio de `array.slice` já conhecido (não é o objectivo deste passo, mas vale registar se acontecer).

---

## Critério de fecho do passo

- [ ] Sonda ampla completa, todos os construtos testados individualmente.
- [ ] Cada construto com bug confirmado, corrigido e testado.
- [ ] Sem regressão em `cargo test --workspace`, atenção a regressão silenciosa.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p729.md`, com hash do commit.
- [ ] Lista de controlo (`achados-adiados-cetz.md`) actualizada, se aplicável.
