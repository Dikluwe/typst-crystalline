# P1498 — fecho restrito das sete lacunas P1489

## Proveniência e medições, antes da decisão

O HEAD cristalino medido é
`7424d5eaa808463b32b3a7f668df7055f599c5e8` (P1497). Em todas
as janelas abaixo, `git status --porcelain=v1 --untracked-files=all`
e `git diff HEAD --stat` estavam vazios antes/depois; os arquivos
deste relatório e de P1502 foram escritos **depois** das sondas.
O alvo vanilla continua o commit
`9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, não
`/usr/local/bin/typst`.

| Produto | SHA-256 no momento da medição |
|---|---|
| vanilla `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| cristalino `target/release/typst` | `2620ffb09b9f1ed2f51bdb412cd883cc1bf944c5f0a6a7433926eee7146412e3` |
| extrator vanilla `p1140-inventory` | `216c89f75bef84d5f6276ce20cd4dbabd01c6329cdcf7f56fdbc1eef3384e7b8` |
| extrator cristalino `surface-inventory-p1140` | `a9f043e391fcab59bdd6efd9015a00762f752bb7da4efed494ea074229c4c173` |
| cristalino debug, primeira extração | `016805567c7037787664ab3c0682037697a39b7004958b1bb74b492b9156014c` |
| cristalino debug após `CARGO_BUILD_JOBS=1 cargo build --quiet` | `b5e1c3fe857c0a2a470dcb752e542aa646522ab75d2affd8226aae744c6b3aad` |

O build após o commit P1497 reembutiu o HEAD no debug, alterando seu
SHA sem alteração de código produtivo. Por isso os seis inventários
foram repetidos com o **segundo** debug. Os JSONs pré-build e
pós-build diferem só em `catalog_provenance.crystalline.product_sha256`;
entradas, contagens, SHA do catálogo e proveniência vanilla são iguais.

## Sete RED P1489: observável focal completo

De 2026-09-25 10:11:05 a 10:13:50 UTC, os sete casos foram repetidos
individualmente nos perfis aplicáveis com os dois binários release
acima. Receita (variar ID e perfil, saída em `/tmp/p1498-focal.wWP5aU/`):

~~~sh
python3 lab/parity/matrix/runner.py --profile html \
  --case P1489-S-001 \
  --vanilla lab/typst-original/target/release/typst \
  --crystalline target/release/typst \
  --output /tmp/p1498-focal.wWP5aU/P1489-S-001-html.json
~~~

Todos deram `MATCH`, `expectation_met=true`,
`order_check.identical=true`, ambos os CLI exit 0. As fixtures
`L-001`, `I-001` e `B-001` executam asserções internas antes de
emitir artefato; `B-002`–`B-004` comparam valores tipados e `repr`.
`S-001` exige **exatamente** os dois parágrafos
`CJK_JOINED` e `CJK_EXPLICIT_SPACE` em ambos, não só HTML
compilável nem igualdade da árvore com classes incidentais.
`I-001` e `S-001` só recebem crédito nos três perfis HTML;
nos outros, `DISABLED_BY_PROFILE` não conta como MATCH.

| Caso/perfil | SHA-256 do JSON focal |
|---|---|
| L-001/default | `75d9102cd893568c5b44c6da75b495674b9fcef11b54e92210c947fcaacf6f33` |
| I-001/html | `a962e4ef64500c1f7d309708646b1ad344adb199b3782247ab0985be3e4f671f` |
| S-001/html | `bdce5420a8e1bf3b91c5ca44eeda7db2e4c9c3a9273e1b8d2f93cda2c85e01dc` |
| B-001/default | `7bad30974f6df0b6450ffc0e02d590f94d0507c8d09163db1ed8ba3308ac0d92` |
| B-002/default | `edc1701c5d2fbcd674ff67d8df26c6f13c75582391776d7484db84343b5eb61e` |
| B-003/default | `f1325f56d045794ee57981ba48c12bc9e913e45e458a35dfa83b3c90622cc423` |
| B-004/default | `284d66b263f0b257ab6692e0723ca39562ab42d4e1cdfc437989d444694b3708` |

Histórico RED→GREEN causal, com L0 e gates, está nos diagnósticos
P1490–P1496. Esta repetição não rebatiza resultados antigos sem
prova nova. `B-002/003/004` devolveram respectivamente
`<module chess>`, `symbol("⊩", ("not", "⊮"))` e
`symbol("⌙")` nos dois lados.

## Matriz bilateral: 43 casos × seis perfis

Janela 2026-09-25 10:11:12–10:15:18 UTC, HEAD e worktree limpos
antes/depois, binários release com os SHA da tabela inicial. Receita
por perfil:

~~~sh
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/matrix/runner.py \
  --profile default \
  --vanilla lab/typst-original/target/release/typst \
  --crystalline target/release/typst \
  --output /tmp/p1498-matrix.Tp0aNmzC/default.json
~~~

| Perfil | MATCH | DIFFERENCE | DISABLED_BY_PROFILE | VIOLATED/UNKNOWN | SHA-256 do JSON |
|---|---:|---:|---:|---:|---|
| default | 27 | 2 | 14 | 0 | `8be0079a8ade40ca4ba05427c0468bac8cf564703bca6a4509d759b8a72ca527` |
| html | 34 | 2 | 7 | 0 | `ee9ec8ab7db54db7cf91efaa6eb1e63bb29435f6049ac09d019204adf6bd7180` |
| a11y-extras | 28 | 2 | 13 | 0 | `2394ee96d956b0315f2fdb3da98fec11fc3c8ae7e594441b42af8e62b1455caf` |
| html+a11y | 35 | 2 | 6 | 0 | `d3e6beb5173950d03003784c447e4455ed74dcd7a7ccec6ebd09c385c6005625` |
| bundle | 31 | 2 | 10 | 0 | `1768e14a9754f63843bca1198f109b2af2f50854b7f692dff2406934b33612c3` |
| html+bundle | 40 | 2 | 1 | 0 | `aa5fbaa578a7289aeefc0a946f875ca7e1f4e9d828214c8d4617a16de13fbc37` |

Todos os runners saíram 0, expectativas satisfeitas e ordem idêntica;
`EXTRA_BINDING` e `BASELINE_ONLY` também são zero. O PNG
`P1138-X-002` conserva dimensões 1191×1684 e 11 de 2.005.644
pixels diferentes por delta máximo de um canal. O PDF
`P1138-X-003` mantém página/caixa 595,276×841,89 pt e texto
`Hello, parity.`; só o nome interno da fonte difere
(`LibertinusSerif-Regular-Identity-H` versus
`LibertinusSerif-Regular`). Nenhum novo testemunho semântico
autoriza promover os dois `DIFFERENCE` a MATCH ou VIOLATED.

## Inventário de superfície, com debug pós-build

Janela final 2026-09-25 10:27:18–10:28:25 UTC, HEAD e worktree
limpos antes/depois; extratores, oráculo e debug pós-build pinados na
tabela inicial. A mesma receita de extração/merge do diagnóstico P1497
foi repetida nos seis perfis. JSONs em
`/tmp/p1498-inventory-postbuild.jcwUrMUM/`.

| Perfil | MATCH | UNVERIFIED_METADATA | EXTRA_BINDING bruto | SHA-256 do merge final |
|---|---:|---:|---:|---|
| default | 1598 | 469 | 49 | `ca704b42734a516d83723a5e470dae0f25a4fc8a08102eacce8727be98f95605` |
| html | 1598 | 699 | 50 | `cb951137a3b4d23a76e4f881bb2fbd266a5cca493e33cdae8fc02599d4acac2b` |
| a11y-extras | 1598 | 475 | 49 | `ecd3b69b2bb0465fad9fa5167ac6efb9c0fcf666420f7b3a9d0cdb6143481edd` |
| html+a11y | 1598 | 705 | 50 | `423976a8146a5d999df188e306e319bf8017cb2ab65f01f86100a979c6a50305` |
| bundle | 1598 | 472 | 49 | `f3d6d9b7bb81775b2646bf169027e99e92a055d38e33294d16cb291db9521cd6` |
| html+bundle | 1598 | 702 | 50 | `00c11c73ddabc4d433d1df8f2f86ccc6a42bda2e1228f6e8aafc58a4db719ff5` |

`MISSING_MEMBER`, `MISSING_BINDING`, `WRONG_KIND`,
`VERIFIED_METADATA_DIFFERENCE`, `UNKNOWN` e
`blocked_by_ancestor` são zero em todos. Os seis pais
`{sym,math}.{chess,keyboard,ordinal}`, os 42 filhos imediatos e os
24 bindings dos 12 grupos espelhados são MATCH em cada perfil.
A sonda bilateral de valor/`repr` P1495 foi repetida sobre o
release com:

~~~sh
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/tools/p1495_symbols_probe.py \
  --crystalline target/release/typst > /tmp/p1498-symbols.QUt2LV.json
~~~

O JSON `/tmp/p1498-symbols.QUt2LV.json` tem SHA-256
`14205762f1da58f2c165e3a3c5c0d823f22bcef17b5de0efb11ab144a8d29538`;
198 casos, 146 MATCH, 36 EXTENSION, 16
TEXT_MATCH_CARRIER_DIFF, zero RED. O arquivo terminou às
10:18:33 UTC, com HEAD e worktree limpos; resultados iguais aos
P1496. As 16 diferenças de carrier e as 36 extensões não recebem
crédito de paridade.

Seis defaults `format.*` duplicam rotas no default; HTML acrescenta
um sétimo duplicado. Resultam 43 rotas extras normalizadas em cada
perfil, reduzidas a 41 famílias de decisão ao agrupar os pares
`sym/math.join` e `sym/math.registered`. São extensões/observações,
não defeitos automaticamente removíveis. Os 469 metadados
não verificados no default permanecem **inconclusivos**.

## Triagem P1497 e limite novo P1407

A sonda P1497 foi repetida no HEAD commitado, worktree limpa,
2026-09-25 10:12:09–10:12:12 UTC. JSON
`/tmp/p1498-language.YynKfx.json`, SHA-256
`2139d341b475ec19df6a69ea6bc17de288f38acb0e027f8e7210161b58848147`;
5 MATCH, 4 VIOLATED, 3 DISABLED_BY_PROFILE, exit 1 **esperado**
pelos RED. As quatro linhas VIOLATED são três contratos fora do fecho
P1489: `stroke` posicional (P1499), `--font-path ''` (P1500)
e `completions nushell` (P1501). Math kern ainda não tem
observável no ramo relevante; MathML e SVG continuam fora do perfil.
Não extrapolar percentual global.

A suíte Rust ampla de `typst-infra` revelou cinco falhas
`pipeline::tests::p1407_*`. Uma execução focal reproduziu
**0/5 GREEN** (log SHA-256
`2345f9d878b86a2345afe30ec7477d16ba550a82a40b711741608d66624311fb`).
A suíte ampla não passou; seu teste raster P1431 continuou consumindo
CPU por mais de quatro minutos e foi encerrado como processo desta
execução, sem processos remanescentes. O log parcial tem SHA-256
`8b136b8b0e4aad3410b1cb3da65de386d3026d4514304502f99b2a87f8ee1323`.
Isso não é contado como gate verde.

A falha P1407 tem testemunho **de língua**, não só frame interno.
Em HEAD limpo, janela 10:27:34–10:32:26 UTC, ambos os produtos
compilaram com exit 0 `lab/parity/corpus/p1407/{repeat,outline,narrow}.typ`.
A extração textual `pdftotext -layout` mediu:

| Fixture (SHA-256 da fonte) | Pontos vanilla/cristalino | Observação |
|---|---:|---|
| repeat `4242ed2345e44d8babc48ac7f51b5dd174804944bec67ffa746fad6487b9e90a` | 23/1 | cristalino `AA.` duplica prefixo e não repete o ponto |
| outline `0de527c4ae84e84a164656ff1a346b1f29cae4b106cbe6eace700e5c205264ef` | 76/18 | linhas, números e fill da entrada divergem |
| narrow `58ce5ae0af11ad2c3f69b1edcde54e8a4643c5ddea33a370c82af718b3f8158d` | 16/5 | fill estreito diverge |
| controle sem prefixo `b990216334a6b73f69b309e7c88184aa9396fbdec8aa022b59e6a43cedb3cc02` | 23/23 | texto completo idêntico bilateralmente |

| PDF | SHA-256 vanilla | SHA-256 cristalino |
|---|---|---|
| repeat | `547dfdc51a0236d0b93b22e8dd2cde12e5767004f6ceef838bac0ceeb572a116` | `31fa19233deee19d0b4e28203108bd7273080d2d1095adb8df9db009669f5754` |
| outline | `25923467fb7dee85ca9ed673300606b5b27e35f557430932d100955b3c5b6b22` | `05cf5b5abea08a1410d0b7e2a0b4527f0dcb8bbd90e1fa235f4fd77ed3cbc0b6` |
| narrow | `66078ff2c27c49caf03f02004e4b5c5ed71e3341df54b78be68fa99209935679` | `811582d50aa43cca573122eabcdeb35d0170272516184c00276242cbe8dac571` |
| controle sem prefixo | `4e121dfb24826e84e186183bf845f592eaa86edd89d0e86f785db0dac93b3b6b` | `9b950601483e460824efaf2f95899f0d22ec476da0138d38b9a2673f03bbeb7e` |

Receita (para cada caso e lado, saída somente em
`/tmp/p1498-p1407.dhj9Ov/`):

~~~sh
lab/typst-original/target/release/typst --color=never compile \
  lab/parity/corpus/p1407/repeat.typ \
  /tmp/p1498-p1407.dhj9Ov/repeat-vanilla.pdf
target/release/typst --color=never compile \
  lab/parity/corpus/p1407/repeat.typ \
  /tmp/p1498-p1407.dhj9Ov/repeat-crystal.pdf
pdftotext -layout /tmp/p1498-p1407.dhj9Ov/repeat-vanilla.pdf -
pdftotext -layout /tmp/p1498-p1407.dhj9Ov/repeat-crystal.pdf -
~~~

Fonte exata do controle sem prefixo, para regenerar o fixture
temporário após a limpeza:

~~~typ
#set page(width: 180pt, height: auto, margin: 20pt)
#box(width: 100pt, repeat(".", gap: 2pt, justify: false))B
~~~

O controle sem prefixo refuta falha genérica de fonte ou extração.
P1407 historicamente obteve 5/5 e 23/23, 76/76, 16/16 no pin antigo;
o diagnóstico P1494 já reproduziu 0/5 no archive do commit P1493,
sem o código P1494. A hipótese causal atual, **inferência não
demonstrada por bisect**, é a retirada/restauração do prefixo em
`01_core/src/compiler/layout/boxed.rs:157-179` (blame P1491)
sem rebasing de `PendingRepeat.start/end` criados em
`layout/repeat.rs:32-51`; `layout/cursor.rs:489-519` usa os índices
após restaurar o prefixo. Um teste que mostrasse índices corretos
após restauração e ainda produzisse `AA.`, ou build pré-P1491 com
a mesma falha, refutaria essa causa. O L0 vigente exige tanto
isolamento da box quanto expansão finita; não se deve desfazer
P1491 para mascarar a regressão. P1502 isola diagnóstico,
RED→GREEN, owners L0 e aceitação. É lacuna separada das sete P1489,
portanto não impede seu **fecho restrito**, mas impede qualquer
declaração de fecho geral.

## Gates e decisão

`CARGO_BUILD_JOBS=1 cargo build --quiet` saiu 0. A suíte
`typst-core --lib` passou **6190/6190** (log SHA-256
`b685deb841763250eb5a28e63be9a58b05884681507691c46967d96e2d541bc2`);
testes focais `typst-infra --lib p1493` passaram 6/6 e
`p1494` 4/4. As suítes Python da matriz, inventário e harness
P1497 passaram 85/85, 25/25 e 15/15; o manifesto validou 43
casos. `crystalline-lint --quiet .` e
`crystalline-lint --checks v5,v15,v26 --quiet .` saíram 0.
Nenhum L0 ou código produtivo mudou neste passo; não há resselo
novo. `git diff --cached --check` passou. Os artefatos temporários
criados por P1498 em `/tmp/p1498-*` foram removidos após guardar
comandos, contagens e SHA-256 neste relatório; não são recuperáveis
diretamente, mas podem ser regenerados pelas receitas. Nenhum
processo Cargo ou teste desta execução ficou ativo.

**Decisão nova:** as **sete lacunas de linguagem confirmadas por
P1489 estão fechadas no pin ratificado e nos perfis aplicáveis**,
pelas asserções e observáveis acima. Não declarar paridade global,
não converter os 469 metadados inconclusivos, 16 carriers, 36
extensões, dois DIFFERENCE ou três lacunas P1497 em MATCH. A
regressão de morfologia P1407 permanece aberta em P1502;
assim este fecho é deliberadamente restrito.
