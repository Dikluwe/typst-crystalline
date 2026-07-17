---
# P763e — Reprodução com metodologia exacta de P763c + sanity-check da medição

> **Passo:** 763e
> **Data:** 2026-07-15
> **Foco:** P763d reportou AE=0 em todos os 7 casos testados, sem alterar código, revertendo um bug que P763c tinha medido com precisão de coordenada (deslocamento de ~61pt, inversão de eixo Y). A maioria dos resultados de P763d não mostra o comando `compare` usado, nem confirma se passou por `mutool draw` (rasterização a 300ppp, o método de P763c) ou por outro caminho. A explicação proposta ("P762 corrigiu isto") é incompatível com a cronologia — P762 já estava commitado antes de P763c medir o bug. Este passo não aceita nem rejeita a conclusão de P763d; reproduz com o método exacto de P763c e testa a própria ferramenta de medição antes de confiar em qualquer resultado.
> **Tipo:** Sonda de verificação metodológica. Sem implementação — só decide se há ou não um bug remanescente a corrigir.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — não aceitar resultado sem confirmar directamente o método que o produziu.
> **Dependências:** P763c (medição original do bug, commit `dee903868c9c7d6b63e8fbc47257e5082f7638f0`), P763d (medição de AE=0, commit `547ad10a711bdebf49e1355e7b4cebdd110cd891`).

---

## Passo 0 — Sanity-check da própria ferramenta de medição

Antes de confiar em qualquer AE medido neste passo ou nos anteriores, confirmar que `compare -metric AE` está a funcionar como esperado:

```bash
# Duas imagens claramente diferentes — AE deve ser grande, não zero
convert -size 200x200 xc:white /tmp/p763e-branco.png
convert -size 200x200 xc:black /tmp/p763e-preto.png
compare -metric AE /tmp/p763e-branco.png /tmp/p763e-preto.png /tmp/p763e-sanity-diff.png
```

```bash
# A mesma imagem comparada consigo própria — AE deve ser exactamente 0
compare -metric AE /tmp/p763e-branco.png /tmp/p763e-branco.png /tmp/p763e-sanity-same.png
```

```bash
# Comparar directamente dois PDFs (o que P763d parece ter feito nalguns casos, sem passar por mutool)
# vs comparar as versões rasterizadas em PNG (o que P763c fez)
compare -metric AE /tmp/p763d-completo-vanilla.pdf /tmp/p763d-completo-cristalino.pdf /tmp/p763e-direct-pdf-diff.png 2>&1
echo "--- vs PNG rasterizado ---"
mutool draw -o /tmp/p763e-vanilla-raster.png -r 300 /tmp/p763d-completo-vanilla.pdf
mutool draw -o /tmp/p763e-cristalino-raster.png -r 300 /tmp/p763d-completo-cristalino.pdf
compare -metric AE /tmp/p763e-vanilla-raster.png /tmp/p763e-cristalino-raster.png /tmp/p763e-png-diff.png
```

Se a comparação directa de PDF e a comparação via PNG rasterizado derem resultados diferentes, isso já explica a discrepância — registar qual delas é o método correcto (P763c estabeleceu PNG rasterizado como o método de referência; qualquer desvio disso precisa de justificação explícita, não silenciosa).

---

## Passo 1 — Git diff entre os dois commits

```bash
git log --oneline dee903868c9c7d6b63e8fbc47257e5082f7638f0..547ad10a711bdebf49e1355e7b4cebdd110cd891
git diff --stat dee903868c9c7d6b63e8fbc47257e5082f7638f0..547ad10a711bdebf49e1355e7b4cebdd110cd891
```

Confirmar exactamente o que mudou entre os dois commits — mesmo que P763d declare "nenhuma alteração de código", os commits intermédios (P765b, P766, e qualquer outro) podem ter tocado em código de layout/renderização sem que o relatório tenha percebido a relação. Se algum ficheiro em `01_core/src/rules/layout/` ou relacionado a `place`/transformações de grupo aparecer no diff, essa é a explicação candidata a confirmar — não "P762", que é anterior a P763c.

---

## Passo 2 — Reprodução com o método exacto de P763c

Usar exactamente os mesmos documentos e o mesmo pipeline de P763c (import → compile → `mutool draw -r 300` → `compare -metric AE`, com highlight map), sem as variações que P763d introduziu (fonte explícita `DejaVu Sans`, `#set page` adicional em casos que P763c não tinha):

```bash
# Documento IDÊNTICO ao de P763c, byte a byte — sem #set text nem #set page adicionais
cat > /tmp/p763e-cetz-original.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF

lab/typst-original/target/release/typst compile /tmp/p763e-cetz-original.typ /tmp/p763e-vanilla.pdf
./target/release/typst compile /tmp/p763e-cetz-original.typ /tmp/p763e-cristalino.pdf

mutool draw -o /tmp/p763e-vanilla.png -r 300 /tmp/p763e-vanilla.pdf
mutool draw -o /tmp/p763e-cristalino.png -r 300 /tmp/p763e-cristalino.pdf

compare -metric AE /tmp/p763e-vanilla.png /tmp/p763e-cristalino.png -highlight-color red -lowlight-color none /tmp/p763e-diffmap.png
```

Se o resultado for AE=0: extrair as coordenadas via `mutool trace`, como em P763c, e confirmar directamente que a translação do canvas e o eixo Y batem agora com o vanilla — não só que a métrica de pixel deu zero (uma métrica pode dar zero por coincidência de arredondamento em certos casos; a comparação de coordenadas exactas é a prova mais forte).

```bash
mutool trace /tmp/p763e-vanilla.pdf > /tmp/p763e-trace-vanilla.txt
mutool trace /tmp/p763e-cristalino.pdf > /tmp/p763e-trace-cristalino.txt
diff /tmp/p763e-trace-vanilla.txt /tmp/p763e-trace-cristalino.txt
```

Se o resultado for AE > 0 (reproduzindo o bug de P763c): P763d tem um problema de metodologia real — investigar se foi a fonte explícita, o `#set page` adicional, ou algo no ambiente de execução que mascarou o resultado.

---

## Critério de fecho do passo

- [ ] Sanity-check da ferramenta `compare` confirmado (branco vs preto dá AE grande; imagem consigo própria dá AE=0; comparação directa de PDF vs PNG rasterizado dão o mesmo resultado, ou a diferença está explicada).
- [ ] Git diff entre os commits de P763c e P763d inspeccionado; qualquer mudança em código de layout/renderização identificada e avaliada como candidata a explicação.
- [ ] Documento idêntico byte a byte ao de P763c reproduzido com o pipeline exacto (`mutool draw` → `compare`).
- [ ] Se AE=0 confirmado: coordenadas exactas comparadas via `mutool trace`, não só a métrica de pixel.
- [ ] Se AE>0: metodologia de P763d identificada como a causa da discrepância, registada explicitamente.
- [ ] Conclusão final registada com evidência directa — "corrigido, confirmado por coordenadas" ou "ainda bugado, P763d mediu errado" — nunca "provavelmente foi outra coisa".
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763e.md`.

---

## Próximo passo

Se confirmado AE=0 com coordenadas batendo: fechar definitivamente a linha `cetz`/download de pacotes (P763–P763e), com a causa real da correcção identificada (não "provavelmente P762").
Se reproduzido o bug: reabrir a correcção como P763f, agora com a certeza de que P763d mediu de forma inconsistente — e revisar se outros relatórios recentes (P765b, P766) usaram a mesma metodologia potencialmente falha antes de aceitar os seus resultados sem verificação adicional.
