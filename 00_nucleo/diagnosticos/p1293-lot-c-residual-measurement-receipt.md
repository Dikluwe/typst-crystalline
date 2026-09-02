# P1293 — recibo independente de medição causal dos resíduos do Lote C

**Papel:** medidor/diagnosticador causal independente  
**Regime:** protocolo Tekt completo; segregado por capacidades e artefatos,
sem isolamento técnico de leitura  
**Escrita autorizada e realizada:** somente este recibo diagnóstico  
**Escritas não realizadas:** produto, Prompt L0, contrato, oráculo, manifesto,
selo, veredito e artefatos do Lote D

## 1. Proveniência reproduzível

- instante da captura final: `2026-09-02T12:14:19-03:00`;
- `HEAD`: `7dd25ff0e222b6c7c640d6bc7957b98f94227507`;
- estado: working tree não commitada;
- SHA-256 de `git status --short` antes de criar este recibo:
  `8c3182af46ea5f32559581b81129233238a7fbf3c7c80e5426df20c031147308`;
- SHA-256 de `git diff HEAD --stat`:
  `16f834256e272a978c5435425eeb605157ea4f7ce9028f9f82cbfa234f52df3a`;
- manifesto recebido:
  `00_nucleo/diagnosticos/p1293-manifest.json`, SHA-256
  `73fbab030e2eebf14b275914d851106582402b3c33f58e6a745c86b0c5e67e6d`;
- selo encontrado no estado medido:
  `00_nucleo/diagnosticos/p1293-contract-seal.json`, SHA-256
  `13abcda7c4b955b83b6694dc26840f9362fa0cae112f52f0696d95a96b289af7`;
- contrato protegido medido:
  `04_wiring/tests/p1293_contract.rs`, SHA-256
  `6f695c54582dbfbf335da7ab6d69324a712494a21b4aa725c73369eaa04a85b5`;
- vanilla ratificado: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- candidato executado: `target/debug/typst`, SHA-256
  `b9d02544caa4537c4999bb7166a520e4de82cec80c0e5a9a0597039fe5625612`.

Fontes causais e seus hashes no instante medido:

| Prompt/consumer ou fonte | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/eval/repr.md` | `d9499bbbc889eb42d485c700823cd428950bedefda4496065488a678c6e8b500` |
| `01_core/src/compiler/eval/repr.rs` | `9c5316ecaa9332379bad5afa1c1e8de1e504e77d085c91b475480591b30a3d1b` |
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `f683a20d0171fe82983ac20886b79d09710ce1c38b036a43e8ac56e58e8959f7` |
| `01_core/src/compiler/eval/call_dispatch.rs` | `4d7002a7475fb8999025fbb28a2381647557ad0617697c52113c34ed9cd1664b` |
| `00_nucleo/prompts/compiler/stdlib/html.md` | `66aeb6c2c84ca0ab4c6ffff99b97c22de02a2c438ca9a40c797ee4237d5229de` |
| `01_core/src/compiler/stdlib/html.rs` | `7f88674ef792b04994f6c5cdc98f43830a68f6ca37537071bce8e196cee3c7e3` |
| `00_nucleo/prompts/infra/export/html.md` | `b5560e10cc7161551a8c1bba38f8747bde059367550162834dbffd2b3c9715cd` |
| `03_infra/src/export/html.rs` | `49324678c2e37464fa813ce87e2d78156a19cc17eaf83bb88b2173f27de0bf12` |
| `00_nucleo/prompts/entities/args.md` | `3a33f1f2628348e3484b8346ab55aa82474b789e6836c62303b24b43297f5d56` |
| `01_core/src/entities/args.rs` | `c4390cee53010c891656e33dc08159a597f99b10f8f9a1a54e9eb67acd15155a` |
| asset HTML pinado `94dcb99/files/html/data.rs` | `593214ab0d92d12c9ef603a983958a432f46be278d805152d38a65818fb266b8` |
| `lab/typst-original/crates/typst-html/src/typed.rs` | `b702bfe01ab81ab238a96bcbf93302bbd4eba9875c7d996354e89428f07c67f7` |
| `lab/typst-original/crates/typst-html/src/encode.rs` | `9176b861480dc799245f95445dedbfed6f454e4982e69933faa2bc76493ba0ef` |
| `lab/typst-original/crates/typst-html/src/charsets.rs` | `4971d652ab985bb37e23a854acf7457b870769e9bcadf583d7e6a66740bcabaf` |

Estado exato antes de criar este recibo:

```text
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/eval/bindings/field_access.md
 M 00_nucleo/prompts/compiler/eval/call_dispatch.md
 M 00_nucleo/prompts/compiler/eval/math.md
 M 00_nucleo/prompts/compiler/eval/repr.md
 M 00_nucleo/prompts/compiler/eval/tests.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/helpers.md
 M 00_nucleo/prompts/compiler/layout/text.md
 M 00_nucleo/prompts/compiler/math/layout/_comum.md
 M 00_nucleo/prompts/compiler/math/layout/attach.md
 M 00_nucleo/prompts/compiler/stdlib/foundations/float.md
 M 00_nucleo/prompts/compiler/stdlib/html.md
 M 00_nucleo/prompts/compiler/stdlib/math_style.md
 M 00_nucleo/prompts/compiler/stdlib/structural/math.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/elements/math_attach.md
 M 00_nucleo/prompts/entities/layout_types.md
 M 00_nucleo/prompts/entities/style_chain.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/shell/cli.md
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/call_dispatch.rs
 M 01_core/src/compiler/eval/math.rs
 M 01_core/src/compiler/eval/mod.rs
 M 01_core/src/compiler/eval/repr.rs
 M 01_core/src/compiler/eval/tests.rs
 M 01_core/src/compiler/layout/equation.rs
 M 01_core/src/compiler/layout/helpers.rs
 M 01_core/src/compiler/layout/text.rs
 M 01_core/src/compiler/math/layout/accent.rs
 M 01_core/src/compiler/math/layout/attach.rs
 M 01_core/src/compiler/math/layout/cancel.rs
 M 01_core/src/compiler/math/layout/cases.rs
 M 01_core/src/compiler/math/layout/frac.rs
 M 01_core/src/compiler/math/layout/matrix.rs
 M 01_core/src/compiler/math/layout/mod.rs
 M 01_core/src/compiler/math/layout/root.rs
 M 01_core/src/compiler/math/layout/spacing.rs
 M 01_core/src/compiler/math/layout/tests.rs
 M 01_core/src/compiler/math/layout/underover.rs
 M 01_core/src/compiler/math/layout/vec.rs
 M 01_core/src/compiler/stdlib/foundations/float.rs
 M 01_core/src/compiler/stdlib/html.rs
 M 01_core/src/compiler/stdlib/math_style.rs
 M 01_core/src/compiler/stdlib/structural/math.rs
 M 01_core/src/entities/content.rs
 M 01_core/src/entities/elements/math_attach.rs
 M 01_core/src/entities/layout_types.rs
 M 01_core/src/entities/style_chain.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/export/tests.rs
 M 03_infra/src/font_metrics.rs
 M 03_infra/src/shaper.rs
?? -
?? 00_nucleo/diagnosticos/p1293-attach-ic-residual-measurement-receipt.md
?? 00_nucleo/diagnosticos/p1293-attach-layout-causal-measurement-receipt.md
?? 00_nucleo/diagnosticos/p1293-authoring-baseline-status.txt
?? 00_nucleo/diagnosticos/p1293-baseline-status.txt
?? 00_nucleo/diagnosticos/p1293-carrier-own-test-residual-audit.md
?? 00_nucleo/diagnosticos/p1293-cli-borrow-discrimination-receipt.json
?? 00_nucleo/diagnosticos/p1293-contract-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-attach-ic-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-attach-refutator-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-empty-markup-carrier-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-equation-frame-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-independent-red-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-layout-owner-gap-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-spans-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-stale-regression-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-textitem-provenance-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-textitem-ssty-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-b-textitem-style-axis-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-morphology-b-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-reopen-span-receipt.md
?? 00_nucleo/diagnosticos/p1293-contract-seal.json
?? 00_nucleo/diagnosticos/p1293-empty-markup-carrier-measurement-receipt.md
?? 00_nucleo/diagnosticos/p1293-implementation-receipt-a.md
?? 00_nucleo/diagnosticos/p1293-implementation-receipt-b.md
?? 00_nucleo/diagnosticos/p1293-implementation-receipt-c.md
?? 00_nucleo/diagnosticos/p1293-lot-b-verification-receipt.json
?? 00_nucleo/diagnosticos/p1293-manifest.json
?? 00_nucleo/diagnosticos/p1293-pre-gate-l0-receipt.md
?? 00_nucleo/diagnosticos/p1293-preseal-discrimination-receipt.json
?? 00_nucleo/diagnosticos/p1293-red-tests-receipt.md
?? 00_nucleo/diagnosticos/p1293-textitem-ic-residual-measurement-receipt.md
?? 00_nucleo/diagnosticos/p1293-textitem-spacing-residual-measurement-receipt.md
?? 00_nucleo/diagnosticos/p1293-vanilla-measurement-receipt.md
?? 00_nucleo/materialization/typst-passo-1293.md
?? 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml
?? 00_nucleo/prompts/wiring/tests/p1293_contract.md
?? 04_wiring/tests/p1293_contract.rs
```

## 2. Comandos e resultados

Comandos principais, sempre com o contrato protegido byte-idêntico ao hash
acima:

```text
cargo test -p typst-wiring --test p1293_contract \
  p1293_c_surface_all_specific_casts_and_closed_errors -- --exact --nocapture
=> FAIL, exit 101; 0 passed / 1 failed / 8 filtered; 44/46 casos internos divergentes

cargo test -p typst-wiring --test p1293_contract \
  p1293_c_dom_order_void_escaping_and_nesting -- --exact --nocapture
=> FAIL, exit 101; 0 passed / 1 failed / 8 filtered; 1 violação nominal

cargo test -p typst-wiring --test p1293_contract \
  p1293_c_feature_and_target_are_independent_axes -- --exact --nocapture
=> PASS, exit 0; 1 passed / 0 failed / 8 filtered

cargo test -p typst-core p1293_c_ --lib
=> PASS, exit 0; 4 passed / 0 failed / 5404 filtered
```

Hashes dos logs transitórios completos usados na classificação:

```text
surface  9766c30c3835ab8926b8fc8757e55ce74713b6844400a0ed74c269ea1b737d80
DOM      03bc534ebbce77377a96bb7af260a3c622b6cbc30d3a5887ecda3d47253c8481
feature  c1fd612f834ffec00beec9472e264876fa9a29670046dd14ceffc349f014ea99
core     9b71cfbcc230220a2f56e4701685980fb63d6dfdeb9a9909b4463edf352a8acd
```

Também foram executadas bilateralmente, com `--format json --features html`,
as sondas de fronteira de `repr`, `video.preload` e `iframe.referrerpolicy`, e
foi compilada no vanilla a fixture de escaping com:

```text
printf '%s' '#html.button(value:"x&\"<>")[#text("<&>")]' |
  /usr/local/bin/typst compile - - --format html --features html
```

## 3. Resultado nominal da superfície C

A matriz protegida contém **46 casos**. Passaram byte a byte apenas
`C-P01-P02` (presença, tipo, nomes e chamadas vazias) e
`C-P04-col-all` (repr curto). Divergiram **44/46**:

### 3.1 Sete positivos: somente forma multiline de `repr`

`C-P03-button-all`, `C-P04-iframe-all`, `C-P04-select-all`,
`C-P04-template-all`, `C-P04-video-all`, `C-P03-wbr-global` e
`C-P05-presence-order` têm os mesmos campos, valores, ordem e morfologia
elementar, mas o vanilla usa a forma pública multiline e o candidato achata
tudo numa linha.

Exemplos de fronteira reproduzidos:

```text
repr(html.col(span:2,id:"c"))
  ambos => elem(tag: "col", attrs: (span: "2", id: "c"))

repr(html.wbr(id:"w"))
  ambos => elem(tag: "wbr", attrs: (id: "w"))

repr(html.button(value:"v"))
  ambos => elem(tag: "button", attrs: (value: "v"), body: none)

repr(html.button(value:"v",name:"n"))
  vanilla => forma multiline, um campo por linha e vírgula final
  candidato => forma compacta numa linha
```

Logo a regra não é “todo HtmlElem é multiline”: o limite canônico continua
a preservar as três formas curtas acima. A causa direta está em
`01_core/src/compiler/eval/repr.rs:580-597`: `Content::HtmlElem` usa dois
`join(", ")` e não usa a disciplina canônica já implementada em
`repr.rs:1087-1140` (`pretty_comma_list`/`pretty_array_like`, fronteira ASCII
de 50). O owner provável e suficiente desta morfologia é
`compiler/eval/repr.md` → `compiler/eval/repr.rs`.

**Classificação ADR-0107/0108:** `repr` é morfologia pública da linguagem, não
igualdade Rust nem preferência estética. A inferência de owner seria refutada
se um `HtmlElem` já chegasse com perda de campo/ordem, ou se uma sonda curta
também divergisse; nenhum desses refutadores ocorreu.

### 3.2 Trinta e sete negativos: span agregado em todos

Todos os **37 casos negativos** preservam a polaridade de falha, mas todos
divergem no intervalo sublinhado. São:

- 21 named desconhecidos/data/cross-tag:
  `C-N-{button,col,iframe,select,template,video,wbr}-{unknown,data,cross}`;
- 4 formas de body/aridade:
  `C-N-col-body`, `C-N-wbr-body`, `C-N-normal-extra-body`,
  `C-N-normal-named-body`;
- 12 casts:
  `C-N-string-cast`, `C-N-presence-cast`, `C-N-enum-cast`,
  `C-N-target-cast`, `C-N-positive-zero`, `C-N-positive-negative`,
  `C-N-nonnegative`, `C-N-none-enum`, `C-N-enum-list`,
  `C-N-autocomplete-list`, `C-N-template-enum`, `C-N-video-enum`.

Em **29/37**, a mensagem é igual e somente o span diverge: os 21 named, os 4
body/aridade e `enum-cast`, `enum-list`, `autocomplete-list`, `template-enum`.
O vanilla ancora named desconhecido/body named no named completo, erro de cast
no valor, body void no positional e aridade excedente no segundo positional.
O candidato ancora tudo na lista agregada de argumentos.

A causa direta é verificável em `html.rs:819-843,859-862`: todos os caminhos
recebem apenas `args.span`. `Args` conserva deliberadamente só o span agregado
em `entities/args.rs:18-28`; o seu L0 vigente explicita em
`entities/args.md:36-48` que não há span por argumento. Entretanto,
`call_dispatch.rs:139-189` já demonstra a captura transitória de positional,
named completo e valor named antes de `eval_args`; e
`call_dispatch.rs:1450-1489` seleciona a âncora por identidade resolvida antes
de delegar. Portanto o owner provável e suficiente dos spans HTML é
`compiler/eval/call_dispatch.md` → `compiler/eval/call_dispatch.rs`, sem mudar
o contrato público de `Args`.

**Exclusão causal:** `entities/args.md`/`args.rs` não é necessário nem
recomendado. Alterá-lo ampliaria o contrato público e contrariaria o precedente
estreito vigente. A inferência de `call_dispatch` seria refutada se a AST não
expusesse os spans antes de `eval_args`, se o callee ainda não tivesse
identidade resolvida ou se a falha nascesse antes da chamada nativa; as linhas
citadas e os 37 resultados refutam essas alternativas para os casos medidos.

### 3.3 Oito mensagens além do span

Em **8/37** negativos há também divergência textual:

| Caso | Vanilla | Candidato |
|---|---|---|
| `C-N-string-cast` | `expected string, found integer` | `expected string, found int` |
| `C-N-presence-cast` | `expected boolean, found string` | `expected boolean, found str` |
| `C-N-target-cast` | termina em `found integer` | termina em `found int` |
| `C-N-positive-zero` | `number must be positive` | `expected positive integer, found int` |
| `C-N-positive-negative` | `number must be positive` | `expected positive integer, found int` |
| `C-N-nonnegative` | `number must be at least zero` | `expected non-negative integer, found int` |
| `C-N-none-enum` | enum termina em `or none` | omite `none` e cita `"unsafe-url"` como último membro |
| `C-N-video-enum` | `expected none, auto, or "metadata"` | trata os três como strings citadas |

As mensagens são produzidas em `html.rs:864-910,969-999,1044-1052`.
`type_error` usa `Value::type_name()` curto; positive/nonnegative fundem falha
de tipo e de domínio; e os formatters de enum não representam os valores de
linguagem `none`/`auto`. O owner provável e suficiente das mensagens e casts é
`compiler/stdlib/html.md` → `compiler/stdlib/html.rs`. Mudar globalmente
`Value::type_name()` não é necessário e arriscaria diagnósticos alheios.

**Classificação ADR-0107/0108:** mensagem e intervalo do diagnóstico são o
observável da linguagem nestes casos, logo a igualdade do transcript é gate
legítimo. A inferência seria refutada se a mensagem fosse reescrita depois de
`html.rs`, ou se a fonte vanilla usasse um formatter global incompatível com
um ajuste local; o transcript e o ponto de criação do `SourceDiagnostic`
mostram o contrário.

## 4. Resíduo semântico adicional: `video.preload`

A fonte pinada prova em
`94dcb99/files/html/data.rs:1445-1449` que `preload` é a união
`Type::None | Type::Auto | Type::Strings(139,140)`. O índice 139 começa em
`data.rs:1846` e contém somente `"metadata"`. `typed.rs:176-206` converte os
tipos e `typed.rs:466-475` serializa `AutoValue`/`NoneValue` como `auto`/`none`.

Sondas bilaterais frescas:

| Expressão | Vanilla | Candidato |
|---|---|---|
| `repr(html.video(preload:none))` | sucesso, atributo `"none"` | erro |
| `repr(html.video(preload:auto))` | sucesso, atributo `"auto"` | erro |
| `repr(html.video(preload:"none"))` | erro | sucesso |
| `repr(html.video(preload:"auto"))` | erro | sucesso |
| `repr(html.video(preload:"metadata"))` | sucesso | sucesso |

Isto não é só mensagem: há **4 diferenças de aceitação/payload**. O contrato
protegido atual inclui apenas `preload:"bad"`; a sua mensagem esperada denuncia
a união correta, mas não mata uma implementação que aceite indevidamente as
strings `"none"`/`"auto"` e rejeite os valores `none`/`auto`. O L0 em
`compiler/stdlib/html.md:544` escreve `none|auto|metadata`, mas deve distinguir
explicitamente os dois valores da linguagem da única string enumerada.

Conclusão causal: este resíduo pertence ao mesmo owner 1:1
`compiler/stdlib/html.md` → `compiler/stdlib/html.rs`. Antes de aprovar C, o
contrato/oráculo protegido deve ser refinado por autoridade segregada com os
cinco casos acima; qualquer mudança protegida invalida o selo e exige novo gate
discriminatório e selo substituto. Não há mudança de entidade, default ou fase.

`iframe.referrerpolicy` foi controlado: `none` é aceito por ambos e serializa
string vazia; `""` e `"none"` são rejeitados por ambos, restando aí somente a
mensagem `or none`, o span e a forma multiline já classificados.

## 5. DOM e escaping

O teste protegido de cinco fixtures preservou presença das sete tags, nesting,
ordem/omissão de atributos, ausência de end tags de `col`/`wbr` e DOM após
decodificação. Houve exatamente **1 violação nominal**, na fixture `escaping`:

```text
vanilla:
<button value="x&amp;&quot;<>">&lt;&amp;></button>

candidato:
<button value="x&amp;&quot;&lt;&gt;">&lt;&amp;&gt;</button>
```

O candidato escapa em excesso `<` e `>` em atributo e `>` em texto. A função
única `escape` em `03_infra/src/export/html.rs:506-511` substitui sempre
`& < > "`; ela é usada indistintamente para texto em `html.rs:190-194` e para
atributo em `html.rs:211-218`.

A fonte ratificada separa contextos: `encode.rs:99-107` consulta charset de
texto, `encode.rs:111-133` consulta charset de atributo;
`charsets.rs:21-35` exige escape apenas de `&` e `"` em atributo normal e
`charsets.rs:38-49` apenas de `&` e `<` em texto normal. O fato de
`write_escape` saber representar outros caracteres (`encode.rs:367-380`) não
os torna obrigatórios nesses contextos.

O L0 vigente de `infra/export/html.md:19-21` ainda manda escapar os quatro
caracteres sem distinguir contexto; portanto ele está desatualizado e não
legitima a correção. O owner necessário é
`infra/export/html.md` → `03_infra/src/export/html.rs`.

**Classificação ADR-0107/0108:** aqui os bytes/contexto de escape são a sintaxe
HTML observável do target, por isso a mecânica de serialização é o próprio
observável. A inferência seria refutada se o `HtmlElem` já contivesse entidades,
se a divergência surgisse no cast L1 ou se os DOMs diferissem antes do encode;
os valores decodificados iguais e os dois call sites de `escape` refutam essas
alternativas.

## 6. Owners L0 e allowlists recomendadas

### Atualização L0 obrigatória, primeiro

| Prompt L0 1:1 | Obrigação estreita |
|---|---|
| `00_nucleo/prompts/compiler/eval/repr.md` | `HtmlElem` usa a disciplina canônica curta/multiline, inclusive nesting e vírgula final, sem tornar todo elemento multiline |
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | preservar transitoriamente os spans sintáticos somente para as sete identidades HTML; named completo, valor named e positional conforme a classe; fallback agregado para forma opaca/spread |
| `00_nucleo/prompts/compiler/stdlib/html.md` | mensagens exatas; separar erro de tipo de domínio positivo/não negativo; união `preload = none | auto | "metadata"`; preservar casts/ordem/body vigentes |
| `00_nucleo/prompts/infra/export/html.md` | escaping contextual: atributo normal escapa `&`/`"`; texto normal escapa `&`/`<`; demais regras vigentes permanecem |

Essa decomposição não duplica ownership: `html.md` é dono dos casts e da
mensagem; `call_dispatch.md` é dono somente da âncora sintática transitória;
`repr.md` é dono da projeção morfológica; `infra/export/html.md` é dono dos
bytes HTML.

### Allowlist produtiva suficiente

```text
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/stdlib/html.rs
03_infra/src/export/html.rs
```

Exclusões explícitas:

```text
00_nucleo/prompts/entities/args.md
01_core/src/entities/args.rs
00_nucleo/prompts/entities/html.md
01_core/src/entities/html.rs
01_core/src/compiler/eval/mod.rs
```

Não foi medida necessidade de novo campo, variante de `Content`, entidade,
assinatura pública, default ou mudança de fase. Se qualquer uma aparecer na
implementação, refuta esta allowlist e exige nova parada ADR-0127.

O refinamento do contrato protegido e seus testes próprios pertence a
autoridades segregadas e não integra a allowlist produtiva. Como quatro L0s e
o contrato protegido precisam mudar, o selo atual deve ser tratado como
invalidado, seguido de resselo de linhagem, gate discriminatório fresco e selo
substituto antes de nova escrita produtiva.

## 7. Unknown, limites e recomendação de transição

- `Unknown` entre os resíduos medidos: **0**.
- sobrevivência não decidida pelo contrato atual: os quatro casos de
  `preload:none/auto/"none"/"auto"`; não são `Unknown` semanticamente, pois a
  sonda bilateral e a fonte pinada os decidiram, mas são lacuna discriminatória
  do contrato vigente.
- fora deste fragmento e não promovidos a sucesso: spans após spread opaco,
  unidade de largura de `repr` não ASCII e escaping de controles/noncharacters.
- os quatro testes próprios C verdes não fecham o lote: usam expectativas do
  candidato e não exercitam os transcripts/bytes divergentes.

Recomendação exata: **não aprovar C e não iniciar D**. Reabrir a cadeia no L0
com os quatro owners acima; refinar o contrato para `preload`; executar novo
gate mutacional com `Unknown=0`; emitir selo substituto; somente então permitir
ao implementador a allowlist produtiva de quatro consumers. Após o candidato,
reexecutar os três testes protegidos C em ambas as ordens e exigir
`surface 46/46`, `DOM 5/5`, `feature/target 1/1`, além dos cinco probes de
`preload` e das regressões dos owners globais de `repr` e exporter.
