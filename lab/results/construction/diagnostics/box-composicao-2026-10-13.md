# Composição de Box e extensões do subframe

Caixas não fracionárias agora compõem o corpo em uma região isolada, com
inset dentro das dimensões explícitas, outset restrito à pintura e recorte
local. A implementação ainda não encerra a etapa A nem a geometria de todos
os containers: o caso pequeno inicial descrito abaixo permanece aberto.

## Proveniência e medições anteriores

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshot inicial em 2026-10-14T02:22:02.365038+00:00, com fontes, owners,
status e git diff HEAD --stat em before.json.gz. O estado final dos owners
está em pre-validation.json.gz; workspace-pre.json.gz contém estado,
hashes, ambiente e comando da suíte. Este relatório não atribui a este
lote todas as alterações anteriores do workspace.

As sondas anteriores estão em box-dimensoes-2026-10-13 e
containers-clip-2026-10-13, com estados e binários próprios. Elas mostraram
inset/outset alterando indevidamente o avanço de uma largura explícita,
corpo shape alterando a baseline externa, clipping horizontal ausente e
corpos perdidos ou transladados novamente. O contrato foi medido também
em container.rs:96-127, inline/box.rs:26-50 e flow/block.rs:255-295 da fonte
vanilla pinada. Não inferimos que toda diferença do Tekt seja um defeito.

## Código, contratos e testes

boxed.rs usa subframe geométrico para capturar corpo e cauda uma vez.
Agrupa fundo, corpo e máscara em coordenadas locais; acompanha o movimento
da linha externa, usa a baseline textual real ou a base da caixa e
restaura bbox/pendentes do pai. Fixups locais usam as dimensões conhecidas.
measurement.rs preserva dimensões explícitas totais na pré-medição.

mod.rs respeita dimensões finitas da sub-região mesmo dentro de uma página
automática. sub_frame.rs isola ascent/descent, acomoda a cauda e deixa de
acrescentar descender de fonte a grupos sem texto direto. Publica suas
extensões medidas em estado privado distinto do estado do pai; checkpoint
restaura esse resultado e measurement o consome. O novo núcleo
subframe-inline-extents é pinado pelos quatro owners, sem API pública nova.

Sete testes iniciais falharam antes do reparo. A primeira suíte core teve
6710 passes, 13 falhas e 5 ignorados: doze observadores exigiam folhas flat;
a falha real P1491 mostrou que a medição usava extensões antes vazadas pelo
subframe. O código foi corrigido mantendo o esperado de baseline 17pt.
Os observadores passaram a percorrer grupos; as expectativas geométricas
foram mantidas, exceto a antiga soma incorreta de inset à largura explícita.
Os antigos testes de clip sem assert agora verificam máscara e conteúdo.
core-first.log e core-second.log preservam ambos os resultados.

Dois testes adicionais cobrem stroke e pré-medição: RED→GREEN em
stroke-red.log/measurement-red.log/green-final.log. A inspeção posterior
refutou a primeira fórmula de raio: shapes.rs:1163-1178 define o raio
solicitado como externo. Para stroke uniforme, o fundo usa o raio central
(externo menos meia espessura) e a máscara usa o interno (externo menos
espessura inteira), com limites não negativos. radius-red.log prova a
falha do esperado independente 2pt para raio 4pt/stroke 2pt; os nove testes
de composição passaram após a correção em radius-green.log.

A primeira suíte completa deste lote teve 8370 passes, 4 falhas e 7
ignorados. Todas as falhas eram P1493: posição causal HTML y=21.1pt em vez
de 20pt. A origem legada instalava vertical_metrics, enquanto a cauda
usava text_edges como ascent presumido. O reparo registra o ascent realmente
instalado, preservando a exceção Raw e a origem geométrica. Os seis testes
infra P1493 passaram sem alterar seus valores esperados. A suíte falha,
suas fontes e resultados estão em radius-legacy-before.json.gz; não foram
apagados pelo novo resultado.

## Sondas finais de geometria

Binário Tekt debug: f9caf89704df31e2a2c4e34c08af67a3f9f0b8ddcd9396fabbebbf90a34d29e8.
Vanilla: pin 9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
As receitas, fontes, XML, PDFs e rasters estão preservados no arquivo de
evidências; cada rodada registra binários e UTC.

Os oito controles P+Box+Q de dimensão/inset/outset/clip mantêm a posição
do sufixo e coincidem pixel a pixel com os controles pinados a 144dpi.
Os quatro controles com stroke preservam áreas e bounds de tinta/borda;
rect e outset também coincidem integralmente no raster. Os dois casos
arredondados têm um pixel diferente cada, delta máximo de canal 68;
não alegamos identidade raster ou bytes PDF. As equações dos contornos e
as posições independentes dos testes sustentam a geometria do recorte.
final-pixel-comparison.json registra arquivos e hashes dos dois lados.

## Validação do estado final

Gates locais terminados em 2026-10-14T02:55:15.522926+00:00: check all-targets, build,
Rustdoc -D warnings, formato, linhagem estrita (V1/V5/V15/V26), linhagem
reversa e diff check passaram. Arquitetura: 0 erros,
382 avisos e 1620 notas. Isto não equivale a zero violações.
Clippy continua com 228 diagnósticos, sem delta deste ajuste;
o inventário cap-lints warn não aprova o gate CI -D warnings.

Suíte workspace offline/locked, todos os pacotes, serial, tmpfs e oráculo
comprovado: **8374 passaram, 0 falhas, 7 ignorados**,
concluída em 2026-10-14T03:01:23.511042+00:00. Fontes permaneceram inalteradas durante a
execução. Comando, ambiente e resultados completos estão nos recibos.

## Pendências verificadas

A etapa A continua aberta: box(width:8pt,height:6pt,clip:true) com shape
como primeiro conteúdo da página começa abaixo do topo útil. A 144dpi,
Tekt tem bounds [70,72,86,84], controle pinado [70,70,86,81]. A origem
inicial ainda adquire ascent de fonte antes de conhecer a baseline da
caixa. O controle movido e os dois controles de texto coincidem. Não
corrigir esse caso reduzindo o ascent de linhas que realmente contêm texto.
Dados finais em container-controls/final-renders.json; preservar também
os controles de texto adjacente e baseline causal ao implementar o reparo.

A etapa B mantém o caminho fracionário antigo explicitamente separado;
clip/inset/outset/fundo devem acompanhar sua alocação final. Etapa C:
Block continua perdendo corpos nos controles de clip; alinhamentos RTL e
auto-width, decoração e parent bbox precisam de medições próprias. Não
atribuir prontidão geral às caixas nem tratar o caminho fracionário como
resolvido. Conic, itens restantes da auditoria, Clippy e matriz remota de
CI/release seguem abertos. Não houve commit, push ou publicação.

Nos defeitos recentes confirmados predomina implementação que descumpre
ou materializa parcialmente o L0; também existem contratos que preservam
limites de rascunho. Não há inventário global classificado que permita
publicar uma proporção código versus prompts.

## Preservação

Snapshots .json.gz preservam estados intermediários. evidence.json.gz
contém logs, resultados, receitas, diffs e artefatos, cada um com SHA-256
verificado após descompressão. Relatórios anteriores conservam seus próprios
snapshots: contagens antigas não descrevem automaticamente o estado final.

Seguimento em 2026-10-14: a baseline da caixa pequena inicial e o controle
de texto menor foram corrigidos e revalidados em
[box-baseline-inicial-2026-10-14.md](../diagnosticos/box-baseline-inicial-2026-10-14.md).
A etapa B agora tem as sondas de box-fracionaria-2026-10-14.md; demais
limites permanecem abertos. Esta nota não altera as medições históricas.
