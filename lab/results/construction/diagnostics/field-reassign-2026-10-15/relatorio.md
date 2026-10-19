# field_reassign_with_default — etapa concluída

Os 25 casos foram corrigidos em oito consumers, todos em fixtures/helpers de teste. Oito L0 foram atualizados antes do código e resselados de forma restrita. Nenhuma supressão nova, API ou comportamento padrão foi introduzido. A configuração de cada fixture agora fica reunida em um literal com `..Tipo::default()`.

Foram movidos 41 valores explícitos para os inicializadores, preservando os defaults dos demais campos. Foram retirados 23 `mut` dispensáveis. Stroke permanece mutável para `specified.thickness/cap`; o gerador de recursos conserva a atribuição condicional de variações em `if axes`. Os clones e as mutações posteriores de peso/tracking foram preservados. O probe de timing F020 continua ignorado por padrão e foi compilado por all-targets; nenhum resultado de benchmark é alegado.

[Lista dos 25 casos](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/lista.md) · [CSV](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/cases.csv) · [JSON](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/cases.json) · [Diff da etapa](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/stage.diff). A lista identifica campos, função/helper, fonte atual e L0 proprietário. O [controle de escopo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/scope-guard.json) registra gates cfg(test), nomes/atributos de testes, trechos de assertions e partes produtivas preservadas. A revisão do diff confirmou as mutações posteriores intactas. Esta manutenção de sintaxe não mede ganho para IA nem paridade nova.

## Validação

O gate RED reproduziu os 25 erros antes da edição; o GREEN passou com `-D clippy::field_reassign_with_default -D unfulfilled_lint_expectations`. O inventário completo caiu de **144 para 119 avisos**, com delta exclusivamente de −25 nessa regra. `type_complexity` segue em zero. Os 57 casos de `too_many_arguments` permanecem na auditoria anterior.

A suíte workspace terminou com **8593 testes aprovados, 0 falhas e 7 ignorados**. Inputs, nomes, esperados e tolerâncias não foram relaxados; não foram criados testes que espelhem a sintaxe do inicializador. Build, Clippy all-targets, cargo fmt, Rustdoc com `RUSTDOCFLAGS="-D warnings"`, git diff --check, linhagem estrita V1/V5/V15/V26 e linhagem reversa passaram. Os quatro índices HTML foram conferidos após a suíte. Fontes e L0 permaneceram imutáveis durante a execução e continuam idênticos na conferência de fechamento.

O Tekt completo registrou {'warning': 380, 'note': 1668}; avisos por regra: {'V16': 327, 'V17': 28, 'V18': 6, 'V12': 19}. Nenhum erro ou aviso arquitetural novo. Os avisos restantes de Clippy/Tekt continuam pendentes; o CI global ainda não está livre de avisos.

[Comandos, horários e estado da validação](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/validation-details.json) · [Resultado da suíte](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/workspace/workspace-result.json) · [Gate RED](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/red-clippy-result.json) · [Gate GREEN](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/green-clippy-result.json).

## RAM e proveniência

Os serviços usaram **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. O pico do cgroup da validação foi **12.000 GiB**, incluindo cache de arquivos. O limite foi atingido: memory.events max=413; OOM=0 e OOM-kill=0. Esses eventos de max registram pressão/recuperação no limite, não OOM. O limite cobre os processos dos serviços, não os demais aplicativos do host. [Recibo de memória](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/validation-result.json).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Origem em `2026-10-15T18:14:08.656151+00:00`; validação entre `2026-10-15T18:19:53.023441+00:00` e `2026-10-15T18:32:12.039546+00:00` (UTC). [Snapshot inicial](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/before.json.gz) e recibos conservam fontes/L0, hashes, status, arquivos não rastreados e a lista exata de alterações (`git diff HEAD --stat`). [Conferência final](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/final-state.json) registra os hashes atuais e sua equivalência com o estado medido.

A suíte usou o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, e o linter CI pinado em `49c6bb0e584104a0c89270431d8402f5c6c61150`. Os caminhos temporários de execução constam nos recibos; os scripts temporários desapareceram após a interrupção, e não se alega preservação desses workers. O finalizador deste fechamento está salvo em workers/finalize.py. [Manifesto SHA-256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/field-reassign-2026-10-15/manifest.sha256).

Sem commit, push ou release. Alterações anteriores preservadas.
