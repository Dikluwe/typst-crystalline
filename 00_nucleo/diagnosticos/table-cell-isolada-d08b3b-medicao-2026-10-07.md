# D08B3-B — Cell isolada e reconstrução em show: medição antes da decisão

Estado: medido e candidato L0 preparado; aguardando ratificação do novo contrato público. Nenhum código produtivo ou Prompt L0 vigente foi alterado nesta etapa. A confirmação anterior de D08B3-A não inclui este pacote.

## Proveniência

`table-cell-isolada-d08b3b-2026-10-07/complete.json.gz` registra 53 sondas da mesma rodada, UTC inicial 2026-10-07T14:47:56.340144+00:00, final 14:48:12.375245+00:00. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado: lista exata, `git diff HEAD --stat`, hashes das fontes alteradas, comandos, ambiente, cwd, fontes completas e stdout/stderr/status estão na coleta. Snapshot textual integral das fontes/L0 alterados em `before-state.json.gz`. `before` e `refuters` são rodadas preliminares, não medições que se somam à final.

Oracle pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 do binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`, ambos congelados pelo script. O primeiro corresponde à proveniência pin/árvore/receita já ratificada em P1489. Diretório/cwd e dependências do ambiente permanecem os do registro; não se afirmou hermeticidade de fontes externas. Texto PDF não teve whitespace normalizado; HTML completo e erros completos estão armazenados. Nenhuma conclusão geométrica nova é tomada nesta medição.

## Resultados que produzem a decisão

| Caso na coleta final | Tekt | Vanilla pinado | Classificação |
|---|---|---|---|
| isolated-default | Só body; duas metadata | Body e nove propriedades; uma metadata | Defaults e efeito: língua |
| isolated-x | Erro de campo desconhecido | x/y e campos Smart = auto, spans 1 | Semântica |
| raw-fields/raw-x | Só body; x desconhecido | Mesmo resultado da linguagem | Controle de omissão lexical; stderr de empty-file difere |
| isolated-table-style | Só body | Defaults avulsos, sem herdar os da Table | Default intrínseco separado de ocupação |
| raw-vs-prepared | true | false | Conhecimento dos campos observado pela igualdade |
| reused | Quatro metadata | Duas metadata | Uma receita por inserção crua |
| body-effects | CELL, CELL, HEADING | CELL, HEADING | Efeito repetido |
| hidden-body-effects | CELL | CELL | Corpo substituído não executa |
| nested-effects | Quatro valores, corpo externo já transformado | Dois valores; body externo lexical | Origem e execução única |
| fresh-isolated/fresh-table | Aceita e renderiza a | Rejeita por limite de show, caret no constructor novo | Nova geração não é ponto-fixo por mesma forma |
| fresh-block/fresh-styled | Aceita | Rejeita pelo mesmo limite | Fronteiras transparentes não escondem geração |
| identity/body/alias | Aceita | Aceita | Reuso de it não é novo constructor |
| fresh-conditional/hidden | Aceita | Aceita | Nova geração finita; conteúdo criado mas ocultado não realiza |
| multi-identity | a | Aa | it conserva possibilidade de regra anterior |
| multi-return-table | Ba | BAa | Não revogar o snapshot inteiro |
| multi-fresh | Aceita | Rejeita por limite | Regra anterior não encerra reconstrução nova pela forma |
| selector-auto | a seguido de b | AUTO a seguido de b | Selector recebe defaults preparados |
| isolated-query | cell is not locatable | Mesmo erro completo | Não inventar Location |
| isolated-location | Duas none | Uma none | Valor correto; repetição incorreta |
| html-fields | Campo x desconhecido | Produto X auto a | Default também é superfície HTML |
| cell-set | Warning de target não suportado; defaults ausentes | fill/inset/breakable aplicados | Débito separado D08B3-C |

O vanilla materializa campos antes de entregar o argumento show (`typst-realize/src/lib.rs:571–586`), guarda a regra no argumento (`:379–386`) e limita a visita do produto (`:418–429`). `model/table.rs:734–778` declara propriedades/defaults. Essas fontes descrevem comportamento e mecânica; não provam intenção. O Tekt continua livre para sua representação nominal pura.

## Decisão proposta, não implementada

Preparar Cell avulsa antes de selecionar/aplicar show, distinguindo-a do plano de Cell posicionada. Introduzir um carrier avulso e um campo público opcional na entidade; x/y auto nunca são coordenadas fictícias. Defaults só ficam conhecidos na preparação: constructor cru continua omitido. Body permanece lexical, visual completo é a única contribuição percorrida/indexada/renderizada.

Conservar guards por regra na ocorrência preparada. Devolver it ou alias mantém a regra consumida, mas admite regras anteriores ainda não consumidas. Novo constructor inicia nova ocorrência, mesmo com body igual; suas gerações seguem limite causal e diagnóstico com span do constructor. Somente produtos emitidos são realizados. Esta é uma decisão nominal do Tekt inferida dos observáveis, não obrigação de copiar Packed/guarded/materialize upstream. É refutada por qualquer negativo persistente de fields, igualdade, effects, duas regras, wrappers/alias/fresh, erro causal ou controles de Table/Raw.

Contrato completo, ownership e candidatos: `00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md` e diretório irmão. O manifesto tem 20 owners 1:1 e um núcleo compartilhado pinado; bases e candidatos completos estão preservados. Não instalar, selar ou materializar antes do gate ADR-0127 para o campo/tipo público novo. Após ratificação, testes RED→GREEN e revalidações indicadas no pacote são obrigatórios. Nenhum build/teste/lint novo foi alegado como aprovação de implementação nesta etapa.

D08B3-C conserva `set table.cell`, medido como negativo próprio. Defaults/fields/igualdade gerais de Table, casts/paint/strokes/insets ricos, linhas/gutters, spans multipágina, PDF/UA, warning de arquivo vazio e terminação global de não-localizáveis seguem fora da certificação. Esta coleta não mede percentual global de paridade.

Execução posterior confirmada pelo dono e concluída: ver `table-cell-isolada-d08b3b-execucao-2026-10-07.md`. O estado aguardando ratificação acima é o registro desta medição anterior, supersedido pelo recibo e relatório; seus dados não foram apagados.
