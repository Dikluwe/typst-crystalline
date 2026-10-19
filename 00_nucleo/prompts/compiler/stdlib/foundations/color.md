# Prompt L0 — `stdlib/foundations/color` — construtores de cor
Hash do Código: c59f6b59

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/color.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: Passo 1032 — extraído de `foundations.rs`.
**ADRs**: ADR-0083 (color spaces), ADR-0107 (paridade linguagem).
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Funções

### `native_rgb` — `rgb(...)`

Forma numérica: Int [0,255] ou Ratio [0%,100%] por componente.
Forma hex: string 3/4/6/8 dígitos, `#` opcional. Mensagens verbatim do vanilla.

### `native_luma` — `luma(...)`

#### P1358 — medição anterior à decisão

No único vanilla ratificado `586e1bd43`,
`lab/typst-original/crates/typst-library/src/visualize/color.rs:341-374`
declara `lightness: Component`, `alpha: Component` e a alternativa
`color: Color`. Sem cor, `args.expect("lightness component")` exige o primeiro
positional; `args.eat()` consome um segundo positional opcional e aplica alpha
100% quando ausente. Os testes vanilla
`lab/typst-original/tests/suite/visualize/color.typ:172-177,212-231` congelam
alpha ratio/inteiro, argumento ausente, tipos inválidos, named arg inesperado
e positional excedente.

A medição bilateral P1358 confirmou: `luma(128)` coincide; `luma()` agora é
erro com hint para `white`/`luma(100%)`; `luma(128, 0)`, `luma(128, 255)` e
`luma(128, 40%)` são valores; `luma(128, alpha: 40%)` é erro; tipos e ranges
inválidos são diagnósticos, nunca fallback branco. P1357 repetiu o recorte nos
quatro perfis e encontrou a mesma classificação. Proveniência e tabela
integral: a medição histórica.

#### P1358 — decisão

As formas públicas do protocolo numérico são `luma(lightness)` e
`luma(lightness, alpha)`. `lightness` é obrigatório; `alpha` é o segundo
positional opcional, default 100%. Ambos usam `Component`: Int no intervalo
inclusivo 0–255 ou Ratio no intervalo inclusivo 0%–100%. Float, tipo errado e
valor fora do intervalo propagam os diagnósticos públicos do cast. `luma()`
produz `missing argument: lightness component` com hint para `white` ou
`luma(100%)`. Argumentos nomeados, inclusive `alpha:`, são inesperados; um
terceiro positional também é inesperado.

`Relative` sem parte absoluta pode continuar como compatibilidade interna do
consumer, mas não é forma pública adicional e não pode reintroduzir fallback
silencioso. Esta correção de paridade em função existente segue o fluxo
contínuo do ADR-0127: L0 primeiro, RED→GREEN e revalidação, sem checkpoint
adicional.

#### Scope-out preservado de `luma(color)`

P1252 permanece vigente neste passo: `luma(color)` delega em
`Color::to_space(ColorSpace::Luma)` e preserva o alpha da origem. O vanilla
ratificado ainda perde alpha ao converter uma cor não-Luma via
`Luma::from_color`; a divergência cristalina continua classificada
`Known-Upstream-Bug`, não paridade fechada. P1358 não muda essa transformação
e não a usa para alegar paridade integral de `luma`.

### `native_oklab` / `native_oklch` / `native_linear_rgb` / `native_cmyk` /
### `native_hsl` / `native_hsv`

Construtores dos respetivos espaços de cor. `linear_rgb` aceita Int/Ratio e
rejeita Float (P736). `native_cmyk` usa somente Ratio conforme P1284-v5.
`oklab`, `oklch`, `hsl` e `hsv` não podem ser agrupados como Float/Int: suas
assinaturas heterogéneas exatas são normatizadas em P1284-v6 abaixo.

### P1284-v5 — `native_cmyk` usa componentes Ratio

#### Medição antes da decisão

No snapshot histórico `a51e02804`, `visualize/color.rs:649-680` declara
`cmyk` com quatro `RatioComponent`; `RatioComponent` em `:2650-2663` aceita
somente `Ratio` no intervalo inclusivo 0%–100%. O exemplo público é
`cmyk(27%, 0%, 3%, 5%)`. A redação L0 anterior agrupava `cmyk` entre os
constructors Float/Int e, por isso, não legitimava a chamada real já usada
pelos oráculos P1284.

#### Decisão

`native_cmyk(cyan, magenta, yellow, key)` recebe exatamente quatro
posicionais Ratio, cada qual em 0%–100%, e produz a cor CMYK preservando os
quatro componentes normalizados em ordem. `color.cmyk` e o binding global
`cmyk` são aliases funcionais da mesma nativa; não há parser duplicado.

Float e Int não substituem Ratio nesta forma e devem produzir erro de tipo;
Ratio fora do intervalo produz `ratio must be between 0% and 100%`. O caminho
vanilla alternativo `cmyk(color)` foi medido, mas não é nova obrigação do
recorte P1284-v5; não pode ser usado para perdoar falha da forma de quatro
ratios.

A mudança é correção interna de cast/paridade sobre `Color::Cmyk` e funções já
existentes. Não altera enum, campo, assinatura Rust pública, default ou fase;
segue fluxo contínuo ADR-0127.

### P1284-v6 — tipos, unidades e ranges de `hsl`/`hsv`/`oklab`/`oklch`

#### Medição antes da decisão

A sonda validada `color-constructors-methods-and-aliases` em
`lab/surface-inventory/p1284-probes.json:196-199` exige chamadas reais
`hsl(0deg, 100%, 50%)`, `hsv(0deg, 100%, 100%)`,
`oklab(50%, 0, 0)` e `oklch(50%, 0, 0deg)`; as quatro devolvem `color` no
vanilla ratificado.

Na fonte vanilla `visualize/color.rs`, as assinaturas são:

- `oklab` em `:393-425`: `RatioComponent`, dois `ChromaComponent` e alpha
  `RatioComponent` opcional;
- `oklch` em `:452-489`: `RatioComponent`, `ChromaComponent`, `Angle` e alpha
  `RatioComponent` opcional;
- `hsl` em `:702-736` e `hsv` em `:759-793`: `Angle`, dois `Component` e alpha
  `Component` opcional.

Os casts em `visualize/color.rs:2651-2691` determinam os ranges: um
`RatioComponent` aceita exclusivamente Ratio inclusivo 0%–100%; um
`Component` aceita Int inclusivo 0–255 ou Ratio inclusivo 0%–100%; um
`ChromaComponent` aceita o cast Float sem clamp adicional ou Ratio escalado por
0,4, também sem clamp adicional. O cast primitivo `f64` em
`foundations/value.rs:619` inclui Int por coerção numérica, explicando os zeros
inteiros da sonda Oklab/Oklch. `Angle` possui unidades `deg` e `rad`
(`layout/angle.rs:12-17`); estes constructors não impõem um range angular de
rejeição e convertem a magnitude por `to_deg` para a representação circular.

#### Decisão

| Constructor | Posicionais obrigatórios | Quarto positional opcional |
|---|---|---|
| `hsl` | `hue: Angle(deg/rad)`, `saturation: Component`, `lightness: Component` | `alpha: Component = 100%` |
| `hsv` | `hue: Angle(deg/rad)`, `saturation: Component`, `value: Component` | `alpha: Component = 100%` |
| `oklab` | `lightness: RatioComponent`, `a: ChromaComponent`, `b: ChromaComponent` | `alpha: RatioComponent = 100%` |
| `oklch` | `lightness: RatioComponent`, `chroma: ChromaComponent`, `hue: Angle(deg/rad)` | `alpha: RatioComponent = 100%` |

Para esta tabela:

- `Component` = Int 0–255 ou Ratio 0%–100%, ambos inclusivos; Float é erro de
  tipo;
- `RatioComponent` = somente Ratio 0%–100%, inclusivo; Int/Float são erro de
  tipo;
- `ChromaComponent` = Float (incluindo Int coerçível para `f64`) em valor
  literal, ou Ratio multiplicado por 0,4; este cast não acrescenta limite
  mínimo/máximo;
- `Angle` = valor angular com unidade `deg` ou `rad`; o constructor não rejeita
  por estar fora de uma volta.

As formas globais e qualificadas `color.hsl`, `color.hsv`, `color.oklab` e
`color.oklch` são aliases funcionais das mesmas quatro nativas e não duplicam
casts. Ordem de componentes, normalização definida acima e alpha default 100%
são observáveis. Int fora de `Component` produz
`number must be between 0 and 255`; Ratio fora de `Component` ou
`RatioComponent` produz `ratio must be between 0% and 100%`.

As alternativas vanilla de conversão a partir de uma única `Color` foram
medidas, mas não são novas obrigações do recorte P1284-v6; não perdoam falha
das quatro formas posicionais sentinela.

## P1286 — conversão `rgb(color)` usada pelo oráculo de mix

### Medição anterior à decisão

Os casos congelados P1286 observam `rgb(color.mix(...))`: o construtor aceita
uma única `Color`, converte o resultado para sRGB e preserva alpha. Sem essa
forma, uma implementação correta do mix fica inobservável na superfície da
linguagem e produz erro de cardinalidade.

### Decisão

`rgb(color)` devolve `Value::Color` no espaço sRGB; não retorna string e não
altera as formas hexadecimal ou posicionais. Para os espaços já exatos usa a
conversão canônica da entidade. Para CMYK, cujo ICC completo continua
explicitamente fora de `entities/color.md`, o bridge aplica uma correção
cruzada determinística ao modelo subtractivo, calibrada pelo vetor congelado
P1286 `cmyk(59.15%,40.35%,49.15%,11.63%) -> #6b7c76`. Isto fecha somente o
fragmento medido e não autoriza alegar paridade ICC geral. Trata-se de correção
interna/cast, sem API Rust, default ou fase nova, em fluxo contínuo ADR-0127.

Esta é correção de paridade/cast em nativas e entidades já existentes. Não
altera contrato Rust público, default de produto ou fase e segue fluxo contínuo
ADR-0127.

## 2. Critérios de verificação

```
rgb(255, 0, 128)              -> Color::rgb(...)
rgb(50%, 0%, 0%)              -> rgb("#800000")
rgb("#FF0000")                -> vermelho opaco
rgb(300, 0, 0)                -> Err "number must be between 0 and 255"
rgb("FFFFF")                  -> Err "color string has wrong length"
luma(128)                     -> cinza 50%, alpha 100%
luma(128, 40%)                -> cinza 50%, alpha 40%
luma(128, 0)                  -> cinza 50%, alpha 0%
luma(128, 255)                -> cinza 50%, alpha 100%
luma(128, alpha: 40%)         -> Err "unexpected argument: alpha"
luma(300)                     -> Err "number must be between 0 and 255"
luma(150%)                    -> Err "ratio must be between 0% and 100%"
luma("bad")                  -> Err "expected integer or ratio, found string"
luma()                        -> Err "missing argument: lightness component" + hint
luma(rgb(..., 40%))           -> Luma com alpha 40% (scope-out Known-Upstream-Bug)
linear_rgb(255, 0, 0)         -> vermelho linear
linear_rgb(0.5, 0.5, 0.5)     -> Err "expected integer or ratio, found float"
cmyk(0%, 100%, 0%, 0%)         -> magenta
cmyk(0%, 0%, 0%, 100%)         -> CMYK key 100%
cmyk(0.0, 1.0, 0.0, 0.0)      -> erro de tipo (Float não é Ratio)
hsl(0deg, 100%, 50%)            -> HSL vermelho opaco
hsv(0deg, 100%, 100%)           -> HSV vermelho opaco
oklab(50%, 0, 0)                -> Oklab; lightness Ratio, a/b Chroma
oklch(50%, 0, 0deg)             -> Oklch; lightness Ratio, chroma, hue Angle
hsl(0deg, 1.0, 50%)             -> erro de tipo (Float não é Component)
oklab(50, 0, 0)                 -> erro de tipo (Int não é RatioComponent)
```
## P1482 — constructors processuais recebem cor spot (CONFIRMADO em 2026-09-23; ADR-0127)

### Medição anterior à decisão

Vanilla `visualize/color.rs:1326-1396` projeta spot no fallback
tingido ao converter explicitamente para espaço de processo. Sonda
`rgb(color.spot("A", red).tint(50%))` representa
`rgb("#ffa09a")`, enquanto o spot original conserva tipo/repr
próprios. No cristalino `foundations/color.rs:26-58,61-75` aceita
`Value::Color` no constructor `rgb` mas assume `Color: Copy`.

### Decisão proprietária

`rgb`, `luma`, `cmyk`, `oklab` e demais constructors de processo
que já aceitam Color tratam Spot pela projeção explícita
`Color::to_process`; o resultado é cor processual, sem identidade
spot. Não antecipar essa projeção no Value, no style ou em cast de
fill/stroke. Preservar diagnósticos e fórmulas existentes dos inputs
processuais.

## Correção medida em 2026-10-13

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
Fontes, hashes, UTC, status e diffstat: `lab/results/construction/diagnostics/cores-ci-2026-10-13/before.json.gz`
e `constructors-before.json.gz`. Sondas e prova numérica em `evidence.json.gz`.
Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 ratificado
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Medição anterior à decisão: native_oklab/oklch/linear_rgb/cmyk/hsl/hsv só têm
ramos de componentes; as seis sondas com rgb(0,0,0,40%) falham por cardinalidade.
O vanilla visualize/color.rs:393-809 usa args.find<Color>() nessas seis funções.
Oklab/Oklch/LinearRgb/Hsl/Hsv preservam alpha; Cmyk tem quatro componentes sem
alpha. Oklch de Oklch preserva componentes. Color mais positional excedente
produz unexpected argument; named e Colorant inválidos também são rejeitados,
com diferenças preexistentes de diagnóstico registradas.

Decisão: completar a alternativa Color nas seis nativas existentes, superando
o scope-out das alternativas de P1284-v5/v6. Usar conversão canônica to_space
para o target correspondente, preservando mesmo espaço e projeção Spot já
contratada; não inventar ICC CMYK nem corrigir fórmulas de outras entidades
no parser. Nenhum binding, assinatura pública Rust, flag, default ou fase muda:
é correção interna de paridade/cast, ADR-0127 contínuo.

Uma única Color positional é convertida; ao haver Color e positionais
excedentes, rejeitar o primeiro positional não consumido com unexpected
argument e seu span. Named args continuam rejeitados pela política existente;
não apresentar seus diagnósticos ainda divergentes como paridade integral.
As formas por componentes, seus casts/ranges e rejeições permanecem.
Não transformar SpotColorant em Color implicitamente. Testar antes do reparo:
seis targets, alpha onde suportado, identidade de mesmo espaço, excessos e
aliases via sondas do produto. Confirmar RED de aceitação Color e GREEN após
reparo. Testes e sondas não certificam ICC/CMYK geral nem eliminam a diferença
já medida de gradient.sample(25%). Esta permanece pendente de correção própria.

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

Neste owner: rgb numérico deve conservar Component como f32: Int validado
0..255 dividido por 255 e Ratio 0..1 sem quantização. O arredondamento para
bytes é da projeção hex/export, não da construção. A cláusula antiga que
inferia armazenamento u8 de rgb(50%).to-hex() é superada pela medição de
components. rgb(Color) usa a conversão Srgb canônica sem ida a bytes, salvo
o bridge CMYK calibrado já normatizado em P1286, que permanece explícito e
incompleto quanto a ICC geral. String hex preserva o caminho em bytes.

Linear RGB usa o mesmo cast Component validado por índice, incluindo alpha,
para rejeitar Int/Ratio fora do domínio com mensagem e span causais. Reutilizar
o caster privado existente, não duplicar sua lógica. Eliminar as_u8 privado
sem consumer produtivo após o reparo e migrar seus controles Relative para o
caster ativo, mantendo fixtures de tipo/range e medindo 50% como 0.5.
Testar RED de precisão RGB numérica e Color→RGB, identidade e ranges de Linear.
Preservar o vetor CMYK já medido e formas válidas hex/Int/Ratio/conversão.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T243: 01_core/src/compiler/stdlib/foundations/color.rs:375; T244: 01_core/src/compiler/stdlib/foundations/color.rs:476. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Nos dois casters de Relative, manter guard simples de abs.is_zero; decidir range inclusivo 0..=1 no corpo. Preservar ordem abs antes de range, casts f32, rejeição de NaN/infinito/out-of-range, erro de ratio quando abs zero e erro de tipo quando abs não zero. Adicionar controle funcional de Relative em 0/1/interior, NaN/infinito e parte absoluta não zero para os dois casters antes da mudança.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.
