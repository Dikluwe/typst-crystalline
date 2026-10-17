# V16 — espaços de eval e HTML

Etapa concluída em 2026-10-17T06:13:50.308262+00:00. **V16 152 → 141**, **V19 1028 → 1041 notas**, **V20 869 → 869 notas**. Os três consumers deste lote ficaram sem V16 emitido. O gate global continua com exit 1 pelos **141 V16 restantes**; o objetivo geral permanece aberto.

## Correções e equilíbrio com V19

Fechados **11 V16 emitidos e um forwarding latente** em doze braços originais de onze matches. Os módulos tratam origem/reconstrução/forma canônica de espaços, texto auxiliar, matching textual, projeção visual e suporte de espaço HTML. [Lista individual](casos.md), [spans/guards/complementos](scope.json). Complementos excluem somente casos incondicionais; HtmlElem guardado e o guard had_newline conservam sua decisão anterior. Os corpos anteriores permanecem, exceto a correção de fronteiras explicitamente prescrita. Sem alteração de API, dependência, default ou fase.

Há **13 V19 novos**, todos justificados nos L0: doze complementos e a parada nominal do helper privado novo. Não foram adicionados allow/expect ou captura @. Dois complementos são tabelas de SpaceSupport: Boundary absorve a borda, Supportive/Supportive usa espaço normal e as outras três células usam espaço protegido. As nove combinações foram verificadas nos dois helpers (18 observações em três testes). Os demais grupos e todos os outros diagnósticos foram preservados, com posições mapeadas por AST/linhas. [architecture-delta.json](architecture-delta.json). Uma nota V19 continua sem implicar cobertura de execução por alternativa.

## Defeito produtivo medido e corrigido

O scanner comparava Empty e Linebreak diretos. Em **28 pares** com Source, Located, Realized.visual e Styled, **20 divergiam**: espaços sobreviviam ao lado de quebra explícita ou não colapsavam através de Empty. Parbreak e Text deram os oito controles iguais. O defeito afetava tanto o consumidor in-place quanto a decisão lazy de materializar alteração.

A projeção privada **space_boundary_content** agora identifica essas duas fronteiras pelos quatro carriers por empréstimo. Não modifica nós, spans, estilos, índices ou o marcador de quebra; Realized observa visual. A política Unicode, OR de origem newline, a certificação canônica e as transformações históricas de ocorrências de espaço permanecem. Matching continua respeitando suas fronteiras e snapshots existentes; nenhuma receita é executada pela projeção.

Após o reparo, a mesma sonda no módulo real, dentro de um workspace descartável, registra **28/28 pares iguais**, incluindo os oito controles anteriores. [source-family-before-analysis.json](source-family-before-analysis.json), [source-family-after-analysis.json](source-family-after-analysis.json). As fontes completas medidas e hashes, HEAD/UTC/argv e logs estão nos recibos source-*.json.gz. Isso prova esses casos da política de espaços; não alega paridade global com o vanilla ou igualdade de mecânica Rust.

## Testes, linhagem e sondas

RED funcional: **9 passaram e 7 falharam**. GREEN: **16 passaram**, sem relaxar esperados. Os controles incluem os quatro carriers, Source aninhado, quebra em ambas as bordas, espaços explícitos e de newline, fronteiras Parbreak/Text/MathIdent, forma canônica com span/Styled, Realized.visual separado de original, matching elegível versus fronteiras e as tabelas HTML.

A suíte workspace terminou com **8720 passando, zero falhas e 7 ignorados**, acréscimo de 16. Build, fmt, Clippy workspace/all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings e quatro índices, V1/V5/V15/V26 estrito, reversa, replay e diff-check passaram. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json), [guard-result.json](guard-result.json).

A sonda usa variantes novas **reais** de Content e do SpaceSupport privado, com um erro-testemunha exclusivo de cfg(test) para provar a compilação dos testes. No core, os matches selecionados passaram de 0 a 6 E0004; no infra, de 0 a 6: **12 matches finais exigem revisão**. São 13 braços, pois dois pertencem ao mesmo match. Para compilar o infra, somente E0004 de dependências/unidades fora dos alvos recebem braços panic sintéticos em cópia descartável (57 no baseline, 63 no GREEN). Nas rodadas finais, esses braços não executam, ficam nas fontes/targets descartáveis e nunca corrigem o alvo desta fase. O infra termina com somente sua testemunha e os seis locais esperados no GREEN. Inputs por iteração, reparos e diagnósticos estão preservados. [probe-baseline-result.json](probe-baseline-result.json), [probe-green-result.json](probe-green-result.json).

## Proveniência, recursos e limites

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) e [final-state.json.gz](final-state.json.gz) conservam UTC, fontes/L0/configuração, status e git diff HEAD --stat exato. Foram alterados somente três consumers e seus três owners, com resselo limitado e validação do grafo V15/V26. A referência ao owner de projeção em `00_nucleo/prompts/compiler/eval/rules/textual/matching.md` foi expandida ao caminho canônico antes do código, conforme ADR-0130. [canonical-reference.json](canonical-reference.json), [stage.patch](stage.patch).

Comandos pesados sequenciais sob **MemoryMax=12 GiB**, **swap zero** e **Cargo um job**. Pico do cgroup **12884901888 bytes (12.000 GiB)**, sem OOM/OOM kill; maior memory.events/max **1493**. O dado não é RSS isolado nem prova ganho de desempenho. Linter `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Sem alteração de pin CI, commit, push, merge ou release do Typst.

Duas falhas de preparação foram preservadas e não contam como RED funcional: a cópia externa do algoritmo não podia acessar helpers crate-private (E0624), e a primeira cópia de infra não continha os 16 arquivos usados por 18 include_str de fixtures. As provas finais compilam o módulo real e incluem essas fixtures com hashes. [measurement-setup-repair.json](measurement-setup-repair.json), [probe-fixtures-repair.json](probe-fixtures-repair.json).

O primeiro build completo detectou metadata de Content com a variante artificial no cache compartilhado de target/debug: a sonda havia emitido uma dependência core mutante ali. As fontes reais continuavam intactas. O cache gerado somente de typst-core/typst-infra foi limpo, e sondas/medições passaram a usar dois targets privados, separados do build principal, sem artefatos herdados de typst-core/typst-infra. Os dois baselines/greens foram repetidos; o baseline estrutural final restaura em cópia o checkpoint exato tests-before-code.json. Todos os gates finais rodam após esse reparo. [target-isolation-repair.json](target-isolation-repair.json), [attempt-target-cache/](attempt-target-cache/). Esta falha foi de isolamento dos artefatos da sonda, não uma alteração pública aplicada ao produto.

Após os gates, os dois targets privados gerados nesta etapa foram removidos; o target principal e as fontes foram preservados. As fontes das mutações, comandos, fixtures e resultados continuam nos recibos e no manifesto. [temporary-target-cleanup.json](temporary-target-cleanup.json).

[Lista dos 141 V16 pendentes e obrigações abertas](pendencias.md). math-stretch-composite permanece aberto. As obrigações CJK/show local e precedência de regras idênticas ainda declaradas no L0 de matching também precisam de revalidação própria; esta etapa não as dá por resolvidas.
