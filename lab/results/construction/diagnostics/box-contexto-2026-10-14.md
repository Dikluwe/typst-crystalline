# Posição contextual dentro de Box: referencial local exposto

## Medição e proveniência

Sondas de 2026-10-14T03:27:34.496134+00:00 a 2026-10-14T03:27:40.280883+00:00, HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree
não commitado. probe.json.gz contém status, git diff HEAD --stat, comandos,
fontes, diagnósticos e texto extraído. source-state.json.gz preserva fontes
e L0 inspecionados. Binário Tekt eb5ff271f3eb0edc3986903e6771783f30f78de8794910d947a35210b49bce69; vanilla
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin 9f2b6e8715237cb086899a42873660fe744622e8.
A identidade dos binários foi verificada antes e depois das sondas.

Página 220×100pt, margens 10pt, fonte 11pt. Cada fonte imprime as coordenadas
de here().position() em um context. Cinco fontes nos dois binários:

| Entrada | Tekt (x,y) | Vanilla pinado (x,y) |
|---|---|---|
| Context direto | (10pt,10pt) | (10pt,17.24pt) |
| Box 80×20pt | (0pt,0pt) | (10pt,10pt) |
| Box com inset 2pt | (0pt,0pt) | (12pt,12pt) |
| P antes de Box | (0pt,0pt) | (15.95pt,10pt) |
| Box externa com inset 3pt | (0pt,0pt) | (13pt,13pt) |

As strings são a representação observada das coordenadas, arredondada pelo
produto; não prometem precisão superior à impressão. Todos compilaram sem
erro. Os PDFs estão arquivados.

## Classificação e hipótese antes de decidir

Não copiar a baseline do controle direto por mera divergência: seu y
precisa de classificação própria entre âncora contextual e baseline
textual. A falha dentro de Box tem evidência adicional independente do
vanilla: inserir inset 2pt ou um prefixo horizontal não altera a posição
publicada, apesar de deslocar o conteúdo visual. A posição da página não
pode permanecer no referencial local ignorando essas transformações.

boxed.rs compõe o corpo via subframe geométrico na origem local e depois
insere Group com pos do pai/inset. sub_frame.rs:174-194 muda o cursor para
a região local. mod.rs:1131-1148 escreve runtime.positions a partir desse
cursor; a composição de Box não aplica o deslocamento final a esses
registros. Hipótese causal: a translação acompanha FrameItems, mas não as
posições observáveis já capturadas. Refutar mostrando a transformação
final desses registros antes de publicar o resultado do fixpoint.

O núcleo coordinates ainda fala em origem absoluta de sub-layout e
rebase comum de FrameItems/decorações/fixups; o percurso local precisa de
uma auditoria conjunta de contrato e implementação. Não resolver somando
margens indiscriminadamente nem mudando o esperado para zero: inset,
prefixos, aninhamento, transformações e baselines são eixos distintos.

## Prioridade e critérios do reparo

Priorizar esta composição causal antes de declarar Box pronta. O reparo
precisa conservar identidade/ordem de Location, número de ocorrências,
queries, here, callbacks, checkpoints e posições do pai ao transformar
somente o resultado local pertinente. Repetir layout do corpo ou percorrer
um mapa global inteiro por caixa não é uma solução automaticamente aceita;
medir o comportamento e revisar os owners antes de escolher a estratégia.
O layout canônico aloca Location em ordem de travessia; adiar caixas
fracionárias deve também preservar esse contrato.

Este diagnóstico não é Prompt L0 e não autoriza um novo contrato público
ou mudança de fase. O deslocamento de metadados exige testes causais
RED→GREEN próprios, além das sondas visuais já verdes. O objetivo geral
permanece aberto; não houve alteração de produção neste diagnóstico.

## Preservação

evidence.json.gz contém fontes Typst, PDFs, resultados, snapshots e receita
com SHA-256 conferido após descompressão. Não houve commit/push/publicação.

## Continuação da correção

Os casos de posição em Box foram corrigidos e revalidados em
[posicoes-compostas-2026-10-14.md](posicoes-compostas-2026-10-14.md).
A tabela acima conserva a medição anterior. A âncora do contexto direto
continua separada; o novo diagnóstico de
[links em paginação](links-paginacao-2026-10-14.md) registra um panic
adicional fora da suíte, ainda pendente.
