# Geometria e amostragem cônica no PDF

O exportador passa a respeitar centro autoral, ângulos físicos, paradas de cor e
limites laterais de offsets repetidos. A malha cobre o canvas transformado, em
vez de um círculo inscrito. Isso corrige cantos vazios sem copiar detalhes de
tesselação ou defeitos do vanilla. Não encerra a paridade geral de gradientes.

## Proveniência e decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz,
red-state, center-quantization-before, candidate-final e workspace-pre conservam
UTC, estado, git diff HEAD --stat, fontes/L0 e hashes. Os estados são distintos;
nenhum número deste relatório é atribuído ao commit limpo.

Antes, conic.rs fixava centro/raio e associava cores CMYK por índice circular;
resources.rs fixava Decode unitário e coordenadas de 8 bits. As sondas do lote
conic-posicionamento provaram cantos brancos, centro e offsets sem efeito.
Os owners foram atualizados antes dos testes e da implementação. O núcleo
pdf-conic-mesh compartilha apenas invariantes entre os owners de coleta,
codificação e recursos, com pins completos e ownership 1:1 mantido.

A direção foi medida contra fonte e binário do vanilla pinado
9f2b6e8715237cb086899a42873660fe744622e8, árvore
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489, SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Retificação: nos controles deste lote, o PDF vanilla contém gradiente variável;
a afirmação anterior de fallback uniforme não pode ser generalizada.
A possibilidade de coordenadas de 16 bits e Decode independente é definida na
[PDF Reference 1.3, tabela 4.31](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/pdfreference1.3.pdf).

## Implementação e testes

O coletor transforma os cantos do canvas para o espaço do Pattern e transporta
o tamanho físico da referência. O emissor calcula centro Y-up e raio suficiente
para esse domínio. Emite malha e Decode conjuntamente; os recursos usam exatamente
esses limites. O raio tem margem de quantização, e o centro corresponde ao
código 32768 em vez de cair entre dois códigos. A geometria usa f64/libm.

Intervalos vêm de effective_offsets, com extensão constante e descontinuidades
preservadas. Cada intervalo reparametriza duas paradas no interpolador L1;
CMYK mantém quatro componentes e spot um tint. O limite angular e a subdivisão
adaptativa limitam aproximações sem prometer erro universal. Alpha e domínios
singulares/não finitos não são certificados por este lote. Helpers históricos
ficaram apenas em testes; não há emissor produtivo antigo paralelo.

Seis testes funcionais tiveram RED→GREEN para centro, raio, orientação, mistura
CMYK, offset explícito e offset repetido. Um sétimo teve RED→GREEN para o centro
representável. O oitavo, cobertura fora da caixa nominal, foi acrescentado após
a implementação: é regressão adicional, não prova RED. red-fixture-decoder.log
é erro inicial do decoder de teste; candidate-import-errors.log é erro de imports;
nenhum deles é apresentado como falha funcional do produto.

## Raster e limites preservados

O corpus verificado foi medido de 2026-10-14T08:34:36.446296+00:00 a 2026-10-14T08:34:38.415361+00:00,
binário Tekt SHA-256 e2e9732d450feae6a9fc06c600832c2662a188905a921640d21cb664093431a9.
A recoleta selada e sua equivalência de pixels estão em sealed-render-verification.json.
Todos os PDFs verificados passaram no qpdf; isso só comprova validade estrutural.

Comparação Tekt sólido versus cônico constante, Poppler a 144 dpi, canvas inteiro:

| Caso | Pixels pintados do sólido | Brancos cônicos nessa área | Diferenças integrais >16 |
|---|---:|---:|---:|
| stroke | 9216 | 0 | 0 |
| rotated-stroke | 4362 | 147 | 426 |
| spot | 24576 | 0 | 0 |
| cmyk | 24576 | 0 | 24576 |
| page | 24576 | 0 | 0 |
| offset | 7072 | 0 | 0 |
| center | 24576 | 0 | 0 |
| rotate | 4104 | 77 | 215 |
| parent | 4000 | 0 | 0 |

stroke e rotated-stroke desta tabela usam paint + espessura, que realmente
chega ao exportador como gradiente. No MuPDF, as imagens integrais de sólido e
cônico são idênticas nos dois casos rotacionados. No Poppler, erosão de 5x5 da
máscara sólida identifica interior sem diferenças >16; essa classificação
não substitui nem apaga as diferenças integrais. Streams e arquivos de ambos
os renderizadores estão em additive-boundary-check. Isso sustenta uma diferença
de rasterização de borda nesses controles, não uma certificação universal.

Os controles iniciais de stroke via dicionário renderizaram preto de 1pt em ambos
os lados: são inválidos para provar cobertura de gradiente. rect.rs:40-43 descarta
Value::Dict e aciona fallback. O constructor stroke(paint: gradient, ...) também
rejeita o gradiente; sua mensagem e proveniência estão em stroke-cmyk-open-evidence.
A sonda adicional do binário vanilla pinado aceita os dicionários e a forma
stroke(paint: gradient, thickness: ...). Portanto ambas são lacunas confirmadas
desta superfície Tekt e exigem correção própria; não são só escolhas diferentes
de sintaxe. Os comandos e saídas estão no mesmo recibo.

CMYK sólido continua emitindo RGB convertido, enquanto o gradiente usa componentes
CMYK nativos: os dois renderizadores confirmam divergência de cor. Não alterar a
amostra nativa apenas para igualar o caminho sólido; a política de cor precisa de
correção causal. Não há buracos brancos no controle CMYK.

O corpus bilateral conserva diferenças integrais, inclusive na costura e perto do
centro. final-raster-metrics.json registra 322/322/107/68 pixels >16 nos controles
square/wide/angle/center do candidato verificado. A perturbação qpdf center-exact
reduziu um controle diagnóstico, mas esse ganho NÃO se reproduziu integralmente
no emissor final; não se reivindica essa redução. O teste prova a representação
do centro, não identidade de raster. A linha branca no topo de parte do vanilla
não foi copiada. A posição das cores nos eixos fora da costura responde aos
ângulos e ao centro; paridade visual completa permanece sem comprovação.

## Snapshot, formatação e linhagem

Somente o snapshot 06 foi regenerado, de 5350 para 13477
bytes. Duas gerações idênticas; qpdf passou, raster foi inspecionado. Apenas o
Shading 109 mudou; Matrix, stream de pintura e demais objetos ficaram iguais.
Os outros oito PDFs e fontes foram preservados. SHA-256 novo:
554bea2d6c8e70e786d74b820fe08314c19ed124c50efa771b0a69e16e90c835. MANIFEST atualizado e corpus passou sem UPDATE.

Uma invocação de rustfmt com edição 2024 reformatou módulos vizinhos. O resselo
isolado foi bloqueado antes de copiar alterações. A recuperação usou arquivos
preservados e hashes da suíte anterior; format-recovery-verified comprova bytes
idênticos em todos os fontes Rust fora do conjunto autorizado. O formato final
usa edição 2021 e skip_children. Não houve descarte de alterações preexistentes.

A primeira validação completa terminou com uma falha dependente de relógio e
um V16 novo no fallback de contagem de componentes. parent-failure-comparison.json
prova que somente as datas Info/XMP diferiam. Os quatro controles de coordenadas
parent/self de Block/Boxed passaram a comparar coordenadas numéricas,
sem depender de datas globais; o helper rejeita ausência ou multiplicidade do
Shading e Coords. Page verifica o valor medido; os demais casos conservam as relações originais
de igualdade/diferença, portanto não podem passar por uma diferença de metadata.
O primeiro ensaio acrescentou Y-up presumido para Block/Boxed e falhou; esses
valores não tinham sido medidos. A suposição foi retirada, sem fixar os valores
atuais como correto posicionamento físico. A orientação física de parent linear
permanece aberta; parent-tests-green.log conserva o ensaio incorreto e
parent-tests-final.log a verificação das relações originalmente requeridas. Após a suíte terminar, o L0 e o match passaram a enumerar todos os
espaços atuais. Primeira rodada preservada em first-validation; todas as
verificações e a suíte foram repetidas no candidato final.

## Verificação final e continuidade

Suíte de 2026-10-14T08:44:34.059485+00:00 a 2026-10-14T08:49:57.536231+00:00: 8468 passaram,
0 falharam, 7 ignorados. Fontes estáveis durante a execução;
ambiente /dev/shm, vanilla pinado e comandos estão em workspace-pre/result.
Build, check all-targets, rustdoc -D warnings, fmt, diffcheck, linhagem estrita
V1/V5/V15/V26 e reversa passaram. Clippy foi inventário cap-lints warn:
182 antes, 181 depois. Arquitetura:
0 erros, 382 warnings,
1632 notas. Não equivale a Clippy -D warnings ou ao
objetivo global zero violations. A nota adicional V19 registra que sete
espaços explícitos compartilham o braço RGB; não é erro de ownership e não
certifica cobertura individual de todos os espaços.

Continuam abertos Raw/presença e seu gate público, lacunas de stroke e cor,
diferenças cônicas descritas, demais frentes funcionais e CI/release remoto.
Nenhum commit, push ou publicação. evidence.json.gz preserva receitas, fontes,
owners, núcleo, resultados e artefatos com base64 e SHA-256 verificados após releitura.
