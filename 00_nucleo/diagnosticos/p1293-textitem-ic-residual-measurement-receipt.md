# P1293 — medição causal independente do residual `TextItem`/IC

## Escopo e segregação

Diagnóstico estritamente read-only do produto, realizado sob o regime completo da
skill `tekt-materializacao-segregada`. A única escrita no repositório é este recibo;
os probes próprios vivem sob `/tmp/p1293-textitem-residual-*`. Não foram lidos o
contrato/teste protegido, recibos RED/discrimination, oráculos privados nem
`/tmp/p1293-measure.py`.

Entrada pública autorizada: `p1293-implementation-receipt-b.md`, SHA-256
`785c9683a5f5725c3a0d33c27525dcdcb9638399bbdadbb12f14caad563ee4c3`.

## Proveniência

- início da medição: `2026-09-01T21:54:05-03:00`;
- checkpoint final do estado: `2026-09-01T22:01:14.871396010-03:00`;
- branch: `Tekt`;
- HEAD: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`;
- working tree não commitado: `git diff HEAD --stat` = 46 ficheiros,
  3521 inserções, 381 remoções. O conjunto tracked exacto era:

```text
00_nucleo/prompts/compiler/{eval.md,eval/bindings/field_access.md,eval/call_dispatch.md,eval/math.md,eval/repr.md,eval/tests.md,layout/helpers.md,layout/text.md,math/layout/_comum.md,math/layout/attach.md,stdlib/foundations/float.md,stdlib/html.md,stdlib/math_style.md,stdlib/structural/math.md}
00_nucleo/prompts/entities/{elements/math_attach.md,layout_types.md,style_chain.md}
00_nucleo/prompts/infra/font_metrics.md
01_core/src/compiler/eval/{bindings/field_access.rs,call_dispatch.rs,math.rs,mod.rs,repr.rs,tests.rs}
01_core/src/compiler/layout/{helpers.rs,text.rs}
01_core/src/compiler/math/layout/{accent.rs,attach.rs,cancel.rs,cases.rs,frac.rs,matrix.rs,mod.rs,root.rs,spacing.rs,tests.rs,underover.rs,vec.rs}
01_core/src/compiler/stdlib/{foundations/float.rs,html.rs,math_style.rs,structural/math.rs}
01_core/src/entities/{elements/math_attach.rs,layout_types.rs,style_chain.rs}
03_infra/src/font_metrics.rs
```

Havia ainda artefactos P1293 untracked de outros autores; não foram lidos salvo o
recibo B autorizado. Este recibo não existia no checkpoint acima.

Binários:

- vanilla `/usr/local/bin/typst`: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- candidato `target/release/typst`: `172d54bec2f8fb43bc52b47619e6718c6a2139f50efce63a8d8a677a7ab09a46`.

Fontes causais:

- `layout_types.rs`: `38761a95d2b39d34868af3bb5bbfd58dd9404b902c2d242da93ebe6a94b7a1ed`;
- `math/layout/mod.rs`: `f612884fc3f39cdb509fa20e59d7d5f077be33dd49052b34aaf56e08a9d6c81c`;
- `math/layout/attach.rs`: `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5`;
- `layout/helpers.rs`: `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116`;
- `font_metrics.rs`: `67fd998c7ff584f19aecf65daed4c1b74a245221a3d2ce3905f4c3bc200de6ef`;
- `shaper.rs`: `e5231d6bbeb2235f6582c04bd385faabbf0a214405903ec88bbdec05ce54c185`;
- `NewCMMath-Book.otf`: `60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`.

Vanilla ratificado lido:

- `math/ir/resolve.rs`: `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112`;
- `math/text.rs`: `c913d5620f91e1c747cecd3e05283245c1e558f9db280a193cc69972941b15e8`;
- `text/mod.rs`: `8eec4f723d9eca5602f3af0460cdc3c8507d6c0d2d3bfe109fd012ba95b7f444`.

## Comandos reproduzíveis

```sh
/usr/local/bin/typst compile --format svg SOURCE OUTPUT-vanilla.svg
target/release/typst compile --format svg SOURCE OUTPUT-candidate.svg
python3 -c '... regex de viewBox e matrix(...) nos SVGs ...'
hb-shape 03_infra/fixtures/fonts/NewCMMath-Book.otf R
hb-shape --script=math 03_infra/fixtures/fonts/NewCMMath-Book.otf R
hb-shape --script=math --features='ssty=1' 03_infra/fixtures/fonts/NewCMMath-Book.otf R
hb-shape --features='ssty=1' 03_infra/fixtures/fonts/NewCMMath-Book.otf R
sha256sum /tmp/p1293-textitem-residual-*
```

O `hb-shape` devolveu, respectivamente, `[R=0+736]`, `[R=0+736]`,
`[R.st=0+829]`, `[R=0+736]`: `ssty=1` só produz a variante quando o script
OpenType é `math`.

## Medição pública

Vetor reproduzido:

```typst
#set page(width: auto, height: auto, margin: 0pt)
#math.equation(math.attach([x], t: [T], b: [B], tl: [L], bl: [M], tr: [R], br: [S]), block: false)
```

O SVG é `19.7681 × 9.6041 pt` no vanilla e `19.8682 × 9.6041 pt` no
candidato: residual horizontal `+0.1001 pt`, sem residual de altura.

Decomposição com `Z` imediatamente depois da equação (a coordenada x de `Z` é a
extensão consumida):

| probe | vanilla x(Z) | candidato x(Z) | delta |
|---|---:|---:|---:|
| base `[x]` | 5.8080 | 5.8080 | 0 |
| `tl: [L]` | 11.2365 | 11.2365 | 0 |
| `bl: [M]` | 13.4849 | 13.4849 | 0 |
| `tr: [R]` | 12.0912 | 12.1913 | **+0.1001** |
| `br: [S]` | 10.7052 | 10.5292 | -0.1760 |

No `tr`, as posições visíveis de base e R são idênticas nos dois binários:
`x(base)=0`, `x(R)=5.808`. Portanto posição, kern e largura da base não explicam
o residual; ele é extensão à direita depois do layout.

Hashes dos probes/output principais:

| artefacto | SHA-256 |
|---|---|
| `inline.typ` | `237f5727e22f7947d56179870ed91e583bc4ac1497fffb741df88f8e4e37a95b` |
| `inline-vanilla.svg` | `c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c` |
| `inline-candidate.svg` | `430ce99154846323b74ce5a0e176a7191dd2b647fc315df2cad852f61c1eab9a` |
| `tr-marker.typ` | `0de8d46074a19f17dcb4cbc46a58948d26b9e470d102dbae40b83e556964b8bb` |
| `tr-marker-vanilla.svg` | `f01f0c9aad077b81e6800cd1854d8158d2dd007fd2a8e0df9a644268bdd84dd1` |
| `tr-marker-candidate.svg` | `aa10ded09b43d79c9fb7c24f4a3dde65ad57ebcc59ebe46985a48be9fdd336b3` |
| `br-marker.typ` | `86937bed8527aa790803c9ff01174f318349db41affd1a9786dcdff05f0dd703` |
| `br-marker-vanilla.svg` | `7c62f5e07b5a70847bfaaf9c967daeba3ef50d4f226facaa65fba9919c027269` |
| `br-marker-candidate.svg` | `db619dc7216dddb78d8d4ed3a05d26b18721de0f4fbaa074d59bd3eeff9bf88d` |

## Estado de `math_text_item`

O campo existe em `layout_types.rs:225-231`. Para cada argumento markup direto do
vetor (`x,T,B,L,M,R,S`), `math/layout/mod.rs:897-928` reconhece
`Content::Text`, constrói `TextStyle { math_text_item: true, .. }`, mede e chama
`layout_text_node`; esta função copia o estilo para `FrameItem::Text` em
`mod.rs:1389-1417`. Logo os sete `FrameItem::Text` pré-shaping carregam
`math_text_item=true`. `shaper.rs:656-668` clona esse estilo, inclusive o campo,
para `FrameItem::TextShaped`. A propagação está completa; o consumidor é que não
consulta a proveniência.

MathIdent/MathText continuam com `false` por `mod.rs:918-922` e são, portanto,
uma classe distinta. O residual medido não é IC legítima de MathIdent.

## Fórmula causal mínima

Da fonte, sem constante de fixture:

- `R` tem advance 736du;
- `ssty=1` substitui por `R.st`, advance 829du;
- `SpaceAfterScript` é 56du;
- script tem 7.7pt e a base 11pt.

O MathBox é medido pelo shaper genérico como R regular: `736 × 7.7/1000 =
5.6672pt`. `attach.rs:397-400` reserva ainda `56 × 11/1000 = 0.616pt`. Assim
o limite lógico termina em `5.808 + 5.6672 + 0.616 = 12.0912pt`, exatamente o
vanilla.

Depois do layout, porém, `shaper.rs:525-535` pede `ssty=1` para qualquer texto
math singular e `shaper.rs:567-585` força o script OpenType `math` e aplica a
feature, sem testar `math_text_item`. O R desenhado avança `829 × 7.7/1000 =
6.3833pt`. A extensão real do item é `5.808 + 6.3833 = 12.1913pt`. Logo:

```text
overflow = (829 - 736) × 7.7/1000 - 56 × 11/1000
         = 0.7161 - 0.6160
         = 0.1001 pt
```

Isto é extensão externa pós-layout causada por `ssty`, não padding nem math-kern.

## Contraprovas refutáveis

1. Variar o glifo sem mudar attach deve seguir a tabela da fonte. Para `K`,
   base/st = 778/874du: previsão `(874-778)×7.7/1000-0.616 = +0.1232pt`;
   medido `12.5378-12.4146 = +0.1232pt`.
2. Para `W`, base/st = 1028/1151du: previsão `+0.3311pt`; medido
   `14.6707-14.3396 = +0.3311pt`.
3. Texto multi-caractere `[RR]` é inelegível em `ssty_eligible_text`: previsto
   delta zero; medido x(Z) `17.7584pt` em ambos os binários.
4. `hb-shape --features=ssty=1 R` sem script `math` mantém 736du; com
   `--script=math` produz `R.st`/829du. Isso distingue script/feature de kern.

Hashes: K source `bfab30acc41dd1f399311b59b84eb7f7da9d37216047a656dafc13049c76c8aa`,
vanilla `b68ec198079ffbb5ae76df7ca381101dc7b06a17313b26ac2fc2af98dd345df1`,
candidato `07e85fd052f76f862a51ea77389e8b3125690f52937d16d5893fd1d0b44c9401`;
W source `5be8e5e67932b5543db3dd394e361464d83db278fa060ef9cc62453cc948253a`,
vanilla `06586c857bd63a6bf94168cf6ed215314f4e21c61e3af5786220ee7eb90524a1`,
candidato `48ae6612855efec45c7163999b33225f498cdcb00fa77f16230f6b028db389e3`;
RR source `1a79a83d96842dc78640da708f7be7786a9e628c87b64cfe4b73cbe8c88da5f1`,
vanilla `911ca521812924a6520fe4b955efac465aebb34311c67b252175d0cfecfc9f2a`,
candidato `f155aca0d745bc5935b1440e5f46472bdec01072921eec669eead45d185b20a8`.

## Vanilla e ownership

Vanilla `math/ir/resolve.rs:271-305` classifica o markup como `TextItem` e
`math/text.rs:15-40` o envia ao `inline::layout_inline`. Os estilos locais
(`resolve.rs:28-36`) só alteram edges/overhang. O shaper de texto comum recebe a
feature `ssty` de `text/mod.rs:1457-1462`, mas não força o script OpenType
`math`; a prova `hb-shape` mostra por que a substituição fica inerte.

Owner causal 1:1 exacto do `+0.1001pt`:

- produto: `03_infra/src/shaper.rs`, especialmente `try_shape` em
  `525-585`;
- L0 proprietário declarado no header: `00_nucleo/prompts/infra/shaper.md`.

Portanto a correção **não cabe somente em `attach.rs`**, `_comum/mod.rs` ou
`font_metrics.rs`; o residual positivo requer o owner `shaper.rs` e atualização
prévia do seu L0. A hipótese mínima é: TextItem deve conservar shaping inline,
sem ativar efectivamente GSUB math-only; o campo já transportado fornece a
distinção refutável.

Risco latente separado: `font_metrics.rs:455-463` (`ssty_level_of`) também não
consulta `math_text_item`. Neste vetor ele não é causal porque
`advance_shaped`/`shaped_width` mede o TextItem pelo shaper genérico em 736du,
mas um fallback onde essa via devolva `None` pode voltar a medir `.st`. Owner:
`infra/font_metrics.md` → `font_metrics.rs`; não confundir esse risco com a causa
observada.

Diferença ortogonal confirmada no `br`: `attach.rs:343-382` extrai IC da base
mesmo quando seu `FrameItem::Text.style.math_text_item=true`; para `x`, IC=16du
em 11pt = 0.176pt, exatamente o deslocamento `5.808 -> 5.632`. Esse owner precisa
ser corrigido se o gate exigir paridade de todas as posições, mas não produz o
`+0.1001pt` da largura final (o ramo `tr` domina o máximo).

## Conclusão

Classificação: **causa isolada, não Unknown**. O `+0.1001pt` é a diferença entre
o MathBox TextItem medido com glifo regular e o `FrameItem::TextShaped` renderizado
com `.st`, líquida do trailing `SpaceAfterScript`. É consumo incompleto do novo
estado no owner `shaper.rs`; MathIdent legítimo, padding e kern foram refutados.

SHA-256 deste recibo: calcular após a escrita (`sha256sum
00_nucleo/diagnosticos/p1293-textitem-ic-residual-measurement-receipt.md`).
