# Prompt L0 — stdlib tipo `color` (operadores de cor)
Hash do Código: dfc7912c

## Módulo
`01_core/src/compiler/stdlib/color.rs`

## Camada
L1 (puro; sem I/O; sem estado global).

## Propósito

**P736 — estado vigente:** `color` é exposto no scope de eval como
`Value::Type(Type::Color)` (paridade vanilla — medido: `type(color)` →
`type`; `color("#f00")` → erro "type color does not have a constructor").
Os fields do tipo (`color.rgb`, `color.lighten`, …) resolvem-se por field
access em `Value::Type` via `color_type_field(field) -> Option<Value>`
(padrão P685, mesmo mecanismo de `int.min`/`str.from-unicode`).

Histórico: até P736, `color` era `Value::Dict` (P476, módulo de 4
operadores; P477 aumentou para 6).

Fields do tipo (20): constructors `rgb`, `linear-rgb`, `luma`, `cmyk`,
`hsl`, `hsv`, `oklab`, `oklch` (as mesmas funções nativas registadas
globalmente) + operadores `lighten`, `darken`, `mix`, `negate`,
`saturate`, `desaturate`, `rotate`, `components`, `space` (P742) +
`to-hex`, `transparentize`, `opacify` (P744).

**P1143:** o namespace contém ainda as 18 cores predefinidas ratificadas,
totalizando 38 fields. As cores e os bindings globais consultam uma única
tabela canônica no owner.

Medições vanilla que fundamentam (P736):
- `type(color)` → `type`; `type(red) == color` → `true`; `repr(color)` → `color`.
- `type(color.rgb)` … `type(color.oklch)` → `function` (8 constructors).
- `type(color.lighten)` … `type(color.desaturate)` → `function`; o vanilla
  tem ainda `rotate`, `components`, `space` — **ausentes no cristalino**
  (scope-out; também ausentes como métodos de instância).
- `color("#ff0000")` → erro "type color does not have a constructor"
  (sem constructor — o despacho P685 já emite esta mensagem).
- `color.foo` → erro "type color does not contain field `foo`".

P476 — fecho parcial ADR-0083 §"Operadores cor" scope-out:
4 dos 6 operadores implementados (`saturate`/`desaturate` scope-out futuro P477).

## Função de despacho de fields (P736)

```rust
/// Devolve o valor associado a `field` no tipo `color`, ou `None` se o
/// campo não existir (o chamador emite o erro "does not contain field").
pub fn color_type_field(field: &str) -> Option<Value> {
    // 38 entradas: 8 constructors + 12 operadores + 18 cores predefinidas.
}
```

Registado em `eval/mod.rs` como `scope.define("color", Value::Type(Type::Color))`.
O braço `(Type::Color, _)` de `eval_field_access` (`eval/bindings.rs`) delega
em `color_type_field` e emite "type color does not contain field `<f>`"
(mensagem verbatim do vanilla) para campo inexistente.

## Funções nativas

### `color.lighten(col, amount)` → Color

- `col`: `Value::Color` — cor base.
- `amount`: `Value::Float` | `Value::Relative` (percentagem, rel-only) |
  `Value::Ratio` (P842, #32 — literal percentual) — [0.0, 1.0].
- Delega para `Color::lighten(amount)` (P476 L1 método).
- Erros: argumento count ≠ 2, tipo errado, named inesperado.

### `color.darken(col, amount)` → Color

- Análogo a `lighten`; delega para `Color::darken(amount)`.

### `color.mix(..colors, space: auto)` → Color

- Aceita uma ou mais entradas, cada uma `Value::Color` de peso `1` ou array
  `(cor, peso)` com peso float/ratio; zero entradas falha pela soma não positiva.
- Normaliza todos os pesos pela soma e reduz no espaço alvo; `space:auto` usa
  Oklab para process colors.
- A extensão compatível `weight:` continua aceita somente com exatamente duas
  cores nuas e equivale aos pesos `1-weight, weight`.
- Espaços com hue rejeitam mais de duas cores; detalhes e mensagens estão na
  decisão P1286 abaixo.

### `color.negate(col, space: auto)` → Color

- `col`: `Value::Color`.
- `space`: `Value::Func` cujo nome é um dos 8 constructors de cor
  (`rgb`, `linear-rgb`, `luma`, `cmyk`, `hsl`, `hsv`, `oklab`, `oklch`);
  default `auto` = Oklab. Qualquer outro valor produz erro de tipo.
- Delega para `Color::negate(Some(space))` ou `Color::negate(None)`.

### `color.rotate(col, angle, space: auto)` → Color

- `col`: `Value::Color`.
- `angle`: `Value::Angle`.
- `space`: constructor de cor; default `auto` = Oklch. Só espaços com hue
  (Oklch, Hsl, Hsv) são válidos — os restantes produzem "this color space
  does not support hue rotation".
- Delega para `Color::rotate(angle, Some(space))`.

### Extensões `space:`/`weight:` de `color.mix`

- `space`: constructor de cor; default `auto` = Oklab para process colors. O
  contrato vale para todas as entradas variádicas P1286.
- `weight:` é extensão cristalina de compatibilidade, somente para exatamente
  duas cores nuas, e equivale aos pesos relativos `1-weight, weight`.
- A redução N-ária usa helper privado; a API pública binária `Color::mix`
  permanece como caminho compatível para exatamente duas cores.

### `color.to-hex(col)` → Str

- `col`: `Value::Color`.
- Delega para `Color::to_hex()`.

### `color.transparentize(col, factor)` → Color

- `col`: `Value::Color`.
- `factor`: percentagem [0.0, 1.0] (mesmo tipo que `amount`).
- Delega para `Color::transparentize(factor)`.

### `color.opacify(col, factor)` → Color

- Análogo a `transparentize`; delega para `Color::opacify(factor)`.

## Extração de ratio

```rust
fn extract_ratio_arg(val: &Value, fn_name: &str, arg_name: &str) -> SourceResult<f32> {
    Value::Float(f)   => f as f32,
    Value::Int(i)     => i as f32 / 100.0,
    Value::Relative(r) if r.abs.is_zero() => r.rel as f32,
    Value::Ratio(r)   => r.get() as f32,   // P842 (#32) — literal percentual
    _ => Err(...)
}
```

## Critérios de verificação (P476)

- `color.lighten(Color::srgb_f32(1,0,0,1), 0.2)` → `Ok(Value::Color(_))`.
- `color.darken(Color::srgb_f32(0,0,1,1), 0.2)` → `Ok(Value::Color(_))`.
- `color.negate(Color::srgb_f32(1,0,0,1))` → ciano `(0.0, 1.0, 1.0, 1.0)`.
- `color.mix(red, blue)` sem `weight:` → Ok (default 0.5).
- `color.mix(red, blue, weight: 0.25)` → Ok.
- `color.mix(red, green, blue)` → Ok pela redução variádica P1286.
- `color.mix((red, 1), (green, 2), (blue, 3))` → Ok, pesos relativos.
- ~~`make_color_module()` retorna `Value::Dict` com 4 entradas.~~
  **P736** — substituído por: `color_type_field` devolve `Some(Value::Func(_))`
  para cada um dos 14 fields e `None` para campo inexistente; o scope global
  define `color` como `Value::Type(Type::Color)`.

## P477 — `saturate` e `desaturate`

Adicionados ao módulo e a `make_color_module()` (6 entradas total) —
**P736**: passaram a fields do tipo via `color_type_field`:

```rust
dict.insert("saturate",   Value::Func(Func::native("color.saturate",   native_color_saturate)));
dict.insert("desaturate", Value::Func(Func::native("color.desaturate", native_color_desaturate)));
```

`color.saturate(col, amount)` — aumenta chroma Oklch; clamp mínimo 0.0.
`color.desaturate(col, amount)` — diminui chroma Oklch; clamp mínimo 0.0.

ADR-0083 §"Operadores cor": **TOTALMENTE FECHADO** (6/6) pós-P477.

## Cores predefinidas (P492/P497/P687)

`color.rs` exporta `predefined_color_bindings()` — vector de pares `(EcoString, Value)`
para injeção no scope global de eval (`eval/mod.rs`, `eval/modules.rs`).

**P1143 — paridade vanilla ratificado `a51e02804`.** A fonte ratificada e os
dois binários de referência confirmam exatamente o conjunto de 18 abaixo,
disponível tanto globalmente quanto como `color.<nome>`. Para cada nome,
`<nome> == color.<nome>` é `true` e `type(color.<nome>)` é `color`.
`lib.rs::prelude` registra os globals e `visualize/color.rs` declara as mesmas
constantes no scope do tipo. `cyan`, `magenta`, `color.none`, `color.pink` e
`color.ostrich` não pertencem ao namespace ratificado.

| Nome | sRGB | `Color::rgb` |
|------|------|--------------|
| `black` | `#000000` | `Color::rgb(0x00, 0x00, 0x00)` |
| `gray` | `#AAAAAA` | `Color::rgb(0xAA, 0xAA, 0xAA)` |
| `silver` | `#DDDDDD` | `Color::rgb(0xDD, 0xDD, 0xDD)` |
| `white` | `#FFFFFF` | `Color::rgb(0xFF, 0xFF, 0xFF)` |
| `navy` | `#001F3F` | `Color::rgb(0x00, 0x1F, 0x3F)` |
| `blue` | `#0074D9` | `Color::rgb(0x00, 0x74, 0xD9)` |
| `aqua` | `#7FDBFF` | `Color::rgb(0x7F, 0xDB, 0xFF)` |
| `teal` | `#39CCCC` | `Color::rgb(0x39, 0xCC, 0xCC)` |
| `eastern` | `#239DAD` | `Color::rgb(0x23, 0x9D, 0xAD)` |
| `purple` | `#B10DC9` | `Color::rgb(0xB1, 0x0D, 0xC9)` |
| `fuchsia` | `#F012BE` | `Color::rgb(0xF0, 0x12, 0xBE)` |
| `maroon` | `#85144B` | `Color::rgb(0x85, 0x14, 0x4B)` |
| `red` | `#FF4136` | `Color::rgb(0xFF, 0x41, 0x36)` |
| `orange` | `#FF851B` | `Color::rgb(0xFF, 0x85, 0x1B)` |
| `yellow` | `#FFDC00` | `Color::rgb(0xFF, 0xDC, 0x00)` |
| `olive` | `#3D9970` | `Color::rgb(0x3D, 0x99, 0x70)` |
| `green` | `#2ECC40` | `Color::rgb(0x2E, 0xCC, 0x40)` |
| `lime` | `#01FF70` | `Color::rgb(0x01, 0xFF, 0x70)` |

**Medição morfológica P1143 (ADR-0107/0108):** `black`, `gray`, `silver` e
`white` são cores `luma`; `space()`, `components()` e `repr` expõem esse espaço
na língua (`black` → `luma`, `(0%, 100%)`, `luma(0%)`). As outras 14 são `rgb`.
Logo, representar as quatro primeiras como sRGB equivalente não é mecânica:
é divergência semântica observável e deve ser corrigida na tabela. A estrutura
Rust e o algoritmo de lookup permanecem mecânica livre.

**Extras não-vanilla (compatibilidade global, não namespace):** `cyan`
(`rgb(0x00,0xB3,0xB3)`), `magenta` (`rgb(0xE5,0x00,0xE5)`) e `none`
(`Value::None`) já existiam antes de P687 e são mantidos como bindings globais
para não regredir documentos existentes. Eles não entram em `color.*`; esta
separação é explícita e tabelada. Removê-los do global exige passo próprio de
compatibilidade.
O parser de cores por *string* (`fill: "gray"` → `rgb(128,128,128)`, em `shapes.rs`) é
uma via **separada** (nomes CSS) e **não** é alterado por este passo.

A função `text(...)` é registada separadamente no scope global (P492) para permitir
`#show regex("\\d+"): it => text(red, it)`.

### Decisão P1143 produzida pela medição

O owner mantém duas projeções de uma fonte única:

1. tabela canônica das 18 cores ratificadas, com quatro valores `Color::luma`
   e 14 valores sRGB;
2. tabela separada dos três extras cristalinos, consumida somente pelos
   bindings globais.

`predefined_color_bindings()` concatena as duas projeções globais;
`color_type_field()` consulta apenas a tabela ratificada depois de testar
constructors/operadores. Não duplicar nomes ou canais em outro `match`.

Esta é adição/correção de entradas em tabela para paridade, usando tipos e
assinaturas existentes; não altera entidade pública, trait, default ou fase.
Pelo ADR-0127 segue em fluxo contínuo: L0 primeiro, RED→GREEN e revalidação.

**P1253 — precisão das constantes nomeadas.** As cores nomeadas preservam os
literais `f32` ratificados pelo vanilla, inclusive `red =
(1.0, 0.254902, 0.211765)` e `blue = (0.0, 0.454902, 0.85098)`. Reconstruí-las
por divisão exata dos bytes hex produz a mesma morfologia `#RRGGBB`, mas não a
mesma semântica numérica em conversões e sampling Oklab, portanto é proibido.

## P742 — métodos de instância + fields `rotate`/`components`/`space`

**Medição ADR-0108 (sonda contra vanilla 0.15.0 969087ec):** os métodos de
instância `red.lighten(20%)`, `red.darken(20%)`, `red.negate()`,
`red.rotate(90deg)`, `red.mix(blue)`, `red.components()`, `red.space()`,
`red.saturate(20%)`, `red.desaturate(20%)` existem no vanilla e estavam
**ausentes** no cristalino (pré-existente desde P476 — só havia estáticas).
A sonda mediu também que a semântica dos operadores P476/P477 divergia do
vanilla (lighten/darken/saturate via Oklch, negate em sRGB) — **corrigida no
domínio** (`entities/color.md` §"Operadores de cor (P476/P477, semântica
corrigida em P742)"); a correcção beneficia estáticas e instâncias.

Descoberta-chave da sonda: o `red` vanilla é `rgb(1.0, 0.254902, 0.211765)`
(`#ff4136`, `visualize/color.rs:311`), não `#ff0000`; as fórmulas do
`palette 0.7.6` (crate usada pelo vanilla) reproduzem byte a byte os valores
medidos (verificado em scratch dedicado).

### Despacho de métodos de instância (padrão P506)

Braço `Value::Color` no bloco P506 de `eval_func_call` (`closures.rs`),
interceptando **apenas** os 12 métodos conhecidos — método desconhecido cai
no caminho genérico (erro de field, comportamento pré-P742 preservado):

```rust
Value::Color(ref color) => {
    if crate::compiler::stdlib::color::is_color_instance_method(method) {
        return super::bindings::eval_color_method(
            color, method, call.args(), scopes, ctx, engine,
        );
    }
}
```

`eval_color_method` (`eval/bindings.rs`) faz `eval_args` uma vez e despacha:

- `lighten` / `darken` / `saturate` / `desaturate` / `negate` / `mix` /
  `rotate` / `to-hex` / `transparentize` / `opacify` — sintetiza `Args` com
  a cor como primeiro posicional e **delega nas nativas estáticas**
  (validação e mensagens idênticas aos dois caminhos).
- `components` — named `alpha: bool = true`; delega em
  `native_color_components`.
- `space` — sem argumentos; devolve `Func::native(<nome do constructor>, <nativa>)`
  fresco (nomes vanilla medidos: `rgb`, `luma`, `linear-rgb`, `oklab`,
  `oklch`, `cmyk`, `hsl`, `hsv`). `red.space() == rgb` é `true` via a
  igualdade por nome de nativas introduzida em `entities/func.md` (P742 —
  medido no vanilla; `Func::eq` era só identidade de Arc).

### Seis fields novos do tipo (20 total)

`color_type_field` passa a 20 entradas: as 14 anteriores + `rotate`,
`components`, `space` (P742) + `to-hex`, `transparentize`, `opacify`
(P744).

- `color.rotate(col, angle, space: auto)` → `Color` — default espaço Oklch.
- `color.components(col, alpha: true)` → `Array` de `Ratio`/`Float`/`Angle`.
- `color.space(col)` → `Func` do constructor do espaço.
- `color.to-hex(col)` → `Str` — hex sRGB com alpha quando aplicável.
- `color.transparentize(col, factor)` → `Color` — reduz opacidade.
- `color.opacify(col, factor)` → `Color` — aumenta opacidade.
- **Nomes plain nas funcs do tipo** (medido P742): `repr(color.rgb)` →
  `rgb`, `repr(color.lighten)` → `lighten` no vanilla (não `color.rgb`) —
  as entradas de `color_type_field` passam a registar os constructors e
  operadores com o nome simples do vanilla.
- **Repr de Func sem `#`** (medido P742): `repr(rgb)` → `rgb`,
  `#red.space()` em markup → `rgb`. `repr.rs` passa a formatar Func
  nomeada como `name` (era `#name`); o ramo sem nome mantém-se.

### Scope-outs P742/P744 (medidos, com erro explícito)

- Repr de closure anónima (`#function(...)`) — o vanilla imprime `(..) => ..`
  (P744); fora do caminho da sonda → achado adiado.

## Scope-out

- `color.saturate/desaturate` com `space:` arg — scope-out (não suportado no vanilla).
- ~~`color.rotate`, `color.components`, `color.space`~~ — **P742: implementados**
  (estáticas + instância).
- ~~Named `space:` em `negate`/`rotate`/`mix`~~ — **P744: implementados**.
- ~~`to-hex`/`transparentize`/`opacify`~~ — **P744: implementados**
  (estáticas + instância).
- ~~Métodos de instância de cor (`red.lighten(20%)`, …)~~ — **P742/P744: implementados**
  (12 métodos; despacho P506).

---

## P1284 — `color.map` exato e gate spot

### Medição anterior à decisão

No vanilla ratificado `a51e02804`, arquivo
`lab/typst-original/crates/typst-library/src/visualize/color.rs` SHA-256
`80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713`,
`color.map` é um `module` com 15 arrays, na ordem abaixo. O digest de cada
sequência é SHA-256 dos valores RGBA canonizados como tokens lowercase de oito
dígitos (`0xrrggbbaa`), com left-padding de zero quando o literal Rust omite o
nibble inicial, unidos por vírgula sem whitespace
(ex.: `0x440154ff,...,0xfee825ff`). Assim, contagem, ordem, canais e alpha
ficam integralmente pinados sem copiar milhares de literais.

| Nome | N | Primeiro | Último | SHA-256 da sequência canônica |
|---|---:|---|---|---|
| `turbo` | 256 | `0x23171bff` | `0x900c00ff` | `7909421397bc2bef00faf5a1064d7356ad4cab112ec1fd5e290b124a13529a3e` |
| `cividis` | 256 | `0x002051ff` | `0xfdea45ff` | `dece34103d5266311558bb44f96f02df7090edf8977744fc9a1213b2213b2f79` |
| `rainbow` | 256 | `0x7c4bbbff` | `0x7c4bbbff` | `74fe385a692d3da43ff8afc8f61896f76bc87a1a766f05f4897f5e8164db4ed7` |
| `spectral` | 11 | `0x9e0142ff` | `0x5e4fa2ff` | `1c62ea2e765ddf6d6b1c5189e772605e0203d6116bf384597df0cfa4cdab5f17` |
| `viridis` | 9 | `0x440154ff` | `0xfee825ff` | `3b9d02b0685ec2ae62958a287c4cdd68fdb08f642339aabaa8d96e60aeaf999b` |
| `inferno` | 11 | `0x000004ff` | `0xfcffa4ff` | `20de1b5440e58c3727d645e6f8444d9e6d1c73fb09f9e905a49c314dc58496da` |
| `magma` | 11 | `0x000004ff` | `0xfcfdbfff` | `c634397325d83383609535f10e2d964efd899be34a2cef337379a5368b09fa79` |
| `plasma` | 11 | `0x0d0887ff` | `0xf0f921ff` | `f0ddd8a6d9e12c3c3b5d705dfa2e7c5f7929ce242c8d94ab5e4a0f6b2e0d9f2d` |
| `rocket` | 256 | `0x03051aff` | `0xfaebddff` | `b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b` |
| `mako` | 256 | `0x0b0405ff` | `0xdef5e5ff` | `f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77` |
| `coolwarm` | 256 | `0x3b4cc0ff` | `0xb40426ff` | `13161dddf6ad860268eece02ce446b1025d3ab95871cc1a484a93aa05da4aed2` |
| `vlag` | 256 | `0x2369bdff` | `0xa9373bff` | `d015c8a4cb87aaebc49d0416edf430e5c76392e772602ca1234ce6fe748db81d` |
| `icefire` | 256 | `0xbde7dbff` | `0xffd4acff` | `0de0d1c2a9dd6a8a03cd3155b872ea4a2eeaa74a5454df8f511632aa5d91e55f` |
| `flare` | 256 | `0xedb081ff` | `0x4b2362ff` | `03f70bedc78f4fb3e22b3f7ed06a680d56d721321eca69218ab746d4c803f595` |
| `crest` | 256 | `0xa5cd90ff` | `0x2c3172ff` | `660b6381c0f53156c07b4f0f86988a280c06578f46a44b734c1f388cadeaeb98` |

### Refutação P1284-v4 — contagem 246/252

O oráculo independente `A-P1284-v2` executou 52/52 sondas no vanilla e pinou
`lab/surface-inventory/p1284-probes.json` por
`06e205483a9bc905f778a72fcbb74ddb229b73141ffe6bf27ec95d0a40e145c3` e
seu recibo por
`50b6e34b826041edb61f15828a14c2cfd234bc7bee71215c378e89e68079ba79`.
Ele refutou as contagens v3 diretamente pela superfície de linguagem.

A auditoria reproduziu a causa: o extrator v3 usou o padrão
`0x[0-9a-fA-F]{8}`. `rocket` possui 256 literais, mas 10 são escritos com sete
dígitos (`0x3051aff` … `0xe0b22ff`) e 246 com oito; `mako` possui 4 com sete
(`0xb0405ff` … `0xf0609ff`) e 252 com oito. O regex descartou exatamente
esses 10/4 prefixos. Portanto a fonte não estava truncada: a **extração era
truncante por largura textual**. `Color::from_u32` interpreta os literais como
`0x03051aff`/`0x0b0405ff`, e `to-hex` confirma o left-padding na linguagem.

Uma extração reproduzível deve aceitar de um a oito dígitos, converter o
inteiro e formatar `0x%08x` antes de contar/digerir. Esse procedimento produz
exatamente os N/extremos/digests corrigidos acima. A hipótese v3 seria
refutada — e foi — por qualquer valor válido escrito com menos de oito
dígitos ou por sonda de cardinalidade no valor público.

### Decisão

`color_type_field("map")` devolve `Value::Module("map", scope)`; cada filho
é `Value::Array` de `Value::Color` sRGB na ordem literal, com alpha `ff`.
Nenhum `Value`, `Type`, `Color` ou `Content` novo é necessário. O catálogo do
tipo passa de 38 para 39 fields; desconhecidos continuam erro. Não interpolar,
subamostrar, arredondar, reordenar nem converter espaços. O gate exige os 15
digests acima, além de kind e inventário fechado.

### Gate não autorizado

`color.spot` e `color.spot.tint` não pertencem ao lote contínuo. O vanilla
exige `SpotColorant`/cor spot e a entidade cristalina não possui variante
correspondente. Estado: `BLOCKED_ADR0127_PUBLIC_CONTRACT`. Não usar dict,
module, string, sRGB aproximado ou `none` como substituto; este L0 não autoriza
nova variante/tipo público.

## P1286 — `color.mix` variádico e pesos relativos

### Medição anterior à decisão

O baseline `visualize/color.rs:1058-1090,1136-1204,2422-2459` recebe N
argumentos, cada um cor nua (peso `1`) ou array `(cor,peso)` com peso float ou
ratio; normaliza pela soma. O receipt P1286 mediu 1 e 3 cores, pesos positivos,
zero e negativos com soma positiva, rejeição de soma `<=0`, e proibição de
mais de duas cores em HSL/HSV/Oklch.

### Decisão

`color.mix(..colors, space:auto)` aceita uma ou mais entradas no domínio
medido. Zero entradas e qualquer soma `<= 0` produzem
`sum of weights must be positive`. Array fora de aridade dois produz
`expected a color or color-weight pair`; peso fora de float/ratio preserva o
erro de cast medido. Em espaço com hue, N>2 produz
`cannot mix more than two colors in a hue-based space`.

O método de instância insere o receiver como primeira cor de peso `1` e aceita
as demais formas sem glue paralelo. A extensão cristalina `weight:` permanece
somente para exatamente duas cores nuas, interpretada como pesos
`1-weight, weight`, para não quebrar documentos existentes; não pertence à
alegação de paridade vanilla. `space:auto` continua Oklab para process colors;
spot permanece fora do fragmento e `Unknown`/bloqueado pelo gate P1284.

Nenhuma API Rust pública, default ou fase muda; fluxo contínuo ADR-0127.
