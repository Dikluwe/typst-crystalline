# P1285 — receipt do contrato observável congelado pós-gate

**Estado:** `CONTRACT_FROZEN_POST_L0_GATE`  
**Regime:** protocolo completo de materialização segregada; autoria independente do
contrato, sem atestação de isolamento ambiental forte.  
**Instante da medição:** 2026-08-30T13:29:00-03:00.  
**Gate ADR-0127 confirmado pelo dono:** 2026-08-30.  
**HEAD do repositório hospedeiro:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este documento é diagnóstico e contrato congelado após o gate humano. Os nove L0
finais são a autoridade que legitima a materialização; este receipt não é Prompt L0,
suíte de testes, selo discriminatório nem veredito sobre qualquer implementação P1285.

## 1. Autoridade, isolamento e capacidades

Papel exercido: **autor do contrato**. Entradas autorizadas: AGENTS, skill e suas
referências, o Passo 1285 explicitamente indicado, ADR-0107/0108/0127/0129, L0
proprietários necessários, fonte e binário vanilla ratificados. Escrita autorizada:
somente este receipt.

Restrições efetivamente preservadas:

- não foi lido código candidato cristalino P1285;
- não foi lido `git diff` nem foram inspecionadas alterações candidatas;
- não foram escritos código, testes candidatos, oráculos executáveis, mutantes ou
  veredito;
- não foram lidos outros ficheiros de `materialization/` ou qualquer ficheiro de
  `context/`;
- a working tree não foi usada como oráculo. O seu estado não foi auditado porque a
  restrição de capacidade proíbe `git diff`; por isso este receipt não alega isolamento
  atestado, apenas segregação por allowlist e disciplina de leitura.

Executores futuros não podem tratar o nome deste papel como prova de independência. A
capacidade concedida e os hashes abaixo são parte da evidência.

## 2. Manifesto congelado pós-gate

Identificador lógico: `P1285-CONTRACT-v4`. O manifesto e o contrato estão embutidos
neste receipt; o SHA-256 externo do ficheiro identifica ambos após a escrita. Não há
hash autoembutido, porque isso seria autorreferente.

Adendo de precisão: `CONTRACT_V4_PRECISION_ADDENDUM`, incorporado antes da candidata
v3. Ele torna explícitos três canónicos de testemunhas já medidas em C-JSON; não altera
L0, ownership, obrigação semântica, matriz de mutações, gate ou versão lógica do
contrato.

O v0 foi invalidado antes de código quando o coordenador registou que uma label é
transportada por wrapper separado no cristalino. A cadeia foi reiniciada na primeira
fase afetada; os L0 de CLI e query-helper incorporaram a preservação de
`Content::Label`, foram novamente ressellados, e só então este v1 foi repinado. A
confirmação humana foi transmitida ao autor deste contrato pelo coordenador `/root` e
está gravada expressamente nos L0 finais de CLI/eval/repr em 2026-08-30.

O v1, cujo SHA-256 externo era
`9897a6f893734b66b2d1c93ee653bc1155ab0c51f705c89f22011ee73a3c77ec`, foi
invalidado causalmente antes da primeira edição candidata quando se descobriu o sexto
owner `prompts/wiring.md`. O novo L0 fixa o transporte de `QueryFormat` através de L4 e
o tratamento de input `-` como `Source` markup transitória. Este v2 só foi recongelado
depois de o sexto owner ser atualizado e ressellado.

O v2, cujo SHA-256 externo era
`94bb0f0115e5179b3c744834402058343a53792df8eae1c69102cad403595aa9`, foi
invalidado causalmente depois da implementação candidata v1 e do resultado parcial de
oráculo `26/42`, mas antes da revisão candidata v2. A medição revelou obrigações ainda
não congeladas: constructor `selector` por identidade/regex/vazios e show regex por
ocorrência. `compiler/eval/rules.md` e `compiler/stdlib/foundations/selector.md`
entraram como owners adicionais, enquanto `compiler/eval/selector_matching.md` mudou de
bytes. A candidata v1 e o resultado `26/42` permanecem apenas evidência diagnóstica da
lacuna; não são evidência final, score, selo nem veredito sob este v3.

O v3, cujo SHA-256 externo era
`1ea58e14a01f6cb55fe38695133f4c279164ca2943134d2dcf5cab81c3df681e`, foi
invalidado causalmente antes da revisão semântica seguinte quando se descobriu o nono
owner `compiler/eval/math.md`. O owner preserva a distinção morfológica matemática:
`MathTextKind::Grapheme` produz `Content::MathIdent`, enquanto
`MathTextKind::Number` produz `Content::MathText`. As candidatas v1 e v2 parcial e os
receipts v2/v3 não são evidência final sob este v4; permanecem apenas proveniência da
descoberta causal, sem score, selo ou veredito.

### 2.1 Entradas normativas

| Entrada | SHA-256 |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| `tekt-materializacao-segregada/SKILL.md` | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| `materialization/typst-passo-1285.md` | `5a69234ba5340c92e7cfe523212b102869e8a7fcefafb4647e4acf1dc3690a6c` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |

### 2.2 Nove L0 finais P1285, relidos e congelados

| Prompt L0 | SHA-256 sem a linha `Hash do Código` | SHA-256 integral observado |
|---|---|---|
| `prompts/shell/cli.md` | `8fe484152e1371d7ef4a79659d6388bf5e63ff63ff84dc5698db7147a33543a7` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `7b2dc94f28457d84022f59a80b858c12428f671e67e3a5e90e6c0865d773e6c3` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `3f1bcd114c5f880e10fedd3f5fbaee868d456b9c31b244244cb7a8a15d23860b` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` | `557abd0a6fb52f8b198606e928ba23fd65234e3b75c2a14703b07a9975ba1b11` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `prompts/compiler/eval/rules.md` | `3655f2922835fd0bfcf0a1857a6d12d70ac9c30eae23f9f7b9eea1f69d9fc7c0` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` |
| `prompts/compiler/stdlib/foundations/selector.md` | `9bea0284d242754ca1103baf45c53b7614817cfff1e5358fd708980f971f7c22` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` |
| `prompts/infra/query-helpers.md` | `352604527883503c72e349f9171cdb92cf730af3772cde28820275f11005713d` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` | `2142e48948482f1f86a8390b4dbd40cdc84f67f1783d5c9f3f370e9ed1401a44` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `prompts/compiler/eval/math.md` | `c2ba2c5f028cd7424d15f92486e1d281a6efca1b74696f75249c6b9f1a018a9c` | `7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2` |

O hash sem selo foi calculado para cada L0 com
`sed '/^Hash do Código:/d' "$f" | sha256sum`; ele é o pin canónico da obrigação
semântica neste contrato. O hash integral é preservado como proveniência do artefacto
exatamente relido.

Auditoria do drift de selo: as seções P1285 de `selector_matching.md`, `rules.md` e
`foundations/selector.md` foram relidas integralmente contra o corpo normativo capturado
no v3. Fora da linha `Hash do Código`, o corpo é idêntico. Os hashes integrais anteriores
eram, respectivamente, `fd7a0b50308e119932d9fee90f9aa972d4a6ebab55ad31b15aa9decead59cae6`,
`c97c2f6fc0275e37667b1d183ada5437e523ab3248b2549536da7b038bba875f` e
`6849e281548fd239321dab6fdf31a9a6473ac8bd7a9bc6d7fd8a78c4be63d7d8`.
O novo full hash registra apenas o resselo causado pelos consumers candidatos parciais.

Política de seal para os nove owners: mudança futura restrita exatamente à linha
`Hash do Código:` é drift de selo permitido e não causa restart semântico se o hash
sem essa linha permanecer igual; o novo full hash deve apenas ser registrado como
proveniência. Qualquer mudança no hash sem selo invalida o contrato e reinicia a cadeia
na primeira fase afetada. O full hash continua relevante para identificar o artefacto e
auditar linhagem, mas não define sozinho a identidade semântica congelada.

### 2.2a L0 de dependência lidos, não alterados por P1285

| Prompt L0 | SHA-256 |
|---|---|
| `prompts/entities/show.md` | `2bf028594608a4ee68b0153dd671ef3176b7c28030713c848f00a280c0d6e92a` |
| `prompts/compiler/stdlib/foundations/query.md` | `5f1d37a14449eb9880118866d5fee07b0e4f69ffcf367c3428c4b704ba999121` |
| `prompts/entities/value.md` | `9fe2e8cb8bf806f39ac9e79b77d217d3fdac59475aafc18dd60da44e388c9a40` |
| `prompts/entities/version.md` | `b28c5144e36183c5c9d4eaa7dc4fcbbe38a9b7c202a1049d3ce661e477745b64` |
| `prompts/entities/selector.md` | `8dcd7bfbc6789ed0ef669d8ffd943df63aa06343f0c4ef39f23e797ec276b429` |

`value.md`, `version.md` e `selector.md` foram descobertos como dependências
necessárias para classificar os valores e selectors públicos. Não são, por isso,
automaticamente owners de alterações P1285.

### 2.3 Baseline ratificado

- identidade normativa: upstream/main `a51e02804`, conforme AGENTS;
- binário medido: `/usr/local/bin/typst`;
- SHA-256 do binário: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- `--version`: `typst 0.15.1 (e0e8ca4d)`; esta string não identifica o commit
  vanilla porque o build do lab herda o repositório hospedeiro, armadilha já
  documentada em AGENTS;
- a identidade de paridade usada aqui é, portanto, o pin ratificado + o hash do
  binário + hashes das fontes, nunca a string de versão isolada.

Fontes vanilla decisivas e SHA-256:

| Fonte | SHA-256 |
|---|---|
| `typst-cli/src/args.rs` | `ba140a3ac35ef876f52d5069b24a8346c00abe8758fe291b2c9216128bd21414` |
| `typst-cli/src/eval.rs` | `f582643a9cc8de2c56dbb0b0bf5c3b39543675ba5a83a5d2dfa49e65fc8706c6` |
| `typst-cli/src/main.rs` | `16073e135c38d80ac7e282c6832f2ca6514538ce3a5975da3525cac2670931df` |
| `typst-cli/src/query.rs` | `665f64232e6bb9bbf78cf675dc8676aec32b97608612f7dff8696c4bd1b9cbc7` |
| `foundations/value.rs` | `c25a2bdb0a96c707414f1138f34bcc21c63ac06c507a32c81f029f6a374da885` |
| `foundations/content/mod.rs` | `70f2fe3abe9ed674452ab0d26729e76ac3ebef540b41420fb9feb5a1ab6dd0c9` |
| `foundations/selector.rs` | `84615405b19d4869d261029936d3d5d9077f06e7198e3f60f28273c1ecdfcbdc` |
| `typst-realize/src/lib.rs` | `fd4223ac9de5604b9807a326cd6174293803e4b1cc173caa1820f9a61a258f09` |
| `typst-eval/src/math.rs` | `4aad95925fbc3542fc5709ca4e053895e6aac4c6620195a305a07dab6a24bf7f` |

## 3. Medição antes da decisão

### 3.1 Formatos públicos

Fonte:

- `args.rs:715-733`: query aceita `json|yaml`; eval aceita `json|yaml|raw`;
- `main.rs:115-131`: JSON pode ser compacto ou pretty; YAML usa um único
  serializer, ignorando `pretty`;
- `eval.rs:129-154`: JSON/YAML acrescentam newline; raw escreve bytes sem
  formatação e só aceita string/bytes;
- `query.rs:102-123`: `--one`, `--field` e formato são aplicados depois da
  recuperação dos elementos.

Sondas reproduzíveis:

```sh
/usr/local/bin/typst eval --help
/usr/local/bin/typst query --help
/usr/local/bin/typst eval '(a: 1, b: (2, 3), v: sys.version, t: [Hi])' --format json
/usr/local/bin/typst eval '(a: 1, b: (2, 3), v: sys.version, t: [Hi])' --format yaml
/usr/local/bin/typst eval '(a: 1, b: (2, 3))' --format yaml --pretty
/usr/local/bin/typst eval 'sys.version' --format raw
/usr/local/bin/typst eval '1' --format toml
```

Resultados medidos:

- JSON composto: `{"a":1,"b":[2,3],"v":"version(0, 15, 1)","t":{"func":"text","text":"Hi"}}\n`, exit 0;
- YAML composto, exit 0:

  ```yaml
  a: 1
  b:
  - 2
  - 3
  v: version(0, 15, 1)
  t:
    func: text
    text: Hi
  ```

- `--pretty` não altera YAML;
- raw de `Version` falha, exit 1, com `cannot print version in raw format` e o
  hint que restringe raw a strings/bytes;
- `--format toml` falha no parser, exit 2, e enumera `json, yaml, raw`.

Classificação: nomes de opções, stdout/stderr e exits são CLI pública; a árvore
serializada é semântica/morfologia da linguagem. Indentação específica do YAML é
mecânica salvo quando necessária para preservar a árvore. Inferência: YAML deve usar a
mesma classificação de valores que JSON. Refutação: uma sonda em que JSON e YAML do
mesmo valor produzam árvores semanticamente diferentes.

### 3.2 Serialização de `Value`

Fonte `foundations/value.rs:343-362`:

- `none`, bool, int, float e string serializam estruturalmente;
- bytes e symbol usam a sua serialização pública;
- content, array e dict serializam recursivamente;
- todas as demais variantes caem para **string contendo `repr()`**.

`foundations/content/mod.rs:709-718` serializa qualquer `Content` como mapa iniciado por
`func`, seguido de todos os seus fields públicos. Isto refuta uma serializer nominal
exclusiva de heading como contrato de paridade.

Sondas:

```sh
/usr/local/bin/typst eval 'sys.version' --format json
/usr/local/bin/typst eval '(k-none: none, k-auto: auto, k-length: 12pt, k-symbol: emoji.heart, k-version: sys.version, k-bytes: bytes((65, 66)), k-label: <intro>, k-decimal: decimal("123.4500"), k-duration: duration(seconds: 2), k-func: calc.gcd, k-type: int)' --format json
/usr/local/bin/typst eval '[*Hi* there]' --format json
/usr/local/bin/typst eval 'gradient.linear(red, blue)' --format json
/usr/local/bin/typst eval 'selector(heading.where(level: 1))' --format json
/usr/local/bin/typst eval 'calc.inf' --format json
```

Resultados centrais:

- `sys.version` → `"version(0, 15, 1)"\n`, exit 0;
- `none` → `null`; `auto` → `"auto"`; `12pt` → `"12pt"`;
- symbol `emoji.heart` → `"❤️"`; bytes de comprimento 2 → `"bytes(2)"`;
- label, decimal, duration, função, type, gradient e selector são strings de
  `repr()` público;
- content preserva morfologia como mapa recursivo `func + fields`;
- `calc.inf` → `null`, exit 0 no serializer JSON medido.

Classificação: o texto de `repr()` é linguagem pública para os valores de fallback; a
forma Rust/Serde que o produz é mecânica e não é exigida. Inferência: todo variant
cristalino público e construtível deve cair numa das classes observáveis acima, sem
fallback em `Debug`. Refutação: fonte vanilla pinada com braço nominal diferente ou
sonda direta que não produza a classe prevista.

### 3.3 Query não-heading

Fonte `query.rs:71-123`: o selector é avaliado como Typst, convertido para selector
locatável, a ordem do introspector é preservada e qualquer `Content` recuperado é
serializado pela regra genérica `func + fields`. `--field` filtra campos ausentes;
`--one` exige cardinalidade exatamente 1 antes do field mapping.

Sondas principais (input por stdin):

```sh
printf '%s\n' '#figure(rect(width: 10pt, height: 20pt, fill: red), caption: [Cap]) <fig>' | /usr/local/bin/typst query - figure --format json
printf '%s\n' '$x^2$' | /usr/local/bin/typst query - math.equation --format json
printf '%s\n' '#metadata((name: "x", n: 2))' | /usr/local/bin/typst query - metadata --format json
printf '%s\n' '#quote[First]' '#quote[Second]' | /usr/local/bin/typst query - quote --field body --format json
printf '%s\n' '#figure[one]' | /usr/local/bin/typst query - figure --field does-not-exist --format json
printf '%s\n' '#figure[one]' | /usr/local/bin/typst query - figure --field does-not-exist --one --format json
```

Resultados:

- figure contém `func:"figure"` e fields públicos recursivos, incluindo body,
  caption, kind, numbering, gap, outlined, counter e label;
- equation contém `func:"equation"`, block, numbering, number-align, supplement,
  alt e body;
- metadata contém `func:"metadata"` e o valor dict estrutural;
- duas quotes devolvem bodies `First`, `Second` nessa ordem;
- field ausente sem `--one` produz `[]`, exit 0; com `--one` produz
  `no such field found for element`, exit 1;
- `--one` sobre 0 ou 2 elementos falha com `expected exactly one element, found N`,
  exit 1;
- sucesso emite warning de deprecação em stderr; o valor serializado permanece em
  stdout.

Classificação: kinds, fields, ordem e cardinalidade são semântica/morfologia; JSON/YAML,
warning e exit são CLI pública. A presença de uma ramificação interna “heading-only” é
mecânica; o contrato a rejeita indiretamente pelas testemunhas não-heading.

### 3.4 Selectors textuais e regex

Fonte:

- `foundations/selector.rs:106-124`: string vira regex com escape; regex vazia ou
  capaz de match vazio é rejeitada;
- `foundations/selector.rs:131-153`: o matcher genérico de nós retorna `false` para
  regex;
- `foundations/selector.rs:421-436`: texto/regex não são locatáveis e query os
  rejeita;
- `foundations/selector.rs:508-525` e `typst-realize/src/lib.rs:1328-1368`: show
  aceita regex somente no topo e realiza matches textuais por caminho dedicado.

Sondas observáveis usam `metadata(it)` como testemunha sem depender de bytes de render:

```sh
printf '%s\n' '#show "foo": it => metadata(it)' 'foo bar foo' | /usr/local/bin/typst query - metadata --field value --format json
printf '%s\n' '#show regex("f.o"): it => metadata(it)' 'foo fxo bar' | /usr/local/bin/typst query - metadata --field value --format json
printf '%s\n' '#show ".": it => metadata(it)' 'a.b' | /usr/local/bin/typst query - metadata --field value --format json
printf '%s\n' '#show regex("."): it => metadata(it)' 'a.b' | /usr/local/bin/typst query - metadata --field value --format json
printf '%s\n' '#show "foobar": it => metadata(it)' 'foo#strong[bar]' | /usr/local/bin/typst query - metadata --field value --format json
/usr/local/bin/typst eval 'selector("")' --format json
/usr/local/bin/typst eval 'selector(heading.where(level: 1))' --format json
/usr/local/bin/typst eval 'selector(regex("f.o"))' --format json
/usr/local/bin/typst eval 'selector(regex(""))' --format json
/usr/local/bin/typst eval 'selector(regex("a*"))' --format json
printf '%s\n' 'foo' | /usr/local/bin/typst query - 'regex("foo")' --format json
```

Resultados:

- literal `foo` testemunha dois matches `foo`; regex `f.o` testemunha `foo`, `fxo`;
- literal `.` casa somente o ponto; regex `.` casa `a`, `.`, `b`;
- literal `foobar` não atravessa a fronteira `foo` + `strong[bar]` na sonda;
- texto vazio → `text selector is empty`, exit 1;
- selector composto preserva `heading.where(level: 1)` e regex válida produz selector
  com `repr` `regex("f.o")`, exit 0;
- regex vazia → `regex selector is empty`, exit 1;
- regex que casa vazio → `regex matches empty text`, exit 1;
- query de texto/regex → `text is not locatable`, exit 1.

Classificação: identidade do constructor, matches, fatias e o `it` entregue são
semântica/morfologia. A distribuição do vanilla entre matcher genérico e caminho
dedicado é mecânica e não deve ser copiada como argumento de paridade. Os L0 finais
cristalinos fixam adicionalmente que o helper local responda o mesmo predicado textual
sem alterar fase, viagem de nós ou número de aplicações; isto é invariante de
refinamento do desenho escolhido, não alegação de igualdade mecânica com o vanilla.

### 3.5 Morfologia matemática de grapheme e número

Fonte `typst-eval/src/math.rs:34-41`: `MathTextKind::Grapheme` empacota um
`SymbolElem`, enquanto `MathTextKind::Number` empacota um `TextElem`. A medição foi
repetida em 2026-08-30T15:30:14-03:00, no HEAD hospedeiro
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, com o binário e fonte vanilla pinados na
seção 2.3:

```sh
printf '%s\n' '$x^2$' | /usr/local/bin/typst query - math.equation --format json
```

O body observado é um attach cuja base é `{"func":"symbol","text":"x"}` e cujo
expoente `t` é `{"func":"text","text":"2"}`. Logo, a distinção não é apenas um
enum interno: ela altera a morfologia pública de conteúdo. Classificação: symbol versus
text e sua posição na árvore são língua; os nomes Rust e a forma de empacotamento são
mecânica. Refutação: fonte vanilla pinada ou sonda equivalente que apresente grapheme e
número sob a mesma função pública.

## 4. Contrato observável congelado

### C-YAML — eixo confirmado no gate ADR-0127

1. O help de `eval --format` anuncia exatamente `json`, `yaml`, `raw`; o default
   continua `json`. O help de `query --format` anuncia `json`, `yaml`; o default
   continua `json`.
2. `eval --format yaml` e `query --format yaml` produzem uma árvore YAML
   semanticamente equivalente à árvore JSON do mesmo valor/resultado, usando as
   classes de C-JSON e C-QUERY.
3. YAML acrescenta newline. `--pretty` não altera a árvore nem a forma YAML
   observada. Raw continua exclusivo de eval e de string/bytes.
4. Formato fora da enum falha no parser com exit 2; erros de avaliação ou
   serialização falham com exit 1.
5. `shell/cli.md` final foi atualizado, ressellado e confirmado explicitamente pelo
   dono em 2026-08-30; este requisito causal está satisfeito.
6. L4 encaminha o `QueryFormat` já decidido em L2 sem o reinterpretar nem forçar JSON;
   o serializer e a enumeração dos formatos permanecem em L2.

Adicionar `Yaml` às enums/intents públicos ou anunciá-lo no help é mudança de contrato
e comportamento público. A paragem ADR-0127 foi cumprida para o eixo YAML inteiro, não
só para o serializer.

### C-JSON — correção contínua de paridade

1. Classes estruturais: none→null; bool/int/float/string→scalar correspondente;
   symbol→grapheme; bytes→representação humana medida; array/dict recursivos;
   content→mapa `func + fields` recursivo.
2. Demais valores públicos construtíveis serializam como string contendo o `repr()`
   público. Sentinelas obrigatórios: Version, length, color, label, datetime, decimal,
   duration, function, type, gradient, tiling, regex e selector.
   Canónicos já medidos: Func `calc.gcd` → `gcd`; gradient →
   `gradient.linear((oklab(65.95%, 0.2, 0.108), 0%), (oklab(56.22%, -0.05, -0.17), 100%))`;
   tiling → `tiling((10pt, 10pt), ..)`.
3. `sys.version` serializa exatamente como a string de linguagem
   `version(0, 15, 1)`, não como display `0.15.1`, Debug Rust ou erro.
4. Não se autoriza um fallback em `repr` para as classes estruturais; arrays, dicts e
   content devem preservar a árvore.
5. Raw não é ampliado por este contrato. Version e qualquer tipo não string/bytes
   continuam a falhar com exit 1 e hint adequado.

Isto é correção de paridade em output existente. Os L0 finais também autorizam, no gate
confirmado, a fachada pública total
`repr_value_for_serialization(&Value) -> String` em `compiler/eval/mod.rs`, delegada ao
owner privado `compiler/eval/repr.rs`. Qualquer outra assinatura, método ou variant
público não descrito pelos nove L0 reabre ADR-0127.

### C-QUERY — correção contínua de paridade

1. Todo kind locatável suportado retorna `Content` em ordem documental; não há garantia
   pública limitada a heading.
2. A forma serializada de qualquer resultado é genérica: `func` público seguido de
   todos os fields públicos disponíveis, recursivamente sob C-JSON/C-YAML.
3. A matriz mínima inclui heading como controlo e figure, equation, metadata e quote
   como testemunhas não-heading. Um tipo ainda não materializado pode ser `Unknown`,
   mas não pode transformar uma testemunha obrigatória suportada em sucesso.
4. `--field` preserva ordem, filtra fields ausentes no modo lista e falha com
   `no such field found for element` no modo `--one`.
5. `--one` exige cardinalidade exatamente 1 antes do field mapping; 0 e N>1 falham
   com exit 1 e contagem observada.
6. Warning de deprecação pertence a stderr; o valor pertence a stdout. Diagnósticos
   não podem contaminar JSON/YAML parseável em stdout.
7. Quando L3 transporta uma label explícita como `Content::Label`, L2 serializa o
   elemento interno por `func + fields` e acrescenta `label: "<nome>"`. O wrapper não
   aparece como `func: label`, não cria match extra e não muda a cardinalidade.
8. O input posicional `-` lê stdin como UTF-8, cria em L4 um `SystemWorld::for_eval` e
   uma `Source` markup transitória com `world.main()`, e entrega a mesma combinação
   `World + Source` usada pelo fluxo físico a `query_elements`. Não é interpretado como
   nome de ficheiro literal. O resultado semântico deve coincidir com o de uma fonte
   física com os mesmos bytes.

Remover um braço especial de headings é não-objetivo mecânico: qualquer estrutura é
aceite se satisfizer as testemunhas de língua.

### C-SELECTOR — correção contínua de paridade

1. Selector de string é literal: metacaracteres são escapados. Selector regex usa a
   linguagem regex. Ambos substituem cada match não-vazio e entregam ao recipe somente
   a fatia morfológica casada.
2. Casos positivos mínimos: literal repetido, literal parcial dentro de um nó, regex
   com match e múltiplos matches. Casos negativos: não-match e fronteira de nós medida.
3. Literal `.` casa apenas `.`; regex `.` casa cada scalar observado. Essa dupla é o
   controlo discriminatório contra confundir literal e regex.
4. Texto/regex vazios e regex que casa vazio falham; não podem gerar loops ou sucesso
   silencioso.
5. Query/locate continuam a rejeitar texto/regex como não locatáveis. Generalizar show
   não generaliza introspecção.
6. O caminho de show já existente permanece verde. Não se exige copiar o matcher
   genérico ou o algoritmo de leftmost/revocation do vanilla.
7. `selector(S)` preserva por identidade qualquer `Value::Selector`, incluindo selector
   composto, e conserva sua `repr`. `selector(R)` converte uma `Value::Regex` válida e
   não vazia em `Value::Selector(Regex(R))`; kind, label e função mantêm as conversões
   já existentes.
8. Os três erros do constructor são distintos e observáveis: string vazia →
   `text selector is empty`; regex vazia → `regex selector is empty`; regex não vazia
   que casa texto vazio → `regex matches empty text`.
9. Show regex aplica `Func`, `Content` ou `Str` **por ocorrência**, entregando à recipe
   somente a fatia morfológica casada. Prefixo/sufixo não casados, ordem, fronteira de
   nós, precedência e revogação existentes são preservados.
10. O matcher puro local casa Text/Regex somente contra o `Content::Text` recebido e
    somente se houver ocorrência não vazia; não desce em `Sequence`, não concatena nós
    e não torna `is_node_rule(Text|Regex)` verdadeiro.

Este eixo permanece fluxo contínuo somente enquanto puder ser materializado pela forma
pública já existente. Adicionar variant, campo ou assinatura pública reclassifica a
mudança e exige gate ADR-0127.

### C-MATH — correção contínua de morfologia

1. `MathTextKind::Grapheme` produz `Content::MathIdent`; na serialização pública ele
   preserva morfologia de symbol/ident, não text genérico.
2. `MathTextKind::Number` produz `Content::MathText`; números matemáticos preservam a
   morfologia textual observada.
3. Testemunhas mínimas: `$x$` distingue ident/symbol, `$2$` distingue text e `$x^2$`
   preserva simultaneamente base symbol/ident e expoente text.
4. Shorthands matemáticos, símbolos resolvidos pelo scope e identificadores desconhecidos
   mantêm os caminhos já existentes; esta obrigação não os converte indiscriminadamente
   em `MathIdent` ou `MathText`.
5. A correção não autoriza novo tipo, campo, assinatura pública ou mudança de fase.
   `MathTextKind` e os variants internos são mecanismo; o observável exigido é a árvore
   morfológica de conteúdo.

## 5. Ownership 1:1 e fronteiras de edição futura

| Obrigação P1285 | Prompt proprietário único | Consumer produtivo único | Situação |
|---|---|---|---|
| CLI, enums de formato, JSON/YAML e apresentação de query | `shell/cli.md` | `02_shell/src/cli.rs` | final, ressellado e confirmado |
| fachada pública total de repr para L2 | `compiler/eval.md` | `01_core/src/compiler/eval/mod.rs` | final, ressellado e confirmado |
| implementação privada da repr morfológica | `compiler/eval/repr.md` | `01_core/src/compiler/eval/repr.rs` | final e ressellado; permanece privado |
| predicado textual/regex local e unidades puras de splice | `compiler/eval/selector_matching.md` | `01_core/src/compiler/eval/selector_matching.rs` | final repinado após a candidata v1 |
| show literal/regex por ocorrência, precedência e revogação | `compiler/eval/rules.md` | `01_core/src/compiler/eval/rules.rs` | owner adicional final e ressellado |
| constructor público `selector` por identidade/regex/vazios | `compiler/stdlib/foundations/selector.md` | `01_core/src/compiler/stdlib/foundations/selector.rs` | owner adicional final e ressellado |
| morfologia grapheme→MathIdent e number→MathText | `compiler/eval/math.md` | `01_core/src/compiler/eval/math.rs` | nono owner final e ressellado |
| dados de show existentes | `entities/show.md` | `01_core/src/entities/show.rs` | não adicionar variant/campo público em fluxo contínuo |
| `query()` da linguagem | `compiler/stdlib/foundations/query.md` | `01_core/src/compiler/stdlib/foundations/query.rs` | só editar se a obrigação residir realmente aqui |
| recuperação integrada, não-heading e transporte de label | `infra/query-helpers.md` | `03_infra/src/query_helpers.rs` | final e ressellado após restart causal |
| composição de query, transporte de `QueryFormat` e stdin transitório | `wiring.md` | `04_wiring/src/main.rs` | final e ressellado; lineage declarado `bbc0ca9b` |
| valor, Version e selector públicos | `entities/value.md`, `entities/version.md`, `entities/selector.md` | ver headers dos prompts | dependências somente; não são autorização P1285 |

Não se atribui uma mesma obrigação a dois consumers. Se uma claim normativa idêntica
precisar realmente ser compartilhada por dois ou mais prompts, ela deve ser extraída
para Núcleo Tekt pinado, sem transformar o Núcleo em owner.

Há uma irregularidade pré-existente a não ampliar: `entities/version.md` declara como
alvos `entities/version.rs` **e** `entities/value.rs`, enquanto `entities/value.md` já
declara ownership de `entities/value.rs`. P1285 não deve editar ou ressellar essa relação
como se fosse 1:1. Se uma edição nesses consumers se tornar necessária, V15/ADR-0129
exigem individualizar o owner antes do resselo.

## 6. Política de `Unknown`

Resultado por caso:

- `Preserved`: comando suportado, exit esperado, stdout parseável quando aplicável e
  árvore/valor/diagnóstico satisfaz o contrato;
- `Violated`: divergência observável com testemunha reproduzível;
- `Unknown`: o observável não pôde ser determinado sem capacidade fora da allowlist,
  o tipo é opaco/não construtível no harness, o parser/harness não preserva os bytes, ou
  o budget termina antes da medição.

`Unknown` nunca conta como sucesso, nunca é convertido implicitamente em `Preserved` e
impede selo se atingir um caso obrigatório. Casos opacos deliberados para o futuro gate:

- valor dinâmico/Styles/Args sem construção pública isolada;
- raw com bytes arbitrários quando o capturador fizer decode lossy — deve usar
  comparador byte-safe ou devolver `Unknown`;
- selector cujo agrupamento atravesse cadeias de estilo/contexto não cobertas pelas
  fronteiras medidas;
- target HTML/bundle e kinds não materializados, que estão fora deste fragmento.

Ausência de L0 final ou confirmação YAML seria `Blocked`, não `Unknown`; essa condição
foi resolvida em 2026-08-30. `Unknown` continua proibido como sucesso nos gates
subsequentes.

## 7. Matriz de mutações e poder discriminatório requerido

| ID | Mutação semanticamente negativa | Testemunha/controlo que deve rejeitá-la | Esperado |
|---|---|---|---|
| M-Y1 | omitir YAML do help/enum eval | inventário `eval --help` | `Violated` |
| M-Y2 | emitir JSON sob `--format yaml` | composto com array, Version e content | `Violated` |
| M-Y3 | `--pretty` mudar YAML | par com/sem `--pretty` | `Violated` |
| M-Y4 | aceitar raw em query | `query --format raw` deve exit 2 | `Violated` |
| M-Y5 | L4 descartar `QueryFormat` e forçar JSON | query YAML pelo fluxo completo | `Violated` |
| M-J1 | rejeitar Version | `eval sys.version --format json` | `Violated` |
| M-J2 | serializar Version como `0.15.1`/Debug | string exata `version(0, 15, 1)` | `Violated` |
| M-J3 | serializar array/dict/content por repr | composto e content forte | `Violated` |
| M-J4 | fallback desconhecido em Debug/erro | gradient/selector/regex públicos | `Violated` |
| M-J5 | ampliar raw a Version | raw Version deve exit 1 | `Violated` |
| M-Q1 | serializer heading-only | figure/equation/metadata/quote | `Violated` |
| M-Q2 | omitir `func` ou fields públicos | shape de figure/equation | `Violated` |
| M-Q3 | reordenar/deduplicar resultados | duas quotes `First`, `Second` | `Violated` |
| M-Q4 | field ausente sempre errar ou sempre filtrar | par lista vs `--one` | `Violated` |
| M-Q5 | validar `--one` depois do filtro | one + field ausente; 0/2 elementos | `Violated` |
| M-Q6 | expor wrapper como `func: label`, perder ou duplicar label | metadata/figure rotulada | `Violated` |
| M-Q7 | tratar input `-` como path físico | mesma fonte por stdin e ficheiro | `Violated` |
| M-Q8 | criar Source code ou com identidade divergente para stdin | query contextual e diagnóstico resolvível por stdin | `Violated` |
| M-S1 | tratar literal como regex | literal `.` vs regex `.` | `Violated` |
| M-S2 | aceitar somente o primeiro match | `foo bar foo`; `foo fxo` | `Violated` |
| M-S3 | transformar não-match | regex `z+` sobre `foo fxo` | `Violated` |
| M-S4 | aceitar selector vazio/match vazio | `""`, `regex("")`, `regex("a*")` | `Violated` |
| M-S5 | tornar texto/regex locatável em query | query textual/regex | `Violated` |
| M-S6 | regredir show literal existente | literal parcial `xfooY` | `Violated` |
| M-S7 | rejeitar ou reconstruir `Value::Selector` recebido | `selector(heading.where(level: 1))` preserva repr | `Violated` |
| M-S8 | rejeitar regex válida ou devolver `Value::Regex` cru | `selector(regex("f.o"))` produz selector | `Violated` |
| M-S9 | conflar os três diagnósticos de vazio | string vazia, regex vazia e regex `a*` | `Violated` |
| M-S10 | chamar recipe regex uma vez com o nó inteiro | `f.o` sobre `foo fxo`; `.` sobre `a.b` | `Violated` |
| M-M1 | colapsar Grapheme e Number em `Content::MathText` | `$x$` e base de `$x^2$` devem ser symbol/ident, não text | `Violated` |
| M-M2 | converter Number em `Content::MathIdent` | `$2$` e expoente de `$x^2$` devem ser text | `Violated` |

O gate discriminatório futuro deve executar mutantes reais ou equivalentes sob os
oráculos independentes e obter `mutation_score = 1.0` sobre todas as mutações válidas.
Este autor não executou nem validou mutantes; portanto não existe score nem selo agora.

## 8. Controles de repetição, reordenação e streams

O verificador independente deve:

1. correr cada sonda obrigatória pelo menos duas vezes contra o mesmo binário/hash e
   exigir o mesmo verdict;
2. reordenar a execução dos casos e exigir verdicts idênticos por ID;
3. capturar stdout e stderr separadamente e preservar exit code;
4. comparar JSON/YAML como árvores tipadas; comparar raw como bytes; comparar mensagens
   por conteúdo semântico, exceto os fragmentos explicitamente fixados;
5. preservar ordem de arrays de query e de matches; não transformar ordem de mapa JSON
   em requisito de língua;
6. fixar `NO_COLOR`/formato diagnóstico ou remover somente ANSI, sem normalizar texto
   substantivo;
7. recalcular hashes de prompt, contrato, baseline e suites antes e depois do gate.
8. executar as sondas obrigatórias de query tanto por ficheiro físico quanto por stdin
   `-`, exigindo a mesma árvore semântica e preservando diferenças legítimas de origem
   nos diagnósticos.

## 9. Gate ADR-0127 satisfeito e estado causal

O eixo YAML e a fachada pública de repr mudam contrato público. A sequência foi
cumprida:

1. a decisão final foi incorporada nos nove owners L0 1:1 aplicáveis;
2. os hashes foram ressellados via `crystalline-lint`, conforme comunicação do
   coordenador;
3. o dono confirmou explicitamente o gate P1285 em 2026-08-30;
4. a descoberta pré-código do wrapper de label invalidou o primeiro pin, causou
   restart e novo resselo de `shell/cli.md` e `infra/query-helpers.md`;
5. este receipt repinou inicialmente cinco L0 somente após a estabilização causal de
   label;
6. a descoberta posterior do owner L4 invalidou aquele receipt, e este v2 repinou os
   seis L0 somente após `wiring.md` final e ressellado;
7. o resultado parcial `26/42` da candidata v1 revelou as obrigações de constructor e
   show regex por ocorrência antes da revisão candidata v2; v2 e a candidata v1 foram
   invalidados como evidência final;
8. este v3 repinou oito L0 após adicionar `eval/rules.md`,
   `stdlib/foundations/selector.md` e atualizar `eval/selector_matching.md`.
9. a descoberta final de morfologia matemática invalidou v3 e as candidatas v1/v2
   como evidência final; este v4 adicionou `eval/math.md` e repinou os nove owners;
10. o resselo simultâneo de três consumers mudou somente suas linhas
    `Hash do Código`; a auditoria da seção 2.2 preservou o corpo normativo v3 e adotou
    o hash sem selo como identidade semântica canónica.

O contrato está congelado para implementação e construção independente de oráculos. Uma
alteração posterior em qualquer um dos nove hashes sem selo, no baseline ou neste
fragmento observável invalida este congelamento e reinicia a cadeia na primeira fase
afetada. Mudança apenas no full hash causada exclusivamente pela linha `Hash do Código`
segue a política seal-only da seção 2.2 e não reinicia a semântica.

Estado final deste receipt: `CONTRACT_FROZEN_POST_L0_GATE`. A confirmação/L0 autorizam a
materialização, mas este autor não leu nem julgou implementação. Ainda não existem neste
receipt mutation score, selo discriminatório, certificado ou veredito de refinamento.
