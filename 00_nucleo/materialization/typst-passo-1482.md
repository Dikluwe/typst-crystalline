# Passo 1482 — `color.spot.tint`: valor de cor e semântica da linguagem

**Estado:** contrato L0 confirmado pelo dono em 2026-09-23; implementação
P1482 materializada e validada. Fatia incompleta deliberada: P1483 fecha
export PDF `/Separation` e render dos valores aqui introduzidos. Ambos os L0s
registram a divisão.

**Baseline:** 2026-09-22, HEAD limpo
`90aad760ba472cffb70063baa684c461a240e447`, cristalino release SHA-256
`b70537dbcfa4184d900892623c440c01b92ae5bfee32b752ea9f7921eaf64fba`,
vanilla pinado `586e1bd43` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## 1. Medição anterior à decisão

Sonda bilateral `color.spot("Brand", red).tint(50%)`: vanilla devolve um
valor do tipo `color` cuja `repr` é
`color.spot("Brand", rgb("#ff4136")).tint(50%)`; cristalino erra
`type spot colorant has no method tint`. Inventário dos quatro perfis
registra **um membro ausente**, `color.spot.tint`; não são quatro defeitos.

`lab/typst-original/crates/typst-library/src/visualize/color.rs:2254-2388`
mede `SpotColor{colorant,tint}`, operações e validação do intervalo 0–100%.
`00_nucleo/prompts/compiler/stdlib/color.md` §P1473,
`entities/color.md` §P1473 e `entities/value.md` §P1473 explicitamente
deixaram `.tint()` fora: hoje `SpotColorant` é irmão de `Color`, sem
`Color::Spot`. `01_core/src/entities/color.rs:1025-1047` confirma o corte.

Classificação: tipo, chamada, diagnóstico, `repr` e operações da cor são
superfície da linguagem; layout interno/enum Rust não é critério de paridade.
Inferência: ampliar `Color` é o caminho mais compatível com os consumers
atuais e com a fonte; refutação: uma representação alternativa que preserve
todos os observáveis com menor mudança de contrato, medida antes do L0.

## 2. Auditoria obrigatória antes de escrever L0

Medir bilateralmente `type`, `repr`, igualdade de linguagem, `0%`, `100%`,
fora do intervalo, tipo de argumento errado, `.lighten`, `.darken`,
`.negate`, `.fallback`, cast para campos de fill/stroke e serialização
pública. Localizar **todos** os matches exaustivos de `Color` e `Value`
(`rg` por enum/variant e testes); decidir por consumer o significado de
spot versus fallback de processo. Não tratar o fallback como identidade da
cor nem reduzir spot a `rgb` no valor da linguagem.

## 3. Contrato da fatia

Após medição, L0 deve escolher e especificar o carrier de spot+tint, o
método `.tint(ratio)`, a morfologia pública, as operações comprovadas e os
casts de cor. Preservar nome/fallback do colorant e percentual original;
`type(resultado) == color`. Fallback de processo serve aos backends que
não dispõem de tinta spot, mas não apaga o colorant do valor.

Fica **fora** deste passo a conformidade de separação PDF e qualquer
afirmação de fechamento completo de `color.spot.tint`: P1483 a completa.
O L0 inicial deve nomear explicitamente P1483 como responsável por PDF e
render, conforme a regra de constructor dividido.

## 4. Gate e aceitação

Owners candidatos: `entities/color.md` ↔ `entities/color.rs`,
`entities/value.md` ↔ `entities/value.rs`, `compiler/stdlib/color.md` ↔
`compiler/stdlib/color.rs`, mais L0s de dispatch/repr/consumers alterados.
Não adicionar código antes de atualizar L0s e resolver V15/V26. Como a
introdução de variante pública modifica o contrato de entidade, **parar
após a redação/resselo documental e obter confirmação humana** antes de L1.

RED por sonda de linguagem e controles de erro; GREEN quando as chamadas
medidas coincidem bilateralmente, os consumers de cor continuam totais,
testes de regressão de cores de processo passam, `cargo build` e
`crystalline-lint .` ficam verdes. Registar no resultado que PDF spot segue
RED até P1483; não retirar `color.spot.tint` do inventário por mera
existência do nome se o uso real ainda falha.
## 5. Auditoria L0 e gate aberto (2026-09-23)

Medição por `/usr/local/bin/typst eval` e
`target/release/typst eval` em `2026-09-23T15:47:54-03:00`,
HEAD `3d2aea6618f81bcde66aaf8a4dce8aea1bc0f3be`,
working tree não commitada. SHA-256 dos binários, respectivamente:
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
e `ba164438e499d0ae1980993bdfc751539f43bbedf029875a102495a508ef3442`.
Antes do L0, `git diff HEAD --stat` era vazio; durante a medição final
há somente dez prompts editados e os dez headers Rust correspondentes
(lista exata: `git diff HEAD --stat`), além deste passo não rastreado.
Os rascunhos P1483–P1488 continuam não rastreados e intocados.

Vanilla: `s = color.spot("Brand", red)` tem tipo `spot`;
`s.tint(50%)` tem tipo `color`, repr
`color.spot("Brand", rgb("#ff4136")).tint(50%)`, identidade
distinta de `rgb(s.tint(50%))`, e funciona também como
`color.spot.tint(s, 50%)`. `s.tint` isolado falha. `0%` e
`100%` são válidos; `-1%`, `101%` e float são rejeitados.
Lighten/darken/negate ajustam o tint; `space()` devolve o
colorant; `components()` devolve apenas o percentual. Mistura e
gradiente conservam spot somente com o mesmo colorant completo.
Fill/stroke aceitam a cor. O cristalino ainda erra
`type spot colorant has no method tint`.

Fonte: vanilla `visualize/color.rs:280-286,889-1020,1147-1245,
1260-1415,2254-2397,2471-2549`; cristalino
`entities/color.rs:31-61,356-367,616-680`,
`compiler/stdlib/color.rs:118-138,743-808`,
`entities/gradient.rs:204-273,369-385`. Tipo, repr, igualdade
de linguagem e diagnósticos são paridade; enum e bytes PDF são
mecânica (ADRs 0107/0108). A hipótese de carrier alternativo
sem `Color::Spot` é refutável por proposta que preserve todos
os casts com menor quebra pública.

L0 propostos: `Color::Spot(SpotColor)`, tint `Ratio`,
`ColorSpace::Spot(SpotColorant)`, `Color`/`ColorSpace`/
`GradientStop` sem `Copy`, `to_space` checked e `to_process`
explícito; dispatch estático/ligado, repr e gradientes. P1483
completa PDF `/Separation` e render; PDF spot continua RED.
`crystalline-lint --fix-hashes .` terminou sem drift;
V5/V15/V26 e `git diff --check` passaram. Em Rust, só
`@prompt-hash` mudou. Nenhum teste RED nem código funcional
P1482 foi escrito. **Aguardar confirmação do dono antes de L1.**

SHA-256 finais dos L0 propostos (caminhos relativos a
`00_nucleo/prompts/`):
```text
entities/color.md                      57bdca5f0246e0c04c2d9dd789df8d4c42fa2aba6e99e8e8738f0f9d784d26af
entities/value.md                      62d8f24db8ffdf4bfc4760aecb174d64dfe6cd041d562ee5c0ffabdb83f11623
entities/gradient.md                   100830f6641ac02cdd3365d2fe45a4ada1231aa3e5bba6448ed6d55a18280fad
compiler/stdlib/color.md               043951e2e993568332ad3bc8a493ffe5a8adb5cf6f3db5982753390286a4679c
compiler/stdlib/gradients.md           7ed3fea58c87a77efd65fdee53028e73fde25d456aec5477493ff281836caa2b
compiler/stdlib/foundations/color.md   7c636de48be81669ad7bb82a8b5c28aff8ff96c2a8e9ef60291373bb0034bdcb
compiler/eval/repr.md                  41f999ad06d86afba8b31c3eded895b99cf9880f8facf87af39b3565d85679c3
compiler/eval/call_dispatch.md         33fc72da448372437233551f7f123e22af71890797c3b2dc6541a7629c0583f1
compiler/eval/bindings/field_access.md b749b08b4a0f362f18583b8f0b2fd9525fad5987f63f70173172a22f160c18db
infra/export/stream.md                 b8cd0aa54bb26c93e63289abb30ba6070f3b470b34b032dc28088741fddb59c1
```
Arquivos do `git diff HEAD --name-only` na medição final (20; o passo
P1482 era adicionalmente não rastreado):
```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/eval/repr.md
00_nucleo/prompts/compiler/stdlib/color.md
00_nucleo/prompts/compiler/stdlib/foundations/color.md
00_nucleo/prompts/compiler/stdlib/gradients.md
00_nucleo/prompts/entities/color.md
00_nucleo/prompts/entities/gradient.md
00_nucleo/prompts/entities/value.md
00_nucleo/prompts/infra/export/stream.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/stdlib/color.rs
01_core/src/compiler/stdlib/foundations/color.rs
01_core/src/compiler/stdlib/gradients.rs
01_core/src/entities/color.rs
01_core/src/entities/gradient.rs
01_core/src/entities/value.rs
03_infra/src/export/stream.rs
```

## 6. Materialização após confirmação (2026-09-23T18:01:47-03:00)

O dono confirmou explicitamente o L0 público (ADR-0127). Os dez prompts
proprietários foram marcados como confirmados e ressellados; `--fix-hashes`
terminou com zero drift. O novo valor é `Color::Spot(SpotColor)`, com
colorant preservado e tint `Ratio`; `color.spot.tint`, dispatch ligado e
estático, repr, igualdade, casts, operações de cor e gradientes passaram
aos testes de linguagem. Consumidores L1–L4 foram adaptados à perda de
`Copy` sem converter o valor spot prematuramente. PDF e backends de
processo usam projeção explícita do fallback tingido; `/Separation` e
render spot nativos continuam RED e pertencem a P1483.

RED antes do código: três testes P1482 falharam por ausência de `.tint()`;
um teste adicional de precisão falhou porque `.components()` truncava
`Ratio` para `f32`. O vanilla pinado devolveu `true` para
`color.spot("Brand", red).tint(12.3456789%).components().at(0) == 12.3456789%`;
o mapeamento da linguagem agora preserva o `Ratio` original.

Verificação na working tree não commitada derivada de
`3d2aea6618f81bcde66aaf8a4dce8aea1bc0f3be`, em `2026-09-23T18:04:18-03:00`:
`cargo test -p typst-core --lib` — 6135/6135;
`cargo test --workspace --no-run` — compila;
`cargo build` — passa;
`crystalline-lint .` — exit 0, sem violações bloqueantes;
`git diff --check` — passa. `cargo fmt --all -- --check` assinala
formatação em arquivos alterados e também em arquivos preexistentes;
não se aplicou formatação global que expandiria o diff para fora da
fatia. O diff rastreado tinha 64 arquivos, 1052 inserções e 401
remoções; SHA-256 de `git diff HEAD`:
`34f47966acfee4f30c9d4905f6617323448bdf647b12e0e516f511a337ef34a0`.
Separadamente, `cargo test -p typst-infra --lib` passou 1045/1045 no
estado imediatamente anterior, diff SHA-256
`1add7135e640a7247d7ee8c5ecba81ed8ad6dbc54ae847e0f2a18c5fb0da9d8a`;
entre essa medição e o estado final apenas se removeu um import não usado.
O passo P1482 permanece não rastreado neste estado; P1483–P1488 já
estavam não rastreados e não foram alterados. Nenhum commit P1482 foi
criado.

Arquivos rastreados alterados na medição (`git diff HEAD --name-only`):

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/eval/repr.md
00_nucleo/prompts/compiler/stdlib/color.md
00_nucleo/prompts/compiler/stdlib/foundations/color.md
00_nucleo/prompts/compiler/stdlib/gradients.md
00_nucleo/prompts/entities/color.md
00_nucleo/prompts/entities/gradient.md
00_nucleo/prompts/entities/value.md
00_nucleo/prompts/infra/export/stream.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/introspect.rs
01_core/src/compiler/layout/block.rs
01_core/src/compiler/layout/boxed.rs
01_core/src/compiler/layout/cursor.rs
01_core/src/compiler/layout/decorations.rs
01_core/src/compiler/layout/grid.rs
01_core/src/compiler/layout/tests.rs
01_core/src/compiler/layout/text.rs
01_core/src/compiler/math/layout/callbacks.rs
01_core/src/compiler/math/layout/cancel.rs
01_core/src/compiler/math/layout/matrix.rs
01_core/src/compiler/math/layout/mod.rs
01_core/src/compiler/math/layout/p1431_tests.rs
01_core/src/compiler/math/layout/underline.rs
01_core/src/compiler/stdlib/color.rs
01_core/src/compiler/stdlib/foundations/color.rs
01_core/src/compiler/stdlib/gradients.rs
01_core/src/compiler/stdlib/html.rs
01_core/src/compiler/stdlib/layout.rs
01_core/src/compiler/stdlib/mod.rs
01_core/src/compiler/stdlib/shapes.rs
01_core/src/compiler/stdlib/structural/table_grid.rs
01_core/src/compiler/stdlib/text/constructor.rs
01_core/src/compiler/stdlib/visualize.rs
01_core/src/entities/color.rs
01_core/src/entities/content.rs
01_core/src/entities/elements/boxed.rs
01_core/src/entities/elements/grid.rs
01_core/src/entities/elements/grid_cell.rs
01_core/src/entities/elements/overline.rs
01_core/src/entities/elements/strike.rs
01_core/src/entities/elements/table.rs
01_core/src/entities/elements/table_cell.rs
01_core/src/entities/elements/underline.rs
01_core/src/entities/gradient.rs
01_core/src/entities/paint.rs
01_core/src/entities/style.rs
01_core/src/entities/style_chain.rs
01_core/src/entities/tiling.rs
01_core/src/entities/value.rs
02_shell/src/cli.rs
03_infra/src/export/builder.rs
03_infra/src/export/gradients/adaptive.rs
03_infra/src/export/gradients/conic.rs
03_infra/src/export/render.rs
03_infra/src/export/stream.rs
03_infra/src/export/svg.rs
03_infra/src/export/tests/mod.rs
03_infra/tests/p1250_svg_integrated.rs
```
