---
# P750 — Corrigir a fórmula de posicionamento da primeira baseline (`margem + cap-height`, não `margem + ascender`)

> **Passo:** 750
> **Data:** 2026-07-10
> **Foco:** P749 confirmou que o vanilla posiciona a primeira baseline de texto a `margem + cap-height`, enquanto o cristalino usa `margem + ascender` (mais um resíduo de ~1,23pt ainda não identificado). Isto não é uma diferença de fonte — foi confirmado directamente (forçar `Liberation Serif` no cristalino não mudou a baseline). Afecta a posição da primeira linha de texto em qualquer página, documento por padrão. Prioridade alta — mecanismo fundamental de layout, alcance universal.
> **Tipo:** Sonda + Implementação. Prioridade alta.
> **Tamanho:** M — inclui confirmar a fórmula exacta na fonte do vanilla, corrigir, e ainda caçar o resíduo de 1,23pt.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mecanismo central de layout, usado por todo o texto.
> **Dependências:** P749 (onde a causa foi isolada, mas não corrigida), P748 (correcção irmã, para formas).

---

## Contexto

Medido por P749: vanilla `baseline ≈ margem + cap_height` (78,10pt ≈ 70,87 + 7,24pt). Cristalino `baseline ≈ margem + ascender + resíduo` (81,90pt = 70,87 + 9,80(ish) + 1,23pt não identificado).

---

## Sonda

### Confirmar a fórmula exacta na fonte do vanilla, não só por medição

```bash
grep -rn "cap_height\|first.*baseline\|top.*edge.*text" lab/typst-original/crates/typst-layout/src/*.rs 2>/dev/null | head -20
```

Não assumir que é literalmente `cap-height` só porque o valor bate por coincidência — confirmar no código a métrica real usada.

### Confirmar mais casos, com tamanhos de fonte diferentes

```bash
cat > /tmp/p750-tamanhos.typ <<'EOF'
#set text(size: 8pt)
X

#set text(size: 24pt)
X
EOF
lab/typst-original/target/release/typst compile /tmp/p750-tamanhos.typ /tmp/p750-vanilla.pdf
```

Confirmar se a fórmula escala correctamente com o tamanho da fonte.

### Localizar o código exacto do cristalino a corrigir

```bash
grep -n "ascender\|cap.height\|cursor_y = margin" 01_core/src/engine/layout/mod.rs 01_core/src/entities/layout_types.rs 2>/dev/null | head -20
```

### Caçar o resíduo de 1,23pt

Confirmar se o resíduo desaparece naturalmente ao trocar `ascender` por `cap-height`, ou se persiste como um problema à parte.

### Critério de fecho da sonda

- [ ] Fórmula exacta confirmada na fonte do vanilla, não só por coincidência numérica.
- [ ] Confirmado com mais de um tamanho de fonte, escalando correctamente.
- [ ] Código do cristalino a corrigir localizado com `file:line`.
- [ ] Confirmado se o resíduo de 1,23pt desaparece com a correcção da fórmula, ou é um problema separado.

---

## Implementação

Corrigir o cálculo da posição inicial do cursor de texto para usar a métrica confirmada pela sonda, em vez de `ascender`.

### Critério de fecho da implementação

- [ ] Primeira baseline corrigida, testada com múltiplos tamanhos de fonte.
- [ ] Resíduo de 1,23pt resolvido ou explicado.
- [ ] Texto multi-linha sem regressão.

---

## Validação

```bash
cat > /tmp/p750-texto.typ <<'EOF'
X
EOF
./target/release/typst /tmp/p750-texto.typ /tmp/p750-depois.pdf
mutool show /tmp/p750-depois.pdf 4 2>&1 | head -10
```

```bash
cargo test --workspace
crystalline-lint .
```

Correr o corpus completo com atenção a regressão silenciosa.

### Repetir a reprodução final de `cetz` e o teste de formas de P748

```bash
cat > /tmp/p750-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p750-cetz.typ /tmp/p750-cetz.pdf
mutool draw -o /tmp/p750-cetz.png -r 150 /tmp/p750-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p750-cetz.typ /tmp/p750-cetz-vanilla.pdf
mutool draw -o /tmp/p750-cetz-vanilla.png -r 150 /tmp/p750-cetz-vanilla.pdf
python3 /tmp/pngdiff.py /tmp/p750-cetz-vanilla.png /tmp/p750-cetz.png
```

---

## Critério de fecho do passo

- [ ] Sonda completa, fórmula confirmada na fonte, testada com múltiplos tamanhos.
- [ ] Resíduo de 1,23pt resolvido ou explicado.
- [ ] Corrigido e testado, incluindo texto multi-linha.
- [ ] Sem regressão em `cargo test --workspace`, corpus completo verificado.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` e o teste de formas de P748 sem regressão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p750.md`, com hash do commit.
