# Matriz de paridade por domínio — baseline atual

A coleta atual conserva **36 MATCH/36** nos critérios por modo, com Tekt
SHA-256 `11263dff535e04c00c15c8df48013bd4160431487e9b2c4ed25cc2bfd7ed12a7` e vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`. `measurement.json.gz` e
`summary.json` apontam para a coleta de identidade de Source; os arquivos
`before-source-identity-*` preservam o estado anterior. Proveniência completa,
UTC/HEAD/árvore alterada e critérios estão na coleta e no
[relatório de identidade de Source](origem-fontes-expressao-2026-10-06.md).
A coleta dedicada de diagnóstico está em 16/16 iguais; os dois carets de
warnings foram corrigidos (empty-markup-warning-2026-10-06.md). Não há percentual global de paridade.

## Histórico — coleta anterior de HTML/headings


A amostra macro atual tem **36 sondas declaradas: 31 MATCH e 5 DIFF**,
sem sonda inválida no resultado final. O corpus detalhado de texto/layout tem
**228/228**, com coleta independente no mesmo binário. **Não há percentual global
de paridade**: os denominadores são amostras escolhidas, com observáveis diferentes,
não uma enumeração ou amostragem estatística da linguagem inteira.

## Proveniência daquela coleta

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, início UTC `2026-10-06T13:47:53.182834+00:00`,
fim `2026-10-06T13:48:05.820391+00:00`. Tekt SHA-256 `d448be2e5831441836dba19333c0d74989a64c16c6cb72bcdd17075fe7ac80b8`;
vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada P1489.
measurement.json.gz conserva stat/status, hashes da árvore produtiva/L0 alterada,
fontes, comandos, resultados/retornos, PDFs/XML/DOM e SHA de raster. Fontes
materializadas nesta rodada em stroke-none/execution.json.gz e
html-headings/execution.json.gz. SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1,
NO_COLOR=1; fontes em font-provenance.json. Binários congelados. O L0 proposto
de show consta como unmaterialized_l0: sua presença não significa implementação.
Dados antes dos reparos em before-repairs-measurement.json.gz e before-repairs-summary.json.

## Medição antes da classificação

| Domínio da amostra macro | MATCH | DIFF | Total |
|---|---:|---:|---:|
| Avaliação | 8 | 0 | 8 |
| Sintaxe e diagnósticos | 0 | 4 | 4 |
| Introspecção e identidade | 5 | 0 | 5 |
| Contexto e contadores | 4 | 0 | 4 |
| Matemática | 6 | 0 | 6 |
| Formas e pintura | 3 | 0 | 3 |
| HTML | 3 | 0 | 3 |
| Recursos externos | 2 | 0 | 2 |
| Realização e limites de show | 0 | 1 | 1 |

Corpus detalhado de **texto/fontes/bidi/layout paginado**: 228/228 contra o mesmo
pin/binário; foco Align 48/48 em cada modo PDF; 12/12 controles de tinta. Dados
em [Align: fontes e observáveis](align-tabela-2026-10-06.md). Esse corpus se
sobrepõe internamente às frentes anteriores; não somar 168+228+48+12 para criar
um denominador maior. Sondas macro são outra coleta e não estendem automaticamente
sua certificação aos demais recursos do domínio.

## O que MATCH certifica

- Avaliação: asserts tipados, observação OK e caixas/páginas de saída, cobrindo
  operações, closures, coleção, loop, destructuring, escopo e comprimento UTF-8.
- Introspecção: JSON público de campos consultados; headings, labels, metadata,
  alias e heading gerado sob regra seletiva. Não comparar enum/PartialEq interno.
- Contexto: texto, páginas e caixas de contadores de página/heading nos casos
  declarados, incluindo atualização do contador de página.
- Matemática: seis controles básicos; texto/páginas/caixas e tinta raster da
  primeira página a 600dpi, tolerância bilateral ±1px. Todas as fixtures têm
  uma página. Não certifica a superfície matemática inteira.
- Formas: linha, rect e circle na amostra macro, com tinta raster. Controles
  adicionais em stroke-none certificam None explícito, omissão e fill em quatro formas.
- HTML: DOM do body de parágrafo e headings, com e sem show, incluindo tags/atributos/texto. Não
  exigir bytes de cabeçalho ou copiar a representação interna de HTML.
- Recursos: leitura JSON e import local com valor esperado. Não certifica
  packages remotos, bibliografia, imagens, plugins ou cache de watch.

PDF: texto sem formatação de direção/espaçamento do extrator, caixas de palavras
adjacentes agrupadas, tolerância <0,05pt, dimensão/quantidade de páginas. Tinta
é obrigatória nos controles de matemática/formas; uma página sem palavras não
é declarada visualmente equivalente somente por ter bbox vazio.

## Diferenças abertas, com classe e evidência

1. **Diagnósticos, apresentação do caminho (4 controles).** Ambos rejeitam e
   coincidem em mensagem, posição, marcação e hints após projetar exclusivamente
   o caminho da fonte. O Tekt mostra `/tmp/...`; o vanilla mostra `../../../tmp/...`.
   O stderr bruto diverge e permanece DIFF. Não é erro sintático diferente.
   `message_span_hint_match_after_source_path_projection` registra a comparação
   adicional, com script próprio e versão bruta da coleta preservada.
2. **Realização de show recursivo (1 controle).** Uma regra que cria outro heading
   abrangido por ela é rejeitada no vanilla por maximum show rule depth exceeded;
   o Tekt compila. A [auditoria](show-recursao-auditoria-2026-10-06.md) amplia o
   testemunho e encontra a política α documentada. O L0 por ocorrência está pronto,
   aguardando confirmação no gate de compatibilidade; nenhuma implementação de
   show foi modificada nesta rodada.

**Fechados nesta rodada:** as rejeições de `stroke: none` em rect/circle e os
headings de HTML comum/show. [Formas](stroke-none-2026-10-06.md): 16/16;
[HTML](html-headings-2026-10-06.md): 13/13, incluindo raiz explícita sob estilos.

A primeira coleta usava essa regra recursiva para consultar identidade. A
referência rejeitou a fixture, invalidando aquele uso como testemunho de query.
O controle de identidade foi refinado para `heading.where(level: 1)`, e passou;
a regra original permanece como sonda específica de recursão, DIFF. Os dados
iniciais não foram apagados: initial-manifest.json/initial-measurement.json.gz.
Não escolher seletivamente a saída verde nem reclassificar a recursão como sucesso.

## O que falta medir antes de falar de paridade integral

| Área | Estado desta baseline |
|---|---|
| Sintaxe válida ampla / parser | Asserts exercitam parsing; sem catálogo/sampler amplo ou AST oracle atual |
| Biblioteca padrão inteira | Oito casos de eval; sem inventário runtime bilateral atualizado |
| Layout complexo | Corpus causal atual; floats, footnotes, colunas, spans/headers complexos não certificados amplamente |
| Fontes / OpenType amplo | Recortes bidi/face/eixos do corpus; catálogo de shaping completo não reenumerado |
| Matemática ampla | Seis casos; atributos, construtores e combinações não inventariados |
| SVG / PNG | Não reexecutados nesta baseline; PDF raster não prova esses targets |
| HTML / bundle | Três casos HTML; bundle e superfície inteira não reexecutados |
| CLI / recursos / watch | Erros e dois recursos locais; ciclos de edição, memória e invalidação não medidos aqui |

As porcentagens do inventário histórico P447/P453 e o rebaseline P1210 (pin
anterior) não são evidência do estado atual. Testes unitários e lint também
não viram percentual de paridade. O próximo ciclo deve escolher uma unidade
aberta, escrever o L0 proprietário após medir a fonte e usar RED→GREEN e
revalidação bilateral. Próxima frente preparada: revisão de recursão de show por ocorrência, após
confirmação. O caminho de diagnóstico permanece uma diferença de apresentação
separada. Numbering/avisos de headings profundos e demais limites HTML não foram
certificados por esses controles.

## Atualização — show por ocorrência materializado em 2026-10-06

O gate foi confirmado e implementado. Ver `00_nucleo/diagnosticos/show-ocorrencia-reparo-2026-10-06.md` e seu receipt repair/execution.json.gz: 6.551 testes L1, 1.183 L3 e 105 CLI passaram; 228/228 fixtures de layout não regrediram. As quatro gerações ilimitadas medidas agora rejeitam, com aliases e gerações finitas preservados. A matriz macro mantém 31 MATCH/5 DIFF crus; todos os cinco DIFF atuais se limitam ao caminho da fonte, com mensagem/range/hints iguais após projeção explícita. A proposta de apply não está mais aguardando confirmação. Persistem origem de Content constante, caminhos crus, duas fixtures de matriz e seletor table.cell; não se afirma ganho de tempo.

## Atualização — caminhos dos diagnósticos, 2026-10-06

O cálculo relativo de arquivos foi corrigido com pathdiff pinado, sem mudança de API ou unsafe. A matriz macro passou a 36 MATCH/36 nos critérios originais, incluindo stderr completo nos cinco erros antes divergentes, sem projeção. A suíte CLI passou 108 testes; três testes físicos foram repetidos após delimitar a expectativa transiente incorreta. A coleta ampliada de 15 casos mantém seis DIFF explícitos de origem/avisos/stdin. Ver `00_nucleo/diagnosticos/diagnostic-paths-2026-10-06.md`.

A auditoria seguinte encontrou identidade compartilhada de Source entre documento e expressão, com perda de origem em eval contextual; o L0 vigente também determina essa identidade. A frente requer especificação antes do código, conforme o passo `00_nucleo/materialization/origem-fontes-expressao-2026-10-06.md`. Matemática fora de Equation, morfologia de query body, selector math.equation e table.cell permanecem lacunas próprias. Os resultados desta matriz não são percentual global de paridade.

## Atualização — identidade de Source corrigida, 2026-10-06

Após o gate confirmado, eval contextual passou a manter documento, expressão
e stdin com identidades distintas, preservando origem de imports/callbacks e
base de read. A compilação usa o introspector existente sem outra execução do
programa. O alocador não recicla ids nem sobrescreve fontes ao esgotar.

A coleta dedicada subiu de 5/16 para 14/16 coincidências integrais; os dois
negativos restantes são o caret curto de **, já com origem correta. Matriz macro
36 MATCH/36 nos critérios anteriores e 111 testes CLI passaram. Não é percentual
global de paridade nem ganho de tempo. Ver
`00_nucleo/diagnosticos/origem-fontes-expressao-2026-10-06.md` e receipt associado.
As fachadas string legadas conservam compatibilidade; a CLI usa Source-based.
Matemática automática/morfologia/seletores, table.cell, origem de Content
constante, warnings de query e compile stdin continuam em frentes próprias.

## Atualização — carets vazios corrigidos, 2026-10-06

O alcance de **/__ foi corrigido pela relação real entre o marcador de warning
e seu construct na Source numerada, sem alterar a CST ou fases. A coleta de
identidade passou de 14/16 para 16/16 integrais; seis controles adicionais e a
matriz macro de 36 mantiveram seus critérios. Ver
`00_nucleo/diagnosticos/empty-markup-warning-2026-10-06.md` e receipt. Estes dois
negativos estão fechados; demais frentes permanecem conforme seus próprios
diagnósticos. Não é prova de paridade global nem de ganho de tempo.

## Atualização — compile stdin com saída explícita, 2026-10-06

Corrigida a composição de compile - OUTPUT: stdin é adquirido uma vez e
for_stdin conserva Source/base de recursos. Output - agora publica em stdout,
também para input físico; o arquivo físico - não é aberto/sobrescrito.
15/16 controles bilaterais passam nos critérios declarados; a omissão de output
continua DIFF no parser L2 e fica aberta. PDF mede texto/páginas/caixas;
HTML mede DOM; SVG/PNG têm alcance restrito, sem nova certificação de tinta.
115/115 CLI no estado final, três regressões RED→GREEN, builds e diff-check
passam; lint zero erros, 530 warnings, 1578 notes, owners sem V5/V6/V15/V26.
Somente dois owners selados, sem API/fase/unsafe; tempo não medido. Não somar
esta coleta à matriz macro anterior. Ver
`00_nucleo/diagnosticos/compile-stdin-2026-10-06.md` e execution.json.gz.

## Atualização — plano vivo e primeiros débitos, 2026-10-06

O inventário consolidado está em
`00_nucleo/diagnosticos/plano-debitos-2026-10-06.md` e a execução em
`00_nucleo/diagnosticos/plano-debitos-execucao-2026-10-06.md`.
Corrigida a exigência de output para stdin (usage bruto ainda diferente) e
reconhecimento estático de math.equation em query/selector/alias. Filtros,
morfologia matemática, realização automática e table.cell ficam separados.
Warnings aguardam revisão da ordem vigente ADR-0045 e gate da função pública
L2, com L0s candidatos e manifesto completos. 118 CLI no estado entregue,
L1/L2/builds/diff-check aprovados; owners selados sem drift. Coletas não são
somadas à matriz macro histórica; o binário novo não reexecutou essa matriz.
Watch/incremental têm inventário de dependências e plano de medição, sem API
ou ganho de tempo escolhido. A aprovação geral do plano não fecha as frentes.
