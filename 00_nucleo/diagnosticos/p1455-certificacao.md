# P1455 — certificado de remoção da paleta string hardcoded

## Identidade

- Proposta confirmada: `e5505f17ceb342b94424c2d1e5627cc523eb469a`.
- Produto avaliado: `4d0f521441bc3e34cf4664a4bd07d95cefe38451`.
- Árvore avaliada: `a7e838265a9c7e103b7cf0ad46cf708455c6e15b`.
- Captura: `2026-09-19T00:58:07Z`.
- Regime: executado sem atestação de isolamento.

## Integridade do material

| Artefacto | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/shapes.rs` | `8231e1e0271131f04a99dc7de7c664380e600ef1e97af48362e2c13d474e69a2` |
| `01_core/src/compiler/stdlib/text/deco.rs` | `bfe2bad04acee4bd6a5bb12c5aab82e0fe2247e4b19befc513224c533a148b97` |
| `01_core/src/compiler/stdlib/mod.rs` | `d0640072de23c2811fc6e373c9c80c77b3628bc8c7ca267b11f9235aaedb031d` |
| `00_nucleo/prompts/compiler/stdlib/shapes.md` | `bd0aaae5d743f1890547c6e8bd8fa24aa03b792f49ab5aaa32ffa983d5918f0f` |
| `00_nucleo/prompts/compiler/stdlib/text/deco.md` | `6238284029f77370cfd7757eb27c99657f6fd337178a7a689d69c0f4591aa5f5` |
| `00_nucleo/prompts/compiler/stdlib/_comum.md` | `61255791dbf880217985fb2a8b42cdb2bdea9a84850be3998e7e7e81302f8e3f` |

## Evidência RED → GREEN

Antes do candidato, o teste
`p1455_parse_color_rejeita_catalogo_string_hardcoded` falhou em `"red"`:
o baseline devolvia `Some(red)` onde o contrato exigia `None`. No produto, os
dois testes P1455 passam e cobrem todos os 19 spellings removidos, além do
controle `Value::Color` tipado.

A campanha CLI congelada em `p1455-oracles.json` executou quatro negativos e
seis positivos em cada um dos dois binários, totalizando **20 verificações** e
**zero falhas**. Vanilla e cristalino rejeitaram strings com diagnóstico que
contém `expected` e `found string`; ambos aceitaram cor tipada, omissão e
`none`. Binários medidos:

- vanilla: `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino: `cfbfd03361331ee422c97007d4cf222e265d5922c0a86c2c16ba711b6f541ecc`.

## Ataques congelados

Os 12 ataques de `p1455-mutants.json` foram confrontados com os predicados
congelados, nas ordens forward, reverse e forward-repeat:

| Família | Mutantes mortos | Evidência |
|---|---:|---|
| retenção de catálogo/aliases | 4/4 | teste unitário dos 19 spellings + quatro negativos CLI |
| aceitação silenciosa em fill/stroke | 2/2 | negativos de rect/line/deco exigem erro tipado |
| regressão de controles tipados/default/none | 3/3 | seis positivos em ambos os binários |
| timeout/resultado opaco/binário único | 3/3 | estado exige saída observável nos dois executáveis e repetição estável |

Mutation score semântico: **12/12 = 1,0**. Nenhum `Unknown` foi promovido a
`Passed`.

## Gates

- `cargo test -q -p typst-core p1455_parse_color -- --nocapture`: passou, 2/2.
- testes focais de shapes: passaram, 7/7.
- testes de rect, polygon, curve e highlight diretamente afetados: passaram.
- campanha pública vanilla × cristalino: passou, 20/20.
- `cargo build --release --bin typst`: passou.
- `cargo build`: passou.
- `cargo fmt --all -- --check`: passou.
- `crystalline-lint .`: zero violations; avisos informativos históricos permanecem.
- `git diff --check`: passou.
- auditoria de catálogo nas camadas produtivas: nenhum match de nome de cor
  permanece; a única ocorrência `"black" =>` é `font_book.rs`, que classifica
  peso de fonte e não paint.

`cargo test --workspace` não ficou verde: 5.911/5.919 passaram e oito falhas
fora do recorte permaneceram em repr matemático, delimitadores `lr`, align e
contagens HTML. O certificado não converte esse resultado em aprovação; a
aceitação P1455 apoia-se nos gates focais, campanha diferencial e lint, todos
verdes.

## Veredito

**P1455 conforme no recorte declarado.** O hardcode produtivo de paleta string
foi eliminado, strings deixam de ser promovidas a paint e os valores Typst já
tipados preservam o comportamento. A limitação explícita é o gate global não
verde pelas oito falhas não relacionadas acima.
