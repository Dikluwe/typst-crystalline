# P1293 — recibo parcial de implementação do lote B

## Estado

```text
status: BLOCKED_EMPTY_MARKUP_CARRIER_COLLISION
lot: B-only
serial: attach-fragment-semantic-IC
own-red-green-core: GREEN 35/35
own-red-green-infra: GREEN 4/4
B-P07: 8/8 GREEN
attach-display: GREEN exato
attach-inline: GREEN exato
own-empty-markup-control: RED, candidato -0.6160pt na largura
unknown: 1 bloqueante
lot-B-approval: false
lot-C-D: forbidden
```

O serial attach-fragment-IC foi implementado exclusivamente em
`01_core/src/compiler/math/layout/attach.rs`. Ao selecionar a IC semântica da
base, somente `FrameItem::Text` com `style.math_text_item=true` fornece zero;
Text não-TextItem, Glyph, TextShaped, MathIdent e NumberItem preservam o lookup
métrico anterior. A fórmula, a posição e a extensão de `br` não mudaram.

Os REDs próprios e os oito vetores públicos B-P07 ficaram GREEN; o attach
inline fechou em `20.7614 × 9.7735pt`. Contudo, um controle público próprio
separado encontrou que `$#math.attach([x], br: [])$` ainda mede
`5.8080 × 7.5130pt` no candidato contra `6.4240 × 7.5130pt` no vanilla,
faltando exatamente `0.6160pt` de `SpaceAfterScript`. O mesmo source com
omissão ou `br:none` mede `5.8080pt` bilateralmente. O carrier público de
markup vazio chega, portanto, colidido com a sentinela eliminada pelo owner;
corrigir a distinção exige owner anterior fora da allowlist. STOP aplicado sem
nova hipótese ou edição. Este recibo não aprova B nem autoriza C/D.

## Autoridade, entradas e capacidades

- executor: `implementador_b_p1293`;
- regime: protocolo completo da skill `tekt-materializacao-segregada`, por
  capacidades e artefatos, sem atestação de isolamento técnico de leitura no
  filesystem compartilhado;
- selo ativo: SHA-256
  `863602b26cb9871d7ba2d84b040fe5de2dfa3d1b73ba701524c706e9cc21e9c8`;
- bloco canônico `$.serial_textitem_math_text_item_b`: SHA-256
  `4c1b3c1799c6e56efb8780538b49df0de191b33750b9d8dbaff76ba90615f7a7`;
- manifesto autorizado: SHA-256
  `9582de20437fb1b3a77e938061cf9be0901284508c49c42fa6bd303b02df9ce7`;
- recibo de reabertura: SHA-256
  `da2a6d840adfe30c297b6cf381838e56b48ac00c5998bfab1d19cbeed01854dd`;
- recibo causal público: SHA-256
  `123a7c5b58e04a8701cb50f0c9224dd4b63ddc61c6120759391db30456277386`;
- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- release candidato recompilado: SHA-256
  `172d54bec2f8fb43bc52b47619e6718c6a2139f50efce63a8d8a677a7ab09a46`.

L0s ressellados lidos integralmente e byte-idênticos ao selo:

| Owner | SHA-256 |
|---|---|
| `00_nucleo/prompts/entities/layout_types.md` | `c7243b67dc03e51cfe733d6f1bc455dc946c16bba5a27854cd42c802b37838d4` |
| `00_nucleo/prompts/compiler/math/layout/_comum.md` | `71c304857a76eb71fb7698be061bdd449e330410bf081e48d8ba21caa816b2b1` |
| `00_nucleo/prompts/infra/font_metrics.md` | `4fac5e047b87f83a7f06b3f59f062308d7d1f9d67fd51347e8e26aeb2c093190` |
| `00_nucleo/prompts/entities/style_chain.md` | `1efe881899ab5cdc290be6350e2da6f5e3ea8cbff60ab4e65e01585ec28cdf6c` |
| `00_nucleo/prompts/compiler/layout/text.md` | `9b1e1f4f5651b2bce61f17aab9c59bd6172c2bd86b316b1c536b93cd8bbaa42d` |

Não foram lidos nem executados o oracle protegido, o RED receipt privado, o
discrimination receipt privado ou outputs privados de testador, atacante ou
verificador. Não foram escritos contrato, manifesto, selo, L0, oracle,
ataques, veredito ou qualquer consumer fora da allowlist exata.

## RED próprio antes do comportamento candidato

O RED foi dividido para manter falhas próprias e discriminatórias mesmo com a
introdução de um campo público.

### Carrier/default

Antes do campo, um teste próprio verificou a ausência do eixo no default sem
referenciá-lo em compilação:

```text
cargo test -q -p typst-core \
  p1293_text_style_default_expoe_proveniencia_textitem_desligada -- --nocapture
RED: 0 passed / 1 failed / 1 executado
falha própria: Debug de TextStyle::default não continha math_text_item:false
falhas alheias: 0
```

Depois de adicionar o campo default-false e adaptar os dois literals
exaustivos autorizados, o mesmo teste ficou GREEN `1/1`.

### Propagação, IC e cache

Com o carrier compilável, os testes próprios foram escritos antes de mudar o
layouter e as métricas:

```text
cargo test -q -p typst-core p1293_ -- --nocapture
RED: 28 passed / 2 failed / 30 executados
falhas próprias: Content::Text singular e multiglifo ainda emitiam math=false
falhas alheias: 0

cargo test -q -p typst-infra p1293_ -- --nocapture
RED: 0 passed / 2 failed / 2 executados
falha própria 1: FontBookMetrics não suprimia IC para math_text_item
falha própria 2: AdvanceWidthKey colidia entre glifo e TextItem
falhas alheias: 0
```

Após a implementação:

```text
typst-core P1293: GREEN 30/30
typst-infra P1293: GREEN 2/2
```

O teste de métricas usa uma fonte MATH embutida real e um glifo com IC
não-zero; o delta esperado deriva de `char_italics_correction`, sem constante
de fixture. O teste de cache compara duas chaves idênticas salvo o novo bit.

## Implementação preservada

### `entities/layout_types.rs`

Adicionado somente `pub math_text_item: bool` a `TextStyle`. O derive
`Default` produz `false`, e `regular`, `bold` e `italic` continuam a herdar
esse default. Nenhum `StyleDelta`, parser, namespace de utilizador ou default
de linguagem foi criado.

### `compiler/math/layout/mod.rs`

O braço estrutural direto `Content::Text` clona o estilo matemático efetivo e
define somente `math_text_item=true`. A mesma cópia é usada por
`layout_text_node`, `text_width`, `MathBox` e `FrameItem::Text`. A forma
refutada `math=false` foi removida e não há subtração manual de IC.

`MathIdent`, `MathText`, números e containers permanecem
`math=true && math_text_item=false` nos testes próprios.

### `infra/font_metrics.rs`

`FontBookMetrics::advance` e `FallbackFontMetrics::advance` aplicam IC ao
glifo-base singular somente sob
`style.math && !style.math_text_item`. Resolução de primárias, fallback math,
ssty, variações, kerning, ink bounds e shaping permanecem inalterados.
`AdvanceWidthKey` ganhou somente o bit `math_text_item`.

### `entities/style_chain.rs` e `compiler/layout/text.rs`

O bridge genérico fixa `math_text_item=false`. O merge do layouter textual
herda `layouter.style.math_text_item`, sem sintetizar um valor e sem criar
canal configurável.

## Bilateral público após rebuild

Comandos por fonte:

```text
target/release/typst compile <fonte> <candidate.svg> --format svg
/usr/local/bin/typst compile <fonte> <vanilla.svg> --format svg
```

| Vetor | SHA-256 fonte | Vanilla viewBox | Candidato viewBox | Estado |
|---|---|---:|---:|---|
| attach display, 7 slots | `59667014f0dbc6e3a492de28ac43a4dbc2b098e8ff7320af4c59a3ec62be903d` | `23.2705x19.4843` | `23.2705x19.4843` | GREEN exato |
| attach inline qualificado, 7 slots | `237f5727e22f7947d56179870ed91e583bc4ac1497fffb741df88f8e4e37a95b` | `19.7681x9.6041` | `19.8682x9.6041` | RED largura `+0.1001pt` |
| binom display, 3 lower | `ed857c1e7628f470a4ca3370c833c5467b98f549ffe3ff06561d0f14763e977f` | `46.797666667x24.057` | `46.797666666666665x24.057` | GREEN |
| binom inline qualificado | `c71aa486168cb1aa6c476a49359c804a83bf5a8a37a37043f1bb60d5b50fa2f4` | `30.3325x7.8815` | `30.3325x7.8738` | GREEN, tolerância `0.01pt` |
| mono display | `8c988a15906e018938840ccd58abc41ba5724ad2eb4605c59e7ac69f04fea3d1` | `25.029888889x8.921` | `25.029888888888888x8.921` | GREEN |
| mono em script | `44d6243ec1eb10f3904db5c306818dac985f188addd7ea0e076004482073aad0` | `14.0987x6.2447` | `14.0987x6.2447` | GREEN exato |
| script cramped true | `686483da4c2789459a3aa589c9dad52696e266510266bf23514cb4460c32f27f` | `8.3578x5.9708` | `8.3578x5.9708` | GREEN exato |
| script cramped false | `ef200fecd812f4bc2b637caa15385dbbddf32b600d30b5b88f89381a7aaf8b86` | `8.3578x6.5406` | `8.3578x6.5406` | GREEN exato |

Hashes SVG do vetor bloqueante:

- vanilla: `c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c`;
- candidato: `430ce99154846323b74ce5a0e176a7191dd2b647fc315df2cad852f61c1eab9a`.

O novo bit é causalmente efetivo e preserva o eixo math, mas a hipótese de que
somente a IC explicava o residual foi refutada pelo `+0.1001pt`. Pelo selo,
isso permanece `Unknown=1` e bloqueia B; não autoriza investigação adicional.

## Gates deste serial

| Comando | Resultado |
|---|---|
| teste próprio do default, antes/depois | RED próprio `0/1` → GREEN `1/1` |
| `cargo test -q -p typst-core p1293_ -- --nocapture` | RED próprio `28/30` → GREEN `30/30` |
| `cargo test -q -p typst-infra p1293_ -- --nocapture` | RED próprio `0/2` → GREEN `2/2` |
| `cargo build --release -q -p typst-wiring` | PASS |
| 16 compilações bilaterais públicas | executadas; `7/8` GREEN |
| `git diff --check` restrito aos cinco consumers | PASS |
| P1105 / math / workspace regressions amplas | não executadas após o residual; STOP obrigatório |
| `cargo fmt --all -- --check` | não executado após o residual; STOP obrigatório |
| V5 / V15 / V26 | não executados após o residual; STOP obrigatório |
| hash dry-run | não executado após o residual; STOP obrigatório |

## Proveniência e hashes finais

- instante: `2026-09-01T21:50:59-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree: compartilhada e não commitada;
- arquivos produtivos escritos neste serial: exatamente os cinco consumers
  abaixo;
- outros arquivos produtivos escritos pelo executor: nenhum.

| Consumer | SHA-256 final antes deste recibo |
|---|---|
| `01_core/src/entities/layout_types.rs` | `38761a95d2b39d34868af3bb5bbfd58dd9404b902c2d242da93ebe6a94b7a1ed` |
| `01_core/src/compiler/math/layout/mod.rs` | `f612884fc3f39cdb509fa20e59d7d5f077be33dd49052b34aaf56e08a9d6c81c` |
| `03_infra/src/font_metrics.rs` | `67fd998c7ff584f19aecf65daed4c1b74a245221a3d2ce3905f4c3bc200de6ef` |
| `01_core/src/entities/style_chain.rs` | `02a1a8bb1016bf33a384f841026d70f0d7306ed30b4275204728c990bfacbd19` |
| `01_core/src/compiler/layout/text.rs` | `1b7d44fda64cb801c99cd6397669d6b7cfd9c37313f24f5c7ed4fa00deedf0e3` |

Arquivos produtivos congelados e byte-idênticos ao selo:

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/compiler/layout/helpers.rs` | `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116` |
| `01_core/src/compiler/math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |

## Serial TextItem-ssty — autoridade e RED→GREEN

Este bloco supersede o estado operacional anterior do recibo sem apagar sua
proveniência histórica.

- manifesto atual: SHA-256
  `108d4d241c6a571b93ed019b4cf550c099c44d9ef68a9240f7ffe86956e7e950`;
- selo ativo: SHA-256
  `8ed4c7287cc399ca8278238a691b1edbe557e5003ec191482a0566ad4062db90`;
- bloco `$.serial_textitem_ssty_b`: SHA-256 canônico
  `9eeba6e35a3897f10fdeee2844ccafcdac44fbf548caffdb4b2ac866d64cbec9`;
- L0 `infra/shaper.md`: SHA-256
  `fbeaa9aa47bfc6ba97f2f163ba84e8728395ad3c8121cc5cf776aad5869b5456`;
- L0 `infra/font_metrics.md`: SHA-256
  `405416e31c400ea478f80e678ff1bee304ec6c26867e1c43eb4f3b995dad6078`;
- recibo causal público: SHA-256
  `f727ddfbc4130e3f1941a67f0493d6ab69c3e52b130133e0b29a0197b772120d`;
- recibo de reabertura TextItem-ssty: SHA-256
  `cda00672388200351ab4cf5673dbb6732c3f25e71bac7b057c9eab514e47e96a`.

RED próprio, sem artefato protegido:

```text
cargo test -q -p typst-infra \
  p1293_textitem_ssty_fallback_exclui_somente_proveniencia_textual \
  -- --nocapture
RED: 0/1; ssty_level_of(TextItem Script)=Some(1), esperado None;
falhas alheias: 0

cargo test -q -p typst-infra \
  p1293_textitem_shaper_nao_solicita_ssty_e_preserva_controles \
  -- --nocapture
RED de compilação: E0425 somente no helper próprio ainda ausente;
4 chamadas discriminatórias; falhas alheias: 0
```

Implementação estreita:

- `shaper.rs`: `ssty_level_for_subrun` devolve `None` para TextItem e preserva
  `Script→1`, `ScriptScript→2`, elegibilidade e todos os demais eixos;
- `font_metrics.rs`: `ssty_level_of` ganhou somente o early return para
  `style.math_text_item`; IC, GSUB, ink, fallback, variações, kerning e cache
  permanecem inalterados.

GREEN e regressões executadas antes do STOP:

| Comando | Resultado |
|---|---|
| `cargo test -q -p typst-infra p1293_textitem -- --nocapture` | GREEN `2/2`, `914` filtrados |
| `cargo test -q -p typst-core p1293_ -- --nocapture` | GREEN `30/30`, `5366` filtrados |
| `cargo test -q -p typst-infra p1293_ -- --nocapture` | GREEN `4/4`, `912` filtrados |
| `cargo build --release -q -p typst-wiring` | PASS |

Release recompilado: SHA-256
`f0e5ce0810c7a7184f17b45e719c850fe30d14598d12f369b54fd060ddb3aed3`.

## Bilateral público do serial TextItem-ssty

Foram executadas as dezesseis compilações públicas:

```text
target/release/typst compile <fonte> <candidate.svg> --format svg
/usr/local/bin/typst compile <fonte> <vanilla.svg> --format svg
```

| Vetor | SHA-256 fonte | Vanilla viewBox | Candidato viewBox | Estado |
|---|---|---:|---:|---|
| attach display | `59667014f0dbc6e3a492de28ac43a4dbc2b098e8ff7320af4c59a3ec62be903d` | `23.2705x19.4843` | `23.2705x19.4843` | GREEN |
| attach inline | `237f5727e22f7947d56179870ed91e583bc4ac1497fffb741df88f8e4e37a95b` | `19.7681x9.6041` | `19.1521x9.6041` | RED `−0.6160pt` |
| binom display | `ed857c1e7628f470a4ca3370c833c5467b98f549ffe3ff06561d0f14763e977f` | `46.797666667x24.057` | `46.797666666666665x24.057` | GREEN |
| binom inline | `c71aa486168cb1aa6c476a49359c804a83bf5a8a37a37043f1bb60d5b50fa2f4` | `30.3325x7.8815` | `30.3325x7.8815` | GREEN |
| mono display | `8c988a15906e018938840ccd58abc41ba5724ad2eb4605c59e7ac69f04fea3d1` | `25.029888889x8.921` | `25.029888888888888x8.921` | GREEN |
| mono script | `44d6243ec1eb10f3904db5c306818dac985f188addd7ea0e076004482073aad0` | `14.0987x6.2447` | `14.0987x6.2447` | GREEN |
| cramped true | `686483da4c2789459a3aa589c9dad52696e266510266bf23514cb4460c32f27f` | `8.3578x5.9708` | `8.3578x5.9708` | GREEN |
| cramped false | `ef200fecd812f4bc2b637caa15385dbbddf32b600d30b5b88f89381a7aaf8b86` | `8.3578x6.5406` | `8.3578x6.5406` | GREEN |

SVG bloqueante: vanilla SHA-256
`c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c`;
candidato SHA-256
`78760639f48dc12c87929cf86bacc00403272745c18a58c56f5caf9e8459c03a`.

Resultado `7/8`, `Unknown=1`. Contraprobes K/W/RR, P1105, regressões amplas,
fmt, V5/V15/V26, dry-run e diff-check final não foram executados após o
residual, pois o selo exige STOP imediato. Nenhum desses gates é alegado PASS.

## Proveniência e hashes finais do serial

- início: `2026-09-01T22:32:52-03:00`;
- STOP/hashes: `2026-09-01T22:36:43-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada;
- `git diff HEAD --stat` inicial: `48 files changed, 3710 insertions(+),
  384 deletions(-)`; inclui materializações anteriores preservadas;
- escrita produtiva deste serial: exatamente os dois consumers abaixo;
- outra escrita: somente este recibo.

| Consumer | SHA-256 final |
|---|---|
| `03_infra/src/shaper.rs` | `1b7917b1efb442e1d21862d3117cd95196f640eabdede9ea83d9e78ca2c7074e` |
| `03_infra/src/font_metrics.rs` | `f18bff7c5637f0d94eb48c6d184090d04e0446ef9a71132f8dfef2a050fef990` |

Congelados byte-idênticos ao selo:

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/entities/layout_types.rs` | `782b5866d5c97beb41f1df4338fb62bf5f133fabfbf02181d1c6760c631ea4db` |
| `01_core/src/compiler/layout/helpers.rs` | `91a0bfddc52592427d9403d8cc5a6956b396c8e30c947c0a93bae035d19c8116` |
| `01_core/src/compiler/math/layout/attach.rs` | `4a3b6195cacbba955a70f7011b711bc2b39c2d025841344810ae26c96653e8f5` |

O produto preserva a correção estreita autorizada, mas B continua bloqueado.
O hash deste recibo deve ser calculado externamente após a gravação.

## Serial equation-frame/empty-markup — fechamento parcial

Este bloco supersede somente o estado operacional no topo do recibo e
preserva os seriais anteriores como histórico.

### Autoridade e isolamento

- manifesto atual `00_nucleo/diagnosticos/p1293-manifest.json`: SHA-256
  `bfaeda600914ff3537bbfa26889a4ce5fb3d1e3e892802d383161176a5105c17`;
- selo ativo `00_nucleo/diagnosticos/p1293-contract-seal.json`: SHA-256
  `c7d280e4ea8c9f5dd8342a5bfc92f9b782cc90aefff35dd3b2ba8df4350375f1`;
- bloco `$.serial_equation_frame_b`: SHA-256 canônico
  `ca7df58c739158f6fc327dbf93be87dcd38d884186766195bcdaf5f4675e36ca`;
- L0 `compiler/layout/equation.md`: SHA-256
  `9efca34585e1776267286b66dc74ed6ca213832b91568a8ffd79ccd2b8578179`;
- L0 `compiler/math/layout/attach.md`: SHA-256
  `fc189ea4f52436ca9c1adf52ba6767e3a2ff241211fb6edf4de9e4632cae6ff9`;
- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- release candidato recompilado: SHA-256
  `612ac055aca6d18b06a200c00d74080a602e90ad0e766667077e91d1d65ccf34`.

Foram usados apenas o manifesto, selo, L0s e probes públicos próprios. Não
foram lidos ou executados o contrato executável protegido, oracle, RED
receipt, discrimination receipt, expectativas ou outputs privados de
testador/verificador. Shaper, font metrics, `layout/mod.rs`, helpers,
`layout_types.rs`, contrato, L0s e lotes C/D permaneceram congelados neste
serial.

### RED próprio e implementação causal

Antes da implementação produtiva, os testes foram escritos apenas nos dois
owners autorizados.

```text
cargo test -q -p typst-core \
  p1293_formula_terminal_auto_preserva_extent_em_group_identidade \
  -- --nocapture
RED: 0/1; primeiro filho de Semantic::Formula era Text, não Group;
falhas alheias: 0; 5398 filtrados.

cargo test -q -p typst-core \
  p1293_markup_vazio_presente_recebe_um_space_after_script \
  -- --nocapture
RED: 0/1; quadrante 0 não reservava SpaceAfterScript;
falhas alheias: 0; 5398 filtrados.
```

Implementação estreita preservada:

- `equation.rs`: os itens já calculados da fórmula são localizados em relação
  a um `FrameItem::Group` identidade, sem clip e sem novo layout. A posição do
  grupo usa a origem lógica da equação; `inner_width=EquationExtent.width` e
  `inner_height=ascent+descent`. O envelope externo continua
  `SemanticKind::Formula`, e o cursor posterior continua baseado no mesmo
  extent;
- `attach.rs`: `visible_slot` elimina somente `Content::Empty`. A antiga
  filtragem por largura/itens materializados foi removida; assim, uma
  `Content::Sequence` vazia sintaticamente presente continua a gerar o espaço
  OpenType único do quadrante, enquanto omissão e `none` explícito não geram
  caixa nem spacing.

Controles próprios GREEN:

| Comando | Resultado |
|---|---|
| teste de Formula terminal isolado | GREEN `1/1` |
| teste de Formula + marcador/página finita isolado | GREEN `1/1`; marcador `Z` inicia no fim lógico do Group |
| teste de markup vazio isolado | GREEN `1/1`; quatro quadrantes |
| teste omissão versus `Content::Empty` isolado | GREEN `1/1`; largura e itens idênticos |
| `cargo test -q -p typst-core p1293_` | GREEN `33/33`, `5366` filtrados |
| `cargo test -q -p typst-infra p1293_` | GREEN `4/4`, `912` filtrados |
| `cargo build --release -q -p typst-wiring` | PASS |

O teste terminal percorre `R`, `K`, `W` e `RR`; todos produzem página finita,
Group identidade sem clip e extent interno positivo. O teste de página finita
preserva exatamente `123 × 80pt`. O controle `none` versus markup vazio não
usa constante de fixture: a diferença esperada deriva de
`MathConstants::space_after_script` no tamanho efetivo.

### Oito vetores públicos após rebuild

Comandos por fonte:

```text
/usr/local/bin/typst compile <fonte> <vanilla-equation-frame.svg>
target/release/typst compile <fonte> <candidate-equation-frame.svg>
```

| Vetor | SHA-256 fonte | Vanilla viewBox | Candidato viewBox | Estado |
|---|---|---:|---:|---|
| attach display | `e233f5519925a6b3e4fa45dcbba6e03e447864514331c52d804ec1a3ac762f15` | `22.1925×19.4843` | `22.1925×19.4843` | GREEN |
| attach inline | `6c135e979542187d7cb286ea29d00ef5c401710e890348707c276afe3609face` | `20.7614×9.7735` | `20.5854×9.7735` | RED largura `−0.1760pt` |
| binom display | `ed857c1e7628f470a4ca3370c833c5467b98f549ffe3ff06561d0f14763e977f` | `46.797666667×24.057` | `46.797666666666665×24.057` | GREEN |
| binom inline | `c71aa486168cb1aa6c476a49359c804a83bf5a8a37a37043f1bb60d5b50fa2f4` | `30.3325×7.8815` | `30.3325×7.881500000000001` | GREEN |
| mono display | `8c988a15906e018938840ccd58abc41ba5724ad2eb4605c59e7ac69f04fea3d1` | `25.029888889×8.921` | `25.029888888888888×8.921` | GREEN |
| mono script | `44d6243ec1eb10f3904db5c306818dac985f188addd7ea0e076004482073aad0` | `14.0987×6.2447` | `14.0987×6.2447` | GREEN |
| cramped true | `686483da4c2789459a3aa589c9dad52696e266510266bf23514cb4460c32f27f` | `8.3578×5.9708` | `8.3578×5.9708000000000006` | GREEN |
| cramped false | `ef200fecd812f4bc2b637caa15385dbbddf32b600d30b5b88f89381a7aaf8b86` | `8.3578×6.5406` | `8.3578×6.5405999999999995` | GREEN |

SVG do vetor bloqueante:

- candidato: SHA-256
  `c96002af51d5fe3c3ce626d4ef112eb7fb218236996b54b64c2c14222e5604fb`;
- vanilla: SHA-256
  `461397ee3dbd7b7f6239d6931326342c3f5009dcd642f36910a90f47bb896339`.

Resultado: `7/8`, com `Unknown=1`. O Group lógico recuperou parte do extent
terminal, mas não eliminou o residual público de `0.1760pt`; esta medição
refuta o fechamento por estes dois owners. Pelo selo, o STOP foi aplicado
imediatamente, sem novo probe causal nem tentativa em owner adicional.

### Gates e proveniência do STOP

| Gate | Resultado |
|---|---|
| `rustfmt --edition 2021` nos dois consumers | PASS |
| `git diff --check -- <dois consumers>` antes da medição | PASS |
| testes P1293 core/infra | PASS `33/33` e `4/4` |
| release rebuild | PASS |
| 16 compilações bilaterais públicas | executadas; `7/8` GREEN |
| contraprobes bilaterais K/W/RR, P1105 e regressões amplas | não executados após o residual; STOP obrigatório |
| `cargo fmt --all -- --check`, lint V5/V15/V26 e hash dry-run | não executados após o residual; STOP obrigatório |

- instante do STOP/hashes: `2026-09-01T23:18:01-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada;
- `git diff HEAD --stat` no instante: `50 files changed, 4164 insertions(+),
  402 deletions(-)`; inclui materializações anteriores preservadas;
- escrita produtiva deste serial: exclusivamente os dois consumers abaixo;
- outra escrita deste serial: somente este recibo.

| Consumer | SHA-256 final |
|---|---|
| `01_core/src/compiler/layout/equation.rs` | `96139ff7ba16826a1ea6db217ae2d0a68b4fd7e729a292d04db5f70a39f25aea` |
| `01_core/src/compiler/math/layout/attach.rs` | `1a660cf5b86860414d58654e945c35d1ce6b243585baa8ef3b0aa848b078a526` |

Estado final deste serial: `BLOCKED_EQUATION_FRAME_INLINE_RESIDUAL`. As duas
correções causais permanecem preservadas porque passaram seus RED→GREEN
próprios e reduziram o observável, mas B não está aprovado e C/D continuam
bloqueados. O hash deste recibo deve ser calculado externamente após a
gravação final.

## Serial attach-fragment-IC — fechamento parcial

Este bloco supersede somente o estado operacional no topo e preserva os
seriais anteriores como histórico.

### Autoridade e capacidade

- regime: protocolo completo da skill `tekt-materializacao-segregada`, papel
  implementador, sem alegação de isolamento técnico do filesystem
  compartilhado;
- manifesto atual `00_nucleo/diagnosticos/p1293-manifest.json`: SHA-256
  `d8cfaa974c7998098a3de4e08a32ccc453599282b5f90742887591ea9f71c1be`;
- selo ativo `00_nucleo/diagnosticos/p1293-contract-seal.json`: SHA-256
  `77dd70e005c0868988c707b0e941a00a21e4f358d7e7de8ece3d02e59486f213`;
- bloco `$.serial_attach_fragment_ic_b`: SHA-256 canônico
  `07dfc66df76ef68b4e040f2a69eb0c68b043bed259d75dfa44680a407c4387fd`;
- L0 `compiler/math/layout/attach.md`: SHA-256
  `24cb3d594438709e759d01cae6b0dc0c7c8162c95f81c78ed4cd094d2d1a0c83`;
- consumer inicial autorizado: SHA-256
  `7a8eb1c243fec8e67aa40ef384a0950c4096712963f8964d090444b01733a01e`;
- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- release candidato recompilado: SHA-256
  `66f1bfd5494d546d6788d31b3738bbe78e990fb197cfd9834e56d3ee6024862f`.

Foram lidos somente manifesto, selo, L0 e consumer autorizados, além dos
probes públicos próprios. Não foram lidos ou executados o contrato protegido,
oracle, RED receipt, discrimination receipt, expectativas ou outputs privados.
Equation, shaper, font metrics, layout/mod, helpers, layout types, L0, contrato
e lotes C/D permaneceram congelados.

### RED próprio e correção estreita

Os testes discriminatórios foram escritos antes da mudança produtiva:

```text
cargo test -q -p typst-core \
  p1293_textitem_br_usa_ic_semantica_zero_e_mathident_preserva_ic_metrica \
  -- --nocapture
RED: 0/1; TextItem x ainda subtraía a IC métrica;
falhas alheias: 0; 5400 filtrados.

cargo test -q -p typst-core \
  p1293_ic_semantica_nao_compensa_tr_ausencia_e_preserva_full_display \
  -- --nocapture
RED: 0/1; full attach TextItem x ainda subtraía a IC métrica;
falhas alheias: 0; 5400 filtrados.
```

Após a implementação, ambos ficaram GREEN `1/1`. O único delta produtivo
está no `find_map` já existente de `base_ic`: o braço
`FrameItem::Text { style: item_style, .. }` devolve `Some(0.0)` quando
`item_style.math_text_item`; caso contrário executa literalmente o lookup
`char_italics_correction` anterior. Os braços Glyph e TextShaped, o fallback
por `base_char`, `br_kern`, `br_post` e `x_br` não foram alterados. Não há
heurística de texto, caractere, fonte, largura, tinta ou fixture em produção.

Testes próprios sintéticos derivam as expectativas da IC fornecida pelo stub
de métrica e cobrem:

- TextItem `x/f/R` com IC semântica zero e controles `A/1` de IC métrica zero;
- MathIdent `x/f/R` preservando a IC métrica normal;
- ausência de `br`, somente `tr`, full attach inline e display;
- scripts `K/W/RR`;
- omissão, `Content::Empty` e carrier estrutural `Content::Sequence([])`.

| Comando | Resultado |
|---|---|
| dois testes discriminatórios isolados | RED próprio `0/1 + 0/1` → GREEN `1/1 + 1/1` |
| `cargo test -q -p typst-core p1293_` | GREEN `35/35`, `5366` filtrados |
| `cargo test -q -p typst-infra p1293_` | GREEN `4/4`, `912` filtrados |
| `cargo build --release -q -p typst-wiring` | PASS |

### B-P07 público após rebuild

Foram executadas dezesseis compilações públicas, sempre com a mesma fonte em
`/usr/local/bin/typst` e `target/release/typst`.

| Vetor | SHA-256 fonte | Vanilla viewBox | Candidato viewBox | Estado |
|---|---|---:|---:|---|
| attach display | `e233f5519925a6b3e4fa45dcbba6e03e447864514331c52d804ec1a3ac762f15` | `22.1925×19.4843` | `22.1925×19.4843` | GREEN |
| attach inline | `6c135e979542187d7cb286ea29d00ef5c401710e890348707c276afe3609face` | `20.7614×9.7735` | `20.7614×9.7735` | GREEN exato |
| binom display | `ed857c1e7628f470a4ca3370c833c5467b98f549ffe3ff06561d0f14763e977f` | `46.797666667×24.057` | `46.797666666666665×24.057` | GREEN |
| binom inline | `c71aa486168cb1aa6c476a49359c804a83bf5a8a37a37043f1bb60d5b50fa2f4` | `30.3325×7.8815` | `30.3325×7.881500000000001` | GREEN |
| mono display | `8c988a15906e018938840ccd58abc41ba5724ad2eb4605c59e7ac69f04fea3d1` | `25.029888889×8.921` | `25.029888888888888×8.921` | GREEN |
| mono script | `44d6243ec1eb10f3904db5c306818dac985f188addd7ea0e076004482073aad0` | `14.0987×6.2447` | `14.0987×6.2447` | GREEN |
| cramped true | `686483da4c2789459a3aa589c9dad52696e266510266bf23514cb4460c32f27f` | `8.3578×5.9708` | `8.3578×5.9708000000000006` | GREEN |
| cramped false | `ef200fecd812f4bc2b637caa15385dbbddf32b600d30b5b88f89381a7aaf8b86` | `8.3578×6.5406` | `8.3578×6.5405999999999995` | GREEN |

O SVG candidato do vetor attach inline tem SHA-256
`37725b5a0ff732226d5be112787e60df1e92ad6df4e60fa844fd41227030cb69`;
o vanilla tem SHA-256
`461397ee3dbd7b7f6239d6931326342c3f5009dcd642f36910a90f47bb896339`.

### Controle público próprio que bloqueou o serial

Uma matriz independente, escrita em `/tmp/p1293-fragment-ic-own`, confirmou
bilateralmente TextItem `x/f/R/A/1`, MathIdent `x/f/R`, ausência de `br`,
somente `tr`, full inline/display e `K/W/RR`. Exemplos:

| Controle | Vanilla viewBox | Candidato viewBox | Estado |
|---|---:|---:|---|
| TextItem x + br | `10.4896×7.5130` | `10.4896×7.5130` | GREEN |
| TextItem f + br | `8.0476×7.5130` | `8.0476×7.5130` | GREEN |
| TextItem R + br | `12.7776×7.5130` | `12.7776×7.5130` | GREEN |
| MathIdent x/f/R + br | `11.0583 / 10.1563 / 13.1153` | iguais | GREEN |
| sem br | `5.8080×7.5130` | `5.8080×7.5130` | GREEN |
| somente tr | `10.4896×7.5130` | `10.4896×7.5130` | GREEN |
| full inline | `20.7614×9.7735` | `20.7614×9.7735` | GREEN |
| full display | `20.7614×19.0267` | `20.7614×19.0267` | GREEN |
| br K/W/RR | `12.4146 / 14.3396 / 17.7584` | iguais | GREEN |
| `br:none` | `5.8080×7.5130` | `5.8080×7.5130` | GREEN |
| `br:[]` | `6.4240×7.5130` | `5.8080×7.5130` | **RED `−0.6160pt`** |

Fonte bloqueante `$#math.attach([x], br: [])$`: SHA-256
`e537feb7afed566779f6aebc4e1a46005c8afee85db553825b3cff164974c175`.
SVG candidato: SHA-256
`391ec89b6fd55f1aa1f323a07be3dff00a431bed2d35bbda91510e12145547ec`;
SVG vanilla: SHA-256
`4e1cf65687856187a0c06da0b9cc42857ca818dbac9c57af6631e60079ed028e`.

O delta é exatamente o `SpaceAfterScript` público já ressellado. O teste
unitário com `Content::Sequence([])` fica GREEN, mas a rota pública qualificada
produz o mesmo observável que omissão/`none` no candidato. Isso localiza uma
perda de proveniência antes de `layout_attach`; este owner não pode restaurar
a distinção sem heurística, payload ou owner anterior. O STOP ocorreu antes de
investigar ou editar esse owner.

### Gates, hashes e proveniência

| Gate | Resultado |
|---|---|
| `rustfmt --edition 2021 attach.rs` | PASS |
| `git diff --check -- attach.rs` antes dos bilaterais | PASS |
| P1293 core/infra | PASS `35/35` e `4/4` |
| release rebuild | PASS |
| B-P07 bilateral | PASS `8/8` |
| matriz pública própria | IC e controles `16/17` GREEN; `br:[]` RED |
| regressões amplas, cargo fmt global, V5/V15/V26, dry-run | não executados após o residual; STOP obrigatório |

- instante do STOP/hashes: `2026-09-01T23:47:30-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada;
- `git diff HEAD --stat`: `50 files changed, 4463 insertions(+), 405
  deletions(-)`; inclui materializações anteriores preservadas;
- escrita produtiva deste serial: somente
  `01_core/src/compiler/math/layout/attach.rs`;
- outra escrita: somente este recibo.

| Consumer | SHA-256 final |
|---|---|
| `01_core/src/compiler/math/layout/attach.rs` | `36a6699f507b3ddc4f1360c04bcc46c451dd42010f7e9051ed1536e738a7acd4` |

Estado final: `BLOCKED_EMPTY_MARKUP_CARRIER_COLLISION`. A correção de IC
semântica permanece preservada porque passou RED→GREEN próprio e fechou o
vetor público attach inline, mas o residual de markup vazio mantém
`Unknown=1`. B não está aprovado; C/D permanecem bloqueados. O hash deste
recibo deve ser calculado externamente após a gravação final.

## Checkpoint parcial — carrier triestatal e gap test-only não selado

Estado: `BLOCKED_TEST_ONLY_OPTION_READERS_OUTSIDE_ALLOWLIST`. Este checkpoint
substitui somente o estado operacional final acima; não constitui aprovação do
lote B nem autoriza C/D.

Proveniência: `2026-09-02T09:54:26-03:00`, HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch `Tekt`, working tree
compartilhada e não commitada. O SHA-256 de `git status --short` foi
`9c599fda46ef70a605f080a77a1bcf7d17b4bec72185b5b00c752ff6ee99549a`;
o SHA-256 de `git diff HEAD --stat` foi
`c917a7d26485c2ce769437907e0ba7fe37255e5db5b336dee870c87d379f5c3c`.

Sob o replacement seal SHA-256
`9f393909528966547b79f3e682669a0a2f581286b8dbaf16812b6672fe73877f`,
bloco canônico `serial_empty_markup_carrier_test_only_b` SHA-256
`d277cefceffdd051e0bfd8be204994ac816283e632ad61809fe7de49cab3d88a`,
foram materializados os oito consumers produtivos do carrier fechado
`MathAttachSlot::{Omitted, ExplicitNone, Present(Content)}`. Não foi criado
adaptador legado, sentinel, heurística, `as_ref`, `is_none` ou outro método de
compatibilidade no enum.

Os 45 callsites test-only inventariados e autorizados foram migrados
mecanicamente na cardinalidade selada `1 + 1 + 41 + 2`:

- `compiler/layout/equation.rs`: 1;
- `compiler/math/layout/spacing.rs`: 1;
- `compiler/math/layout/tests.rs`: 41;
- `infra/export/tests.rs`: 2.

Nesses quatro escopos, somente imports e argumentos de constructor foram
alterados de `None`/`Some(content)` para `Omitted`/`Present(content)`; nenhum
assert, valor esperado ou fixture foi alterado.

### RED→checkpoint

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core p1293_carrier_distingue_omissao_none_e_markup_vazio_no_hash --lib` antes do produto | RED, exit `101`, `MathAttachSlot` ausente (`E0412`/`E0433`) |
| `cargo check -p typst-core` após os oito consumers | GREEN, exit `0` |
| `cargo test -p typst-core --lib --no-run` | bloqueado exclusivamente por 25 leituras que ainda tratam os campos como `Option` em `01_core/src/compiler/eval/tests.rs`, arquivo fora da allowlist |

As 25 ocorrências bloqueantes são:

- linhas `9334–9339`: 6 chamadas `as_ref`;
- linhas `9354–9357`: 4 chamadas `as_ref`;
- linha `9898`: 1 padrão `if let Some` sobre `&MathAttachSlot`;
- linhas `16671–16674`: 4 leituras `as_ref`/`is_none`;
- linhas `16692–16695`: 4 chamadas `as_ref`;
- linhas `16715–16720`: 6 chamadas `as_ref`.

`01_core/src/compiler/eval/tests.rs` não foi editado. A compilação parou antes
de suites funcionais, release, bilaterais e gates finais, conforme a condição
de STOP do selo. Nenhuma falha funcional foi mascarada e nenhum artefato
protegido foi lido ou executado.

### Hashes do checkpoint

| Arquivo | SHA-256 |
|---|---|
| `01_core/src/entities/elements/math_attach.rs` | `60e38cb641bada40339788337fc5cd3e70f6adf3ff56d184e2f45d1168aa2bb6` |
| `01_core/src/entities/content.rs` | `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `657b33efd5a08c5bc845d9190cfdfa5fc62442349894336fc377b19d3aa85bec` |
| `01_core/src/compiler/eval/math.rs` | `16d0defc53929da55f7f45dfdcea0a068d8b59152051922b47ec1c166b1e4c18` |
| `01_core/src/compiler/eval/repr.rs` | `9c5316ecaa9332379bad5afa1c1e8de1e504e77d085c91b475480591b30a3d1b` |
| `01_core/src/compiler/math/layout/mod.rs` | `1c386e48028c85d1a93ab35829b910a9f03d1b8e12155bae4c14efadf75492a4` |
| `01_core/src/compiler/math/layout/attach.rs` | `73c77f10fcd663fbd07f5a102458a5a226b92fdd99b4c9911ed0173583558be3` |
| `02_shell/src/cli.rs` | `4f9671dc6cbb0ac3e718503fd16eb8b9b6d3d072a61190856b75781806b572c5` |
| `01_core/src/compiler/layout/equation.rs` | `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23` |
| `01_core/src/compiler/math/layout/spacing.rs` | `3a525ea69566d52d9345d4242bfdaf4e439f66d6d8fed2e151c24f449f0a00d8` |
| `01_core/src/compiler/math/layout/tests.rs` | `0a19d473687fdab69883d99c982c4f40492949f4f1b69337705346f6d680e95f` |
| `03_infra/src/export/tests.rs` | `d09d31af8829351fa014db279fe735f9a79dfe6a2816d02a1f9a8d427bcf3387` |

O hash deste recibo é calculado externamente após esta gravação.

## Checkpoint — dois hunks próprios do carrier

Estado: `OWN_TEST_HUNKS_GREEN_AWAITING_INDEPENDENT_JUDGMENT`. Este registro
fecha exclusivamente o serial de dois hunks de teste; não aprova o lote B e
não autoriza C/D.

Regime A/B segregado, sem alegação de atestação técnica de isolamento. O
implementador recebeu somente o manifesto, o replacement seal e os dois
hunks classificados; não leu nem executou contrato, oracle, RED,
discriminação ou saídas privadas protegidas.

O implementador também não executou `typst-wiring` nem qualquer verificação
protegida. Após os resultados unprotected abaixo, o coordenador comunicou uma
falha `E0308` em `02_shell/src/cli.rs`, observada por sua própria verificação:
a closure `first_supplied` foi inferida incorretamente com
`&MathAttachSlot`. Como a productive allowlist deste selo é vazia, `cli.rs`
não foi lido, editado nem corrigido neste serial; esse residual bloqueia a
continuação fora desta autoridade.

Autoridade conferida antes da escrita:

- manifesto atual SHA-256
  `ea5a1942ef8d5d7f19b07a25b7322fcd6b320021180a0fa92d5e18d078cff5e3`;
- seal file SHA-256
  `6e517d96b29124d2774048328126248d7a94644fa956523e9728e4c812915cbb`;
- bloco canônico `serial_empty_markup_carrier_own_test_hunks_b` SHA-256
  `504d25e8b17e5ffccfb3a0b0a6989bb24c4c611e2ef229715bbd0061d69465a7`.

Foram aplicadas estritamente as duas mudanças autorizadas, sem corpo
produtivo:

1. `p1293_omissao_e_none_explicito_continuam_sem_spacing` passou a chamar
   `layout_attach_slots`, comparando `ExplicitNone` no slot `tr` contra seis
   slots `Omitted`; as duas asserções e seus significados permaneceram
   intactos. O caminho `super::MathAttachSlot` foi usado dentro do mesmo hunk
   para resolver o nome no submódulo `smoke`, sem criar import/terceiro hunk.
2. `p1293_plain_text_e_traversal_preservam_carrier` alterou somente a string
   esperada de `^tlx^_b^_br` para `^tlx_b^_br`, mantendo inputs, percurso e
   demais asserts intactos.

### Comandos e resultados

| Comando | Resultado |
|---|---|
| primeira compilação isolada do teste `p1293_omissao_e_none_explicito_continuam_sem_spacing` | RED técnico `E0433`: nome `MathAttachSlot` não visível no submódulo; resolvido no mesmo hunk com `super::MathAttachSlot` |
| `cargo test -p typst-core p1293_omissao_e_none_explicito_continuam_sem_spacing --lib` | GREEN `1/1` |
| `cargo test -p typst-core p1293_plain_text_e_traversal_preservam_carrier --lib` | GREEN `1/1` |
| `cargo test -p typst-core p1293_carrier_distingue_omissao_none_e_markup_vazio_no_hash --lib` | GREEN `1/1` |
| `cargo test -p typst-core p1293_ --lib` | GREEN `38/38`, `5366` filtrados |
| `cargo test -p typst-core --lib --no-run` | GREEN, exit `0` |
| `cargo test -p typst-core p1105_ --lib` | GREEN `6/6`, `5398` filtrados |
| `cargo test -p typst-infra p1293_` | GREEN `4/4`, `912` filtrados; integração filtrada `0/1` |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | GREEN, `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | GREEN, `Nothing to fix` |
| SHA bruto do núcleo + busca do pin efetivo | raw `29a7ec096d899850eb602afa0cbe0b44d46147214b06296f2e90c40776e3cb7b`; pin efetivo `81b492ca...` presente em `8/8` prompts consumidores |
| `git diff --check -- attach.rs math_attach.rs receipt` | GREEN |

`rustfmt --edition 2021 --check` passou para `attach.rs`, mas o comando com
os dois arquivos retornou exit `1` por uma linha longa já herdada em
`math_attach.rs:200`, fora dos dois hunks autorizados. Essa linha e seu assert
não foram alterados: corrigi-la constituiria terceiro hunk proibido. Os dois
hunks novos não introduzem whitespace error e o diff-check focal é GREEN.

### Hashes e proveniência

- instante: `2026-09-02T11:11:30-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada: `55 files changed, 5245
  insertions(+), 687 deletions(-)` em `git diff HEAD --stat`;
- arquivos de teste escritos neste serial: exatamente os dois abaixo;
- artefato adicional escrito: somente este recibo.

| Arquivo | SHA-256 após os hunks |
|---|---|
| `01_core/src/compiler/math/layout/attach.rs` | `34766e5355e737187716891cf25142aadb5f23311a6919351fe79ced2471b1da` |
| `01_core/src/entities/elements/math_attach.rs` | `d9b45f7534f6ec3fd363594c6316077cb3a41917fa99125141aec02d80f31b09` |

Resultado restrito: os dois resíduos próprios classificados foram removidos
sem mudança de produto, e o filtro P1293 passou de `36/38` para `38/38`.
Qualquer veredito ou aprovação do lote B permanece reservado à autoridade
independente. O hash deste recibo deve ser calculado externamente após esta
gravação final.

## Checkpoint parcial — readers do carrier em `eval/tests.rs`

Estado: `BLOCKED_OWN_TEST_RESIDUAL_OUTSIDE_READER_ALLOWLIST`. Este
checkpoint substitui somente o estado operacional do checkpoint anterior;
não aprova o lote B e não autoriza C/D.

Autoridade validada antes da escrita:

- manifesto SHA-256
  `aac2fdd004d5e7a8ab785fc23e452f3329d0c3aa7aa5d9b3af7e53f82daf1bb1`;
- selo SHA-256
  `a6ce0c756b683e1811dd6da08d4ff99efa199224043b2aa00ed8b5d3d473b4c5`;
- bloco canônico `serial_empty_markup_carrier_eval_tests_readers_b`
  SHA-256
  `3eb9959fe07ba307661bd6e89a3a8b0cb32ec3b227c79fc641e3c0a97be3daab`;
- preimage de `01_core/src/compiler/eval/tests.rs` SHA-256
  `829b49bb` (prefixo selado), confirmado antes da migração.

Foram migradas somente as 25 leituras seladas em
`01_core/src/compiler/eval/tests.rs`, mediante `match`/`matches!` explícito
sobre `MathAttachSlot`. A cardinalidade permaneceu `6 + 4 + 1 + 4 + 4 + 6`:

- 6 projeções no visitante de `MathOp`;
- 4 projeções no visitante de `MathIdent`;
- 1 projeção no percurso P958;
- 4 leituras no controle P1105 básico;
- 4 leituras no controle P1105 de quatro cantos;
- 6 leituras no controle P1105 de seis slots.

Nenhuma expectativa, fixture ou valor esperado foi mudado por este serial.
As alterações de header e das duas expectativas textuais P1105 já estavam na
preimage selada e foram preservadas. Não foi adicionado `as_ref`, `is_none` ou
outro método de compatibilidade a `MathAttachSlot`. Os oito consumers
produtivos do carrier e as 45 migrações test-only anteriores foram
preservados sem nova edição neste serial.

### Compilação e STOP discriminatório

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core --lib --no-run` | GREEN, exit `0`; o gap anterior das 25 leituras está fechado |
| `rustfmt --edition 2021 01_core/src/compiler/eval/tests.rs` | PASS |
| `cargo test -p typst-core p1293_carrier_distingue_omissao_none_e_markup_vazio_no_hash --lib` | GREEN `1/1` |
| `cargo test -p typst-core p1293_ --lib` | STOP: `36/38` GREEN, `2` falhas, `5366` filtrados |

As duas falhas próprias que acionaram o STOP são exteriores à allowlist de
readers deste selo:

1. `compiler::math::layout::attach::smoke::p1293_omissao_e_none_explicito_continuam_sem_spacing`, em
   `01_core/src/compiler/math/layout/attach.rs:757`: largura observada
   `7.871999999999999`, esperada `7.199999999999999`;
2. `entities::elements::math_attach::tests::p1293_plain_text_e_traversal_preservam_carrier`, em
   `01_core/src/entities/elements/math_attach.rs:189`: texto observado
   `^tlx_b^_br`, esperado `^tlx^_b^_br`.

Por isso não foram executados P1105 funcional, suites infra/shell/workspace,
release rebuild, bilaterais públicos, `crystalline-lint`, V5/V15/V26 nem
diff-check conclusivo. O primeiro `cargo fmt --all -- --check`, anterior ao
STOP, também não fechou: encontrou formatação pendente em múltiplos arquivos
da working tree compartilhada, incluindo a forma pré-rustfmt dos matches em
`eval/tests.rs`; somente o arquivo reader autorizado foi formatado. Nenhum
owner fora da allowlist foi editado e nenhum artefato protegido foi lido ou
executado.

### Hashes e proveniência do checkpoint

- instante: `2026-09-02T10:30:38-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada, não commitada: `55 files changed, 5238
  insertions(+), 687 deletions(-)` em `git diff HEAD --stat`;
- escrita deste serial: somente
  `01_core/src/compiler/eval/tests.rs` e este recibo.

| Arquivo | SHA-256 no STOP |
|---|---|
| `01_core/src/entities/elements/math_attach.rs` | `60e38cb641bada40339788337fc5cd3e70f6adf3ff56d184e2f45d1168aa2bb6` |
| `01_core/src/entities/content.rs` | `34838354261fa472c5efbc73c39ee6587b8b77be4d4606ca7587537818ed4a98` |
| `01_core/src/compiler/stdlib/structural/math.rs` | `657b33efd5a08c5bc845d9190cfdfa5fc62442349894336fc377b19d3aa85bec` |
| `01_core/src/compiler/eval/math.rs` | `16d0defc53929da55f7f45dfdcea0a068d8b59152051922b47ec1c166b1e4c18` |
| `01_core/src/compiler/eval/repr.rs` | `9c5316ecaa9332379bad5afa1c1e8de1e504e77d085c91b475480591b30a3d1b` |
| `01_core/src/compiler/math/layout/mod.rs` | `1c386e48028c85d1a93ab35829b910a9f03d1b8e12155bae4c14efadf75492a4` |
| `01_core/src/compiler/math/layout/attach.rs` | `73c77f10fcd663fbd07f5a102458a5a226b92fdd99b4c9911ed0173583558be3` |
| `02_shell/src/cli.rs` | `4f9671dc6cbb0ac3e718503fd16eb8b9b6d3d072a61190856b75781806b572c5` |
| `01_core/src/compiler/layout/equation.rs` | `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23` |
| `01_core/src/compiler/math/layout/spacing.rs` | `3a525ea69566d52d9345d4242bfdaf4e439f66d6d8fed2e151c24f449f0a00d8` |
| `01_core/src/compiler/math/layout/tests.rs` | `0a19d473687fdab69883d99c982c4f40492949f4f1b69337705346f6d680e95f` |
| `03_infra/src/export/tests.rs` | `d09d31af8829351fa014db279fe735f9a79dfe6a2816d02a1f9a8d427bcf3387` |
| `01_core/src/compiler/eval/tests.rs` | `b6b87d6786874a39b3071c667476d062a4fc41a9298c6519d9434e0ae637df20` |

O hash deste recibo é calculado externamente após esta gravação.

## Checkpoint — reparo local do empréstimo CLI

Estado: `CLI_BORROW_GREEN_AWAITING_INDEPENDENT_JUDGMENT`. Este checkpoint
substitui apenas o bloqueio `E0308` anterior; não aprova B nem autoriza C/D.

Regime A/B segregado, executado sem atestação técnica de isolamento. O
implementador não leu nem executou contrato, oracle, RED, discriminação ou
saídas privadas protegidas.

Autoridade conferida antes da escrita:

- manifesto SHA-256
  `b68c6402f825bce9276ff4823e3ca50527125536ad28fc6b9b19a8df376ea9a1`;
- seal file SHA-256
  `878c5a735783f25642e74a825607de6c3453d2c78778e80aa80a03b480fe4a8d`;
- bloco canônico `serial_cli_borrow_b` SHA-256
  `0250c8329b01e0c0a6e9f1c5cb3172020783d57d626a53c2480c5cf910f41c07`.

Em `02_shell/src/cli.rs`, somente a closure local `first_supplied` foi
substituída por um helper local privado com lifetime comum
`fn first_supplied<'a>(primary: &'a MathAttachSlot, fallback: &'a
MathAttachSlot) -> &'a MathAttachSlot`. Corpo e precedência foram preservados:
`Omitted` devolve fallback; `ExplicitNone` e `Present(Content)` devolvem
primary. Não houve clone, move, alocação, sentinel, mudança de serialização
ou outra alteração no arquivo.

### Comandos e resultados

| Comando | Resultado |
|---|---|
| `cargo check -p typst-wiring` | GREEN, exit `0`; os cinco `E0308` classificados não reapareceram |
| `cargo test -p typst-core p1293_ --lib` | GREEN `38/38`, `5366` filtrados |
| `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia --lib` | GREEN `1/1`, `60` filtrados; controle pertinente de serialização `MathAttach` |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | GREEN, `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | GREEN, `Nothing to fix` |
| núcleo/pins | raw `29a7ec096d899850eb602afa0cbe0b44d46147214b06296f2e90c40776e3cb7b`; pin efetivo `81b492ca...` em `8/8` consumers |
| `rustfmt --edition 2021 --check 02_shell/src/cli.rs` | GREEN |
| `git diff --check -- 02_shell/src/cli.rs receipt` | GREEN |

### Hash e proveniência

- instante: `2026-09-02T11:36:08-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada: `55 files changed, 5250
  insertions(+), 687 deletions(-)` em `git diff HEAD --stat`;
- escrita produtiva deste serial: somente `02_shell/src/cli.rs`;
- artefato adicional escrito: somente este recibo;
- SHA-256 final de `02_shell/src/cli.rs`:
  `b0cbc485dd56b8f0ecb72938ef42482f576fecf2ef904ded9d4c7744acda501a`.

Resultado restrito: o reparo de lifetime está GREEN nos gates unprotected
autorizados. Aprovação do lote B permanece reservada à autoridade
independente. O hash deste recibo deve ser calculado externamente após esta
gravação final.
