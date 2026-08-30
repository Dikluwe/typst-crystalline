# P1256 — localizar worst-t e intervalo emitido dos gaps SVG

**Estado:** EXECUTADO — CAUSA REDUZIDA; SEM ESCRITA PRODUTIVA  
**Predecessor:** P1255  
**Owner candidato:** `infra/export/gradients/adaptive`, ainda não adjudicado.

## Objetivo

Para as 24 fixtures congeladas do P1237, localizar separadamente o pior ponto
de cor e de alpha, os stops SVG que o cercam, a largura do intervalo e uma
profundidade diádica inferida. A inferência descreve somente a saída emitida;
não é trace privado do helper.

## Regras

- usar apenas o oráculo vanilla e os SVGs congelados pinados pelo P1237;
- preservar Linear/Radial, espaço, role e fixture como identidades distintas;
- medir cor premultiplicada e alpha separadamente;
- não alterar threshold, cap, budgets, L0 ou produção;
- `Unknown` permanece `Unknown`; sem mutation score sem mutantes executáveis.

## Gate de decisão

Uma correção futura só pode apontar `adaptive.rs` se o witness mostrar erro já
presente entre samples emitidos. Se a divergência nascer em cor/opacity
serializada, o owner é SVG serialization. A profundidade inferida não autoriza
aumentar o cap: primeiro deve ser comparada com o intervalo e o canal causal.

## Saídas

- `00_nucleo/diagnosticos/p1256-localization.tsv`;
- `00_nucleo/diagnosticos/p1256-summary.tsv`;
- `00_nucleo/diagnosticos/p1256-manifest.tsv`.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

## Resultado executado

Foram produzidas 48 observações (cor máxima e alpha máximo para cada uma das
24 fixtures), em duas execuções byte-idênticas.

- o witness `two-wide-stroke` atinge 64 subdivisões nos quatro pares. O excesso
  de `color_max` permanece tanto em Oklab quanto em LinearRgb; portanto este é
  o caso adequado para testar comportamento no cap;
- os maiores gaps Oklab em `base-square-fill` e `coincident-tall-fill` ocorrem
  com apenas 32 subdivisões, abaixo do cap. A hipótese “somente cap 64” é
  refutada para a família dominante;
- os piores pontos Linear/Radial são idênticos por fixture no corpus congelado,
  mas continuam adjudicados separadamente;
- alpha zero escolhe `t=0` apenas como primeiro máximo empatado e não constitui
  witness causal;
- os intervals emitidos foram localizados, porém a decisão privada tomada em
  cada midpoint continua opaca. Assim, ainda não se atribui definitivamente o
  owner ao helper nem à serialização.

Próximo gate: criar um teste diagnóstico do helper puro que registre, para os
witnesses cap-hit e below-cap, o erro antes da serialização e depois da
serialização. Somente esse contraste autoriza uma correção produtiva.
