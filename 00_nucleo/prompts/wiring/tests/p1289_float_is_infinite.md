# Prompt L0 — `wiring/tests/p1289_float_is_infinite`
Hash do Código: 9cb55bde

**Camada:** L4 — teste de integração
**Ficheiro alvo exclusivo:** `04_wiring/tests/p1289_float_is_infinite.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Propriedade

Este prompt possui exclusivamente o teste black-box acima. A semântica de
float pertence ao owner produtivo L1.

## Medição anterior à decisão

Os observáveis de `float.is-infinite` são exercidos por
`lab/surface-inventory/run_p1289_oracles.py`; o integration test conecta esse
runner ao binário Cargo sem duplicar a semântica numérica.

## Contrato

O consumer localiza runner e baseline, executa `CARGO_BIN_EXE_typst`, exige
exit bem-sucedido e veredito `Preserved`, e rejeita `Unknown` ou `Violated`.
Não escreve baseline, não normaliza falhas e não reimplementa observáveis.

## Aceitação

O teste passa com runner e baseline íntegros; ausência, erro de execução ou
resultado não preservado falha explicitamente.
