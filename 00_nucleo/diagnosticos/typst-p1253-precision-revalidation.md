# P1253 — revalidação de precisão Luma/Oklab e fronteira SVG

**Veredito final:** L1 preservado nos fragmentos medidos; zero promoção
SVG; nenhuma mutation score alegada.

## Medição antes da decisão

P1234 foi reexecutado duas vezes contra o vanilla ratificado `a51e02804`, o
binário cristalino atual e os SVG congelados: 15 fixtures, 30 probes, zero
divergência pública e 30 fronteiras internas opacas mantidas `Unknown`.

P1236 foi reexecutado duas vezes: 6 fixtures, 42 pares, `luma_max=0`, 28
`Preserved`, 14 `Known-Upstream-Bug` apenas pelo alpha perdido no vanilla e
zero `Unknown`. Isso incorpora P1239 e o P1252 revalidado sem misturar os dois
observáveis.

P1237 foi reexecutado duas vezes sobre os SVGs congelados corretos. O primeiro
ensaio com o diretório P1231 genérico falhou por ausência do servidor esperado
num caso fallback e foi descartado, sem adjudicação. Com a raiz candidata
pinada pelo próprio P1237: grafo 24/24, numérico 6/24 e zero promoções.

## Decisão

- não há correção produtiva adicional legitimada pelo P1253 atual;
- Luma e Oklab L1 permanecem fechados somente nos fragmentos medidos;
- os quatro pares Oklab/LinearRgb SVG permanecem nas classificações individuais
  `Unknown-native-approximation` ou `Unknown-fallback`;
- o threshold produtivo, cap e whitelist não mudam;
- os hashes históricos soltos e a mutation score antiga não fecham o passo;
  a revalidação publica artefatos e certificado próprios.

## Limite de segregação

Contrato, execução e veredito foram ordenados por artefatos, porém realizados
no mesmo filesystem e contexto. Não há atestação ambiental forte nem campanha
nova de mutantes.

Os gates integrais e os hashes dos resultados estão selados em
`00_nucleo/diagnosticos/p1253-final-certificate.tsv`. O passo fecha sem escrita
produtiva e sem ampliar o alcance das evidências focais.
