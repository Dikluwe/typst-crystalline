# Relatório — lote V16 calc/power

Etapa concluída. **V16 131 → 128**, V19 **1059 → 1060 notas**, V20 **867 notas** preservadas. O gate global continua aberto pelos 128 V16 restantes; não houve commit/push do Typst, merge ou release.

## Correções

O owner `00_nucleo/prompts/compiler/stdlib/calc/power.md` foi atualizado antes do código. A conversão explícita de Value delimita Int/Float/Decimal na base e Int/Float no expoente; os predicados de zero e o despacho usam enums privados exaustivos. Tipos não numéricos compartilham um único braço de erro, justificando a nota V19. Nenhuma supressão V16 ou alteração pública.

Corrigida a precedência: base inválida é rejeitada antes dos guards do expoente (por exemplo, `calc.pow(none,2147483648)`). Expoente decimal passa a receber o hint de conversão explícita. O erro de base agora inclui Decimal entre os tipos aceitos. Resultados e tipos numéricos, overflow, zero elevado a zero, erro decimal×float, sqrt/root e os demais diagnósticos locais foram preservados.

## Validação

Oito testes novos: RED funcional com três falhas e cinco controles passando; GREEN com oito passando. Suíte completa: **8750 passaram, zero falhas e 7 ignorados**. Os 20 casos CLI medidos antes da decisão foram repetidos: resultados/códigos preservados, diagnósticos alterados somente nos cinco casos previstos. Fonte `calc.rs:172-201` conferida byte a byte no pin `9f2b6e8715237cb086899a42873660fe744622e8`; binários e estado registrados em before-measurement.json.gz/after-measurement.json.gz. Isso não certifica paridade geral de mensagens, spans, infinitos ou fórmulas de powf/powi.

Sonda isolada rustc: as 39 variantes atuais compilam; uma variante futura é rejeitada com E0004 na conversão produtiva extraída. Payloads não numéricos são stubs nessa sonda, que prova cobertura estrutural, não execução completa do compilador. Não compartilhou target Cargo.

Linter pinado `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Build do workspace, fmt, Clippy estrito para todos os targets (incluindo expectativas não cumpridas), Rustdoc com -D warnings, V1/V5/V15/V26, linhagem reversa, diff check e diferencial SARIF completo passaram. O diferencial remove exatamente os três V16 selecionados e acrescenta somente o V19 prescrito; demais diagnósticos são idênticos após mapear linhas.

## RAM, proveniência e limites

Todos os comandos pesados foram sequenciais em cgroup com **12 GiB**, swap zero e Cargo jobs=1. Pico máximo registrado **12884901888 bytes (12.000 GiB)**: atingiu o teto rígido. O grupo registrou 1127 eventos memory.max, sem OOM nem OOM-kill. O limite foi mantido, porém houve pressão de memória; estas amostras agregadas não atribuem o pico a um processo ou gate específico. Em lotes futuros, separar gates pesados em cgroups sequenciais permite descartar a contabilidade acumulada entre etapas sem repetir validações já aprovadas. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, status completo, diffstat exato e SHA-256 de todos os fontes/L0s em before.json.gz, validation-result.json e final-state.json.gz. Apenas o par power.rs/power.md mudou desde o checkpoint; alterações anteriores foram preservadas.

A primeira medição com argumentos CLI errados está arquivada sem crédito funcional em attempt-cli-arguments. Um erro de retorno da nova conversão, corrigido antes do GREEN, está em attempt-result-type e também não conta como RED. Os logs reais do RED funcional e de todos os gates são preservados.

O [inventário restante](inventario-v16-restante.md) registra 128 casos em 97 consumers. A proposta pública RG1 de grupos de tabelas permanece aguardando aprovação ADR-0127; seu manifesto `a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9` foi revalidado integralmente e não foi alterado por este lote independente.
