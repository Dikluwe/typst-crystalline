# V16 — representação e identidade: lote concluído

**11 avisos V16 corrigidos**, uma nota V16 retirada e nove fallbacks adicionais nominalizados: **21 matches em sete consumers**, com sete L0 prescritos antes do código. **Tekt 342 → 331 avisos emitidos**; Clippy permanece **zero**. Não há API, enum/campo novo, alteração de default ou fase. [classification.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/classification.json) · [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/stage.diff) · [code-guard.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/code-guard.json).

## Mudança e finalidade

Os catchalls dos enums fechados passaram a nomear as variantes atuais. Os corpos, guards, precedência e chamadas foram preservados. Uma variante futura exige decisão de compilação nesses pontos; não foi substituído match por if-let ou introduzido helper que escondesse a mesma adoção silenciosa. Char e flags de mapeamento matemático conservam seus fallbacks próprios.

| Owner | Matches | Comportamento preservado |
|---|---:|---|
| Color | 1 | Eq por espaço/campos, f32 por bits e Spot estrutural; sem conversão para comparar |
| Color adjustments | 3 | darken, rotate e saturate; mesmas rotas, falhas, fórmulas, alpha e erro interno |
| Color conversion | 4 | Oklab/Oklch, rejeição ProcessToSpot e clone/projeção processual |
| Color mixing | 4 | defaults por compatibilidade Spot e seleção de hue, pesos e ordem |
| MathStyleKind | 4 | VS, bases de letras/dígitos e exceções BMP; normalização Roundhand e char aberto |
| Func | 4 | element_name, namespace, Eq e Hash; nome nativo por kind e Arc para as demais representações |
| StateUpdate | 1 | Set por Value, Func por identidade Arc e pares cruzados distintos |

No L0 math_style, o fragmento atual agora inclui Display/Inline já existentes; a descrição Roundhand acompanha P812-C, e os três exemplos Unicode com redação de rascunho foram saneados. P964 foi qualificada sobre as notas antigas de scope-out. O L0 StateUpdate distingue o registro histórico de stub do caminho atual com Engine e do legacy, delegando execução ao owner correto sem alterar runtime. O caminho desse owner foi canonizado antes da implementação conforme ADR-0130. [l0-path-clarification.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/l0-path-clarification.json).

## Prova de evolução — RED→GREEN estrutural

Na cópia descartável do workspace, foi acrescentada uma variante sentinela a cada um de **Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate**. As definições produtivas permaneceram idênticas, conferidas pelo guard. A sonda compilou o core real, com os mesmos manifests/lock/dependências e cargo check offline/locked; nenhuma substituição de corpos por stubs.

Antes: **0 de 21 matches selecionados** exigiam decisão, enquanto outros matches já produziam 28 erros E0004 na cópia. Depois: **21 de 21** exigem decisão, dentro de 49 E0004 totais. A seleção é por source e linha do match, sem confundir erros de outros pontos. Em ambas as rodadas não houve outro tipo de erro de compilação. As cópias foram descartadas; arquivos/hashes/UTC/comandos e diagnósticos permanecem em [probe-baseline-input.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/probe-baseline-input.json.gz), [probe-baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/probe-baseline-details.json), [probe-green-input.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/probe-green-input.json.gz) e [probe-green-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/probe-green-details.json).

O gate V16 por escopo tinha **11 avisos** antes e agora tem **zero** nesses sete consumers. O linter global continua retornando falha por 312 V16 externos; o wrapper registra esse resultado e não o apresenta como gate global aprovado. [baseline-red.log](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/baseline-red.log) · [scope-gate-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/scope-gate-result.json).

Essa é prova de exaustividade futura no mecanismo Rust, não medição nova de paridade Typst, melhoria de leitura por IA ou performance. A aceitação funcional permanece nos resultados da linguagem e nas políticas de identidade já vigentes.

## Testes e validação

Quatro testes novos foram escritos antes da implementação: matriz de igualdade entre nove espaços Color, clone e política de bits; seis representações de Func com Eq/Hash/namespace/element_name, nomes nativos por kind e ausência de execução de host/ctor; StateUpdate distinguindo Arc de nativas que comparam iguais por nome; exemplos Unicode e variants de tamanho matemático sem mudança de glyph/VS. Nenhum esperado histórico foi relaxado. A baseline de entidades passou com **1.695 testes, zero falhas e zero ignorados**. [tests-materialization.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/tests-materialization.json) · [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/baseline-details.json).

Final: **8621 aprovados, 0 falhas, 7 ignorados** na suíte workspace. Build, Clippy normal e estrito em todos os targets (-D warnings e -D unfulfilled_lint_expectations), formato, linhagem V1/V5/V15/V26 direta/reversa, Rustdoc -D warnings, quatro índices HTML e diff --check passaram. Fontes/L0/configuração ficaram congelados durante a validação. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/validation-details.json) · [strict-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/strict-details.json) · [workspace/workspace-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/workspace/workspace-result.json).

## Notas V19 e pendências

Os padrões OR explícitos acrescentam **21 notas V19**: elas avisam que cobertura por braço não equivale a cobertura de todas as alternativas agrupadas. Os grupos compartilham o mesmo corpo vigente; não se duplicou lógica para eliminar notas. Estão preservadas e classificadas em [new-notes.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/new-notes.json); não foram rebaixadas, suprimidas ou contadas como avisos. Não se alega cobertura completa de caminhos apenas pelo número de testes.

Restam **312 V16 e 19 V12**, sem avisos novos por mensagem/local no SARIF. Os casos sem emissão preservados de V17 permanecem pendentes. Ordem seguinte: V16, depois V12. [lista-pendencias.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/lista-pendencias.md) · [latent-v16.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/latent-v16.json). A meta geral continua aberta. A extração AST posterior, somente de leitura, reuniu 312 avisos e dois locais sem emissão em 314 contextos e 136 owners, sem resolver tipos por inferência: [índice de contextos](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-wildcards-2026-10-16/indice-contextos.md). contexts-result.json registra seu serviço separado sob o mesmo teto.

## Recursos e proveniência

Execuções pesadas sequenciais em cgroup **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. Pico final **12.000 GiB**, incluindo cache: atingiu o teto e o kernel aplicou o limite; eventos max=599, OOM=0, OOM-kill=0. Baseline: 4.359 GiB, também sem OOM. [validation-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/validation-result.json) · [memory-samples.jsonl](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/memory-samples.jsonl).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot 2026-10-16T19:29:57.297819+00:00; validação 2026-10-16T19:39:35.463780+00:00–2026-10-16T19:51:50.334696+00:00 UTC. Fontes/L0/configuração/lock, hashes, status e git diff HEAD --stat estão em [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/before.json.gz), [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/baseline-details.json) e [final-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/final-state.json). A preservação do estado externo ao lote foi conferida globalmente. Scripts em workers/; [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-enums-2026-10-16/manifest.sha256).

Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150, SHA-256 26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde reconfirmado. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 e binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Nenhum commit, push ou release.
