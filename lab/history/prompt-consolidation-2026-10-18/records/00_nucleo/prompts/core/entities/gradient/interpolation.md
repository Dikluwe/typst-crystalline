# L0 — entities/gradient/interpolation
Hash do Código: e589a56f

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/entities/gradient/interpolation.rs`. No HEAD `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229` e árvore limpa, `01_core/src/entities/gradient.rs:285-620` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.

## Contrato

Converter e interpolar cores nos espaços suportados, preservando alpha e caminhos de hue. Preservar contratos públicos, defaults, semântica de amostragem e os critérios de paridade em `00_nucleo/prompts/core/entities/gradient.md`. A extração permanece em L1, sem I/O nem estado global mutável.

## Aceitação

Os testes de gradiente e a suíte L1 passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

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

As 18 sugestões de grafia de coeficientes foram compiladas como f32 e comparadas
por to_bits: todas idênticas antes da alteração. Aplicar somente essas grafias,
com os mesmos valores e ordem; não truncar valores nem decompor fmaf/gamma.
A retificação do owner fachada entities/gradient explicita essa distinção.
Adicionar controles de quadrantes, alpha e hue curto à interpolação.

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

Neste owner, corrigir somente os dois helpers de decode sRGB na parte linear:
(1.0_f64 / 12.92) como f32, multiplicado pelo componente, conforme fonte medida.
Não alterar pesos, caminho de hue ou sampling neste reparo. A diferença de
chroma em sample(25%) permanece aberta com sua medição anterior. A ordem das
demais operações e as grafias f32 já comprovadas permanecem.

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


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T289: 01_core/src/entities/gradient/interpolation.rs:107. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Unir par Spot/Spot e mover a comparação de colorants ao corpo. Compatível conserva Arc de a e a mesma fórmula de tint; incompatível reconstrói os mesmos Color::Spot para projeção processual e interpolate_oklab. O par genérico continua igual. Preservar clamp, alpha, identidade e ordem numérica sem novas alocações/clones.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.
