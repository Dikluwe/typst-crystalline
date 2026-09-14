# P1361 — relatório de verificação final

## Veredito

**PASS_RETROSPECTIVE_LIMITED.** O fragmento observável do formatter puro de
duração está materializado e os recibos finais limitados passam. Este veredito
não é um selo prospectivo, não atesta isolamento técnico, não demonstra
equivalência funcional geral e não diz que o estado de `watch` está em
paridade.

O regime foi o protocolo completo de materialização segregada, executado sem
atestação de isolamento. O workspace e o contexto eram partilhados. Além
disso, os metadados corretos do owner produtivo e o Prompt L0/header do teste
de integração foram formalizados somente depois do primeiro candidato e do
primeiro GREEN. Por isso, a cadeia final só admite verificação retrospectiva
limitada.

## Entradas e selo

Os hashes atuais coincidem com os pins da revisão 3:

- manifesto: `10c0d40a0e8dc6fc4507dd5cb9675018a7f55bf100e559e72916d084e051ac40`;
- contrato: `e0362460eceb7b74232463a694ee4c0dada9cfcd50ad2f724db4d46887e54ced`;
- oráculos: `92ca8560fdb6714e23957ce0f0b74da1baeeb2adf13e383a663d0852b7de48e5`;
- mutantes: `1153d68cff2cba129e1aed9f57276f52ff80f5430088617c1ca204230de82315`;
- gate estático: `7102b7b1f8cd3ef2731381d8da2affbde7b8e2e9a0952034e84f07a43d9d6fda`;
- selo retrospectivo: `3065b76d59540284a6e2d1508cf44a383c5fb27b3c6d801f7af842c61c2ad8f6`.

Passo, preflight, Prompts L0 e fonte do vanila também permanecem nos hashes
pinados. O baseline é o vanila ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

O gate é estático e determinístico: 22 dos 22 mutantes válidos são
classificados `Violated`, com testemunha, para score `1.0`. Os sete casos
opacos permanecem `Unknown` e recebem crédito zero. Esse score mede o poder
discriminatório do contrato congelado; não equivale a executar 22 patches
mutantes contra o candidato.

## Implementação e memória

`02_shell/src/duration.rs` expõe a assinatura requerida e conserva uma
implementação pura. O wrapper guarda somente um `Duration`; o formatter usa um
número fixo de escalares e escreve diretamente no `Formatter`. Não há coleção
dimensionada pela entrada, recursão, loop, cache persistente ou estado global.
A memória auxiliar produtiva é constante em relação à duração.

A inspeção não encontrou leitura de relógio, `sleep`, filesystem, ambiente,
rede, processo, `watch` ou `w`. O teste usa exatamente 22 casos literais e
mapas limitados a esse vetor, em observações sequenciais.

Hashes candidatos inspecionados:

- `02_shell/src/duration.rs`:
  `b0bd3c7e720b276b09b5fd5d796f7b0ce75af0d93fa5c84d5c11fedb438ed928`;
- `02_shell/src/lib.rs`:
  `6f558a169f306c04dc9ef848945f3f380e2ac0d81ba7d45e28289bd072cc7f82`;
- `02_shell/tests/p1361_duration.rs`:
  `fcae4541293b23630b335f92d05929307c9ba337c8170a1735f707a3166d5252`.

## RED, correções e GREEN

O RED anterior foi causal: `/tmp/p1361-red-test.stderr` contém `E0432`, pois
`typst_shell::duration` ainda não existia. O recibo tem 23.171 bytes e SHA-256
`be5e5b58fe9ccaf52b327c21ab964d33c7c418509b139ca27a398b50cec08b5b`.

Duas correções do desenho ficaram explícitas antes do contrato final:

- zero produz `0 s`, não `0 ns`, conforme a fonte do vanila ratificado;
- trocar isoladamente `>= 5 min` por `> 5 min` é mutante equivalente. Ele foi
  excluído como E01, sem crédito; M07 desloca o limiar para 300,010 s e é
  discriminado por B04 em 300,006 s.

O recibo GREEN final mostra exatamente um teste executado e aprovado:
`1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`. Build,
`cargo fmt --check`, `git diff --check` e `crystalline-lint .` terminaram com
exit code 0. O lint reporta zero violações; seus avisos e informações não foram
convertidos em violações.

Os dez ficheiros de stdout/stderr aceitos somam **323.191 bytes**. O maior é o
stdout do lint, com 277.268 bytes, abaixo do limite individual de 4 MiB; o total
fica abaixo de 16 MiB. Os comandos Cargo usaram um job, timeout máximo de 120 s
e streams redirecionados. Não houve suíte ampla, timeout, OOM, recibo de zero
testes ou início de processo `watch`/`w`.

O verificador não reexecutou Cargo, produto, testes, `watch` ou `w`; conferiu
somente fontes, artefatos protegidos e recibos existentes. Os hashes completos
dos recibos constam em `p1361-verification.json`.

## Limite da conclusão

Fica certificado apenas o adaptador/formatter S11 de duração, para o vetor e as
fronteiras congeladas. A futura ligação desse formatter às mensagens ou ao
ciclo de `watch` precisa de um passo próprio, observação segura própria e uma
nova cadeia causal.
