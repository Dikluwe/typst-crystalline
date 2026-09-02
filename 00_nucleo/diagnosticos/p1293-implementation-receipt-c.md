# P1293 — recibo de implementação do Lote C

Estado: `LOT_C_RESIDUAL_CANDIDATE_GREEN_WORKSPACE_BLOCKED_BY_OUT_OF_SCOPE_B_STATE`.
Este recibo não aprova o Lote C, não constitui veredito do passo e não
autoriza o Lote D.

## Autoridade e segregação

Regime A/B segregado, executado sem atestação técnica de isolamento. O papel
desta sessão foi somente implementador serial do Lote C.

- manifesto SHA-256:
  `73fbab030e2eebf14b275914d851106582402b3c33f58e6a745c86b0c5e67e6d`;
- seal file SHA-256:
  `13abcda7c4b955b83b6694dc26840f9362fa0cae112f52f0696d95a96b289af7`;
- bloco canônico `serial_lot_c_html_seven_constructors` SHA-256:
  `223b1ddea355939115ab8e7acb57280dfad960911eea46e561b647de7f936ec0`;
- L0 lido integralmente:
  `00_nucleo/prompts/compiler/stdlib/html.md`, SHA-256
  `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de`;
- preimage do consumer: SHA-256
  `82f89b1f6ca1dedd183731936998ca844a5eceaf6be280c43a4011c184dad202`.

O implementador não leu nem executou `04_wiring/tests/p1293_contract.rs`, o
oracle, o RED receipt, gates de discriminação ou saídas privadas de testador,
atacante/verificador. A escrita ficou restrita a
`01_core/src/compiler/stdlib/html.rs` e a este recibo.

## RED próprio

Quatro testes próprios foram escritos no módulo `#[cfg(test)]` do owner antes
do corpo candidato. O comando
`cargo test -p typst-core p1293_c_ --lib` terminou inicialmente com exit
`101`: 34 diagnósticos `E0425`/`E0282` decorrentes somente da ausência das
tabelas e símbolos C (`BUTTON_ATTRS`, `COL_ATTRS`, `IFRAME_ATTRS`,
`SELECT_ATTRS`, `TEMPLATE_ATTRS`, `VIDEO_ATTRS`, `WBR_ATTRS`, `SANDBOX` e
`AUTOCOMPLETE`). Não houve falha alheia usada como RED.

Os testes discriminam:

- as sete identidades curtas no módulo;
- body `None` para cinco tags normais e `Unset`/rejeição de body para `col` e
  `wbr`;
- cardinalidade específica `14+1+10+7+5+11+0 = 48`, unicidade e ausência de
  duplicação dos 76 globais;
- ordem de chamada, Presence true/false, listas `sandbox`/`autocomplete` e
  globais representativos;
- domínios `>0` e `>=0`, enums fechados, cross-tag, `data-*`, erro e span.

## Implementação candidata

O dispatcher estático existente recebeu somente:

```text
html.button html.col html.iframe html.select
html.template html.video html.wbr
```

`button`, `iframe`, `select`, `template` e `video` reutilizam
`native_typed_html`; `col` e `wbr` reutilizam `native_typed_html_void`.
Foram adicionadas somente as sete tabelas específicas e os dois casts
internos necessários para inteiro estritamente positivo e inteiro não
negativo. Os outros casts reutilizam `Str`, `Presence`, `Enum`, `EnumList`,
`EnumOrStr` e `NoneEmptyOrEnum` existentes.

As tabelas possuem exatamente 48 entradas específicas nas cardinalidades
seladas. Todos os sete constructors continuam a concatenar os mesmos 76
globais. Não foi criado fallback de named, `data-*`, entidade, variante,
exporter, target, default, fase ou outro consumer.

## GREEN e gates unprotected

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core p1293_c_ --lib` | GREEN `4/4`, `5404` filtrados |
| `cargo test -p typst-core compiler::stdlib::html::tests --lib` | GREEN `21/21`, `5387` filtrados |
| `cargo test -p typst-core p1293_ --lib` | GREEN `42/42`, `5366` filtrados |
| `cargo check -p typst-wiring` | GREEN, exit `0` |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | GREEN, `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | GREEN, `Nothing to fix` |
| `rustfmt --edition 2021 --check 01_core/src/compiler/stdlib/html.rs` | GREEN |
| `git diff --check -- 01_core/src/compiler/stdlib/html.rs` | GREEN |

Os warnings já existentes do workspace permaneceram não fatais. Não houve
residual, crash, timeout ou `Unknown` nos testes próprios executados.

## Proveniência e hashes

- instante: `2026-09-02T12:00:58-03:00`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada: `55 files changed, 5735
  insertions(+), 687 deletions(-)` em `git diff HEAD --stat`;
- consumer final `01_core/src/compiler/stdlib/html.rs`: SHA-256
  `7f88674ef792b04994f6c5cdc98f43830a68f6ca37537071bce8e196cee3c7e3`.

Resultado restrito: candidato do Lote C implementado e GREEN apenas na
evidência unprotected acima. A aprovação permanece reservada à autoridade
independente. O hash deste recibo deve ser calculado externamente após a
gravação final.

## Checkpoint do replacement seal — serialização HTML

Em `2026-09-02T14:16:47-03:00`, o implementador retomou sob manifesto
SHA-256 `8c3ec13e0cf82f43f5c64bd577a20a1001422f052ed7cafd1502d68122fd6115`,
seal SHA-256 `b6466b5acd4b91fec5b876e0a141fb79d557d8d472d8adf0ba869be7a9430258`
e bloco canônico C SHA-256
`9b56d4f3c51548756df9d030b0ee14c24fc03fc362f9a04249a27a6c5967d010`.
Os sete L0s vigentes foram lidos integralmente e os sete preimages do selo
foram conferidos antes da escrita. O contrato protegido, oracles, REDs,
discrimination receipts e gates privados não foram lidos nem executados.

O RED próprio
`cargo test -p typst-shell p1293_c_html_serialization_compile_only_e_default_crystalline --lib`
terminou com exit `101`, exclusivamente por `E0609`/`E0433`: campo e enum
`HtmlSerialization` ainda ausentes. Antes desse comando já havia sido
materializado no owner L3 o teste próprio de escaping, portanto este checkpoint
não alega uma execução RED independente para esse teste específico.

Foram escritos, somente na allowlist, os seguintes candidatos parciais:

- L2: enum cru, wrapper exclusivo de `compile`, default crystalline e campo no
  `CompileIntent`; `watch` continua sem aceitar a flag;
- L1 html: união exata de `video.preload` e separação tipo/domínio dos inteiros;
- L1 repr: `HtmlElem` delegado ao formatter canônico curto/multiline;
- L3 exporter: enum e entry point explícitos, com default antigo cristalino e
  escaping vanilla contextual separado;
- L3 pipeline: início do threading do modo.

A implementação parou antes de `call_dispatch` e `wiring` por owner gap
objetivo: `03_infra/src/export/mod.rs:24` declara `mod html;` privado e
`:534` reexporta somente `export_html`. Assim, `pipeline.rs`, módulo irmão,
não pode nomear `export::html::HtmlSerializationMode` nem a nova função. O L0
atribui o enum ao owner `export/html.rs`; movê-lo para `pipeline.rs` violaria
ownership 1:1. Alterar ou acrescentar o reexport em `export/mod.rs` exige owner
produtivo fora da allowlist vigente. Nenhum contorno, API duplicada ou edição
fora do selo foi feito.

Hashes no STOP parcial:

- `call_dispatch.rs` (inalterado neste serial):
  `b0fd79d469a50ce7952ee08057f5a3b3e54be0fcd63cf97090a06cbff9ebe269`;
- `repr.rs`: `576141a2b2963ac392bc6944d8e8db46788fc7fdae7e8f5d435c4964f443d78a`;
- `stdlib/html.rs`:
  `78f9312601acb66156a0e26a515fcaff08fe66e13fabd318d49196b9410c8c4f`;
- `cli.rs`: `d3e9fecdfce6c1aad0f9badce9000f0b4de41888e299cad63522d25223b42989`;
- `export/html.rs`:
  `c081108b8518556c7fa920f9aabdc9e2f40facccfc90c7ed661ffc780cdaca9d`;
- `pipeline.rs`:
  `88dd7075b58e0f013c0942a72e52a7de7b104ebc087812a8a96f56b82ab7d167`;
- `wiring/main.rs` (inalterado neste serial):
  `cb25e0662f4f2f165059a599f77429c2dfc270f6a8f6740cab6ce7c609e3272f`.

Proveniência: `HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch
`Tekt`, working tree compartilhada não commitada com `61 files changed, 6330
insertions(+), 741 deletions(-)`. `git diff --check` focal nos cinco arquivos
tocados por este serial ficou GREEN. Gates finais não foram executados nem
alegados como aprovados por causa do STOP obrigatório.

## Fechamento sob o replacement seal final

Em `2026-09-02T14:57:14-03:00`, o implementador retomou e concluiu o
candidato C sob:

- manifesto `00_nucleo/diagnosticos/p1293-manifest.json`, SHA-256
  `a53cdb82874f38200835b75ef1e88019e11bc26c12665cfe726ccaa0846562de`;
- selo `00_nucleo/diagnosticos/p1293-contract-seal.json`, SHA-256
  `6d3c1102a105e701700c74080f25ec5f379fa2b6c1f0a9f0b8ec368aefeb35bc`;
- bloco canônico ativo `serial_lot_c_export_facade_final`, SHA-256
  `3650fb061a332ff2b2563cc32c2f728d4322a14da746fb25c6286e2143ed8065`.

Os oito L0s e respectivos preimages do selo foram conferidos antes da
continuação. O contrato protegido `04_wiring/tests/p1293_contract.rs`, os RED
receipts, discrimination receipts, oracle e saídas privadas de testador,
atacante ou verificador não foram lidos nem executados.

### Implementação final da allowlist C

- `call_dispatch.rs` reconhece somente as sete identidades nativas reais,
  preserva transitoriamente spans de chamada/posicionais/named/spread e
  reancora os diagnósticos sem reavaliar os argumentos;
- `repr.rs` usa o formatter canônico curto/multiline para `HtmlElem`;
- `stdlib/html.rs` contém exatamente os sete constructors e 48 atributos
  específicos, os 76 globais existentes, os domínios numéricos separados e a
  união fechada de `video.preload`;
- `cli.rs` transporta o enum cru `crystalline|vanilla`, com default
  `crystalline`, somente no subcomando `compile`;
- `export/html.rs` é o único owner de `HtmlSerializationMode`, mantém a API
  antiga com default cristalino e separa apenas o escaping contextual do modo
  vanilla;
- `export/mod.rs` recebeu somente o reexport direto de `export_html`,
  `export_html_with_serialization` e `HtmlSerializationMode`, mais seu teste
  próprio in-file; o submódulo `html` permaneceu privado e o header não mudou;
- `pipeline.rs` usa exclusivamente a facade `crate::export::{...}` e preserva
  as APIs antigas no modo cristalino;
- `wiring/main.rs` faz o mapping L2→L3 exaustivo somente no braço HTML. Os
  outros formatos ignoram o modo sem alterar suas fases ou semântica.

Nenhuma entidade, feature, inferência de target, default antigo, fase, owner
fora da allowlist ou item do Lote D foi alterado por este fechamento.

### RED→GREEN próprio e suites não protegidas

O RED CLI já registrado acima foi causado exclusivamente pela ausência do enum
e campo C. Depois da implementação, o primeiro comando agregado C encontrou
uma falha própria discriminatória de nome público (`found str` contra `found
string`); a correção causal passou a usar `vanilla_type_name`. Uma falha de
compilação posterior pertenceu somente ao teste próprio recém-adicionado
(`FileId` não importado) e foi corrigida no teste. O teste do exporter foi
adicionado depois do corpo candidato; portanto não se reivindica RED
independente executado para esse teste.

| Comando | Resultado final |
|---|---|
| `cargo test -p typst-core p1293_c_ --lib` | GREEN `8/8`, `5404` filtrados |
| `cargo test -p typst-core p1293_ --lib` | GREEN `46/46`, `5366` filtrados |
| `cargo test -p typst-core compiler::stdlib::html::tests --lib` | GREEN `23/23` |
| `cargo test -p typst-core compiler::eval::repr::tests --lib` | GREEN `67/67` |
| `cargo test -p typst-core compiler::eval::call_dispatch::tests --lib` | GREEN `4/4` |
| `cargo test -p typst-infra export::html::tests --lib` | GREEN `15/15` |
| `cargo test -p typst-infra p1293_c_facade_reexporta_api_sem_duplicar_tipo --lib` | GREEN `1/1` |
| `cargo test -p typst-shell --lib` | GREEN `62/62` |
| `cargo check -p typst-wiring` | GREEN, exit `0` |
| `cargo build --release --bin typst` | GREEN, exit `0` |
| `rustfmt --edition 2021 --check <oito consumers C>` | GREEN |
| `git diff --check -- <oito consumers C> <receipt C>` | GREEN |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | GREEN, `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | GREEN, `Nothing to fix` |

### Probes release próprios

Fonte pública própria `/tmp/p1293-c-own.typ`:

```typst
#html.button(value: "x&\"<>")[#text("<&>")]
```

Os três comandos `target/release/typst compile --features html --format html`
com modo omitido, `crystalline` e `vanilla` passaram. O default e crystalline
foram byte-idênticos, SHA-256
`c976ee7ba093d61ac396cab74f210ab8d6cbf5b24d2ef48040b28cdd5baabfeb`.
Vanilla produziu SHA-256
`182fdd9790389e33af1c5ad6479bc6e318afcd465122e92067cc02ea8c515fcc`:
texto `>` e `<`/`>` de atributo seguiram apenas sua regra contextual, enquanto
`&`, `<` de texto e `&`/`\"` de atributo continuaram escapados.

Os probes `typst eval --features html --format raw` confirmaram:

- `video(preload: none|auto|"metadata")`: três sucessos com attrs textuais
  `none|auto|metadata`;
- strings `"none"` e `"auto"`: erro exato
  `expected none, auto, or "metadata"`, ambas no range `25..31` da expressão;
- `html.col(span: 0)`: `number must be positive`, no valor;
- named desconhecido: `unexpected argument: spam`, no named completo;
- body excedente normal e body em void: `unexpected argument`, no positional
  ofensivo.

Dois compiles PDF da fonte própria `neutral`, um por modo, tiveram sucesso e
ambos extraíram exatamente `neutral` via `pdftotext`; os bytes PDF diferem pelo
identificador de instância aleatório já documentado e não foram usados como
prova de neutralidade. A neutralidade é adicionalmente coberta pelo teste L2 e
pelo match L4 exclusivo do braço HTML.

### Gates amplos bloqueados fora do escopo C

`cargo test -p typst-infra --lib` terminou com `910 passed / 8 failed`.
As oito falhas são todas regressões históricas de layout math na working tree B
compartilhada, sem dependência dos consumers HTML C:

```text
p1132e_cases_conteudo_preserva_centro_da_chave_sec09
p1132g_binom_sec07_sem_barra_e_com_posicao_vanilla
p1132k_sec12_limite_unilateral_preserva_altura_vanilla
p1132n_sec18_chave_externa_seleciona_variante_vanilla
p1132o_sec18_fracao_aninhada_suprime_spacing_em_script
p1132p_sec10_assembly_de_bracket_usa_attachment_central
p1133b_sec04_dif_com_sup_preserva_altura_vanilla
p1136_sec17_operadores_large_alinham_conteudo_adjacente_no_eixo
```

`cargo fmt --all -- --check` também falhou somente em arquivos B fora da
allowlist C (math layout/tests, attach-related, shaper/font_metrics e outros).
O implementador não os formatou nem corrigiu, pois o selo autoriza escrita
somente nos oito consumers C e neste recibo. Assim, o corpo C está completo e
seus gates focais estão verdes, mas este recibo não declara a working tree
globalmente verde nem aprova o lote.

### Proveniência final e hashes

- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- working tree compartilhada e não commitada no instante da medição:
  `63 files changed, 6598 insertions(+), 757 deletions(-)`;
- `01_core/src/compiler/eval/call_dispatch.rs`:
  `75399c76d99dfaeb9b515d3152f824dcb7dc8815e693f45a2be10b3d394c2759`;
- `01_core/src/compiler/eval/repr.rs`:
  `c0c604da25f573ac14b1cdeaeb55bfe490b827f594e059d5f22a69f7e66a1149`;
- `01_core/src/compiler/stdlib/html.rs`:
  `b6b339d0a7ef0a29a868ee1bbf671cc348c00e08cac123f0aa78f0fbe41df427`;
- `02_shell/src/cli.rs`:
  `d3e9fecdfce6c1aad0f9badce9000f0b4de41888e299cad63522d25223b42989`;
- `03_infra/src/export/html.rs`:
  `ad761fd3678b231f805868101664dc83f389d434d4cf912d7c29f275847377c4`;
- `03_infra/src/export/mod.rs`:
  `80234564bc8e47c6e42e76ee4f6380bcd46d7a535638601ea2b461d24719a16b`;
- `03_infra/src/pipeline.rs`:
  `72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088`;
- `04_wiring/src/main.rs`:
  `ca65db98f911f03c7e1772802dafc8003e50c0c3cfefcef7cc2249549c2004da`.

Nota operacional: uma busca para localizar o executável do linter usou por
engano `rg` sobre `00_nucleo` e atravessou `00_nucleo/materialization`,
imprimindo apenas linhas que continham o literal `crystalline-lint`. Nenhum
passo foi aberto ou usado na implementação e nenhum input protegido P1293 foi
exposto, mas a varredura contrariou a restrição operacional do repositório e é
registrada aqui sem reivindicar crédito.

Resultado restrito: candidato produtivo C completo na allowlist e evidência C
própria verde, aguardando juízo independente. Os dois gates globais acima
continuam bloqueados pela working tree B fora do escopo deste selo. Este recibo
não aprova o Lote C, não constitui veredito e não autoriza o Lote D.

## Fechamento residual C-P03..C-P05 / diagnóstico `none`

Em `2026-09-02T15:25:40-03:00`, a correção residual foi executada sob
manifesto SHA-256
`99f5e4a612408b0e91fcfc570e7146fffc5f5b0d5d56241bc3fae1504ec138e8`,
selo SHA-256
`d8c581bc5c6b47d7cf7087570225f2d34c98c675412409b7b036132d41078553`
e bloco canônico `serial_lot_c_p08_semantic_residual` SHA-256
`dbb255fa970acec4cd3d56fa44dd697a37acfa13ace28a1bfc7b1099e11d26cd`.
Os oito hashes L0 e os oito preimages produtivos coincidiram exatamente com o
selo antes da escrita. O regime permaneceu segregado sem atestação técnica de
isolamento; esta sessão atuou somente como implementador.

O contrato/oráculo protegido, RED/discrimination receipts e saídas privadas
não foram lidos nem executados. A escrita produtiva residual ficou restrita a
`01_core/src/compiler/eval/repr.rs` e
`01_core/src/compiler/stdlib/html.rs`; os demais seis consumers da allowlist
permaneceram byte-idênticos aos preimages.

### RED próprio e correção causal

Foram escritos antes do produto dois REDs próprios, um por owner:

| Comando RED | Resultado |
|---|---|
| `cargo test -p typst-core p1293_c_html_elem_usa_formatter_canonico_curto_e_multiline --lib` | exit `101`; `0/1`; attrs longos permaneceram lineares em vez da forma aninhada canônica |
| `cargo test -p typst-core p1293_c_referrerpolicy_nomeia_none_como_literal_de_linguagem --lib` | exit `101`; `0/1`; mensagem começou em `"no-referrer"` e omitiu o literal de linguagem `none` |

Nenhum dos REDs usou falha alheia. As correções são estritamente locais:

- `HtmlElem.attrs` deixa de concatenar `join(", ")` e delega seus campos ao
  mesmo `pretty_array_like` canônico já usado pelo tuple externo. Attrs curtos
  continuam lineares; attrs longos tornam-se multiline, com nesting e
  indentação preservados;
- `NoneEmptyOrEnum` continua aceitando somente `Value::None` ou os enums
  string fechados, mas a mensagem de string/tipo inválido é construída por
  `expected_none_or_enum`, que lista `none` sem aspas antes das alternativas.
  Nenhuma string `"none"` ou `"auto"` foi tornada válida para `video.preload`.

### GREEN e controles

| Comando | Resultado |
|---|---|
| os dois comandos RED acima, repetidos após a correção | GREEN `1/1` cada |
| `cargo test -p typst-core p1293_c_ --lib` | GREEN `9/9`, `5404` filtrados |
| `cargo test -p typst-core p1293_ --lib` | GREEN `47/47`, `5366` filtrados |
| `cargo test -p typst-core compiler::stdlib::html::tests --lib` | GREEN `24/24`, `5389` filtrados |
| `cargo test -p typst-core compiler::eval::repr::tests --lib` | GREEN `67/67`, `5346` filtrados |
| `cargo test -p typst-infra export::html::tests --lib` | GREEN `15/15` |
| `cargo test -p typst-infra p1293_c_facade_reexporta_api_sem_duplicar_tipo --lib` | GREEN `1/1` |
| `cargo test -p typst-shell p1293_c_ --lib` | GREEN `1/1` |
| `cargo check -p typst-wiring` | GREEN, exit `0` |
| `cargo build --release --bin typst` | GREEN, exit `0` |
| `rustfmt --edition 2021 --check <oito consumers C>` | GREEN |
| `git diff --check -- <oito consumers C> <receipt C>` | GREEN |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | GREEN, `No violations found` |
| `crystalline-lint --fix-hashes --dry-run .` | GREEN, `Nothing to fix` |

Os probes próprios no binário release confirmaram:

- `button` com dois attrs longos produz `attrs: (` multiline, um atributo por
  linha, indentação de quatro espaços e vírgula final; `col(span:2,id:"c")`,
  `wbr(id:"w")` e `button(value:"v")` permanecem lineares;
- `iframe(referrerpolicy:"invalid")` produz a lista fechada iniciada por
  `expected none, "no-referrer", ...`, ancorada no valor;
- `video(preload:"none")` e `video(preload:"auto")` continuam inválidos com
  `expected none, auto, or "metadata"`; `video(preload:none)` continua válido
  e serializa o atributo textual `none`.

### Hashes pós-correção residual

- `01_core/src/compiler/eval/call_dispatch.rs`:
  `75399c76d99dfaeb9b515d3152f824dcb7dc8815e693f45a2be10b3d394c2759`;
- `01_core/src/compiler/eval/repr.rs`:
  `ef68c178e15c5d87d74e71c9b1bf3113851b440fc3129bf074784289f88aa9c9`;
- `01_core/src/compiler/stdlib/html.rs`:
  `b1a990fa271c9f73939363d801e00b7915f0190ff44369d9bf58749b18f5eb78`;
- `02_shell/src/cli.rs`:
  `d3e9fecdfce6c1aad0f9badce9000f0b4de41888e299cad63522d25223b42989`;
- `03_infra/src/export/html.rs`:
  `ad761fd3678b231f805868101664dc83f389d434d4cf912d7c29f275847377c4`;
- `03_infra/src/export/mod.rs`:
  `80234564bc8e47c6e42e76ee4f6380bcd46d7a535638601ea2b461d24719a16b`;
- `03_infra/src/pipeline.rs`:
  `72f9c080b55ca295e5b51ae45acf527c76921cd06211a64c2c43fdd010af6088`;
- `04_wiring/src/main.rs`:
  `ca65db98f911f03c7e1772802dafc8003e50c0c3cfefcef7cc2249549c2004da`.

Proveniência: HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, branch `Tekt`, working tree
compartilhada não commitada com `63 files changed, 6656 insertions(+), 761
deletions(-)` no instante registrado. O incidente operacional de varredura
acidental descrito na secção anterior permanece registrado, não absolvido e
reservado à avaliação do verificador independente.

Resultado restrito: os dois resíduos semânticos C estão GREEN na evidência
própria; os bloqueios globais B já descritos permanecem fora do escopo. Este
recibo não aprova C, não constitui veredito e não autoriza D.

## Retificação final da ordem diagnóstica `referrerpolicy`

Em `2026-09-02T15:37:26-03:00`, ainda sob o mesmo selo residual SHA-256
`d8c581bc5c6b47d7cf7087570225f2d34c98c675412409b7b036132d41078553`,
foi reaberto somente o texto do diagnóstico `NoneEmptyOrEnum`. A redação
literal exigida é: primeiro todos os valores string aceitos, na ordem
contratada, e por último o valor de linguagem sem aspas, `, or none`. Esta
secção substitui somente a ordem diagnóstica descrita na secção residual
anterior; o reparo `repr` permaneceu byte-idêntico e GREEN.

O teste próprio foi ajustado antes do produto para comparar a mensagem completa
e o mesmo `Span::from_range(..., 23..32)`. O RED isolado
`cargo test -p typst-core p1293_c_referrerpolicy_nomeia_none_como_literal_de_linguagem --lib`
terminou com exit `101`, `0/1`: o candidato ainda emitia `expected none,
"no-referrer", ...`. Após alterar somente `expected_none_or_enum`, o mesmo
comando ficou GREEN `1/1` e passou a emitir exatamente:

```text
expected "no-referrer", "no-referrer-when-downgrade", "same-origin", "origin", "strict-origin", "origin-when-cross-origin", "strict-origin-when-cross-origin", "unsafe-url", or none
```

O erro por tipo mantém o mesmo prefixo e acrescenta `, found boolean`; ambos os
casos preservam o span fornecido. O helper não reconhece a grafia textual
`"none"`: `Value::None` continua sendo o único polo `none` deste cast.
`video.preload` permaneceu separado e seus controles confirmaram novamente que
strings `"none"`/`"auto"` são inválidas, enquanto `Value::None` é válido.

Gates focais desta retificação:

- teste diagnóstico: GREEN `1/1`;
- teste dos polos preload: GREEN `1/1`;
- `cargo test -p typst-core p1293_c_ --lib`: GREEN `9/9`;
- `cargo test -p typst-core p1293_ --lib`: GREEN `47/47`;
- suíte HTML L1: GREEN `24/24`;
- `cargo check -p typst-wiring`: GREEN;
- `cargo build --release --bin typst`: GREEN;
- `rustfmt --check` focal e `git diff --check` focal: GREEN;
- V5/V15/V26: `No violations found`;
- hash dry-run: `Nothing to fix`.

Probe próprio release reproduziu a mensagem exata acima no valor de
`iframe(referrerpolicy: "invalid")`. Os probes de preload conservaram as duas
rejeições e o sucesso de `preload: none`.

Hashes finais desta retificação:

- `01_core/src/compiler/eval/repr.rs` (inalterado):
  `ef68c178e15c5d87d74e71c9b1bf3113851b440fc3129bf074784289f88aa9c9`;
- `01_core/src/compiler/stdlib/html.rs`:
  `c6186114b04108e83b1382753bec4146aa126cf8055200e5695141881d95ff52`.

A working tree compartilhada tinha `63 files changed, 6667 insertions(+), 761
deletions(-)` no instante do probe. O incidente operacional anterior permanece
registrado e reservado ao verificador independente. Nenhum input protegido foi
lido/executado, nenhum owner B/D foi tocado e esta retificação não aprova o
Lote C.
