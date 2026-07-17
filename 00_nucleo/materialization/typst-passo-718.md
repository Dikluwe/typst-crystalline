---
# P718 — Spread em literais de array/dict (`(1, ..(2, 3))`)

> **Passo:** 718
> **Data:** 2026-07-10
> **Foco:** P717 confirmou que `#let x = (1, ..(2, 3))` dá `(1)` (length 1) no cristalino, em vez de `(1, 2, 3)` (length 3) no vanilla — spread em literal de array é ignorado. Consumidor real: `coordinate.typ:440` (`return (ctx, ..result)`), cujo retorno alimenta desestruturações de 2 elementos, causando o bloqueio actual de `cetz`. P717 notou também que `eval_args` (chamadas de função) tem a mesma fronteira deliberada — confirmar se também precisa de correcção, ou se é um caso genuinamente diferente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se for só literais de array/dict; M se `eval_args` também precisar de mudança.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P717 (onde o bloqueio foi isolado com `file:line` exacto de `cetz`).

---

## Sonda

### Confirmar o comportamento completo de spread em literais no vanilla

```bash
cat > /tmp/p718-spread.typ <<'EOF'
#let a = (1, ..(2, 3), 4)
#a
#let d = (a: 1, ..(b: 2, c: 3), d: 4)
#d
#let vazio = (..(), 1, ..())
#vazio
EOF
lab/typst-original/target/release/typst compile /tmp/p718-spread.typ /tmp/p718-vanilla.pdf
pdftotext /tmp/p718-vanilla.pdf -
```

Confirmar spread no meio (não só no fim), spread de dict dentro de literal de dict, e spread de valor vazio.

### Confirmar se `eval_args` (spread em chamadas de função) é um problema separado ou relacionado

```bash
grep -n "fn eval_args\|Arg::Spread" 01_core/src/engine/eval/closures.rs
```

```bash
cat > /tmp/p718-call-spread.typ <<'EOF'
#let f(a, b, c) = a + b + c
#f(1, ..(2, 3))
EOF
lab/typst-original/target/release/typst compile /tmp/p718-call-spread.typ /tmp/p718-call-vanilla.pdf
./target/release/typst /tmp/p718-call-spread.typ /tmp/p718-call-cristalino.pdf
echo "Exit code: $?"
```

Confirmar se este segundo caso já funciona, ou se é o mesmo tipo de gap, também bloqueando `cetz` nalgum ponto ainda não alcançado.

### Confirmar o estado actual do cristalino para o caso de literais

```bash
./target/release/typst /tmp/p718-spread.typ /tmp/p718-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Comportamento completo de spread em literais confirmado (array, dict, meio, vazio).
- [ ] Confirmado se `eval_args` tem o mesmo gap ou já funciona.
- [ ] **[scope-out]** Se `eval_args` tiver o gap mas sem consumidor confirmado em `cetz`: decisão registada aqui, medida, não estimada.

---

## Implementação

Implementar spread em literais de array/dict, seguindo o comportamento confirmado pela sonda. Se `eval_args` também precisar de correcção e tiver consumidor real, incluir no mesmo passo (mesmo mecanismo de fundo, spread de uma sequência/dict noutra estrutura).

### Critério de fecho da implementação

- [ ] Spread em literal de array funciona, em qualquer posição (início, meio, fim).
- [ ] Spread em literal de dict funciona.
- [ ] `eval_args` corrigido, se confirmado necessário.
- [ ] **[scope-out]** Qualquer forma não implementada, com razão medida.

---

## Validação

```bash
./target/release/typst /tmp/p718-spread.typ /tmp/p718-depois.pdf
pdftotext /tmp/p718-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p718-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p718-cetz.typ /tmp/p718-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p718-cetz.png -r 150 /tmp/p718-cetz.pdf 2>/dev/null
```

Registar: tempo de compilação, exit code, próximo bloqueio com `file:line` exacto — ou, se produzir PDF, comparar com `mutool draw` do lado do vanilla e, se possível, um diff de pixels com limiar, não só inspecção visual.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado, `eval_args` esclarecido.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (spread, `Arg::Spread`) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p718.md`, com hash do commit.
