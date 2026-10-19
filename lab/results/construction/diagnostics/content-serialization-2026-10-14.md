# Serialização de conteúdo vazio e wrappers transparentes

A CLI agora publica Empty como sequence com children vazio e conserva text e
lang disponível de Raw. O encoder da stdlib atravessa Source, ShowRecipe,
Located e Realized antes de projetar os campos do conteúdo. Snapshots presentes,
inclusive vazios, continuam autoritativos. A presença lexical de block e
lang:none em Raw permanece pendente de mudança de contrato público.

## Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. A coleta before ocorreu
entre 2026-10-14T06:45:03.913126+00:00 e 2026-10-14T06:45:34.274417+00:00. O arquivo before/results.json.gz conserva
status, git diff HEAD --stat, texto e hashes das fontes/owners, UTC, argv,
saídas e identidade dos executáveis. Vanilla pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

No baseline a CLI já retirava os wrappers, mas seu match não projetava Empty
ou Raw. O encoder tinha esses braços estruturais, porém Source impedia chegar
a eles: sequência perdia children, Styled perdia child/styles e Raw perdia
text/lang. A correção trata as causas distintas. Aceitação usa a morfologia e
os valores da linguagem; não exige copiar estrutura Rust, recursão do upstream
ou uma grafia específica de YAML quando o valor é o mesmo.

## L0 e implementação

Os owners shell/cli e compiler/stdlib/loading/encoding foram atualizados antes
dos testes e da implementação. O segundo passa a pinar explicitamente o núcleo
content/source-origin. A projeção dos wrappers não executa callbacks, não cria
Location e não substitui um snapshot Some pelo fallback estrutural. O nome de
Empty é sequence também quando o snapshot vazio proíbe sintetizar children.
Raw publica apenas a informação já disponível: text e lang Some. Não se infere
presença de block a partir do bool nem presença de lang a partir de None.

Oito testes novos reproduziram as falhas: quatro no core e quatro na shell.
Depois do reparo, todos passaram. O primeiro ensaio nem compilou porque o
fixture shell chamava um helper pub(crate) do core; foi corrigido para construir
o SourceContent público. red-compile.log é erro do fixture, não prova RED da
funcionalidade. red.log é o RED executável, green.log é o GREEN. red-state
preserva o fixture anterior à correção; não é apresentado como snapshot exato
do RED executável. Os scripts, o log RED e os snapshots posteriores preservam
a sequência disponível sem inventar uma captura ausente.

O primeiro resselo foi bloqueado por PARSE em &raw.lang, apesar do rustc aceitar
o código. Renomear a variável para raw_elem removeu essa limitação do parser.
first-validation conserva o bloqueio e os checks já realizados. Uma formatação
posterior ao resselo alterou bytes e foi detectada pelo controle reverso;
second-validation preserva essa falha. Após a suíte terminar, somente o Hash
do Código no owner da CLI foi corrigido, com prova metadata-only-reseal. Os
bytes produtivos testados não mudaram; os gates de linhagem foram repetidos.
Não se alterou
o pin do linter de CI nem se relaxaram V1/V5/V15/V26. Resselo restrito aos dois
consumers e seus owners.

A primeira validação completa passou nos 8.455 testes, mas introduziu um V16
no wildcard de saída do loop de wrappers. first-complete-validation preserva
essa rodada. A saída agora retorna explicitamente a variante final, sem a
descartar. Após esse ajuste foram repetidos a suíte e os controles finais
relatados abaixo, recuperando o inventário arquitetural anterior.

## Recoleta e limites

after foi medido entre 2026-10-14T07:11:03.567402+00:00 e 2026-10-14T07:11:40.837261+00:00, Tekt SHA-256
992779989361adf927185bf6423d7e8279dac376cac4bd3178de5d0631cfdb7a. Fontes permaneceram estáveis durante a coleta.
O corpus tem 22 expressões em 52 combinações de caso/formato. Antes, seis
combinações coincidiam em exit/stdout/stderr; depois, 21. Interpretando os
valores JSON/YAML válidos, sete coincidiam antes e 24 depois. Os critérios são
separados em acceptance.json, e todos os bytes originais são preservados.

A diferença YAML de aspas em styles(..) e linha vazia final não foi tratada
como defeito de linguagem. As 28 combinações restantes concentram-se em Raw:
block omitido/false/true, lang omitido/none, métodos de campos, markup e
apresentação Bool/Auto, além das aceitações legadas raw() e block:none.
Não há certificação de paridade integral de Raw ou de todos os elementos.

raw-controls acrescenta 17 controles bilaterais de fields/has/at, igualdade,
argumentos e show. raw("x") e raw("x",block:false) comparam iguais no Tekt,
mas diferentes no pin; lang:none também se perde. A diferença é observável da
linguagem, não só do serializer. Por isso não foi remendada apenas no JSON.
A coleta usou o binário anterior identificado em cada registro; não representa
um build das fontes de serialização já editadas naquele momento. Raw não foi
alterado. A morfologia de show raw e os casts/diagnósticos do constructor ainda
exigem decisão própria; não se presume que toda divergência de Tekt seja erro.

## Proposta pública separada

00_nucleo/materialization/raw-presenca-2026-10-14/review.md contém proposta
inativa de set_fields:u8 em RawElem, duas constantes e um núcleo compartilhado.
O manifesto e os candidatos têm hashes verificados. Isso distingue omissão de
false/none sem mudar a fase do pipeline ou inferir presença pelo render.
A mudança quebra construção Rust externa por struct literal, portanto aguarda
confirmação explícita segundo ADR-0127 §2.1/§2.4. Nenhum campo público, L0
canônico Raw ou núcleo candidato foi instalado neste lote. O gate não inclui
RawLine ou alteração das aceitações legadas do constructor. Os hashes de base
do manifesto são o estado de preparação, anterior ao resselo final da CLI e
do encoder; a proposta e seus candidatos permanecem intactos.

## Validação final

Suíte completa: 8455 passados, 0 falhas e 7
ignorados, entre 2026-10-14T07:10:02.923782+00:00 e 2026-10-14T07:15:49.622079+00:00. workspace-pre/result conservam
estado exato, hashes, comando e ambiente /dev/shm. Build, check all-targets,
rustdoc -D warnings, fmt, diffcheck, linhagem estrita e reversa passaram.
Clippy é inventário cap-lints warn, não aprovação do gate -D warnings:
227 antes e 227 depois, delta {}.
Arquitetura: 0 erros,
382 warnings e 1631
notas. Isso não é zero violações nem certificação de produção.

O objetivo amplo segue aberto, incluindo labels vazios/repetidos, continuação
e Box fracionária, pendências de layout, avisos e matrizes remotas CI/release.
Nenhum commit, push, PR ou publicação foi feito. evidence.json.gz preserva
fontes, owners, scripts, sondas, logs e relatórios com base64/SHA-256 verificado
após releitura. A proposta pública fica separada do L0 vigente.
