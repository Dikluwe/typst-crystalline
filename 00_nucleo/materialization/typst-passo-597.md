---
# P597 — Cristalino força duas colunas mesmo com conteúdo curto; vanilla não

> **Passo:** 597
> **Data:** 2026-07-05
> **Foco:** O Documento 1 de P596 encontrou, de passagem, que `#set page(columns: 2, height: 200pt)` com pouco texto (`#lorem(30)`) produz uma coluna só no vanilla (o texto não precisa de duas colunas para caber) mas duas colunas no cristalino, mesmo sem precisar. Isto pode ser a causa real por trás do comportamento diferente observado no documento de teste original de P595, não a gestão de overflow de notas em si. Este passo confirma o alcance desta diferença.
> **Tipo:** Sonda directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P596 (onde a diferença apareceu, sem ser o foco desse passo), P552/P553 (correcções anteriores na área de colunas).

---

## Contexto

P596, Documento 1:

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)
```

- Vanilla 0.15.0: 1 página, **uma coluna** (o texto curto não precisa de duas).
- Cristalino: 1 página, **duas colunas**, mesmo com o mesmo texto curto.

Se o cristalino divide sempre visualmente em duas colunas, independentemente de o conteúdo precisar, isto é uma diferença de comportamento com o vanilla, separada de qualquer questão de overflow de notas de rodapé.

---

## Sonda

### Confirmar com mais casos de tamanho de conteúdo

```bash
for n in 5 15 30 50 80; do
  cat > /tmp/p597-n$n.typ <<EOF
#set page(columns: 2, height: 200pt)
#lorem($n)
EOF
  echo "=== n=$n ==="
  ./target/release/typst /tmp/p597-n$n.typ /tmp/p597-cristalino-$n.pdf
  mutool draw -o /tmp/p597-cristalino-$n.png -r 100 /tmp/p597-cristalino-$n.pdf
  lab/typst-original/target/release/typst compile /tmp/p597-n$n.typ /tmp/p597-vanilla-$n.pdf
  mutool draw -o /tmp/p597-vanilla-$n.png -r 100 /tmp/p597-vanilla-$n.pdf
done
```

Comparar as imagens: a partir de que tamanho de texto o vanilla começa a mostrar duas colunas visíveis? O cristalino mostra sempre duas, independentemente do tamanho?

### Localizar a causa no código

```bash
grep -n "fn layout\|two.column\|force.*column" 01_core/src/rules/layout/columns.rs | head -20
```

Confirmar se o layout de colunas, no cristalino, desenha sempre a segunda coluna (mesmo vazia) como parte da estrutura da página, em vez de só desenhar as colunas que o conteúdo de facto precisa.

### Critério de fecho

- [ ] Confirmado com vários tamanhos de conteúdo se o cristalino força sempre duas colunas visíveis.
- [ ] Confirmado a partir de que ponto o vanilla começa a mostrar a segunda coluna.
- [ ] Causa localizada com `file:line`.

---

## Decisão

Se confirmado que o cristalino desenha sempre o número de colunas pedido, mesmo vazias, isto pode ser intencional (o utilizador pediu duas colunas, recebe duas colunas, mesmo que uma fique parcialmente vazia) ou pode ser uma diferença de paridade a corrigir. Não assumir nenhuma das duas — decidir com base no que o vanilla faz e no que faz mais sentido para quem usa o cristalino.

---

## Critério de fecho do passo

- [ ] Alcance da diferença confirmado com vários tamanhos.
- [ ] Causa localizada.
- [ ] Decisão registada — corrigir para bater com o vanilla, ou aceitar com razão nova.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p597.md`, com hash do commit.
