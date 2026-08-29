# P1239 — Luma delimitado e corrigido em L1

**Veredito:** Linear/Luma e Radial/Luma preservados no fragmento público
P1236; alpha continua `Known-Upstream-Bug` conforme P1252; SVG continua
`Unknown`.

## Medição antes da decisão

O witness mínimo era `luma(red)`: vanilla ratificado `a51e02804` expunha
`54.02%`, enquanto o cristalino expunha `54.01%`. A fonte do cristalino usava
`0.2126/0.7152/0.0722`; `palette 0.7.6`, dependência do vanilla, usa a linha Y
da matriz sRGB `0.2126729/0.7151522/0.0721750`
(`palette-0.7.6/src/matrix.rs:286`). A diferença era portanto fórmula L1,
não interpolação, evaluator ou compensação SVG.

## Resultado

A fórmula foi corrigida no owner `entities/color`. O corpus P1236 foi
reexecutado: seis fixtures × sete posições × dois sistemas = 84 observações,
42 pares. Todos os 42 deltas de luminância ficaram zero; 28 pares são
`Preserved` e 14 divergem apenas no alpha perdido pelo vanilla, já classificado
`Known-Upstream-Bug`. Não houve `Unknown` na fronteira pública.

Os cinco ataques históricos não tinham fontes mutadas, comandos e recibos
duráveis; foram revogados e não há mutation score. Linear/Luma e Radial/Luma
continuam fallback explícito no SVG.

O runner reproduzível `lab/parity/matrix/p1239_luma.py` confirmou `54.02%`
nos três witnesses públicos em ambos os binários e vinculou por hash o corpus
P1236 atual: zero deltas Luma, 28 pares preservados, 14 divergências somente de
alpha e zero `Unknown`. Duas execuções completas foram byte-idênticas.

## Proveniência

- HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`;
- working tree não commitada, medida em `2026-08-27T22:47:55-03:00`;
- `git diff HEAD --stat`: 59 ficheiros, 2247 inserções, 188 remoções;
- summary do probe SHA-256
  `e695b924d2bc0b5ee2f2b7e668d7b87b5dcbd7069a7e857ad0fbfd1cb8ee17d3`;
- matriz `palette` SHA-256
  `64012446b58a0d095e0879fa12d316fb795da0440eba3f85f967ea55645efc64`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
