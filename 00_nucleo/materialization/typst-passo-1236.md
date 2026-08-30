# P1236 — fechar a semântica de alpha dos gradients

**Estado:** EXECUTADO — ACEITE COMO DIAGNÓSTICO DE FRAGMENTO
**Predecessor:** P1234 corrigido

## Resultado

O probe público corrigido mediu seis fixtures Luma (Linear/Radial), sete
posições por fixture e os dois sistemas: 84 observações, agrupadas em 42 pares.

- 14 pares divergiram em alpha até `0.6`. O L0 P1252 vigente classifica a
  preservação cristalina como `Known-Upstream-Bug`; não é correção a reverter.
- Nenhum par divergiu em luminância no binário atual.
- 28 pares foram preservados e nenhum ficou `Unknown` na fronteira pública B03.

O resultado não promove nenhuma combinação SVG: a exceção de alpha não pode
perdoar o delta separado de luminância. Não houve mudança produtiva nem de L0.
Duas execuções completas foram byte-idênticas. Os antigos nove “ataques” eram
linhas sintéticas produzidas pelo próprio runner; foram revogados e nenhum
`mutation_score` é alegado. A classificação P1252 é uma reavaliação sob o L0
vigente, não uma autoridade congelada antes da execução histórica.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
