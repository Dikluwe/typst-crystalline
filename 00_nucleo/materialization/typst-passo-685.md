---
# P685 — Expor nomes de tipos (`length`, `ratio`, `int`, ...) como valores no scope

> **Passo:** 685
> **Data:** 2026-07-10
> **Foco:** P683 encontrou que `cetz` usa `type(x) == length` e `type(length) in (typst-length, ratio)` — tratando `length`, `ratio`, e outros nomes de tipo como valores de primeira classe no scope global, comparáveis directamente. O cristalino só tem `type()` como função; os nomes de tipo não estão ligados como valores. Este é o próximo bloqueio confirmado de `cetz`.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P683 (onde o bloqueio foi encontrado, com `file:line` do `cetz`).

---

## Sonda mínima

### Confirmar todos os tipos que o vanilla expõe como valores no scope

```bash
cat > /tmp/p685-tipos.typ <<'EOF'
#type(1) == int
#type(1.0) == float
#type(1pt) == length
#type(50%) == ratio
#type("s") == str
#type(()) == array
#type((:)) == dictionary
#type(true) == bool
#(int)
EOF
lab/typst-original/target/release/typst compile /tmp/p685-tipos.typ /tmp/p685-vanilla.pdf
pdftotext /tmp/p685-vanilla.pdf -
```

Confirmar a lista completa de nomes de tipo disponíveis como valores globais, e o que `#(int)` (o valor em si, não comparado) produz — provavelmente algo como um `Value::Type` que faz `repr` para o nome do tipo.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p685-tipos.typ /tmp/p685-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Lista completa de tipos expostos como valores confirmada.
- [ ] Confirmado o que o valor do tipo em si produz (`repr`, comparação, etc.).

---

## Implementação

Adicionar cada nome de tipo standard ao scope global como um valor (`Value::Type` ou equivalente já usado internamente por `type()`), de forma que `length`, `ratio`, `int`, etc. sejam identificadores válidos fora de uma chamada a `type()`, e que `type(x) == length` funcione por comparação directa de valores de tipo.

### Critério de fecho da implementação

- [ ] Todos os tipos confirmados pela sonda expostos como valores no scope global.
- [ ] `type(x) == length` (e equivalentes para outros tipos) funciona.
- [ ] `type()` continua a funcionar como já funcionava, sem regressão.
- [ ] Nomes de variáveis do utilizador que coincidam com nomes de tipo (por exemplo, `#let length = 5`) sombreiam correctamente o valor de tipo global, seguindo as regras normais de escopo.

---

## Validação

```bash
./target/release/typst /tmp/p685-tipos.typ /tmp/p685-depois.pdf
pdftotext /tmp/p685-depois.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

### Confirmar que `cetz` avança mais

```bash
cat > /tmp/p685-cetz.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({
  draw.line((0,0), (1,1))
})
EOF
./target/release/typst /tmp/p685-cetz.typ /tmp/p685-cetz.pdf
echo "Exit code: $?"
```

Mesma disciplina dos passos anteriores: registar o próximo estado com honestidade, não assumir que `cetz` está resolvido só porque este bloqueio específico desapareceu.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, lista de tipos confirmada.
- [ ] Tipos expostos como valores, testados contra o vanilla.
- [ ] Sombreamento por variável do utilizador testado.
- [ ] `cetz` re-testado, próximo estado registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p685.md`, com hash do commit.
