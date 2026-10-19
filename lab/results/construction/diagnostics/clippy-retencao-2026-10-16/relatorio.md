# Retenção e código sem uso — etapa concluída

Clippy **17 → 13 avisos**, sem novos avisos: `dead_code` **3 → 0** e `vec_box` **1 → 0**. Dois consumers e seus L0 foram atualizados com resselo restrito. Nenhum allow/expect novo ou mudança de limiar. [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/stage.diff) · [lista-pendencias.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/lista-pendencias.md).

Producer.parent foi removido somente do armazenamento. O argumento parent de Session.add continua herdando o target do pai, sem mudar descoberta, filhos, geração ou roteamento HTML/bundle.

Execution.final_styles tornou-se _retained_styles, com comentário sobre sua função. O mesmo styles continua retido até o drop do registro. O campo não é lido por replay, mas guarda valores/closures: apagá-lo para zerar dead_code anteciparia a liberação. A nova nomenclatura explicita essa finalidade; não cria getter ou leitura artificial.

retired usa Vec<Execution>. As três transições movem o registro inteiro para o vetor e liberam sua caixa externa, conservando ordem, ctx/observações, resultado, sink, Location e Arc de entrada. current permanece Option<Box<Execution>>. Não foram descartados históricos, refeitas closures ou deduplicadas gerações. A auditoria não encontrou dependência do endereço de Execution; identidades dos dados referidos permanecem nos Arcs. Não se afirma ganho de RSS/tempo ou redução do número total de alocações sem benchmark próprio.

O wrapper subset_font foi retirado: só era chamado por dois testes, dentro de mod subset privado e sem reexportação. Os testes agora usam subset_font_with_mapping e .data, conservando GIDs, .notdef, nomes e assertions. Todos os adapters produtivos, variações, mapping, mensagens e fallbacks ficaram preservados. O L0 anterior exigia manter um wrapper de compatibilidade sem consumer produtivo; a busca refutou essa prescrição, que foi corrigida antes do código. Embora escrito pub, o símbolo não era externamente alcançável. É alteração interna no fluxo contínuo da ADR-0127 §2, item 4; não se criou nem retirou API pública resolvida. [code-guard.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/code-guard.json) e [materialization.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/materialization.json) registram as edições exatas e a preservação dos contratos produtivos.

## Verificação

Antes da implementação passaram **2 testes de retenção/snapshots e 9 de subset**, incluindo o novo controle. O gate RED identificou exatamente três dead_code e um vec_box. [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/baseline-details.json).

O novo teste usa Session.execute real e Arc/Weak para provar que um valor de estilo sobrevive na execução atual, na aposentada e após crescimento real do Vec, e é liberado ao encerrar a sessão. Confere também a mesma identidade do input. É testemunha mecânica de lifetime; não substitui a semântica de linguagem testada pelos controles existentes.

Após a mudança, **8600 testes passaram, 0 falhas e 7 ignorados** na suíte workspace completa. Build, Clippy all-targets e gate estrito de dead_code/vec_box/aridade/expectativas, formato, Rustdoc com warnings como erro, quatro índices HTML, diff --check, linhagem estrita V1/V5/V15/V26 e reversa passaram. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/validation-details.json) · [strict-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/strict-details.json) · [workspace/workspace-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/workspace/workspace-result.json).

Tekt mantém **380 avisos**, {'V16': 327, 'V17': 28, 'V18': 6, 'V12': 19}, sem erros nem aumentos por regra. A dívida anterior continua aberta. Contagens e testes não certificam paridade global ou conformidade PDF/UA.

A primeira abertura do serviço de validação não executou: a revisão automática de permissão expirou no prazo. A única repetição permitida foi liberada e executou estes comandos; nenhum teste ou gate foi contornado. [approval-retry.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/approval-retry.json).

## RAM e proveniência

Serviços pesados com **12 GiB, swap zero e CARGO_BUILD_JOBS=1**. Pico da validação **11.448 GiB**, incluindo cache; eventos max=0, OOM=0, OOM-kill=0. [validation-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/validation-result.json).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; snapshot 2026-10-16T16:41:36.286718+00:00; validação 2026-10-16T16:48:34.640013+00:00–2026-10-16T16:59:31.836226+00:00 UTC, fontes/L0 imutáveis durante os comandos. [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/before.json.gz) e [final-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/final-state.json) guardam fontes/hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8 com SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/clippy-retencao-2026-10-16/manifest.sha256).

Restam **13 avisos Clippy** (5 sugestões de traits, 3 to_string, 5 enums grandes) e a dívida Tekt. A meta geral continua aberta. Sem commit, push ou release.
