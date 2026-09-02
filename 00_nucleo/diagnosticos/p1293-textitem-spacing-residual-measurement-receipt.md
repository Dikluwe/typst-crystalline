# P1293 — medição independente do residual `SpaceAfterScript`

## Escopo e regime

Auditoria causal read-only do produto sob o protocolo completo da skill
`tekt-materializacao-segregada`, por segregação de capacidades e artefactos, sem
alegação de isolamento técnico do filesystem compartilhado. A única escrita no
repositório é este recibo; probes próprios foram escritos somente em
`/tmp/p1293-textitem-spacing-residual-*`.

Não foram lidos materialization adicional, contrato/oracle/testes protegidos,
manifesto, selo, RED/discrimination privados ou expectativas protegidas. Este
recibo não emite aprovação do lote B.

Entrada pública autorizada: `p1293-implementation-receipt-b.md`, SHA-256
`382df2f2d7623acd7ba1eb10ee66f483f1af546c752c36bb66ba0769747c107d`.

## Proveniência

- instante do checkpoint: `2026-09-01T22:42:59.184987970-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada;
- `git status --short | sha256sum`:
  `939716d9e64155d2cb88cb0fc44a60878dae35bd545dfc9e8119bdf676b069f7`;
- `git diff HEAD --stat | sha256sum`:
  `65451d28c3c1612e10f4646f354d0b548f513d8d90b188f37639f76b0de4b285`;
- stat: `48 files changed, 3781 insertions(+), 397 deletions(-)`; este recibo
  ainda não existia nesse checkpoint.

Binários:

- candidato release: SHA-256
  `f0e5ce0810c7a7184f17b45e719c850fe30d14598d12f369b54fd060ddb3aed3`;
- vanilla ratificado `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Fontes produtivas medidas:

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/compiler/math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |
| `01_core/src/compiler/math/layout/mod.rs` | `f612884fc3f39cdb509fa20e59d7d5f077be33dd49052b34aaf56e08a9d6c81c` |
| `01_core/src/compiler/layout/equation.rs` | `5a33b6dc9e7981fbeaf3fd1645460cc58f2f85ed89644d67e589fae301c41af0` |
| `01_core/src/compiler/layout/helpers.rs` | `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116` |
| `01_core/src/compiler/layout/mod.rs` | `9d4aa8e56a4e9157cbab345ddd40cfaae73c4c4c60f7fac85741b4c79e05069d` |
| `01_core/src/entities/layout_types.rs` | `782b5866d5c97beb41f1df4338fb62bf5f133fabfbf02181d1c6760c631ea4db` |
| `03_infra/src/shaper.rs` | `1b7917b1efb442e1d21862d3117cd95196f640eabdede9ea83d9e78ca2c7074e` |
| `03_infra/src/font_metrics.rs` | `f18bff7c5637f0d94eb48c6d184090d04e0446ef9a71132f8dfef2a050fef990` |

Fonte vanilla `math/scripts.rs`: SHA-256
`d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b`.
Fonte NewCMMath-Book: SHA-256
`60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`.

L0s relevantes confirmados/lidos:

| Owner | SHA-256 |
|---|---|
| `compiler/layout/equation.md` | `8a557ebeb62cef433b30d7f1db66acb7ed27526e72770ceb2bd0a12514117387` |
| `compiler/layout.md` | `8b3248416ede53bcb836b219cab4de53129f597a154959a19c1d22d26a09e835` |
| `compiler/layout/helpers.md` | `7481a304279c26a6df3ff9c5258e883d2f5a45eba53b8de2c717f543a68f2c6c` |
| `compiler/math/layout/attach.md` | `db1abcc014473353baab0ed8d678b9449cd30a3668f98cb456f18b20cd2078af` |
| `entities/layout_types.md` | `60cde357984c8b75f7e580ee26e1cb2f826b98e48f4b6cf364b8b5fde160b0fd` |
| `infra/shaper.md` | `fbeaa9aa47bfc6ba97f2f163ba84e8728395ad3c8121cc5cf776aad5869b5456` |
| `infra/font_metrics.md` | `405416e31c400ea478f80e678ff1bee304ec6c26867e1c43eb4f3b995dad6078` |

## Comandos reproduzíveis

Para cada fonte pública:

```sh
/usr/local/bin/typst compile --format svg SOURCE OUTPUT-vanilla.svg
target/release/typst compile --format svg SOURCE OUTPUT-candidate.svg
```

Inspecção independente de fonte/shaping:

```sh
hb-shape 03_infra/fixtures/fonts/NewCMMath-Book.otf R
hb-shape --script=math --features='ssty=1' \
  03_infra/fixtures/fonts/NewCMMath-Book.otf R
python3 -c 'from fontTools.ttLib import TTFont; ... MathConstants, cmap e hmtx ...'
python3 -c '... extrai viewBox e transforms matrix dos SVGs ...'
sha256sum /tmp/p1293-textitem-spacing-residual-*
```

Resultados de fonte: `R=736du`, `R.st=829du`, `K=778du`, `W=1028du`,
`x=528du`, `SpaceAfterScript=56du`. HarfBuzz devolveu `[R=0+736]` no
shaping comum e `[R.st=0+829]` com script `math` + `ssty=1`.

## Reprodução bilateral

Vetor bloqueante:

```typst
#set page(width: auto, height: auto, margin: 0pt)
#math.equation(math.attach([x], t: [T], b: [B], tl: [L], bl: [M], tr: [R], br: [S]), block: false)
```

Medido: vanilla `19.7681 × 9.6041pt`, candidato atual
`19.1521 × 9.6041pt`; residual `−0.6160pt`. Antes da exclusão de `ssty`, o
candidato medido no predecessor era `19.8682pt`.

Adicionar `Z` imediatamente depois da equação muda o diagnóstico:

- x(Z) vanilla = `19.7681pt`;
- x(Z) candidato = `19.7681pt`;
- viewBox final vanilla = candidato = `26.4121pt`.

Portanto o avanço inline lógico já preserva a largura inteira da equação. A
perda ocorre somente quando a fórmula é terminal e `width:auto` deriva a página
dos filhos materiais.

### Decomposição exacta do vetor completo

No candidato atual:

- início do `tr: [R]`: `13.4849pt`, igual ao vanilla;
- advance comum de R: `736 × 7.7 / 1000 = 5.6672pt`;
- limite material direito: `13.4849 + 5.6672 = 19.1521pt`;
- trailing MATH requerido: `56 × 11 / 1000 = 0.6160pt`;
- largura lógica/frame: `19.1521 + 0.6160 = 19.7681pt`.

O `tr` começa exactamente na aresta direita da base (`pre_width + base_width`),
logo seu math-kern é zero neste witness. A IC de R na fonte não entra porque é
`TextItem`; a IC de x (`16du × 11/1000 = 0.176pt`) só explica o deslocamento
independente do `br` (`13.4849 → 13.3089`) e não governa o máximo direito.

## Refutadores K/W/RR

Probes terminais `math.attach([x], tr: [...])`:

| Script | Candidato | Vanilla | Delta |
|---|---:|---:|---:|
| `[R]` | 11.4752 | 12.0912 | -0.6160 |
| `[K]` | 11.7986 | 12.4146 | -0.6160 |
| `[W]` | 13.7236 | 14.3396 | -0.6160 |
| `[RR]` | 17.1424 | 17.7584 | -0.6160 |

O delta invariável atravessa advances distintos e texto multi-caractere
inelegível a `ssty`; isso refuta IC, kerning, GSUB, tamanho de glifo e escala de
script como causa do novo residual. O controle sem post-script é `5.808pt` nos
dois renderers.

Um slot material vazio expõe uma segunda divergência de obrigação: `tr: []`
mede `6.424pt` no vanilla e `5.808pt` no candidato, ou seja, o vanilla ainda
preserva o frame com `SpaceAfterScript` enquanto o L0 attach vigente diz que
`Some(Content::Empty)` não recebe spacing. Este achado refuta essa cláusula,
mas não muda a decomposição do witness não vazio.

Hashes seleccionados:

| Artefacto | SHA-256 |
|---|---|
| `inline.typ` | `237f5727e22f7947d56179870ed91e583bc4ac1497fffb741df88f8e4e37a95b` |
| `inline-vanilla.svg` | `c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c` |
| `inline-candidate.svg` | `78760639f48dc12c87929cf86bacc00403272745c18a58c56f5caf9e8459c03a` |
| `inline-marker.typ` | `b6cf78ebd37d621ef303d54c25e8a4e9cde0ff5c34eb802c5d2d7f403b712d76` |
| `inline-marker-vanilla.svg` | `28527d8d53f6691643f28a9d7c4dc47da60d971befee8a30bb8d1f4939814d13` |
| `inline-marker-candidate.svg` | `d3e40803f36eb6bf5384e11d65da2950905e39a380f24b3f7bedda19c748c126` |
| `tr-r.typ` | `a3755539c3d129f0ef3a7824e46c54f426932f05543a776f2f4359700cddea23` |
| `tr-r-vanilla.svg` | `098a1ce8c307a741fe4bf669367da01cded9ff6659da571c791e99add3dea455` |
| `tr-r-candidate.svg` | `263439789b1a9d3020864546af1f8fd42da43b4f3f6c9283e321f9e5c2254fce` |
| `tr-k.typ` | `ee07a2de2945700fbe3f3f1a5ff2c10c1ed721e54f6c5617a87e8de49a0e01d2` |
| `tr-w.typ` | `266e47b7212d746bf0cee4456c24b83c5d8eca7a56ab5defeb7c49cf58d5673c` |
| `tr-rr.typ` | `8c0dc8364a425e70455caacbcb52ab86877f35337402dfa032fcf47f520e72c7` |

## Cadeia causal file:line

1. `attach.rs:298-301,397-426` lê `SpaceAfterScript`, acrescenta-o ao
   `tr_post` e produz `total_width`; a fórmula do attach está correcta para o
   witness material não vazio.
2. `math/layout/mod.rs:598-624` preserva `math_box.width` em
   `EquationExtent.width`, separadamente dos items achatados.
3. `layout/equation.rs:495-499` avança o cursor inline por esse extent. Isto é
   confirmado pelo marcador na posição exacta.
4. `layout/equation.rs:454-472`, contudo, envolve somente os filhos em
   `FrameItem::Semantic::Formula`; o envelope público em
   `layout_types.rs:354-367` não tem largura/frame size.
5. `finish()` em `layout/mod.rs:2118-2138` faz flush; `flush_line` em
   `cursor.rs:664-666` reinicia `cursor_x`. A largura lógica terminal deixa de
   estar disponível.
6. `compute_page_width` em `layout/mod.rs:1222-1252` usa
   `helpers::line_content_right` sobre os items. `helpers.rs:46-77` mede
   `Semantic` apenas pela união material dos filhos, portanto obtém `19.1521`,
   não o frame lógico `19.7681`.

Vanilla `math/scripts.rs:149-177` inclui `space_after_script` no post-width e
`scripts.rs:188-209` materializa `Frame::soft(Size::new(width, height))`; o
tamanho do frame sobrevive mesmo sem tinta na cauda.

## Por que o predecessor parecia `+0.1001pt`

Antes da exclusão de `ssty`, o filho R era renderizado como `R.st`:

```text
extra ssty = (829 - 736) × 7.7 / 1000 = +0.7161pt
frame trailing perdido                           = -0.6160pt
observável líquido                               = +0.1001pt
```

A hipótese anterior mediu correctamente o termo `ssty` e até registou essa
subtracção, mas a extensão visível de `.st` mascarava uma falha independente de
preservação do tamanho do frame. Retirar `ssty` elimina `+0.7161` e deixa o
termo `−0.6160` exposto. Não foi alteração de constante, kern, IC ou escala.

## Classificação ADR-0107/0108 e owner

ADR-0107: dimensão de uma página `auto`, largura mensurável da fórmula e posição
do conteúdo seguinte são geometria observável da linguagem. `MathBox`, cursor,
`Semantic`, flattening e a escolha Rust de recomputar filhos são mecânica usada
somente para localizar a causa.

ADR-0108: a decisão segue as medições acima. A conclusão é refutada se:

- algum K/W/RR deixar de perder exactamente a constante MATH derivada da fonte;
- o marcador posterior não começar no extent vanilla;
- `EquationExtent.width` não contiver os `0.6160pt`;
- preservar o frame lógico ainda deixar a página terminal em `19.1521pt`; ou
- outra fonte fizer o delta divergir do seu próprio
  `SpaceAfterScript × tamanho-base / upem`.

O menor owner semântico existente é
`00_nucleo/prompts/compiler/layout/equation.md` →
`01_core/src/compiler/layout/equation.rs`: ele já possui simultaneamente o
`EquationExtent.width` e a autoria do envelope `Semantic::Formula`. O L0 vigente,
porém, é insuficiente e contradito: `equation.md:529-533` declara este residual
fora de equation/cursor/auto-page. Deve ser reaberto antes de qualquer código.

`helpers.md` não é owner suficiente: sua regra estrutural mede somente filhos e
proíbe semântica específica de elemento; sem metadata, não pode reconstruir um
frame maior que a sua tinta. `attach.md` também não é owner da perda terminal:
ele já calcula o total correcto, embora sua cláusula sobre slot material vazio
tenha sido refutada e exija revisão separada. O owner genérico `layout.md` só
seria necessário se a decisão escolhida alterar o estado/finalização global em
vez de preservar a largura no produtor Formula.

Gate ADR-0127:

- reabertura do L0 `equation.md` é obrigatória porque sua classificação vigente
  exclui precisamente o owner agora medido;
- se a solução escolhida acrescentar largura/tamanho a
  `FrameItem::Semantic`, isso muda campo de entidade pública e exige nova
  confirmação humana, categoria 1, além de atualizar
  `entities/layout_types.md`;
- esta auditoria não demonstra que um novo campo público seja a única solução.
  Uma retenção interna pode ser correção de fórmula/paridade em fluxo contínuo,
  mas só depois de L0 atualizado e de o owner 1:1 exacto ser declarado.

## Estado

Causa numérica e cadeia de perda: **isoladas**. Solução/owner físico final:
**bloqueado à reabertura L0 e à escolha ADR-0127 acima**. Nenhuma compensação
nominal, constante de fixture ou aprovação do lote é proposta.

O SHA-256 deste recibo deve ser calculado externamente após a gravação.
