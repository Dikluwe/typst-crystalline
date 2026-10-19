# P1494 — medição e gate da origem de newline em espaços

## Estado e proveniência

Diagnóstico de baseline em 2026-09-24, anterior à proposta L0 e a qualquer
código P1494. O HEAD medido foi
`6da06e849d75972651a342ad6aad337313451c79`, com working tree limpa
(`git diff HEAD --stat` vazio no momento das sondas). Não há registo de hora
exata do comando; as horas dos arquivos temporários são apenas mtime, não
identificam por si o estado do código. Alterações posteriores no runner e no
manifest não pertencem a este baseline.

| Binário medido | SHA-256 |
|---|---|
| vanilla upstream/main pinado `9f2b6e8715237cb086899a42873660fe744622e8`, `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| cristalino, `target/release/typst` | `35a2fdea38c3e5a9b8b0c2a8a93c1cf9f4b6439965ed0103aa0e564c9ca3ca83` |

Reprodução matricial:
`python3 lab/parity/matrix/runner.py --profile html --case P1489-S-001`.
O resultado gravado em `/tmp/p1494-baseline.json` (SHA-256
`701e31977f894ffed5ecbaf764baa54bc3af79c7d682d9676f1c8100de3e3166`)
foi `VIOLATED`, com saída 0 e artefato HTML em ambos. Os digests de ordem
forward/reverse foram idênticos:
`9c0be292de3b9ee78b186eba78daea3a012d5be5e04ec6b1e7ce7c0c438cf2ca`.
O vanilla produziu `CJK_JOINED` e `CJK_EXPLICIT_SPACE`; o cristalino produziu
`水 果` e `CJK_EXPLICIT_SPACE`. O comparador desta medição ainda é
`semantic_tree` total; o passo P1494 exige um observável focal antes do gate
GREEN.

As sondas adicionais estão em `/tmp/p1494-{morph,regex,dynamic,strong}.typ`,
com HTML bilateral ao lado. SHA-256 dos fontes, nessa ordem:
`b09d9b7889a608f1a7ebe7b17bfbb60559af7a4d79c8dd49cfe756ceafb7d9eb`,
`e0a59b94f59328393edca34654b3661f1edcb2f04f5697a39281ac4a5fb79c14`,
`8f3391be671d3f5697892e6ab82cb838cb36fd502e217896cdfd6c32762c7fb2`
e `eda7fb6be2af8b4cb74eaba3bdd7a245385a5fa23c4bb4729285015efe21cea0`.
Foram medidos também PDFs bilaterais de `morph` e `regex`. A sonda
`/tmp/p1494-nontext.typ` (SHA-256
`f5ea67c6e11192bad46fbfaef46180806ce441de0678405643d09d8a8bbb3eed`)
tem apenas HTML vanilla registado; não é prova bilateral.

### Sondas suplementares da revisão L0

As sondas suplementares correram ainda no HEAD
`6da06e849d75972651a342ad6aad337313451c79` e com os mesmos binários
da tabela acima. A working tree já tinha edições de L0, runner e manifest;
não foi capturado o diff exato no instante de cada comando, nem a hora
exata das chamadas. Os `mtime` dos artefactos `/tmp` não substituem esses
dados. Os resultados orientam os testes e a revisão do L0; não fecham
dívida ou paridade sem nova medição com diff e hora registados.

| Fonte temporária | SHA-256 do fonte | Resultado bilateral |
|---|---|---|
| `/tmp/p1494-literal.typ` | `719230d9d8812efed38b812a3d04dd15c7f4f5368d9a17fcfce5eb1ad6f37615` | Vanilla `LITERAL_LATIN`, `LITERAL_CJK`; cristalino `new line`, `水 果`: show literal também atravessa nós no vanilla. |
| `/tmp/p1494-snapshot.typ` | `161c06c311c616ebe730be2f9ff5332eb50c736737d47cb1f121de678a700575` | Para `foo` antes/depois de duas regras show, vanilla `foo`, `EARLY`, `LATE`; cristalino `foo`, `EARLY`, `EARLY`. A precedência vigente já diverge nesse controle. |
| `/tmp/p1494-style2.typ` | `b707f354ee5039219212c0c096a9082a974a378b6de600d2fc178baf9af9a151` | `regex("ab")` não cruza `a#strong[b]` nem `#strong[a]b`; ambos emitem `a<strong>b</strong>` e `<strong>a</strong>b`. |
| `/tmp/p1494-boundaries.typ` | `4d16da2f86dd8bc987adfc18009bc6463326c648ce6b3ec414582e25d28cd259` | `regex("AA")` não casa através de `A#box(width: 1pt)A` no vanilla HTML nem nos PDFs bilaterais. HTML cristalino falha por falta de suporte a `box`, logo não é prova bilateral desse target. |

Comandos exatos de compilação HTML, a partir da raiz do repositório:

```sh
lab/typst-original/target/release/typst --color=never compile /tmp/p1494-literal.typ /tmp/p1494-literal-vanilla.html --format html --features html
target/release/typst compile /tmp/p1494-literal.typ /tmp/p1494-literal-crystal.html --format html --features html --color=never
lab/typst-original/target/release/typst --color=never compile /tmp/p1494-snapshot.typ /tmp/p1494-snapshot-vanilla.html --format html --features html
target/release/typst compile /tmp/p1494-snapshot.typ /tmp/p1494-snapshot-crystal.html --format html --features html --color=never
lab/typst-original/target/release/typst --color=never compile /tmp/p1494-style2.typ /tmp/p1494-style2-vanilla.html --format html --features html
target/release/typst compile /tmp/p1494-style2.typ /tmp/p1494-style2-crystal.html --format html --features html --color=never
lab/typst-original/target/release/typst --color=never compile /tmp/p1494-boundaries.typ /tmp/p1494-boundaries-vanilla.html --format html --features html
target/release/typst compile /tmp/p1494-boundaries.typ /tmp/p1494-boundaries-crystal.html --format html --features html --color=never
lab/typst-original/target/release/typst --color=never compile /tmp/p1494-boundaries.typ /tmp/p1494-boundaries-vanilla.pdf
target/release/typst compile /tmp/p1494-boundaries.typ /tmp/p1494-boundaries-crystal.pdf --color=never
```

Os PDFs `boundaries` foram inspecionados com `pdftotext -layout -enc UTF-8`.
A ausência de `BAD_BOX_CROSS` é controle negativo; o texto extraído `AA`
sozinho não prova que `box` desapareceu da morfologia.

### Verificação focal pós-comparador com proveniência

Entre 2026-09-24 21:53:34 e 21:53:49 UTC, no HEAD
`6da06e849d75972651a342ad6aad337313451c79`, o mesmo status foi
capturado antes e depois da execução. `git diff HEAD --stat` registrou
19 arquivos rastreados, 665 linhas adicionadas e 3 removidas. A lista
exata de arquivos alterados (`git status --short`) era:

```text
 M 00_nucleo/materialization/typst-passo-1494.md
 M 00_nucleo/prompts/compiler/columns.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/eval/repr.md
 M 00_nucleo/prompts/compiler/eval/rules.md
 M 00_nucleo/prompts/compiler/eval/selector_matching.md
 M 00_nucleo/prompts/compiler/introspect.md
 M 00_nucleo/prompts/compiler/introspect/locatable.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/sequence.md
 M 00_nucleo/prompts/compiler/math/layout/spacing.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/infra/export/html.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/query-helpers.md
 M lab/parity/matrix/manifest.yaml
 M lab/parity/matrix/runner.py
 M lab/parity/matrix/test_runner.py
?? 00_nucleo/diagnosticos/typst-p1494-gate-cjk-newline.md
```

O diagnóstico ainda não rastreado tinha SHA-256
`7ec550af0fdbccff0834373a66a60eb623a590fc7c906cf02ce24fcf1e5310c4`
antes desta anotação. `python3 -m unittest -q
lab/parity/matrix/test_runner.py` passou 60 testes;
`python3 lab/parity/matrix/runner.py --validate-only` validou 43 casos;
`python3 lab/parity/matrix/runner.py --profile html --case P1489-S-001`
continuou `VIOLATED`, com oracle
`[CJK_JOINED,CJK_EXPLICIT_SPACE]` e cristalino
`[水 果,CJK_EXPLICIT_SPACE]`. O digest forward/reverse foi idêntico:
`720c4fc295a73ff9121907e70feffe45c7cd9b8321bc269f568c7e9529f27c25`.
Isto valida o comparador e confirma o baseline ainda aberto; não fecha
a lacuna de linguagem.

## Controles medidos antes da decisão

As células HTML abaixo são o texto dos parágrafos, preservando espaços
explícitos. A fonte dos controles vem da suite vanilla
`lab/typst-original/tests/suite/text/space.typ:83-213`, mas os valores são
das sondas bilaterais acima, não uma inferência a partir da suite.

| Controle HTML | Vanilla | Cristalino |
|---|---|---|
| ideogramas separados por newline | `水果` | `水 果` |
| espaço explícito entre ideogramas | `水 果` | `水 果` |
| comentário de linha entre ideogramas | `注释` | `注 释` |
| espaço explícito antes de comentário/newline | `空格注释` | `空格  注释` |
| newline, comentário de bloco e espaço | `水果` | `水  果` |
| newline entre palavras latinas | `new line` | `new line` |
| CJK → latim; latim → CJK | `换行newline`; `newline换行` | `换行 newline`; `newline 换行` |
| coreano; pontuação ideográfica + coreano | `줄 바꿈`; `쉼표，줄 바꿈` | `줄 바꿈`; `쉼표， 줄 바꿈` |
| pontuação chinesa; pontuação ambígua | `你好，你好。`; `“你好” “你好”` | `你好， 你好。`; `“你好” “你好”` |
| quebra explícita `\\` | `水<br>果` | `水 <br> 果` |

O controle de strong confirmou a fronteira semântica: no parágrafo
`*换*\n行`, vanilla emitiu `<strong>换</strong>行` e cristalino
`<strong>换</strong> 行`. Os outros parágrafos dessa sonda e os smart quotes
também diferem; não se assume que todas essas diferenças sejam causadas
somente pela origem do espaço.

| Controle de show/regex HTML | Vanilla | Cristalino |
|---|---|---|
| `new\nline` | `REGEX_KEPT_SPACE` | `new line` |
| `注\n释` | `REGEX_JOINED` | `注 释` |
| `换行\nnewline` | `REGEX_JOINED` | `换行 REGEX_JOINED` |
| `newline\n换行` | `REGEX_JOINED` | `REGEX_JOINED 换行` |

Na sonda dinâmica, `#let cjk = [水果]` em duas ocorrências separadas por
newline gerou `水果水果` no vanilla e `水果 水果` no cristalino; o controle
latino `fruit fruit` coincidiu. A variável `#let one-newline = [\n]` entre
`啊` e `啊` gerou `啊啊` no vanilla e `啊 啊` no cristalino; `[ ]` explícito
produziu `啊 啊` em ambos. `#assert.eq(one-newline, no-newline)` passou nos
dois binários. A diferença de saída após `#let` é medida; a obrigação de
transportar a origem pelo conteúdo dinâmico até a realização é a inferência
de desenho abaixo. O `#assert.eq` mede ausência de distinção no operador
de igualdade da linguagem, sem provar por si outros observáveis.

No PDF, os sentinelas da sonda `regex` foram extraídos por
`pdftotext -layout`: vanilla, em ordem,
`REGEX_KEPT_SPACE` + três `REGEX_JOINED`; cristalino,
`new line`, `注释`, `换行 REGEX_JOINED`,
`REGEX_JOINED 换行`. A diferença de aplicação de show existe nos dois
targets. O mesmo extrator escreveu `水果` para o primeiro controle CJK em
ambos os PDFs, apesar da diferença demonstrada no HTML; portanto texto
extraído de PDF não prova a largura do espaço nesses glifos. Na pontuação,
extraiu `你好，你好。` no vanilla e `你好， 你好。` no cristalino.

## Causa medida e classificação ADR-0107/0108

O lexer cristalino mantém o texto completo do token de espaço
(`01_core/src/compiler/lexer/mod.rs:163-177`), mas `eval_markup` converte
todo `SyntaxKind::Space` em `Content::Space` sem origem
(`01_core/src/compiler/eval/mod.rs:6970`). A variante pública é unitária
(`01_core/src/entities/content.rs:158-170`). Texto já atravessa
`intercept_content` ao ser avaliado
(`01_core/src/compiler/eval/mod.rs:6912-6918`), e o caminho regex atual
aceita apenas um `Content::Text` por vez
(`01_core/src/compiler/eval/rules.rs:978-1053`). Seu próprio contrato L0
preserva a fronteira de nós (`compiler/eval/rules.md`, P1285).

No vanilla pinado, `had_newline` é guardado ao avaliar o espaço
(`lab/typst-original/crates/typst-eval/src/markup.rs:97-107`); espaços
adjacentes combinam a origem por OR, e o descarte depende dos textos
vizinhos na realização (`typst-realize/src/spaces.rs:176-205`).
O agrupamento textual inclui `Text`, `Linebreak` e `SmartQuote`, admite
`Space` dentro do grupo e é interrompido por estilo
(`typst-realize/src/lib.rs:1034-1057`). A busca regex usa uma representação
textual com espaços virtualmente colapsados para procurar o match; o colapso
físico ocorre depois, quando há match (`typst-realize/src/lib.rs:1261-1325`).

Pelo ADR-0107, a junção CJK, a preservação do espaço explícito e a fatia
entregue à regra show são morfologia/semântica da linguagem. A variante Rust,
o número de passes, os bytes HTML/PDF e a ordem interna dos helpers são
mecânica. A igualdade positiva de `[\n]` e `[ ]` mede apenas o operador
`==`; a ausência da origem em `repr` vem de
`typst-library/src/text/space.rs:36-39`. Esta classificação vem depois da
medição bilateral, conforme ADR-0108; o código vanilla explica uma
implementação possível, mas não é por si prova da intenção para todos os
casos da suite.

**Inferência de desenho:** a origem deve atravessar conteúdo dinâmico até
uma normalização L1 comum aos targets, antes do casamento textual/regex
do grupo. Seria refutada por um carrier privado já existente que preservasse
essa origem através de `#let` e de show sem alterar a superfície de
`Content`, ou por uma sonda bilateral que mostrasse os mesmos resultados
com aplicação textual eager. Nenhum desses refutadores foi encontrado no
estado medido. Normalizar apenas no exporter HTML ou aplicar `trim` após
show não explica os sentinelas PDF nem a regex que cruza nós.

### Predicado Unicode e dependência L1

O vanilla pinado importa `codex::space_discarding::discard_space_between`
(`lab/typst-original/crates/typst-realize/src/spaces.rs:3`). Seu Cargo.toml
pina `codex` git rev `2e43ab9` (`lab/typst-original/Cargo.toml:51`);
`src/space_discarding.rs:53-73,194-256` dessa revisão usa propriedades
Unicode estáticas para classificar os caracteres internos adjacentes.
O cristalino usa `codex = "=0.3.0"` (`Cargo.toml:74`), cuja fonte local não
exporta `space_discarding`; chamar diretamente esse módulo não compila.

A solução **ratificada no L0** em 2026-09-24,
declara `icu_properties` 2.2.0 diretamente em L1 e autoriza na whitelist
`crystalline.toml` apenas consultas imutáveis de dados estáticos.
A versão 2.2.0 já está no `Cargo.lock:987-997` como transitiva;
`icu_segmenter` puro tem precedente em `crystalline.toml:121-122`.
Testes devem confrontar a classificação com o predicado vanilla pinado,
incluindo ideogramas, coreano, pontuação ideográfica, aspas ambíguas e
emoji. Essa escolha de biblioteca é mecânica; os observáveis de língua
continuam sendo a morfologia e as regras `show` medidas.

## Desenho ratificado no gate ADR-0127

1. Especificar no L0 de `entities/content.md` uma origem de newline para
   espaços produzidos por markup, mantendo o construtor `Content::Space`
   para espaços explícitos. A forma Rust exata precisa preservar `repr`,
   texto plano e `==` da linguagem, além da coerência mecânica entre
   `PartialEq` e `Hash` do Rust. A origem é causal, não um campo
   observável de `space.fields()`.
2. Em `compiler/eval.md`, capturar a origem do token sem antecipar a
   decisão CJK. Realizar os fluxos textuais em L1, colapsando espaços
   adjacentes e decidindo descarte pelas fronteiras textuais medidas, com
   resultado partilhado por HTML e PDF.
3. Em `compiler/eval/rules.md` e
   `compiler/eval/selector_matching.md`, aplicar show/regex ao texto
   realizado do grupo, preservando fronteiras de estilo, snapshots léxicos,
   precedência, guards e fatias de match. Atualizar `compiler/eval/repr.md`
   e os owners secundários abaixo antes de mudar seus consumers.
4. Testar RED separadamente: origem/normalização sem show e matching
   literal/regex atravessando nós `Text`. O GREEN deve distinguir
   `CJK_JOINED` de `CJK_EXPLICIT_SPACE` e manter controles latinos,
   coreanos, dinâmicos, strong, comentários, pontuação, smart quotes,
   `Linebreak` destrutivo e vizinhança não textual. A divergência de
   precedência lexical medida em `snapshot` precisa de teste ou
   scope-out explícito; não é fechada por estes controles. O comparador
   focal da matriz deve anteceder o rebaseline.

### Inventário dos dez owners secundários redigidos

A nova variante pública de `Content` alcança consumers produtivos além
dos cinco L0 primários. Foram redigidas seções P1494, ratificadas em 2026-09-24, em:

- `00_nucleo/prompts/compiler/columns.md`;
- `00_nucleo/prompts/compiler/introspect.md`;
- `00_nucleo/prompts/compiler/introspect/locatable.md`;
- `00_nucleo/prompts/compiler/layout.md`;
- `00_nucleo/prompts/compiler/layout/sequence.md`;
- `00_nucleo/prompts/compiler/math/layout/spacing.md`;
- `00_nucleo/prompts/infra/export/html.md`;
- `00_nucleo/prompts/infra/pipeline.md`;
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md`;
- `00_nucleo/prompts/infra/query-helpers.md`.

O owner produtivo indireto `00_nucleo/prompts/entities/content_hash.md`
também recebeu seção P1494: `hash_content` usa o Debug manual, que deve
conservar o mesmo hash para as duas origens iguais. São dezesseis L0s
produtivos no inventário (cinco centrais, dez secundários e este owner
de hash). `00_nucleo/prompts/compiler/eval/tests.md` é adicional,
mas test-only, não um novo consumer produtivo.

O L0 ratificado especifica `Content::SpaceFromNewline` como variante
pública e mudança de ordem eval→realização para show textual. São
gatilhos independentes de paragem nos itens 1 e 3 do ADR-0127.
**Estado do gate: confirmado explicitamente pelo dono em 2026-09-24**
para a variante pública e a mudança da ordem show/realização. O código L1–L4
está autorizado sob os L0s ratificados; este diagnóstico não o modifica.

## Fechamento factual P1494 — working tree não commitada

A validação ocorreu no HEAD
`6da06e849d75972651a342ad6aad337313451c79`, com working tree
não commitada. A janela reportada pelo integrador foi aproximadamente
22:46–22:55 UTC em 2026-09-24; não há timestamp por comando.
Antes da suíte, `git diff HEAD --stat` registrou 42 arquivos
rastreados, 1878 inserções e 64 remoções. A lista exata de caminhos
alterados, reconfirmada antes deste relato, era:

```text
 M 00_nucleo/materialization/typst-passo-1494.md
 M 00_nucleo/prompts/compiler/columns.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/eval/repr.md
 M 00_nucleo/prompts/compiler/eval/rules.md
 M 00_nucleo/prompts/compiler/eval/selector_matching.md
 M 00_nucleo/prompts/compiler/eval/tests.md
 M 00_nucleo/prompts/compiler/introspect.md
 M 00_nucleo/prompts/compiler/introspect/locatable.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/sequence.md
 M 00_nucleo/prompts/compiler/math/layout/spacing.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/content_hash.md
 M 00_nucleo/prompts/infra/export/html.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/query-helpers.md
 M 01_core/Cargo.toml
 M 01_core/src/compiler/eval/mod.rs
 M 01_core/src/compiler/eval/repr.rs
 M 01_core/src/compiler/eval/rules.rs
 M 01_core/src/compiler/eval/selector_matching.rs
 M 01_core/src/compiler/eval/tests.rs
 M 01_core/src/compiler/introspect.rs
 M 01_core/src/compiler/introspect/locatable.rs
 M 01_core/src/compiler/layout/columns.rs
 M 01_core/src/compiler/layout/mod.rs
 M 01_core/src/compiler/layout/sequence.rs
 M 01_core/src/compiler/math/layout/spacing.rs
 M 01_core/src/entities/content.rs
 M 01_core/src/entities/content_hash.rs
 M 03_infra/src/export/html.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/query_helpers.rs
 M Cargo.lock
 M Cargo.toml
 M crystalline.toml
 M lab/parity/matrix/manifest.yaml
 M lab/parity/matrix/runner.py
 M lab/parity/matrix/test_runner.py
?? 00_nucleo/diagnosticos/typst-p1494-gate-cjk-newline.md
```

O vanilla permaneceu no pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O cristalino `target/release/typst` usado na matriz tinha SHA-256
`b692397823a0e85c9c945feda329fb9687eef0cd6f30e91fc77caf99b04f6394`.
Ambos os hashes foram reconfirmados no fechamento.

### Comandos e resultados

```sh
CARGO_BUILD_JOBS=1 cargo test -p typst-core --lib --quiet
CARGO_BUILD_JOBS=1 cargo build
CARGO_BUILD_JOBS=1 cargo build --release
CARGO_BUILD_JOBS=1 cargo test -p typst-infra p1494_ --lib --quiet
CARGO_BUILD_JOBS=1 cargo test -p typst-infra --lib --quiet
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/matrix/test_runner.py
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/matrix/runner.py --validate-only
PYTHONDONTWRITEBYTECODE=1 python3 lab/parity/matrix/runner.py --profile html --case P1489-S-001 --vanilla lab/typst-original/target/release/typst --crystalline target/release/typst
crystalline-lint --fix-hashes .
crystalline-lint .
git diff --check
```

A suíte L1 passou 6174/6174, os builds `dev` e `release` passaram
e a suíte L3 focal P1494 passou 4/4. O comparador Python passou
60/60 testes; o manifest validou 43 casos. O caso `P1489-S-001`,
com `expected_state: MATCH`, terminou com código 0: oráculo e
cristalino deram `[CJK_JOINED,CJK_EXPLICIT_SPACE]`. Digest
determinístico: `a36fe2787915d2a4d76549abff8f6f2bb285519b59fd803ab10597986bd58bc8`.
Esse comparador focal exige os dois parágrafos exatos; não mede
todos os observáveis da suite de espaços.

A suíte infra integral passou 1067/1072; as cinco falhas restantes
são P1407 repeat/outline/justify. Para atribuição causal, o
integrador extraiu `git archive` do commit HEAD acima, sem qualquer
alteração P1494, em `/tmp/p1494-baseline-fnssTi` e executou:

```sh
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/repos/Antigravity/typst-crystalline/target cargo test --offline --manifest-path /tmp/p1494-baseline-fnssTi/Cargo.toml -p typst-infra p1407_ --lib -- --test-threads=1
```

O binário desse archive exato foi
`target/debug/deps/typst_infra-6654a47aec628dc4`, SHA-256
`08362babbbe0581a55f19285f579febed7188d4265f4c898fead2106ea42a663`.
A execução reproduziu 0/5, as mesmas cinco falhas e valores,
com código 101. Assim, as falhas P1407 já estão presentes no
commit P1493, sem código P1494. A reprodução foi posterior à
suíte, aproximadamente às 23:05 UTC; hora exata não capturada.

`crystalline-lint --fix-hashes .` ressellou 17 owners sem drift;
`crystalline-lint .` saiu com código 0 e zero violações críticas
V3/V4/V5/V14/V15/V26. `git diff --check` passou.
`cargo fmt --all -- --check` falhou em divergências amplas e
não foi usado como gate; este passo não afirma limpeza global
de formato.

### Dívida separada — show local CJK

Um ContentBlock bruto com `show` local literal `水果` sobre
`水`-newline-`果`, guardado e depois interpolado, produz
`<p>X</p>` no vanilla. O cristalino preserva agora a morfologia
bruta de `repr` e normaliza na inserção ao fluxo/root, mas ainda
não tem carrier de `show` local diferido para casar esse fragmento
depois da normalização. A combinação não é paridade fechada por
P1494 nem compõe o GREEN da matriz focal.
