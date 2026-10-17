# Transferência de buffers de fontes no PDF — melhoria aceita

## Resultado

No caso de 1.600 células com pesos alternando entre 800 valores, o Tekt
anterior e o final preservam o mesmo conteúdo/saída. A versão final poupa
**240,59 MiB de bytes solicitados ao alocador**, reduz o pico de RSS de
**512,75 para 286,09 MiB** (−44,20% pareado) e o tempo de **938,32 para
792,07 ms** (−15,85% pareado). São medidas desta janela, não uma promessa
para todo documento. O pico de RSS não é memória retida após export/watch.

Único consumer produtivo alterado nesta rodada: `03_infra/src/pipeline/pdf.rs`.
L0 proprietário: `00_nucleo/prompts/infra/pipeline/pdf.md`. Hash do código
`3a663c49`, @prompt-hash `b2eae7f9`; resselo restrito ao owner, sem novos
contratos públicos, flags, fases, núcleos ou representação de Content.

## Causa medida antes da decisão

O pipeline em `03_infra/src/pipeline/pdf.rs:436-437` passava uma referência
temporária da tabela owned retornada por resolve_fonts ao binder e chamava
into_owned. `03_infra/src/export/font_binding.rs:127-148` devolve Cow::Borrowed
quando a tabela já inclui as faces necessárias. Into_owned nesse caminho
clona cada chave e Vec<u8>, embora a tabela original possa ser transferida.

O helper privado `bind_owned_pdf_fonts` conserva a tabela até o resultado do
binding: no caso plural Borrowed, transfere a original; no caso Owned, usa
a tabela completa produzida pelo binder. Fontes ausentes continuam sendo
anexadas com seus bytes efetivos. Chaves, ordem, variantes, eixos, seleção,
fontTools, fallback e export normal/oráculo permanecem no mesmo percurso.

Tabelas vazias/unitárias usam o caminho de alocação anterior. A primeira
versão transferia qualquer tabela e mostrou +2,84% RSS local em uma janela
com nomes diferentes de executável. A origem não foi atribuída naquele
momento. O limite plural foi adotado como medida conservadora, dado que o
caso unitário poupava apenas três pedidos e cerca de 0,32 MiB solicitados.
Ele depende apenas da cardinalidade da tabela de recursos, sem reconhecer
fixtures, famílias, pesos ou texto. O controle posterior com argv homogêneo
não apresentou regressão de RSS. Não há prova causal de que a antiga
oscilação viesse do prefixo lexical ou somente da transferência de buffers.

## Proveniência e método

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`pdf-font-buffer-reuse-2026-10-10/before.json.gz` conserva arquivos anteriores,
UTC, lista de alterações e diffstat. `candidate.json.gz`, `owner-diff.patch`,
`seal.json` e `final-validation.json.gz` conservam a implementação final,
hashes, estado completo e logs. Comparação das 966 fontes produtivas com a
rodada anterior confirmou que os dois owners de font_metrics já existentes
continuam idênticos ao snapshot aceito; somente pipeline/pdf.rs mudou aqui.
StyleChain e seu L0 foram restaurados exatamente ao snapshot anterior.

- Tekt anterior SHA-256: `783572ba02f8b423935179dd7c81f62895bcf21db9144d48d2fa924421249ef0`.
- Tekt final SHA-256: `5e66c09d44e2b59545c98ce85dc8fb6aedb1df165d4641a07c1bfb100abb358a`.
- Vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; receita/árvore ratificadas no diagnóstico P1489.

A decisão final usa `measurements-equal-argv.json.gz` e
`allocation-equal-{red,green}.json.gz`: ambas as cópias Tekt são chamadas
`typst` em caminhos de mesmo comprimento, com identidade SHA comprovada.
Há três rodadas de quatro pares por fixture, warmups excluídos, ordem
embaralhada, CPU fixa, Rayon=1, stack=64 MiB, fontes iguais e
SOURCE_DATE_EPOCH=0. Os receipts conservam horários UTC exatos, argv,
environment selecionado, hashes de fixtures/fontes/scripts e estatísticas.
A padronização de argv eliminou uma diferença constante de bytes solicitados
nos controles; nome do executável é uma variável relevante para esse ensaio.
As janelas anteriores e seus drivers exatos permanecem arquivados.

## Medições finais

Os tempos/RSS são medianas; os percentuais são medianas dos quocientes
pareados, portanto não precisam coincidir com o quociente das medianas.

| Caso | Tempo anterior → final (ms) | Tempo pareado | Pico RSS anterior → final (MiB) | RSS pareado |
|---|---:|---:|---:|---:|
| local-set-400 | 119.89 → 117.92 | +0.08% | 30.26 → 30.31 | +0.23% |
| local-set-1600 | 160.78 → 161.04 | -0.04% | 41.47 → 41.20 | -0.45% |
| no-fill-1600 | 145.03 → 144.21 | -0.16% | 37.26 → 37.19 | -0.26% |
| inline-set-1600 | 186.58 → 183.97 | -2.15% | 46.64 → 46.57 | -0.17% |
| variants-1600 | 938.32 → 792.07 | -15.85% | 512.75 → 286.09 | -44.20% |
| families-1600 | 162.48 → 161.64 | -1.32% | 37.59 → 37.55 | +0.14% |

No caso de variantes, as rodadas de tempo foram −15,02%, −15,69% e −16,90%.
Pedidos de alocação: 1.134.037→1.132.436 (1.601 a menos). Bytes solicitados:
1.814.950.909→1.562.669.617; a diferença é volume acumulado, não RSS.
Nos cinco controles unitários, contagem e bytes solicitados são exatamente
iguais entre os binários com argv padronizado. As oscilações pequenas de
tempo nesses controles não certificam aceleração por este reparo.

## Verificação

- RED original: o helper com o comportamento antigo copiava o buffer completo; o teste de transferência falhou. O controle de face faltante já passava. Log real e reconstrução identificada estão em final-validation.json.gz/red-source.json.gz.
- O gate do protótipo geral falhou no novo critério de conservar as contagens unitárias; `all-tables-plural-gate-red.json.gz` registra essa falha antes de introduzir o limite. Os gates finais passaram com contagens unitárias iguais e redução plural.
- Testes atuais verificam transferência dos dois buffers numa tabela plural e preservação/anexação de uma face shaped ausente. A identidade de alocação é trava de custo interno; não é critério de paridade da linguagem.
- Núcleo restaurado: 6.680 aprovados, cinco ignorados. Infra final: 1.275 aprovados, dois ignorados, com SOURCE_DATE_EPOCH=0 e execução serial. Logs integrais em final-validation.json.gz.
- Uma execução anterior da infra teve falha em p273_5_relative_self_preserva_p272_p273_bit_exact. Os dois PDFs diferiam somente em quatro bytes de CreationDate/ModDate e equivalentes XMP, separados por um segundo. O teste chama export_pdf diretamente, passou no replay isolado e a suíte inteira passou com epoch fixo. Nenhum código/teste de gradiente foi alterado.
- 36 documentos antes/final preservaram texto, palavras, contagem/tamanho de páginas, caixas (delta zero na extração) e raster a 36 dpi. Incluem tabelas, context, source/include, espaços, counter/query, fontes reais, matemática/RTL pequenos e eixos VF. Seis sondas query(table).len retornaram 1 nos três motores.
- cargo build e cargo build --release passaram. crystalline-lint retornou zero, sem V5 no owner alterado e sem V15/V26. Permanecem avisos anteriores em outros módulos; não foi executado fix-hashes global.

## Experimento descartado e frentes abertas

O experimento de captura direta do prefixo lexical poupou 4.800 pedidos no
inline-set, mas não mostrou ganho temporal robusto. Foi revertido; ver
style-prefix-reuse-2026-10-10.md. Seus primeiros resultados de RSS tinham
nomes distintos de executável e não estabelecem uma causa exclusiva.

Restam: reconstrução de TextStyle nos escopos locais; cadeia primária repetida
do shaper; buffers originais repetidos por FontKey e outros recursos dentro
dos 286 MiB restantes. A representação pública das fontes não foi mudada.
As divergências conhecidas de raster com vanilla e paginação da matemática
repetida continuam independentes. A comparação principal desta melhoria é
Tekt anterior/final com saída preservada; os tempos de variantes contra
vanilla não certificam equivalência global de linguagem desse caso.


### Continuação: cadeia primária do shaper

O passo seguinte reduziu as buscas de 9.619 para 19 e cerca de 9% do tempo
das tabelas grandes, mantendo a melhoria de buffers PDF. Ver
[shaper-primary-reuse-2026-10-10.md](shaper-primary-reuse-2026-10-10.md).
