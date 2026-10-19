# L0 — entities/color/conversion
Hash do Código: 836c47af

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/entities/color.rs:280-714`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/conversion.rs`. Converter cores entre espaços de processo e representar sRGB em bytes e floats. Preservar arredondamento, fórmulas e identidade spot; manter as assinaturas públicas existentes. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.

## Representable f32 constants — 2026-10-13

Measurement: the Rust 1.92 Clippy inventory identifies excessive decimal
precision in this consumer's f32 matrix coefficients. All 50 suggested literal
replacements were compiled independently as f32 and compared with `to_bits()`;
every pair is identical. Source, original/proposed spellings, full probe and
working-tree provenance are conserved in
`lab/results/construction/diagnostics/qualidade-ci-2026-10-13/color-literals-before.json.gz`.
This is a mechanics-preservation check, not a new language parity claim.

Decision: use the measured representable spellings, preserving each f32 value,
formula, operation order, signature and output. Do not apply these shortened
spellings to f64 contexts or change transcendental implementations in this
repair. Existing color and conversion behavior tests remain mandatory.

## Correção medida em 2026-10-13

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Fontes, hashes, UTC, status e diffstat: `lab/results/construction/diagnostics/cores-ci-2026-10-13/before.json.gz`
e `constructors-before.json.gz`. Sondas e prova numérica em `evidence.json.gz`.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 ratificado
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Medição anterior à decisão: `clippy.toml` proíbe sin/cos/atan2 nativos de f32;
conversion.rs:60-61/246 e interpolation.rs:200 ainda os usam. O upstream
configura palette sem default-features, com libm (`lab/typst-original/Cargo.toml`);
`palette-0.7.6/src/num/libm.rs:5-41` chama sinf/cosf/atan2f. Os lockfiles fixam
libm 0.2.11. As sondas de cores/gradientes capturam resultados, alpha e hue;
registram também a rejeição preexistente de construtores Color e a diferença
preexistente no chroma de gradient.sample(25%), sem normalizá-las.

Decisão interna contínua (ADR-0127): usar sinf/cosf/atan2f da libm pinada nas
chamadas correspondentes deste owner; whitelist explícita desses símbolos.
Não alterar fórmula, ordem de operações, alpha, caminhos de hue, identidade
spot, API, default ou fase. A mecânica transcendental é escolha de implementação;
aceitação de linguagem observa componentes, tipos, cores e alpha.

Substituir o smoke vazio por testes de conversão com referências independentes
medidas no oráculo, quadrantes e alpha. A exclusão de mudança transcendental
na rodada anterior limitava aquela rodada; esta decisão separada a autoriza.

Preservar regressões de color/gradient, compilar e selar ownership/hashes.

## Limites e precisão de cor — 2026-10-13

Medição anterior à decisão, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
working tree não commitado: fontes, SHA-256, UTC, status e diffstat em
`lab/results/construction/diagnostics/limites-cor-2026-10-13/before.json.gz` e
`parents-before.json.gz`; sondas em `evidence.json.gz`. Oráculo pinado
`9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

`red.rotate(45deg)` expõe blue negativo no cristalino; o vanilla limita a zero.
`rgb(10%,20%,30%,40%)` expõe 10.2%/30.2% em vez de 10%/30%. Linear RGB aceita
256 e alpha -1%, que o vanilla rejeita. Chroma Oklch negativo converte para
cor cromática no cristalino e cinza no vanilla. As saídas completas ficam
conservadas, inclusive dígitos adicionais dos componentes públicos.

Fonte: upstream `visualize/color.rs:1825-1895` distingue FromColor limitado,
identidade e from_linear; palette 0.7.6 `convert/from_into_color.rs:53-59`
chama clamp após converter. `encoding/srgb.rs:100-120` usa multiplicação pelo
recíproco na parte linear do decode e mul_sub no encode; `num.rs:718-722`
implementa mul_sub SEM fusão. A antiga cláusula P1253 de encode com fmaf era
incorreta para esta fonte. `oklab.rs:355-368` limita chroma a zero antes da
conversão polar; `hues.rs:155-157` usa sincosf via o trait libm.

Correção interna contínua ADR-0127: nenhum campo/assinatura pública, binding,
modo padrão, compatibilidade externa ou fase muda. L0 primeiro, testes RED
para observáveis, implementação e revalidação. Diferenças restantes das
sondas não são normalizadas nem recebem certificado genérico de paridade.

Neste owner: codificar sRGB por powf * 1.055 - 0.055 sem fusão; manter decode
fused na parte exponencial e usar (1/12.92 como f32) * c na parte linear.
A conversão polar privada limita chroma mínimo a zero e usa sincosf da libm
pinada; reutilizar em Oklch→sRGB e Oklch→LinearRgb. Autorizar sincosf na whitelist.

Em to_space(Srgb), aplicar limites 0..1 aos canais/alpha nos caminhos que o
upstream converte com FromColor; preservar a identidade Srgb e o caminho
LinearRgb→Srgb por from_linear, que não acrescenta clamp. O helper bruto
to_rgba_f32 continua apropriado aos consumers existentes e não passa a impor
clamp global. Em to_space(LinearRgb), Oklab/Oklch convertem diretamente pela
matriz, sem encode+decode intermediários; limitar os resultados convertidos
como FromColor. Demais fontes usam o decode existente; identidade mantém-se.

Testar canais de rotate e re-conversão Oklch, limites dos targets, controle
LinearRgb→Srgb bruto, alpha, chroma negativo e valores públicos independentes
medidos. Não trocar fórmulas CMYK/ICC nem outros espaços neste reparo.

### Refinamento medido da normalização de hue

A primeira implementação ainda falhou no teste de rotate; a tolerância não foi
alterada. `limites-cor-2026-10-13/hue-intermediate.json` (conservado em
`evidence.json.gz`) mede L/chroma iguais, mas hue 28.442533493041992 no vanilla
e 28.442523956298828 no cristalino. Após +45deg a diferença chega ao canal RGB.
A fonte palette 0.7.6 `hues.rs:134-146` usa PI + atan2(-b,-a), depois graus;
os helpers próprios usavam atan2(b,a) e rem_euclid, matematicamente próximos
mas com arredondamento observável diferente em f32.

Decisão interna: usar a normalização medida PI + libm::atan2f(-b,-a), depois
graus e normalização circular existente, nos helpers Color→Oklch deste owner.
Adicionar controle independente de hue do vermelho nominal, mantendo os
quadrantes e as tolerâncias anteriores. Não corrigir a falha afrouxando a
tolerância de rotate nem por constante especial para red.

### Exaustividade nominal do fallback LinearRgb

Medição posterior ao reparo numérico: o SARIF preservado em
`lab/results/construction/diagnostics/limites-cor-2026-10-13/evidence.json.gz` aponta V16
no novo match de conversion.rs:383. O enum Color atual tem nove variantes;
Oklab/Oklch possuem rotas diretas, e o braço restante usa o decode existente.

Enumerar nominalmente Srgb, Luma, LinearRgb, Hsl, Hsv, Cmyk e Spot nesse
braço, preservando seu corpo e a semântica para todas as variantes atuais.
Novas variantes passam a exigir decisão explícita do compilador. Esta é
uma correção interna de exaustividade, sem mudança de contrato público.
Validar os testes de conversão e o desaparecimento desse V16 específico.


## Exaustividade de representação e identidade — V16, 2026-10-16

### Medição anterior à decisão

01_core/src/entities/color/conversion.rs:230. Fonte e owner vigentes lidos e conferidos por hash antes da decisão. `lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` conserva HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, fontes/L0/configuração, status e git diff HEAD --stat. O SARIF anterior registra onze avisos e uma nota V16 nestes sete owners. Foram também medidos os fallbacks sem emissão; estes fazem parte do mesmo recorte. Classificação: padrão Rust e evolução do enum são mecânica; resultados/campos/identidade observáveis e erros devem permanecer. Não se infere intenção upstream ou ganho para IA.

### Responsabilidade exclusiva

to_oklab_p476: fallback por sRGB nomeia Srgb/LinearRgb/Oklch/Hsl/Hsv/Cmyk/Spot, conservando rotas diretas Oklab e Luma. to_oklch_p476: fallback nomeia todas as variantes exceto Oklch. to_space para target Spot: ProcessToSpot nomeia os oito espaços processuais; os dois braços Spot e sua ordem ficam. to_process: clone processual nomeia os oito espaços processuais; Spot projeta como antes. Nenhuma fórmula, ordem de operações, arredondamento, conversão/erro ou identidade muda; a nominalização inclui fallbacks que a heurística não emite.

### Aceitação e gate

Manutenção interna ADR-0127 em fluxo contínuo: nenhuma definição de enum, campo, trait, assinatura pública, default, dependência, compatibilidade ou fase muda. Conservar cada corpo, ordem e guard; não mover lógica entre owners nem criar helper para esconder o catchall. Padrões nominais devem forçar erro de compilação ao surgir variante nova; comentários de neutralidade não servem como supressão.

Antes do código, testes funcionais locais e gate V16 RED por escopo. Executar sonda de evolução em cópia descartável do workspace: acrescentar somente uma variante sentinela a Color, ColorSpace, MathStyleKind, FuncRepr e StateUpdate, sem alterar as definições produtivas. Usar cargo check --offline --locked -p typst-core com um job e cache de artefatos compartilhado; registrar fontes/hashes/argv/UTC/diagnósticos. Para cada um dos 21 matches selecionados, a baseline deve demonstrar a ausência de E0004 naquele match; GREEN exige E0004 ali, independentemente de erros de exaustividade já emitidos por outros matches na cópia. O controle de compilação sem sentinelas e os testes de comportamento aprovam o código real; não usar a sonda negativa como prova de paridade. Não editar source produtivo durante comandos.

Resselo restrito, direto e reverso, sem reparo global da working tree. Revalidar build, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, formato, suíte workspace completa, Rustdoc -D warnings e quatro índices HTML, linhagem V1/V5/V15/V26 estrita/reversa, SARIF sem avisos novos e diff --check. Limite MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1, tarefas pesadas sequenciais. Nenhuma tolerância/fixture histórica é relaxada. As pendências V16 externas, inclusive T027/T158/T160 rastreadas sem emissão, continuam abertas.
