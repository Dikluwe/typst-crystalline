# Passo 1320 — verificar as lacunas reais de paridade após P1319

Estado: executado; auditoria concluída, paridade aberta.

Objetivo: responder o que falta para paridade, com evidência atual, distinguindo
divergências de linguagem/diagnóstico, lacunas de medição e backlog obsoleto.
Implementar este passo significa construir e executar a verificação; não
implementar indiscriminadamente as funcionalidades que ela encontrar.

Escopo: auditoria focal de CSV e sondas transversais da matriz pública vigente,
contra vanilla ratificado upstream/main `a51e02804`. Não inventar percentual
global nem promover smoke, nomes presentes ou sucesso do harness a paridade.

1. Preservar o working tree P1319, sem commit, resselo ou leitura de passos antigos.
2. Fixar HEAD, diff completo/stat, SHA dos binários, código e ferramentas; registrar UTC.
3. Implementar runner diagnóstico e testar seus classificadores, com Unknown explícito.
4. Remedir CSV: valores, Bytes/Path/Str, erros UTF-8 válido/inválido, I/O,
   ordem de validação, Symbol, argumentos e base capturada. Conferir `csv.encode`
   na fonte e no runtime antes de repetir sua classificação histórica.
5. Reexecutar a matriz pública existente com binário P1319 explícito e perfis
   default/html/a11y-extras. Separar resultado observado da expectativa histórica.
6. Publicar inventário priorizado em `diagnosticos/`, indicando para cada frente
   prova, limites, hipótese/refutação e critério de fechamento futuro.

Regime: investigação somente leitura do produto; segregação não aplicável,
sem alegação de independência. Ferramentas/fixtures diagnósticas não materializam
L1–L4; nenhum contrato L0 produtivo muda. Mudanças futuras de API/default/fase
ou compatibilidade obedecerão ADR-0127 em passo próprio.

Temporários em diretório exclusivo de teste; não sobrescrever evidências anteriores.
Critério de encerramento: runner testado, medições rastreáveis e relatório útil
com diferenças/Unknown preservados. O passo pode estar concluído com paridade RED.

## Resultado

Verificador diagnóstico implementado e executado; testes do classificador e
do runner existente passam. Matriz transversal e corpus focal remediram o
produto P1319, sem alterações produtivas. Controles separados distinguem Path
capturado válido de string em closure resolvida no diretório errado.

Foram confirmadas lacunas de argumentos, resolução, Symbol e diagnósticos CSV,
além do construtor array(Bytes). `csv.encode` foi retirado do backlog de paridade
porque também não existe no vanilla ratificado. Export PDF/PNG requer classificar
diferenças antes de propor reparos. Cobertura ampla permanece não verificada.

Relatório substantivo e priorização:
`00_nucleo/diagnosticos/p1320-o-que-falta-para-paridade.md`.
Proveniência e integridade:
`00_nucleo/diagnosticos/p1320-reconciliation.json`.
Falhas de invocação/sonda foram preservadas e corrigidas focalmente, não contadas
como defeitos do produto. Nenhuma evidência P1319 alterada; sem commit ou push.
