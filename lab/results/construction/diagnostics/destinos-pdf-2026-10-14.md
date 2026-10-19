# Navegação interna do PDF resolve destinos nomeados

O exportador agora emite destinos que o leitor PDF consegue resolver.
Árvore de nomes e ações GoTo usam a mesma string UTF-16BE, com preservação
de Unicode e delimitadores. Destinos de páginas excluídas são omitidos e
os retidos apontam para o índice exportado correto. A associação de labels
criados por expressão ainda falha em eval, antes da exportação, e segue aberta.

## Evidência anterior e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshot inicial 2026-10-14T05:08:33.116318+00:00, em before.json.gz, com fontes, L0, status e
git diff HEAD --stat. red-state.json.gz conserva testes e contrato antes
da implementação; candidate.json.gz e pre-validation.json.gz conservam o
candidato e o estado ressellado. Não atribuir a este lote a árvore suja inteira.

labels-fronteira-2026-10-14/reader-probes.json.gz reproduziu o defeito:
MuPDF 1.23.10 reconhecia as ações internas, mas resolveLink retornava -1.
No vanilla pinado retornava 0, primeira página. O binário anterior também
foi usado em before-probes e before-markup-probes; seu SHA-256 identifica
o build anterior mesmo quando a árvore já continha novos testes ou L0.
As sondas não são atribuídas a um build limpo do HEAD.

navigation.rs misturava duas estruturas: colocava um dicionário de destinos
por nomes dentro de Catalog.Names.Dests. Esse local exige uma árvore de
nomes com chaves de string; referências à árvore também usam strings. O
formato de dicionário por nomes pertence diretamente a Catalog.Dests.
Referência normativa: [ISO 32000-1, seções 7.9.6 e 12.3.2.3](https://developer.adobe.com/document-services/docs/assets/35e4369068f86065372c18787171a17e/PDF_ISO_32000-1.pdf).
A falha era observável no leitor, não uma divergência de bytes a copiar.

## Contrato e implementação

O owner de navigation foi atualizado antes do código. Catalog.Names.Dests
passou a apontar para uma folha com /Names e pares string/destino. As chaves
e os operandos GoTo usam o serializer UTF-16BE com BOM já existente. Não
há nomes PDF para essas referências, nem delimitadores de label interpretados
como sintaxe PDF. A ordenação usa unidades UTF-16, equivalentes à ordem dos
bytes big-endian com BOM comum, preservando também a ordem entre prefixos.

Mantiveram-se conversão y-up, seleção de páginas, remapeamento, omissão de
destinos excluídos, fallback legado de página inexistente, alocação de IDs
e merge de EmbeddedFiles no dicionário Names. Captura de labels e contadores
não foram alteradas neste reparo. Não houve nova API, default ou fase;
correção interna contínua segundo ADR-0127.

Os três testes novos falharam antes e passaram depois: topologia e ordem
binária das chaves, identidade entre destino/ação com Unicode e barra,
e exclusão/remapeamento de páginas. Os quatro testes agregadores existentes
mantiveram nomes, labels, exigência GoTo e verificação y-up; suas asserções
passaram a observar a string e o array da árvore em vez dos tokens inválidos
/sec1 e /D /sec1. Atualizou-se o L0 de testes antes dessa mudança.

## Verificação por leitor independente

Sondas CLI de 2026-10-14T05:13:27.388088+00:00 a 2026-10-14T05:13:38.283624+00:00; Tekt SHA-256
9867b8fbb83eed6a2eef2402ce7dc468339414ad4be4f1945234475200387549. Vanilla SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Fontes, comandos, PDFs, qpdf JSON, estado e hashes estão em after-probes.
A [API MuPDF 1.23](https://mupdf.readthedocs.io/_/downloads/en/1.23.0/pdf/)
foi usada para obter os links e resolver seus destinos, sem abrir URLs.

| Caso válido do CLI | Antes | Depois |
|---|---|---|
| Label literal ASCII | Destino não resolvido | Primeira página |
| Seleção das páginas 2 e 3 | Destino retido não resolvido | Primeira página exportada |
| Só página de referências, sem alvos | Nenhuma anotação para os alvos excluídos | Preservado |

reference-probes recompila os casos anteriores page-ref, logical-ref e
label-ref. Os três links são resolvidos para a primeira página; o texto
continua page 1 e R7 quando pertinente. Hashes, receitas e saídas completas
estão em reference-probes/results.json.gz.

A prova adicional pela API pública de exportação está em api-probe.json.gz.
O harness Rust externo ao produto foi ligado às bibliotecas já construídas;
sua fonte, comando e hashes das bibliotecas foram registrados. Ele exporta
labels com prefixos a/aa, acento, barra, emoji, caractere fora do ASCII no
BMP, hash, parênteses, espaço e quebra de linha. Os oito links do PDF são
resolvidos pelo MuPDF para a primeira página. qpdf também lê o arquivo sem
erro. A entrada e as bibliotecas não mudaram durante a prova. Essa medição
isola o exportador da falha de associação de labels descrita abaixo.

## Falha independente em labels criados por expressão

`#let target = label("alvo")` seguido de `#heading[Alvo]#target` é aceito
pelos dois binários. No Tekt, o label aparece como texto `<alvo>` e a consulta
retorna COUNT=0; no vanilla o label não vira texto e COUNT=1. Acrescentar
ref(target) causa erro de label inexistente no Tekt. A reprodução mínima,
sem referência que aborte a compilação, está em dynamic-label-probes, com
fontes/owner de markup_stream e proveniência completa.

markup_stream.rs só aplica associação retroativa no ramo SyntaxKind::Label.
A expressão avaliada como Value::Label segue para value_to_display_content,
onde cai no fallback de repr. Essa é a hipótese causal concreta para o
próximo RED unitário; não exige mudar o exportador novamente.

As tentativas de escrever barra, parênteses, emoji ou hash diretamente na
sintaxe curta de label foram rejeitadas pelos dois binários com unclosed
label. Elas são controles de sintaxe inválida, não falhas exclusivas do Tekt.
A classificação completa está em probe-classification.json. Não declarar
suporte de ponta a ponta aos labels por expressão usando só a prova da API.

## Validação e continuidade

A primeira suíte workspace serial em Linux terminou com 8417
aprovações, 2 falhas e 7 ignorados, de 2026-10-14T05:12:36.351946+00:00
a 2026-10-14T05:18:37.198104+00:00. As únicas falhas foram os snapshots 02-markup-heading e
07-multi-feature, que ainda exigiam os bytes da estrutura inválida anterior.
Todos os outros alvos passaram, incluindo CLI, seleção de páginas e anexos.
TMPDIR em /dev/shm, oráculo pinado e linter publicado no PATH; fontes
inalterados durante a execução, conforme workspace-pre.json.gz.

Após atualizar primeiro o L0 do corpus, somente essas duas referências foram
regeneradas pelo harness Compact/Disabled/epoch fixo, duas vezes. As gerações
foram idênticas. qpdf aprovou os arquivos; texto, rasterização e todos os
objetos PDF fora da árvore de destinos ficaram idênticos. Cada PDF alterou
somente um objeto: obj:9 no 02 e obj:175 no 07. Os outros sete snapshots
mantiveram seus hashes. As imagens foram inspecionadas; a pintura anterior
foi preservada. MANIFEST registra os novos hashes e tamanhos. Evidências em
snapshots/before.json.gz, update-results.json e comparison.json.

A suíte inteira de infraestrutura foi então repetida sem UPDATE:
1292 passaram, 0 falharam, 2 ignorados,
de 2026-10-14T05:23:05.790638+00:00 a 2026-10-14T05:23:52.264488+00:00, incluindo os nove snapshots. Nenhum código
produtivo mudou após a suíte workspace; além das referências e do MANIFEST,
mudaram apenas o L0 do corpus e o header de linhagem do harness. Os checks
de linhagem, arquitetura e diff também foram repetidos após essa manutenção.

Build, check de todos os alvos, Rustdoc com -D warnings, formatação,
linhagem estrita V1/V5/V15/V26, linhagem reversa e git diff --check passaram.
Clippy de inventário passou de 228 para 227
avisos: desapareceu um useless_format na asserção de destino atualizada.
Não é aprovação de -D warnings. Lint arquitetural: 0
erros, 382 warnings e
1631 notas. Zero violations continua pendente.

Próximos reparos: associação de labels por expressão, prioridade de links
aninhados, corpos rotulados multipágina e os demais itens da auditoria,
incluindo Box fracionária. O objetivo geral permanece aberto. Não houve
commit, push ou publicação. Evidências e receitas estão em evidence.json.gz,
com SHA-256 verificado após releitura e descompressão.
