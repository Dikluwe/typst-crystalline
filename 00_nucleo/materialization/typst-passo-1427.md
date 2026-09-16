# Passo 1427 — convergência vertical completa de `math.frac`

## Estado medido antes da decisão

P1426 fechou `math.root` e deixou 20 rotas `math.*` residuais da coorte
P1422. A leitura comparativa rejeitou dois atalhos: expor apenas a fração
vertical default chamaria um fragmento de assinatura de convergência, e
reutilizar o `overline` textual para `math.overline` colapsaria identidades
que o vanilla mantém distintas. `math.frac` é a próxima fatia que já possui
carrier matemático próprio e pode ser fechada sem essa perda semântica.

Medição executada em 2026-09-16T17:31:22-03:00 no HEAD limpo
`451f3ca3d208c997a41d93ccc597a9c117fe1de9`, tree
`fc442ca9056397eb311dcd4f759ec6b6478f71ea`. Os doze pares L0/consumer
congelados são:

| L0 | SHA-256 L0 | consumer | SHA-256 consumer |
|---|---|---|---|
| `compiler/stdlib/structural/math.md` | `7a4493eee032effa591c95d72682a0ed25a69785b5c62bc88c27f303f3dfda16` | `compiler/stdlib/structural/math.rs` | `26ecf0e75c588a6b73918c75eb11329ad4bf164337d39aa0138e07edc302e9be` |
| `compiler/eval/math.md` | `ee4adc68250498f7c1e6db43203bb405fc15e4f0620baee96d7d7bdfe10d0dcf` | `compiler/eval/math.rs` | `aa13b1e69a3719de416383385100511b2e9429ba767c45999038fefde8bdfbb9` |
| `compiler/eval/repr.md` | `978a7f4807858bce5e80438be090c8654ac5c8e2ac898d2c1305a703365262c4` | `compiler/eval/repr.rs` | `67c9ced123c4d1afd26dba72961fa05dcf03f08fe20242cafd8793df5f5f4574` |
| `entities/elements/math_frac.md` | `a9ea29fabacea327e895f78a3494307ddee58629cba25fec8e26be01699a20af` | `entities/elements/math_frac.rs` | `978c37cec5d20db44860a093faa8b701301e79e196151a419138f0882319cf74` |
| `entities/content.md` | `0bdb4e175c76a3152aca272a87a8aad265814f9800f200f66a758a6534cbc2fa` | `entities/content.rs` | `73ca290094d7e8813e140f406e9326a64533251371ec5ad77230a3cd741ab3c7` |
| `compiler/eval.md` | `b41d143f7e3fd6fe95ccc91854ce0971547be0218036e56fb8aafb861a536833` | `compiler/eval/mod.rs` | `6c77a5ccb43fad1a8d7a0912e0f65f5a8842354c6d57202e9cda714533f8b080` |
| `compiler/math/layout/_comum.md` | `7d412567f409d2e4b103013a620622722f36d35a857789a369fa94a4b2b6afef` | `compiler/math/layout/mod.rs` | `0976fc69c6019b633f563958ad512e78fa0516ab101ce2a7ca0590daec471e7a` |
| `compiler/math/layout/frac.md` | `9664f7d3320690a96129cd5ec39b405a0a4f499186559907fd27238b4d597003` | `compiler/math/layout/frac.rs` | `d6fdffb6a119aa8bb63b60f72de7156e3f16126dc460602fa269df0ef6f8e7d2` |
| `compiler/eval/rules.md` | `e532c1be0409dbbd9c32926f6c3b09220141feb7297be50c5b9e02987fd3be61` | `compiler/eval/rules.rs` | `63ee40a94d41603a6221b846dbc5393e82394279b81c189181a7cd10d5bbe227` |
| `compiler/math/layout/tests.md` | `00ddf49230e2c14ad29bf1f63ddc87ebf9b3accd5a1fa4863a328db4deb6fb9c` | `compiler/math/layout/tests.rs` | `259d8d86287ca8b55904a4f9ad73a3590c900b943ef141be71a453fa463bdc7b` |
| `entities/math_constants.md` | `745bd852e48f07a23aaa65ccc64dd4ef2fbc8e9af92f5187f52cedbc841bd292` | `entities/math_constants.rs` | `7954ed7045f293ead728c784fb669aa3077303576de35f91dde6e4ef9717f672` |
| `infra/font_metrics.md` | `cb6c3c9ba90bbcf0e60fcddf64065e324c567ef908eab6a6eaa02a930fa53692` | `infra/font_metrics.rs` | `5303946475331165548a48164e8c99b87c2051ec9d62c9f1b4f8332be5907d5e` |

No vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`math/frac.rs` (SHA-256
`b3d7d9e63b3ed7e7ade154ce03d510e232a806c34f32db7aaee3c444300b100e`)
declara `num`, `denom`, `style: vertical | skewed | horizontal` e dois bits
internos de remoção de parênteses. O avaliador de math (SHA-256
`4aad95925fbc3542fc5709ca4e053895e6aac4c6620195a305a07dab6a24bf7f`)
e o parser/AST (SHA-256 `91f4e947474c3fe1e46550cc4bc4f18145fe4952b8ecb2ec56e6b28371a66dde`
e `2f373bf3fe49ffb6eff3f870690622f50549edc5d4700d0719b46a8e65c4ea2a`)
provam que `/` é associativo à esquerda e transporta esses bits. O binário
ratificado tem SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
O layout vanilla que define a geometria dos três estilos,
`typst-layout/src/math/fraction.rs`, tem SHA-256
`6f1ccbf9553e6df2763ac6628a2d112a8e68cab5b231ec3d100df9b3c6e1e9da`.

Sondas públicas mediram:

| Expressão | Resultado público |
|---|---|
| `repr(math.frac())` | erro `missing argument: num` |
| `repr(math.frac([a]))` | erro `missing argument: denom` |
| `repr(math.frac([a], [b]))` | `frac(num: [a], denom: [b])` |
| `repr(math.frac(style: "skewed", [a], [b]))` | `frac(num: [a], denom: [b], style: "skewed")` |
| `repr(math.frac([a], [b], style: "vertical"))` | inclui `style: "vertical"` por ter sido explícito |
| `repr(math.frac(none, none))` | `frac(num: [], denom: [])` |
| terceiro posicional | erro `unexpected argument` |
| `num:` ou `denom:` named | erro ``the argument `<nome>` is positional`` |
| style inválido | erro com domínio `vertical`, `skewed`, `horizontal` |

`$frac(a,b)$` e `$a/b$` produzem a mesma fração vertical pública;
`$a/b/c$` associa à esquerda. Um par exterior é removido por `/`, mas um
segundo par permanece. Os bits são omitidos de `repr`, porém observáveis no
layout horizontal. Strings e `none` convertem para conteúdo; inteiros não.

## Decisão proposta

1. ampliar `MathFracElem` com estilo opcional explícito e os dois bits causais
   de deparenthesization, preservando o `line` interno usado exclusivamente
   pelo `binom` cristalino;
2. expor `MathFracStyle::{Vertical, Skewed, Horizontal}` e um constructor
   completo, mantendo wrappers legados estreitos para callsites verticais e
   para `binom` sem barra;
3. registrar `math.frac(num, denom, style:)` com aridade, casts, duplicados e
   diagnósticos medidos; `style` omitido permanece distinto de `vertical`
   explícito em `repr`;
4. fazer `frac(a,b)`, `a/b` e a função qualificada convergirem no mesmo
   carrier; somente `/` transporta os bits causais do AST;
5. materializar os três layouts: stack vertical, fração inclinada e fração
   horizontal, preservando a morfologia de parênteses no modo horizontal;
6. acrescentar e extrair as constantes OpenType MATH
   `SkewedFractionVerticalGap` e `SkewedFractionHorizontalGap`, usadas com
   `AxisHeight` na geometria inclinada; a fonte de referência
   `NewCMMath-Book.otf` tem SHA-256
   `60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`;
7. tornar `math.frac.style` settable: regras validam o domínio fechado e
   guardam a string canônica no `StyleChain`; o consumer converte-a de forma
   total para `MathFracStyle`. Estilo explícito vence o cascaded e ausência em
   ambos usa vertical;
8. preservar estilo, explicitude, bits e `line` em walkers, igualdade
   observacional e reconstruções; `binom` continua vertical, sem barra e não
   herda `math.frac.style`.

A aceitação é semântica, sintática e morfológica. Estrutura Rust, sequência de
operações, fragmentação de caixas e bytes de render são mecânica; mensagens de
erro e presença explícita de `style` em `repr` são observáveis.

## Limites explícitos

P1427 não altera `math.binom`, salvo preservar sua forma sem barra; não fecha
as outras 19 rotas residuais; não exige igualdade de estrutura Rust com o
vanilla; não aceita um binding vertical-only como convergência integral.
Casos sem identidade verificável, falha de build/execução, timeout, OOM, swap,
stream truncado ou ilegível são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo acrescenta enum/campos públicos, amplia constructor L1, cria binding
público, torna uma propriedade settable e acrescenta dois caminhos de layout.
Incide nos itens 1, 2 e 3 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os doze L0 proprietários congelados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, produzir contrato, oráculos, ataques e testes
   segregados, selar, obter RED e só então implementar.

## Regime posterior ao gate

Usar protocolo completo `tekt-materializacao-segregada`, declarando
“executado sem atestação de isolamento” no workspace partilhado. O contrato
deve cobrir binding, aridade, casts, named, style/default/explicitude, bits
causais de `/`, associatividade, set rule, walkers, repr, três layouts e
regressão de `binom`. Mutation score exigido: 1.0; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED da ausência do binding e das três variantes de estilo;
- RED de explicitude, casts, diagnósticos, associatividade e parênteses;
- RED do set rule e da precedência elemento explícito > cascaded > vertical;
- GREEN bilateral de chamada qualificada, `frac(a,b)` e `/`;
- controle `binom` sem barra e regressões verticais preservados;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**Confirmado explicitamente pelo dono em 2026-09-16.** A materialização fica
autorizada somente dentro do recorte completo de `math.frac` e dos doze pares
L0/consumer acima; as outras 19 rotas math permanecem fora de escopo.
