# P1339 — recibo mecânico do adaptador das três APIs

Estado: fonte de teste prospectiva pronta para revisão independente, **NOT_EXECUTED_PRESEAL**, sem selo e sem crédito de execução. Autor `/root/p1336_tests`, papel adicional estritamente mecânico segundo suplemento; isolamento apenas procedural, contexto herdado P1336. Nenhum código candidato ou produtivo foi escrito/lido para esta tradução.

## Entradas pinadas antes da tradução

- Manifesto r2: `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`.
- Suplemento closed-harness: `08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`.
- Freeze L0: `397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04`.
- Contrato r3: `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`.
- Design r3: `854942ed6ef360af29fc021a0919e1300feb09e826ee5f4e9e8192488c0ed918`.
- Fixtures independentes `p1339-ab-closed-api-fixtures-r1.json`: `dda8618c9ede3640fab05d6cd4278f6c766c3410e4a646561f4bd32436f20dd5`.
- Interface pública/mecânica: `7ff05129bd9213ffbe1a9c72b403262559834c782bb9f10928ea7d20bfd02f8d`.
- Inventário de declarações baseline: `d0ab8a2a2297a8f8fcff36ac372a8b418368b814c92cf25184fbe17f7efd101f`.

O arquivo independente foi lido integralmente em partes antes da tradução; nenhuma fonte, ordem, valor esperado ou predicado foi corrigido. A revisão final R3 só alterou política de fases/causalidade, não estas chamadas aprovadas. Os pins de assinaturas baseline estão na interface original. Fontes adicionais para a projeção de diagnósticos foram `entities/source_result.rs` e `entities/span.rs`; o adaptador usa exclusivamente seus campos/métodos reais e match explícito de Severity/Tracepoint.

## Saída

`p1339-mutant-closed-state-api-harness.rs`, SHA-256 `09c467410cb0f4e37d11dca47c15f4913271f74e7b1400457513d76cd9c373c0`.

Teste previsto: `p1339_closed_apis_frozen_matrix`. Ele executa as 26 fixtures nos quatro perfis e três ordens (312 células **planejadas**, zero executadas aqui). A ordem `repeat` reconstrói tudo; `reverse` inverte as fixtures. Todos os outputs `P1339_CLOSED_API` deverão ser preservados integralmente pelo runner final, juntamente com argv/configuração/binário/UTC/canais da execução Cargo. Falha ou célula ausente não recebe sucesso implícito.

Integração futura exclusivamente test-only como descendente de `compiler::eval`, depois do selo:

```rust
#[cfg(test)]
#[path = "../../../../00_nucleo/diagnosticos/p1339-mutant-closed-state-api-harness.rs"]
mod p1339_closed_api_adapter;
```

Comando final previsto, no target coordenado pelo implementador/verificador:

```text
cargo test -p typst-core --release --locked p1339_closed_apis_frozen_matrix -- --nocapture --test-threads=1
```

O package `typst-core` foi confirmado em `01_core/Cargo.toml:2`. O adaptador não executa Cargo por conta própria enquanto o alvo produtivo está ausente.

## Transformação mecânica

- Cada expressão declarada recebe Source parse_code real e FileId próprio em um World de memória imutável. Sources coexistem durante corpo e replay; diagnósticos mantêm raw Span, origem, range, linha/coluna, mensagem, hints, severidade e trace ordenado.
- Bindings são avaliados uma vez em ordem e inseridos no scope real. `clone_of` clona snapshots reais, sem reavaliar bindings/closures; overlays alteram somente carriers/registries especificados.
- Locations simbólicas são alocadas por Locator; `label:name` consulta o label real daquele snapshot. Missing symbol/operação/tipo falha fechado.
- Snapshot de Content usa o entrypoint `pure` ou `runtime` que a fixture declarou. Nenhuma ausência é convertida em snapshot válido artificialmente.
- Um EvalContext real é criado para o corpo. Seu Result permanece separado do getter, de cada validação e do resultado de diagnóstico. Três chamadas são para as APIs reais aprovadas; nenhum comparador ou estabilizador existe no harness.
- Sinks de construção/corpo/validação/diagnose são distintos e todos projetados na saída. O corpo não é reexecutado para criar um resultado conveniente. O adaptador somente aplica os predicados presentes no JSON independente; chave de predicado desconhecida não é ignorada.

## Limites e verificações efetivamente feitas

`rustfmt --edition 2024 p1339-mutant-closed-state-api-harness.rs` terminou com exit 0 e formatou a fonte, confirmando análise sintática Rust. Não se reivindica type-check, link, execução de API futura, RED ou GREEN. Não foram usados stubs para fingir que as APIs já existem. Qualquer problema de ligação/tipo posterior deve ser registrado e auditado sob a política de binding, sem adaptar expectativas ao produto.

A projeção JSON de Value serve apenas para resultados públicos/predicados desta suíte: folhas/arrays/dicts são tipados (Float usa bits), demais resultados carregam tipo e repr pública. Não é comparador privado, serializer exaustivo de Content ou prova de F06. A representação completa de diagnostics/spans é separada e exaustiva para os carriers baseline declarados.

Esta fonte **não** observa a variante privada Same/Different/Unproven, nem o ciclo integral de produtor/retention/attempts. Portanto não fecha F06, F07 ou F08; marcações de matrix_ids nas fixtures não concedem essa cobertura ausente. As portas adicionais e seus predicados independentes precisam de artefatos próprios, congelados antes do candidato, com ligação posterior real auditada pelo verificador.

## Veredito

Nenhum. O verificador independente decide suficiência, congelamento, ligação e execução. Este recibo não é selo e não altera nenhuma obrigação final.
