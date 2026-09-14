# P1387 — relatório de paridade

## Veredito

`MATCH` para exatamente dois wrappers `Styled(Text)` size-only de tamanho
igual, acima do tamanho exterior, quando o gate produtivo de largura existente
aceita a forma. Os pares 12=12, 14=14, 16=16 e 18=18 coincidem com o vanila
ratificado em uma página e nas seis coordenadas medidas.

Não há alegação geral. O par 20=20 é rejeitado pelo gate de largura e continua
`Unknown`, assim como crescimento, primeiro não maior, delta adicional,
terceiro fragmento, wrap e formas opacas.

## Implementação e calibração

Cursor passou a comparar o tamanho do texto com o exterior somente a partir do
segundo item textual. Sequence reconhece o par igual acima do exterior e aplica
a mesma boundary fixa já validada por P1386. A primeira candidata comparava
também o primeiro texto e foi rejeitada porque regrediu dois controles singleton
de P1381; a versão final preserva esses controles sem recursão nem coleção.

A sonda sintética positiva cobre 12, 14 e 16. Os tamanhos 18 e 20 atingem o gate
sintético conservador; por isso o veredito usa a matriz produtiva bilateral. O
18=18 foi aceito pelos observáveis reais, enquanto 20=20 permaneceu `Unknown`.

## Evidência

- RED inicial: 0/1; candidata v1: P138 23/25; GREEN final: P138 25/25 e
  P137 21/21.
- Matriz bilateral final: 5 `MATCH`, 3 `Unknown`; quatro `MATCH` são novos e
  um é o controle P1386 18→14.
- Mutantes declarados: 9/9 discriminados.
- `cargo build --release`, `cargo check -p typst-core`, `cargo fmt --check` e
  `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 248 warnings e 1185 infos (1433
  advisories, sem aumento).
- Execução serial: link/release e testes sob 4 GiB, check/lint sob 3 GiB,
  matriz sob 2 GiB/45 s. Pico de teste 1 963 520 KiB; release 1 137 944 KiB;
  matriz 37 572 KiB. Não houve OOM do host, timeout, SIGKILL ou abort por limite.

Proveniência final em 2026-09-14T09:47:58-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`5192` inserções, `419` remoções) e 377
entradas de status. Binários: vanila `eb60986b...`, cristalino `b361893b...`,
teste `cbc47d10...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, implementação, ataques e veredito; hashes demonstram ordem
e identidade, não independência.
