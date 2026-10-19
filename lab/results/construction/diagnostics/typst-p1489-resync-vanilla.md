# P1489 — novo pin vanilla e recalibração da busca de paridade

## Proveniência da fonte e dos produtos

Medição concluída em 2026-09-24 15:50:08 UTC, HEAD cristalino
`9e3a267f554b89107d66e5001400de56d3cc6bb7`, working tree não commitado.
A lista exata de arquivos alterados consta no snapshot ao fim deste relatório.
`git ls-remote upstream HEAD refs/heads/main` devolveu, para ambos,
`9f2b6e8715237cb086899a42873660fe744622e8`. O pin antigo era
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. O intervalo contém
29 commits e modifica 132 caminhos upstream. A árvore do novo commit é
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`.
O novo alvo é esse commit fixo, não a tag 0.15.1 nem `main` móvel.
A fonte oficial é
[typst/typst 9f2b6e87](https://github.com/typst/typst/commit/9f2b6e8715237cb086899a42873660fe744622e8).

`lab/typst-original` foi atualizado por diff Git entre os dois commits.
Não tem `.git` próprio. Uma index temporária com `git read-tree
--prefix=lab/typst-original/ upstream/main^{tree}` e `git diff
--name-status` encontrou **somente `Cargo.lock`** diferente entre os
arquivos rastreados pelo upstream e a cópia do lab. Essa diferença, mais
`crates/p1140-inventory/Cargo.toml` e
`crates/p1140-inventory/src/main.rs`, é o overlay local do extrator.
Os três foram preservados. `target/` ignorado e `/usr/local/bin/typst`
não foram alterados.

Construção sequencial, com um job e lockfile:

~~~sh
CARGO_BUILD_JOBS=1 TYPST_COMMIT_SHA=9f2b6e8715237cb086899a42873660fe744622e8 \
  cargo build --release --locked --manifest-path lab/typst-original/Cargo.toml \
  -p typst-cli -p p1140-inventory --quiet
CARGO_BUILD_JOBS=1 cargo build --release --locked \
  --manifest-path lab/surface-inventory/Cargo.toml --quiet
CARGO_BUILD_JOBS=1 cargo build --quiet
~~~

| Produto medido | SHA-256 |
|---|---|
| `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| `lab/typst-original/target/release/p1140-inventory` | `216c89f75bef84d5f6276ce20cd4dbabd01c6329cdcf7f56fdbc1eef3384e7b8` |
| `target/debug/typst` após resselo L0 e build | `180ef6f3be355d26840001f1217b4a496254946fcb5c75977717c40de9956c6a` |
| `lab/surface-inventory/target/release/surface-inventory-p1140` | `449dd2cb1757ab114eb70d7eba2525a146e89a2e1044cbfb0089e062df33fb36` |

`lab/typst-original/target/release/typst --version` imprimiu
`typst 0.15.1 (9f2b6e87)`, mas essa string é evidência auxiliar.
`/usr/local/bin/typst --version` ainda imprime
`typst 0.15.1 (586e1bd4)`; usá-lo agora como oráculo invalidaria as
medições. Ambos os produtos devolvem `"version(0, 15, 1)"` para
`eval 'sys.version' --format json`. Assim, o L0
`prompts/entities/version.md` foi atualizado antes de ressellar seu
consumer, mas `PARITY_VERSION` não mudou.

## Critério novo de busca

O pin novo substitui a **autoridade de observação futura**. Medições antigas
permanecem registros históricos, nunca crédito automático contra o novo
snapshot. Paridade é semântica, sintaxe e morfologia da linguagem
(ADR-0107/0108), não igualdade de bytes de PDF, algoritmo ou código-fonte.
Bugs de execução do vanilla não são requisitos a copiar. Uma divergência
observada é classificada conforme o observável; um catálogo não substitui
sonda executável. Perfis desativados não recebem MATCH. Extensões deliberadas
do cristalino não viram crédito de paridade. Nenhum percentual global é
extrapolado de uma amostra de casos.

Essa regra está agora em `CLAUDE.md`, no auditor L0 e no README da matriz;
o inventário antigo de cobertura ganhou adendo que marca seus percentuais
como históricos. Configurações operacionais da matriz, inventário e smoke
apontam para o binário novo do lab. O smoke não consulta PATH, que ainda
poderia escolher o pin velho.

## Matriz bilateral: 39 casos × seis perfis

A matriz foi executada com os dois binários de SHA acima. Estados esperados
dos sete casos novos são VIOLATED (RED deliberado), não resultado de
implementação cristalina. Os 32 casos anteriores também foram repetidos.
`order_check.identical` é verdadeiro em cada perfil, não houve
expectativa descasada nem UNKNOWN.

| Perfil | MATCH | DIFFERENCE | DISABLED_BY_PROFILE | VIOLATED | UNKNOWN |
|---|---:|---:|---:|---:|---:|
| default | 22 | 2 | 10 | 5 | 0 |
| html | 24 | 2 | 6 | 7 | 0 |
| a11y-extras | 23 | 2 | 9 | 5 | 0 |
| html+a11y | 25 | 2 | 5 | 7 | 0 |
| bundle | 26 | 2 | 6 | 5 | 0 |
| html+bundle | 29 | 2 | 1 | 7 | 0 |

O caso `P1137-E-001` mede o triplo impresso por `--version` com
`semantic_version`: é proxy auxiliar do CLI, **não** prova direta de
`sys.version` na linguagem. O `eval 'sys.version' --format json` bilateral
foi medido separadamente acima. Após a execução, só o rótulo e a nota
desse caso no manifesto foram corrigidos; comando, comparador e contagens
da matriz não mudaram.

Os dois DIFFERENCE já conhecidos mantêm mesmo tamanho de página PNG,
com 11 de 2.005.644 pixels diferentes por no máximo um canal, e no PDF
margens/texto coincidentes com nome interno de fonte distinto. Não são
MATCH nem violação semântica demonstrada por essa medição.

Os sete RED novos são: `measure().baseline` (vanilla aceita 20pt no
retângulo, cristalino não possui a chave);
`location.page()/position()` em HTML (vanilla `none` fora de frame;
cristalino retorna página); descarte de quebra de linha entre ideogramas
CJK (forma do texto HTML difere, espaço explícito permanece);
`repr(stroke())` (vanilla `stroke()`, cristalino
`1pt + black`); `sym.chess` ausente; `sym.tack.r.double`
com símbolo `⊩` no vanilla e `⊨` no cristalino; e
`sym.not.inv` ausente. As fixtures estão no manifesto P1489 e medem
valor/morfologia da linguagem, não só sucesso bilateral de compilação.

Fontes do delta, antes da decisão:
`lab/typst-original/crates/typst-library/src/layout/measure.rs:44-47,105-110`;
`.../introspection/location.rs:87-132`;
`lab/typst-original/crates/typst-realize/src/spaces.rs:178-207`;
`.../visualize/stroke.rs:310-319`. Outros deltas de SVG, MathML,
kern matemático e CLI são candidatos a sondas futuras, não recebem estado de
paridade a partir do diff de fonte.

## Inventário de superfície: saída p1282-v2

A comparação de metadados verificada deixou de ser ocultada em
`UNVERIFIED_METADATA`. Agora `MISSING_OBSERVATION` permanece
inconclusiva nessa classe; `VERIFIED_DIFFERENCE` vira
`VERIFIED_METADATA_DIFFERENCE`. A distinção não promove automaticamente
metadados a violação de linguagem. Os catálogos de entrada continuam
`p1282-catalog-v1`; o merge de saída é `p1282-v2`.

| Perfil | MATCH | MISSING_MEMBER | UNVERIFIED_METADATA | VERIFIED_METADATA_DIFFERENCE | EXTRA_BINDING | UNKNOWN |
|---|---:|---:|---:|---:|---:|---:|
| default | 1526 | 6 | 469 | 24 | 49 | 0 |
| html | 1526 | 6 | 699 | 24 | 50 | 0 |
| a11y-extras | 1526 | 6 | 475 | 24 | 49 | 0 |
| html+a11y | 1526 | 6 | 705 | 24 | 50 | 0 |
| bundle | 1526 | 6 | 472 | 24 | 49 | 0 |
| html+bundle | 1526 | 6 | 702 | 24 | 50 | 0 |

Hashes SHA-256 dos JSONs finais (perfil → matriz / merge), antes da limpeza
dos artefatos temporários:

| Perfil | Matriz | Inventário v2 |
|---|---|---|
| default | `1a4b4a42a16a74be21669325b0947aa451ad8dbba5c1cf1c31bad50dad13cd25` | `a98913aa10d42a20f9c9de1115165bbc372406b378d103a0f3de6b077fe6615c` |
| html | `f44003784abf58a9dd4e61b4d5b8cf14abf6c528328a478d460c7bdc6452143f` | `edc43d62b3d7d62a0e53e5bd0748f79940b3df48b86e2207fdf5cb19d36c6be4` |
| a11y-extras | `b88b2e0584dd0125db5df2a817d41648aa952e97d72f023187720b6cf199dd63` | `2591764a7133e593d981499cbc7d20ecd917a9a769c83aac2238f3f1b137c5a3` |
| html+a11y | `7fe15630f9b68cb517996da3f13f226e08bbe7f71a9c5995e8b32076c6bd8cee` | `c0ba174b7e3056701e04cec6801748045bea630b6646f357ffba1a95c688531e` |
| bundle | `24319854dea6c670463311ab10999d20d7a8d612ed9347b8c7305cfa0c23a688` | `29c8b6e847d088f2b926040e8c431dc329a25d8a24e7b887761e7e86087b1982` |
| html+bundle | `d60589d6876d0767ba9f63ad946145015a6daf087a2bb7dab762667e1854ab1d` | `78cdf7c80fdf7d84cf4e033cd13f825e29e42ca98fb10208888cde1acf1064fa` |

`MISSING_BINDING` e `WRONG_KIND` são zero em todos os perfis.
Os seis pais ausentes são
`{sym,math}.{chess,keyboard,ordinal}`, com 42 descendentes
`blocked_by_ancestor` adicionais — seis não é a dimensão total da
lacuna. Sondas diretas confirmaram filhos como `sym.chess.king.white`,
`math.keyboard.tab.r` e `sym.ordinal.feminine`. As 24 diferenças
verificadas são 12 grupos duplicados em `sym` e `math`:
`eq, equiv, gt, lat, lt, not, prec, smt, succ, tack, tilde, trademark`.
`tack` e `not.inv` já têm violação de linguagem demonstrada;
as outras famílias aguardam sonda específica.

A origem principal é upstream `codex` git `2e43ab9`
(`lab/typst-original/Cargo.toml:51`) versus `codex 0.3.0`
no cristalino (`Cargo.toml:74`). `sym.join` e `math.join`
foram removidos pelo upstream novo e continuam no cristalino; são os dois
extras brutos novos, não lacunas. Após excluir seis ou sete defaults duplicados
de `format.*`, restam 43 rotas extras normalizadas (41 antigas +
2 `join`). Removê-las seria mudança pública de compatibilidade e exige
L0 causal e gate humano ADR-0127; nenhuma remoção foi feita aqui.

**Inferência causal:** a troca de `codex` explica os novos módulos, variantes,
valores e a remoção de `join`; a diferença de dependência foi medida, mas
a causalidade exclusiva seria refutada se ambos os produtos com o mesmo
catálogo ainda divergissem por overrides próprios. As sondas de linguagem
estabelecem as lacunas independentemente dessa hipótese de causa.

O pin novo **não ressella automaticamente os L0 proprietários**: vários ainda
chamam `586e1bd43` de vigente no presente. Antes de implementar cada RED,
reauditar medição e L0 do owner 1:1, sem substituição textual em massa.
Candidatos iniciais: `compiler/stdlib/layout.md`/`compiler/layout.md` para
baseline; `compiler/introspect.md` para localização HTML;
`compiler/eval/markup.md` para morfologia CJK; `compiler/eval/repr.md` para
stroke; `compiler/stdlib/sym.md` e `entities/symbol.md` para símbolos.
A escolha final depende da auditoria de ownership e das ADRs vigentes.

Receita de medição por perfil (exemplo `default`; repetir com `html`,
`a11y-extras`, `html+a11y`, `bundle` e `html+bundle`; nos nomes de
arquivo, trocar `+` por `-`):

~~~sh
mkdir -p /tmp/p1489-probes.Q6AMA0
python3 lab/parity/matrix/runner.py --profile default \
  --vanilla lab/typst-original/target/release/typst \
  --crystalline target/debug/typst \
  --output /tmp/p1489-probes.Q6AMA0/matrix-final-default.json
lab/typst-original/target/release/p1140-inventory \
  /tmp/p1489-probes.Q6AMA0/inventory-final-vanilla-default.json default \
  lab/typst-original/target/release/typst
lab/surface-inventory/target/release/surface-inventory-p1140 \
  /tmp/p1489-probes.Q6AMA0/inventory-final-crystal-default.json \
  /tmp/p1489-probes.Q6AMA0/inventory-final-vanilla-default.json default \
  lab/surface-inventory/extra_seeds.json target/debug/typst
python3 lab/surface-inventory/merge.py \
  /tmp/p1489-probes.Q6AMA0/inventory-final-vanilla-default.json \
  /tmp/p1489-probes.Q6AMA0/inventory-final-crystal-default.json \
  /tmp/p1489-probes.Q6AMA0/inventory-final-merged-default.json --profile default
~~~

## Gates, limite e limpeza

Rodaram `crystalline-lint --checks v5,v15,v26 --fix-hashes .`,
`CARGO_BUILD_JOBS=1 cargo build --quiet` e
`crystalline-lint .` com código 0; o lint emitiu avisos informativos
preexistentes. `git diff --check` passou. A suite Python da matriz
passou 81/81; a do inventário passou 25/25. O smoke Rust isolado do CLI
passou 2/2 via `rustc --edition=2021 --test`, sem recompilar a crate.
Nenhuma mudança funcional produtiva foi materializada
neste passo; apenas o header `@prompt-hash` de
`01_core/src/entities/version.rs` mudou pelo resselo L0.
Passos futuros para os RED devem começar pela medição e L0 proprietário,
com RED→GREEN, revalidação por perfil e sem copiar bugs do vanilla.

A execução foi sequencial, com um job nos builds, sem servidor, watch ou
processo Cargo/Rustc deixado ativo. Artefatos temporários em
`/tmp/p1489-probes.Q6AMA0` foram removidos após fixar hashes, contagens
e comandos neste relatório. Não são recuperáveis diretamente, mas podem
ser regenerados pela receita acima; os binários construídos foram mantidos.

## Snapshot exato do worktree

Segue a saída de `git status --porcelain=v1 --untracked-files=all` após os gates,
antes da própria inclusão da lista neste relatório. A proveniência de cada
número acima é o HEAD, os quatro hashes de produto e essa lista, não
somente a string de versão.

Snapshot em 2026-09-24 15:56:33 UTC; `git status --porcelain=v1 --untracked-files=all`
SHA-256 `b0f2602318d428c8a378b0f4767d831ca13f8b22051a0618f4d256bef53b2c2b` (151 caminhos). O arquivo deste diagnóstico
já constava como não rastreado; a inclusão deste bloco só altera seu conteúdo.

~~~text
 M 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
 M 00_nucleo/prompts/auditar-spec.md
 M 00_nucleo/prompts/entities/version.md
 M 01_core/src/entities/version.rs
 M CLAUDE.md
 M lab/parity/matrix/README.md
 M lab/parity/matrix/manifest.yaml
 M lab/parity/matrix/runner.py
 M lab/parity/matrix/schema.json
 M lab/parity/tests/vanilla_cli_smoke.rs
 M lab/surface-inventory/merge.py
 M lab/surface-inventory/test_merge.py
 M lab/typst-original/.github/workflows/ci.yml
 M lab/typst-original/Cargo.lock
 M lab/typst-original/Cargo.toml
 M lab/typst-original/crates/p1140-inventory/src/main.rs
 M lab/typst-original/crates/typst-bundle/Cargo.toml
 M lab/typst-original/crates/typst-cli/Cargo.toml
 M lab/typst-original/crates/typst-cli/build.rs
 M lab/typst-original/crates/typst-cli/src/args.rs
 M lab/typst-original/crates/typst-cli/src/compile.rs
 M lab/typst-original/crates/typst-cli/src/fonts.rs
 M lab/typst-original/crates/typst-cli/tests/smoke.rs
 M lab/typst-original/crates/typst-eval/Cargo.toml
 M lab/typst-original/crates/typst-eval/src/markup.rs
 M lab/typst-original/crates/typst-html/Cargo.toml
 M lab/typst-original/crates/typst-html/src/format.rs
 M lab/typst-original/crates/typst-html/src/mathml.rs
 M lab/typst-original/crates/typst-ide/Cargo.toml
 M lab/typst-original/crates/typst-ide/src/docs.rs
 M lab/typst-original/crates/typst-kit/Cargo.toml
 M lab/typst-original/crates/typst-layout/Cargo.toml
 M lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs
 M lab/typst-original/crates/typst-layout/src/math/mod.rs
 M lab/typst-original/crates/typst-library/Cargo.toml
 M lab/typst-original/crates/typst-library/src/diag.rs
 M lab/typst-original/crates/typst-library/src/format.rs
 M lab/typst-original/crates/typst-library/src/foundations/calc.rs
 M lab/typst-original/crates/typst-library/src/foundations/content/element.rs
 M lab/typst-original/crates/typst-library/src/foundations/float.rs
 M lab/typst-original/crates/typst-library/src/foundations/func.rs
 M lab/typst-original/crates/typst-library/src/foundations/int.rs
 M lab/typst-original/crates/typst-library/src/foundations/path.rs
 M lab/typst-original/crates/typst-library/src/foundations/scope.rs
 M lab/typst-original/crates/typst-library/src/foundations/symbol.rs
 M lab/typst-original/crates/typst-library/src/foundations/sys.rs
 M lab/typst-original/crates/typst-library/src/foundations/ty.rs
 M lab/typst-original/crates/typst-library/src/foundations/version.rs
 M lab/typst-original/crates/typst-library/src/introspection/location.rs
 M lab/typst-original/crates/typst-library/src/introspection/position.rs
 M lab/typst-original/crates/typst-library/src/layout/align.rs
 M lab/typst-original/crates/typst-library/src/layout/dir.rs
 M lab/typst-original/crates/typst-library/src/layout/measure.rs
 M lab/typst-original/crates/typst-library/src/math/mod.rs
 M lab/typst-original/crates/typst-library/src/symbols.rs
 M lab/typst-original/crates/typst-library/src/text/font/book.rs
 M lab/typst-original/crates/typst-library/src/text/font/exceptions.rs
 M lab/typst-original/crates/typst-library/src/text/font/info.rs
 M lab/typst-original/crates/typst-library/src/text/lang.rs
 M lab/typst-original/crates/typst-library/src/text/mod.rs
 M lab/typst-original/crates/typst-library/src/text/space.rs
 M lab/typst-original/crates/typst-library/src/visualize/color.rs
 M lab/typst-original/crates/typst-library/src/visualize/gradient.rs
 M lab/typst-original/crates/typst-library/src/visualize/image/mod.rs
 M lab/typst-original/crates/typst-library/src/visualize/image/svg.rs
 M lab/typst-original/crates/typst-library/src/visualize/stroke.rs
 M lab/typst-original/crates/typst-macros/src/elem.rs
 M lab/typst-original/crates/typst-macros/src/func.rs
 M lab/typst-original/crates/typst-macros/src/lib.rs
 M lab/typst-original/crates/typst-macros/src/scope.rs
 M lab/typst-original/crates/typst-macros/src/ty.rs
 M lab/typst-original/crates/typst-macros/src/util.rs
 M lab/typst-original/crates/typst-pdf/Cargo.toml
 M lab/typst-original/crates/typst-realize/Cargo.toml
 M lab/typst-original/crates/typst-realize/src/lib.rs
 M lab/typst-original/crates/typst-realize/src/spaces.rs
 M lab/typst-original/crates/typst-render/Cargo.toml
 M lab/typst-original/crates/typst-svg/Cargo.toml
 M lab/typst-original/crates/typst-syntax/Cargo.toml
 M lab/typst-original/crates/typst-syntax/src/ast.rs
 M lab/typst-original/crates/typst-syntax/src/highlight.rs
 M lab/typst-original/crates/typst-syntax/src/kind.rs
 M lab/typst-original/crates/typst-syntax/src/lexer.rs
 M lab/typst-original/crates/typst-syntax/src/node.rs
 M lab/typst-original/crates/typst-syntax/src/parser.rs
 M lab/typst-original/crates/typst-syntax/src/reparser.rs
 M lab/typst-original/crates/typst-syntax/src/source.rs
 M lab/typst-original/crates/typst-utils/Cargo.toml
 M lab/typst-original/crates/typst-utils/src/lib.rs
 M lab/typst-original/crates/typst-utils/src/macros.rs
 M lab/typst-original/crates/typst/Cargo.toml
 M lab/typst-original/docs/Cargo.toml
 M lab/typst-original/docs/components/category.typ
 M lab/typst-original/docs/content/changelog/0.12.0.typ
 M lab/typst-original/docs/content/changelog/0.15.0.typ
 M lab/typst-original/docs/content/changelog/0.8.0.typ
 M lab/typst-original/docs/content/changelog/0.9.0.typ
 M lab/typst-original/docs/content/reference/library/foundations.typ
 M lab/typst-original/docs/content/reference/library/math.typ
 M lab/typst-original/docs/src/args.rs
 M lab/typst-original/docs/src/live.rs
 M lab/typst-original/docs/src/main.rs
 M lab/typst-original/docs/src/reflect.rs
 M lab/typst-original/docs/src/world.rs
 M lab/typst-original/tests/Cargo.toml
 M lab/typst-original/tests/ref/html/hashes.txt
 M lab/typst-original/tests/ref/pdf/hashes.txt
 M lab/typst-original/tests/ref/render/image-svg-linked-many-formats.png
 D lab/typst-original/tests/ref/render/symbol-modifier-deprecated.png
 D lab/typst-original/tests/ref/render/symbol-sect-deprecated.png
 M lab/typst-original/tests/ref/render/text-chinese-basic.png
 M lab/typst-original/tests/ref/svg/hashes.txt
 M lab/typst-original/tests/suite/foundations/repr.typ
 M lab/typst-original/tests/suite/html/frame.typ
 M lab/typst-original/tests/suite/html/mathml.typ
 M lab/typst-original/tests/suite/introspection/convergence.typ
 M lab/typst-original/tests/suite/introspection/locate.typ
 M lab/typst-original/tests/suite/layout/measure.typ
 M lab/typst-original/tests/suite/math/text.typ
 M lab/typst-original/tests/suite/symbols/symbol.typ
 M lab/typst-original/tests/suite/text/space.typ
 M lab/typst-original/tests/suite/visualize/image.typ
 M lab/typst-original/tests/suite/visualize/stroke.typ
?? 00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md
?? 00_nucleo/materialization/typst-passo-1489.md
?? lab/parity/matrix/fixtures/cjk-newline-space.typ
?? lab/parity/matrix/fixtures/location-html.typ
?? lab/parity/matrix/fixtures/measure-baseline.typ
?? lab/parity/matrix/fixtures/stroke-repr.typ
?? lab/typst-original/crates/typst-library/translations/za.txt
?? lab/typst-original/crates/typst-utils/src/args.rs
?? lab/typst-original/crates/typst-utils/src/collection.rs
?? lab/typst-original/crates/typst-utils/src/def_site.rs
?? lab/typst-original/crates/typst-utils/src/defer.rs
?? lab/typst-original/crates/typst-utils/src/fmt.rs
?? lab/typst-original/crates/typst-utils/src/get.rs
?? lab/typst-original/crates/typst-utils/src/math.rs
?? lab/typst-original/crates/typst-utils/src/numeric.rs
?? lab/typst-original/crates/typst-utils/src/option.rs
?? lab/typst-original/crates/typst-utils/src/static.rs
?? lab/typst-original/tests/ref/render/image-svg-foreign-object-inside-switch-svg1-1.png
?? lab/typst-original/tests/ref/render/image-svg-foreign-object-inside-switch-svg2-0.png
?? lab/typst-original/tests/ref/render/image-svg-foreign-object-warning.png
?? lab/typst-original/tests/ref/render/issue-792-newline-space-discarding.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-dynamic.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-korean.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-non-text.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-punctuation.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-regex-show-rule.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-smart-quotes.png
?? lab/typst-original/tests/ref/render/newline-space-discarding-strong.png
~~~
