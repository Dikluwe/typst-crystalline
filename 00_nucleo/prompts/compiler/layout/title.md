# Prompt L0 — `compiler/layout/title` — layout de `TitleElem`
Hash do Código: 3da4144c


**Camada:** L1  
**Ficheiro proprietário:** `01_core/src/compiler/layout/title.rs`  
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129  
**Medição de paridade:** P765a

## Medição vigente

O consumer proprietário contém a free function estática `title::layout`, chamada
pelo match exaustivo do núcleo de layout conforme a forma B da ADR-0109. Ela recebe
`&mut Layouter` e `&TitleElem`.

## Contrato

- calcular o tamanho como `1.7 × style.size`;
- substituir temporariamente o estilo por `TextStyle` bold, não italic e com o
  tamanho calculado, preservando os demais defaults vigentes usados pelo código;
- fazer flush anterior somente se o cursor já avançou além da margem esquerda;
- materializar o body e fazer flush posterior;
- restaurar integralmente o estilo anterior ao retornar.

## Aceitação em nível de linguagem

O título aparece como bloco próprio, maior e em negrito, sem contaminar o estilo do
conteúdo posterior.

## Fora de escopo

Não pertencem a este owner: construção ou campos de `TitleElem`, resolução do
argumento e do metadado `document.title`, parsing, introspecção, HTML e mudanças
futuras de estilo. A metodologia histórica de P765 não é contrato deste módulo.
