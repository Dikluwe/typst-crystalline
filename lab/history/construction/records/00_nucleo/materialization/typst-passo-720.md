---
# P720 — Concatenação de arrays (`array + array`)

> **Passo:** 720
> **Data:** 2026-07-10
> **Foco:** P719 confirmou que `(1,2) + (3,4)` falha no cristalino ("cannot apply Add to array and array"), enquanto o vanilla concatena. Consumidor real, múltiplos sítios de `cetz`: `path-util.typ:423,430`, `bezier.typ:413`, `hobby.typ:77,78,126`.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S se for só `Array + Array`; M se `Dict + Dict` ou outras combinações relacionadas também estiverem em falta e forem necessárias.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P719 (onde o bloqueio foi isolado com `file:line` exacto de `cetz`).

---

## Sonda

### Confirmar o comportamento exacto no vanilla

```bash
cat > /tmp/p720-concat.typ <<'EOF'
#((1, 2) + (3, 4))
#(() + (1, 2))
#((1, 2) + ())
#((a: 1) + (b: 2))
#((a: 1) + (a: 99))
EOF
lab/typst-original/target/release/typst compile /tmp/p720-concat.typ /tmp/p720-vanilla.pdf
pdftotext /tmp/p720-vanilla.pdf -
```

Confirmar `Array + Array`, casos vazios, e `Dict + Dict` (se existir — confirmar comportamento com chaves repetidas, qual valor vence).

### Confirmar exactamente que combinações `cetz` usa

```bash
grep -n "+ (" ~/.cache/typst/packages/preview/cetz/0.5.2/src/path-util.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/bezier.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/hobby.typ
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p720-concat.typ /tmp/p720-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] `Array + Array` confirmado, incluindo casos vazios.
- [ ] `Dict + Dict` confirmado (comportamento, não assumido).
- [ ] Uso real em `cetz` confirmado, com `file:line`.
- [ ] **[scope-out]** Se `Dict + Dict` não tiver consumidor em `cetz`: decisão registada, medida.

---

## Implementação

Implementar `Array + Array` (concatenação), e `Dict + Dict` se confirmado necessário pela sonda.

### Critério de fecho da implementação

- [ ] `Array + Array` funciona, testado com casos vazios.
- [ ] `Dict + Dict`, se implementado, testado com chaves repetidas.
- [ ] **[scope-out]** Qualquer combinação não implementada, com razão medida.

---

## Validação

```bash
./target/release/typst /tmp/p720-concat.typ /tmp/p720-depois.pdf
pdftotext /tmp/p720-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p720-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p720-cetz.typ /tmp/p720-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p720-cetz.png -r 150 /tmp/p720-cetz.pdf 2>/dev/null
```

Registar: tempo de compilação, exit code, próximo bloqueio com `file:line` exacto — ou, se produzir PDF, diff de pixels com o vanilla, não só inspecção visual.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento confirmado.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados.
- [ ] Grep às ADRs em vigor pelos termos centrais (`Add`, concatenação) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p720.md`, com hash do commit.
