---
# P702 — Implementar `Func::with()`, aplicação parcial de argumentos

> **Passo:** 702
> **Data:** 2026-07-10
> **Foco:** P701 isolou, através de `cetz`, que `.with(...)` (aplicação parcial de argumentos, disponível em qualquer função no vanilla) não existe no cristalino. Não é específico de `cbor`, plugins, ou `cetz` — é um mecanismo geral da linguagem, usado tipicamente para pré-configurar funções (por exemplo, `calc.round.with(digits: 2)`, ou numeração/formatação customizada em muitos documentos reais). Este passo implementa o mecanismo em geral, não só o suficiente para desbloquear `cetz`.
> **Tipo:** Sonda + Implementação. Prioridade alta, dado o alcance provável além de `cetz`.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P701 (onde a ausência foi isolada, com reprodução mínima).

---

## Sonda

### Confirmar como `Func::with` funciona no vanilla

```bash
grep -n "fn with\|struct.*With\|FuncRepr" lab/typst-original/crates/typst-library/src/foundations/func.rs | head -30
```

Confirmar a estrutura interna — provavelmente um novo `FuncRepr` (ou variante) que envolve a função original e um conjunto de argumentos pré-preenchidos, delegando à função original na chamada final, combinando os argumentos pré-ligados com os novos.

### Confirmar o comportamento exacto com vários tipos de função

```bash
cat > /tmp/p702-with.typ <<'EOF'
#let f = calc.round.with(digits: 2)
#f(3.14159)

#let g(a, b, c) = a + b + c
#let g2 = g.with(1, 2)
#g2(3)

#let h(a, named: 10) = a + named
#let h2 = h.with(named: 20)
#h2(5)
EOF
lab/typst-original/target/release/typst compile /tmp/p702-with.typ /tmp/p702-vanilla.pdf
pdftotext /tmp/p702-vanilla.pdf -
```

Confirmar: funciona com funções nativas (`calc.round`), closures definidas pelo utilizador, argumentos posicionais e nomeados, misturados. Confirmar também se `.with()` pode ser encadeado (`f.with(a: 1).with(b: 2)`).

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p702-with.typ /tmp/p702-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda

- [ ] Estrutura interna de `Func::with` confirmada no código fonte do vanilla.
- [ ] Comportamento confirmado com funções nativas, closures, argumentos posicionais e nomeados, e encadeamento.

---

## Implementação

Adicionar `.with(...)` como método disponível em qualquer `Value::Func` (nativo, closure, ou elemento), devolvendo uma nova função que, ao ser chamada, combina os argumentos pré-ligados com os novos argumentos, delegando à função original.

### Critério de fecho da implementação

- [ ] `.with(...)` funciona em funções nativas (`calc.round.with(...)`).
- [ ] `.with(...)` funciona em closures definidas pelo utilizador.
- [ ] Argumentos posicionais e nomeados pré-ligados correctamente.
- [ ] Encadeamento de `.with()` funciona, se confirmado pela sonda.
- [ ] `it.with(...)` em elementos/`Content` (mecanismo já existente, distinto) sem regressão — confirmar que os dois mecanismos coexistem sem colisão.

---

## Validação

```bash
./target/release/typst /tmp/p702-with.typ /tmp/p702-depois.pdf
pdftotext /tmp/p702-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P701/P700

```bash
cat > /tmp/p702-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p702-cetz.typ /tmp/p702-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p702-cetz.png -r 150 /tmp/p702-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688 — se coincidir, a cadeia P678-702 fecha com um pacote real da comunidade a funcionar de ponta a ponta. Se não: registar o próximo bloqueio com honestidade.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, estrutura e comportamento de `.with()` confirmados contra o vanilla.
- [ ] Implementado para todos os tipos de função (nativa, closure), testado.
- [ ] Mecanismo já existente (`it.with(...)` em elementos) sem regressão.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p702.md`, com hash do commit.
