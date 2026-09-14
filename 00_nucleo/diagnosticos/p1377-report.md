# P1377 — relatório da fronteira vertical de Columns

## Resultado

O contentor contínuo de colunas agora entra e sai do flow como o Block auto
usado pelo vanila. A matriz bilateral fechou 6 `MATCH` e reteve 1 `Unknown`:
o modo segmentado por `colbreak()` continua explicitamente fora do crédito.

## Evidência de linguagem

- topo preservado: `One=7,404`, `Two=82,724670`, `After=220,166`;
- depois de texto: `One=27,842`, `Two=96,350000`, `After=220,166`, contra
  `96,349990` no vanila;
- saída: `Inside=27,842`, `Outside=48,280`, igualdade exata exibida;
- quebra explícita: mesma morfologia vertical do caso sem quebra forte;
- `par.spacing: 20pt`, `par.leading: 2pt`: `Inside=34,642` e
  `Outside=61,880000`, contra `61,879990` no vanila.

Todas as diferenças estão dentro da tolerância declarada de 0,001pt. A
aceitação é sobre posições e fluxo observáveis, não sobre igualdade mecânica
do algoritmo Rust.

## RED → GREEN e ataque

O baseline P1376 não tinha fronteira de bloco: `Inside=21,792` e
`Outside=36,180`. A primeira candidata somou corretamente o delta visível,
mas fixou cedo demais a baseline; `One` fechou, enquanto `Two=101,175330`
matou essa mutação contra `96,349990`. A candidata final guarda o topo do
Block pendente e deixa o cursor fixar a baseline somente na emissão. Os seis
mutantes declarados são discriminados; score 1,0.

## Limite de atestação

Protocolo completo executado sem atestação de isolamento: uma autoridade
compartilha contrato, código, ataques e veredito. Os hashes provam identidade,
não independência.

## Memória

Check serial sob 3 GiB; links seriais sob 4 GiB com debug desligado; matriz
sequencial sob 2 GiB, ficheiro máximo 16 MiB e timeout 45 s. Binário final:
49.112.152 bytes; binário focal de testes: 87.214.760 bytes; maior PDF final
da matriz: 5.852 bytes. Não houve OOM, timeout nem `SIGKILL`; RSS máximo não
foi medido.
