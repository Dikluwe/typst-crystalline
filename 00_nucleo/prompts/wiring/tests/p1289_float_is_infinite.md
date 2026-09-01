# Prompt L0 — `wiring/tests/p1289_float_is_infinite` — contrato black-box P1289
Hash do Código: 00000000

**Camada**: L4 — teste de integração
**Ficheiro alvo**: `04_wiring/tests/p1289_float_is_infinite.rs`
**Origem**: P1289
**Contrato observado**: `C-P1289-FLOAT-IS-INFINITE-v3`
**ADRs**: ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Medição anterior à decisão

O primeiro verificador P1289 executou o teste com sucesso, mas
`crystalline-lint .` produziu V1 porque o consumer L4 não possuía `@prompt`.
O teste não pode partilhar o owner `foundations/float`, já materializado pelo
consumer produtivo, pois isso violaria a bijeção Prompt L0 ↔ consumer.

## Contrato

Este consumer é a ponte black-box entre a suíte Cargo e o oráculo independente
`lab/surface-inventory/run_p1289_oracles.py`. Ele:

- localiza runner e baseline a partir do root do repositório;
- executa o binário `CARGO_BIN_EXE_typst` como candidato;
- exige exit 0 e veredito `Preserved`;
- exige zero `Unknown` e zero `Violated`;
- exige `mutation_score` exatamente `1.0`.

O teste não reimplementa os onze observáveis, não escreve baseline/runner, não
normaliza falhas e não contém semântica numérica. Mudança nos artefatos A exige
novo selo e atualização explícita da cadeia; `Unknown` nunca vira sucesso.

## Verificação

`cargo test --workspace --test p1289_float_is_infinite`, V1/V5/V15/V26,
`cargo fmt --all -- --check` e `git diff --check`. Este L0 possui exatamente
este consumer e não é partilhado com produção.

