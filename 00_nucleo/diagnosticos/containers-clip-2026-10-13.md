# Composição de containers com clip — diagnóstico aberto

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
UTC `2026-10-14T01:37:58.262282+00:00`. before.json.gz preserva fontes/L0 dos
produtores, status, diffstat e hashes dos binários. before-renders.json
contém comandos, páginas, imagens e hashes, entre 2026-10-14T01:38:15.259941+00:00 e
2026-10-14T01:38:23.205439+00:00. Binários Tekt `ddb376464df76ebe6052072a824de3ffffb5782805a526316665c851d1db09e5` e vanilla
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; este último é o build comprovado do pin
9f2b6e8715237cb086899a42873660fe744622e8. Receitas e oito fontes completas
estão no arquivo evidence.json.gz, juntamente com PDFs e rasters a 144 dpi.

## Medição

Cada fonte declara largura 8pt, altura 6pt, clip:true, corpo rect de
14×10pt ou texto ABCD, em página 120×120pt com margem 35pt; metade aplica
move de (7,9)pt. Pixel pintado significa mínimo RGB <240, no canvas
inteiro de cada página, sem recorte/alinhamento. A contagem registra a
falha de cobertura; não é critério geral de paridade nem prova da correção
de todo o raster vanilla.

| Caso | Binário | Páginas | Pixels pintados |
|---|---|---:|---:|
| block-shape | tekt | 2 | 0 |
| block-shape | vanilla | 1 | 204 |
| block-shape-moved | tekt | 2 | 0 |
| block-shape-moved | vanilla | 1 | 204 |
| block-text | tekt | 1 | 0 |
| block-text | vanilla | 1 | 61 |
| block-text-moved | tekt | 1 | 0 |
| block-text-moved | vanilla | 1 | 61 |
| box-shape | tekt | 1 | 560 |
| box-shape | vanilla | 1 | 204 |
| box-shape-moved | tekt | 1 | 560 |
| box-shape-moved | vanilla | 1 | 204 |
| box-text | tekt | 1 | 0 |
| box-text | vanilla | 1 | 61 |
| box-text-moved | tekt | 1 | 32 |
| box-text-moved | vanilla | 1 | 61 |

## Causas no código e limites da inferência

- block.rs:243-295 executa o corpo no estado corrente, extrai somente
  current_items da região final e os envolve sem converter suas posições
  absolutas para coordenadas locais. Dimensões são lidas após o corpo,
  podendo pertencer a outra região; a altura natural substitui a declarada.
- boxed.rs:151-206 usa layout_sub_frame_inline, que devolve apenas a cauda
  de current_line. Formas já comitadas em current_items permanecem fora do
  Group de clip. O fallback ainda soma body_origin_x às posições dessas
  formas. Em altura explícita, o clip é condicionado somente à altura
  excedente, usa a largura do corpo e não localiza o Group extraído.
- O helper de subframe realmente isolado também precisava de reparo:
  descendentes podiam criar páginas externas por overflow ordinário. O
  diagnóstico subframe-isolamento-2026-10-13 contém a regressão direta
  e a correção dessa base; isso sozinho não reúne os corpos de block/box.

O L0 já atribui geometria, dimensões, clipping e origem aos respectivos
produtores. Algumas cláusulas históricas mandam preservar emissão atual;
precisam de atualização específica antes de alterar a composição. A fonte
upstream consultada dispõe o body em frames, aplica dimensões e só então
clipa cada fragmento. Isso explica uma alternativa de implementação; não
legitima copiar sua mecânica nem declara correto qualquer detalhe do
vanilla. O contrato independente é que clip restringe a pintura à região
especificada sem perder a posição do container ou criar páginas externas
por mero overflow de um frame isolado.

## Trabalho restante

Compor o corpo em coordenadas locais e recolher todos os itens uma vez;
resolver dimensões e aplicar clip/radius na caixa correta; preservar a
baseline inline, prefixos e frações/repeats, inset/outset, preenchimento,
traço, parent bbox, decoração, alinhamentos diferidos e fluxo posterior.
Block paginado requer tratamento de cada fragmento, sem drenar a região
errada ou substituir todos os casos por um único frame infinito. Casos
com página explícita, múltiplas regiões, conteúdo contextual e alinhamento
vertical precisam de regressões. Os oito controles acima permanecem
abertos até nova medição, não foram aceitos por serem diferenças históricas.

## Repetição após o reparo do isolamento

Entre 2026-10-14T01:50:19.918662+00:00 e 2026-10-14T01:50:28.017888+00:00, binário Tekt
`330ed7e0e258fa51a656937e625452ee03784366dad6ddb99a47af10d867ff1c`, foi repetido o corpus sem editar as fontes.
block-shape-moved passou de duas páginas para uma, ainda sem pintura.
Os demais resultados de páginas/pintura permaneceram iguais. Todos os
controles continuam abertos. O estado fonte/L0 está no snapshot
pre-validation.json.gz do lote subframe-isolamento-2026-10-13; o registro
after-renders.json e PDFs/PNGs foram adicionados a evidence.json.gz.
