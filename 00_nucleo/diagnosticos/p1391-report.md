# P1391 — relatório de materialização

## Resultado

Os quatro pares 11↔39, 39↔11, 11↔40 e 40↔11 são `MATCH` bilateral:
uma página, duas linhas e `After=Tail=220,166pt`. A maior diferença observada
foi 0,00001pt, dentro do oráculo de 0,001pt. O controle 40↔8 permanece
`Unknown`, e P137/P138/P1390 preservam os vereditos anteriores.

## Causa e implementação

O candidato inicial resolveu a ordem exterior↔maior, mas não a inversa. Um
probe causal mostrou que 39pt/40pt torna `Lead` uma palavra overlong para a
coluna. O layout de texto fechava essa linha antes de Sequence alcançar o
carrier P1389, usando o próprio tamanho grande para prever a baseline seguinte.

O scanner agora adia somente essa rejeição horizontal e só a legitima depois
de provar a forma P1391 completa. Sequence arma um estado privado one-shot;
o primeiro `flush_line` automático o consome e reutiliza o fechamento
singleton exterior P1389. Checkpoint preserva o estado e sub-frame o isola.
Não há recursão, coleção proporcional, API pública, mudança de default ou
nova fase.

## Verificação e limite de autoridade

RED observado; GREEN focal 2/2; P138 29/29; P137 21/21; filtro P139 4/4;
matriz bilateral 4/4. Oito mutantes declarados foram discriminados pelos
oráculos, mas não injetados mecanicamente. A mesma autoridade produziu
contrato, implementação, ataques e veredito; independência não é atestada.

As execuções foram seriais. Um teto virtual inicial de 4 GiB abortou o LLVM
de forma contida com 2.646.996 KiB RSS; não houve OOM do host nem swap. O teto
de ligação foi corrigido para 6 GiB. O maior RSS observado foi 3.372.104 KiB;
o teste final usou 1.641.108 KiB e a matriz, 37.344 KiB.
