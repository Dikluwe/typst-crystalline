# P1392 — relatório de materialização

## Resultado

Os quatro pares 8→40, 40→8, 7→40 e 40→7 são `MATCH` bilateral: uma página,
duas linhas e `After=Tail=220,166pt`. A maior diferença observada foi
0,00001pt, dentro do oráculo de 0,001pt. O controle 40→8 com `fill` adicional
permanece `Unknown`, e P137/P138/P1391/P1390 preservam seus vereditos.

## Causa e implementação

O máximo da linha partia do fallback exterior de 11pt. Por isso um singleton
real de 7pt/8pt não era distinguido do exterior. A correção compara o único
item textual real: ele conserva baseline/ascent próprios e usa somente o
leading exterior no fechamento certificado.

Na ordem maior→menor, a palavra grande fecha antes de Sequence visitar o
segundo wrapper e deixa uma baseline exterior. O carrier P1392 antecipa essa
baseline uma única vez pelo delta de top-edge exterior→menor. A reserva já
contém a mesma redução por linha. O estado é escalar, one-shot e restrito ao
certificado; não há recursão, coleção proporcional, API pública ou mudança de
default.

## Verificação e limite de autoridade

RED observado; GREEN focal 2/2; P138 29/29; P137 21/21; filtro P139 5/5;
matriz bilateral 4/4. Dez mutantes declarados foram discriminados pelos
oráculos, mas não injetados mecanicamente. A mesma autoridade produziu
contrato, implementação, ataques e veredito; independência não é atestada.

Todas as execuções foram seriais e limitadas. O maior RSS observado foi
2.123.328 KiB na ligação do RED; o teste final usou 1.638.008 KiB, o build
release final 1.597.064 KiB e a matriz 37.552 KiB. Não houve swap, timeout,
SIGKILL nem OOM do host.
