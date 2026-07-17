---
# P761 — Completar o diagnóstico que P760 não fez: a correcção só explica 2,2% do resíduo

> **Passo:** 761
> **Data:** 2026-07-14
> **Foco:** P760 corrigiu dois bugs reais (métricas verticais da fonte errada, `FontDescriptor` genérico), mas a redução medida foi de apenas 2,2% (116.612 → 114.019 pixels de diferença) — quase nada, face a uma anomalia sessenta vezes maior do que o resíduo já estabelecido como aceitável noutro contexto (P745-754). Os passos de diagnóstico explicitamente pedidos em P760 (mapa visual de diferenças, confirmação directa de identidade de fonte, comparação de coordenadas Y de linha) não foram executados. Este passo fá-los.
> **Tipo:** Sonda directa. Prioridade máxima — ~97,8% da diferença original continua sem explicação real.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Corrigir um bug real não é o mesmo que explicar um resíduo — os dois só coincidem se o número o confirmar, e aqui não confirma.
> **Dependências:** P760 (onde as correcções foram feitas mas o diagnóstico pedido não foi executado).

---

## Sonda

### Produzir e inspeccionar o mapa visual de diferenças, sem falta desta vez

```bash
compare /tmp/p760-fixo-vanilla.png /tmp/p760-fixo-cristalino-p760.png -compose src /tmp/p761-diffmap.png 2>&1
```

Inspeccionar `/tmp/p761-diffmap.png` directamente (usar `view`, não descrever de memória) — as diferenças estão espalhadas uniformemente por cada glifo (consistente com hinting/AA), ou concentradas nalgum padrão (linhas inteiras, margens, espaçamento entre palavras)?

### Confirmar directamente se a fonte embutida é idêntica nos dois PDFs

```bash
mutool show /tmp/p760-fixo-vanilla.pdf | grep -i "BaseFont\|FontFile"
mutool show /tmp/p760-fixo-cristalino-p760.pdf | grep -i "BaseFont\|FontFile"
```

Extrair os dois streams de fonte e comparar directamente (não assumir "ambos dizem DejaVu Sans, logo são iguais" — os nomes podem coincidir com dados de fonte diferentes, como já aconteceu antes nesta conversa com o subsetting).

```bash
mutool extract /tmp/p760-fixo-vanilla.pdf
mutool extract /tmp/p760-fixo-cristalino-p760.pdf
# comparar os ficheiros de fonte extraídos, byte a byte ou pelo menos por tamanho/checksum
```

### Comparar coordenadas Y de cada linha do parágrafo, não só a primeira

```bash
mutool show /tmp/p760-fixo-vanilla.pdf 4 2>&1 | grep -E "^\-?[0-9.]+ \-?[0-9.]+ Td|Tm" | head -20
mutool show /tmp/p760-fixo-cristalino-p760.pdf 4 2>&1 | grep -E "^\-?[0-9.]+ \-?[0-9.]+ Td|Tm" | head -20
```

Extrair a posição Y de cada linha de texto (não só a primeira, como fizeram passos anteriores) — confirmar se todas as linhas têm exactamente a mesma posição vertical nos dois lados, ou se há um deslocamento acumulado que cresce linha a linha (o que apontaria para espaçamento entre linhas ligeiramente diferente, não hinting).

### Comparar a largura de cada linha (posição X do fim de cada linha)

```bash
mutool show /tmp/p760-fixo-vanilla.pdf 4 2>&1 | grep "Tj\|TJ" | head -20
```

Se as quebras de linha são idênticas (mesmo texto por linha) mas a largura renderizada de cada linha for ligeiramente diferente, isso apontaria para espaçamento entre letras/palavras diferente — um problema de shaping, não de layout.

### Testar com uma única linha, sem quebra nenhuma, ao maior detalhe possível

```bash
cat > /tmp/p761-linha-unica.typ <<'EOF'
#set page(width: 400pt, margin: 20pt, height: 50pt)
#set text(font: "DejaVu Sans", size: 24pt)
Testing
EOF
lab/typst-original/target/release/typst compile /tmp/p761-linha-unica.typ /tmp/p761-linha-vanilla.pdf
./target/release/typst /tmp/p761-linha-unica.typ /tmp/p761-linha-cristalino.pdf
mutool draw -o /tmp/p761-linha-vanilla.png -r 600 /tmp/p761-linha-vanilla.pdf
mutool draw -o /tmp/p761-linha-cristalino.png -r 600 /tmp/p761-linha-cristalino.pdf
compare -metric AE /tmp/p761-linha-vanilla.png /tmp/p761-linha-cristalino.png /tmp/p761-linha-diff.png
```

Uma única palavra curta, resolução muito alta (600dpi) — se ainda houver uma percentagem de diferença de pixels grande aqui, isola completamente o problema de qualquer coisa relacionada com layout de parágrafo/quebra de linha, apontando definitivamente para o glifo/shaping em si.

### Critério de fecho da sonda

- [ ] Mapa de diferenças produzido e inspeccionado visualmente, não descrito de memória.
- [ ] Fonte embutida confirmada byte a byte (ou por checksum) como idêntica, ou identificada a diferença exacta.
- [ ] Coordenadas Y de todas as linhas comparadas, não só a primeira — confirmado se há deslocamento crescente.
- [ ] Largura de linha comparada, para descartar diferença de espaçamento entre letras/palavras.
- [ ] Caso de palavra única, alta resolução, testado e inspeccionado.

---

## Decisão

Com base na evidência directa (não em suposição), identificar a causa real do resíduo de ~9% e decidir: é corrigível (e nesse caso corrigir), ou é genuinamente uma diferença de renderização de baixo nível (por exemplo, diferença na implementação de rasterização de curvas Bézier entre `ttf-parser`/o motor de render do cristalino e o que o vanilla usa) que nenhum dos dois lados pode "corrigir" sem trocar de motor de rasterização — nesse caso, aceitar com esta evidência concreta, não com suposição.

---

## Critério de fecho do passo

- [ ] Todos os passos de diagnóstico da sonda executados e documentados com evidência directa.
- [ ] Causa real do resíduo confirmada.
- [ ] Se corrigível: corrigido e testado, com nova medição a confirmar melhoria substancial (não 2,2%).
- [ ] Se não corrigível: aceite com evidência concreta a explicar exactamente porquê, não com a mesma frase genérica repetida.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p761.md`.
