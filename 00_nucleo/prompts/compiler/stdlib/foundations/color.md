# Prompt L0 — `stdlib/foundations/color` — construtores de cor
Hash do Código: 40355d04

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

Aceita as formas públicas `luma()`, `luma(lightness)`,
`luma(lightness, alpha: alpha)` e `luma(color)`. `lightness` e `alpha` aceitam Int
[0,255], Ratio ou `Relative` sem parte absoluta. Ausência de argumento e
componente numérico inválido preservam o fallback branco P705; `alpha` é
exclusivamente nomeado e dois posicionais continuam erro estrutural.

P1252: `luma(color)` delega em `Color::to_space(ColorSpace::Luma)` e preserva
o alpha da origem. Esta preservação diverge deliberadamente do vanilla
ratificado `a51e02804`, que perde alpha ao converter uma cor não-Luma via
`Luma::from_color`; a divergência é classificada `Known-Upstream-Bug`, não
paridade fechada. `luma(lightness, alpha: alpha)` permite observar publicamente que
uma cor que já nasce Luma conserva transparência.

### `native_oklab` / `native_oklch` / `native_linear_rgb` / `native_cmyk` /
### `native_hsl` / `native_hsv`

Construtores dos respetivos espaços de cor. `linear_rgb` aceita Int/Ratio e
rejeita Float (P736). `native_cmyk` usa somente Ratio conforme P1284-v5.
`oklab`, `oklch`, `hsl` e `hsv` não podem ser agrupados como Float/Int: suas
assinaturas heterogéneas exatas são normatizadas em P1284-v6 abaixo.

### P1284-v5 — `native_cmyk` usa componentes Ratio

#### Medição antes da decisão

No vanilla ratificado `a51e02804`, `visualize/color.rs:649-680` declara
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

A sonda selada `color-constructors-methods-and-aliases` em
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
luma(128)                     -> cinza 50%
luma(128, alpha: 40%)         -> cinza 50% com alpha 40%
luma(rgb(..., 40%))           -> Luma com alpha 40% (Known-Upstream-Bug)
luma(300)                     -> branco (fallback silencioso)
luma()                        -> branco
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
