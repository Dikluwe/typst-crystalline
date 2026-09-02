# P1293 — recibo de medição causal do layout de `math.attach`

## Autoridade, regime e limites

- papel: `medidor_attach_p1293`, diagnóstico causal independente e read-only;
- regime: protocolo completo da skill `tekt-materializacao-segregada`,
  segregado por capacidades e artefatos, sem isolamento técnico de leitura do
  filesystem compartilhado;
- entradas autorizadas: L0s `math/layout/attach.md` e `_comum.md`, recibos
  públicos de medição vanilla e implementação B, fontes produtivas correntes,
  vanilla ratificado e probes próprios;
- escrita exercida: somente este recibo e temporários
  `/tmp/p1293-attach-causal-*`;
- não foram lidos nem executados oracle/contrato protegido, RED/discrimination
  privados, `/tmp/p1293-measure.py` ou fontes privadas;
- não foram editados produto, L0, manifesto, contrato, selo, testes ou
  veredito;
- política de `Unknown`: bloqueia. Não é convertido em sucesso.

Este recibo identifica o defeito que produz a largura vazia simétrica. Não
aprova o lote B nem propõe patch.

## Proveniência

Medição anterior à decisão:

| Entrada | Identidade |
|---|---|
| instante final da medição | `2026-09-01T19:50:26-03:00` |
| HEAD / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| árvore | compartilhada e não commitada; `36 files changed, 2671 insertions(+), 366 deletions(-)` antes deste recibo |
| SHA-256 `git status --short` | `41695492598973db685969ca8e3ee44b92ed47718b970cf40bfa54fcad913dee` |
| SHA-256 `git diff HEAD --stat` | `207429f7fe474cb7d337921065d9a5d56117856760efae23f04ed6fbedeaa74f` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| candidato `target/release/typst` | `ec08572f54af4908af1d1f9e546c3bfae4cfe79596e03586d24574818ca21233` |
| L0 `attach.md` | `db1abcc014473353baab0ed8d678b9449cd30a3668f98cb456f18b20cd2078af` |
| L0 `_comum.md` | `09e32fbcfa6c12e88b05744041e9430f266204e71a10816f0d16442007fa5b19` |
| recibo vanilla público | `39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7` |
| recibo B público | `8e9ff5cfa877191cdcf9ac61da0070ea9b6e13f945040014ce99cf7374b8dd90` |

Fontes produtivas decisivas:

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/compiler/layout/helpers.rs` | `933e4d51a6d97a811512ce03861b3de789c22898d6a5de3af52af37e68633e92` |
| `01_core/src/compiler/layout/equation.rs` | `5a33b6dc9e7981fbeaf3fd1645460cc58f2f85ed89644d67e589fae301c41af0` |
| `01_core/src/compiler/math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |
| `01_core/src/compiler/math/layout/mod.rs` | `312be2fd7aed72f2c6e4c90a442e21f70f2e755ef4ee093cd270f7a3dffe853b` |
| `03_infra/src/font_metrics.rs` | `2716bf725b8bb6802fcfea13ea0a0a9caa1b49278c472760e7874b087833dfd6` |
| vanilla `math/scripts.rs` | `d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b` |
| vanilla `math/text.rs` | `c913d5620f91e1c747cecd3e05283245c1e558f9db280a193cc69972941b15e8` |
| vanilla `fragment/glyph.rs` | `1b57225c90db398ecf9a9af7c4bef47285eb5e434a554948bd086cbe7ad411fb` |

Comandos principais, repetidos bilateralmente para cada fonte:

```text
/usr/local/bin/typst compile SOURCE OUT-vanilla.svg --format svg
target/release/typst compile SOURCE OUT-candidate.svg --format svg
rg -o 'viewBox="[^"]+"|matrix\(1 0 0 -1 [^)]+' OUT.svg
sha256sum SOURCE OUT-vanilla.svg OUT-candidate.svg
```

## Reprodução independente dos dois vetores congelados

As fontes exatas foram reconstruídas sem consultar fonte privada:

```typst
// /tmp/p1293-attach-causal-display.typ
#set page(width: auto, height: auto, margin: 0pt)
$ attach(x, t: T, b: B, tl: L, bl: M, tr: R, br: S) $
```

SHA-256 `59667014f0dbc6e3a492de28ac43a4dbc2b098e8ff7320af4c59a3ec62be903d`.

```typst
// /tmp/p1293-attach-causal-inline.typ
#set page(width: auto, height: auto, margin: 0pt)
#math.equation(math.attach([x], t: [T], b: [B], tl: [L], bl: [M], tr: [R], br: [S]), block: false)
```

SHA-256 `237f5727e22f7947d56179870ed91e583bc4ac1497fffb741df88f8e4e37a95b`.

| Vetor | Vanilla | Candidato | SHA vanilla | SHA candidato |
|---|---:|---:|---|---|
| display | `23.2705 × 19.4843` | `31.6481 × 19.4843` | `421fc32b2ab73b4e116ad1300958b32259915759c5dde8d95367df1559a23f72` | `62269597e1e308a7c044db5e88e138dbd0d2e4c5b1b73390adcc0a1fd6533dba` |
| inline | `19.7681 × 9.6041` | `27.0523 × 9.6041` | `c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c` | `49c6cdf86ffd9df7bd7f869de0057e810f8e9f40ad175cd30c1bc85ea2765e54` |

Os hashes vanilla coincidem exatamente com o recibo congelado. Portanto os
probes próprios não são apenas substitutos da mesma classe: reproduzem os dois
casos originais.

## Decomposição por slots

Probes mínimos independentes mostraram:

| Conteúdo | Vanilla width | Candidato width | Resultado causal |
|---|---:|---:|---|
| base `x` | `6.292` | `6.292` | controle GREEN |
| `attach(x)` | `6.292` | `6.292` | controle GREEN |
| somente `tr:R` | `13.6609` | `13.6609` | post-script GREEN |
| somente `br:S` | `12.5906` | `12.5906` | post-script GREEN |
| `tr:R, br:S` | `13.6609` | `13.6609` | dois post-scripts GREEN |
| somente `tl:L` | `12.9448` | `18.9816` | RED, `+6.0368` vazio |
| somente `bl:M` | `15.9016` | `24.8952` | RED, `+8.9936` vazio |
| `tl:L, bl:M` | `15.9016` | `24.8952` | RED, máximo do lado esquerdo |

Nos pares GREEN e RED, posições relativas e paths dos glifos permanecem.
Logo kern, base, script e shifts visíveis não explicam a cauda. A condição
necessária e suficiente deste defeito é existir filho material à esquerda do
primeiro item do wrapper semântico; em `attach`, isso é exatamente um
pre-script.

## Causa mínima do excesso simétrico

O `attach.rs` calcula a extensão e posições na mesma forma do vanilla:

- cristalino `attach.rs:424-426`: `pre + base + post`;
- cristalino `attach.rs:432-468`: insere a base primeiro e depois os
  pre/post-scripts nas posições calculadas;
- vanilla `scripts.rs:173-189`: a mesma largura cria um `Frame::soft` com
  tamanho explícito; `scripts.rs:191-204` insere base e attachments dentro
  desse frame.

A diferença surge depois do math layout:

1. `equation.rs:454-472` envolve os itens achatados em um único
   `FrameItem::Semantic`, preservando a ordem: base primeiro, pre-script
   depois;
2. `helpers.rs:28-30` define a posição do `Semantic` como a posição do
   **primeiro** filho;
3. `helpers.rs:60-69` define sua largura como
   `right(children) - min_x(children)`;
4. `helpers.rs:77-87` volta a somar `item_pos + item_width`.

Assim, o limite direito medido é:

```text
first_child_x + rightmost_child_right - min_child_x
```

e não `rightmost_child_right`. Quando a base é o primeiro filho e um
pre-script está à esquerda, `first_child_x > min_child_x` e essa diferença é
adicionada uma segunda vez.

Probe mínimo inline `math.attach([x], tl:[L])`:

```text
base/primeiro filho x = 5.4285
pre-script/min x       = 0.6160
posições candidato = posições vanilla
viewBox vanilla        = 11.2365
viewBox candidato      = 16.2250
```

O termo espúrio é `5.4285 - 0.6160 = 4.8125`; o residual de `0.1760` é
separado abaixo. Sem pre-script, `first_x == min_x` e o erro desaparece.

No vetor display congelado, a medição semântica excede a extensão matemática
em `8.3776pt`; a centragem adiada da equação desloca todos os glifos por
`8.3776 / 2 = 4.1888pt`, produzindo precisamente a margem vazia simétrica
observada. Nenhuma constante de fixture entra na explicação.

### Hipótese refutável H1

H1: todo `Semantic` cujo primeiro filho não é o mais à esquerda recebe
exatamente o sobrealcance produzido pela composição `first + (right - min)`.

Refutadores executados:

- post-scripts mantêm `first_x == min_x` e são GREEN;
- pre-script único mantém toda posição relativa mas abre exatamente a cauda;
- top+bottom do mesmo lado conserva o máximo, como a fórmula de `attach`;
- adicionar marcador após a equação expõe o cursor/extensão real separado da
  largura da página e confirma que a cauda grande pertence ao wrapper.

H1 ficou `Preserved` para todos os probes próprios; `Unknown = 0` para a
causa da cauda grande.

## Residual independente do inline qualificado

O marcador seguinte separa a extensão matemática da cauda do wrapper:

```typst
#math.equation(math.attach([x], t: [T], b: [B], tl: [L], bl: [M], tr: [R], br: [S]), block: false)Z
```

Fonte SHA-256
`b6cf78ebd37d621ef303d54c25e8a4e9cde0ff5c34eb802c5d2d7f403b712d76`.
O marcador começa em `19.7681` no vanilla e `19.9914` no candidato. Portanto,
remover apenas a dupla origem semântica ainda deixa `+0.2233pt` no vetor
inline qualificado.

Dois probes mínimos isolam a mesma família:

| Probe + marcador | Vanilla x do marcador | Candidato x | Delta |
|---|---:|---:|---:|
| `math.attach([x], tl:[L])` | `11.2365` | `11.4125` | `0.1760` |
| `math.attach([x], tr:[R])` | `12.0912` | `12.3145` | `0.2233` |

Os deltas convertem exatamente para as entradas MATH de italics correction:

```text
0.1760 / 11pt × 1000 = 16du   (x)
0.2233 / 7.7pt × 1000 = 29du  (R em Script)
```

`font_metrics.rs:1511-1537` adiciona italics correction a qualquer texto de
um único caractere sempre que `style.math` é verdadeiro. Porém, o vanilla
separa as duas morfologias:

- `math/fragment/glyph.rs:207-215` adiciona IC ao `GlyphFragment`;
- `math/ir/resolve.rs:271-305` converte conteúdo textual `[x]`/`[R]` em
  `TextItem`;
- `math/text.rs:15-40` layouta esse `TextItem` como inline/hbox, sem chamar o
  update de glifo e sem IC matemática.

No cristalino, `_comum/mod.rs:897-903` reconhece corretamente que
`Content::Text` é `TextItem`, mas produz o mesmo `FrameItem::Text` e o mesmo
`TextStyle.math=true` usados por folhas math. A chamada posterior de
`FontMetrics::advance` perdeu a proveniência necessária para decidir IC.

### Hipótese refutável H2

H2: o residual qualificado é a IC aplicada indevidamente a `TextItem`, não
uma constante/fórmula de `attach`.

Contraprova: retirar IC indiscriminadamente de todo texto math quebraria
`MathIdent`/`MathText` que correspondem ao `GlyphFragment` vanilla e para os
quais `glyph.rs:207-215` exige IC. Logo uma alteração unilateral e global de
`font_metrics.rs` não é uma correção segura. H2 fica `Preserved` quanto à
causa numérica; a forma mínima segura de transportar a distinção
`GlyphFragment` versus `TextItem` fica `Unknown` e requer decisão L0.

## Ownership e veredito causal

| Questão | Veredito |
|---|---|
| a fórmula de `attach.rs` causa a cauda simétrica? | **não**; fonte e posições a refutam |
| `_comum/mod.rs` causa a cauda grande? | **não diretamente**; ele achata os itens, mas o sobrealcance nasce na medição do wrapper |
| owner 1:1 exato da cauda grande | `00_nucleo/prompts/compiler/layout/helpers.md` → `01_core/src/compiler/layout/helpers.rs` |
| owner do valor indevido de IC residual | `00_nucleo/prompts/infra/font_metrics.md` → `03_infra/src/font_metrics.rs` |
| pode corrigir somente `layout/attach.rs`? | **não** |
| basta corrigir somente `helpers.rs` para os dois fingerprints? | display: causalmente sim para a cauda; inline qualificado: **não**, resta `0.2233pt` |
| owners mínimos seguros para fechar inline | **Unknown** entre `font_metrics.rs` e a projeção tipada em `_comum/mod.rs`; o estado atual perdeu a proveniência |

Conclusão operacional: o lote B deve permanecer bloqueado. Reabrir somente o
L0 de `attach` seria owner incorreto. A próxima cadeia precisa ao menos do
owner `layout/helpers`; para fechar também o inline qualificado, deve medir e
decidir em L0 como preservar a distinção `GlyphFragment`/`TextItem` entre
`_comum/mod.rs` e `infra/font_metrics`, sem remover IC dos glifos matemáticos.

Nenhum código foi escrito e nenhum valor de fixture foi proposto.
