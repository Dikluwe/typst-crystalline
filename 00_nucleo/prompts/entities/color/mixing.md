# L0 — entities/color/mixing
Hash do Código: f352354a

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), o comportamento está em `entities/color.rs:391-529`. Trata-se de delimitação de código vigente; não há medição de desempenho nova.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/mixing.rs`. Converte componentes para vetores no espaço pedido e mistura duas ou N cores, preservando o caminho circular de hue, os pesos e a identidade de spot. Mantém as assinaturas públicas, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

Testes L1, build e crystalline-lint passam. Sem inferência de ganho de desempenho.

## Iteração e parâmetros internos — 2026-10-13

### Medição anterior à decisão

`01_core/src/entities/color/mixing.rs` nas linhas 14 contém os avisos selecionados do Clippy. O snapshot anterior à decisão, spans conferidos contra a fonte, owners, referências, binário, UTC, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, status e diffstat estão em `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`. A inspeção mede mecânica interna e usos; não é nova prova bilateral de paridade ou ganho de desempenho.

### Decisão e aceitação

Renomear apenas o helper privado consumidor de self para into_vec4_in_space e atualizar suas chamadas no mesmo consumer. Preservar consumo, clones, fallback, conversões e pesos; não mudar as assinaturas públicas de mix/mix_weighted ou tornar o helper público. Não alterar fórmulas nem alegar melhoria da paridade colorimétrica desta renomeação.

Refatoração interna equivalente em fluxo contínuo ADR-0127, sem contrato público, default ou fase novos. Conservar as expectativas dos testes existentes; não criar testes que apenas espelhem estes loops. Validar suíte core (incluindo conflitos, spans, fixups, marcadores e seleção diferencial), todos os targets, build, documentação, Clippy e linhagem com resselo restrito. Nenhuma supressão adicional de lint é autorizada por este lote.


## Exaustividade de representação e identidade — V16, 2026-10-16

### Medição anterior à decisão

01_core/src/entities/color/mixing.rs:71; 01_core/src/entities/color/mixing.rs:116. Fonte e owner vigentes lidos e conferidos por hash antes da decisão. `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, status e git diff HEAD --stat. O SARIF anterior registra onze avisos e uma nota V16 nestes sete owners. Foram também medidos os fallbacks sem emissão; estes fazem parte do mesmo recorte. Classificação: padrão Rust e evolução do enum são mecânica; resultados/campos/identidade observáveis e erros devem permanecer. Não se infere intenção upstream ou ganho para IA.

### Responsabilidade exclusiva

mix/mix_weighted: os dois hue_idx nomeiam Srgb/Luma/LinearRgb/Oklab/Cmyk/Spot como sem canal angular; Oklch e Hsl/Hsv preservam índices 2 e 0. O default de mix conserva primeiro o par Spot compatível e depois fallback Oklab com primeiro eixo nominal das nove Color. O default de mix_weighted conserva primeiro Spot condicionado à compatibilidade de todos e depois fallback Oklab com as nove variantes nominais. Manter guards, ordem e curto-circuito, aliases/clones, arrays, pesos, rota circular e chamadas sem mudar corpos. Não introduzir helpers ou normalizações.

### Aceitação e gate

Manutenção interna ADR-0127 em fluxo contínuo: nenhuma definição de enum, campo, trait, assinatura pública, default, dependência, compatibilidade ou fase muda. Conservar cada corpo, ordem e guard; não mover lógica entre owners nem criar helper para esconder o catchall. Padrões nominais devem forçar erro de compilação ao surgir variante nova; comentários de neutralidade não servem como supressão.

Antes do código, testes funcionais locais e gate V16 RED por escopo. Executar sonda de evolução em cópia descartável do workspace: acrescentar somente uma variante sentinela a Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate, sem alterar as definições produtivas. Usar cargo check --offline --locked -p typst-core com um job e cache de artefatos compartilhado; registrar fontes/hashes/argv/UTC/diagnósticos. Para cada um dos 21 matches selecionados, a baseline deve demonstrar a ausência de E0004 naquele match; GREEN exige E0004 ali, independentemente de erros de exaustividade já emitidos por outros matches na cópia. O controle de compilação sem sentinelas e os testes de comportamento aprovam o código real; não usar a sonda negativa como prova de paridade. Não editar source produtivo durante comandos.

Resselo restrito, direto e reverso, sem reparo global da working tree. Revalidar build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, suíte workspace completa, Rustdoc -D warnings e quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF sem avisos novos e diff --check. Limite MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1, tarefas pesadas sequenciais. Nenhuma tolerância/fixture histórica é relaxada. As pendências V16 externas, inclusive T027/T158/T160 rastreadas sem emissão, continuam abertas.
