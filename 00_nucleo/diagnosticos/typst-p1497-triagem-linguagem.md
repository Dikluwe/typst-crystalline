# P1497 — triagem bilateral das observações inconclusivas

## Medições e proveniência, antes da classificação

O alvo vanilla continua o commit `9f2b6e8715237cb086899a42873660fe744622e8`
ratificado em P1489, não `/usr/local/bin/typst` nem o `typst` da raiz.
O HEAD cristalino das medições é
`e0ee8be5e2b2eeb5da983bbbb36e4bb99d6c493e` (P1496). Nenhum código
produtivo L1–L4 foi alterado em P1497. As medições ocorreram em working
tree não commitado: no inventário, o único arquivo modificado era
`lab/surface-inventory/Cargo.lock` (`git diff HEAD --stat`: 1 arquivo,
3 inserções, 2 remoções); a sonda de linguagem registra em seu próprio
JSON a lista completa de arquivos e o horário UTC antes/depois. A matriz
tem snapshot independente abaixo. Esses três snapshots não devem ser
fundidos em um estado implícito.

| Produto | SHA-256 medido |
|---|---|
| vanilla `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| extrator vanilla `lab/typst-original/target/release/p1140-inventory` | `216c89f75bef84d5f6276ce20cd4dbabd01c6329cdcf7f56fdbc1eef3384e7b8` |
| cristalino `target/debug/typst` após `CARGO_BUILD_JOBS=1 cargo build --quiet` | `016805567c7037787664ab3c0682037697a39b7004958b1bb74b492b9156014c` |
| cristalino `target/release/typst` | `2620ffb09b9f1ed2f51bdb412cd883cc1bf944c5f0a6a7433926eee7146412e3` |
| extrator cristalino reconstruído `lab/surface-inventory/target/release/surface-inventory-p1140` | `a9f043e391fcab59bdd6efd9015a00762f752bb7da4efed494ea074229c4c173` |

O extrator cristalino **anterior** tinha SHA
`449dd2cb1757ab114eb70d7eba2525a146e89a2e1044cbfb0089e062df33fb36`
e devolvia artificialmente as contagens antigas (6 `MISSING_MEMBER` e
24 `VERIFIED_METADATA_DIFFERENCE` no default), apesar de
`target/debug/typst eval 'repr(sym.chess)' --format json` já aceitar
`sym.chess`. Era um produto compilado antes de P1496, não regressão de
linguagem. `CARGO_BUILD_JOBS=1 cargo build --release --offline
--manifest-path lab/surface-inventory/Cargo.toml --quiet` reconstruiu o
extrator contra o catálogo atual e atualizou **somente** seu `Cargo.lock`
para o pin `codex` git `2e43ab90079cf9f9fa670d421c8c38233143b376`.
O SHA do lockfile resultante é
`a0f392ea9ea324c654327b226c773804551987921f95b1f0338abe85d6db9847`.
Confundir binário desatualizado com lacuna produziria falsos RED.

## Inventário recompilado: seis perfis

Medição concluída em 2026-09-25 02:40:31 UTC, com HEAD, worktree e
hashes acima. A receita de P1489 (`p1140-inventory`,
`surface-inventory-p1140`, `merge.py`) foi repetida para cada perfil,
com `lab/surface-inventory/extra_seeds.json` e o binário cristalino debug
atual. Os JSONs intermediários estão em `/tmp/p1497-inventory.dzMeyc/`;
os hashes dos merges finais tornam a saída identificável mesmo se `/tmp`
for limpo.

Receita para `default` (repetir os três comandos para os outros cinco
perfis, trocando `+` por `-` nos nomes de arquivo):

~~~sh
lab/typst-original/target/release/p1140-inventory \
  /tmp/p1497-inventory.dzMeyc/vanilla-default.json default \
  lab/typst-original/target/release/typst
lab/surface-inventory/target/release/surface-inventory-p1140 \
  /tmp/p1497-inventory.dzMeyc/crystal-default-current.json \
  /tmp/p1497-inventory.dzMeyc/vanilla-default.json default \
  lab/surface-inventory/extra_seeds.json target/debug/typst
python3 lab/surface-inventory/merge.py \
  /tmp/p1497-inventory.dzMeyc/vanilla-default.json \
  /tmp/p1497-inventory.dzMeyc/crystal-default-current.json \
  /tmp/p1497-inventory.dzMeyc/merged-default-current.json --profile default
~~~

| Perfil | MATCH | MISSING_MEMBER | UNVERIFIED_METADATA | VERIFIED_METADATA_DIFFERENCE | EXTRA_BINDING | SHA-256 do merge |
|---|---:|---:|---:|---:|---:|---|
| default | 1598 | 0 | 469 | 0 | 49 | `ccee7d8acdd4316107f3eacbb19bf34d360d17a789daab1decdc327e97d779f3` |
| html | 1598 | 0 | 699 | 0 | 50 | `e8d7761be079d958fdfc91ba74a44c4953e39b09733e409fff35e04a55cc3c30` |
| a11y-extras | 1598 | 0 | 475 | 0 | 49 | `c11d17e6bb196a97c8a65fa8800d4d46252fcb5b8b5fa685765307fc500025b0` |
| html+a11y | 1598 | 0 | 705 | 0 | 50 | `39142c19d0d934cdecb6354af7a663156422d62289500a755818ce56dc53a612` |
| bundle | 1598 | 0 | 472 | 0 | 49 | `fc9a0ad66b4fda00533a489597292e708462c152c69247ee7a1621e7567b2120` |
| html+bundle | 1598 | 0 | 702 | 0 | 50 | `0ee3f990e576df952af46d13d5a5d1261da5cabaf48ed762ae58d99bc786fd49` |

`MISSING_BINDING`, `WRONG_KIND` e `UNKNOWN` são zero em todos os perfis.
P1496 fechou os seis pais ausentes, os 42 descendentes bloqueados e as
24 diferenças verificadas (12 famílias espelhadas em `sym`/`math`), sem
converter os 469 metadados não verificados em paridade. Nos extras do
default, seis rotas `format.*` duplicam caminhos/defaults; restam 43
rotas normalizadas. HTML acrescenta `format.html.pretty`, também uma
duplicação, e mantém 43. `sym.join`/`math.join` e
`sym.registered`/`math.registered` são pares espelhados, não quatro
decisões independentes. A deduplicação por membro, feature e perfil
produz 41 famílias de decisão no default, **não 41 defeitos**.
Os aliases legados de `join` permanecem extensão deliberada P1495;
nenhum extra autoriza remoção ou crédito de paridade por mera presença.

## Sondas de linguagem e limites do observável

O executor `lab/parity/tools/p1497_language_probes.py` fixa SHA do
oráculo, binários, HEAD, `git status --short --untracked-files=all`,
`git diff HEAD --stat`, horário UTC e ambiente relevante. Cada caso
registra target e feature. Executar na raiz:

~~~sh
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/tools/p1497_language_probes.py
~~~

Exit 1 significa RED observado, não falha do executor; exit 2 é
instrumentação inconclusiva. Um controle bilateral só comprova o
observável nomeado; igualdade de mensagens, bytes ou implementação
não é inferida de controles de aceitação. As quatro linhas `VIOLATED`
resolvem-se em **três causas** — duas sondas de round-trip de `stroke`
medem o mesmo contrato. O JSON da execução final está em
`/tmp/p1497-language-final.rZ99ba.json` (SHA-256
`726ef7c8ca5ad96d26e54fc55e9b33618c77d3733e35de602397eb89dc7eb595`),
com execução de 2026-09-25 02:54:56 a 02:55:00 UTC: 5 `MATCH`, 4
`VIOLATED`, 3 `DISABLED_BY_PROFILE`, sem erro de instrumentação. O
snapshot antes/depois foi idêntico: HEAD acima, `Cargo.lock` modificado
(3 inserções/2 remoções) e seis arquivos ainda não rastreados
(este diagnóstico, P1499/P1500/P1501 e os dois scripts P1497). Os dois
binários mantiveram seus SHA-256 da tabela acima.

Fontes e observações diretas, anteriores à decisão:

| Candidato, controle e fonte | Observável bilateral | Classificação |
|---|---|---|
| `fonts --font-path '' --ignore-system-fonts`, contra omissão da flag; upstream `lab/typst-original/crates/typst-cli/src/args.rs:544-550,880-884` | vanilla aceita valor vazio (exit 0); cristalino o rejeita no parser (exit 2); omissão passa em ambos | `VIOLATED` em sintaxe CLI; P1500 |
| `completions nushell`, controle Bash; upstream `.../args.rs:283-312` | vanilla gera script Nushell não vazio; cristalino rejeita `nushell`; ambos geram Bash válido | `VIOLATED` em sintaxe CLI; P1501 |
| `repr(stroke(eval(repr(stroke()))))` e variante com paint/thickness explícitos; upstream `lab/typst-original/crates/typst-library/src/visualize/stroke.rs:227-243` | vanilla devolve `"stroke()"` e `"2pt + rgb(\"#ff4136\")"`; cristalino rejeita o argumento posicional reconstruído. `repr(stroke(1pt))` também difere; named args coincidem | `VIOLATED` em valor/morfologia; P1499 |
| expressão inválida para `stroke`, controle negativo | ambos rejeitam; só aceitação negativa é comparada, não texto do diagnóstico | `MATCH` limitado ao controle |
| HTML simples e MathML/CSS; upstream `lab/typst-original/crates/typst-html/src/mathml.rs:103-149` | HTML simples preserva tags/texto; o HTML com matemática é emitido só pelo vanilla | `MATCH` apenas no controle simples; `DISABLED_BY_PROFILE` para MathML/CSS conforme `prompts/infra/export/html.md:37-38` |
| SVG com `foreignObject` e SVG inválido; upstream `lab/typst-original/crates/typst-library/src/visualize/image/svg.rs:56-109,193-256,488-493` | vanilla renderiza e avisa sobre objeto externo; cristalino não aceita SVG em bytes nesse perfil | `DISABLED_BY_PROFILE` (DEBT-67/P834); nenhum crédito de warning, decoder ou render |

**Hipóteses causais, não intenções deduzidas do resultado:** o parser
cristalino trata a string vazia como ausência de valor; a enumeração
de shells não inclui Nushell; o constructor de `stroke` rejeita qualquer
posicional. Cada hipótese seria refutada por um teste isolado do parser,
da enumeração ou do constructor que aceitasse respectivamente o valor
vazio, Nushell ou um `Stroke` posicional e ainda divergisse no fluxo CLI/
round-trip. As próprias sondas acima provam a violação observável,
independentemente de essas hipóteses de implementação serem exclusivas.

Não se copiou bug do vanilla como requisito. `MathML` e `SVG` são
lacunas de perfil já conhecidas, não passos produtivos novos sob P1497.
Uma futura habilitação exige observador semântico novo; o controle HTML
ignora atributos e não prova paridade de CSS.

O delta de *math kern* em
`lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs:518`
permanece `MISSING_OBSERVATION`: candidato `$L^A$` versus controle
`$L$` compilou nos dois lados, mas a fonte matemática efetiva
`NewCMMath-Book` teve alturas de correção zero e não cruzou o ramo
`height > threshold` versus `>=`. Uma tentativa de forçar outra fonte
com seletor `math.equation` não foi aceita no cristalino, portanto não
forma controle bilateral. Nenhum MATCH ou RED foi inferido; futura sonda
deve construir um fixture comum que cruze o limite e consultar o owner
`00_nucleo/prompts/compiler/math/layout/attach.md` se revelar lacuna.
Os fixtures temporários estão em `/tmp/p1497-kern.MwWjXR/`:
`candidate.typ` SHA-256
`dbce098baa74f670e31a9be93e25aa76673fcde61a0fde4e0cd9f918abde37cf`
e `control.typ` SHA-256
`6defed89b900a85d41a208fc608525e969a6d4bbea8969e841d2efb8e05c0615`.
A execução terminou em 2026-09-25 02:40:41 UTC, com HEAD e só o
lockfile modificado como no inventário.

## Matriz de regressão e decisão

Os 43 casos do manifesto foram repetidos nos seis perfis em 2026-09-25
02:54:30–02:56:07 UTC, com o binário vanilla e o cristalino release da
tabela inicial. Todos os seis runners saíram 0; `order_check.identical`
é verdadeiro em cada um. O HEAD, hashes dos binários,
`git status --short` e `git diff HEAD --stat` permaneceram idênticos antes/depois:
o lockfile modificado (3 inserções/2 remoções) e os mesmos seis arquivos
não rastreados da sonda. JSONs em `/tmp/p1497-matrix-final.kY6ATxBx/`.
Receita por perfil:

~~~sh
python3 lab/parity/matrix/runner.py --profile default \
  --vanilla lab/typst-original/target/release/typst \
  --crystalline target/release/typst \
  --output /tmp/p1497-matrix-final.kY6ATxBx/default.json
~~~

| Perfil | MATCH | DIFFERENCE | DISABLED_BY_PROFILE | VIOLATED/UNKNOWN | SHA-256 do JSON |
|---|---:|---:|---:|---:|---|
| default | 27 | 2 | 14 | 0 | `8be0079a8ade40ca4ba05427c0468bac8cf564703bca6a4509d759b8a72ca527` |
| html | 34 | 2 | 7 | 0 | `ee9ec8ab7db54db7cf91efaa6eb1e63bb29435f6049ac09d019204adf6bd7180` |
| a11y-extras | 28 | 2 | 13 | 0 | `2394ee96d956b0315f2fdb3da98fec11fc3c8ae7e594441b42af8e62b1455caf` |
| html+a11y | 35 | 2 | 6 | 0 | `d3e6beb5173950d03003784c447e4455ed74dcd7a7ccec6ebd09c385c6005625` |
| bundle | 31 | 2 | 10 | 0 | `1768e14a9754f63843bca1198f109b2af2f50854b7f692dff2406934b33612c3` |
| html+bundle | 40 | 2 | 1 | 0 | `aa5fbaa578a7289aeefc0a946f875ca7e1f4e9d828214c8d4617a16de13fbc37` |

Não há expectativas descasadas, `EXTRA_BINDING` ou `BASELINE_ONLY` na
matriz. Os dois `DIFFERENCE` repetidos em todos os perfis são
`P1138-X-002` PNG e `P1138-X-003` PDF: o PNG mantém dimensão igual,
11 de 2.005.644 pixels com delta máximo de um canal; o PDF mantém
páginas, caixas e texto, mas difere no nome interno da fonte. Nenhum
tem testemunho semântico novo, logo não vira `MATCH` nem `VIOLATED`.
Os 43 casos existentes não incluem as três lacunas recém-sondadas:
seu RED fica registrado pelo executor focal e pelos passos próprios,
sem alterar expectativas do manifesto neste passo de triagem.

## Verificação do passo

`CARGO_BUILD_JOBS=1 cargo build --quiet` e o build release offline do
extrator passaram. A suíte Python da matriz passou 85/85, a do
inventário 25/25 e a do executor P1497 15/15;
`python3 lab/parity/matrix/runner.py --validate-only` aceitou os
43 casos. `crystalline-lint .` terminou exit 0, com avisos informativos
preexistentes. `git diff --cached --check` passou com os oito arquivos
preparados. A triagem não mudou L0 nem código produtivo, portanto
não há resselo de lineage ou tentativa de GREEN para lacunas futuras.

## Fila de trabalho

- P1499: contrato posicional de `stroke`, owner
  `00_nucleo/prompts/compiler/stdlib/layout.md`.
- P1500: valor vazio de `--font-path`, owner
  `00_nucleo/prompts/shell/cli.md`.
- P1501: `completions nushell`, owner
  `00_nucleo/prompts/shell/completions.md` e, se o parser mudar,
  `00_nucleo/prompts/shell/cli.md` para seu consumer próprio.

P1498 já existia; não foi sobrescrito. Cada novo passo exige L0
proprietário atualizado antes de código, teste RED→GREEN e gate humano
ADR-0127 porque muda contrato/sintaxe pública. P1497 não autoriza
materialização produtiva dessas lacunas. Não há percentual global de
paridade extraído destas amostras.
