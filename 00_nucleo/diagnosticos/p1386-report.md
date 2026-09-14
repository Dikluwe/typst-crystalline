# P1386 — relatório de paridade

## Veredito

`MATCH` para exatamente dois `Styled(Text)` size-only em ordem
`primeiro > segundo > exterior`, sem wrap. 18→12, 18→14, 20→14 e 20→18
coincidem com o vanila ratificado em uma página e nas seis coordenadas medidas.

Não há alegação geral. Igualdade, crescimento, primeiro não maior, delta
adicional, terceiro fragmento e wrap continuam `Unknown`.

## Implementação e calibração

Sequence reconhece a forma escalar sem recursão ou coleção, usa o primeiro
wrapper como máximo e remove da reserva fixa a boundary que o runtime já
entrega. Matrizes release mostraram que a cauda ainda quebrava no gate discreto
de Cursor. O owner Cursor agora fixa no fundo somente uma baseline terminal já
marcada como cauda fracionária cuja ultrapassagem não excede a boundary
exterior `below - leading`; acima dela, a quebra permanece normal.

Guardas de ULPs e margens de 0,000001pt/0,0009pt foram rejeitadas antes do
veredito. A sonda sintética também rejeitou 20pt por largura, mas a matriz
produtiva final demonstrou `MATCH`; o observável produtivo prevaleceu.

## Evidência

- RED inicial: 0/1; GREEN final: P138 24/24 e P137 21/21.
- Matriz bilateral final: 5 `MATCH`, 3 `Unknown`; quatro `MATCH` são novos.
- Mutantes declarados: 9/9 discriminados.
- `cargo build --release`, `cargo check -p typst-core`, `cargo fmt --check` e
  `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 248 warnings e 1185 infos (1433
  advisories, sem aumento).
- Execução serial: link/release sob 4 GiB, check/lint sob 3 GiB, matriz sob
  2 GiB/45 s. Pico de teste 1 961 048 KiB; release 1 139 480 KiB; matriz
  37 532 KiB. Um link herdou acidentalmente o teto de 3 GiB e o LLVM abortou
  de forma controlada; foi relançado isoladamente sob 4 GiB. Não houve OOM do
  host, timeout ou SIGKILL.

Proveniência final em 2026-09-14T00:03:49-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`5087` inserções, `419` remoções) e 366
entradas de status. Binários: vanila `eb60986b...`, cristalino `3a2273ae...`,
teste `e40db598...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, implementação, ataques e veredito; hashes demonstram ordem
e identidade, não independência.
