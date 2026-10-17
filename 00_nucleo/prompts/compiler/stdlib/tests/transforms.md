# L0 — compiler/stdlib/tests/transforms
Hash do Código: 3999f6c2

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/transforms.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the transforms regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## Operações numéricas puras — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/stdlib/tests/transforms.rs` nas linhas 80 contém as chamadas sinalizadas como disallowed_methods pelo Clippy vigente. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. `00_nucleo/diagnosticos/geometria-numerica-ci-2026-10-13/before.json.gz` conserva fonte, owner, configuração, libm, compilador, UTC, status e diffstat. A sonda host-math compara os backends antes da escolha: 4968 casos, 69 diferenças de até 1 ULP em trigonometria/atan2, sem mudança de classe numérica ou sinal de zero; potências e hypot coincidem na amostra. É medição deste host, não prova universal nem nova paridade contra vanilla.

### Decisão e aceitação

O teste de skew com 15 graus usa o valor analítico tan(15°)=2-sqrt(3) como expectativa independente, conservando a tolerância 1e-9 e a verificação de 45 graus. Não chamar a mesma função trigonométrica do candidato para formar o oráculo.

Mudança interna em fluxo contínuo ADR-0127, sem contrato público novo, default ou fase. Preservar testes de geometria/linguagem existentes e comparar os exemplos SVG medidos antes/depois, declarando diferenças. A comparação geométrica não certifica todos os casos ou plataformas. Validar build, todos os targets, core, Clippy e linhagem. Não suprimir o lint nem substituir aceitação por identidade de bytes do compilador.

## Transformações tipadas — X1/X2 aprovados em 2026-10-14

### Medição anterior à decisão

O corpus em `00_nucleo/diagnosticos/transformacoes-2026-10-14/results.json.gz`
registra identidade/presença perdida, named x afetando y e casts incorretos.
Controles suplementares confirmam body string/none, factor depois de body,
erro de dois auto apenas no layout e campos omitidos distintos de explícitos.
O manifesto 960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d
foi aprovado; recibo no diretório materialization/transformacoes-tipadas-2026-10-14.

### Contrato dos testes

Testar operações por campos e identidade da linguagem; não exigir matriz
antecipada. Substituir fixtures antigas que aceitam Float como Angle/Length/Ratio,
rotate angle nomeado ou dois fatores scale: o pin rejeita essas chamadas.
Cobrir omissão, zero, false, auto, eixos parciais, units, sobras e spans causais,
body string/none e mapas que preservam operação. X3–X5 continuam no contrato
aprovado e serão completados durante esta mesma frente de materialização.
