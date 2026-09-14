# P1378 — relatório da origem lógica dos segmentos de colunas

## Resultado

Segmentos explícitos separados por `colbreak()` agora partem da mesma origem
lógica, não vazam o estado de baseline entre irmãos e deixam o fluxo seguinte
abaixo do segmento não vazio mais profundo. A matriz bilateral fechou 5
`MATCH` e reteve 1 `Unknown`: a altura fracionária segmentada continua fora do
crédito.

## Evidência de linguagem

- no topo, `One=Two=7,404` e `Outside=27,842`;
- após texto anterior, `One=Two=27,842` e `Outside=48,280`;
- no segmento desigual, `Lower=21,792`, enquanto `Outside=42,230`, sem
  sobreposição;
- um primeiro segmento vazio mantém `Two=7,404` e `Outside=27,842`;
- o controle contínuo P1377 preserva `Inside=27,842` e `Outside=48,280`.

As posições exibidas coincidem com o vanila dentro da tolerância declarada de
0,001pt. A aceitação é sobre a morfologia vertical observável, não sobre a
estrutura interna do algoritmo.

## RED → GREEN e ataques

Antes da implementação, os três testes focais falharam: a primeira baseline
recebia ascender duas vezes, o estado entrava no irmão seguinte e a saída
usava o último segmento em vez do mais profundo. A correção removeu a
pré-soma, restaura snapshots escalares de entrada para cada irmão e conserva
somente a baseline mais profunda para a saída. Os seis mutantes declarados
foram discriminados; score 1,0. O pré-selo v1 foi invalidado antes do código
porque o oráculo do segmento vazio comparava duas origens igualmente erradas;
o pré-selo v2 usa texto direto no topo como controle.

## Limite conhecido

Com `block(height: 1fr)`, as origens agora coincidem (`One=Two=7,404`), mas
`After` e `Tail` ficam em `42,230` no cristalino contra `220,166` no vanila.
Esse caso permanece `Unknown` e delimita o próximo passo; não recebeu crédito
por melhora parcial.

## Limite de atestação

Protocolo completo executado sem atestação de isolamento: uma autoridade
compartilha contrato, código, ataques e veredito. Os hashes provam identidade,
não independência.

## Memória

Check serial sob 3 GiB; links e testes seriais sob 4 GiB com debug desligado;
matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e timeout 45 s. Binário
final: 49.106.896 bytes; binário focal de testes: 87.233.832 bytes; maior PDF
final da matriz: 6.160 bytes. Não houve OOM, timeout nem `SIGKILL`; RSS máximo
não foi medido.
