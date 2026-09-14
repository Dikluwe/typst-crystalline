# P1385 — relatório de paridade

## Veredito

`MATCH` para exatamente dois `Styled(Text)` size-only quando o primeiro é
maior que o tamanho exterior e o segundo é menor ou igual ao exterior. Os
pares 18→8, 18→11 e 16→8 coincidem com o vanila em uma página e em todas as
coordenadas medidas. P1383 crescente e P1384 com segundo texto direto também
permanecem exatos.

Não há alegação geral. 18→14, 18→18, 18→20, 10→8, size+fill e fragmento único
maior continuam `Unknown` e fail-closed.

## Implementação e calibração

O reconhecedor usa os acumuladores escalares existentes: ambos os wrappers
precisam ser size-only, o primeiro deve superar o exterior e o segundo deve
retornar ao exterior ou abaixo. O máximo vem do primeiro; próximo ascent,
leading e boundary continuam exteriores como em P1384.

A candidata v1 aceitava qualquer segundo tamanho abaixo do primeiro. Três
positivos passaram, mas 18→14 deixou a cauda 6,05pt curta. O contrato foi
reaberto e a candidata v2 adicionou o teto exterior. Não há recursão, coleção
proporcional, carrier, API pública ou mudança de pipeline.

## Evidência

- RED revisado: 3/6; apenas os três positivos falharam. GREEN final: 6/6.
- Controles: P1380–P1384 15/15 e P1374–P1379 16/16.
- Matriz bilateral: 5 `MATCH`, 6 `Unknown`.
- `cargo check --workspace`, `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 1433 advisories preexistentes e zero
  advisory novo.
- Execução serial: testes/link sob 4 GiB, matriz sob 2 GiB/45 s. Picos: testes
  1 963 624 KiB, release 1 137 700 KiB, matriz 37 540 KiB; nenhum OOM,
  timeout ou SIGKILL.

Proveniência final medida em 2026-09-13T23:03:03-03:00: HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado, 47 ficheiros no diff (`4941` inserções, `419` remoções) e 356
entradas de status. Binários: vanila `eb60986b...`, cristalino `9b089823...`,
teste `cf3e2c70...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, testes, candidato, ataques e veredito; hashes demonstram
ordem e identidade, não independência.
