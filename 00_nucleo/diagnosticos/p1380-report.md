# P1380 — relatório do prefixo inline antes de `fr`

## Resultado

Prefixos inline simples, estilizados e compostos agora contam como uma única
linha fixa antes do primeiro Block `fr` de um segmento explícito. A matriz
bilateral fechou quatro `MATCH` e manteve o caso com wrap como `Unknown`.

## Evidência de linguagem

- Styled: `Lead=7,404`, `One=27,842`, `After=220,166`;
- Strong: `Lead=7,261`, `One=27,699`, `After=220,166`;
- composto: `Lead=More=7,404`, `One=27,842`, `After=220,166`;
- o controle direto P1379 conserva `One=27,842` e `After=220,166`.

Todas as caudas da coluna irmã permanecem em `220,166`. As coordenadas dos
PDFs coincidem com o vanila dentro de 0,001pt; tamanhos/bytes de PDF não fazem
parte da alegação de paridade.

## RED → GREEN e ataques

Os três testes positivos falharam antes do candidato. Styled e Strong
paginavam a cauda; o composto somava fragmentos como linhas independentes.
A implementação passou os três, e uma calibração corrigiu o teste Strong:
`yMin` óptico do glifo bold não é a baseline interna do Block. A trava opaca
rejeita largura acima da região e estilo vertical. Os sete mutantes declarados
foram discriminados; score 1,0.

## Implementação

O preflight percorre a fatia anterior ao primeiro `fr` sem recursão ou árvore
auxiliar. Aceita Text/Space/Empty e wrapper textual de profundidade um com
estilo verticalmente neutro, acumula largura e reserva uma única altura de
linha mais a fronteira `par.spacing - par.leading`. A execução aplica essa
fronteira uma vez antes do Block. Nenhum estado ou contrato público mudou.

## Limites

Wrap, wrapper aninhado, mudança vertical de estilo, tracking, troca de fonte,
script, canal custom, fração aninhada e multi-região continuam `Unknown`.

## Atestação e memória

Protocolo completo sem atestação de isolamento: uma autoridade compartilhou
contrato, código, ataques e veredito. Testes e links foram seriais sob 4 GiB;
check/lint sob 3 GiB; matriz sob 2 GiB e timeout 45 s. Pico final observado:
37.576 KiB RSS; maior PDF: 9.879 bytes. Não houve OOM, timeout ou `SIGKILL`.
