# P1214 — paridade dos métodos públicos de `bytes`

**Resultado:** `BYTES METHODS PARTIAL — MAP RELATION REMAINS PARTIAL`  
**Data:** 2026-08-26  
**Regime:** protocolo completo numa única sessão, **sem atestação de isolamento**.  
**Vanilla ratificado:** `a51e02804`.

## Veredito

`bytes.len`, `bytes.at` e `bytes.slice` foram materializados no dispatcher
público. Os 22 casos de valor da matriz A/B são iguais ao vanilla, incluindo
UTF-8, índices negativos, default, conteúdo de slice, precedência de `end`
sobre `count:` e wrapping extremo de `i64`.

Os 21 casos de erro preservam exit status e mensagem central, mas não o
diagnóstico integral: o vanilla imprime o span da expressão e o cristalino
recebe `Args` com localização destacada neste caminho. Como posição é
observável do diagnóstico, a relação `language-bytes` permanece `parcial` e a
fila P1213 passa de `MISSING` para `PARTIAL`, não `RESOLVED`.

## Cadeia causal e RED

- A — contrato: fonte vanilla e 43 oráculos congelados antes do patch;
- B — ataques: 12 mutantes contratuais possuem testemunha na matriz;
- C — implementação: dispatcher mínimo em `compiler/stdlib/collections.rs`;
- D — A/B: 43 expressões contra os dois binários;
- E — veredito: promoção recusada devido aos spans.

O RED foi reproduzido por
`cargo test -p typst-core p1214_bytes --no-run`: sete referências aos helpers
ainda inexistentes falharam com `E0425`. Depois da implementação, os quatro
testes focais passaram. Os 12 ataques declarados são discriminados pelos
oráculos, mas não houve motor externo de mutation testing; não se alega score
independente.

O primeiro conjunto de sondas de conteúdo usava `array(bytes.slice(...))`.
Ele foi invalidado porque o construtor público `array` é uma lacuna alheia no
cristalino. Os casos afetados foram recongelados primeiro no vanilla como
igualdades `bytes.slice(...) == bytes(...)`, preservando a inspeção do conteúdo
sem misturar outra feature.

## Proveniência

Congelamento inicial em `2026-08-26T14:43:00-03:00`; gates finais em
`2026-08-26T14:56:16-03:00`:

- HEAD cristalino `dc47c9c32b8b6769a58622c98b885cb094337508` + working tree P1212–P1214;
- vanilla SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino release final SHA-256 `d42bafa253a475b07884dfa14c588c7a2d9bccde5d2851fe9ea3cda36b1ed203`;
- lente SHA-256 `9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`;
- L0 `bytes.md` SHA-256 `bd6199460444e5c82754a5d7050733bd0a5c36bf58d907855287fd1b6b539a7f`;
- L0 `collections.md` SHA-256 `cffec50a0d60f842dffca39138a7653db36fd87bbde90bcec37e01a9e51c8d70`;
- oráculos SHA-256 `4c5ec60d2f1e8c70d488109b2d9f79b7ba783a213a581f645f01aad6e72d6411`;
- resultados SHA-256 `9b2ff8362e4f890198e1fe6ef8ffa9c31a2b151301ba26dfea3a354f07896a45`;
- mapa final SHA-256 `f87270d8aa5b203aee434272e98143bc580da69f8851adb53a879ccb30284459`.

## Gates

- focais P1214: 4/4 passaram;
- matriz A/B final: 22 `GREEN`, 21 `PARTIAL-SPAN`, zero diferença de mensagem central;
- `cargo build --workspace --quiet`: passou;
- `crystalline-lint .`: passou, sem violations; avisos informativos preexistentes permanecem;
- `git diff --check`: passou;
- lente com mapa: passou duas vezes, saída byte a byte idêntica, SHA-256
  `dccbed37b74a7bd938cefa4695de71bdb23b46e9979ec6e0baf7c07c2416dab1`;
- `cargo test --workspace`: uma falha temporal em
  `p1137_watch_dependencias_recuperacao_e_filtro` após timeout de 20 s; a
  repetição isolada passou (1/1). Nenhum teste P1214 falhou.

## Próxima obrigação

Preservar spans no caminho de chamadas de método de `bytes` sem ampliar a
superfície pública. Só depois dessa matriz integralmente verde a fila pode ser
marcada `RESOLVED` e o mapa promovido. O cluster funcional seguinte continua
`operator-diagnostics`, mas a dívida diagnóstica de bytes permanece explícita.
