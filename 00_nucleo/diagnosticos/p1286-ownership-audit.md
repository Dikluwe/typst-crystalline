# P1286 — auditoria de ownership L0 e gate ADR-0127

**Papel:** auditor de ownership somente leitura.  
**Regime:** auditoria diagnóstica; a skill de materialização segregada é usada
somente para restringir capacidades e registrar entradas. Não há contrato
selado, campanha adversarial, implementação, certificado ou veredito final.  
**Escopo:** `smartquote(alternative:/quotes:)`, `line(start:)` não-zero,
`color.mix` com N cores/pesos, `pdf.attach` e semântica de `pdf.artifact`.  
**Restrição de conteúdo cumprida:** nenhum ficheiro em
`00_nucleo/materialization/` ou `00_nucleo/context/` foi aberto ou teve o
conteúdo lido. Limitação processual: o `git status --short` global usado para
proveniência expôs nomes de paths não rastreados em `materialization/`; não foi
feita listagem dirigida, glob, `rg` ou leitura desses paths.

## 1. Proveniência desta medição

A fotografia foi tomada em `2026-08-30T18:20:53-03:00`, antes da criação deste
diagnóstico, sobre `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88` e uma
working tree não commitada. O SHA-256 de `git status --porcelain=v1 -uno` era
`ac43b721d970d7bc3fdd3b6830ff1706d21b8c6f95b4f1eb0fe85a6055cac3db`; o de
`git diff HEAD --stat` era
`0e0a46b92b8be440eb2900cfe1bc394b49aea4f6e9cf288d0012d65624ac3724`.
Entre os inputs materiais desta auditoria estavam modificados:

```text
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/value_methods.md
00_nucleo/prompts/compiler/eval/rules.md
00_nucleo/prompts/compiler/stdlib/color.md
00_nucleo/prompts/entities/color.md
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/rules.rs
01_core/src/compiler/stdlib/color.rs
01_core/src/entities/color.rs
```

Logo, os hashes abaixo identificam os bytes efetivamente auditados, não apenas
o commit base. As autoridades processuais lidas integralmente foram:

| entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `typst-adr-0127-gate-l0-paragem-vs-fluxo.md` | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| `typst-adr-0129-nucleos-tekt-l0-compartilhado.md` | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| `tekt-materializacao-segregada/SKILL.md` | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |

O baseline vanilla ratificado continua sendo `a51e02804`. Esta auditoria não
infere comportamento novo: mede apenas as estruturas pinadas relevantes:
`text/smartquote.rs:33-89,201-310`, `visualize/line.rs:18-49`,
`typst-layout/src/shapes.rs:19-52`, `visualize/color.rs:1058-1090,1136-1204,
2422-2459`, `pdf/attach.rs:31-73`, `pdf/accessibility.rs:36-93`,
`typst-layout/src/rules.rs:824-826`, `typst-pdf/src/attach.rs:13-67` e
`typst-pdf/src/convert.rs:75-94`. Os SHA-256 integrais dessas fontes são,
respetivamente, `5ecdcbff06730194a01fc8d8da1acc5564ea914efdefb6b550d5cd0dfe286a1d`,
`5816aaffa4a339799fec9e69f6a87b31953a7401b3bcb711b6bc83beac9d84ba`,
`875df8a8b9775d305bd05be60cb3e2889a671053990bb543ec7f70912ff7d35f`,
`80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713`,
`525d2505b229baa9ae1a6078de290bddaeba23a4378304c695aa36c55f63aaed`,
`dba1d5f5e4fe3e8c24376cd96616be0f183926fa15b6303ee75c3806e8567d51`,
`72b34f1640483893fd2f5da814f6260921b3786cfa60dd1046073330af12e634`,
`7d88f3efd8579ccd8b91070c408a6ba139715acd9a01911e5d43e7e513005a13`
e `0efa046cfdf78abd62e60836f104ca00811972b16372eb6e92ac303666eec3a2`.

## 2. Estado mecânico de ownership

`crystalline-lint --checks v15,v26 --fail-on warning .` terminou com exit 0 e
`No violations found`. O binário auditado tem SHA-256
`80bb6b2aa23ce1b83f9a0a2540a4ff61a9a4993aca01b2e44f72bf16378e5cff`.
Isto prova apenas que a árvore **atual** não contém V15/V26; não autoriza criar
novos consumers com prompts compartilhados.

Cada source listado abaixo declara o owner na linha 2 e o `@prompt-hash` na
linha 3. `Hash do Código` é a metadata L0 da linha 2 do respetivo prompt; os
SHA-256 são dos bytes completos atuais.

| consumer produtivo | SHA-256 source | owner L0 / SHA-256 | header / `Hash do Código` |
|---|---|---|---|
| `compiler/stdlib/text/smartquote.rs` | `398e55df09841c2b2edafac9db4df4e5cfe6537dd97d4508af303e38b9a88508` | `compiler/stdlib/text/smartquote.md` / `f5af11123419001512a298bd0741a9f2ba009225d88b42ac996c547299c45e59` | `5a495c14` / `844653a1` |
| `entities/elements/smartquote.rs` | `b729e800419410297b582eb1bad7adee868de5da11b8836eb2ee33e0cc4c9eed` | `entities/elements/smartquote.md` / `bcc77462ab61b8587e72615be2f8464750024bcdd8f5d4c67cefca54773e205b` | `08af9b7e` / `10180bfe` |
| `compiler/layout/smartquote.rs` | `c308b24bca545682f009fc8579560d05771d6656e611933c32acf3ee88e37e61` | `compiler/layout/smartquote.md` / `c7d6c227f359aa6cec358fb160046a1b6d0b3d774f5edc678cd3794d0cbb87c5` | `02ba82cf` / `b1fb93ac` |
| `compiler/lang/quotes.rs` | `2cf5aea5d5fba68b9fbee332fbe4c82cfe69db4dd327ecb7a2c513a0006fa4f9` | `compiler/lang/quotes.md` / `176638ebac89bf43321c67af3c3e4130cdf5ee288ff3df2de642632b2f50573a` | `07e3cb92` / `32112a2f` |
| `compiler/eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` | `compiler/eval/rules.md` / `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` | `cc162c8a` / `66b2bbbc` |
| `compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` | `compiler/eval.md` / `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` | `41d1716d` / `d47b439e` |
| `entities/content.rs` | `713f72418b00f1b64c31e864083af7153c9c4ebe384945a4886a58314355aed5` | `entities/content.md` / `62229b878b405a73d4d767294f557d35f3a4c3b6eb773c13a74facad4150ba35` | `89bcf65b` / `21f9a700` |
| `compiler/stdlib/shapes.rs` | `e7869c8625620a8e7248f38ac97cc833d918691bb5f272e2364502df64ec3239` | `compiler/stdlib/shapes.md` / `f24f10c596c776da6c9e164c6f92aea4084ec611bc50c325d5c8378f538de698` | `2b236727` / `91e1828b` |
| `entities/geometry.rs` | `ef7a4b8471daba09e5c0fbe28e4f26f0dc2ba2f675245c434326391540b57749` | `entities/geometry.md` / `a8cfbe9a22342b65654e4d9d35458523e11c02dd013651f7f4ec7eeda8ff565c` | `77f1f320` / `9475ace5` |
| `entities/elements/shape.rs` | `ddcb323e3099d9da61f7f82b8bfbb3afde0fd5c0385b4e2597f154cfdc266ac3` | `entities/elements/shape.md` / `355bd581359e1312e1a483e1294063b8a81b74b8348a56d57bb4f713ad894ea8` | `0d1c4297` / `5cfe8219` |
| `compiler/layout/shape.rs` | `4a388fbc6a46da93e663eb6e962ad80b11277353a25e5941152ef55b7ecf34fc` | `compiler/layout/shape_block_behaviour.md` / `f064ba47d34d4f899051f58c4afa6d31b06ea6bcff135aa4de87f00f95b1ca75` | `3c4b21a1` / `af64b3ce` |
| `compiler/stdlib/color.rs` | `1ff3cdac5a70807cad4cb70c214590a985a2bfb3bbf0423b46ffd4e4592d9792` | `compiler/stdlib/color.md` / `7149cf1a93c96dab1c4d31aa491fd2093bfccd06459681c6f9f04ee1eff2677c` | `eedc5348` / `9a2e6451` |
| `entities/color.rs` | `18daa8487216f56dc6d315cc024bcb4130a63af6b977da8986144d06fbb39d4e` | `entities/color.md` / `41d6aaa1e696de39823ca63fe179351125a8573467521c1a0e6d795afdb4562a` | `9f4d3b50` / `3a7a726b` |
| `compiler/eval/bindings/value_methods.rs` | `139873711c2d6fba0216152753e602a90083c98db5850e5495a13504b48bb522` | `compiler/eval/bindings/value_methods.md` / `85d31916ee6091f9853e2605760c3ecc41d4be5c5a702f1fa7e4b3bacc1b6091` | `9af814fe` / `def19f1d` |
| `compiler/stdlib/pdf.rs` | `095d2b030d17134ad860c5acc303c06b5bfe76243b1190ba8b033b09e3315346` | `compiler/stdlib/pdf.md` / `5d32e0013e91b42943b99276f2f334000ed85c60d71147b950c1b1bd645b9f4b` | `38bda66a` / `23b735af` |
| `entities/layout_types.rs` | `da93d06308d3cfc9a6667e9391a83eebfaaa77a2761f4f1f489356f4f9159a17` | `entities/layout_types.md` / `68f453b4abfd333f17f6f989ac83e11846926b9b91b16e70ab455bb17f603779` | `b771743d` / `66b28ef2` |
| `infra/export/builder.rs` | `a16749b41661f8b967f00cda0edbf93d09fdbafbf8a7d633bba85ab58cea07b9` | `infra/export/builder.md` / `fe6b11c2e2cf060a94d714908fdbae0c37d61dc32d34012a432291e11ba36a7f` | `3c4fcbaa` / `0327af88` |
| `infra/export/stream.rs` | `be705e28601585e84d2bc4a81a66b870e39d79e77334791d992a346e23478923` | `infra/export/stream.md` / `a17500e97ce6e6c884e62510cfc3a68bc2933cf203c54a5cecb30f8ed8d8142d` | `412e3910` / `0f5f9079` |
| `infra/export/mod.rs` | `56d2484579e26715fa870b66cdbfb6aaca8aa88a674d1ab9b9efefda2490632c` | `infra/export/mod.md` / `bdf4ec679daede866f8f1f303cd17389f6e5f3c2cc0d06849132903870eaa6ce` | `a181f89d` / `353ce4a4` |
| `infra/pipeline.rs` | `41e085a949d139c6d4e5456a2bdeca60f0361164059a08e34f8493f651339dcd` | `infra/pipeline.md` / `8a4caaec2147a5848bafbd717aa129d235b3cff8dccd7d2c76c4da4e7257169b` | `35c87fd3` / `b975c6d6` |
| `contracts/world.rs` | `ccc9aa3460267283135cfc7c1e967fbfb72bda552765510deaf5713afd051d6c` | `contracts/world.md` / `cd2b0d775bf3dd759908f3440eb90758cfef8f0ed8d8529f0cce18be9ef12078` | `c15e79e2` / `5abd444e` |

## 3. Núcleos Tekt e pins relevantes

Os únicos consumos declarados nos owners acima são:

| prompts consumidores | Núcleo / pin efetivo declarado | relação com P1286 |
|---|---|---|
| `compiler/layout/smartquote.md:4-5` | `_nuclei/layout/element-form-b.toml` / `6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5` | preserva a forma B de ADR-0109; qualquer novo layout elementar PDF precisa de owner próprio e só consome este Núcleo se cumprir a mesma claim |
| `compiler/lang/quotes.md:4-5` | `_nuclei/lang/defaults.toml` / `c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9` | defaults/fallbacks de idioma; relevante se `alternative` ampliar a tabela |
| `compiler/eval.md:4-5`, `compiler/eval/rules.md:4-5` | `_nuclei/eval/core.toml` / `e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58` | partilha legítima entre dois prompts proprietários, não owner de código |
| `infra/export/builder.md:4-5` | `_nuclei/export/bitmap-embedding.toml` / `016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def` | pin existente a preservar; não cobre ficheiros anexos |
| `infra/pipeline.md:4-6` | `_nuclei/export/svg-destination-context.toml` / `13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1`; `_nuclei/export/svg-glyph-font-context.toml` / `4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c` | exclusivos de SVG; não legitimam transporte PDF |

V26 aceitou todos os pins e o DAG no estado medido. Não existe Núcleo para
attachment, artifact ou mistura N-ária. Criá-lo não é requisito automático:
só é válido se houver uma claim realmente compartilhada por pelo menos dois
prompts proprietários; caso contrário seria Núcleo órfão ou abstração sem
consumer legítimo.

## 4. Fatia `smartquote(alternative:/quotes:)`

### Medição antes da classificação

- A entidade atual tem apenas o campo público `double` em
  `entities/elements/smartquote.rs:16-20`; o L0 congela essa forma em
  `entities/elements/smartquote.md:10-21`.
- A nativa rejeita `alternative` e `quotes` em
  `compiler/stdlib/text/smartquote.rs:79-90`; o L0 torna o scope-out normativo
  em `compiler/stdlib/text/smartquote.md:50-56,79-80`.
- O layout escolhe apenas o par primário pela língua e usa ASCII para simples
  em `compiler/layout/smartquote.rs:15-36`.
- A tabela atual não representa pares alternativos em
  `compiler/lang/quotes.rs:19-47`; o seu L0 proíbe alternância aninhada em
  `compiler/lang/quotes.md:24-28`.
- O caminho markup/#set é separado: `rules.rs:1881-1939` aceita `enabled` e
  apenas `quotes` string/auto/none; `eval/mod.rs:696-767` consome esses styles e
  pré-resolve markup em `Content::Text`. Não há `smartquote.alternative` em L1.
- Já existe um carrier interno de style: `Styles::push_custom` transporta
  `(key, Value)` em `entities/style.rs:314-327`, `StyleChain::custom` resolve a
  chave em `entities/style_chain.rs:405-417`, e `Content::Styled` empilha esse
  delta durante layout em `compiler/layout/mod.rs:1516-1530`. A existência
  desse canal refuta que novos campos de `SmartQuoteElem` sejam inevitáveis;
  ainda não prova, porém, que ele preserve toda a morfologia da chamada direta
  e de `#set`. Os SHA-256 dessas três fontes de suporte são, pela mesma ordem,
  `48514114042c66fe98c39514b9e7a1ac44b0ff061a8a84161efab21963081f42`,
  `b49594292bcf9bf89cefe173f852f35f988a6350d73828d6d5cbb3ad0ef48185` e
  `2b4ea5711fc513850bc071c1895bdab2ff8631effddd091244cb15e8442ff9bb`.
- O vanilla pinado expõe `alternative: bool` e
  `quotes: Smart<SmartQuoteDict>` em `text/smartquote.rs:51-89` e resolve ambos
  junto de língua/região em `:201-310`.

### Ownership e gate

Os owners certos para retirar a rejeição, selecionar par e preservar a tabela
linguística são `compiler/stdlib/text/smartquote.md`,
`compiler/layout/smartquote.md` e `compiler/lang/quotes.md`. Se o recorte inclui
`#set smartquote(...)` e markup — como no modelo vanilla — também são afetados
`compiler/eval/rules.md` e `compiler/eval.md`; a restrição de não ler o passo
impede reduzir honestamente essa condição. `entities/elements/smartquote.md` e
`entities/content.md` só entram obrigatoriamente se o carrier público atual for
alargado.

Há duas classes de implementação mínima ainda abertas, e esta auditoria não
escolhe entre elas:

1. envolver o `Content::SmartQuote` vigente num `Content::Styled` com chaves
   custom (ou carrier interno equivalente) e consumir os valores no layout.
   Se a medição de morfologia confirmar equivalência e não houver API pública,
   default ou fase alterados, isto é correção interna de paridade em fluxo
   contínuo ADR-0127; e
2. acrescentar `alternative`/pares a `SmartQuoteElem` (ou tipo público
   equivalente). Isso é **campo público novo** e ativa paragem obrigatória
   ADR-0127 §2.1, além de exigir os L0 de entidade/conteúdo.

Nenhuma das rotas exige por si nova variante `Content`, `Value` ou `FrameItem`,
nem novo método de trait: `Value::{Bool,Str,Array,Dict,Auto}` e o layout textual
já oferecem os tipos de valor. Mover a resolução de eval para layout seria gate
de fase separado (`compiler/stdlib/text/smartquote.md:63-68`). Os defaults
vanilla medidos permanecem `alternative=false` e `quotes=auto`, logo não há
mudança de default inevitável. O conflito certo a remover está em
`compiler/stdlib/text/smartquote.md:50-56`; `entities/content.md:647-700` só é
conflito se o payload for alterado.

## 5. Fatia `line(start:)` não-zero

### Medição antes da classificação

- `native_line` calcula `dx/dy`, mas rejeita `start != (0,0)` em
  `compiler/stdlib/shapes.rs:404-425,436-449`; o L0 congela o scope-out em
  `compiler/stdlib/shapes.md:127-168,175-181`.
- `ShapeKind::Line` é enum público com apenas `dx/dy` em
  `entities/geometry.rs:110-142`, reproduzido em
  `entities/geometry.md:136-148`.
- `ShapeElem` é público em `entities/elements/shape.rs:19-29`; o layout calcula
  o bbox como `abs(dx),abs(dy)` em `compiler/layout/shape.rs:46-60` e posiciona
  um `FrameItem::Shape` em `:126-153`.
- O domínio já tem `PathItem::{MoveTo,LineTo}` e `ShapeKind::Path` em
  `entities/geometry.rs:53-63,137-142`; a stdlib já constrói paths em
  `compiler/stdlib/shapes.rs:643-710`, e o layout aceita a variante vigente em
  `compiler/layout/shape.rs:49-60`. Isso torna possível uma terceira rota
  interna, mas não demonstra ainda paridade de bbox, stroke e morfologia.
- O vanilla preserva `start`, `end`, `length`, `angle` como campos em
  `visualize/line.rs:18-49`; o layout resolve o início e empurra a shape nesse
  ponto em `typst-layout/src/shapes.rs:19-52`.

### Ownership e gate

O único owner certamente editado é `compiler/stdlib/shapes.md`. Há três rotas
de carrier cuja suficiência precisa ser medida antes de qualquer L0 final:

1. acrescentar origem a `ShapeKind::Line` → editar `entities/geometry.md`,
   `compiler/stdlib/shapes.md` e todos os consumers exaustivos do enum,
   incluindo `compiler/layout/shape_block_behaviour.md`; ou
2. acrescentar origem à `ShapeElem` → editar `entities/elements/shape.md`,
   `entities/content.md`, `compiler/stdlib/shapes.md` e
   `compiler/layout/shape_block_behaviour.md`; ou
3. baixar a linha não-zero para o `ShapeKind::Path` já existente. Se medição no
   nível da língua confirmar bbox, stroke, posicionamento e morfologia
   equivalentes ao vanilla, essa rota não acrescenta contrato público e pode
   ser correção interna de paridade em fluxo contínuo; o L0 de layout só entra
   se o comportamento do seu consumer mudar.

As rotas 1 e 2 introduzem **campo/variante pública** e ativam ADR-0127 §2.1; a
rota 3 refuta que isso seja inevitável, mas não está validada por output nesta
auditoria. `FrameItem::Shape.pos` já transporta posição
(`layout_types.rs:320-328`), e nenhuma rota exige inevitavelmente nova variante
`Content`, `Value` ou `FrameItem`, nem trait novo. A fase permanece eval →
layout → export e o default `start=(0,0)` pode permanecer. O conflito certo é o
scope-out em `compiler/stdlib/shapes.md:175-181`; os demais L0 dependem do
carrier escolhido.

## 6. Fatia `color.mix` com N cores e pesos

### Medição antes da classificação

- A nativa aceita exatamente duas cores e um named `weight` em
  `compiler/stdlib/color.rs:347-378`; seu L0 declara a mesma forma e o
  scope-out N-ário em `compiler/stdlib/color.md:74-79,303-320`.
- O método de instância apenas insere o receiver e rejeita named `weight` em
  `compiler/eval/bindings/value_methods.rs:190-219`.
- O domínio atual oferece a operação binária pública
  `Color::mix(self, other, weight, space)` em `entities/color.rs:699-733`; o L0
  fixa essa assinatura em `entities/color.md:211-228` e registra a lacuna de
  par `(cor,peso)` em `:425-428`.
- O vanilla pinado declara `colors: Vec<WeightedColor>` variádico em
  `visualize/color.rs:1058-1090`, normaliza N pesos em `:1138-1204` e aceita
  cor nua ou array `[cor,peso]` em `:2422-2459`.

### Ownership e gate

Os L0 a editar são `compiler/stdlib/color.md` e `entities/color.md`.
`compiler/eval/bindings/value_methods.md` só precisa de edição se o glue AST
for alterado; o caminho atual já entrega os posicionais avaliados à mesma
nativa. Não há owner ausente.

A implementação mínima pode manter a assinatura pública binária existente e
introduzir apenas helper privado/`pub(crate)` de redução ponderada. Portanto
não exige campo público, variante `Content`/`Value`/`FrameItem`, método de
trait, mudança de fase ou mudança de default. Nessa forma, é correção de
paridade interna e segue fluxo contínuo ADR-0127, sempre com L0 primeiro,
RED→GREEN e resselo. Se for escolhida uma nova API Rust pública
`Color::mix_iter`, a classificação muda para paragem ADR-0127 §2.1; ela não é
necessária para o mínimo medido.

## 7. Fatia `pdf.attach`

### Medição antes da classificação

- `native_pdf_attach` sempre retorna o erro de scope-out em
  `compiler/stdlib/pdf.rs:48-61`; `compiler/stdlib/pdf.md:13-23,29-31,43-46`
  legitima essa rejeição.
- O próprio L0 `compiler/stdlib/pdf.md:5` ainda lista dois ficheiros alvo
  (`pdf.rs` e `eval/mod.rs`), mas os headers produtivos medidos são biunívocos:
  `pdf.rs:2` aponta a esse L0 e `eval/mod.rs:2` aponta a `compiler/eval.md`.
  V15 passa porque a relação efetiva dos headers é 1:1, porém a redação do L0
  é ambígua sob ADR-0129 e deve ser reduzida ao seu único consumer.
- DEBT-66 mantém explicitamente o scope-out e exige reabertura pelo dono em
  `diagnosticos/debt/DEBT.md:271-300`. O mesmo texto mede o custo como “novo
  canal L1→L3 (variant Content ou side-channel)” em `:285-292`.
- O `World` já oferece `file` e `read_bytes(current_file,path)` em
  `contracts/world.rs:33-80`. Não há necessidade medida de acrescentar método
  ao trait para ler os bytes.
- O vanilla transporta `path`, bytes, relationship, MIME e description em
  `pdf/attach.rs:31-73`, realiza show vazio em `typst-layout/src/rules.rs:824`
  e coleta os elementos globalmente para embedding em
  `typst-pdf/src/attach.rs:13-67`, chamado por `convert.rs:75-94`.

### Ownership, owners ausentes e gate

`compiler/stdlib/pdf.md` deve deixar de declarar o scope-out antes de qualquer
código. Um carrier produtivo ainda não existe. Se for adotada uma variante
elementar, faltam hoje owners L0 1:1 para, no mínimo:

- `01_core/src/entities/elements/pdf_attach.rs` ↔ novo prompt proprietário
  `00_nucleo/prompts/entities/elements/pdf_attach.md`;
- qualquer novo consumer de layout `compiler/layout/pdf_attach.rs` ↔ novo L0
  próprio; e
- qualquer novo módulo `infra/export/attachments.rs` ↔ novo L0 próprio. Se o
  embedding ficar em `infra/export/builder.rs`, o owner vigente é
  `infra/export/builder.md` e não se cria o terceiro par.

Além desses owners, `entities/content.md` precisa mudar se houver
`Content::PdfAttach`; `entities/layout_types.md` se o side-channel for campo de
`PagedDocument` ou variante de `FrameItem`; `infra/export/builder.md` para
embedded-file stream, Filespec e catálogo; e `infra/pipeline.md` se a coleção
for feita entre eval/layout/export. `infra/export/mod.md` só precisa mudar se a
assinatura pública do exportador for ampliada — não é inevitável se o carrier
viajar em `PagedDocument`.

Qualquer carrier fiel exige **uma alteração pública inevitável**: nova variante
`Content`, nova variante `FrameItem`, ou novo campo em `PagedDocument`. Logo há
paragem ADR-0127 §2.1 independentemente da escolha. Não é necessária variante
`Value` (`Value::Content` basta) nem método novo no trait `World`. A ordem de
pipeline não precisa inevitavelmente mudar, mas a escolha “introspecção global
vs marker de layout vs campo do documento” altera a fronteira de fase; enquanto
não for decidida, ADR-0127 manda parar por dúvida de classificação. Não há
mudança de default de produto identificada.

## 8. Fatia semântica `pdf.artifact`

### Medição antes da classificação

- A nativa valida `kind`, mas devolve o body sem wrapper em
  `compiler/stdlib/pdf.rs:63-126`; o L0 justifica isso alegando ausência global
  de tagging em `compiler/stdlib/pdf.md:19-23`.
- Essa premissa está desatualizada: `infra/export/mod.md:119-136` define
  `PdfTags::Enabled` por default; `entities/layout_types.rs:298-328` já possui
  `FrameItem::Semantic`; `infra/export/stream.rs:1540-1553,2026-2039` emite
  conteúdo marcado; `infra/export/builder.rs:2434-2484` constrói a estrutura de
  fórmulas. Os owners correspondentes medem a mesma frente em
  `entities/layout_types.md:67-86`, `infra/export/stream.md:437-470` e
  `infra/export/builder.md:740-765`.
- O stream atual escreve `/Formula` para qualquer `FrameItem::Semantic` que
  entre nesses braços (`stream.rs:1540-1553,2026-2039`); portanto não se pode
  reutilizar o envelope sem discriminar artifact.
- O vanilla representa `ArtifactElem { kind, body }` e `ArtifactKind` em
  `pdf/accessibility.rs:36-93`, mostra o body em
  `typst-layout/src/rules.rs:826` e usa essa identidade na árvore de tags
  (`typst-pdf/src/tags/tree/build.rs:341-346`).

### Ownership, owners ausentes e gate

Os L0 existentes necessariamente afetados são `compiler/stdlib/pdf.md`,
`entities/content.md`, `entities/layout_types.md`,
`infra/export/stream.md` e `infra/export/builder.md`. Faltam owners próprios
para os consumers naturais ainda inexistentes:

- `entities/elements/pdf_artifact.rs` ↔ novo
  `prompts/entities/elements/pdf_artifact.md`;
- `compiler/layout/pdf_artifact.rs` ↔ novo
  `prompts/compiler/layout/pdf_artifact.md`, consumindo
  `_nuclei/layout/element-form-b.toml` somente se as claims se aplicarem e com
  pin efetivo válido.

A implementação mínima requer nova variante pública `Content::PdfArtifact` e
um `ArtifactKind` público, ou carrier público semanticamente equivalente. Ela
pode reutilizar a variante `FrameItem::Semantic`; não exige necessariamente
nova variante de `FrameItem`, mas exige ao menos nova variante pública de
`SemanticKind` (ou outro campo discriminante) para não emitir `/Formula`.
`Value::Content` basta e nenhum trait precisa mudar. O tagging já é default
`Enabled`, portanto não há mudança inevitável de default; a fase continua
eval → layout → export. O gate obrigatório é contrato público ADR-0127 §2.1.

## 9. Lista de L0 a editar antes de código

Lista consolidada, separando obrigação certa de escolha ainda não resolvida:

| fatia | L0 certos | L0 condicionais ao carrier/superfície |
|---|---|---|
| smartquote | `compiler/stdlib/text/smartquote.md`; `compiler/layout/smartquote.md`; `compiler/lang/quotes.md` | `compiler/eval/rules.md` + `compiler/eval.md` se inclui `#set`/markup; `entities/elements/smartquote.md` + `entities/content.md` se o payload público mudar |
| line start | `compiler/stdlib/shapes.md` | `compiler/layout/shape_block_behaviour.md` se o layout mudar; rota `ShapeKind::Line`: `entities/geometry.md`; rota `ShapeElem`: `entities/elements/shape.md` + `entities/content.md`; rota `Path` não requer novo owner medido |
| color.mix N | `compiler/stdlib/color.md`; `entities/color.md` | `compiler/eval/bindings/value_methods.md` somente se o glue AST mudar |
| pdf.attach | `compiler/stdlib/pdf.md`; `infra/export/builder.md` | novo L0 de entidade; `entities/content.md`; `entities/layout_types.md`; `infra/pipeline.md`; `infra/export/mod.md`; novos L0 de layout/export conforme os ficheiros criados |
| pdf.artifact | `compiler/stdlib/pdf.md`; `entities/content.md`; `entities/layout_types.md`; `infra/export/stream.md`; `infra/export/builder.md`; novos L0 de entidade e layout | `infra/pipeline.md` apenas se walkers/dataflow mudarem; `infra/export/mod.md` apenas se API pública mudar |

## 10. Bloqueios V15/V26 potenciais

1. **V15 — reutilizar `compiler/stdlib/pdf.md` nos novos ficheiros.** Hoje ele
   é owner apenas de `compiler/stdlib/pdf.rs` (header `:2`). Apontar entidade,
   layout ou exporter ao mesmo prompt cria `1:N` e bloqueia resselo.
   A linha 5 desse L0, que também nomeia `eval/mod.rs`, deve ser saneada; ela
   não transfere ownership contra o header proprietário de `eval/mod.rs`.
2. **V15 — um prompt conjunto para attach e artifact.** São consumers e
   obrigações distintas: attach é efeito global/embedding; artifact é wrapper
   semântico/tagging. Cada novo ficheiro produtivo recebe exatamente um prompt
   proprietário.
3. **V15 — colocar dois consumers num novo L0 de “pdf semantics”.** O texto
   compartilhado deve permanecer completo nos prompts próprios; somente claims
   genuinamente comuns podem ser extraídas para Núcleo.
4. **V26 — código apontando a Núcleo.** Nenhum `@prompt` pode apontar para
   `_nuclei/**`; novos layout files apontam ao seu L0 proprietário.
5. **V26 — Núcleo órfão ou pin copiado.** Um eventual Núcleo PDF precisa de ao
   menos dois prompts consumidores, DAG válido e pin efetivo completo em cada
   um. Os pins atuais de bitmap/SVG não cobrem attachments ou artifacts.
6. **Preflight.** ADR-0129 proíbe `--fix-hashes` enquanto V15/V26 falhar. O
   estado atual passa, mas qualquer lote novo deve repetir
   `--checks v15,v26 --fail-on warning` antes do resselo.

## 11. Gates ADR-0127 abertos, sem veredito

- `smartquote`: gate condicional — fluxo contínuo somente se o carrier interno
  de styles preservar a morfologia sem API/default/fase nova; paragem
  obrigatória se acrescentar campos públicos ou mover fase.
- `line(start != 0)`: gate condicional — fluxo contínuo somente se o lowering
  para `ShapeKind::Path` vigente provar paridade no nível da língua; paragem
  obrigatória em qualquer carrier público novo.
- `color.mix` N-ário: fluxo contínuo somente se conservar APIs Rust públicas;
  criar helper público muda o gate.
- `pdf.attach`: paragem obrigatória por novo carrier público; dúvida adicional
  de fase até escolher o canal global.
- `pdf.artifact`: paragem obrigatória por nova identidade pública de conteúdo e
  discriminação semântica no frame.

Este receipt não escolhe carriers, não remove scope-outs, não mede paridade de
output, não aprova defaults e não emite veredito de materialização.
