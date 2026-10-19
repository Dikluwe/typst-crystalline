---
# P709 — Implementar módulo `std` (acesso à stdlib não-sombreada)

> **Passo:** 709
> **Data:** 2026-07-10
> **Foco:** P708 confirmou que `std` (módulo que dá acesso ao scope global padrão, mesmo quando o utilizador sombreou um nome com `#let`) está ausente. `cetz` usa `std.length`, `std.measure`, `std.color`, `std.stroke`, etc. quando o próprio pacote define algo com o mesmo nome que um builtin. Este passo implementa `std` como um snapshot imutável do scope base, anterior a qualquer binding do utilizador.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P708 (onde o bloqueio foi isolado com reprodução mínima).

---

## Sonda

### Confirmar a natureza exacta de `std` no vanilla

```bash
grep -rn "\"std\"\|struct.*Std\|fn std" lab/typst-original/crates/typst-library/src/lib.rs lab/typst-original/crates/typst-eval/src/*.rs 2>/dev/null | head -20
```

Confirmar: `std` é um módulo construído uma vez, com uma cópia do scope global padrão (antes de qualquer `#let` do documento), acessível em qualquer ponto do documento independentemente de quantos nomes tenham sido sombreados entretanto?

### Confirmar o comportamento com vários níveis de sombreamento

```bash
cat > /tmp/p709-std.typ <<'EOF'
#let length = 5
#(std.length)

#let calc = "não sou a calculadora"
#(std.calc.round(3.7))
EOF
lab/typst-original/target/release/typst compile /tmp/p709-std.typ /tmp/p709-vanilla.pdf
pdftotext /tmp/p709-vanilla.pdf -
```

Confirmar que `std` continua a funcionar mesmo com múltiplos níveis de sombreamento, e que aceder a sub-módulos através de `std` (como `std.calc`) também funciona.

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p709-std.typ /tmp/p709-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Natureza de `std` confirmada (snapshot imutável do scope base).
- [ ] Comportamento com sombreamento e sub-módulos confirmado.

---

## Implementação

Registar `std` no scope global como um `Value::Module` (ou equivalente) contendo uma cópia do scope base construído por `make_stdlib` (ou a função equivalente), antes de qualquer binding do utilizador ser aplicado.

### Critério de fecho da implementação

- [ ] `std.nome` funciona mesmo depois de `nome` ser sombreado por `#let`.
- [ ] Sub-módulos acessíveis através de `std` (`std.calc.round(...)`).
- [ ] `std` em si não pode ser sombreado de forma a quebrar o seu próprio funcionamento (confirmar o que o vanilla faz se o utilizador tentar `#let std = ...`).

---

## Validação

```bash
./target/release/typst /tmp/p709-std.typ /tmp/p709-depois.pdf
pdftotext /tmp/p709-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-708

```bash
cat > /tmp/p709-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p709-cetz.typ /tmp/p709-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p709-cetz.png -r 150 /tmp/p709-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688 — este seria o fecho real da cadeia P678-709. Se não: registar o próximo bloqueio, usando o tempo de compilação como sinal de progresso.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, natureza de `std` confirmada.
- [ ] Implementado e testado, incluindo sombreamento e sub-módulos.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p709.md`, com hash do commit.
