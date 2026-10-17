# L0 — stdlib/calc/round
Hash do Código: f89e7df1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/decimal-rounding.toml sha256:46c39d0f408e354d91f39bd6f78bd5b77707b4d9f982b3dff9bfa9140d1ecba4

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/round.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:360-516 e 1002-1051` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

### Medição e decisão de arredondamento finito (2026-10-13)

Antes desta decisão, no HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`
com working tree não commitado, `01_core/src/compiler/stdlib/calc/round.rs:67-69` truncava `digits` de i64
para i32 e `01_core/src/compiler/stdlib/calc/round.rs:117-119` multiplicava por uma potência decimal mesmo
com precisão negativa. Os snapshots integrais, UTC, diffstat e hashes dos
binários estão em `00_nucleo/diagnosticos/arredondamento-ci-2026-10-13/`.
O oráculo pinado `9f2b6e8715237cb086899a42873660fe744622e8` satura a precisão
por tipo (`lab/typst-original/crates/typst-library/src/foundations/calc.rs:923-949`)
e evita overflow intermediário (`lab/typst-original/crates/typst-utils/src/round.rs:26-60`).
As sondas semânticas comparam tipo e valor na linguagem: 24 passam no oráculo;
15 falham no cristalino antes da correção. A notação textual de floats grandes
não é usada como critério de igualdade. Proveniência: `probe-before-refined.json`
arquivado em `evidence.json.gz`; o primeiro ensaio com literal i64 mínimo
foi substituído pela expressão inteira válida em ambos os parsers.

Classificação: correção interna de paridade de valores finitos e precisão
inteira, em fluxo contínuo ADR-0127; sem nova assinatura, default ou fase.
`digits` aceita i64 e é saturado a i16 para Int/Float e i32 para Decimal,
nunca convertido por truncamento modular. O arredondamento de Float finito
segue as claims do núcleo decimal compartilhado consumido abaixo.

A política categórica atual de não finitos permanece: precisão zero usa
`round`, precisão >=15 devolve o operando e precisão <-308 usa `value * 0`.
No intervalo restante o resultado passa por `guard_float`, inclusive overflow
real; a nova preservação de valores grandes exige operando finito. Não ampliar
a propagação IEEE nem declarar paridade total de diagnósticos neste passo.
Floor, ceil, trunc e fract mantêm o contrato anterior.

Implementar floor, ceil, round, trunc e fract com tipos Int/Float/Decimal preservados nos casos contratados. Precision inteira negativa arredonda away from zero com overflow checado.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.

Regressões devem chamar calc.round pelo mesmo caminho de argumentos da função
nativa e demonstrar RED→GREEN para valores grandes e digits extremos nos três
tipos. Verificar também empate, zero com sinal e política de não finitos.
As 24 sondas de tipo/valor devem passar nos dois binários; controles de erro
do cristalino devem manter seus diagnósticos. Não comparar a implementação
com uma cópia da própria fórmula como teste.

## Medição e compartilhamento declarativo (2026-10-13)

`01_core/src/compiler/stdlib/calc/round.rs:104-124` já implementa as
regras finitas protegidas; a medição anterior à extração declarativa está em
`00_nucleo/diagnosticos/representacao-medidas-ci-2026-10-13/before.json.gz`.
O diagnóstico de `repr` demonstra a mesma obrigação de preservar finitos
na representação. Compartilhar apenas essas invariantes em núcleo ADR-0129;
a política `guard_float`, conversões por tipo e o corpo Rust de calc não
mudam. A distinção entre erro de calc e propagação de repr permanece local.
