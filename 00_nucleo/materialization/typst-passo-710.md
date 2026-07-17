---
# P710 — Métodos e campos de `Length` (`.pt()`, `.to-absolute()`, ...)

> **Passo:** 710
> **Data:** 2026-07-10
> **Foco:** P709 confirmou que `Value::Length` não tem nenhum campo ou método implementado. `cetz` usa `.to-absolute()` (confirmado, `canvas.typ:36`). O vanilla também expõe `.pt()`/`.mm()`/`.cm()`/`.inches()`/`.abs`, com regras específicas sobre a componente `em` (alguns métodos falham se não for zero; `.to-absolute()` precisa de contexto de estilo para a resolver).
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P709 (onde o bloqueio foi isolado, com `file:line` do vanilla).

---

## Sonda

### Confirmar a semântica exacta de cada método no vanilla

```bash
grep -n "impl Length\|pub fn " lab/typst-original/crates/typst-library/src/foundations/layout/length.rs
```

Não assumir a lista — confirmar todos os métodos/campos, e ler o corpo de cada um para entender o tratamento da componente `em`.

```bash
cat > /tmp/p710-length.typ <<'EOF'
#set text(size: 12pt)
#context [
  #(6pt).to-absolute()
  #(6pt + 10em).to-absolute()
  #(6pt).pt()
  #(6pt).mm()
  #(6pt).cm()
  #(6pt).inches()
  #(6pt + 1em).pt()
  #(6pt).abs
]
EOF
lab/typst-original/target/release/typst compile /tmp/p710-length.typ /tmp/p710-vanilla.pdf
pdftotext /tmp/p710-vanilla.pdf -
```

Confirmar quais métodos exigem contexto (`context [...]`), e o comportamento exacto quando a componente `em` não é zero para os métodos que a rejeitam.

### Confirmar quais destes `cetz` de facto usa

```bash
grep -rn "\.to-absolute()\|\.pt()\|\.mm()\|\.cm()\|\.inches()\|\.abs\b" ~/.cache/typst/packages/preview/cetz/0.5.2/src/*.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/**/*.typ 2>/dev/null
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p710-length.typ /tmp/p710-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Semântica exacta de cada método confirmada, incluindo tratamento de `em`.
- [ ] Métodos realmente usados por `cetz` confirmados, com `file:line`.

---

## Implementação

Implementar os métodos/campos confirmados como necessários pela sonda (pelo menos `.to-absolute()`, confirmado por P709; os outros conforme a segunda sonda revelar).

### Critério de fecho da implementação

- [ ] `.to-absolute()` funciona, com o contexto de estilo necessário.
- [ ] Outros métodos confirmados como usados por `cetz` implementados.
- [ ] Comportamento com componente `em` não-zero testado contra o vanilla (erro ou valor, conforme o método).

---

## Validação

```bash
./target/release/typst /tmp/p710-length.typ /tmp/p710-depois.pdf
pdftotext /tmp/p710-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-709

```bash
cat > /tmp/p710-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p710-cetz.typ /tmp/p710-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p710-cetz.png -r 150 /tmp/p710-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688. Se não: registar o próximo bloqueio, usando o tempo de compilação como sinal de progresso.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, semântica confirmada, incluindo tratamento de `em`.
- [ ] Métodos usados por `cetz` implementados e testados.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p710.md`, com hash do commit.
