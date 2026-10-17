# Cobertura conic — controle independente do vanilla

## Estado e receita

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC
`2026-10-14T01:23:33.487147+00:00` até `2026-10-14T01:23:36.842599+00:00`. Fonte completa dos casos,
fontes Rust consultadas, status e diffstat estão em probe.json.gz.
Binário Tekt: `9163155b83268d800cafbcd31adba2a4b8024494df71433f17ffa83d77107637`. Rasterizador: pdftoppm,
144 dpi; cada canvas tem 192×128 pixels. Sem comparação com vanilla.

Para cada caso, compilar o mesmo retângulo trocando somente o fill
red por gradient.conic(red,red,space:rgb), incluindo caixa deslocada,
centro deslocado, rotação e referência parent. A cor constante deve
preservar a cobertura, independentemente de tesselação ou formato.
Critério pré-fixado na receita: pixel pintado do controle sólido tem
algum canal mais de 32 níveis distante do branco; branco do gradiente
significa todos os canais acima de 240. Compara-se também o canvas
inteiro com diferença por canal >16, sem alinhamento ou recorte.

| Caso | Pixels pintados no sólido | Brancos do conic nessa área |
|---|---:|---:|
| page | 24576 | 24571 |
| offset | 7072 | 7072 |
| center | 24576 | 24571 |
| rotate | 4104 | 4104 |
| parent | 4000 | 4000 |

## Classificação após medição

A perda de cobertura é defeito do Tekt: uma cor constante não pode
sumir por ser declarada como gradiente. Não é uma decisão baseada em
aparência do vanilla. A fonte conic escreve uma malha em domínio 0..1;
resources.rs:564 aplica Pattern Matrix identidade a RGB/CMYK, ao passo
que a malha precisa de posição e dimensão físicas. O L0 conic já atribui
o posicionamento físico ao builder; a execução não o realiza nesses ramos.

A especificação PDF, seção 4.6.1, coloca o Pattern no espaço inicial do
stream que o declara; transformações posteriores de pintura não corrigem
sua matriz implicitamente. Ver [Adobe PDF Reference 1.3, página 202](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/pdfreference1.3.pdf).
Logo, além da escala, a correção precisa considerar grupos e posição.

O código de Group local em stream/draw.rs:947-952 também descartava
matriz, posição e recorte dos filhos. É defeito independente já contrário
ao L0 de draw; sua correção e os testes causais ficam no diagnóstico
grupo-aninhados (arquivo grupos-aninhados-2026-10-13.md). O controle
sólido não certifica essa geometria anterior: ele isola a perda adicional
causada pelo conic. Não inferir que o sólido esteja globalmente correto.

## Pendências explicitamente abertas

Este lote mede, mas não fecha o posicionamento conic. A correção precisa
preservar centro, ângulo, relative self/parent, caixas próprias e de grupos,
transformações afins, páginas de alturas diferentes e reutilização do mesmo
Arc. Escalar um círculo apenas até a caixa inscrita ainda deixa cantos sem
cobertura. O caminho CMYK também possui uma amostragem diferente que exige
investigação separada. Não copiar o fallback uniforme observado no vanilla.
A classificação de todas as divergências de imagem permanece em aberto.

## Arquivo de evidências

probe.json.gz preserva o estado inicial. evidence.json.gz contém a receita,
fontes Typst, PDFs e PNGs, identificados por SHA-256. As imagens comprovam
a medição da baseline, não o estado posterior a correções de outros módulos.
