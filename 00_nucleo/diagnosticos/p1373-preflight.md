# P1373 — preflight do gate ADR-0127

## Resultado

A autoria L0 está completa e a implementação não começou. O gate é obrigatório
porque `BlockElem::height` é campo público. Não houve Cargo, rustc, teste,
matriz, rede ou processo de produto nesta fase.

## L0 congelado para confirmação

| Owner | SHA-256 do prompt proposto |
|---|---|
| `entities/elements/block.md` | `59f5bdd0c2b76a5f9c7b80db54c38bd5810df9460f5c2ddf85e20b5b083e9be3` |
| `entities/content.md` | `5cae8e4a6656c888f5d46a3a71616af3501eb1f527b1bd06c7b6de316ab91ed4` |
| `compiler/stdlib/layout.md` | `5eb467083301a4700ea94af1be51c882dd51410f79b6a0e0079d8053d10067d3` |
| `compiler/layout/block.md` | `40094c329e6a62b272adf2211e6f370d087ed6861497ad2a90c9a8024d868cc8` |

O código permanece no estado anterior. Seus hashes medidos antes da proposta
são, respectivamente, `dc6abff7…30219c6`, `3f2d8fd6…845c66`,
`495b7865…47aa2` e `635135fa…d1857`. Portanto os headers de linhagem não foram
resselados prematuramente contra comportamento ainda inexistente.

## Regime

Protocolo completo planejado, executado sem atestação de isolamento. A mesma
autoridade produziu proposta, contrato e oráculos; nenhum certificado de
independência será alegado. Após confirmação, contrato/oráculos serão selados,
o RED será demonstrado e somente então a implementação poderá começar.
