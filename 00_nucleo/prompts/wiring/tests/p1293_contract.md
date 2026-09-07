# Prompt L0 — `wiring/tests/p1293_contract` — oráculo black-box protegido P1293
Hash do Código: 58d73bce

**Estado:** `REOPENED-FINAL-ADVERSARIAL-CORRECTIONS` — lotes A–D aprovados; o
oráculo substantivo permanece congelado e a finalização aguarda correções
test-only de observação/transportes e um gate global fresco.

**Camada:** L4 — teste de integração
**Ficheiro alvo exclusivo:** `04_wiring/tests/p1293_contract.rs`
**Origem:** P1293
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0128, ADR-0129
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

No baseline `7dd25ff0e222b6c7c640d6bc7957b98f94227507`, o recibo independente
P1293 de SHA-256
`39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`
reproduziu as 15 diferenças acionáveis: 12 membros ausentes e 3 nomes públicos
errados. Foram executados 203 casos, 624 casts sobre 48 atributos, 5 vetores
DOM, 10 SVG e zero `Unknown`. O baseline exato tem SHA-256
`682cad4bbef22ed6364fd0a5fcb9a8994230e7d22eb716a631f2f2fba6a294b2`.

O gate humano P1293 foi confirmado em `2026-09-01T13:51:42-03:00`. Os sete
L0s de produto foram ressellados mecanicamente sem implementação e V5/V15/V26
ficaram verdes. Este owner nasceu depois desse predecessor causal e antes do
oráculo; a atualização abaixo registra somente artefatos posteriores, sem
alterar comparação, lots, expectativas ou mutações.

Após os gates e reparos seriais, o consumer/oráculo protegido atual é
`04_wiring/tests/p1293_contract.rs`, SHA-256
`ed3e0b57c703a1c4dbdbeda187fcf74dcf46476f60a2c850e48a9ef8f0b2df1e`,
com corpo canônico SHA-256
`b07eb8d6eb6bfa0a892c06bb1f5599d8e2e188963dcca821d83f4a34badfb13a`
e registry de 31 casos SHA-256
`417d82202439b39388d223fe46c9dc193e41b705a951dfeb19c80eddfd75bd8b`.
O gate discriminatório vigente
`p1293-preseal-discrimination-c-p08-semantic-neutrality-receipt.json`, SHA-256
`c2ede2169456b0517d03586412f9aa34f508f9b18d8a7347756cc2dc3c87e7cb`,
registra duas baselines `11/11`, 62 runs, 31/31 mutações rejeitadas em ambas as
ordens, `mutation_score = 1.0`, zero equivalentes, survivors e `Unknown`. Sua
autorreferência de hash é defeito documental pendente e não recebe crédito no
gate final.

O plano adversarial SHA-256
`ecc12db3e5d9826dafdb7b42653e28d5f4f8e01772a8554e6718b1ac8129e226`
e o recibo SHA-256
`8c742753c64524578f08151e604668d047be23a526d9acac9088398870f79868`
confirmaram o oráculo P1293 em `11/11`, mas rejeitaram a finalização por dois
observadores test-only que perdem transformações ancestrais e por pendências
documentais/operacionais. Isso não refuta as expectativas substantivas nem os
produtos A–D aprovados.

## Ownership e isolamento

Este prompt possui exatamente um consumer materializado:
`04_wiring/tests/p1293_contract.rs`. Não legitima código produtivo, outro teste,
harness de mutação ou veredito. O consumer mantém ownership exclusivo `1:1`;
qualquer segundo consumer, owner alternativo, V15 ou V26 bloqueia o selo.

O autor independente do oráculo recebe somente este L0, o manifesto, o recibo
de medição e o contrato candidato. Não lê patch de implementação. O teste usa
o candidato por `CARGO_BIN_EXE_typst`, o vanilla por path pinado e diretórios
temporários isolados. Ausência ou hash incorreto do vanilla, crash, timeout ou
falha de parser é falha explícita, nunca skip ou sucesso.

## Política de comparação

- Presença, tipo, nome público, booleanos, casts, morfologia `repr`, mensagem e
  span de erro, layout semântico e DOM são observáveis de linguagem.
- Estrutura Rust, function pointer/endereço, ordem de passes, bytes completos
  SVG/HTML, IDs de glyph/path, metadata e whitespace mecânico são excluídos,
  exceto ordem de atributos e texto de diagnóstico, explicitamente observáveis.
- `eval --format json` deve preservar tipo JSON; string `"false"` não equivale
  ao boolean `false`.
- `repr` e stdout raw são comparados exatamente após remover somente newline
  terminal do transporte. Diagnósticos comparam severidade, mensagem, source e
  início/fim do span; não comparam ANSI nem path temporário.
- HTML C de paridade é executado explicitamente com
  `--html-serialization vanilla` e parseado como árvore ordenada: tag, attrs na
  ordem emitida, valores, nesting, texto escapado e presença/ausência de end tag
  void. Bytes completos continuam excluídos, exceto a fixture estreita que
  discrimina os escapes observáveis dos dois modos.
- SVG é parseado por `viewBox`, dimensões e presença/ausência semântica de regra
  e delimitadores; tolerância numérica máxima `0.01pt`. Não compara path data.
- Cada lote roda em ordem direta e reversa; transcript normalizado idêntico é
  obrigatório.

Resultados possíveis: `Preserved`, `Violated { witness }` e `Unknown { cause }`.
`Unknown` só é esperado nos casos `O-*` deliberadamente opacos abaixo e nunca
satisfaz um caso requerido `P-*`/`N-*`.

## Lote A — `float.is-nan`

### Positivos requeridos

- `A-P01`: `type(float.is-nan)` é `function` e `repr` é `"is-nan"`.
- `A-P02`: forma estática retorna `[true,false,false,false,false]` para NaN,
  Float finito, Int, `+inf`, `-inf`.
- `A-P03`: forma ligada em Float retorna `true` para NaN e `false` para
  finito/inf; repetições e ordem reversa são idênticas.

### Negativos requeridos

- `A-N01`: sem `self` -> `missing argument: self`.
- `A-N02`: segundo positional -> `unexpected argument` no segundo valor.
- `A-N03`: named na forma ligada -> `unexpected argument: value` no named.
- `A-N04`: string -> `expected float, found string` no argumento.
- `A-N05`: receiver Int ligado -> `type integer has no method is-nan`.
- `A-N06`: obter `float.nan.is-nan` sem chamada continua erro de field.

### Opaco deliberado

`A-O01`: payload binário, sinal ou quiet/signaling bit de NaN é mecânica não
congelada. Consulta a esse detalhe resulta `Unknown`, nunca crédito para A.

## Lote B — `math.attach`, `math.binom`, `math.mono`, `math.script`

### Positivos requeridos

- `B-P01`: as quatro funções têm nomes curtos e tipo `function`.
- `B-P02`: `attach([x])` preserva `base`; payload completo preserva `t,b,tl,bl,tr,br`
  nessa ordem.
- `B-P03`: `attach(t:none)` conserva `t: none` no `repr`; seu layout é
  equivalente ao omitido, sem transformar `none` em texto.
- `B-P04`: `binom([n],[k])` preserva upper e tupla lower singleton; três
  lowers preservam ordem/vírgulas, fração sem barra e parênteses extensíveis.
- `B-P05`: `mono([x])` e `script([x], cramped:true|false)` preservam
  `MathStyled`; default de script é `true` e false permanece observável no
  layout.
- `B-P06`: sintaxe math e chamada qualificada convergem morfologicamente para
  attach, binom, mono e script, sem usar `PartialEq` Rust como prova.
- `B-P07`: os oito vetores de layout do recibo medido preservam viewBox dentro
  de `0.01pt`: attach display `23.2705x19.4843`, attach inline
  `19.7681x9.6041`, binom display `46.797666667x24.057`, binom inline
  `30.3325x7.8815`, mono display `25.029888889x8.921`, mono em script
  `14.0987x6.2447`, script cramped true `8.3578x5.9708` e false
  `8.3578x6.5406`.

### Negativos requeridos

- attach: base ausente, segundo positional, named desconhecido e inteiro em
  base/slot falham com mensagem/span vanilla;
- binom: zero args, só upper, inteiro em Content e named desconhecido falham;
  `upper` named não satisfaz lower variádico posicional;
- mono/script: missing, extra, body named e inteiro falham; script rejeita
  segundo positional, named desconhecido e `cramped` não bool.

### Opaco deliberado

`B-O01`: bytes SVG, IDs e path data exatos são `Unknown`. O caso passa como
teste de opacidade somente se os observáveis B-P07 continuarem verificáveis.

## Lote C — sete constructors HTML tipados

### Positivos requeridos

- `C-P01`: `button`, `col`, `iframe`, `select`, `template`, `video`, `wbr`
  são funções com nomes curtos.
- `C-P02`: cinco tags normais aceitam body opcional e nesting;
  body omitido é `HtmlBody::None`. `col`/`wbr` são void, sem body e com
  morfologia unset.
- `C-P03`: todos aceitam os 76 globais; `id`, `class`, `hidden` cobrem string,
  lista e Presence.
- `C-P04`: os 48 específicos e casts abaixo têm ao menos um valor válido e um
  inválido por classe; enum/lista usa exatamente o conjunto fechado do L0 HTML.
- `C-P05`: Presence true emite atributo vazio, false omite; attrs restantes
  preservam ordem após omissões.
- `C-P06`: sob `--html-serialization vanilla`, DOM `all-seven`,
  `attribute-order`, `void-only`, `escaping` e nesting preserva
  tags/attrs/texto; não há `</col>` ou `</wbr>`.
- `C-P07`: feature off rejeita paged e HTML; feature on aceita ambos. Target
  nunca habilita feature.
- `C-P08`: sem `--html-serialization`, o output HTML usa o modo cristalino e é
  idêntico a `--html-serialization crystalline`; o modo conserva a fixture
  atual `value="x&amp;&quot;&lt;&gt;"` e texto `&lt;&amp;&gt;`. A flag explícita
  `vanilla` produz `value="x&amp;&quot;<>"` e texto `&lt;&amp;>`. Ambos decodificam
  para o mesmo DOM; a flag só é aceita por compile e só afeta output HTML.
- `C-P09`: `video.preload` aceita os valores de linguagem `none` e `auto` e a
  string `"metadata"`; rejeita as strings `"none"` e `"auto"`. Os cinco probes
  são obrigatórios, com os quatro polos valor/string discriminados.
- `C-P10`: a fachada L3 reexporta diretamente `export_html`,
  `export_html_with_serialization` e `HtmlSerializationMode` do módulo HTML
  privado, sem wrapper nem lógica duplicada; o caminho legado permanece
  cristalino e ambos os entry points produzem o mesmo DOM para o mesmo modo.

Matriz dos 48 específicos:

| Tag | atributos e cast |
|---|---|
| `button` | `command` enum-or-string; `commandfor,form,formaction,name,popovertarget,value` string; `disabled,formnovalidate` Presence; `formenctype,formmethod,popovertargetaction,type` enums fechados; `formtarget` target-or-string |
| `col` | `span` inteiro positivo |
| `iframe` | `allow,src,srcdoc` string; `allowfullscreen` Presence; `height,width` inteiro não negativo; `loading` enum; `name` target-or-string; `referrerpolicy` none-or-enum; `sandbox` enum-list de 13 tokens |
| `select` | `autocomplete` enum-list fechada; `disabled,multiple,required` Presence; `form,name` string; `size` inteiro positivo |
| `template` | quatro Presence `shadowrootclonable,shadowrootcustomelementregistry,shadowrootdelegatesfocus,shadowrootserializable`; `shadowrootmode` enum |
| `video` | `autoplay,controls,loop,muted,playsinline` Presence; `crossorigin` enum; `height,width` inteiro não negativo; `poster,src` string; `preload` none/auto/metadata |
| `wbr` | nenhum |

### Negativos requeridos

Cada tag rejeita named desconhecido, `data-*`, atributo específico de outra
tag e cast inválido. `col`/`wbr` rejeitam body. Positivo fora do domínio,
inteiro negativo, zero onde estritamente positivo, enum inválido e lista com
token inválido são testemunhas obrigatórias. Erro deve ancorar o nome/valor
ofensivo, não desaparecer no dispatcher.

Valor alheio de `--html-serialization` falha no parser. A flag em `watch`,
`eval`, `query`, `info` ou outro comando também falha. Em compile PDF/PNG/SVG,
`crystalline|vanilla` é aceito mas não altera o artefato. As strings
`preload:"none"` e `preload:"auto"` permanecem negativas mesmo que produzissem
bytes HTML válidos por outra via.

Wrapper intermediário, duplicação da lógica de serialização, exposição pública
do módulo privado ou divergência DOM entre entry points viola `C-P10`.

### Opaco deliberado

`C-O01`: comportamento de browser, CSS, execução de mídia/rede e acessibilidade
fora do DOM serializado é `Unknown` e não pertence ao denominador de C.

## Lote D — nomes públicos `grid`/`table`

### Positivos requeridos

- `D-P01`: os dez `repr` são `cell,header,footer,hline,vline` em ambos os
  namespaces.
- `D-P02`: chamadas diretas e via `.with(...)` preservam tipo Content e o
  payload observável baseline de cada sibling.
- `D-P03`: os seis aliases flat `grid_cell/header/footer` e
  `table_cell/header/footer` mantêm nomes históricos; aliases flat hline/vline
  continuam ausentes.
- `D-P04`: aridade e diagnósticos cristalinos preexistentes, inclusive a
  exigência de ao menos uma célula em header/footer e caminhos que alcançam o
  serializer, são comparados ao transcript baseline, não ao vanilla.

### Negativos requeridos

Qualquer underscore remanescente no namespace, nome de grid aplicado a table,
alias flat renomeado/criado, `.with` perdido, tipo/payload de chamada alterado
ou relaxamento da divergência D-P04 é `Violated`.

### Opaco deliberado

`D-O01`: identidade de endereço do function pointer é mecânica e `Unknown`.
Chamabilidade/payload equivalentes continuam obrigatórios e não podem ser
substituídos por esse caso opaco.

## Matriz mínima de mutações

O gate discriminatório independente deve rejeitar, no mínimo:

| ID | Mutação | Testemunha mínima |
|---|---|---|
| `MA1` | is-nan sempre falso | `A-P02` NaN |
| `MA2` | infinito tratado como NaN | `A-P02` +/-inf |
| `MA3` | forma ligada usa fórmula distinta | `A-P03` |
| `MA4` | named desconhecido ignorado | `A-N03` |
| `MA5` | nome `float.is-nan` no repr | `A-P01` |
| `MB1` | slot attach trocado/descartado | `B-P02` |
| `MB2` | `none` colapsado ou textual | `B-P03` |
| `MB3` | binom desenha barra | `B-P04/B-P07` |
| `MB4` | lower perde ordem/vírgula | `B-P04` |
| `MB5` | sintaxe e função divergem | `B-P06` |
| `MB6` | mono/script usam wrapper distinto | `B-P05/B-P06` |
| `MB7` | cramped explícito ignorado | `B-P05/B-P07` |
| `MC1` | qualquer named vira string | negativos C |
| `MC2` | Presence false é emitida | `C-P05` |
| `MC3` | void aceita body | `C-P02`/negativos C |
| `MC4` | body normal omitido vira Unset | `C-P02` |
| `MC5` | attr específico cruza tag | negativos C |
| `MC6` | enum aceita token alheio | `C-P04`/negativos C |
| `MC7` | target liga feature | `C-P07` |
| `MC8` | void recebe end tag | `C-P06` |
| `MC9` | ordem/escaping é alterado | `C-P05/C-P06` |
| `MC10` | ausência da flag troca o default para vanilla | `C-P08` |
| `MC11` | modos crystalline/vanilla colapsam no mesmo escaping | `C-P08` |
| `MC12` | preload confunde `none`/`auto` com strings homônimas | `C-P09` |
| `MC13` | fachada substitui reexport direto por wrapper/lógica | `C-P10` |
| `MC14` | entry points divergem no DOM para o mesmo modo | `C-P10` |
| `MD1` | só três nomes amostrados corrigidos | `D-P01` dez siblings |
| `MD2` | nome curto aplicado ao alias flat | `D-P03` |
| `MD3` | table recebe nome grid | `D-P01` |
| `MD4` | nome e call mudam juntos | `D-P02/D-P04` |
| `MD5` | um sibling conserva underscore | `D-P01` |

Todas as 31 mutações válidas devem produzir `Violated` com witness; score
exigido `31/31 = 1.0`. Mutante equivalente só sai do denominador por decisão do
adversário aceita pelo adjudicador antes do cálculo. Crash, timeout, harness
ambíguo ou mutante sobrevivente é `Unknown` bloqueante.

## Lineage materializada e gates

O consumer atual foi mantido pelo autor independente do oráculo com:

```text
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/tests/p1293_contract.md
//! @prompt-hash b1f580e2
//! @layer L4
//! @updated 2026-09-02
```

O raw SHA-256 vigente é
`ed3e0b57c703a1c4dbdbeda187fcf74dcf46476f60a2c850e48a9ef8f0b2df1e`,
o corpo canônico é
`b07eb8d6eb6bfa0a892c06bb1f5599d8e2e188963dcca821d83f4a34badfb13a`
e o bloco registry de 31 IDs é
`417d82202439b39388d223fe46c9dc193e41b705a951dfeb19c80eddfd75bd8b`.
C-P10 e MC13/MC14 já estão materializados nesse consumer; esta revisão fecha
somente a omissão do owner documental e não autoriza mudança de expectativa,
fixture, comparação ou corpo substantivo do oráculo.

Os checkpoints independentes aprovados são: lote A, receipt de implementação
SHA-256 `891f705c75a7cf5583956233f69aac9df8807f77c9449594af4e9fbf043543c6`;
lote B, verificação SHA-256
`16f862d5dafa343309ac9172e94e590036e66a8a45aba7114339fe4ba44d5d0b`;
lote C, verificação SHA-256
`d3dce5dca7c747f3c0abf37665153713e7a70bdda9947889edbb841239580206`;
lote D, verificação SHA-256
`b53d060ac1cffb5cdfe338d206fb18442519a5d05f60129ec70088a69d3a8460`.
A–D estão aprovados, mas isso não equivale a certificado final.

## P1293.final — decisão após campanha adversarial

### Medição anterior à decisão

O plano adversarial SHA-256
`ecc12db3e5d9826dafdb7b42653e28d5f4f8e01772a8554e6718b1ac8129e226`
e o receipt SHA-256
`8c742753c64524578f08151e604668d047be23a526d9acac9088398870f79868`
mediram `11/11` casos P1293 verdes, zero `Unknown`, e registry 31 coerente com
C-P10/MC13/MC14. A campanha também mediu `910/918` na integração L3 e `10/11`
no contrato P1292: as nove falhas são observadores que descartam posição ou
transformação ancestral. O produto e o L0 `compiler/layout/equation.md` não são
refutados por esses resultados.

Medição: `04_wiring/tests/p1293_contract.rs:917-947` contém C-P10;
`:1147-1148` contém MC13/MC14; `:1155-1162` exige 31 IDs. Inferência: o owner
estava documentalmente atrasado, enquanto o consumer já satisfazia o contrato
confirmado. Refutadores: mudança de expectativa substantiva, registry diferente
de 31, C-P10 ausente, ou qualquer lote A–D sem aprovação independente.

### Decisão, isolamento e gate final

Preservam-se integralmente os casos, expectativas e política de comparação do
oráculo. O único efeito permitido neste consumer após esta revisão é o resselo
mecânico do `@prompt-hash`; correções dos observadores pertencem aos owners
test-only `infra/integration_tests.md` e `wiring/tests/p1292_contract.md`.
Formatos de receipts, limpeza de artefatos e qualquer incidente operacional
permanecem fora desta autorização e devem ser julgados separadamente.

Classificação ADR-0107/0108: C-P10, DOM, mensagens, spans e registry são prova
de linguagem/contrato; hashes e parsing são transporte, medidos antes da
decisão. ADR-0127: atualização test-only/documental, sem nova API, default,
fase ou quebra, segue fluxo contínuo. ADR-0129: este L0 continua com exatamente
um consumer.

Esta edição torna o header `b1f580e2` transitoriamente stale e torna qualquer
selo/final handoff anterior inadequado para nova autorização; manifesto e selo
não são alterados neste passo. Após resselo mecânico dos três consumers, devem
rodar de forma independente os testes corrigidos, V5/V15/V26, gate global
fresco de 31/31 sem `Unknown`, adversário e adjudicação final. Até isso ocorrer,
o estado é `final-corrections-pending`, nunca `final-approved`.

## P1308 — migração independente da repr de With

Medição anterior à decisão: `00_nucleo/diagnosticos/p1308-measure.json`,
SHA-256 `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`,
confirma bilateralmente `(..) => ..` para With; fonte ratificada
`foundations/func.rs:460-470`. P1307 já corrigiu o produto, mas a expectativa
histórica de D ainda exige o nome nativo. É morfologia de linguagem.

Com autorização humana P1308, o autor independente pode alterar somente a
expectativa de repr dos parciais no teste
`p1293_d_ten_short_names_with_calls_and_flat_aliases_preserved` para a forma
anônima medida. Nomes diretos e aliases flat, chamadas, payloads e todas as
demais expectativas permanecem intactos. Esta exceção sucede a proibição
anterior de alteração substantiva apenas nesse assert; não revalida selos
históricos nem dispensa testes frescos.

## P1308-R2 — trace dos erros históricos de D-P04

Medição anterior à decisão: `p1308-workspace-tests.json` em diagnosticos,
SHA-256 `d6d8ac5baf46ec195a164965c5140819c1dd00ec08c93e3b0a9063c507c937c2`,
registra 6619 testes verdes e uma falha em
`p1293_d_preexisting_arity_and_serializer_transcript_is_frozen`: a mensagem
de grid.cell permanece a mesma e o stderr ganha o trace da chamada. A
verificação independente P1308 (SHA-256
`06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60`)
liga o delta à Source resolvível de eval, não à alteração de aridade.

Após autorização explícita do dono para migrar esses transcripts e
commitar, o autor independente deve medir todos os seis erros do mesmo
teste (cell/header/footer de grid/table) e pode acrescentar somente seus
traces naturais aos expected. Preservar expressão, mensagem portuguesa,
exit, stdout, spans primários, ordenação e controles de sucesso/serializer.
Os traces devem corresponder à chamada interna efetiva, não ao repr externo;
comparar o transcript completo, sem remover ou normalizar traces.

Esta exceção sucede a preservação byte-idêntica de D-P04 apenas para o
trace que antes faltava. Não corrigir a dívida de aridade, spans detached,
mensagens ou outros lotes. A migração anterior de With permanece válida.
Não alterar registry de mutantes nem alegar selo histórico revalidado.
L0 primeiro, autoria independente, resselo e testes frescos; nenhum código
produtivo novo é autorizado por este owner test-only.
