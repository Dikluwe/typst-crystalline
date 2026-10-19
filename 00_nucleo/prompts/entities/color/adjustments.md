# L0 — entities/color/adjustments
Hash do Código: c2ea3e2f

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), o comportamento está em `entities/color.rs:325-390,530-608,619-666`. Trata-se de delimitação de código vigente; não há medição de desempenho nova.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/adjustments.rs`. Aplica clareamento, escurecimento, negação, rotação, saturação e ajuste de alpha no espaço da cor. Preserva fórmulas, semântica de spot e as falhas representadas por Option. Mantém as assinaturas públicas, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

Testes L1, build e crystalline-lint passam. Sem inferência de ganho de desempenho.


## Exaustividade de representação e identidade — V16, 2026-10-16

### Medição anterior à decisão

01_core/src/entities/color/adjustments.rs:141. Fonte e owner vigentes lidos e conferidos por hash antes da decisão. `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, status e git diff HEAD --stat. O SARIF anterior registra onze avisos e uma nota V16 nestes sete owners. Foram também medidos os fallbacks sem emissão; estes fazem parte do mesmo recorte. Classificação: padrão Rust e evolução do enum são mecânica; resultados/campos/identidade observáveis e erros devem permanecer. Não se infere intenção upstream ou ganho para IA.

### Responsabilidade exclusiva

darken: o braço que delega a lighten(-factor) nomeia Srgb/Luma/LinearRgb/Oklab/Oklch/Hsl/Hsv; Spot e Cmyk continuam com seus corpos prioritários. rotate: o braço unreachable nomeia Srgb/Luma/LinearRgb/Oklab/Cmyk/Spot, preservando mensagem e os três braços hue válidos. saturate: a conversão via Hsv nomeia Srgb/LinearRgb/Oklab/Oklch/Cmyk; Luma/Spot continuam None e Hsl/Hsv continuam no espaço próprio. Só os padrões mudam, sem tocar chamadas, fórmula, alpha, clamp ou tratamento de erro.

### Aceitação e gate

Manutenção interna ADR-0127 em fluxo contínuo: nenhuma definição de enum, campo, trait, assinatura pública, default, dependência, compatibilidade ou fase muda. Conservar cada corpo, ordem e guard; não mover lógica entre owners nem criar helper para esconder o catchall. Padrões nominais devem forçar erro de compilação ao surgir variante nova; comentários de neutralidade não servem como supressão.

Antes do código, testes funcionais locais e gate V16 RED por escopo. Executar sonda de evolução em cópia descartável do workspace: acrescentar somente uma variante sentinela a Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate, sem alterar as definições produtivas. Usar cargo check --offline --locked -p typst-core com um job e cache de artefatos compartilhado; registrar fontes/hashes/argv/UTC/diagnósticos. Para cada um dos 21 matches selecionados, a baseline deve demonstrar a ausência de E0004 naquele match; GREEN exige E0004 ali, independentemente de erros de exaustividade já emitidos por outros matches na cópia. O controle de compilação sem sentinelas e os testes de comportamento aprovam o código real; não usar a sonda negativa como prova de paridade. Não editar source produtivo durante comandos.

Resselo restrito, direto e reverso, sem reparo global da working tree. Revalidar build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, suíte workspace completa, Rustdoc -D warnings e quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF sem avisos novos e diff --check. Limite MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1, tarefas pesadas sequenciais. Nenhuma tolerância/fixture histórica é relaxada. As pendências V16 externas, inclusive T027/T158/T160 rastreadas sem emissão, continuam abertas.
