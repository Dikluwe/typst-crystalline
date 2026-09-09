# P1325 — RED r0 rejeitado por instrumentação

Veredito `NOT_RED_COMPILE_FAILURE`. Regime A/B sem atestação técnica de
isolamento. Recibo julgado `p1325-unit-red.json`, SHA-256
`1bf630283a87ed40d99ad612c47042dc6d226f0c63c6cde19ed6822f3918a22a`,
execução `2026-09-09T00:57:33.784119+00:00` a
`2026-09-09T00:57:49.344876+00:00`. O recibo guarda HEAD, diff/stat e
inventários antes/depois.

O comando cargo test terminou em exit 101 por E0432: import inexistente
`crate::entities::route::Route`. Não compilou o test binary, portanto nenhuma
assertion foi executada. A falha não prova divergência semântica do produto
e não satisfaz o gate RED. É necessário o autor corrigir a instrumentação
para o contrato público existente `entities::world_types::Route`, registrar
novo hash, integrar literalmente e repetir RED antes de C.

O snippet congelado de SHA-256
`8f281a54ca706bf2f8a98c50c8e3758f290d999794e4b19dd929498f4f53fcfb`
foi lido pelo revisor e conferido como sufixo literal do consumer. Remover
esse sufixo e o header de hash reproduz exatamente o baseline produtivo,
sem mudança candidata. Nenhuma assertion privada foi comunicada ao
implementador. O revisor não alterou teste nem produto.
