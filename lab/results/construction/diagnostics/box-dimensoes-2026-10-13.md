# Dimensões e composição de Box — defeitos confirmados, reparo aberto

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Medição entre
`2026-10-14T02:09:53.318325+00:00` e `2026-10-14T02:10:02.304954+00:00`. probe.json.gz contém fontes,
comandos, UTC, status, git diff HEAD --stat, fontes/L0 e hashes. Binário
Tekt `e2b75b298c46d0a7962f44cfb21e4ffc2c26f1ee7dec9ba35b6eb73c4268d905`. Vanilla comprovado
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin 9f2b6e8715237cb086899a42873660fe744622e8.
O Tekt já contém o reparo de subframe-pendentes; boxed.rs ainda não mudou
neste lote. Os arquivos permaneceram idênticos durante as sondas.

## Medição anterior à decisão

Cada documento dispõe P, uma box e Q numa página 120×100pt, margem 10pt,
fonte 11pt. Variantes têm largura 20pt, altura 14pt, inset 2pt, outset 3pt
ou radius 4pt; as fontes exatas estão no snapshot. Formas do corpo têm
40×24pt ou 40×8pt, fill red e stroke none. pdftotext -bbox mede posições
em pontos; pdftoppm a 144dpi gera o canvas integral. Pixel vermelho exige
R>150, G<100, B<100. Estes números descrevem geometria, não paridade geral
nem identidade de bytes/antialiasing.

| Caso | Binário | Q.x | Q.y | Pixels vermelhos |
|---|---|---:|---:|---:|
| empty | tekt | 35.949000 | 14.166000 | 1120 |
| empty | vanilla | 35.951000 | 14.166000 | 1120 |
| empty-inset | tekt | 39.953000 | 18.166000 | 1344 |
| empty-inset | vanilla | 35.951000 | 14.166000 | 1120 |
| empty-outset | tekt | 41.955000 | 14.166000 | 2080 |
| empty-outset | vanilla | 35.951000 | 14.166000 | 2046 |
| shape-clip | tekt | 35.951000 | 44.128000 | 3840 |
| shape-clip | vanilla | 35.951000 | 14.166000 | 1120 |
| shape-inset-clip | tekt | 39.951000 | 48.128000 | 3840 |
| shape-inset-clip | vanilla | 35.951000 | 14.166000 | 864 |
| shape-radius-clip | tekt | 35.951000 | 44.128000 | 3840 |
| shape-radius-clip | vanilla | 35.951000 | 14.166000 | 1050 |
| shape-horizontal-clip | tekt | 35.951000 | 44.128000 | 1280 |
| shape-horizontal-clip | vanilla | 35.951000 | 30.166000 | 640 |
| shape-auto-height-clip | tekt | 35.951000 | 22.128000 | 1280 |
| shape-auto-height-clip | vanilla | 35.951000 | 8.166000 | 640 |

## Contrato e classificação

A caixa vazia de tamanho explícito é controle positivo: a geometria e a
linha coincidem entre as implementações, ressalvada a precisão de texto
registrada no PDF. O inset da caixa com dimensões já fixadas deve reduzir
o espaço interior, e outset expande a pintura sem consumir espaço de
layout. Fonte normativa da linguagem no snapshot upstream:
container.rs:96-127; implementação auxiliar inline/box.rs:26-50 e
flow/block.rs:255-295. Não se exige copiar os buffers ou o algoritmo.

As relações também são verificáveis dentro do próprio Tekt: adicionar
inset à caixa vazia de largura/altura explícitas não deve deslocar Q;
adicionar outset não deve mudar Q.x ou Q.y. Ambos mudam atualmente.
O corpo de uma caixa inline também não deve empurrar Q para outra baseline
apenas por conter uma forma, e clip deve restringir a pintura na largura,
mesmo quando a altura do corpo cabe ou height é auto. As sondas refutam
essas propriedades, independentemente de se aceitar todo o output vanilla.

## Causas e próximas mudanças necessárias

- boxed.rs:99-107 instala width sem compensar o limite direito da região;
  :219-225 soma inset/outset à largura explícita já declarada.
- :164-174 usa a cauda inline, deixando fora dela as formas comitadas.
  :195-202 soma novamente body_origin_x às formas. cursor_y alterado pelo
  corpo não é restaurado, por isso Q desce sem o prefixo P.
- :261-329 recorta só por overflow vertical medido e usa a largura do corpo,
  não a largura da caixa. O Group adicional não localiza seus filhos;
  radius não participa da máscara. Height auto e overflow só horizontal
  escapam desse caminho.
- :337-383 desenha o fundo diretamente nos itens comitados, em vez de
  mantê-lo junto ao corpo inline; tamanho e origem usam proxies tipográficos.

A composição deve manter uma região local completa, dimensão lógica da
caixa e dimensão de pintura separadas; aplicar inset dentro do tamanho
explícito, outset fora do avanço, e recortar o corpo na mesma origem do
fundo. Preservar baseline textual/não textual, prefixos/sufixos, frações,
repeats, decoração e metadados de alinhamento/parent bbox. Isso exige
atualizar primeiro o L0 de boxed, que ainda manda conservar a emissão
histórica. O snapshot conserva esse conflito em vez de tratá-lo como
contrato correto por estar escrito no prompt. Nenhum reparo de Box foi
feito neste lote; a investigação não autoriza declarar os casos fechados.

## Evidências

probe.json.gz preserva a proveniência completa; evidence.json.gz reúne
fontes Typst, PDFs, rasters, XML, receitas e este relatório, com SHA-256
verificado após descompressão. O lote não altera o pin vanilla ou o CI.
