# P1324 — campos ausentes em funções nativas com namespace

## Problema medido e alcance

`json.nope` publicava `function does not contain field "nope"` e destacava
`json.nope` inteiro. No vanilla ratificado upstream `a51e02804`, o erro nomeia
`json`, usa backticks e destaca somente `nope`. A medição fresca reproduziu
a mesma causa em yaml, toml, cbor, assert e table, além de alias, With aninhado,
parênteses e chamada de campo ausente. Não é criação de campos ou namespaces.

O passo sucede P1323, que fechou apenas o warning experimental HTML. A fila
P1322 foi usada como hipótese; a decisão nova se apoia na medição P1324 e na
inspeção do owner, de Func e das fontes vanilla. O L0 proprietário
`compiler/eval/bindings/field_access.md` substitui expressamente a proteção
P1311 do diagnóstico Some ausente; as outras obrigações permanecem.

Status final: **implementado e aprovado no recorte do L0**, conforme
`p1324-review-final.md`. A suíte A/B bruta não é integralmente GREEN:
as doze falhas de ordem de avaliação permanecem registradas e não recebem
crédito de paridade. O suplemento comprova sua preservação, exigida pelo L0.

## Proveniência

Working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`. O baseline fresco foi concluído em
`2026-09-09T00:04:40.172521Z`; `p1324-baseline.json` conserva cada expressão,
argv, UTC, stdout/stderr, exit, diff/stat, inventário e owner/L0 originais.

- Manifesto `p1324-manifest.json`: SHA-256
  `414bdf11ea6cc34565e8db0dbc4fb7966685d62abbd41195f40a3ae1ea985a2d`.
- Baseline P1323 `/tmp/p1323-target.9NrOxY/release/typst`: SHA-256
  `f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`.
- Vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato final `/tmp/p1324-target.x5jDkw/release/typst`: SHA-256
  `c5c9aa39c8b7053a19f53bf9f37c8d3731a4081683a51cf8ca5e9e9c0197d2f7`.
- Corpo normativo congelado: SHA-256
  `4b85d3606fc3dc9e1390449bb856b0fcde5959b644294ba6f728c04b33f807c3`.
  Exclui somente a linha canônica recíproca `Hash do Código:`.

O target `/tmp/p1324-target.x5jDkw` é uma cópia dedicada sem hardlinks.
Não foi repetida a tentativa RAM: a diferença entre as visões de espaço do
host/sandbox registrada em P1323 impede confiar na leitura isolada do sandbox.
Nenhum temporário anterior foi apagado.

## Decisão e implementação

Regime contínuo ADR-0127: mensagem e origem são observáveis de linguagem;
nome/categoria/AST já bastam no mesmo owner. Não há novo contrato público,
default ou fase. O parecer `p1324-review-l0.md` aprovou a obrigação antes da
correção, sem aprovar antecipadamente o resultado.

A implementação ficou restrita ao par L0/owner de field_access. O helper
privado projeta o nome de qualquer nativa, e o lookup só emite o erro após
confirmar a ausência; a escolha de field.span() inclui a categoria Some.
Nenhuma identidade é inferida apenas de um nome. As seis
alterações dirty anteriores em loading, call_dispatch e wiring, seus L0 e
os artefatos anteriores ficam preservados. Namespace Some vazio ou populado
e Native/NativeWithEngine, inclusive With, recebem o diagnóstico nominal.
Lookup presente conserva valor/chamada. None e categorias não nativas,
Module, Dict, Type, snapshots, PDF feature-gated, text contextual e float/is-nan
mantêm suas obrigações. Preservação de divergência excluída não é paridade.

## Testes independentes e incidente de integração

A skill `tekt-materializacao-segregada` orienta regime A/B, sem selo de
refinamento. Root redige intenção/L0 e implementa; `/root/p1324_tests` recebe
L0, vanilla e contratos públicos, sem código candidato; `/root/p1324_review`
audita sem editar julgados. Ambiente compartilhado, sem atestação técnica de
isolamento. As calibrações do observador não são mutantes produtivos.

O snippet unitário foi congelado em `p1324-ab-unit-freeze.json` antes do
candidato. A integração literal não exibiu seus testes privados ao
implementador. As duas expectativas legadas de Some no teste P1311 foram
sucedidas explicitamente; construtores, variantes e controles de sucesso
permaneceram. O original completo está no baseline e no snapshot do revisor;
`p1324-test-succession.json` registra a transição.

A primeira aplicação via diff foi rejeitada sem escrita. Um comando RED
foi iniciado antes da integração efetiva; ele executou zero testes filtrados
e retornou sucesso. `p1324-unit-red.json` é preservado como falha de
instrumentação, NÃO como RED/GREEN. A execução sucessora com source estável
é obrigatória antes de qualquer correção semântica. Nenhum resultado dessa
primeira tentativa fecha a obrigação.

O sucessor `p1324-unit-red-r2.json` executou cinco testes com inventário
produtivo idêntico antes/depois: duas falhas reais de asserção, três controles
aprovados, exit 101. `p1324-review-red.md` confirmou o RED antes do candidato.
O snippet original permanece congelado, sem adaptação à implementação.

`p1324-unit-green.json` executou os cinco testes novos e os cinco controles
P1311: dez passaram, nenhum falhou. O conjunto cobre Some vazio/populado,
ambas as categorias nativas, With, nomes qualificados, sucesso e categorias
não nativas. O snippet integrado ainda precisava de formatação mecânica;
`p1324-fmt.json` registra essa falha, sem interpretá-la como falha semântica.
O snippet bruto congelado foi preservado. O revisor reproduziu rustfmt e
confirmou igualdade exata da forma formatada com o bloco integrado, SHA-256
`bd351888c15a75cd043b92edd4a6517badde68264024933fc3ef468864798a60`.
Os gates finais abaixo foram executados após essa formatação, sem mudanças
no inventário produtivo durante o build e os testes.

Na suíte CLI, o primeiro adaptador passou `--diagnostic-format human`, opção
rejeitada pelo cristalino. Essa execução é falha de instrumentação, não
divergência semântica. O adaptador V2 preservou casos/oráculos e confirmou
os vetores vanilla com default human; invocação rejeitada passou a Unknown.
As perdas de captura das tentativas iniciais estão registradas em
`p1324-ab-baseline-findings.md`; nenhuma embasa RED ou aprovação.

## Resultado bilateral e limites encontrados pelos testes

A suíte V3 tem 50 casos nos quatro perfis. O candidato foi executado em
ordem normal, repetida e inversa: 600 processos, de
`2026-09-09T00:26:14.077335Z` a `00:27:09.506285Z`, conforme
`p1324-ab-candidate-v3.json`, SHA-256
`0a9ef0dc59229a575690b003b786a8642c1c2d4339e5eed5519c0fa7485fbb72`.
O recibo preserva stdout/stderr integrais, exit, argv, ambiente, HEAD/diff/stat
e executáveis. Não há normalização ampla dos diagnósticos.

**Impacto observado:** 204 vetores mudaram em relação ao baseline, e todos
esses 204 coincidiram integralmente com o vanilla. Os outros 396 conservaram
o envelope do baseline. As 400 comparações de repetição/inversão do candidato
foram estáveis. O baseline foi medido/pinado, não repetido nas três ordens;
não se alega estabilidade de três ordens do baseline.

O resultado bruto da obrigação V3 é **588/600 Preserved, 12 Violated, zero
Unknown**, e continua assim no recibo. As doze falhas correspondem ao único
caso `json.nope(panic("argument-must-not-run"))`, nos perfis e ordens. Baseline
e candidato publicam o panic do argumento; vanilla acusa primeiro o field.
A diferença já estava na medição baseline V2, mas o autor dos testes não a
separou antes da execução candidata. Isso foi uma lacuna de classificação,
não regressão introduzida pelo patch nem motivo para ocultar a falha.

A inspeção independente encontrou a causa em
`01_core/src/compiler/eval/call_dispatch.rs:1331–1348,1585`: avaliação de args
antes do lookup na tentativa de despacho de coleção. Esse owner está
intocado neste passo e o L0 P1324 exige explicitamente preservar a ordem de
avaliação. O suplemento `p1324-ab-order-preservation-supplement.json`, SHA-256
`042e006c4e04ba46ba645f33d3de2d4550eb8f57fec72c90aae91360b5f06982`,
confrontou os doze vetores completos e confirmou 12/12 baseline=C, sem novos
processos. É uma análise posterior ao resultado candidato, não um novo freeze
prévio. V3 não se torna integralmente GREEN e não se concede paridade ao
caso. Não houve correção de call_dispatch nem mudança de fase/ordem.

Outras duas fixtures originais acessavam `.body` depois de `table.cell[ok]`,
incluindo With. Isso testa Content, não o lookup de namespace. O baseline já
falhava; após parecer `p1324-review-ab-reopen.md`, V3 conservou os literais
como controles baseline=C e acrescentou positivos de chamada/tipo. Sucesso
de lookup/chamada ficou exercitado sem fabricar o field Content ausente.

V3 foi criada depois de existir candidato, mas antes de o autor A/B observá-lo,
apoiada exclusivamente em L0 e baseline/oráculo. Não se alega freeze integral
V3 anterior à implementação. A hora copiada incorretamente no campo `at` do
freeze foi retificada em `p1324-ab-freeze-v3-timing-correction.md`: criação
efetiva `00:25:27Z`, primeira observação focal C `00:25:28.192259Z`; o freeze
original permanece imutável. O alerta preliminar sobre coluna de csv também
foi refutado e corrigido no relatório do autor, sem mudar o comparador.

## Gates finais e linhagem

Cada recibo contém a working tree/diff integral antes/depois e UTC; as
contagens abaixo pertencem ao estado formatado pinado, não ao HEAD isolado.

| Gate | Resultado e evidência |
|---|---|
| Build workspace release locked | exit 0; `p1324-final-build.json` |
| Workspace release locked | 6.672 aprovados, 0 falhas, 3 ignorados; `p1324-workspace-tests.json`, 2026-09-09T00:23:13.619472Z–00:26:01.242214Z |
| Formatação final | exit 0; `p1324-final-fmt.json` |
| Lint completo | 0 erros, 240 warnings, 1.138 infos; `p1324-final-lint.json`, 2026-09-09T00:23:47.726079Z |
| V5/V15/V26 com fail-on warning | exit 0, sem violations; `p1324-final-lineage.json` |
| Diff check | exit 0; `p1324-final-diff-check.json` |

O lint não está livre de avisos. O dry-run final disse `Nothing to fix` mesmo
com hash recíproco desatualizado, limitação já conhecida e novamente medida.
O cálculo canônico independente determinou `Hash do Código: c4567498`;
somente essa metadata foi corrigida após os gates funcionais, com recibo
`p1324-reciprocal-correction.json`. Norma, source compilado e binário não
mudaram; o hash efetivo do L0/header é `740a39f5`. O pin do Núcleo permaneceu
intacto. O gate de linhagem foi repetido depois da correção.

O diff final sobre HEAD inclui o trabalho anterior, não apenas P1324:

```text
.../prompts/compiler/eval/bindings/field_access.md |  66 ++-
00_nucleo/prompts/compiler/eval/call_dispatch.md   |  53 ++-
00_nucleo/prompts/compiler/stdlib/loading.md       | 198 ++++++++-
00_nucleo/prompts/wiring.md                        |  56 ++-
01_core/src/compiler/eval/bindings/field_access.rs | 258 +++++++++++-
01_core/src/compiler/eval/call_dispatch.rs         |  92 +++-
01_core/src/compiler/stdlib/loading.rs             | 466 +++++++++++++++++++--
04_wiring/src/main.rs                              |  30 +-
8 files changed, 1144 insertions(+), 75 deletions(-)
```

O recibo de fechamento identifica os novos diagnósticos/fixtures não rastreados
e verifica os hashes da evidência anterior. Nenhum arquivo anterior foi
apagado ou silenciosamente substituído.

## Limites

Este passo não resolve o diagnóstico de closures, fields de outros tipos,
members ainda ausentes ou os demais débitos P1322. Não declara paridade geral
de funções, HTML, PDF, ANSI ou da linguagem. Sem commit, staging ou push.
