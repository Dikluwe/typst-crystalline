---
# P703 — `rgb()` deve aceitar hex/string de 1 argumento, além de 3-4 `Int`

> **Passo:** 703
> **Data:** 2026-07-10
> **Foco:** P702 isolou que `rgb("#FF0000")` (forma de 1 argumento, string hexadecimal) falha no cristalino, que só aceita `rgb(r, g, b)`/`rgb(r, g, b, a)` com inteiros. `cetz` usa esta forma na sua paleta de cores pré-definidas (`palette.typ`, `.map(rgb)` sobre strings hex). Este passo estende `rgb()` para a forma que falta.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P702 (onde o bloqueio foi isolado com reprodução mínima).

---

## Sonda

### Confirmar todas as formas que `rgb()` aceita no vanilla

```bash
cat > /tmp/p703-rgb.typ <<'EOF'
#rgb(255, 0, 0)
#rgb(255, 0, 0, 128)
#rgb("#FF0000")
#rgb("FF0000")
#rgb("#FF0000FF")
#rgb("red")
EOF
lab/typst-original/target/release/typst compile /tmp/p703-rgb.typ /tmp/p703-vanilla.pdf
pdftotext /tmp/p703-vanilla.pdf -
```

Confirmar: aceita hex com e sem `#`? Aceita hex de 3, 6, ou 8 dígitos (com/sem alfa)? Aceita nomes de cor como string (improvável, mas confirmar, já que `red` já existe como valor global desde P687)?

### Confirmar o código fonte, não só o comportamento observado

```bash
grep -n "fn rgb\|impl.*Color" lab/typst-original/crates/typst-library/src/visualize/color.rs | head -20
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p703-rgb.typ /tmp/p703-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Todas as formas de `rgb()` confirmadas contra o vanilla, incluindo formatos de hex (com/sem `#`, 3/6/8 dígitos).

---

## Implementação

Estender `native_rgb` para aceitar um único argumento `Value::Str`, parseando como cor hexadecimal, seguindo exactamente os formatos confirmados pela sonda.

### Critério de fecho da implementação

- [ ] `rgb("#FF0000")` e as outras formas confirmadas funcionam.
- [ ] Forma de 3/4 inteiros (já existente) sem regressão.
- [ ] Formato hex inválido produz erro claro.

---

## Validação

```bash
./target/release/typst /tmp/p703-rgb.typ /tmp/p703-depois.pdf
pdftotext /tmp/p703-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-702

```bash
cat > /tmp/p703-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p703-cetz.typ /tmp/p703-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p703-cetz.png -r 150 /tmp/p703-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688 — se coincidir, a cadeia P678-703 fecha, com um pacote real da comunidade a funcionar de ponta a ponta pela primeira vez. Se não: usar o tempo de compilação como sinal de progresso (como P702 fez), e registar o próximo bloqueio.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, formas de `rgb()` confirmadas contra o vanilla.
- [ ] Implementado e testado.
- [ ] Forma de inteiros sem regressão.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado com o sinal de progresso (tempo de compilação).
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p703.md`, com hash do commit.
