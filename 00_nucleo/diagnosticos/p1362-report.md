# P1362 — relatório final de verificação

## Veredito

`PASS_WITH_LIMITED_ATTESTATION` para o fragmento observável congelado em
P1362: quatro corpos de status sem cor, seleção por outcome tipado, emissão em
stderr com uma quebra de linha, ativação exclusiva pelo ciclo `watch` e
`Compiling` antes de cada chamada de compilação observada.

O protocolo foi **executado sem atestação de isolamento**. Este veredito não
declara equivalência funcional geral com o vanila, nem equivalência geral dos
comandos `watch` ou `compile`.

## Integridade da cadeia

Os hashes atuais reproduzem os pins protegidos:

- manifesto: `3216260f0b29fcd3b8dbc2cf483f9749573f4e89975de2b5a20ac809aefbf39e`;
- contrato: `17ae4ff69097473817b0f3e2d135c85f5aeee854de980286b74cabaf98799bb2`;
- oráculos: `4dea845b6f0beb0ed47da423075863d52cba435360658527560cecdc48255663`;
- mutantes: `e346091c23e47c4f37671329293a21ef40fd41073e9724e97e9172818655ab85`;
- gate: `4a97797d63cd2ef7c855cebdf7977cd0c45b69adcb03c610a19ce2f0b13e3e64`;
- selo: `8ad117ae2bb8bfc9ec49a7a60aa252bf9741af59a4b1c748d30ea41a75e805e5`.

Após o resselo mecânico, cada Prompt contém exatamente uma linha `Hash do
Código:` e mantém o digest normativo congelado:

- `shell/duration.md`: `02bb6aa40c3fd936d786ee20ecdbd3d078c41dc23bbbf547a7608db0c3c3499b`;
- `shell/tests/p1361-duration.md`: `14216054705c8f2b0cee0e3f8b17677d37065c7dd706a49086d1ed9206ba1ded`;
- `wiring.md`: `e9907dedd503cbc2e82829632b0a6996342ea67cffe5545fffb23a6cda76ab1e`.

O gate estático registrou 22/22 positivos `Preserved`, 22/22 mutantes válidos
`Violated`, zero sobreviventes, mutation score `1.0`, e 8/8 casos opacos
`Unknown` com crédito zero. Essa discriminação é binding-free e estática; os
mutantes não foram executados contra o produto.

## Inspeção da implementação

L2 contém o enum fechado `WatchStatus` e somente produz o corpo textual. O
módulo não faz I/O nem lê relógio. Em L4:

- `run_compile` apenas devolve o exit de `run_compile_observed` e não emite os
  novos status;
- `run_watch` usa `eprintln!` para `Compiling`, captura `Instant`, chama a
  compilação observada e emite exatamente um status final selecionado;
- `has_warnings` é calculado com `!warnings.is_empty()` sobre a coleção tipada,
  antes de ela ser drenada para stderr; nenhum texto de stderr é analisado;
- erro de compilação seleciona `Error` independentemente do bit de warnings;
- retornos anteriores à criação da coleção transportam `false`;
- a inspeção do diff preserva no fluxo existente staging, coleção de
  dependências, `arm`, `publish`, `abandon`, espera por alteração e eviction.

Os testes integrados não criam processos. O RED é causal: o recibo L2 contém
E0432 por ausência de `WatchStatus`; o recibo L4 contém E0432 e E0425 por
ausência de `WatchStatus` e `watch_status_for_outcome`.

## Gates e contenção de memória

Os dois recibos GREEN finais mostram, individualmente, `1 passed; 0 failed`.
O build termina com `Finished dev profile in 2.23s`. Os streams finais de
formatação e `git diff --check` estão vazios. O lint contém warnings e
informações, nenhuma linha de erro observada, e o recibo de resselo diz
`Nothing to fix`.

Os exit-status 0 desses comandos foram informados pelo executor/orquestrador;
eles não foram gravados em ficheiros separados. O conteúdo e os hashes dos
streams os corroboram, mas não substituem um recibo de status autônomo.

Os streams aceitos de testes, build, fmt, diff e lint somam **370.903 bytes**;
incluindo o recibo de `fix-hashes`, **370.918 bytes**. O maior stream tem
**277.268 bytes**, abaixo do limite individual de 4 MiB e do total de 16 MiB.
O verificador não executou `cargo`, produto, testes, `watch` ou `w`.

## Limitações e scope-outs

- Todos os papéis compartilharam o workspace, e o implementador viu
  acidentalmente o teste A/B; a independência prática é limitada.
- Nenhuma execução externa validou o envelope, a repetição ou a ordem real do
  ciclo `watch`.
- Não há prova externa de monitoramento de processos; há apenas política,
  inspeção dos testes e ausência de recibo de execução de `watch`/`w`.
- Timestamp, cor, fullscreen, limpeza do terminal, posição/conteúdo/ordem de
  diagnósticos, polling, renovação de dependências, arm/publish/abandon,
  staging, eviction, telemetria e desempenho continuam fora do recorte.

O certificado estruturado correspondente é
`00_nucleo/diagnosticos/p1362-verification.json`.
