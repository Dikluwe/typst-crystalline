# V17 — guards compostos: etapa concluída

**28 V17 → zero**, sem supressões, mudança de limite ou API. **Clippy zero**, inclusive workspace/all-targets com -D warnings e expectativas não cumpridas como erro. Atualizados **21 consumers e 21 L0** com prescrição por caso antes da implementação; resselo restrito. [classification.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/classification.json) · [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/stage.diff) · [code-guard.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/code-guard.json).

## O que foi corrigido

- Presença de campos e snapshots: braços unificados com escolha explícita entre accessor declarado e getter; snapshot Some continua autoritativo mesmo sem a chave. Sem snapshot, conteúdo comum conserva getter. O texto do adendo foi precisado após a baseline e antes do código, sem alterar os testes ou seu comportamento; recibo l0-clarification.json.
- Avaliação e spans: preservados os oito alvos por identidade nativa, With recursivo, curto-circuito e realização única; caminho sem recipe/preparo continua sem realizar.
- Grid/Table, origens e tracking: elegibilidade/fallback no corpo dos braços, sem ampliar cache, remover células com atributos ou repetir callbacks; RTL continua com tracking assinado e barreiras.
- Matrizes: modos array/delimitado explícitos em padrões de tupla; mistura continua inválida; getters/repr de augment conservam inteiro/dict/presença e stroke auto.
- Caption: padrões estruturais Top/Bottom sem horizontal; mesmos casts, diagnósticos e defaults.
- Cores e níveis de tabela: condições no corpo, mesmos intervalos, casts, mensagens, Debug e interpolação spot/processual. Sem mudança de fórmulas ou tolerâncias.
- PDF/HTML: Link e folhas MCID conservam Artifact, contador, ordem e recursão; Raw inelegível mantém erro sem observar texto; átomos typc conservam escape/code/span.
- Decoder de teste HTML: prefixo #x/#X reconhecido por operação ASCII segura para UTF-8; mesma validação Unicode/decimal e decodificação única.

A regra CI V17 recomenda desdobrar a condição ou movê-la para o corpo. Não foi criado helper para esconder conjunções. Este lote melhora a explicitação local de decisões; não mede benefício de leitura por IA, ganho de RAM ou paridade universal. As assinaturas públicas e o restante das fontes/configuração estão preservados por replay exato de 30 substituições, formatação restrita e hash global.

## Testes antes e depois

Dois controles novos foram escritos antes do código: componentes Relative em 0/interior/1, NaN/infinito/out-of-range e parte absoluta não zero nos dois casters; entidades HTML com #X, Unicode escalar, surrogate, valor acima do máximo, truncamento, prefixos UTF-8 inválidos e decodificação única. Os controles protegem comportamento existente e passaram antes; não foi inventada uma falha de produto. Os esperados históricos ficaram intactos.

Baseline: **8617 aprovados, zero falhas, 7 ignorados**. O gate --checks v17 --fail-on warning falhou com exatamente os 28 avisos anteriores. Após a implementação o mesmo gate passou sem V17. [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/baseline-details.json) · [baseline-red.log](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/baseline-red.log) · [validation-v17.log](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/validation-v17.log).

Final: **8617 aprovados, 0 falhas, 7 ignorados**. Build, Clippy normal/estrito, fmt, linhagem V1/V5/V15/V26 estrita e reversa, diff --check e Rustdoc -D warnings passaram; os quatro índices HTML existem. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/validation-details.json) · [strict-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/strict-details.json) · [workspace/workspace-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/workspace/workspace-result.json).

O primeiro runner de baseline teve erro de sintaxe antes de iniciar Cargo; foi corrigido e reexecutado, com logs do erro preservados. Nenhuma medição desse runner é usada como validação.

A primeira rodada final parou na linhagem reversa: o --fix-hashes reparou apenas pares com drift de prompt, deixando 20 hashes reversos de código antigos após o seal pré-baseline. O worker restrito foi corrigido para solicitar resselo de todos os pares selecionados numa cópia temporária e copiar somente header/metadado autorizados. A correção mudou apenas 20 linhas Hash do Código em L0; fontes Rust, configurações e texto normativo ficaram idênticos. resume-proof.json recompõe e confere os hashes anteriores. Build, Clippy normal/estrito, fmt e replay aprovados foram reutilizados; linhagem direta/reversa foi repetida e os checks restantes foram executados. attempt-1/ preserva a falha e recursos; reverse-reseal.log registra o reparo. Não houve relaxamento do gate.

## Contagem Tekt e continuidade

O SARIF passou de 374 a **342 avisos emitidos**: V16=323 e V12=19, sem erros. A redução é composta por 28 V17 corrigidos e quatro V16 afetados pela união/heurística dos braços, **sem certificação de exaustividade**. T027 e T158/T160 permanecem rastreados para auditoria mesmo sem emissão; T311/T313 viraram um único aviso ainda pendente. O único texto novo de V16 é esse fallback consolidado de metadata, não um novo local. [architecture-delta.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/architecture-delta.json) · [latent-v16.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/latent-v16.json).

Próxima ordem: V16, incluindo os locais preservados sem aviso, depois V12. Lista nominal com arquivos, linhas e L0: [lista-pendencias.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/lista-pendencias.md). A meta geral continua aberta; esta etapa conclui apenas V17.

## Recursos e proveniência

Jobs pesados sequenciais em cgroup **MemoryMax=12 GiB, MemorySwapMax=0, CARGO_BUILD_JOBS=1**. Pico final **9.667 GiB**, incluindo cache; eventos max=0, OOM=0, OOM-kill=0. A baseline atingiu 10.955 GiB, também sem OOM. [validation-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/validation-result.json) · [memory-samples.jsonl](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/memory-samples.jsonl).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot 2026-10-16T18:57:33.016919+00:00; validação final 2026-10-16T19:09:46.000381+00:00–2026-10-16T19:25:08.273244+00:00 UTC. Fontes/L0/configurações permaneceram congelados durante os comandos de cada rodada; entre as duas rodadas finais houve somente a correção dos metadados reversos comprovada em resume-proof.json; [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/before.json.gz), [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/baseline-details.json) e [final-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/final-state.json) guardam conteúdo, hashes, estado, status e git diff HEAD --stat.

Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150, SHA-256 26fe38b564a0aa92e7cc49f3b09a4324d1127e3586862afa5af4480e6c394dde reconfirmado. Vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/ e [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-guards-2026-10-16/manifest.sha256). Sem commit, push ou release.
