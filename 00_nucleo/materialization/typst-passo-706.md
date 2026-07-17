---
# P706 — Operador `in` para `Str`/`Dict`

> **Passo:** 706
> **Data:** 2026-07-10
> **Foco:** P705 isolou que `"chave" in dict` falha no cristalino ("cannot apply In to str and dictionary"), enquanto o vanilla devolve `true`/`false`. O operador `in` não foi ainda localizado numa linha exacta de `cetz`, e a semântica completa (`Dict` testa chaves, `Array` testa elementos, `Str` testa substring) precisa de ser confirmada por completo, não assumida como só "chave em dict".
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P705 (onde o bloqueio foi encontrado, sem isolar a linha exacta de `cetz`).

---

## Sonda

### Localizar a linha exacta de `cetz` que usa `in`

```bash
grep -rn " in " ~/.cache/typst/packages/preview/cetz/0.5.2/src/*.typ ~/.cache/typst/packages/preview/cetz/0.5.2/src/**/*.typ 2>/dev/null | grep -v "for .* in " | head -20
```

Filtrar `for x in ...` (laço, já suportado) do operador `in` de teste de pertença — confirmar a linha exacta que motivou o bloqueio.

### Confirmar todas as combinações que o vanilla suporta

```bash
cat > /tmp/p706-in.typ <<'EOF'
#("a" in (a: 1, b: 2))
#("z" in (a: 1, b: 2))
#(1 in (1, 2, 3))
#(5 in (1, 2, 3))
#("ell" in "hello")
#("xyz" in "hello")
#(1 in "hello")
EOF
lab/typst-original/target/release/typst compile /tmp/p706-in.typ /tmp/p706-vanilla.pdf
pdftotext /tmp/p706-vanilla.pdf -
```

Confirmar cada combinação (chave em dict, elemento em array, substring em string), e o que acontece com tipos incompatíveis (`Int` em `Str`) — erro, ou `false`?

### Confirmar quais já funcionam no cristalino hoje

```bash
./target/release/typst /tmp/p706-in.typ /tmp/p706-cristalino.pdf
echo "Exit code: $?"
```

Isolar exactamente quais combinações já funcionam e quais faltam — P705 só confirmou `Str in Dict` como falha; `Int in Array` pode já funcionar.

### Critério de fecho da sonda

- [ ] Linha exacta de `cetz` que usa `in` localizada.
- [ ] Todas as combinações (`Dict`, `Array`, `Str`) confirmadas contra o vanilla.
- [ ] Estado actual do cristalino confirmado para cada combinação, não assumido.

---

## Implementação

Implementar as combinações confirmadas em falta pela sonda, seguindo exactamente a semântica do vanilla (incluindo o comportamento com tipos incompatíveis).

### Critério de fecho da implementação

- [ ] Todas as combinações em falta implementadas e testadas.
- [ ] Combinações já existentes sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p706-in.typ /tmp/p706-depois.pdf
pdftotext /tmp/p706-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

### Repetir a reprodução de P700-705

```bash
cat > /tmp/p706-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
time ./target/release/typst /tmp/p706-cetz.typ /tmp/p706-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p706-cetz.png -r 150 /tmp/p706-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita por P688. Se não: registar o próximo bloqueio, com o mesmo cuidado de sempre.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, linha de `cetz` localizada, todas as combinações confirmadas.
- [ ] Implementado e testado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado — sucesso completo com comparação visual, ou próximo bloqueio registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p706.md`, com hash do commit.
