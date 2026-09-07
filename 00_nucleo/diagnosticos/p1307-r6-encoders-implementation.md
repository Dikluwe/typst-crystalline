# P1307-R6 — implementação de encoders e repr

Papel: implementador `/root/p1307_encoders_impl`, executado sem atestação de isolamento técnico. Sem autoridade para editar intenção, L0, contrato, baseline, oráculos ou veredito. Consumers exclusivos: loading.rs, stdlib/mod.rs e eval/repr.rs.

Predecessores comunicados pelo coordenador: selo SHA-256 `54a2cc86656c33734058aef77da8fe564721440529aed165dbdd140fb34ae08b`; RED independente `p1307-r6-red.json` SHA-256 `10caac2a33aa4bc13b920e8ac0230f4ffab45fa0cb549a244c8eefc2e2136713`. O coordenador liberou Rust somente após esse RED. Nenhuma expectativa de teste R6 foi lida ou escrita. Uma leitura inicial do baseline autorizado expôs seu diff histórico P1306 embutido; isso foi comunicado imediatamente ao coordenador.

## Medição A e decisão C

Em 2026-09-07T19:10:50Z, com HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais working tree não commitado em materialização, foi executado o harness `/tmp/p1307-yaml-emitter-probe.rs`, SHA-256 `edd06f581306072e0e9ac7029bb157d59c4031159bbab1b5ecc5b86b5dc22e4e`. Depende apenas do saphyr 0.0.6 previamente construído:

```text
rustc --edition=2021 /tmp/p1307-yaml-emitter-probe.rs -L dependency=/dev/shm/p1307-r4-target.8W1BEA/release/deps --extern saphyr=/dev/shm/p1307-r4-target.8W1BEA/release/deps/libsaphyr-f1d33205e32d9461.rlib -o /tmp/p1307-yaml-emitter-probe
/tmp/p1307-yaml-emitter-probe
input="true", saphyr="---\ntrue"
input="'true'", saphyr="---\n\"true\""
input="{x: 'true', y: [1, 2]}", saphyr="---\nx: \"true\"\ny:\n  - 1\n  - 2"
input="{x: \"a\\nb\\n\"}", saphyr="---\nx: |\n  a\n  b"
```

O comando vanilla `/usr/local/bin/typst eval 'yaml.encode((x: "true", y: (1, 2)))'` devolveu a string pública `"x: 'true'\ny:\n- 1\n- 2\n"`. Binário vanilla ratificado a51e02804, SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

A fonte saphyr `src/emitter.rs:197–200` emite sempre prefixo documental; `need_quotes/escape_str` escolhe aspas duplas e `emit_val` indenta sequências em mapas. Esses observáveis refutam o desenho A YAML mesmo retirando prefixo/sufixo. Uma árvore intermediária B não muda a capacidade de emissão. Foi adotado C: writer YAML privado no mesmo owner, conservando adapter serde de JSON/TOML. Não foi adicionada dependência, API global Serialize para Value, I/O ou fase.

## Candidato

JSON/TOML usam `TextualValue`, adapter local por referência com classificação recursiva; Bytes e opacos delegam à fachada canônica, Symbol usa Unicode e Content usa func seguido dos campos de presença. Snapshot Some é autoritativo; None/raw usam projeção pública existente. CBOR conserva sua classificação anterior.

Validação consome o primeiro positional, faz cast TOML, valida cada ocorrência causal de pretty, e rejeita o primeiro remanescente. Spans individuais são mantidos; Missing usa Args.span; diagnóstico positional named tem hint próprio.

Repr projeta State com chave/init, Counter por variante, Location opaca, qualquer With anônimo, Args Some causal com duplicatas e None legado. LocatedContent Some usa snapshot sem label; raw/None permanecem no formatter anterior. O hub só reexporta as três nativas, além das migrações sintéticas de tipos exigidas nos testes antigos.

Revisão estática anterior à QA encontrou classificação Unicode ampla demais na primeira redação YAML; a hipótese foi comunicada ao coordenador e corrigida para limites lexicais YAML explícitos, conservando NBSP como texto comum. Cargo check integrado iniciou próximo desse ajuste; o coordenador foi informado da possível corrida e decide a repetição. Nenhum build integrado foi iniciado por este implementador.

## Proveniência da working tree durante implementação

Snapshot posterior à medição A, ainda sem veredito, capturado antes da correção lexical final. A árvore é compartilhada e inclui edições simultâneas dos demais implementadores; os números abaixo descrevem o estado e não fecham comportamento.

```text
2026-09-07T19:14:58Z
b303f1f15b610e09872b567027e0d806387fde8c
 00_nucleo/prompts/compiler/eval.md                 |  44 +-
 00_nucleo/prompts/compiler/eval/bindings.md        |  18 +
 .../prompts/compiler/eval/bindings/field_access.md | 181 +++++++-
 .../compiler/eval/bindings/method_dispatch.md      |  31 ++
 .../compiler/eval/bindings/value_methods.md        |  36 ++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 136 ++++++
 00_nucleo/prompts/compiler/eval/closures.md        |  74 ++++
 00_nucleo/prompts/compiler/eval/math.md            |  52 +++
 .../prompts/compiler/eval/operators/arithmetic.md  |  31 ++
 .../prompts/compiler/eval/operators/equality.md    |  46 ++
 00_nucleo/prompts/compiler/eval/operators/join.md  |  57 +++
 00_nucleo/prompts/compiler/eval/repr.md            | 130 +++++-
 00_nucleo/prompts/compiler/eval/tests.md           | 167 ++++++-
 00_nucleo/prompts/compiler/introspect.md           |  44 ++
 00_nucleo/prompts/compiler/introspect/from_tags.md |  36 ++
 00_nucleo/prompts/compiler/introspect/heading.md   |  81 ++++
 00_nucleo/prompts/compiler/stdlib/_comum.md        |  41 +-
 00_nucleo/prompts/compiler/stdlib/collections.md   |  61 +++
 00_nucleo/prompts/compiler/stdlib/counter.md       |  27 ++
 .../prompts/compiler/stdlib/foundations/float.md   |  27 ++
 .../prompts/compiler/stdlib/foundations/int.md     |  33 ++
 .../prompts/compiler/stdlib/foundations/query.md   |  32 ++
 00_nucleo/prompts/compiler/stdlib/gradients.md     |  28 ++
 00_nucleo/prompts/compiler/stdlib/loading.md       | 163 +++++++
 00_nucleo/prompts/compiler/stdlib/numbering.md     |  27 ++
 00_nucleo/prompts/entities/args.md                 | 290 +++++++------
 00_nucleo/prompts/entities/func.md                 |  75 +++-
 00_nucleo/prompts/entities/introspector.md         |  40 ++
 00_nucleo/prompts/entities/value.md                |  63 +++
 00_nucleo/prompts/infra/query-helpers.md           |  51 +++
 00_nucleo/prompts/shell/cli.md                     |  49 +++
 01_core/src/compiler/eval/bindings/field_access.rs | 103 +++--
 .../src/compiler/eval/bindings/method_dispatch.rs  |   6 +-
 01_core/src/compiler/eval/bindings/mod.rs          |   4 +-
 .../src/compiler/eval/bindings/value_methods.rs    |  81 +++-
 01_core/src/compiler/eval/call_dispatch.rs         | 122 ++++--
 01_core/src/compiler/eval/closures.rs              |  29 +-
 01_core/src/compiler/eval/math.rs                  |  84 ++--
 01_core/src/compiler/eval/mod.rs                   |  27 +-
 01_core/src/compiler/eval/operators/arithmetic.rs  |   1 +
 01_core/src/compiler/eval/operators/equality.rs    |  65 ++-
 01_core/src/compiler/eval/operators/join.rs        |  16 +-
 01_core/src/compiler/eval/repr.rs                  |  53 ++-
 01_core/src/compiler/eval/tests.rs                 | 478 ++++++++++++++++++++-
 01_core/src/compiler/introspect.rs                 |  11 +-
 01_core/src/compiler/introspect/from_tags.rs       |  11 +-
 01_core/src/compiler/introspect/heading.rs         |  96 +++++
 01_core/src/compiler/stdlib/collections.rs         |  80 +---
 01_core/src/compiler/stdlib/counter.rs             |  10 +-
 01_core/src/compiler/stdlib/foundations/float.rs   |  30 +-
 01_core/src/compiler/stdlib/foundations/int.rs     |  46 +-
 01_core/src/compiler/stdlib/foundations/query.rs   |   2 +-
 01_core/src/compiler/stdlib/gradients.rs           |  20 +-
 01_core/src/compiler/stdlib/loading.rs             | 409 ++++++++++++++++++
 01_core/src/compiler/stdlib/mod.rs                 |  25 +-
 01_core/src/compiler/stdlib/numbering.rs           |  18 +-
 01_core/src/compiler/stdlib/plugin.rs              |   8 +-
 01_core/src/compiler/stdlib/shapes.rs              |  15 +-
 01_core/src/compiler/stdlib/state.rs               |   3 +-
 01_core/src/compiler/stdlib/structural/math.rs     |  15 +-
 01_core/src/compiler/stdlib/structural/mod.rs      |  26 +-
 01_core/src/entities/args.rs                       | 121 +++++-
 01_core/src/entities/introspector.rs               |   4 +-
 01_core/src/entities/value.rs                      |  33 +-
 02_shell/src/cli.rs                                |  76 +++-
 03_infra/src/query_helpers.rs                      |  29 +-
 66 files changed, 3788 insertions(+), 540 deletions(-)

edd06f581306072e0e9ac7029bb157d59c4031159bbab1b5ecc5b86b5dc22e4e  /tmp/p1307-yaml-emitter-probe.rs
c56c6ac57010e2362b8e5b5fe5fab618f7a52a9ac53c36e1cc974a8991f28161  01_core/src/compiler/stdlib/loading.rs
2e67d31dd3d3f648789473716ffc118839ad6b232f5fb4937714d9a12c8801cd  01_core/src/compiler/stdlib/mod.rs
36cfb0c8a3ca5f3c9ab71fdc09a82d6f753dd92ef201874f5e2f6001559a1d10  01_core/src/compiler/eval/repr.rs
7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8  /usr/local/bin/typst
```

Verificação própria limitada: rustfmt nos três consumers e git diff --check, ambos sem erros. Gates funcionais, ataques e veredito pertencem ao coordenador/verificador independente. Este relatório não declara GREEN, preservação geral ou equivalência de todo o domínio.

## Revisão de projeção raw antes de check3

O coordenador apontou que o helper legado de métodos não é uma enumeração
completa de campos. A leitura de `bindings/field_access.rs:634–655` confirmou
que ele enumera somente campos de Heading/Equation/Text/Linebreak e body como
fallback. `entities/content.rs:3522` expõe Metadata.value, mas o helper nunca
pergunta value; Sequence não fornece children via get_field. Logo a partição
de Value não demonstrava completude da projeção de Content.

Foi adicionado um match privado anterior ao helper para dados cuja presença
é estrutural: Empty vira sequence com children vazio; Sequence transporta
children recursivos; Metadata transporta value; Styled transporta child e a
representação pública opaca `styles(..)`; Raw transporta text/lang presente;
Link transporta dest/body; Box e Block transportam body; HtmlElem transporta
tag e attrs/body com a presença armazenada. Some snapshot continua autoritativo.
Fontes ratificadas: content/mod.rs:722–727 e :760–766; metadata.rs:27.

A medição própria vanilla executou:

```text
/usr/local/bin/typst eval '([], [a b], metadata((x: 1)), strong[abc], emph[abc], linebreak(), raw("a"), link("https://example.com")[x], box[x], [#text(fill: red)[x]]).map(json.encode.with(pretty: false))'
```

As formas medidas incluem `{func:sequence,children:[]}`, Metadata com value,
Raw com text, Link com dest/body, Box com body e Styled com child/styles.
Esses exemplos descrevem o mapa; a medição preservou JSON integral no output
da chamada de ferramenta. A fonte/projeção motivou revisão, não o acesso a
oráculos privados. O coordenador autorizou a revisão antes de check3.

Limitação explícita: payloads legados como Raw.block armazenam o valor default
sem máscara de presença; Box/Block também têm cosméticos baked. Esta revisão
não transforma defaults desses payloads em campos explicitamente presentes,
não amplia entidades e não prova paridade de tais entradas. Qualquer obrigação
congelada que exija dados de presença irrecuperáveis continua bloqueante;
não pode ser omitida para declarar sucesso. Outros campos sem enumeração
própria mantêm a projeção pública legada. Não há alegação de completude de
todas variantes internas de Content a partir deste match delimitado.

Antes do build de testes, o coordenador autorizou duas correções finais:
renome mecânico local raw para raw_elem, para compatibilidade com o parser do
linter (rustc já aceitava o código); e Args Some usando pretty_array_like,
conforme fonte ratificada args.rs:455–459, preservando o join legado de None.
Os consumers foram então congelados para medição integrada independente.

## Redesenho após V14: árvores privadas por formato

O linter integral do coordenador identificou que serde, embora já presente
no Cargo.toml, não pertence à whitelist externa de L1. O adapter inicial
implementava esse trait e por isso violava V14. A presença em Cargo foi uma
premissa insuficiente da implementação; nenhuma whitelist foi alterada.

Antes de editar, o implementador comunicou o redesenho: converter por
referência Value nas árvores serde_json::Value e toml::Value já autorizadas,
com as regras próprias dos formatos e sem referência direta a serde. O
coordenador autorizou a janela depois do primeiro resultado funcional.
JSON preserva números e converte não finitos em null; TOML omite None nas
entradas de dict e recusa None em array com a mensagem contratada. Campos
Content, snapshot e fallback continuam compartilhando a mesma projeção
privada anterior. O writer YAML C não foi alterado nesta revisão.

O candidato substituiu integralmente TextualValue/Serialize por
textual_json_value/textual_toml_value. Hash de loading antes de resselo de
linhagem pelo coordenador:
`da9defbb076893154664ba594a39928402f393290ff5250fc5cf4de20aa9b853`.
Essa árvore por formato é uma alternativa interna prevista no L0 e não
introduz tipo público, crate nova, alteração de fase ou mudança de contrato.
Rustfmt e diff-check focal passaram; a equivalência do novo emissor com o
candidato anterior continua a cargo da repetição funcional independente.

## Revisão focal após matriz pública 1 — hipótese anterior ao patch

O coordenador forneceu integração autorizada de actual e expectativa pública
aprovada, limitada aos casos escapes TOML e float-max/float-exponent JSON/YAML.
O recibo `p1307-r6-public-matrix-1.json` identifica o candidato SHA-256
`b10f891342f70bb6964d7881abaa31cc7ca7e330da6c1c185e528fe5e820ff30`
e oracle SHA-256
`99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6`.
As células pertinentes mostram exponentes `e+308`/`e+20` contra `e308`/`e20`,
e aspas duplas cruas dentro de string TOML basic multilinha contra `\"`.
As expectativas não foram alteradas. Isso refuta a suficiência da emissão
direta dos backends nas classes desses tokens.

Hipótese comunicada antes de implementar: um adaptador lexical privado na
saída JSON identifica números fora de strings e retira somente o sinal
positivo do expoente; o writer YAML aplica a mesma operação diretamente no
token numérico. Um adaptador lexical TOML distingue strings basic/literal e
simples/multilinha, preserva escapes existentes e escapa aspas internas
somente no corpo basic multilinha. Delimitadores, conteúdo das strings,
ordem e espaçamento externo ficam sob o backend anterior. Não usar replace
global, inferência por texto de fixture, dependência serde ou nova crate.
