# P1388 — relatório de paridade

## Veredito

`MATCH` para o par 20→20 anteriormente `Unknown`: o prefixo certificado agora
preserva sua boundary mesmo quando whitespace terminal fecha a linha antes do
Block. As seis coordenadas medidas coincidem com o vanila ratificado dentro de
0,001pt. O caso 21→21 possui wrap real e permanece `Unknown`.

Não há alegação geral para wrap, delta adicional, terceiro fragmento, wrapper
aninhado ou forma que não produza distribuição certificada.

## Implementação e ataques

O reparo separa duas obrigações no consumidor da boundary: `flush_line` só é
chamado se a linha continua aberta, mas a boundary certificada é aplicada uma
vez independentemente desse estado incidental. O terminador que fechou a linha
não consome a boundary.

Quatro candidatas horizontais foram executadas e rejeitadas por não mudarem o
resultado produtivo: reconstrução local do estilo, checkpoint vazio, largura
total/checkpoint e classificação whitespace-only. O contraste entre fixtures
`leading-only` e `trailing-only` isolou a perda no handshake vertical. O RED
comportamental mediu `One=36,05pt` com terminador contra `42,10pt` sem ele; o
GREEN igualou os dois caminhos.

## Evidência

- RED focal: 0/1; GREEN focal: 1/1; grupos P138: 26/26 e P137: 21/21.
- Matriz bilateral final: 8 `MATCH` e 1 `Unknown`; o novo `MATCH` é 20→20.
- Mutantes declarados: 8/8 discriminados.
- `cargo build --release -p typst-wiring`, `cargo check -p typst-core`,
  `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 248 warnings e 1185 infos, sem aumento.
- Execução serial: compilação/testes e link sob 4 GiB, check/lint sob 3 GiB,
  matriz sob 2 GiB e timeout de 45 s por processo. Pico global 2 090 988 KiB;
  release 1 136 232 KiB; matriz 37 712 KiB. Não houve OOM do host, timeout,
  SIGKILL, swap ou aborto por limite.

Proveniência final em 2026-09-14T10:44:43-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`5270` inserções, `419` remoções) e 388
entradas de status. Binários: vanila `eb60986b...`, cristalino `6739562e...`,
teste `a710acb0...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, implementação, ataques e veredito; hashes demonstram ordem
e identidade, não independência.
