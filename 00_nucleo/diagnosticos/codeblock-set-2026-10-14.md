# CodeBlock: caudas de set conservam conteúdo e estilos

Cada set direto de um bloco de código agora separa o prefixo já avaliado
da cauda restante. A cauda é convertida em conteúdo e recebe as declarações
da regra antes de ser juntada ao prefixo. False/set vazio mantêm a conversão
sem criar estilo. Texto comum e matemática conservam seus estilos locais;
erros de join apontam para a regra ou expressão que os provoca.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before/results.json.gz
registra 2026-10-14T06:16:16.015165+00:00 até 2026-10-14T06:16:37.796367+00:00, fontes/owners, status, git diff HEAD --stat,
comandos, saídas completas e hashes dos executáveis. Oráculo vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489 e SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Fonte pinada typst-eval/src/code.rs:36–57 converte a cauda antes do join com
o prefixo. O dispatcher Tekt juntava valores em uma sequência plana; só
subconjuntos Table/Cell/Mathy guardavam estilos. O baseline devolvia integer,
string ou none onde a linguagem expõe conteúdo; aceitava juntar array antes
de set a array depois; ignorava o estilo de texto e da equação; omitia o
warning de return sobre conteúdo descartado. São observáveis da linguagem,
não um requisito de copiar a recursão ou a estrutura Rust do vanilla.

A medição física docs-before conserva PDF/XML MuPDF, texto e tamanhos. O
fixture A/B/C/D/E/F usava 11pt em todos os caracteres no Tekt, contra
11/14/18/14/22/11pt no pin. Um alias com declaração repetida de 20pt era
renderizado em 30pt após novo set externo; math x ficava em 11pt contra 20pt.
Table Q20/R10 já funcionava e serviu de controle contra aplicação duplicada.

## L0 e implementação

Owners expression_dispatch e tests atualizados antes dos testes e do código.
Oito testes novos falharam no baseline. A implementação usa uma pilha privada
de fronteiras e deltas locais por ancestralidade, sem nova recursão por número
de sets, API pública ou fase. O fechamento de dentro para fora conserva os
prefixos, incluindo spans de erros. Styled contíguos compõem deltas; Source
conserva origem. Scopes são restaurados também quando há erro.

Table/Cell conserva as receitas locais existentes, sem realizar o corpo ao
guardar o valor e sem aplicar o delta uma segunda vez por célula. Math usa a
fronteira lexical, sem fabricar outra Equation. SetPage continua transportando
suas propriedades físicas na sequência; sua morfologia pública segue pendente.
Correção interna de paridade ADR-0127; resselo só dos dois consumers e owners.

## Refutação da primeira tentativa e retificação de teste

A primeira suíte completa, entre 2026-10-14T06:24:43.239254+00:00 e 2026-10-14T06:30:43.799160+00:00, registrou
8444 passados, 1 falha e 7 ignorados.
O teste que falhou assumia que #heading[Alvo] seguido de um CodeBlock com set
sem corpo deixava o label no heading. Essa expectativa estava errada.
labels-first e labels-final medem o alvo no pin: styled com child vazio;
uma referência a ele é rejeitada como cannot reference styled. O antigo
COUNT=1 não provava identidade do alvo. let também insere um valor vazio no
pin, em vez de deixar o label no heading. O teste anterior foi corrigido:
novo heading após set e comentário continuam controles; a associação ao vazio
de CodeBlock tem teste próprio. O controle let incorreto não é apresentado
como resolvido. Logs e fontes anteriores permanecem preservados.

refuters-after encontrou outra falha real do primeiro candidato: show raw
não atuava sobre Bool convertido pela nova cauda. Um novo teste RED confirmou
a falha. Bool/Auto recém-apresentados agora passam uma vez pelo interceptor
existente, preservando guards/captura; Content já existente não é interceptado
novamente. O helper do teste de estilos passa a rejeitar variantes inesperadas,
removendo o warning V16 que sua versão inicial introduzia. A sequência final
executa dez testes do lote com sucesso. A retificação do teste de label e o
reparo Raw são distintos: não se atribui ao código uma falha de expectativa.

## Recoleta final

Tekt SHA-256 976f15316eb3be613f5c9ac344679ef65a7a16e2e48023d4d8cefc4344d2b3ef. after-final foi coletado de
2026-10-14T06:36:09.987679+00:00 a 2026-10-14T06:36:36.429700+00:00; docs-final de 2026-10-14T06:36:09.992172+00:00 a 2026-10-14T06:36:31.377423+00:00 e
refuters-final de 2026-10-14T06:36:09.993412+00:00 a 2026-10-14T06:36:27.824448+00:00. Fontes não mudaram durante
cada coleta. Dos 37 cenários eval, 28 concordam em exit/stdout/stderr completos;
as nove diferenças continuam enumeradas em acceptance.json. Não se normaliza
JSON, mensagens, spans ou whitespace para fabricar equivalência.

Os doze documentos e dez refutadores adicionais concordam integralmente em
exit, stdout, stderr e texto extraído. Tamanhos dos glifos em style-scopes,
same-style-alias, local-cell e local-math concordam com o pin. O documento de
página preserva 200×100pt. Isso prova esses estilos e a dimensão medida, não
geometria geral ou identidade de bytes PDF. Os previews PNG dos dois PDFs
de style-scopes foram inspecionados; visual-qa.json conserva comandos e hashes.
Os refutadores cobrem Raw,
Table/Cell com regras anteriores/posteriores/aninhadas, corpo ocultado que
chamaria panic, combinações true/false e return com state update.

## Pendências explícitas

- Serialização CLI de Empty/Raw perde tipo/campos; isso aparece nos JSON
  de vazio, Bool e loops, mesmo quando render e controle de fluxo passam.
  02_shell/src/cli.rs:1152–1192 já retira Source; o problema não deve ser
  atribuído a esse wrapper sem prova. Faltam os casos Empty/Raw na projeção
  do conteúdo, separados do helper de encoding da stdlib.
- A apresentação de array ainda é Text onde o pin usa Raw typc. O helper de
  display possui escopo parcial explícito; não foi ampliado sem seu L0.
- styled.child continua rejeitado; Equation expõe campos derivados adicionais;
  SetPage ainda aparece como filho na morfologia pública. Esses owners não
  foram alterados aqui.
- Query de label sobre Styled vazio não publica o alvo no Tekt. Labels após
  let/None e #[] também associam indevidamente o heading anterior. Esse erro
  não é resolvido pelo teste estrutural de CodeBlock e segue aberto para os
  owners de markup/introspecção. Não há falsa certificação de labels vazios.
- Show sem seletor e a cauda genérica de show continuam no passo próprio;
  outras frentes de produção, arquitetura e matrizes remotas CI/release
  permanecem abertas. O objetivo amplo continua ativo.

## Validação final e evidências

Suíte completa: 8447 passados, 0 falhas,
7 ignorados, entre 2026-10-14T06:35:49.692745+00:00 e 2026-10-14T06:41:51.295351+00:00. Fontes preservadas;
workspace-pre/result registram estado, hashes, pin, argv e ambiente /dev/shm.
Build, check de todos os targets, rustdoc -D warnings, fmt, diffcheck,
linhagem estrita V1/V5/V15/V26 e reversa passaram. Clippy foi inventário
cap-lints warn: 227 antes e 227 depois;
delta {}. Arquitetura: 0 erros,
382 warnings e 1631
notas. Warnings e ignorados não são prova de cobertura ou zero violações.

evidence.json.gz preserva scripts, fontes/owners, sondas, PDFs/XML, logs,
diff restrito e relatórios com base64 e SHA-256 verificado após releitura.
Nenhum commit, push, PR, publicação ou mudança do pin de CI foi feito.
