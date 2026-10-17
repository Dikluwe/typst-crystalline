# Box fracionária: medição da composição ainda incompleta

## Proveniência antes da classificação

Sondas de 2026-10-14T03:24:24.063861+00:00 a 2026-10-14T03:24:35.024856+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado. probe.json.gz registra status, git diff HEAD --stat, fontes,
comandos, fontes dos owners e binários; allocation-source.json.gz conserva
o alocador e seu L0. O binário não mudou durante a medição.

Tekt debug eb5ff271f3eb0edc3986903e6771783f30f78de8794910d947a35210b49bce69; vanilla 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8. Página 120×100pt, margem 10pt,
fonte 11pt, P antes e Q depois da caixa. Sete entradas, ambos os binários.
Não houve mudança de produção neste diagnóstico.

## Observações

- Caixa vazia width:1fr/height:14pt/fill:red: Q termina na margem correta
  nos dois lados (xMin102.279pt no Tekt, 102.278pt no pinado), mas o fundo
  fica ausente no Tekt: zero pixels vermelhos contra 4844 a 144dpi.
- Inset 2pt: o Tekt pinta apenas uma faixa estreita e desloca P/Q de
  yMin 14.166pt para 18.166pt; o controle pinado mantém 14.166pt e a largura
  alocada do fundo. Outset 3pt também pinta apenas a largura provisória.
- Shape 140×24pt com clip e altura 14pt: Q passa de yMin 14.166pt para 58.516pt
  no Tekt. O corpo aparece além da região alocada; no pinado Q permanece
  na primeira baseline e o corpo é recortado à caixa.
- Texto ABCD: posição do texto coincide, mas fundo cobre somente a largura
  natural (1366 pixels vermelhos contra 4586), apesar da fração alocada.
- Texto com quebra: linhas seguintes voltam à margem da página no Tekt,
  em vez da origem interna da caixa; Q acompanha a última linha do corpo.
  No controle, Q permanece na primeira baseline e as linhas do corpo
  conservam a origem local. O número de linhas isolado não é o critério.
- Caixa fracionária dentro de caixa 50pt: o texto externo fica correto,
  mas o fundo continua natural, 1366 pixels contra 2542.

Estes números são observações das entradas arquivadas, não métricas globais
nem gates de identidade raster. Aceitação futura deve medir regiões,
origens, baselines, recorte, texto e efeitos.

## Fonte, hipótese e próxima decisão

boxed.rs:17-45 separa explicitamente width_fraction do caminho composto;
:232-245 registra fração e largura natural; :340-386 insere o fundo com
outer_w provisório em current_items. line_commit.rs:106-155 calcula shares
e translada os itens posteriores em current_line por share-natural_width,
sem recompor a região interna nem redimensionar aquele fundo.

Isto explica a combinação medida: sufixo horizontal correto com pintura
natural/ausente, e corpos que ainda interferem na linha externa. A etapa B
já estava declarada incompleta no L0 de boxed e no passo de composição;
não é uma divergência superior do Tekt a preservar nem uma promessa de
paridade já cumprida. A hipótese seria refutada por uma sonda que mostrasse
fundo, clip e corpo recebendo a alocação final antes da emissão; estas
entradas mostram o contrário.

O reparo precisa cobrir a composição do corpo na região alocada e conservar
seus efeitos/ordem e baseline, além da pintura. Só esticar o Shape deixaria
wrap, isolamento e clip errados. Frações irmãs, h(fr), pesos zero/negativos,
largura automática, repeats, callbacks e checkpoints exigem seus controles
antes de declarar o caminho pronto. O contrato específico e os testes
RED devem preceder a implementação; este diagnóstico não é L0.

## Evidências

Arquivos Typst, PDFs, XMLs, PNGs e receita em evidence.json.gz, com SHA-256
verificado após descompressão. Não houve commit, push ou publicação.
