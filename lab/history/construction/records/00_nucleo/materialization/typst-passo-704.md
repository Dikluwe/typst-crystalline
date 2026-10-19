---
# P704 — `range()` deve aceitar `step:`

> **Passo:** 704
> **Data:** 2026-07-10
> **Foco:** P703 isolou que `range(90, 40, step: -12)` falha no cristalino, que só aceita `range(n)`/`range(start, end)`, sem `step:`. `cetz` usa isto na mesma `palette.typ` que motivou o trabalho de `rgb()` em P703. Este passo estende `range()` para aceitar o argumento nomeado em falta.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P703 (onde o bloqueio foi isolado com reprodução mínima e confirmação directa contra o vanilla).

---

## Sonda

### Confirmar a semântica completa de `step:` no vanilla

```bash
cat > /tmp/p704-range.typ <<'EOF'
#range(90, 40, step: -12)
#range(0, 10, step: 2)
#range(0, 10, step: 3)
#range(10, 0, step: -1)
#range(0, 10, step: -1)
#range(0, 10, step: 0)
EOF
lab/typst-original/target/release/typst compile /tmp/p704-range.typ /tmp/p704-vanilla.pdf
pdftotext /tmp/p704-vanilla.pdf -
```

Confirmar: a direcção de paragem segue o sinal do `step` (passo negativo com `start > end` avança para trás correctamente; passo positivo com `start > end` produz sequência vazia, não erro)? `step: 0` produz erro, e qual a mensagem exacta?

### Confirmar o código fonte

```bash
grep -n "fn range\|step" lab/typst-original/crates/typst-library/src/foundations/calc.rs 2>/dev/null
grep -rn "fn native_range\|pub fn range" lab/typst-original/crates/typst-library/src/foundations/*.rs | head -10
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p704-range.typ /tmp/p704-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Semântica completa de `step:` confirmada, incluindo casos de borda (passo zero, direcção incompatível com start/end).

---

## Implementação

Estender `native_range` para aceitar o argumento nomeado `step`, seguindo exactamente a semântica confirmada pela sonda.

### Critério de fecho da implementação

- [ ] `range(start, end, step: n)` funciona para `n` positivo e negativo, testado contra o vanilla.
- [ ] `step: 0` produz o erro certo.
- [ ] Direcção incompatível (passo positivo com `start > end`, ou vice-versa) produz sequência vazia, não erro, se for isso que o vanilla faz.
- [ ] Formas já existentes (`range(n)`, `range(start, end)`, sem `step`) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p704-range.typ /tmp/p704-depois.pdf
pdftotext /tmp/p704-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-703

```bash
cat > /tmp/p704-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p704-cetz.typ /tmp/p704-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p704-cetz.png -r 150 /tmp/p704-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688 — se coincidir, a cadeia P678-704 fecha com um pacote real da comunidade a funcionar de ponta a ponta. Se não: usar o tempo de compilação como sinal de progresso, e registar o próximo bloqueio.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, semântica de `step:` confirmada contra o vanilla.
- [ ] Implementado e testado, incluindo casos de borda.
- [ ] Formas existentes sem regressão.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p704.md`, com hash do commit.
