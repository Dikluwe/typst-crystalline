---
# P689 — Implementar `str.codepoints()`, `str.position()`, `str.match()`

> **Passo:** 689
> **Data:** 2026-07-10
> **Foco:** P688 confirmou, com reprodução mínima, que `str.codepoints()`, `str.position()`, e `str.match()` estão em falta no cristalino, bloqueando o `import` da dependência `oxifmt` de `cetz` logo na primeira linha. É o primeiro bloqueio real confirmado desde que o documento de teste foi corrigido para o padrão certo (P688).
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P688 (onde os três métodos foram confirmados em falta, com reprodução mínima).

---

## Sonda mínima

### Confirmar a assinatura e comportamento exacto de cada método no vanilla

```bash
cat > /tmp/p689-metodos.typ <<'EOF'
#("abc").codepoints()
#("café").codepoints()
#("abc").position("b")
#("abc").position("z")
#("abc").match(regex("b"))
#("abc").match(regex("z"))
EOF
lab/typst-original/target/release/typst compile /tmp/p689-metodos.typ /tmp/p689-vanilla.pdf
pdftotext /tmp/p689-vanilla.pdf -
```

Confirmar:
- `codepoints()`: devolve um array de quê exactamente — strings de um carácter cada, ou outra representação? Confirmar com um caso de acentos/unicode multi-byte.
- `position(...)`: aceita string, ou também regex? Devolve índice, ou `none` se não encontrar?
- `match(...)`: devolve quê exactamente — um dicionário com `start`/`end`/`text`/`captures`, ou outra estrutura?

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p689-metodos.typ /tmp/p689-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Assinatura e comportamento de `codepoints()` confirmados, incluindo caso multi-byte.
- [ ] Assinatura e comportamento de `position()` confirmados.
- [ ] Assinatura e comportamento de `match()` confirmados, incluindo a estrutura devolvida.

---

## Implementação

Adicionar os três métodos ao dispatcher de métodos de `str` (`01_core/src/engine/stdlib/collections.rs`), seguindo o comportamento confirmado pela sonda.

### Critério de fecho da implementação

- [ ] `str.codepoints()` implementado e testado, incluindo caso multi-byte/unicode.
- [ ] `str.position()` implementado e testado.
- [ ] `str.match()` implementado e testado, com a estrutura de retorno correcta.
- [ ] Métodos de `str` já existentes (P688 confirmou 16 já implementados) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p689-metodos.typ /tmp/p689-depois.pdf
pdftotext /tmp/p689-depois.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

### Confirmar que o import de `oxifmt` (via `cetz`) avança

```bash
cat > /tmp/p689-oxifmt.typ <<'EOF'
#import "@preview/oxifmt:1.0.0"
EOF
./target/release/typst /tmp/p689-oxifmt.typ /tmp/p689-oxifmt.pdf
echo "Exit code: $?"
```

### Confirmar `cetz` de novo, com o documento e padrão correctos de P688

```bash
cat > /tmp/p689-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p689-cetz.typ /tmp/p689-cetz.pdf
echo "Exit code: $?"
mutool draw -o /tmp/p689-cetz.png -r 150 /tmp/p689-cetz.pdf 2>/dev/null
```

Se produzir PDF: comparar visualmente com a imagem do vanilla já descrita em P688 (linha diagonal + círculo). Se falhar: registar o próximo bloqueio com a mesma disciplina — P688 já avisou que o plugin WASM do `cetz` 0.5.2 é candidato provável, mas não confirmado.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, os três métodos confirmados contra o vanilla.
- [ ] Os três métodos implementados e testados.
- [ ] Métodos existentes de `str` sem regressão.
- [ ] Import de `oxifmt` confirmado a funcionar.
- [ ] `cetz` re-testado com o documento correcto de P688 — PDF completo comparado visualmente, ou próximo bloqueio registado com honestidade.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p689.md`, com hash do commit.
