# Links e referências atravessando uma quebra automática

## Medição antes da decisão

Sondas de 2026-10-14T04:21:11.963366+00:00 a 2026-10-14T04:21:16.452497+00:00; HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. Estado,
status, diffstat, fontes, comandos, saídas e hashes estão em
posicoes-compostas-2026-10-14/automatic-boundary-probes/controls.json.gz.
O snapshot source-state.json.gz conserva os fontes e L0 inspecionados.

Tekt debug SHA-256 cc8ad709c02ae3a64b23e08f90f06ed03204b81b46057b92db45bb13789ee198.
Vanilla SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4; receita P1489.
Identidades verificadas antes/depois. Não usar esta medição como prova
contra um commit limpo: o snapshot inclui alterações não commitadas.

Página 160×40pt, margem 10pt, heading seguido do conteúdo da linha abaixo:

| Caso | Tekt | Vanilla pinado |
|---|---|---|
| Texto comum | Duas páginas, Alvo e Texto | Mesmo conteúdo/páginas |
| Link URL | Panic, exit 101 | Duas páginas, Alvo e LINK |
| Ref normal | Panic, exit 101 | Duas páginas, heading e Section 1 |
| Ref de página | Panic, exit 101 | Duas páginas, Alvo e page 1 |

As caixas de texto/raster não foram comparadas; o critério aqui é término
normal com o conteúdo e a ligação preservados. O controle de texto comum
isola a falha de coleta de links da transição básica de página.

A sonda original h40.typ, com counter(page).update(7), footer explícito e
consulta contextual, também aborta. backtrace.log localiza o drain em
references.rs:270. Os controles da mesma fonte com altura 50/60/70pt
compilam e mostram R7; results.json.gz conserva essa varredura. A suíte
completa desse mesmo estado passou, portanto esses casos não estavam
cobertos pelos testes existentes.

## Fonte e hipótese causal

references.rs:215-239 e :253-275 guardam comprimentos dos buffers antes de
renderizar o texto e coletam os sufixos depois. link.rs:22-32 faz o mesmo
para o corpo rico. O layout pode fechar uma linha ou página: os buffers
são movidos/esvaziados e os índices anteriores deixam de identificar o
conteúdo produzido. O drain usa o índice antigo e pode ultrapassar o novo
comprimento. O caso de Ref tem proteção parcial somente para a linha;
current_items continua sem proteção. A pilha reproduz o erro concretamente.

O owner de Link já registra a limitação de body-flush e segurança multipágina
não demonstrada. Isso não torna aceitável o panic de produção, mas também
não prova que este lote de posições o introduziu: a medição anterior à
mudança desses pontos não foi executada. Não atribuir origem histórica sem
um bisect/sonda equivalente.

## Critério do próximo reparo

Atualizar primeiro os owners de Link/References para coleta que sobreviva
às transições, conservando cada trecho na sua linha/página e envolvendo
apenas conteúdo do corpo. Não basta limitar índices ao comprimento atual:
essa mudança pode esconder perda de texto, incluir prefixo externo no link
ou mover toda a ligação à última página. Exigir RED→GREEN para quebra antes
do primeiro glifo, quebra dentro do texto, prefixo/sufixo externos, corpo
rico, várias páginas, destino/bounds, contexto/identidade e controles sem
quebra. Conservar os contratos de numeração e o protocolo de posição.

Prioridade imediata antes de declarar paginação pronta. Este diagnóstico
não é Prompt L0 e não altera código. Fontes, PDFs e recibos ficam no arquivo
evidence.json.gz de posicoes-compostas-2026-10-14, com hashes verificados.

## Seguimento da captura

O reparo e suas validações estão em links-captura-2026-10-14.md. As sondas
de fronteira deixaram de abortar. A referência Page após quebra automática
ainda pode apontar para a página seguinte; a ausência de panic não encerra
a correção semântica dessa referência. Este diagnóstico conserva o RED
anterior e a proveniência daquele estado.
