---
# P760 — Investigar os 9,11% de diferença de pixels com quebras idênticas (P759)

> **Passo:** 760
> **Data:** 2026-07-14
> **Foco:** P759 mediu 9,11% AE / 21,32% RMSE de diferença de pixels entre vanilla e cristalino, mesmo com quebras de linha confirmadas idênticas (via texto extraído), e atribuiu isto a "renderização mecânica" sem mais investigação. Isto é sessenta vezes maior do que o resíduo (~0,14%) já confirmado como anti-aliasing genuíno noutro contexto desta conversa (P745-754), depois de investigação a fundo. A mesma explicação, para um número desta magnitude, não pode ser aceite sem verificação directa — especialmente porque a última vez que "mecânico" foi aceite sem confirmar (o resíduo de `cetz`), escondia um bug real de posicionamento (P748).
> **Tipo:** Sonda directa. Prioridade alta — pode revelar um bug real de layout/render em texto latino comum, com alcance universal.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Uma explicação "mecânica" para uma diferença sessenta vezes maior do que o resíduo já confirmado como aceitável não pode ser aceite sem medição directa.
> **Dependências:** P759 (onde o número foi medido e aceite sem investigação), P745-748 (precedente directo — mesma explicação, desta vez escondia um bug real).

---

## Sonda

### Reproduzir a medição exacta de P759

```bash
cat > /tmp/p760-latim.typ <<'EOF'
#set page(width: 350pt, margin: 40pt)
#set text(font: "DejaVu Sans", size: 11pt)
#lorem(200)
EOF
lab/typst-original/target/release/typst compile /tmp/p760-latim.typ /tmp/p760-vanilla.pdf
./target/release/typst /tmp/p760-latim.typ /tmp/p760-cristalino.pdf
mutool draw -o /tmp/p760-vanilla.png -r 150 /tmp/p760-vanilla.pdf
mutool draw -o /tmp/p760-cristalino.png -r 150 /tmp/p760-cristalino.pdf
compare -metric AE /tmp/p760-vanilla.png /tmp/p760-cristalino.png /tmp/p760-diff.png
```

### Produzir e inspeccionar o mapa de diferenças

```bash
compare /tmp/p760-vanilla.png /tmp/p760-cristalino.png -compose src /tmp/p760-diffmap.png
```

Inspeccionar `/tmp/p760-diffmap.png` directamente — as diferenças estão espalhadas uniformemente por todo o texto (consistente com hinting/AA genuíno), concentradas nalguma zona específica (sugerindo um problema localizado), ou sistemáticas (por exemplo, todas as linhas ligeiramente deslocadas verticalmente ou horizontalmente, o que seria um bug de posicionamento, não anti-aliasing)?

### Confirmar se a fonte `DejaVu Sans` é de facto a mesma nos dois lados

```bash
mutool show /tmp/p760-vanilla.pdf | grep -i "BaseFont"
mutool show /tmp/p760-cristalino.pdf | grep -i "BaseFont"
```

Dado o histórico recente (P753/P754, onde a fonte por defeito e a resolução de fontes já causaram problemas reais), confirmar directamente que `DejaVu Sans` é a mesma fonte, embutida da mesma forma, nos dois PDFs — não assumir.

### Confirmar se há um deslocamento sistemático (posição), não só diferença de glifo

```bash
mutool show /tmp/p760-vanilla.pdf 4 2>&1 | head -20
mutool show /tmp/p760-cristalino.pdf 4 2>&1 | head -20
```

Comparar as coordenadas Y da primeira e da última linha do parágrafo — confirmar se há um deslocamento vertical (o mesmo tipo de bug já corrigido em P748/P750/P751/P752 para o caso de formas/baseline inicial, mas talvez ainda presente para parágrafos multi-linha) ou se a posição vertical bate exactamente.

### Testar com um documento menor, mais fácil de inspeccionar visualmente ao detalhe

```bash
cat > /tmp/p760-pequeno.typ <<'EOF'
#set page(width: 200pt, margin: 20pt)
#set text(font: "DejaVu Sans", size: 24pt)
Testing text
EOF
lab/typst-original/target/release/typst compile /tmp/p760-pequeno.typ /tmp/p760-pequeno-vanilla.pdf
./target/release/typst /tmp/p760-pequeno.typ /tmp/p760-pequeno-cristalino.pdf
mutool draw -o /tmp/p760-pequeno-vanilla.png -r 300 /tmp/p760-pequeno-vanilla.pdf
mutool draw -o /tmp/p760-pequeno-cristalino.png -r 300 /tmp/p760-pequeno-cristalino.pdf
compare -metric AE /tmp/p760-pequeno-vanilla.png /tmp/p760-pequeno-cristalino.png /tmp/p760-pequeno-diff.png
```

Com um documento pequeno e resolução alta, é mais fácil confirmar visualmente se os glifos estão na posição exacta certa, ou se há um deslocamento/diferença de espaçamento sistemática.

### Critério de fecho da sonda

- [ ] Medição reproduzida.
- [ ] Mapa de diferenças inspeccionado — padrão espalhado (AA genuíno) vs concentrado/sistemático (bug real).
- [ ] Fonte confirmada como idêntica nos dois lados, não assumida.
- [ ] Posição vertical de linhas comparada directamente, procurando deslocamento sistemático.
- [ ] Documento pequeno, alta resolução, inspeccionado visualmente ao detalhe.

---

## Decisão

Se o padrão for genuinamente espalhado e uniforme, com posições confirmadas idênticas: aceitar como diferença de renderização/hinting, mas com esta confirmação directa a substituir a suposição anterior.

Se houver um padrão sistemático (deslocamento, espaçamento diferente, fonte não realmente idêntica): investigar e corrigir a causa real, com a mesma prioridade já dada a achados semelhantes nesta conversa (P748).

---

## Critério de fecho do passo

- [ ] Sonda completa, com mapa de diferenças e comparação directa de posições/fontes.
- [ ] Causa confirmada com evidência directa, não suposição.
- [ ] Se bug real: corrigido e testado.
- [ ] Se genuinamente mecânico: confirmado com números e inspecção, não só assumido.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p760.md`.
