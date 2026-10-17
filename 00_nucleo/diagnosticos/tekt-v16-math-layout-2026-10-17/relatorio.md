# V16 — layout matemático e transparência de Source

Etapa concluída em 2026-10-17T05:28:17.504079+00:00. **V16 170 → 152**, **V19 989 → 1028 notas**, **V20 870 → 869 notas**. Os consumers deste lote ficaram sem V16 emitido. O gate global continua com exit 1 pelos **152 V16 restantes**; o objetivo geral permanece aberto.

## O que foi corrigido

Fechados **18 V16 emitidos e 18 fallbacks latentes**, mais um encaminhamento nominal auxiliar em partition_grid. Foram auditados Content, Value, MathStyleKind e FrameItem em 12 consumers e seus 12 owners, incluindo um consumer/owner novo de testes. A [lista dos 36 casos](casos.md) aponta para código e L0; guards, complementos e motivos individuais estão em [scope.json](scope.json). Campos e assinaturas públicas foram preservados. Os grupos V19 mantêm o corpo anterior; variantes com guards recusados continuam chegando ao complemento. Nenhuma supressão foi adicionada.

As medições anteriores mostravam defeitos reais ao acrescentar Source ao mesmo conteúdo matemático. A largura de limits inline mudava de **7.2 para 12.912 pt**, as linhas de uma equação mudavam de **2 para 1** e a última baseline de **16.2 para 0 pt**. A matriz complementar tinha **19 diferenças em 23 pares**. Source agora é normalizado por empréstimo na classificação de limites, scripts/kerns, texto de operadores, acentos/dotless, suportes, alinhamento de matrizes, espaçamento, sequências, stretch e classes. Quebras e pontos de alinhamento também são reconhecidos sob Source. Conteúdo derivado preserva a primeira origem causal pelos helpers existentes; o teste de flatten verifica a precedência entre origem do filho e origem herdada.

Após o build, os mesmos programas públicos e métricas foram repetidos com a biblioteca reconstruída: **todos os 23 pares coincidem**, limits retorna **7.2/7.2 pt**, metadados da equação retornam **2/2 linhas e baseline 16.2/16.2 pt**. As posições dos marcadores de alinhamento também coincidem. Os stdout, fontes da sonda, rlib, comandos e hashes anteriores/posteriores estão preservados nos source-*.json. São provas dessas geometrias e metadados; não alegam paridade integral de matemática com o vanilla.

A projeção Option de attach foi decomposta em `?` e match nominal raso de Content. Isso remove o V20 antigo desse padrão, sem introduzir outro. São **39 V19 novos**: 36 complementos originais, um encaminhamento de grid e dois grupos do observador de testes. A nota V19 da projeção foi atualizada para sua forma rasa. Todos os demais diagnósticos foram preservados, incluindo seus campos e posições mapeadas por AST/linhas. [architecture-delta.json](architecture-delta.json).

## Validação

O RED funcional teve **5 testes passando e 24 falhando**. O GREEN inicial terminou com exit 0; seu recibo de recursos foi preservado, mas o stdout dessa chamada não ficou retido. A suíte completa posterior registra nominalmente os **29 controles passando**, sem relaxar as expectativas. Foram acrescentados 29 testes concretos (sete definições anotadas, incluindo macro com 23 casos). O observador rejeita itens não suportados e compara texto, posições, dimensões, metadados de linhas e substituições lógicas. [controls-workspace-proof.json](controls-workspace-proof.json).

A suíte workspace terminou com **8704 passando, zero falhas e 7 ignorados**, acréscimo de 29 sobre a entrada. A sonda descartável adicionando variantes aos quatro enums passou de **0 para 37 E0004 nos locais selecionados**; total **229 → 266**, sem outro erro. Os 36 locais originais e o encaminhamento auxiliar agora exigem revisão do compilador quando o domínio cresce. As fontes reais não foram modificadas pela sonda.

Passaram build workspace, fmt, Clippy workspace/all-targets com `-D warnings` e `-D unfulfilled_lint_expectations`, Rustdoc com `-D warnings` e quatro índices HTML, V1/V5/V15/V26 estrito, linhagem reversa, replay das alterações e diff-check. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json), [guard-result.json](guard-result.json), [stage.patch](stage.patch).

## Recursos e proveniência

Comandos pesados sequenciais sob **MemoryMax=12 GiB**, **swap zero** e **Cargo um job**. Pico do cgroup: **12884901888 bytes (12.000 GiB)**, sem OOM/OOM kill. Maior memory.events/max: **479**. Esse dado mede o cgroup; não prova ganho de desempenho nem representa RSS isolado.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) e [final-state.json.gz](final-state.json.gz) registram UTC, fontes/L0/configuração, hashes, status e git diff HEAD --stat exato. Fontes ficaram congeladas nos gates. Resselo limitado aos 12 pares, com grafo de ownership/núcleos validado. Linter `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Sem alteração de pin CI, commit, push, merge ou release do Typst.

Foram preservados dois problemas de montagem das provas: contagem inicial do analisador suplementar (24 em vez dos 23 pares já medidos) e uso de Point::new inexistente no teste. Foram corrigidos sem contar erro de compilação/análise como RED funcional; recibos em source-family-analysis-repair.json, point-repair.json e attempt-point/.

O primeiro Clippy estrito apontou uma clonagem usada apenas para montar uma fatia de um item no teste novo. O L0 do teste foi atualizado antes da troca para `std::slice::from_ref`, mantendo dados e expectativas; o candidato anterior e sua falha estão em attempt-clippy/, e o replay inclui esse único refinamento. Todos os gates foram reexecutados sobre o estado final.

## Continuidade

[Lista organizada dos 152 V16 pendentes](pendencias.md), [locais e mensagens completos](remaining-v16.json). **math-stretch-composite continua aberto**: o contrato já aprovado exige anexos/corpos compostos/eixo nativo e não é estreitado pelo reparo atual de Source. V19 permanece informativo e visível; ausência de V16 não demonstra cobertura de cada alternativa.
