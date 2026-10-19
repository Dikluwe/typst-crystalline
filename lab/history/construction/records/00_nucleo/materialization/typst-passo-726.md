---
# P726 — `fill:`/`stroke: none` em `block` (e verificar `box`/`grid`/`table`)

> **Passo:** 726
> **Data:** 2026-07-10
> **Foco:** P725 confirmou que `block(fill: none)` rejeita `none` ("espera Color, recebeu none"), enquanto o vanilla aceita `none` como "sem preenchimento". O mesmo padrão provavelmente afecta `stroke:` (previsto, não confirmado) e possivelmente `box`/`grid`/`table` (mesma família de validação, `layout.rs:321,1098`, `structural.rs:685`). Consumidor real: `canvas.typ:111,129` de `cetz`.
> **Tipo:** Sonda ampla + Implementação. Verificar todas as funções da família de uma vez, não uma a uma.
> **Tamanho:** M — várias funções, mesmo padrão de correcção.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P725 (onde o bloqueio foi isolado e a extensão do padrão prevista).

---

## Sonda ampla

### Confirmar `fill:`/`stroke: none` em todas as funções da família, no vanilla

```bash
cat > /tmp/p726-none.typ <<'EOF'
#block(fill: none)[x]
#block(stroke: none)[x]
#box(fill: none)[x]
#box(stroke: none)[x]
#rect(fill: none, stroke: none)[x]
#grid(fill: none, stroke: none, [a], [b])
#table(fill: none, stroke: none, [a], [b])
EOF
lab/typst-original/target/release/typst compile /tmp/p726-none.typ /tmp/p726-vanilla.pdf
echo "Exit code: $?"
```

Confirmar que todas aceitam `none` sem erro.

### Confirmar o estado actual do cristalino, função a função

```bash
for f in block box rect grid table; do
  echo "=== $f ==="
  cat > /tmp/p726-$f.typ <<EOF
#$f(fill: none)[x]
EOF
  ./target/release/typst /tmp/p726-$f.typ /tmp/p726-$f.pdf 2>&1
done
```

Não assumir que só `block` tem o problema — testar cada função individualmente.

### Localizar todos os pontos de validação no código

```bash
grep -n "espera Color\|extract_stroke\|expect.*Color" 01_core/src/engine/stdlib/layout.rs 01_core/src/engine/eval/structural.rs
```

### Critério de fecho da sonda ampla

- [ ] Todas as funções da família testadas individualmente contra o vanilla, não assumidas.
- [ ] Todos os pontos de validação no código localizados.

---

## Implementação

Corrigir cada ponto de validação confirmado pela sonda para aceitar `none` como valor válido para `fill`/`stroke`, com o significado de "sem preenchimento"/"sem traço".

### Critério de fecho da implementação

- [ ] `fill: none`/`stroke: none` aceites em todas as funções confirmadas pela sonda.
- [ ] Comportamento de `Color`/`Stroke` explícitos sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p726-none.typ /tmp/p726-depois.pdf
echo "Exit code: $?"
```

Comparar com o vanilla (exit 0 nos dois lados).

```bash
cargo test --workspace
crystalline-lint .
```

### Campos fixos de progresso

```bash
cat > /tmp/p726-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p726-cetz.typ /tmp/p726-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p726-cetz.png -r 150 /tmp/p726-cetz.pdf 2>/dev/null
```

Se produzir PDF: diff de pixels contra o vanilla — este pode finalmente ser o fecho da cadeia P678-726.

---

## Critério de fecho do passo

- [ ] Sonda ampla completa, todas as funções da família testadas.
- [ ] Implementado e testado em todas as funções confirmadas.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — campos fixos de progresso registados, ou sucesso completo com diff de pixels.
- [ ] Grep às ADRs em vigor pelos termos centrais (`fill`, `stroke`, `none`) antes de fechar o texto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p726.md`, com hash do commit.
- [ ] Bug de render de `curve` (lista de controlo) — confirmar se ainda bloqueia depois deste passo, dado que P723 o encontrou mas não investigou a fundo.
