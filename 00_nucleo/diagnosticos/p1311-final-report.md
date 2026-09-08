# P1311 — diagnóstico de campo ausente em função nativa

**Concluído no escopo da coorte, com veredito independente PASS_SCOPED.**

## O que mudou

O acesso `csv.encode` agora informa
``function `csv` does not contain field `encode` `` e sublinha somente
`encode`. Antes, informava `cannot access fields on type function` e marcava
`csv.encode` inteiro. O mesmo vale para `read`, `xml` e a categoria de funções
nativas sem namespace, sem lista especial de nomes ou criação de encoders.

O problema estava no fallback de `field_access`: namespace None não distingue
uma nativa sem membros de uma closure/plugin. O helper privado discrimina as
variantes existentes, atravessa With sem executá-lo e obtém o nome público
da função subjacente. Por isso aliases mantêm `csv` e `calc.abs.nope` nomeia
`abs`. A seleção de span usa o identificador do campo nessa categoria apenas.
Em `eval("csv.encode")`, a mensagem muda, mas a propagação causal continua
marcando a string externa, conforme medido no vanilla.

A implementação está em `01_core/src/compiler/eval/bindings/field_access.rs`,
helper na linha 181, seleção de span na linha 271 e fallback na linha 420.
O L0 proprietário foi atualizado antes do código; hash de prompt `15b5f7af`,
Hash do Código `5a601982`. Não houve alteração em Func, dispatch, assinaturas,
fases ou namespaces. ADR-0127 aplicado como correção interna de paridade.

## Evidência e limites do fechamento

A suíte independente foi congelada antes do patch e revisada antes da
implementação. Nas 220 expectativas distintas, havia 96 erros-alvo divergentes
do vanilla e 124 controles de preservação. O candidato satisfaz todas;
repetição e inversão totalizam **660 comparações, zero falhas e zero Unknown**.
São comparações literais de exit/stdout/stderr, incluindo mensagem, marcação,
hints e traces, não uma redução do erro a um booleano.

O RED local teve três falhas de assertion legítimas e dois controles verdes;
o GREEN teve cinco testes passando. Os controles locais incluem namespace
Some vazio, NativeWithEngine, Plugin/Element/Closure nomeados como `csv` e
With. Esses casos Rust não são atribuídos à autoria independente do corpus CLI.
Fontes: `p1311-unit-red.json`, `p1311-unit-green.json`,
`p1311-ab-baseline-final.json`, `p1311-ab-freeze.json`,
`p1311-ab-candidate-runs.json` e `p1311-ab-comparison.json`.

P1310 foi preservado: **1.036 envelopes inalterados, zero violações/Unknown**
em `p1311-p1310-replay.json`, inclusive a política explícita de Symbol e a ordem
de validação de named. Contra o oráculo histórico P1308, que continua imutável,
o resultado bruto é **1.950 Preserved / 32 Violated / 0 Unknown**, não um replay
totalmente verde. O comparador `p1311-p1308-delta.py` demonstra que as 32 diferenças
são as 20 correções já aceitas de P1310 mais 12 de P1311 (`csv/read/xml.encode`
nos quatro perfis). Todas coincidem com o vanilla pinado; os outros **1.970**
envelopes são idênticos ao candidato P1310. Recibo `p1311-p1308-delta.json`.

Namespaces Some, inclusive vazios, closures, plugins e elementos customizados
mantêm suas mensagens anteriores; isso é preservação, não declaração de paridade.
Nos controles CLI, 68 observações já divergiam do vanilla e continuam iguais ao
baseline. Dict/Module/Type/Content, float/is-nan, contexto text e gates PDF não
foram ampliados. `text` builtin é Native None e pertence aos alvos; `assert` e
`table`, com namespace Some, pertencem aos controles. Essa classificação foi
corrigida na revisão **antes** do congelamento, não adaptada ao candidato.

Não se recalculou cobertura global nem se executaram as famílias pendentes de
mutação de produto de P1307. Não há quitação das demais coortes de P1309.

## Validação final

Build release do workspace, formatação/check, diff/check e crystalline-lint
concluídos com sucesso. Linter: **zero erros, 240 warnings e 1.136 infos**;
não se declara ausência de dívida. Resselo final dry-run: `Nothing to fix`.
Suíte completa `cargo test --workspace --release --no-fail-fast`:
**6.629 passaram, zero falharam, três doctests ignorados**. Os ignorados são
`layout_with_introspector` e `TagIntrospector::{inject_pages,inject_positions}`;
não são testes novos. Recibo `p1311-workspace-tests.json`, SHA-256
`9b2cd9ceaecec902d2ba85350e3628d67cbf3759c76c22d0954e04da58b27ac4`.
`p1311-gates.json` confirma escopo, hashes, ausência de stage/commit e preservação
de todos os arquivos anteriores. O revisor recalculou as evidências e emitiu
**PASS_SCOPED**, sem achados pendentes, em `p1311-verification.json`, SHA-256
`5e4d7a3600344cc32cbfb405c33cbd346b976d90300c00c18875aac55e7255c3`.
Fundamentação e limites em `p1311-review-final.md`; a conclusão não amplia
o fechamento além da coorte e dos controles declarados.

## Proveniência e reprodução

HEAD `eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, **working tree não commitado**.
O baseline P1311 já continha as alterações de loading de P1310. Cada recibo
registra argv, cwd, UTC, exit, stdout/stderr e diff/stat integral antes/depois.
Baseline: `p1311-baseline.json`, SHA-256
`2338de6be4567c9ea03be335c5935832241d8010272b10eb29c1ca858c9af753`.
O diff final contra HEAD compreende exatamente:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md | 62 linhas de diff
00_nucleo/prompts/compiler/stdlib/loading.md            | 63 linhas de diff (P1310)
01_core/src/compiler/eval/bindings/field_access.rs       | 175 linhas de diff
01_core/src/compiler/stdlib/loading.rs                  | 143 linhas de diff (P1310)
4 files changed, 434 insertions(+), 9 deletions(-)
```

Medição inicial: `2026-09-08T00:48:36.611014Z`. Freeze:
`2026-09-08T00:52:55.101638Z`. RED: `00:52:55.925515–00:54:41.691838Z`.
GREEN: `00:59:14.043084–01:01:02.015985Z`. Build:
`00:59:34.614288–01:01:38.016012Z`. A/B candidato:
`01:04:11.885154–01:05:46.707756Z`. Workspace:
`01:03:22.019490–01:07:52.022950Z`, todos em 2026-09-08 UTC.

Executáveis identificados por SHA-256, não por string de versão:

- Vanilla ratificado upstream `a51e02804`, `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Baseline `/dev/shm/p1310-target.VeBP6Q/release/typst`:
  `7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146`.
- Candidato `/dev/shm/p1311-target.JE8Cyy/release/typst`:
  `4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb`.

O target dedicado em RAM recebeu cópia do cache anterior e rebuild do workspace;
não se alega rebuild limpo de todas as dependências. O binário precedente não
foi sobrescrito. Comandos de build/test usam o CARGO_TARGET_DIR registrado;
reexecuções do recorder devem receber nomes novos para não sobrescrever recibos.
Não houve stage, commit, push ou limpeza dos artefatos anteriores.

A skill `tekt-materializacao-segregada` determinou a separação entre autoria
de código/L0, testes A/B e revisão, com GO prépatch. Regime proporcional A/B,
**executado sem atestação de isolamento técnico**: filesystem compartilhado,
sem alegação de selo de refinamento completo ou score de mutação. O testador
não leu o patch nem os testes locais; o revisor não editou o objeto julgado.
Ver `p1311-ab-receipt.md` e `p1311-review-prepatch.md`.
