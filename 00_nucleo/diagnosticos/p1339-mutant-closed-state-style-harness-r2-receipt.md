# P1339 — sucessor instrumental do harness de estilo

Autor: `/root/p1336_tests`, papel de tradução mecânica segregada.
Regime: executado sem atestação de isolamento. Contexto P1336 herdado.
Registro causal: 2026-09-10, após mensagem do coordenador reportando E0499
instrumental no harness r1, linha 222 (`engine.sink = &mut validation_tracked`).
Relógio consultado durante preparação: 2026-09-10 06:06:54 UTC.
O erro de compilação foi relatado pelo observer via coordenador; não foi
reproduzido nem atribuído a um build executado por este papel.

## Entradas e saídas imutáveis

- Manifesto de autoridade ancestral: `p1339-authority-manifest-r2.json`,
  SHA `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
- Harness original lido, preservado e hash reconferido:
  `p1339-mutant-closed-state-style-harness.rs`,
  SHA `2f01811e5ae26ea7ed5650adcf359c47b00afdbdd22089bd0775cd6d6f7d8f21`.
- Fixture/DTO original lido e inalterado:
  `p1339-ab-batch1-same-context-style-fixture.json`,
  SHA `70d572c86aee6499c0f5ebf20849187d4113ebc6016888f1e0b4a0ea43d572aa`.
- Sucessor: `p1339-mutant-closed-state-style-harness-r2.rs`,
  SHA `6527552f2ef6518e2fbc22f02ac1a73af8bb7c34803d0f572c1320b1e3258231`.
- Diff exato: `p1339-mutant-closed-state-style-harness-r2.diff`,
  SHA `0505c7a04ead0a076ae425a9aa1f25bdc3ed83bb16501e52c5b91508ecd25957`.

## Causa e transformação restrita

O r1 mantém um `Engine` com referência mutável ao tracked sink, depois atribui
outra referência mutável ao seu campo `sink` dentro do loop. A mensagem E0499
reportada é compatível com o empréstimo persistindo entre iterações, vinculado ao
lifetime do Engine. Nenhuma fonte candidata foi necessária ou lida para esta
análise instrumental.

O r2 conserva os owners de recursos fora das operações e cria apenas visões
`Engine` de empréstimo curto. Cada visão recebe um `TrackedMut::reborrow_mut()`
do sink real apropriado. Setup e hook begin ficam em uma visão; cada operação
fica em outra; hook finish recebe a visão final. `current_file` é carregado da
visão anterior, incluindo alterações produzidas pela chamada real. O booleano
`validation_phase` mantém a troca para o sink de validação a partir da mesma
operação que antes atribuía `engine.sink`; a visão final também usa esse sink.

Permanecem os mesmos `EvalContext`, Scopes, World, FixedMetrics, snapshot,
Location, armazenamento StyleChain, show_rules e active_guards. Não se recriam
o contexto, registros ou valores de captura. Os mesmos recursos lógicos do
Engine persistem; o endereço físico de sua visão Rust não é preservado nem é
um campo/predicado do DTO. Se um binding exigir tal endereço como identidade,
isso é um gap de binding a reportar, não licença para fabricar identidades.

As operações da fixture continuam na mesma ordem: cadeia 10, request10, cadeia
20, request20, cadeia 30, seleção e validação. Corpos, fontes, asserções,
predicados, DTO, cardinalidades, perfis, ordens e serialização dos dois sinks
estão textualmente preservados. Nenhum algoritmo de produto, hook, evento ou
resultado é substituído ou calculado pelo ajuste de lifetime.

## Estado dos gates

Somente leitura do harness/DTO e escrita dos três artefatos sucessores foram
realizadas. A obtenção do diff retornou exit 1, que significa diferenças em
`git diff --no-index`, não falha de compilação. Não houve rustfmt, rustc, Cargo,
execução de testes, imports de checker ou leitura do candidato nesta subtarefa.

Estado: `PROPOSED_MECHANICAL_SUCCESSOR_PENDING_STATIC_VERIFIER`.
Não há crédito de compilação, runtime, PASS ou autorização autônoma de fase F.
O coordenador/verificador deve decidir primeiro o gate afetado e o resselo.
Registry, drivers, wrappers, harness r1 e oráculos não foram alterados.
