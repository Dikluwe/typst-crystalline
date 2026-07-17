---
# P741 — `polygon` com vértices `Ratio` (`50%`)

> **Passo:** 741
> **Data:** 2026-07-10
> **Foco:** P734 confirmou que o vanilla aceita `Ratio` como vértice de `polygon` (resolvido no layout contra o contentor), mas o cristalino rejeita — o construtor corre em tempo de eval, sem dimensão de referência disponível. Requer que a resolução aconteça em tempo de layout, não de eval — mudança de mecanismo, não só de tipo aceite.
> **Tipo:** Sonda + Implementação. Potencialmente maior do que parece à primeira vista.
> **Tamanho:** M-L — depende de como o layout de `polygon` já lida com dimensões relativas noutros contextos.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P734 (onde o scope-out foi registado com medição, 2923 px).

---

## Sonda

### Confirmar exactamente contra que dimensão o vanilla resolve a percentagem

```bash
cat > /tmp/p741-ratio.typ <<'EOF'
#polygon((50%, 0pt), (0pt, 40pt), (25pt, 0pt))
#box(width: 200pt, height: 100pt, polygon((50%, 0pt), (0pt, 40pt), (25pt, 0pt)))
EOF
lab/typst-original/target/release/typst compile /tmp/p741-ratio.typ /tmp/p741-vanilla.pdf
mutool draw -o /tmp/p741-vanilla.png -r 150 /tmp/p741-vanilla.pdf
```

Confirmar se a percentagem resolve contra o tamanho do polígono em si (bounding box dos outros pontos), contra o contentor (`box`), ou contra a página — testar os dois casos (dentro e fora de `box`) para ver se o resultado muda.

### Confirmar como outras formas já resolvem `Ratio`/`Rel<Length>` em tempo de layout no cristalino

```bash
grep -n "Rel<Length>\|resolve.*layout\|Content::Shape" 01_core/src/rules/layout/shapes.rs 01_core/src/rules/layout/mod.rs 2>/dev/null | head -30
```

Confirmar se já existe mecanismo de resolução de `Rel<Length>` em tempo de layout que `polygon` possa reaproveitar (por exemplo, o que `rect`/`box` já usam para `width: 50%`), ou se é preciso construir de raiz.

### Critério de fecho da sonda

- [ ] Dimensão de referência confirmada (bounding box próprio, contentor, ou página).
- [ ] Mecanismo de resolução em tempo de layout já existente confirmado, reaproveitável ou não.
- [ ] Decisão registada: implementar agora, ou scope-out reforçado com custo medido (não estimado) se a mudança for desproporcionalmente grande face ao benefício (sem consumidor em `cetz`).

---

## Implementação, condicional à sonda

Se o custo for razoável: mover a resolução de vértices `Ratio` para o tempo de layout, reaproveitando o mecanismo já existente para outras formas com dimensões relativas.

### Critério de fecho da implementação

- [ ] `polygon` com vértices `Ratio` funciona, testado com diff de pixels contra o vanilla, dentro e fora de contentor explícito.
- [ ] Vértices `Length` (já correctos) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p741-ratio.typ /tmp/p741-depois.pdf
mutool draw -o /tmp/p741-depois.png -r 150 /tmp/p741-depois.pdf
```

Diff de pixels contra o vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa, dimensão de referência e mecanismo confirmados.
- [ ] Implementado com diff de pixels, ou scope-out reforçado com custo medido.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p741.md`, com hash do commit.
- [ ] Item actualizado em `achados-adiados-cetz.md`.
