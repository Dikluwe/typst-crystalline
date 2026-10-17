# Espaço literal terminal no PDF

Corrigido o único controle pendente do lote anterior: **129/129 passam**,
sem regressões. O espaço terminal do trecho LTR no parágrafo RTL agora fica
preservado na extração e em sua caixa, mantendo tinta e posições existentes.
A extração bruta desse caso coincide com o vanilla pinado.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC `2026-10-06T11:53:31.387765+00:00`.
[before.json](espaco-terminal-2026-10-06/before.json) conserva código/L0
anteriores, stat/status completo, binários e SHA do PDF usado nas sondas.
[execution.json.gz](espaco-terminal-2026-10-06/execution.json.gz) conserva
estado final, logs de testes/build e SHA dos binários; seals.json registra
os dois consumers e owners exclusivos. Fontes são as mesmas da rodada
anterior, com SHA em font-provenance.json.

Oracle vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt anterior SHA `94c18ba207426bc5e81259b381b7842717009189f179768e00183e0a76de8f01`;
final SHA `bc072887ed0178966ac08b217c8623b1a1876f3d1856299d05361f565c350bad`.
CLI com SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1, tags desativadas e pretty=false.
Fonte literal, comandos, extração bruta e XML/bbox estão em
[verification.json.gz](espaco-terminal-2026-10-06/verification.json.gz).
Compacto tem coleta independente em compact-verification.json.gz.

## Medição antes da decisão

O Tekt já emite um glifo de espaço em x=121.50342pt. O extrator descarta esse
espaço no fim do run e limita a caixa de 123 a x=121.49581pt. No vanilla a
caixa inclui o espaço e termina em x=125.000002pt. Os glifos visíveis já
estavam posicionados corretamente: modificar o avanço ou mover letras
seria tratar uma causa que a medição não mostrou.

[Streams](espaco-terminal-2026-10-06/streams.json) mostram ActualText
0020202C no vanilla: um espaço seguido de encerramento de direção para a
representação do PDF. A sonda pdf-boundary-probe.json mediu que somente
ActualText 0020 não resolve, enquanto 0020202C conserva texto/caixa. A sonda
alternativa word-envelope-refutation.json, atravessando blocos gráficos
restaurados, muda a linha da extração. Essa construção específica foi
refutada; não é prova de que toda estratégia de agrupamento seria inviável.
Não inferir intenção histórica da emissão anterior.

## Correção interna

O owner separator recebe origin e BidiSpan existentes pelo despacho top/local.
Certifica Literal, span exato de U+0020 no fim de segmento Text com prefixo,
dir LTR e input base RTL. Sem esse certificado permanece a emissão anterior:
markup/newline, espaços internos, base LTR e ausência de origem não recebem
fronteira inferida por posição, fonte ou aparência.

Para o certificado, a exportação abre ActualText espaço+PDF, emite somente o
glifo original de espaço uma vez, fecha EMC antes da restauração Q pendente.
Os helpers compacto/verboso conservam fonte, x, baseline e seus envelopes.
U+202C é formatação derivada exclusiva dessa representação PDF; não entra
em Content, texto autoral dos carriers, input bidi, glifos ou ToUnicode global.
O CMap antes/depois do controle é idêntico. Não há novo avanço, shape,
layout, mudança de fase, API pública, dependência ou unsafe.

Depois, a caixa de 123 termina em 125.00142pt, diferença 0.001418pt do oracle.
O texto bruto do caso original e do controle com dois espaços terminais
coincide com o oracle; ambos preservam a quantidade de espaços (space-extraction.json).

## Validação

| Lote | Antes | Depois |
|---|---:|---:|
| 129 controles herdados | 128 | 129 |
| 19 controles adicionais | 9 | 11 |
| Total verboso | 137/148 | 140/148 |
| Compacto direcionado | 9/20 | 12/20 |

Sem regressões em casos antes positivos, incluindo os oito controles
adicionais que seguem divergentes. Critério: identidade Unicode e caixas
agrupadas com tolerância 0,05pt; maior delta aceito no verboso 0,016206pt.
Texto bruto preservado permite conferir espaços sem a normalização da
comparação. Não usar igualdade de bytes/GIDs como paridade.

Um teste novo falhou antes da correção e passou depois: certificado positivo,
negativos e EMC antes de Q nos dois modos. **1.178 testes L3 passaram** com
SOURCE_DATE_EPOCH=0; build debug/release e diff-check passaram. Erros de tipo
ocorridos na escrita do teste estão nos logs e não são contados como RED
comportamental. L1 não mudou nem foi repetida sua suíte.

Lint: zero erros, 531 warnings e 1.578 notes anteriores, sem novos diagnósticos
por regra/mensagem/arquivo (deslocamento de linhas ignorado). V15/V26 íntegros;
V5 ausente nos dois consumers. Resselo localizado; não zero violations global.

Quatro renders a 600dpi/threshold 180: caixas de tinta iguais ao vanilla e
zero pixels escuros sem correspondente dentro de 1px nas duas direções.
Comparação e inspeção visual em raster.json. Não afirmar igualdade pixel/byte.

## Dívidas reveladas pela matriz adicional

Os oito casos adicionais ainda divergentes já falhavam antes. Tabelas em
ambas as direções têm posições/quebras diferentes; Block RTL com espaço
terminal desloca o texto em aproximadamente um avanço por espaço. No
controle compact rtl-block-1, ABC começa em 152.949pt antes/depois e em
149.45264pt no oracle. Isso é divergência geométrica separada: não é
corrigida por marcação de extração. summary.json enumera as oito fontes e
verification.json.gz conserva antes/depois/oracle. Causas internas ainda não
foram isoladas; nenhuma conclusão histórica ou geral sobre grids é fechada.

Esta rodada fecha o caso solicitado e preserva os positivos. Não certifica
todos os contextos paginados/geométricos, fallback sem fonte, extratores ou
PDFs tagueados. Não mediu tempo nem alega ganho de CLI. Coverage::from_ranges
permanece proposta separada, sem materialização.
