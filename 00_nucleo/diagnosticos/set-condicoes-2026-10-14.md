# Condições de set: ordem, atalhamento e propagação de erros

A condição de set agora é avaliada uma vez, antes do target, dos gates de
features e dos argumentos. False preserva o estilo anterior e não executa
argumentos. Tipos inválidos falham no span da condição. Erros ativos nos
argumentos de heading e de elementos registrados deixam de ser descartados.

## Medição e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. A coleta inicial
before/results.json.gz ocorreu de 2026-10-14T05:56:08.548603+00:00 a 2026-10-14T05:56:32.754422+00:00.
Os arquivos de cada coleta preservam UTC, status, git diff HEAD --stat,
fontes e owners, comandos, saídas completas e SHA-256 dos executáveis.
Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489; binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
A recoleta principal ocorreu de 2026-10-14T06:07:46.487106+00:00 a 2026-10-14T06:08:20.855830+00:00, com Tekt
8062724ab4da4ea08d17b421279eb970d32e2d24bb4d9ac7983b5e14d27abfb2 e fontes inalteradas durante a execução.

Os perfis before adicionais usaram o binário anterior identificado pelo
hash, enquanto a árvore já continha edições do candidato. Não representam
um build dessas edições: a proveniência do executável é a do baseline.
Os perfis after usam o rebuild validado desta correção.

Fonte pinada typst-eval/src/rules.rs:15–34 avalia a condição antes de target
e argumentos. No baseline Tekt, rules/set.rs:44–54 só tratava a condição
para Cell, depois da sondagem do target. Heading e elementos registrados
usavam unwrap_or(Value::None), ocultando erros de argumento. As sondas
refutam o comportamento: false executava panic; o contador da condição
ficava em zero; true contava apenas o argumento; tipos inválidos eram
ignorados; heading perdia a numeração anterior mesmo sob false.

## Contrato e correção

L0 de rules/set e seu owner de testes atualizado antes de escrever os testes
e o código. A condição comum usa o helper existente vanilla_type_name para
mensagens; o ramo particular de Cell deixa de avaliá-la novamente. Os dois
pontos que engoliam erros passam a propagar SourceResult. O esquema aberto
do elemento registrado, extensão Tekt, foi preservado. A referência vanilla
legitima semântica/efeitos/diagnósticos, não uma cópia da estrutura interna.

Correção interna ADR-0127: nenhuma assinatura, campo público, flag ou fase
nova. Resselo limitado aos dois consumers e owners. Os oito testes novos
falharam no baseline e passaram no candidato. RED inclui ordem, avaliação
única, Cell/alias, tipos/ranges UTF-8, show-set, preservação de estilos e
propagação de erros do heading e da extensão registrada. Não foram mudadas
expectativas antigas. candidate-compile.log guarda uma tentativa com import
faltante; essa falha de compilação não foi contada como RED funcional.

## Recoleta e limites

Os 24 casos principais, 6 casos extras e
12 cenários de documentos por perfil concordam
em exit, stdout, stderr e texto extraído completos. Não foram removidos
espaços, carets, mensagens ou linhas para criar equivalência. Os perfis
cobrem default, html, a11y-extras e html+a11y-extras, com false/true/inteiro.
False não acessa o binding de feature; true mantém o gate quando desativado;
inteiro falha antes do gate. Documentos com feature HTML ligada neste corpus
continuam tendo target PDF: não certificam o renderer HTML.

O label após set falso continua órfão: atalhar o efeito de estilo não remove
a fronteira lexical de markup verificada no lote anterior. O efeito da
condição precede o argumento; ambos ocorrem uma única vez quando ativos.
Não há certificação universal de schemas de set, targets desconhecidos ou
atalhos específicos de show page/par.

## Pendência separada: valor e morfologia de CodeBlock

profiles-after conserva 6 diferenças de stdout nos cenários de
sucesso: Tekt retorna JSON numérico, enquanto o pin retorna Content/Styled.
Isso não foi contado como equivalência nem normalizado para um número.
codeblock-observables/results.json.gz, medido de 2026-10-14T06:08:57.693953+00:00 a
2026-10-14T06:09:03.997376+00:00, testa o observável da linguagem: type do bloco com set falso
é int no Tekt e content no pin; somar 1 ao resultado funciona no Tekt e
falha no pin. Blocos sem set são controles separados. O controle show sem seletor também
expõe uma rejeição independente no Tekt; não é prova de que esta correção
cubra show genérico. Logo a diferença não
é somente a serialização JSON ou uma estrutura Rust invisível.

A fonte pinada typst-eval/src/code.rs:36–54 transforma a cauda de set/show
em conteúdo. Tekt expression_dispatch.rs:91–211 junta valores e guarda
estilos apenas em subconjuntos específicos de Content. O L0 vigente desse
owner foi lido: C2/C2-R1 define preservação lexical para Table/Cell e exige
não estender cegamente a Flow/Math. A correção geral da cauda de CodeBlock
exige medição e atualização própria do L0, testes de tipos/join/flow/estilos
locais e preservação da identidade causal. Ela permanece aberta; nenhum
código desse owner foi alterado neste lote. A diferença observada é de
linguagem, mas a decisão arquitetural deve preservar as escolhas internas
legítimas do Tekt, sem copiar o mecanismo recursivo do vanilla.

## Validação

Suíte completa: 8437 passados, 0 falhas e
7 ignorados, entre 2026-10-14T06:07:49.476800+00:00 e 2026-10-14T06:13:23.170352+00:00.
workspace-pre.json.gz registra estado, fontes, ambiente /dev/shm, comando e
pin. workspace-result.json confirma fontes inalteradas durante a suíte.
Ignorados não são cobertura executada.

Build, check de todos os targets, rustdoc com -D warnings, fmt, diffcheck,
linhagem estrita V1/V5/V15/V26 e verificação reversa passaram. Clippy foi
inventário com cap-lints warn: 227 antes e
227 depois, delta {}. Arquitetura:
0 erros, 382
warnings e 1631 notas. O débito permanece;
exit zero não significa zero violações. As matrizes remotas de CI/release e
outras frentes da auditoria não foram certificadas por esta validação local.

## Evidência preservada

O arquivo evidence.json.gz inclui fontes, owners, scripts, logs, coletas,
PDFs, diff restrito e este relatório, com bytes base64 e SHA-256 verificados
após releitura. Nenhum commit, push, PR ou publicação foi realizado.
O objetivo geral de correções continua ativo.
