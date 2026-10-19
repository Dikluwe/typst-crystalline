# L0 — stdlib/calc/trig
Hash do Código: d735b2ac

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/trig.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:601-767` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

Implementar sin/cos/tan e inversas asin/acos/atan/atan2 com AngleLike, ordem pública x,y de atan2 e guards de domínio/resultado. Manter helpers unários compartilhados pelas famílias transcendentais.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/core/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.

## Matemática determinística — medição de 2026-10-13

Estado: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. Fontes, SHA-256, UTC, status e diffstat estão em
`lab/results/construction/diagnostics/matematica-ci-2026-10-13/before.json.gz`; sondas de
linguagem e identidade dos binários estão no recibo `evidence.json.gz` desse
mesmo diagnóstico (entrada `before-probes.json`). O oráculo é o pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 ratificado
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Medição anterior à decisão: `clippy.toml:3-24` proíbe as transcendentais
nativas por dependerem da plataforma. `01_core/Cargo.toml:42` e
`lab/typst-original/Cargo.lock:1620-1623` fixam ambos libm 0.2.11.
`lab/typst-original/crates/typst-library/src/foundations/calc.rs:313-608` e
`lab/typst-original/crates/typst-library/src/layout/angle.rs:81-113` usam
libm nessas famílias. As sondas mostram mesmos tipos, quadrantes e rejeições
de domínio nos casos medidos, com diferenças nos últimos dígitos de cosh(1),
exp(1) e log(81, base: 3). Mensagens de erro já diferem entre as implementações.

Decisão: correção interna contínua (ADR-0127). Usar as funções puras de
libm 0.2.11 já pinada, sem dependência nova. Retirar as exceções locais a
`disallowed_methods` deste consumer. A whitelist nomeia as funções adicionais.
A escolha da implementação transcendental é mecânica; tipos, coerções,
valores numéricos, ordem dos argumentos e erros são observáveis da língua.
Não exigir igualdade geral de bits nem inferir paridade integral das sondas.
Não mudar assinaturas, bindings, guards, mensagens de erro, defaults ou fase.
Preservar as extensões já declaradas em `00_nucleo/prompts/core/compiler/stdlib/calc.md`.

Aceitação adicional: testes com constantes independentes e tolerância explícita,
tipos de resultado, quadrantes e domínios; regressões existentes de calc;
Clippy sem métodos proibidos nem exceções neste consumer; build e linhagem.
O RED do gate de qualidade antecede o reparo; não fabricar uma falha de língua
quando o caso já passa na plataforma atual. Sondas posteriores devem registrar
também diferenças preexistentes que esta correção não cobre.
