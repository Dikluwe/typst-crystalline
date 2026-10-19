# Associação de labels respeita o escopo e atravessa parágrafos

Labels agora procuram o alvo dentro da cauda lexical corrente de markup,
atravessando espaços e quebras de parágrafo. Set e show iniciam uma nova
cauda, incluindo regras sem mudança efetiva de estilo. Configuração de
página deixa de ser tomada como alvo. O caso que antes falhava com cannot
reference text agora produz uma referência navegável ao heading correto.

## Medição e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/results.json.gz
registra a coleta de 2026-10-14T05:42:53.075749+00:00 a 2026-10-14T05:43:19.218979+00:00, com status,
git diff HEAD --stat, fontes, owners, comandos e hashes dos binários.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489; SHA-256 do oráculo
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

No baseline, attach_label ignorava apenas espaços e Empty, aceitando
Parbreak como alvo. O fluxo também não delimitava a associação quando
passava por SetRule/ShowRule. O fold de estilos detecta diferenças de valor,
mas não representa fronteira lexical para set igual ou condicional falso.
A fonte pinada typst-eval/src/markup.rs:35–56 abre uma cauda para essas
regras; model/par.rs:725–735 classifica Parbreak como não rotulável.

Nas sondas, parbreak direto, dentro de bloco e por alias davam COUNT=0
contra 1 no pin; set/show permitiam COUNT=1 onde o pin avisava órfão e dava
0. O controle let e o set interno a code block permitiam associação nos
dois lados. Nenhuma conclusão depende de copiar os buffers upstream.

## L0, implementação e testes

Os owners de markup_stream e de testes foram atualizados antes do código.
Um índice privado registra o início do escopo de associação depois de cada
nó sintático SetRule/ShowRule avaliado, inclusive seus carriers sintéticos.
A busca mutável é restrita à cauda e atravessa separadores com transparência
de Source. Os itens atravessados permanecem nos mesmos índices, preservando
snapshots textuais, estilos, origem e parágrafos. Apenas o alvo é substituído.

Esta é correção interna de paridade ADR-0127, sem nova API, campo, flag ou
fase. O resselo foi restrito aos dois consumers e seus owners. Quatro testes
novos cobrem separadores, regras/condições/mesmo estilo, page órfão e controles
de conteúdo novo/let/bloco. O filtro executou também um teste anterior de
confinamento: antes, três falhas e dois controles passando; depois, cinco
passaram. red-state.json.gz, candidate-final.json.gz e logs preservam a prova.
As expectativas dos testes existentes foram mantidas.

## Recoleta do produto

after/results.json.gz foi medido de 2026-10-14T05:48:14.076741+00:00 a 2026-10-14T05:48:43.486964+00:00;
Tekt SHA-256 4da1015ea55ab9a56f8274fdd43dde610688c46791bcaa608d5b1417f75eedd7.
Fontes preservadas durante a execução. Os 13 controles pertinentes
concordam em texto extraído e stderr completos, sem retirar espaços, linhas,
mensagens ou spans para fabricar equivalência. Os demais casos do mesmo
corpus continuam registrados como divergências.

references-before/after conservam duas sondas adicionais. references-before
usou o binário identificado do lote anterior, enquanto a árvore já continha
o candidato; não se atribui essa medição a um build da árvore registrada.
A referência após
Parbreak era rejeitada como cannot reference text e agora mostra Section 1;
a referência ao novo alvo após set mantém Section 2. Ambas têm COUNT=1.
MuPDF resolve o link da segunda página para o índice 0 nos dois binários.
Os retângulos foram registrados, sem exigir identidade de bytes ou geometria
geral com o vanilla. Isso prova navegação desses destinos, não PDF universal.

## Pendências com prova independente

Labels repetidos ainda precisam corrigir metadados, diagnóstico e ordem de
efeitos juntos. repeated-before/results.json.gz, de 2026-10-14T05:45:42.370838+00:00 a
2026-10-14T05:45:47.040728+00:00, registra quatro controles com fontes/binários/estado:

- Com show do label antigo chamando panic(OLD EXECUTED), seguido de um label
  novo cuja receita produz NEW, o Tekt aborta no antigo; o pin avisa label
  repetido e produz NEW, COUNT=1, OLD=0. Portanto trocar só o nome não basta.
- Sem substituição do label antigo, o panic ocorre nos dois; a fixture não
  esconde erro de parser, função inexistente ou callback não executável.
- Quando o panic pertence ao label final, ambos o executam. A correção deve
  preservar essa falha e não simplesmente suprimir erros de show.
- Repetir o mesmo label exige o warning de múltiplos labels; Tekt ainda omite.

intercept_labelled em compiler/eval/rules/intercept.rs executa a transformação
imediatamente e pode consumir o wrapper. O contrato de realização em
00_nucleo/prompts/compiler/realize/mod.md e o owner apply já distinguem
captura de receitas de execução. A continuação precisa auditar a integração
de labels com esse contrato e atualizar os owners afetados antes de codificar,
preservando identidade, origem, escopo, guards, query e avaliação única.
Não há implementação suplementar legitimada por este diagnóstico.

Empty explicitamente interpolado continua avisando órfão indevidamente no
controle #[]<alvo>; o baseline e a recoleta o conservam. A composição de
heading com show por label permanece aberta conforme o diagnóstico anterior.
O passo labels-associacao-alvos está parcialmente materializado: seleção e
fronteiras estão verificadas; labels-repetidos completa a parte ainda aberta.
As demais frentes da auditoria de produção/CI/release não estão certificadas
por este lote. O objetivo geral continua ativo.

## Falha adicional nas condições de set

A coleta conditional-set/results.json.gz, de 2026-10-14T05:52:56.089124+00:00 a
2026-10-14T05:53:01.478087+00:00, foi feita com o candidato já compilado e preserva
fontes dos owners set/text/page, comandos, binários e estado. Ela não foi
incluída nas contagens de equivalência dos controles de associação:

- set text(size:panic(...)) if false e set page(width:panic(...)) if false
  executam o panic no Tekt; o pin pula os argumentos e produz OK.
- Com if true, o panic ocorre nos dois, confirmando a validade do controle.
- Condição inteira 1 é aceita no Tekt e rejeitada no pin com expected boolean,
  found integer.
- Uma condição que incrementa n uma vez e retorna false deixa N=0 no Tekt,
  enquanto o pin mostra N=1. A condição está sendo ignorada, não avaliada duas vezes.

Fonte: rules/set.rs:44 trata condition somente no ramo de Cell; os owners
set/text.rs e set/page.rs não a consultam antes de avaliar argumentos.
Isso exige auditoria/atualização do L0 de set e RED próprio para precedência,
avaliação única e atalhamento antes de corrigir. As provas de escopo acima
não certificam comportamento geral de set condicional. Esta falha permanece
aberta e passa a integrar a fila de correções de produção.

## Validação e limites

Workspace completo: **8429 passados, 0 falhas,
7 ignorados**, entre 2026-10-14T05:47:58.538233+00:00 e 2026-10-14T05:54:25.134647+00:00.
workspace-pre.json.gz registra argv, ambiente /dev/shm, pin do oráculo,
fontes e hashes; workspace-result.json confirma ausência de mutações durante
a suíte. Ignorados não representam cobertura executada.

Build, check de todos os targets, rustdoc com -D warnings, fmt, diffcheck,
linhagem estrita V1/V5/V15/V26 e verificação reversa passaram. Arquitetura:
0 erros, 382 warnings,
1631 notas. Clippy foi inventário com cap-lints warn:
227 antes e 227 depois, delta {}.
Esses avisos continuam débito; exit 0 não é zero violações.

## Evidência preservada

O arquivo evidence.json.gz guarda sondas, fontes, PDFs, logs, scripts, diff
restrito e os quatro arquivos próprios, com SHA-256 verificado depois de
descomprimir. Nenhum commit, push, PR ou publicação foi realizado.


## Retificação posterior — identidade dos alvos vazios

A coleta codeblock-set-2026-10-14/labels-first/results.json.gz refutou a
inferência de que COUNT=1 nos controles let e CodeBlock significava label no
heading. O pin publica sequence vazia após let e Styled vazio após set no
bloco; o tipo do alvo não havia sido medido no corpus deste relatório.
As contagens/textos registrados continuam sendo os resultados da coleta,
mas não provam equivalência desses alvos. O teste que esperava heading foi
retificado com essa nova medição. Associação de None/Empty e query de vazio
continuam abertas. Ver codeblock-set-2026-10-14.md para fontes, estado,
resultados e limites; a evidência original deste lote permanece preservada.
