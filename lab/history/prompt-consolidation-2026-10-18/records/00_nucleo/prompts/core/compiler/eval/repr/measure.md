# L0 — compiler/eval/repr/measure
Hash do Código: 940ecf77

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/decimal-rounding.toml sha256:46c39d0f408e354d91f39bd6f78bd5b77707b4d9f982b3dff9bfa9140d1ecba4

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/measure.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:202-329` possui o comportamento separado. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Medição de arredondamento (2026-10-13)

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado, `01_core/src/compiler/eval/repr/measure.rs:72-78` multiplica
sempre pela potência decimal antes de arredondar. No binário medido,
`repr(1e308em)`, graus e percentagens grandes tornam-se infinitos e seis
sondas de `eval(repr(value), mode: "code")` perdem o valor; as mesmas
sondas preservam tipo e valor no oráculo pinado
`9f2b6e8715237cb086899a42873660fe744622e8`.
Snapshots, UTC, diffstat, hashes e sondas anteriores à decisão:
`lab/results/construction/diagnostics/representacao-medidas-ci-2026-10-13/before.json.gz`
e `probe-before.json` no arquivo `evidence.json.gz` desse diretório.
A fonte upstream `lab/typst-original/crates/typst-utils/src/round.rs:26-60`
protege mantissa preenchida, precisões extremas e escala negativa;
`lab/typst-original/crates/typst-library/src/foundations/repr.rs:95-130`
aplica esse arredondamento à representação com unidade e componentes.

Comprimentos `1e307pt` não são prova bilateral para esta correção:
`lab/typst-original/crates/typst-library/src/layout/abs.rs:32,265-273`
transborda ao multiplicar o valor por 127 na construção; a divisão por
`1pt` já é não finita no oráculo e finita no cristalino antes de `repr`.
Essa diferença de construção permanece fora deste escopo. A forma `0pt`
de `float.nan * 1em` também não demonstra a política do formatador isolado.

## Decisão e contrato de arredondamento

Correção interna de paridade sem nova assinatura, default ou fase (fluxo
contínuo ADR-0127). Aplicar as claims do núcleo decimal compartilhado;
este owner formata a representação e conserva sua política própria:
NaN e infinitos de entrada passam inalterados pelo arredondamento; overflow
real do resultado continua sendo infinito, representado na forma existente.
A precisão interna continua i32; os limites são tratados antes de negar
precisão, incluindo i32::MIN. Os chamadores produtivos mantêm duas casas
com unidade e três para componentes. Não introduzir I/O, dependência nova,
API pública ou função compartilhada com calc.


## Contrato

Formatar Duration, Relative, tamanho math.lr, Length, Ratio e Angle, incluindo arredondamento e unidades. Preservar sinal, partes explícitas, zeros e formas não finitas segundo a representação vigente.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/core/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.

Regressões: a representação de medidas finitas grandes deve continuar
finita e preservar o valor quando reavaliada; confirmar as seis sondas
RED→GREEN contra o oráculo e conservar o controle em 2^53. Cobrir também
componentes, precisão i32 extrema, escala -308, zero com sinal, empate,
formas não finitas e overflow real. A identidade de bytes Rust ou do
algoritmo não é critério de aceitação da linguagem.
