---
# P775 — Verificação por mapas de diferença e coordenadas: AE~87k nas orientações EXIF 2-8

> **Passo:** 775
> **Data:** 2026-07-16
> **Foco:** P774 reportou AE≈87000 para as orientações EXIF 2-8, uma redução de só ~19% face aos 107362 medidos antes da correcção (orientação 6 completamente errada), e atribuiu isso a "artefactos de recodificação JPEG" sem mapa de diferenças nem coordenadas — só inspecção visual não documentada. O valor é da mesma ordem de grandeza do caso "totalmente errado" que motivou o passo, muito acima de qualquer ruído de recompressão já observado nesta linha de trabalho (baseline limpo: 200-300; bugs geométricos reais: milhares; nunca dezenas de milhares por recompressão). As sete orientações não-triviais deram valores quase idênticos entre si, o que é inconsistente com "ruído de recompressão" (que variaria por tipo de transformação) e consistente com um erro sistemático que afecta todas igualmente. Este passo não aceita nem rejeita a explicação — mede directamente.
> **Tipo:** Sonda de verificação. Sem implementação — decide se há bug remanescente.
> **Tamanho:** S/M.
> **ADR-0108 EM VIGOR** — não aceitar "artefacto de recompressão" sem mapa de diferenças e comparação de conteúdo real, especialmente quando o número é da mesma ordem do erro original.
> **Dependências:** P774 (medições reportadas, commit `680cbbc2466412b6086cddaddc107b678e84e671`).

---

## Passo 0 — Sanity-check: quanto AE produz recompressão JPEG sozinha?

Antes de julgar se 87k é "muito", medir directamente quanto AE uma recompressão JPEG qualidade 95 introduz, isolada de qualquer rotação:

```bash
convert /tmp/p774-base.png -quality 95 /tmp/p775-recomp-a.jpg
convert /tmp/p775-recomp-a.jpg -quality 95 /tmp/p775-recomp-b.jpg
compare -metric AE /tmp/p775-recomp-a.jpg /tmp/p775-recomp-b.jpg /tmp/p775-recomp-diff.png
```

Este número é o tecto plausível para "é só recompressão" — se ficar em centenas ou baixo milhar, e P774 reportou ~87000, a explicação de P774 está desmentida directamente por este sanity-check, sem precisar investigar mais nada.

---

## Passo 1 — Mapa de diferenças para cada orientação

Para cada uma das 8 orientações, gerar o par vanilla/cristalino (reutilizando os documentos de P774) e produzir o mapa de diferenças com destaque:

```bash
for orient in 1 2 3 4 5 6 7 8; do
  echo "=== Orientação $orient ==="
  compare -metric AE /tmp/p774-vanilla-orient${orient}.png /tmp/p774-cristalino-orient${orient}.png \
    -highlight-color red -lowlight-color none /tmp/p775-diffmap-orient${orient}.png
done
```

Inspeccionar cada `/tmp/p775-diffmap-orient*.png`: a diferença está espalhada uniformemente pela imagem toda (consistente com ruído de recompressão), concentrada nas bordas do retângulo de referência (consistente com pequeno desalinhamento sub-pixel), ou é uma área grande e estruturada (consistente com conteúdo errado — orientação parcialmente aplicada, cores trocadas, ou a imagem sendo desenhada na posição/tamanho errado apesar da rotação em si estar correcta)?

---

## Passo 2 — Coordenadas do retângulo de referência

Não confiar em inspecção visual não documentada — extrair a posição real do retângulo vermelho de referência em cada PDF, para cada orientação:

```bash
for orient in 1 2 3 4 5 6 7 8; do
  echo "=== Orientação $orient ==="
  mutool trace /tmp/p774-vanilla-orient${orient}.pdf | grep -A5 "cm /"
  mutool trace /tmp/p774-cristalino-orient${orient}.pdf | grep -A5 "cm /"
done
```

Se possível, usar um script simples de detecção de cor (procurar os pixels vermelhos na imagem rasterizada e reportar bounding box) para confirmar objectivamente a posição do retângulo em vez de inspecção ocular:

```bash
python3 -c "
from PIL import Image
import sys
img = Image.open(sys.argv[1]).convert('RGB')
px = img.load()
xs, ys = [], []
for y in range(img.height):
    for x in range(img.width):
        r, g, b = px[x, y]
        if r > 180 and g < 100 and b < 100:
            xs.append(x); ys.append(y)
if xs:
    print(f'bbox vermelho: x=[{min(xs)},{max(xs)}] y=[{min(ys)},{max(ys)}]')
else:
    print('nenhum pixel vermelho encontrado')
" /tmp/p774-vanilla-orient6.png
```

Repetir para o cristalino e para as 8 orientações. Comparar as bounding boxes — se o retângulo estiver no canto certo mas ligeiramente deslocado, ou se as cores/dimensões da imagem em si divergirem (não só a posição do retângulo).

---

## Conclusão a registar

- **Se o sanity-check do Passo 0 confirmar que recompressão sozinha produz AE na casa de milhares ou menos**, e os mapas do Passo 1 mostrarem diferença estruturada (não uniforme), e as coordenadas do Passo 2 confirmarem qualquer desvio: a explicação de P774 está errada, há um bug remanescente — abrir P776 para investigar a causa (aplicação parcial da transformação? ordem errada de flip+rotate para as orientações compostas 5 e 7? recodificação a alterar mais do que orientação?).
- **Se o sanity-check mostrar que AE~87k é de facto plausível para recompressão** (improvável dado os baselines desta conversa, mas não impossível se a imagem de teste for particularmente sensível a artefactos), e os mapas mostrarem diferença uniforme/difusa, e as coordenadas baterem: aceitar a explicação de P774, com esta evidência a suportá-la — não a frase solta que estava no relatório original.

Não fechar com "provavelmente" — ou fica confirmado com números e mapas, ou fica marcado como bug em aberto.

---

## Critério de fecho do passo

- [ ] Sanity-check de recompressão isolada medido (Passo 0).
- [ ] Mapas de diferença gerados e inspeccionados para as 8 orientações.
- [ ] Bounding box do retângulo de referência confirmada objectivamente (script de detecção de cor, não só inspecção ocular) para pelo menos as orientações 2, 5, 6, 7 (as mais propensas a erro de composição flip+rotate).
- [ ] Conclusão registada com números e evidência, não qualificadores vagos.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p775.md`, com os mapas de diferença referenciados/anexados.

---

## Próximo passo

Se bug confirmado: P776, correcção focada na causa identificada, com a mesma disciplina de coordenadas.
Se explicação de P774 confirmada com evidência real: fechar a linha de trabalho de imagem (P769-P775) definitivamente, com o sanity-check registado como prova, não como suposição.
