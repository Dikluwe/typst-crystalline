# V16 — helpers matemáticos e transparência de Table

Etapa concluída em 2026-10-17T04:34:22.260182+00:00. **V16 222 → 170**, **V19 918 → 989 notas**, **V20 871 → 870 notas**. O consumer de testes de eval ficou sem V16 emitido. O gate global continua com exit 1 pelos 170 casos restantes; o objetivo geral permanece aberto.

## Correções

Fechados os **52 V16 emitidos** e **18 fallbacks latentes** de Content/MathAttachSlot: sete Content e onze slots. As decisões incluem busca de raízes, estilos/labels/headings/contexto, Terms/Quote, operadores/identificadores, delimitadores/anotações/matrizes/frações, fronteiras, quebras e contagens. A lista individual, os spans e a justificativa de cada grupo estão em [scope.json](scope.json) e no [00_nucleo/prompts/compiler/eval/tests.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval/tests.md). Guards recusados continuam chegando ao mesmo fallback; panics e asserções anteriores permanecem.

Foram preservados os corpos, ordem/short-circuit e esperados dos helpers. Dois asserts negativos foram acrescentados ao teste de leaf, e um teste exige rejeitar a fração com barra antes de achar o binom posterior. O caso prime_count foi refinado em dois matches rasos: distingue slots e depois Content. O primeiro candidato nominal aninhado gerava um V20 novo (candidate-architecture.log); a forma final remove também o V20 antigo desse caso. São **71 notas V19 novas justificadas**, sem supressões, bindings @, limiar global ou alegação de cobertura de execução por alternativa. As demais notas/diagnósticos foram preservadas. [architecture-delta.json](architecture-delta.json).

Corrigido também o defeito produtivo da raiz Table: a sonda pública introspect retornava **0 headings** para Realized(Table com heading hidden, replacement), mas **1** ao embrulhar a mesma origem em Source. Agora retorna **0/0**. table_root normaliza c.source_body() por empréstimo. O controle posterior revelou perda de contexto em ShowRecipe: sua delegação reiniciava forced_location e shallow_table. Ela agora os preserva via walk_at, mantendo identidade preparada e excluindo a heading ocultada. Sem campo/assinatura pública, fase, default, dependência ou contrato novo. [source-table-before.json](source-table-before.json), [source-table-after.json](source-table-after.json), [00_nucleo/prompts/compiler/introspect/walk.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/introspect/walk.md).

Os controles cobrem Source único/aninhado, as duas ordens com Located, ShowRecipe com e sem Source, preservação da Location explícita, visual shown uma vez e origem não-Table. Seis sondas CLI bilaterais no pin **9f2b6e8715237cb086899a42873660fe744622e8** conservaram os resultados. Isso não é prova de paridade global de tabelas; a sonda de carrier público distinguiu o defeito que essas seis fixtures não revelavam. [source-table-parity.json](source-table-parity.json).

## Testes e gates

RED inicial: **3 passaram, 2 falharam**. Após normalizar a raiz, **4 passaram, 1 falhou** no carrier ShowRecipe; o RED adicional de contexto/Location teve **4 passaram, 2 falharam**. GREEN final: **6 passaram, nenhuma falha**. As tentativas e fontes estão preservadas; expectativas antigas não foram relaxadas. A suíte workspace completa terminou com **8675 passando, zero falhas e 7 ignorados**, acréscimo de seis testes sobre a entrada.

Na sonda descartável com cfg(test) compilado e variantes novas de Content/MathAttachSlot, os locais selecionados passaram de **0 para 71 E0004**: 70 decisões originais e o novo match interno de prime_count. Total **46 → 117**, sem outros erros. As fontes do workspace não foram alteradas pela sonda. [probe-baseline-result.json](probe-baseline-result.json), [probe-green-result.json](probe-green-result.json). O auditor AST não encontra mais wildcard nominal desses dois enums neste consumer; isso não alega ausência universal de todos os tipos de fallback.

Build workspace, fmt, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices HTML, V1/V5/V15/V26 estrito, linhagem reversa e diff-check passaram. O diferencial verifica os campos de todos os demais diagnósticos, incluindo posições mapeadas por AST/linhas. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json).

O replay confere dois consumers e seus owners, preservando os arquivos externos e todas as definições públicas/enums. A mudança produtiva se limita à classificação normalizada da raiz e ao transporte dos dois valores de contexto em ShowRecipe. O resto é manutenção de testes, conforme L0 anterior a cada implementação/refinamento. [guard-result.json](guard-result.json), [stage.patch](stage.patch).

## Recursos, proveniência e continuidade

Comandos pesados sequenciais com **MemoryMax=12 GiB**, **MemorySwapMax=0** e **Cargo um job**. Pico de memória do cgroup: **12884901888 bytes (12.000 GiB)**; sem OOM/OOM kill. Maior memory.events/max: **909**, preservado nos recibos; não é RSS nem prova de ganho de desempenho.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, working tree não commitado. [before.json.gz](before.json.gz), candidate-state.json.gz, recibos de refinamento e [final-state.json.gz](final-state.json.gz) registram UTC, fontes/hashes, status e git diff HEAD --stat exato. Fontes/L0/configuração ficaram congelados durante os gates. Resselo limitado aos dois pares, com grafo V15/V26 validado. Linter `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Oráculo SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin acima e proveniência ratificada em CLAUDE.md. Sem commit, push, merge, release ou alteração do pin CI.

As primeiras sondas de diagnóstico tiveram erros de montagem (helper crate-private/E0624 e borrow do cursor/E0597), corrigidos e preservados nos recibos audit-repair.json/latent-repair.json; não contam como RED funcional. A revisão automática de permissão excedeu o prazo antes de iniciar a primeira materialização. As fontes foram reconferidas, e a única retentativa permitida, com worker nomeado, foi concluída; não há bloqueio de permissão pendente. [approval-timeout.json](approval-timeout.json).

[Lista organizada dos 170 V16 pendentes](pendencias.md), [inventário completo](remaining-v16.json). As notas V19 justificadas continuam visíveis; as demais correções exigem seus próximos lotes.
