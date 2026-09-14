# P1389 — relatório de paridade

## Veredito

`MATCH` para dois wrappers `Styled(Text)` size-only iguais, acima do tamanho
exterior, com uma palavra por body, exatamente um wrap entre fragmentos e
reserva vertical suficiente. Os pares 21→21, 22→22, 24→24 e 30→30 coincidem
com o vanila ratificado nas seis coordenadas medidas; 20→20 sem wrap permanece
exato.

Multiword, tamanhos diferentes, delta adicional, terceiro fragmento, wrap
interno ou múltiplo, reserva insuficiente e formas opacas continuam `Unknown`.

## Implementação e calibração

Sequence transporta privadamente o índice do único wrap, força o fechamento
antes do segundo wrapper e protege a segunda linha contra whitespace terminal.
Cursor reutiliza `flush_line` num modo transitório que conserva máximo, descent
e correção de ascent da linha corrente, mas usa o exterior somente para próximo
ascent e leading. A reserva acrescenta uma linha com o mesmo máximo; a boundary
P1388 continua separada e única.

A v1 reduziu o avanço FixedMetrics de 28,35pt para 21,85pt. A v2 adotou um
oráculo sintético incorreto de 14,85pt e passou localmente, mas foi rejeitada
pela matriz produtiva: todas as posições de `More` ficaram cedo demais por
`top(interno)-top(exterior)`. O contrato foi reaberto antes da v3, que restaurou
essa correção e fechou a matriz. Um advisory novo por `_ => false` foi removido
sem alterar semântica; o baseline do linter voltou a 1433.

## Evidência

- RED inicial 0/1; GREEN final P1389 3/3; P138 29/29; P137 21/21.
- Matriz bilateral selada: 5 `MATCH`, 1 `Unknown`; quatro `MATCH` são novos.
- Mutantes declarados: 8/8 discriminados.
- Build release e debug, `cargo check -p typst-core`, formatação e diff: sucesso.
- `crystalline-lint .`: zero bloqueios, 248 warnings e 1185 infos, sem aumento.
- Execução serial: testes/link sob 4 GiB, check/lint sob 3 GiB e matriz sob
  2 GiB/45 s. Pico global 2 490 096 KiB; link máximo 1 144 828 KiB; matriz
  37 476 KiB. Não houve OOM, swap, timeout, SIGKILL ou aborto por limite.

Proveniência final em 2026-09-14T11:20:48-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`5482` inserções, `419` remoções) e 399
entradas de status. Binários: vanila `eb60986b...`, cristalino `221784f2...`,
teste `159ae592...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, implementação, ataques e veredito; hashes demonstram ordem
e identidade, não independência.
