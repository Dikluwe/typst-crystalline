# L0 — compiler/eval/call_dispatch/type_fields
Hash do Código: 87cda106

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/type_fields.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:604-721` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Resolver os campos chamáveis estáticos dos tipos, mantendo identidades nativas e as funções expostas. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

### Medição anterior à decisão — catálogo V16, 2026-10-17

lab/results/construction/diagnostics/tekt-v16-catalogos-2026-10-17/before.json.gz e
before-measurement.json.gz fixam HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado, UTC/status/diffstat, fontes/L0/config e binários.
O inventário contém118 campos,115 exercitados pelo CLI e três Location por
teste no módulo. Há159 pares/318 invocações:141 coincidem em código/stdout/
erro principal+hints,18 divergem no diagnóstico já existente. As funções/
constantes inventariadas exercitadas são expostas nos dois backends. Isso
não prova que todo campo omitido deva estar ausente upstream. O pin é
9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Type tem38 variantes e Value39 no value.rs deste snapshot. Os wildcards
sinalizados misturam família fechada e nome aberto de campo/método. O estado
e as localizações anteriores estão no inventário e no before.json.gz.

### Decisão deste owner

Combinar as duas consultas antigas (constantes e funções) em um match exaustivo
de Type. As nove famílias Direction/Alignment/Angle/Function/Duration/Length/
Selector/State/Location possuem lookup interno por nome str aberto. Manter
constantes e nomes, identidade nativa, Native versus NativeWithEngine e
assinaturas; campos de mesmo nome em tipos diferentes não se confundem.
Nome desconhecido retorna None somente dentro da família já classificada.
O grupo V19 das outras29 variantes Type é coeso: este owner não fornece
catálogo para essas famílias, que continuam para os outros owners/caller.
Não significa que o tipo não tenha nenhum campo na linguagem. Enumeração
explícita é obrigatória: uma variante Type nova exige classificação local.
Não converter tipos em strings para fazer o despacho nem suprimir V16/V19.

### Aceitação e classe

Correção interna estrutural ADR-0127 em fluxo contínuo, sem nova API, default,
compatibilidade ou fase; intenção de runtime preservado, não nova certificação
universal de paridade. O RED é estrutural: padrões produtivos antigos aceitam
variante futura; após a mudança, o controle atual compila e a variante futura
é rejeitada E0004. RHS apagados nos probes apenas para isolar cobertura de
padrões; payloads são stubs, não testes de equivalência de execução. Não
mutar workspace/target Cargo para testar variantes futuras. Os controles
funcionais no módulo passam antes/depois; não alegá-los RED funcional.

Quatro V16 dão lugar a três seleções de família fechada (o primeiro owner
combina seus dois matches). Verificar159 pares antes/depois com stdout/
stderr completos preservados dentro de cada backend, inclusive18 negativos
já divergentes. Retê-los no inventário para correção própria. Nenhuma supressão
ou relaxamento global do linter. Testar identidades de funções de mesmo nome,
constante versus função, misses cruzados e Location. L0 primeiro, selos só
dos dois owners após V15/V26 válido; build/suíte workspace/Clippy -D warnings
/-D unfulfilled_lint_expectations, Rustdoc/quatro índices, formato/diff,
linhagem estrita/reversa e SARIF diferencial sob12GiB/high10GiB/swap0/jobs1.
Sem commit/push, merge ou release.
