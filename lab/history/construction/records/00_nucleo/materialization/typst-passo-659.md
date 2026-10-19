---
# P659 — Chave da cache de `shaped_width` não inclui variações de eixo OpenType

> **Passo:** 659
> **Data:** 2026-07-09
> **Foco:** P658 encontrou, de passagem, que `ShapedWidthKey` (a chave da cache de `shaped_width`, usada na decisão de quebra de linha para árabe/devanágari) não inclui as variações de eixo OpenType de fontes variáveis. Dois textos com o mesmo texto, tamanho, e peso nominal, mas eixos de variação diferentes, podem colidir na cache e devolver a largura errada — silenciosamente, alimentando uma decisão de quebra de linha incorrecta. Isto é uma falha silenciosa real, não uma limitação de desempenho.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P658 (onde o problema foi encontrado), P591 (onde a cache foi originalmente criada).

---

## Sonda

### Confirmar a colisão directamente

```bash
cat > /tmp/p659-variacao.typ <<'EOF'
#set text(font: "Noto Sans Devanagari", size: 40pt)
#text(font: (name: "Noto Sans Devanagari", variant: (wdth: 62.5)))[नमस्ते संसार]
#text(font: (name: "Noto Sans Devanagari", variant: (wdth: 125)))[नमस्ते संसार]
EOF
./target/release/typst /tmp/p659-variacao.typ /tmp/p659.pdf
mutool draw -o /tmp/p659.png -r 150 /tmp/p659.pdf
```

Confirmar visualmente se as duas linhas têm larguras diferentes (esperado, porque os eixos são diferentes) e se a decisão de quebra de linha (em documentos mais longos com estes dois estilos) trata as duas larguras correctamente ou como se fossem iguais.

Se o cristalino ainda não expõe sintaxe de eixo de variação directamente (a confirmar), replicar a situação através de duas fontes instaladas com nomes diferentes mas dados de glifo idênticos exceto no eixo, ou confirmar de outra forma que a chave da cache realmente ignora essa informação.

### Confirmar a estrutura exacta da chave

```bash
grep -n "struct ShapedWidthKey" 03_infra/src/font_metrics.rs
```

Listar todos os campos, confirmando a ausência de qualquer referência a eixos de variação.

### Critério de fecho da sonda

- [ ] Colisão confirmada directamente, com um caso reproduzível.
- [ ] Estrutura da chave confirmada, campo a campo.

---

## Implementação

Adicionar os eixos de variação relevantes à chave `ShapedWidthKey`, alinhando com o que a cache de P657 (`ShapeCache`) já inclui na sua própria chave.

### Critério de fecho da implementação

- [ ] `ShapedWidthKey` inclui as variações de eixo.
- [ ] O caso de colisão da sonda deixa de ocorrer — as duas larguras diferentes são cacheadas separadamente.
- [ ] Documentos sem fontes variáveis (a maioria) sem regressão de desempenho perceptível — confirmar que adicionar campos à chave não degrada o hit ratio já medido (99,95%) para o caso comum.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de RTL (P590-592) e devanágari (P622-623), confirmando que continuam correctos.

```bash
./target/release/typst /tmp/p659-variacao.typ /tmp/p659-depois.pdf
mutool draw -o /tmp/p659-depois.png -r 150 /tmp/p659-depois.pdf
```

Confirmar visualmente a diferença correcta entre as duas larguras.

---

## Critério de fecho do passo

- [ ] Sonda completa, colisão confirmada directamente.
- [ ] Chave corrigida, eixos de variação incluídos.
- [ ] Hit ratio do caso comum (sem fontes variáveis) sem degradação perceptível.
- [ ] Testes de RTL e devanágari sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p659.md`, com hash do commit.
