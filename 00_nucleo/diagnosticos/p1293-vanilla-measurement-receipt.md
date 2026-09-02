# P1293 — recibo de medição vanilla independente

## Autoridade, regime e limites

- papel: `medidor_p1293`, sequência causal 1;
- regime: protocolo completo da skill `tekt-materializacao-segregada`;
- manifesto: `00_nucleo/diagnosticos/p1293-manifest.json`, SHA-256
  `1cb2bf59d9294e71bb9e94480386e46151fd08e0facb8bb6327ac3642e428ff7`;
- execução: segregada por capacidades e artefatos, sem isolamento técnico de
  leitura porque o filesystem é compartilhado;
- escrita exercida: somente este recibo, `p1293-baseline-status.txt` e
  `/tmp/p1293-*`;
- não foram escritos L0, contrato, código, testes, oráculos, ataques ou
  veredito;
- política de `Unknown`: nunca é sucesso. Não houve `Unknown` nesta medição.

Este recibo mede o fragmento P1293. Não certifica paridade funcional geral e
não é o veredito final do passo.

## Proveniência reproduzível

Medição antes da decisão:

| Entrada | Identidade |
|---|---|
| instante do baseline fresco | `2026-09-01T13:24:01-03:00` |
| `HEAD` / branch | `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt` |
| árvore | não commitada; lista exata em `p1293-baseline-status.txt` |
| SHA-256 de `git status --short` | `6e06c61607145ee21afc7be6a7a2464040d043dafd714569e9d29d7f808519c3` |
| SHA-256 de `git diff HEAD --stat` vazio | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| vanilla ratificado | `/usr/local/bin/typst`, `a51e02804`, SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| cristalino medido | `target/release/typst`, SHA-256 `ed5f85e03e6fe473c1a7a9a42cceafe8c5fbe075de4dad56a7c88c07ad2c3b6f` |
| runner de superfície | `lab/surface-inventory/run_probes.py`, SHA-256 `3bd082751fbd05c89f24a312353d81f74f2882902db5a203180e5ee5fb2c10cf` |
| catálogo de probes | `lab/surface-inventory/probes.json`, SHA-256 `1e7534a0cc8ebcfa1f5ed6652918711b738d7471d94a883a500becf297357b78` |
| harness independente | `/tmp/p1293-measure.py`, SHA-256 `3f4a4e96ddd1fff309de1aa6bbd02e8355193ab9ad0a4e2673c3aff97c861ab9` |
| recibo bruto | `/tmp/p1293-vanilla-probes.json`, SHA-256 `7f4ad7928dfa1be88cf51da141b8a9dbb01330e46eda4c468acdbb19d1a9c36f` |

O recibo bruto contém 203 casos nomeados, 624 tentativas de cast distribuídas
pelos 48 atributos específicos, 5 compilações HTML e 10 compilações SVG. Para
cada execução guarda argv, stdin quando aplicável, exit code, stdout, stderr,
SHA-256 e comprimento. Os spans abaixo vêm do stderr bruto, não de inferência.

Comandos principais:

```text
python3 /tmp/p1293-measure.py

python3 lab/surface-inventory/run_probes.py \
  --profile default \
  --vanilla-bin /usr/local/bin/typst \
  --crystalline-bin target/release/typst \
  --output /tmp/p1293-surface-default.json \
  --probes /tmp/p1293-default-probes.json

python3 lab/surface-inventory/run_probes.py \
  --profile html \
  --vanilla-bin /usr/local/bin/typst \
  --crystalline-bin target/release/typst \
  --output /tmp/p1293-surface-html.json \
  --probes /tmp/p1293-html-probes.json
```

As listas `/tmp/p1293-{default,html}-probes.json` foram derivadas
mecanicamente de `results` dos inventários fixos P1292, removendo somente
saídas/vereditos anteriores e preservando ids e expressões. Seus hashes são
`3b162e4bbc58b4855cbbad2645c6403ea45627c93ef16` e
`d91e9a1e5cbe71118798378ec85829b18585247164596012fdc33cc2bd58d4e2`.

## Superfícies frescas e as 15 diferenças P1293

Medição:

| Perfil | Recibo fresco | Resultado |
|---|---|---|
| default | `/tmp/p1293-surface-default.json`, SHA-256 `f8a7d54a092a4c1bcf09882492b3d2fbc46bed1780b2d96e0e0b84ceb3014c83` | `111 total / 99 MATCH / 12 diferenças` |
| HTML | `/tmp/p1293-surface-html.json`, SHA-256 `f3cc13993eaed6f92e5c6a3380d17eea32540d4bd7da05d99658ca33c43ad19e` | `115 total / 89 MATCH / 26 diferenças` |

As contagens reproduzem exatamente P1292. No perfil HTML, as 26 diferenças
continuam decompostas em 1 membro desligado (`pdf.data-cell`), 10 extras
cristalinos deliberados, 12 membros ausentes e 3 identidades públicas erradas.

Prova nominal das 15 diferenças acionáveis:

| Classe | Nome | Vanilla | Cristalino |
|---|---|---|---|
| ausente | `float.is-nan` | `(function, "is-nan")` | field ausente |
| ausente | `math.attach` | `(function, "attach")` | member ausente |
| ausente | `math.binom` | `(function, "binom")` | member ausente |
| ausente | `math.mono` | `(function, "mono")` | member ausente |
| ausente | `math.script` | `(function, "script")` | member ausente |
| ausente | `html.button` | `(function, "button")` | member ausente |
| ausente | `html.col` | `(function, "col")` | member ausente |
| ausente | `html.iframe` | `(function, "iframe")` | member ausente |
| ausente | `html.select` | `(function, "select")` | member ausente |
| ausente | `html.template` | `(function, "template")` | member ausente |
| ausente | `html.video` | `(function, "video")` | member ausente |
| ausente | `html.wbr` | `(function, "wbr")` | member ausente |
| nome | `grid.cell` | `(function, "cell")` | `(function, "grid_cell")` |
| nome | `table.cell` | `(function, "cell")` | `(function, "table_cell")` |
| nome | `table.header` | `(function, "header")` | `(function, "table_header")` |

Decisão após a medição: o baseline fresco reproduz todas as 15 diferenças;
portanto a primeira condição de paragem de P1293 não ocorreu. O perfil default
preservou `99/111` e o HTML preservou `89/115` antes de qualquer L0.

## Lote A — `float.is-nan`

### Fonte medida antes da decisão

- `lab/typst-original/crates/typst-library/src/foundations/float.rs:12-30`
  define `float` e declara que `int` é aceito onde float é esperado;
- `float.rs:32-39` instala o scope e as constantes `inf`/`nan`;
- `float.rs:67-80` define `is_nan(self) -> bool` delegando a
  `f64::is_nan`;
- hash da fonte: `8e5c3d84b0263d6217e0f3d3d73dd114127f87f0d913a40eb57b5be6df74b8b2`.

### Binário, assinatura, casts e spans

- `repr((type(float.is-nan), repr(float.is-nan)))` →
  `(function, "is-nan")`;
- estática: NaN → `true`; `1.25`, `1`, `+inf` e `-inf` → `false`;
- ligada em Float: NaN → `true`, finito/inf → `false`;
- ligada em Int: rejeitada (`type integer has no method is-nan`), logo a
  coerção Int pertence à forma estática, não ao receiver ligado;
- acesso ligado sem chamada `float.nan.is-nan` é rejeitado: `cannot access
  fields on type float`;
- missing: `missing argument: self`, span `<input-expression>:1:0`;
- extra estático: `unexpected argument`, span `1:18` sobre `2.0`;
- named na forma ligada: `unexpected argument: value`, span `1:13`;
- string: `expected float, found string`, span `1:13`.

Decisão: assinatura fechada de um argumento Float com coerção Int na forma
estática; a forma ligada delega a mesma semântica apenas para receiver Float.
Não há fórmula, trait ou registry novo exigido. Refutador: fonte/binary que
aceitasse named, permitisse obter o método ligado como valor ou classificasse
infinito como NaN.

## Lote B — `math.attach`, `binom`, `mono`, `script`

### Fonte medida antes da decisão

- `math/mod.rs:42-92` registra canonicamente `AttachElem`, `BinomElem`, `mono`
  e `script` no mesmo módulo;
- `math/attach.rs:19-49` contém exatamente sete campos: `base` obrigatório e
  `t`, `b`, `tl`, `bl`, `tr`, `br` opcionais;
- `math/frac.rs:133-150` define `upper` obrigatório e `lower` requerido,
  variádico e não vazio;
- `math/style.rs:134-145` faz `mono` aplicar `MathVariant::Monospace` ao body;
- `math/style.rs:208-227` faz `script(body, cramped: bool = true)` aplicar
  `MathSize::Script` e o valor de `cramped`;
- `math/ir/resolve.rs:402-415` e `470-570` convertem os seis attachments em
  um único `ScriptsItem` canônico;
- `math/ir/resolve.rs:714-768` converte binom em fração vertical com
  `line = false`, insere vírgulas em ordem e envolve por delimitadores
  esticáveis;
- `typst-layout/src/math/fraction.rs:85-120` contém a matemática de layout da
  fração sem linha;
- `typst-layout/src/math/scripts.rs:99-210` contém a matemática única dos seis
  slots e usa `EquationElem::cramped`;
- hashes principais: `attach.rs` `7efdda52…`, `frac.rs` `b3d7d9e6…`,
  `style.rs` `2fe42e8b…`, `ir/resolve.rs` `115d7756…`, layout fraction
  `6f1ccbf9…`, layout scripts `d3f8a9fc…`.

### Assinaturas, defaults, casts, named, spans e repr

`attach`:

- identidade `(function, "attach")`;
- default `attach(base: [x])`;
- payload completo preserva os seis nomes e a ordem;
- `none` explícito aparece no repr como `t: none` etc., mas não vira texto:
  SVG de default e `t: none` é byte-idêntico, SHA-256
  `7b69129be07d8a3a2eeac947f83cd061e17a5561f507c1d2f87024c5c8f4efc7`;
- `base` e slots exigem `content`; inteiro é rejeitado em `1:17`;
- missing base em `1:0`, segundo posicional em `1:17` e named desconhecido
  em `1:17` são rejeitados.

`binom`:

- identidade `(function, "binom")`;
- dois args → `binom(upper: [n], lower: ([k],))`;
- quatro args preservam `([k], [j], [m])`;
- inteiro é rejeitado como content em `1:16`;
- zero args: `missing argument: upper`; um arg: `missing argument: lower`;
- named desconhecido em `1:21` é rejeitado; `upper` pode ser named, mas isso
  não satisfaz o lower variádico posicional.

`mono` e `script`:

- identidades curtas `(function, "mono")` e `(function, "script")`;
- ambos exigem body content posicional; inteiro, missing, extra e named body
  são rejeitados com span no argumento/call;
- `script` aceita só `cramped: bool`; string em `1:26`, segundo posicional em
  `1:17` e named desconhecido em `1:17` são rejeitados;
- `script([x])`, `cramped: true` e `cramped: false` têm morfologia
  `styled(child: [x], ..)`, enquanto o layout abaixo distingue o valor quando
  ele é observável.

### Sintaxe versus função e layout

Os pares `$ attach(...) $`/`$ #math.attach(...) $`, `$ binom(...) $`/
`$ #math.binom(...) $`, `$ mono(...) $`/`$ #math.mono(...) $` e
`$ script(...) $`/`$ #math.script(...) $` produziram repr lado a lado
idêntico no nível de morfologia. Não se usou `PartialEq` de Content como prova.

Fingerprints SVG vanilla:

| Caso | viewBox | SHA-256 |
|---|---|---|
| attach display, 6 slots | `23.2705 × 19.4843` | `421fc32b2ab73b4e116ad1300958b32259915759c5dde8d95367df1559a23f72` |
| attach inline | `19.7681 × 9.6041` | `c0eb41fbfa15896b19497da9c289f83070ec88b7a9d75934f61f70100b78971c` |
| binom display, 3 lower | `46.797666667 × 24.057` | `279ec8fd256017bff534540e5e7357a549b559b1056c9b10d75eb515000610ea` |
| binom inline | `30.3325 × 7.8815` | `2edbdc73366618cfc7a6933e05844510252fd533a53a7eb42dcd31532acb1258` |
| mono display | `25.029888889 × 8.921` | `c742a7f5c3a16810abade9fd7d9bcf00bdd426b657481a0b98aac9ae50129582` |
| mono em script | `14.0987 × 6.2447` | `21b7dfb57338f087e1257d879f3b3637a39a9f3263d7166f67f37f90aa090d70` |
| script cramped true | `8.3578 × 5.9708` | `1fda5c6cf0fd678af743aaa91f6d90fc12dc6929764c4f843984a61b7c103710` |
| script cramped false | `8.3578 × 6.5406` | `da1bbb4b65548b8873cbc95883c201af7aa44d666c49a08f77797e71bb23f315` |

Decisão: A–B cabem nos payloads e na fase vigentes. Não foi observada
necessidade de novo campo público, entidade ou fase. Inferência: `none`
explícito é ausência semântica apesar de continuar explícito no repr do valor;
refutador: layout/IR que materialize glyph/texto ou altere o fingerprint.

## Lote C — sete constructors HTML

### Fonte medida antes da decisão

- `typst-html/src/lib.rs:33-40` registra o módulo e chama o dispatcher tipado;
- `typed.rs:30-43` cria todas as funções a partir da tabela pinada;
- `typed.rs:45-70` fixa nome público e metadata;
- `typed.rs:73-115` põe todos os atributos como named opcionais e só adiciona
  body posicional opcional a não-void;
- `typed.rs:117-160` consome apenas atributos pertencentes ao elemento,
  preserva a ordem de chamada, faz casts, constrói `HtmlElem` e omite body em
  void;
- `typed.rs:162-247` define Presence, nativos, enums, unions e listas; Presence
  true vira string vazia e false retorna ausência;
- `tag.rs:123-141` prova nominalmente que `col` e `wbr` são void;
- `convert.rs:165-245` converte body/nesting/attrs para o DOM genérico;
- `encode.rs:111-167` preserva ordem, escapa atributos, rejeita filhos void e
  omite end tag de void;
- hashes: `typed.rs` `b702bfe0…`, `tag.rs` `d85ce308…`, `convert.rs`
  `d3f4dd52…`, `encode.rs` `9176b861…`.

### Funções, defaults, body e erros

- as sete identidades são funções com nomes curtos `button`, `col`, `iframe`,
  `select`, `template`, `video`, `wbr`;
- chamada vazia das cinco normais → `elem(tag: ..., body: none)`;
- chamada vazia de `col`/`wbr` → `elem(tag: ...)`, sem body;
- body e nesting `html.span([inner])` são preservados nas cinco normais;
- body em `col` e `wbr` é `unexpected argument`, span `1:9`;
- named desconhecido é rejeitado em todos os sete; `data-p1293` real e
  `data_p1293` também são rejeitados;
- um atributo específico de outra tag foi rejeitado nominalmente para cada
  tag (`button.span`, `col.disabled`, `iframe.size`, `select.src`,
  `template.controls`, `video.command`, `wbr.span`).

### Inventário e casts dos 48 atributos específicos

O conjunto nominal reproduz exatamente a tabela do passo:

| Tag | Casts medidos |
|---|---|
| `button` | `command`: string (metadata enum-or-string); `commandfor`, `form`, `formaction`, `name`, `popovertarget`, `value`: string; `disabled`, `formnovalidate`: Presence; `formenctype`: enum `application/x-www-form-urlencoded`/`multipart/form-data`/`text/plain`; `formmethod`: `GET`/`POST`/`dialog`; `formtarget`: `_blank`/`_self`/`_parent`/`_top` ou string; `popovertargetaction`: `toggle`/`show`/`hide`; `type`: `submit`/`reset`/`button` |
| `col` | `span`: inteiro positivo; `0` e negativo rejeitados |
| `iframe` | `allow`, `src`, `srcdoc`: string; `allowfullscreen`: Presence; `height`, `width`: inteiro não negativo; `loading`: `lazy`/`eager`; `name`: target conhecido ou string; `referrerpolicy`: `none` vazio ou os 8 tokens da fonte; `sandbox`: lista de 13 tokens, serializada com espaço |
| `select` | `autocomplete`: lista de tokens fechados, serializada com espaço; `disabled`, `multiple`, `required`: Presence; `form`, `name`: string; `size`: inteiro positivo |
| `template` | `shadowrootclonable`, `shadowrootcustomelementregistry`, `shadowrootdelegatesfocus`, `shadowrootserializable`: Presence; `shadowrootmode`: `open`/`closed` |
| `video` | `autoplay`, `controls`, `loop`, `muted`, `playsinline`: Presence; `crossorigin`: `anonymous`/`use-credentials`; `height`, `width`: inteiro não negativo; `poster`, `src`: string; `preload`: `none`/`auto`/`metadata` |

Todos os 48 tiveram matriz boolean/int/float/string/none/auto/list e pelo
menos um caso válido ou uma enumeração fechada emitida pelo próprio binário;
as enums sem candidato inicial foram repetidas com token/lista válida. Exemplo
global `id`, `class`, `hidden` também foi aceito, com `hidden: true` serializado
como Presence vazia.

### Feature versus target

| Feature HTML | target paged | target HTML |
|---|---|---|
| off | acesso a `html` rejeitado | export HTML rejeitado por feature ausente |
| on | módulo/função/call disponíveis | módulo/função/call disponíveis; warning experimental |

Decisão: feature e target são eixos independentes. O target não habilita a
feature. Refutador: qualquer execução on/paged rejeitada ou off/HTML aceita.

### DOM, atributos, nesting, void e escaping

Compilação `typst compile - - --format html --features html`:

- `all-seven`, SHA-256 `1228ac50512480745db4f152b1e040f2bd0910ec46c96029459c7750ee017a82`:
  contém `<button command="save" disabled type="submit">A&lt;&amp;B</button>`,
  `<col span="2">`, iframe contendo select, template contendo `<wbr>`, e
  video com `controls` mas sem `muted` falso;
- `attribute-order`, SHA-256
  `2194394850fde2495287dbb3f0be51fa6f13723a7c06a2cd4a3569367d6092a4`:
  emite `value`, `name`, `command`, `formnovalidate` nessa ordem e omite
  `disabled: false`;
- `void-only`, SHA-256
  `072b5b697f4eb6d9335f00592965299ec36c160df4f10fa0d9cbd979489a8bf1`:
  `<col span="3"><p><wbr></p>`, sem `</col>` ou `</wbr>`;
- `escaping`, SHA-256
  `81697b68d1e4429a3e9a8d02676ef377b23d8a30d8d813ff487c870b062ce51e`:
  atributo `x&amp;&quot;<>` e body `&lt;&amp;>`;
- compile sem feature falha e produz stdout vazio SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

Decisão: o exporter genérico satisfaz os observáveis medidos; não apareceu
necessidade de comportamento de browser, CSS, mídia ou rede. Logo nenhuma
condição de paragem C ocorreu.

## Lote D — dez siblings grid/table

### Fonte medida antes da decisão

- vanilla `layout/grid/mod.rs:422-438` instala os cinco tipos no scope de
  `grid`; os elementos declaram nomes públicos curtos em `:574-677` e
  `GridCell` em `:767-768`;
- vanilla `model/table.rs:289-305` instala os cinco tipos no scope de `table`;
  nomes curtos estão em `:494-613` e `TableCell` em `:732-733`;
- cristalino `compiler/eval/mod.rs:1804-1827` mostra que cada binding de
  namespace usa a mesma função nativa de sempre, mas passa `grid_*` como
  primeiro argumento textual de `Func::native`;
- cristalino `compiler/eval/mod.rs:2059-2078` faz o mesmo para `table_*`;
- os seis aliases flat existentes e separados são instalados em
  `compiler/eval/mod.rs:2083-2110`;
- hashes: vanilla grid `baa86ce2…`, vanilla table `46880d72…`, cristalino eval
  `f774abf7…`.

### Identidade, chamadas, aridade, defaults, named e `.with`

- vanilla: todos os dez `repr` são os nomes curtos `cell`, `header`, `footer`,
  `hline`, `vline` em ambos os owners;
- cristalino: os dez são `grid_*`/`table_*`;
- as vinte chamadas bilaterais (dez diretas e dez via `.with`) retornam
  `content` em ambos; dentro de cada produto, chamada direta e `.with`
  preservam o mesmo payload observado;
- vanilla: `cell` exige um body posicional; `header`/`footer` aceitam zero ou
  vários children; `hline`/`vline` aceitam chamada vazia; named desconhecido
  e posicional extra são rejeitados com span no argumento;
- cristalino vigente diverge fora do nome: `header`/`footer` exigem pelo menos
  uma célula e algumas chamadas extras chegam ao serializer em vez do mesmo
  diagnóstico. Esta divergência foi medida, não escondida, mas não pertence às
  15 diferenças fixadas e P1293 ordena preservar aridade/diagnósticos atuais;
- aliases flat: vanilla não possui nenhum dos dez; cristalino possui os seis
  históricos `grid_cell/header/footer` e `table_cell/header/footer`; os quatro
  `*_hline`/`*_vline` são bilateralmente ausentes.

Inferência após fonte + binário: trocar somente o primeiro argumento textual
das dez instâncias namespaced preserva function pointer e chamada; não exige
renomear símbolo Rust nem binding flat. Refutador obrigatório pós-patch: algum
dos dez tipos/payloads/pares `.with` mudar, algum alias flat mudar, ou uma
instância ainda imprimir underscore. Como o refutador é posterior, isto é uma
inferência de escopo, não um veredito de implementação.

Decisão: a causa das dez identidades é nominal e localizada; a condição de
paragem “correção alterar identidade funcional ou chamada” não foi observada
no baseline. A medição cobre os dez siblings, não apenas os três amostrados.

## Condições de paragem e handoff

Checagem final do papel medidor:

- 15 diferenças frescas: reproduzidas nominalmente;
- inventário de 48 atributos: coincide com o passo e com o binário pinado;
- attach: sete campos públicos já existentes, sem campo novo;
- binom: mesma fase/payload, fração vertical vigente sem barra;
- HTML: nenhum comportamento de browser/CSS/mídia/rede requerido;
- grid/table: alteração projetada é somente nome textual, com refutador
  pós-patch explícito;
- entrada selada alterada: não;
- `Unknown`: zero.

Resultado do medidor: **nenhuma condição de paragem imediata foi acionada**.
O próximo papel pode usar este recibo como baseline para redigir L0/contrato,
preservando as divergências fora de escopo explicitamente registradas. O gate
humano pós-L0 de P1293 continua obrigatório.
