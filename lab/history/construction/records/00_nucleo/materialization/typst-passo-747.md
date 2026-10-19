---
# P747 — Investigar o deslocamento vertical de ~11pt entre cristalino e vanilla

> **Passo:** 747
> **Data:** 2026-07-10
> **Foco:** P746 encontrou um deslocamento vertical absoluto de ~11pt entre as coordenadas da linha no cristalino e no vanilla, e classificou-o como "escolha de layout" sem investigar a causa. Isto é inconsistente com P745, que mediu diferenças concentradas em fronteiras finas (0,14%) para um documento diferente — se 11pt fosse um deslocamento real e uniforme, as formas estariam claramente desalinhadas, não só com bordas ligeiramente diferentes. Este passo investiga a causa real, sem aceitar "escolha de layout" como explicação sem prova.
> **Tipo:** Sonda directa. Prioridade alta — pode ser um bug real de posicionamento.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P746 (onde o deslocamento foi encontrado e mal explicado), P745 (onde o padrão de diff pequeno foi confirmado, aparentemente inconsistente).

---

## Sonda

### Confirmar visualmente se o deslocamento de 11pt é real

```bash
cat > /tmp/p747-nativo.typ <<'EOF'
#line(start: (0pt, 0pt), end: (100pt, 50pt))
#circle(radius: 30pt)
EOF
./target/release/typst /tmp/p747-nativo.typ /tmp/p747-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p747-nativo.typ /tmp/p747-vanilla.pdf
mutool draw -o /tmp/p747-cristalino.png -r 150 /tmp/p747-cristalino.pdf
mutool draw -o /tmp/p747-vanilla.png -r 150 /tmp/p747-vanilla.pdf
```

Inspeccionar as duas imagens directamente — as formas estão visualmente na mesma posição na página, ou claramente deslocadas 11pt (≈23 pixels a 150dpi)? Isto resolve imediatamente se o deslocamento medido em P746 é real ou um erro de leitura das coordenadas.

### Repetir o diff de pixels deste documento específico (P746 nunca o fez)

```bash
python3 /tmp/pngdiff.py /tmp/p747-vanilla.png /tmp/p747-cristalino.png
```

P746 comparou coordenadas no content stream, mas nunca fez o diff de pixels para este documento específico (só P745 fez, para o documento `cetz`). Confirmar se este documento também tem diff pequeno (~0,1-0,2%) ou se é maior, o que confirmaria um deslocamento real.

### Se o deslocamento for real: confirmar a origem — tamanho de página, margem, ou origem do sistema de coordenadas

```bash
mutool show /tmp/p747-cristalino.pdf trailer
mutool show /tmp/p747-vanilla.pdf trailer
```

Confirmar o tamanho da página (`MediaBox`) nos dois PDFs — uma diferença de tamanho de página explicaria um deslocamento aparente sem ser um bug de posicionamento do conteúdo em si (a origem do sistema de coordenadas PDF é o canto inferior esquerdo da página; se as páginas tiverem tamanhos diferentes, as mesmas coordenadas relativas ao topo da página ficam em posições absolutas diferentes).

### Se P746 tiver um erro de leitura: confirmar qual

Reler os content streams extraídos por P746 com atenção — confirmar se a transformação `cm` do vanilla foi aplicada correctamente, e se o objecto de content stream identificado em cada PDF é mesmo o correcto (o número do objecto pode não corresponder à mesma página/conteúdo nos dois PDFs).

### Critério de fecho da sonda

- [ ] Confirmado visualmente (imagem, não só coordenadas) se o deslocamento de 11pt é real.
- [ ] Diff de pixels deste documento específico medido, não assumido a partir de outro documento.
- [ ] Se real: origem confirmada (tamanho de página, margem, ou bug genuíno de posicionamento).
- [ ] Se for um erro de leitura de P746: identificado e corrigido nos registos.

---

## Decisão

Se o deslocamento for confirmado como real e não explicável por diferença de tamanho de página: é um bug de posicionamento genuíno, com prioridade alta — investigar e corrigir.

Se for explicável por diferença de tamanho de página (ou outra causa mecânica legítima): registar a explicação real, substituindo "escolha de layout" por uma causa concreta e verificada.

Se for um erro de leitura de P746 (por exemplo, objectos de content stream não correspondentes): corrigir o registo, e confirmar que a geometria é de facto idêntica nos dois lados, sem deslocamento nenhum.

---

## Critério de fecho do passo

- [ ] Sonda completa, com confirmação visual directa, não só análise de coordenadas.
- [ ] Causa real do deslocamento (ou da sua ausência) identificada com evidência.
- [ ] Se for um bug: corrigido e testado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p747.md`, substituindo a conclusão de P746 se necessário.
