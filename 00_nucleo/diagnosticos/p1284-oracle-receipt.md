# A-P1284-v5 — recibo independente de oráculos e ataques

Estado: **SEALED_ORACLES_FOR_IMPLEMENTATION**  
Contrato: **C-P1284-v7**  
SHA-256 do contrato: `ea85130ae2954d0fb930e68ce82b5554780798ef5927bcab0cc7f109ab9c61f6`  
Suíte declarativa: `lab/surface-inventory/p1284-probes.json`  
SHA-256 da suíte: `e8c6a0a2da6dbb784e29ba65e7cfa41b5eb17877e1f1f7fe5760e37d09f89d8f`  
Autor: papel segregado `independent_oracle_author`  
Momento do resselo: `2026-08-30T12:41:54-03:00`  
HEAD observado sem leitura de diff: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`

## 1. Declaração de independência

O selo `A-P1284-v1` foi invalidado: a primeira execução independente contra o
vanilla obteve apenas `24/36`. Este `A-P1284-v2` corrige as testemunhas do
oráculo e só foi resselado depois de obter `52/52` no vanilla ratificado.
Depois, a correção normativa de `rocket`/`mako` alterou o contrato protegido e
o L0 de cor, invalidando causalmente o pin v2. Este `A-P1284-v3` referencia
`C-P1284-v4`, confirmou `22/22` pins L0 e repetiu a mesma suíte sem mudança
semântica, novamente com `52/52` no vanilla ratificado.
Por fim, C-P1284-v5 acrescentou duas pré-condições L0 antes de qualquer leitura
de candidato: chamabilidade de `Type::Arguments` e quatro Ratio posicionais em
`cmyk`. Isso invalidou o pin v3. Uma omissão adicional nos casts heterogêneos
de `hsl`, `hsv`, `oklab` e `oklch` suspendeu o primeiro resselo A-v4 contra
C-v5. Este `A-P1284-v4` referencia agora C-v6, confirmou `23/23` L0 e repetiu
sem expansão as mesmas 52 sondas, que já exercitam `arguments(1, x: 2)`,
`color.cmyk(0%, 0%, 0%, 100%)`, `hsl(0deg, 100%, 50%)`,
`hsv(0deg, 100%, 100%)`, `oklab(50%, 0, 0)` e
`oklch(50%, 0, 0deg)`.

Depois da implementação, sete L0 receberam resselo mecânico exclusivamente na
linha `Hash do Código`. C-P1284-v7 prova a equivalência normativa byte a byte
ao reconstruir os SHA C-v6. Isso invalida apenas os pins do selo A-v4. Este
`A-P1284-v5` referencia C-v7, confirma os `23/23` hashes finais e conserva a
suíte e a matriz sem mudança semântica. O hash canônico da projeção semântica
do JSON, excluindo apenas selo/id/contrato/registro de execução, permaneceu
`4159f8e5b0f683b85eac992f0a99c04e9241d14294721780927fbd4790f3c16c`
antes e depois do repin.

O corpo semântico deste recibo e a suíte foram escritos antes de qualquer
leitura de código, testes, patches ou diffs candidatos P1284. O repin A-v5
ocorre depois de existir implementação, mas alterou somente identidade do
oráculo/contrato, sete hashes L0 e proveniência de execução; nenhuma fonte,
expectativa, mapa, controle, classificador ou matriz foi adaptado. Nenhum
código produtivo foi editado. As únicas saídas desta autoria nesta fase são o
JSON acima e este diagnóstico.

> **Invalidação e resselo:** o ensaio intermediário contra C-v5 foi invalidado
> antes de poder autorizar implementação. C-P1284-v6 fechou a omissão no owner
> `foundations/color`; C-P1284-v7 repinou sete headers sem mudar norma. A suíte
> integral foi reexecutada no vanilla após o último repin. Somente o presente
> pin C-v7 constitui o selo causal.

O ambiente não ofereceu uma sandbox forte que impedisse tecnicamente leituras
fora do conjunto autorizado. A independência aqui é, portanto, uma disciplina
de processo auditável, não uma alegação de isolamento físico: foram lidos
somente `AGENTS.md`, o skill de materialização segregada, o contrato selado, os
23 L0 que ele pina e as fontes/binário vanilla ratificados. Não foi executado
`git diff`, não foram abertos testes candidatos e não foi inspecionado código
produtivo candidato P1284.

O contexto conversacional compartilhado já contém resultados de uma fase
adversarial anterior. Assim, este repin não alega novo isolamento forte do
executor; sua independência causal vem do fato verificável de que a projeção
semântica da suíte permaneceu byte-identical e de que C-v7 prova mudança
normativa nula. O recibo adversarial não foi lido nem atualizado nesta fase.

## 2. Proveniência reproduzível

O contrato foi recalculado localmente e coincidiu integralmente com o pin
fornecido. O binário `/usr/local/bin/typst` tem SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` e
reporta `typst 0.15.1 (e0e8ca4d)`. Essa string não substitui a proveniência:
o alvo normativo permanece upstream `a51e02804` mais os hashes de fonte abaixo.

### L0 finais pinados

| L0 | SHA-256 |
|---|---|
| `compiler/stdlib/collections.md` | `f474e4b654509de4c5dda75223499463388d2e683be36b4eff11dbaf732839e5` |
| `compiler/eval/bindings/method_dispatch.md` | `fb6ca8f6482d08ad0dde8d900bdb882a66f7abafebb2761d3a4b226d6589e951` |
| `compiler/eval/bindings/field_access.md` | `97122927d00c44fac5039b1ae45fc1705e6f6a4743fa5d31e8a7eedbe7e2e649` |
| `compiler/eval/call_dispatch.md` | `9dfe195c585fa95acf71d0c2dec41541bf16b72dac4033c55a83159f8b68809f` |
| `compiler/eval/bindings/value_methods.md` | `85d31916ee6091f9853e2605760c3ecc41d4be5c5a702f1fa7e4b3bacc1b6091` |
| `compiler/stdlib/foundations/str.md` | `4b3fa3d5cb2c313fa13326dc8c398f70ef89efb229a6a8ae8b37c1e77bca0043` |
| `compiler/stdlib/foundations/color.md` | `453059c00c2acefcb7c8610523e986a98edd1ed7a0a1b36fb24dcd357fa5b048` |
| `compiler/stdlib/color.md` | `7149cf1a93c96dab1c4d31aa491fd2093bfccd06459681c6f9f04ee1eff2677c` |
| `entities/color.md` | `41d6aaa1e696de39823ca63fe179351125a8573467521c1a0e6d795afdb4562a` |
| `entities/dir.md` | `b9cd0f95837004169ec06f16ab6c9fb7b138f1407969e3da42e3587d72ae4aba` |
| `entities/layout_types.md` | `68f453b4abfd333f17f6f989ac83e11846926b9b91b16e70ab455bb17f603779` |
| `entities/args.md` | `3a33f1f2628348e3484b8346ab55aa82474b789e6836c62303b24b43297f5d56` |
| `entities/duration.md` | `b16ff16253af6ad0cd119679880a429f401bd3815cebb7077f70179bc1c6ae2e` |
| `compiler/stdlib/primitives-constructors/duration.md` | `c7112b9eddb05d6cd885da3a4e269709ae2467f74a7ca7f6144f3b3531f88454` |
| `entities/selector.md` | `8dcd7bfbc6789ed0ef669d8ffd943df63aa06343f0c4ef39f23e797ec276b429` |
| `compiler/stdlib/foundations/selector.md` | `4e9d874ee11ba8f89aca7999881558d1981c677a6eceeee4afa03da3ea62e46d` |
| `entities/state.md` | `466e16e018de4bd1cc043e1cf79b92bf5c4b375b178a65ec85228e373bdecec4` |
| `compiler/stdlib/state.md` | `daf180d5e77f0a9d307b8b1a2b148741a6ac82a776ab17a510f107c5bff42eb1` |
| `entities/location.md` | `4628ee19327ebe7eb60b13fe010c526e378b11efc76728b878de643d0a188883` |
| `entities/elements/outline.md` | `58ea2a650fdedd81fa267c902f5387ea1a4cfe42e381b9afa13497c486b19fc4` |
| `compiler/stdlib/structural/outline.md` | `e8dc3b96fd9a03ae21f87f602d85bb7b0b5b6eb95d6267dcbfe280a80d3b0632` |
| `compiler/stdlib/pdf.md` | `5d32e0013e91b42943b99276f2f334000ed85c60d71147b950c1b1bd645b9f4b` |
| `entities/bytes.md` | `bd6199460444e5c82754a5d7050733bd0a5c36bf58d907855287fd1b6b539a7f` |

### Fontes vanilla ratificadas

| Fonte | SHA-256 |
|---|---|
| `foundations/array.rs` | `f1a920156cd2eb93f1da9a914307873b8895698c7c25ce9e2a678bae1bf5d35d` |
| `foundations/bytes.rs` | `9307030e7ea1a313752ba568721a3a1eb98228234a5803a77cf29433d9c86929` |
| `foundations/args.rs` | `b681b149809326f2479b99966232680771f8d95a170c82180d30cbd22274849b` |
| `foundations/str.rs` | `8648ad25be763830470664c9dc022f096902a496e4b553a7d75259b25334cdb7` |
| `foundations/dict.rs` | `43b5a6bc1c01ab35b350143ba6444d3a695d60aa628643de42d3ab65e62f9f4b` |
| `foundations/duration.rs` | `42504ed71e3a95104b4b19330d99cb9df9b19268c4ed4c63f489a4b280648ee0` |
| `foundations/selector.rs` | `84615405b19d4869d261029936d3d5d9077f06e7198e3f60f28273c1ecdfcbdc` |
| `introspection/state.rs` | `ea7e29c21185501d4805674734e3df79da9adc921f9bde010812642af7da767f` |
| `introspection/location.rs` | `76ddfc851ddd9a76af3a5b917b609047fc78ccb2fb2a0c378c8f4ee54d828390` |
| `visualize/color.rs` | `80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713` |
| `layout/dir.rs` | `2887408cd29596e8f4bc49a17c53e2e146a11fb5138323aa51e4e344d1f450f4` |
| `layout/align.rs` | `e0ad22c93e775452c10d55c1cc7e360c6bd2bfdcbf5afd66c5f4e9b4d2042993` |
| `layout/length.rs` | `7c74ad8d77ab31cfc646518263cdc52d1901d940c4ca7f074bafc33c1dd161de` |
| `model/outline.rs` | `74c22ad7ef735cfe96e8cd89bc6de342e4550c2847894269c040990d5a8cfd48` |
| `pdf/accessibility.rs` | `dba1d5f5e4fe3e8c24376cd96616be0f183926fa15b6303ee75c3806e8567d51` |
| `pdf/attach.rs` | `525d2505b229baa9ae1a6078de290bddaeba23a4378304c695aa36c55f63aaed` |

Evidência de metadata vem das scopes e atributos da fonte ratificada, entre
outros: `array.rs:150-1160`, `bytes.rs:228-294`, `args.rs:320-443`,
`str.rs:129-700`, `dict.rs:173-350`, `duration.rs:36-130`,
`selector.rs:158-292`, `state.rs:215-340`, `location.rs:77-125`,
`dir.rs:46-170`, `align.rs:176-220`, `length.rs:96-165`,
`color.rs:273-340` e `2695-2743`, `outline.rs:246` e `475-690`,
`pdf/accessibility.rs:35-100`, `pdf/attach.rs:20-70`.

## 3. Forma do oráculo

O JSON é uma suíte declarativa executável por `typst query`: cada sonda contém
fonte Typst real, selector `<p1284>`, valor esperado ou erro normalizado. Não é
um inventário de nomes apenas. Há chamadas ligadas e não ligadas, aliases,
constantes, funções estáticas, módulos, defaults, mutação, variádicos, erros e
contexto presente/ausente.

A política de metadata observa exatamente: nome, `self`, ordem, positional,
named, required, variadic, settable e default. Para cada método autorizado, o
catálogo congelado exige as duas projeções; a relação é avaliada como um todo.
Um nome que existe mas não chama corretamente é `Violated`.

Cobertura congelada:

- 169 paths autorizados, todos enumerados uma vez no catálogo;
- seis sentinelas nominais, mais o roundtrip U+0000 obrigatório;
- chamadas ligadas/não ligadas por família e 12 controles discriminatórios de
  falta, excesso, named desconhecido, tipo, defaults, variádicos e contexto;
  11 ficam na coleção `argument_and_error_probes` e o décimo segundo é o
  `unbound_projection_control` executável aninhado em `array-mutators`;
- 15/15 filhos de `color.map`, cada qual com kind, cardinalidade, extremos e
  digest canônico integral;
- 11 paths bloqueados ou dependentes de ancestral bloqueado, fora do score.

## 4. Gate vanilla executado

Foram recalculados primeiro o contrato (`ea85130a…c61f6`) e os 23 L0 finais
congelados (`23/23`, inclusive `call_dispatch.md 9dfe195c…809f` e
`foundations/color.md 453059c0…b048`). Depois foram
executadas todas as sondas de sucesso/erro e as identidades dos 15 mapas,
alimentando a fonte por stdin para não criar artefatos auxiliares:

```text
/usr/local/bin/typst query - '<p1284>' --field value --one
/usr/local/bin/typst compile - - --format pdf
resultado de sucesso/erro: 37/37
resultado color.map:       15/15
resultado total:           52/52
```

A aritmética integral é `6 + 1 + 18 + 11 + 1 + 15 = 52`: seis
`six_sentinels`, um `required_extra_sentinel`, 18 `real_call_probes`, 11
`argument_and_error_probes`, o controle executável não ligado aninhado em
`array-mutators`, e 15 identidades de mapa. O controle aninhado é executado
como caso independente; por isso o subtotal anterior aos mapas é 37, não 36.

As comparações cobriram JSON exato, exit status, needles de diagnóstico e
digest canônico. O controle não ligado `array.push(x, 3)` é erro esperado
`cannot mutate a constant`, não sucesso fabricado. `pdf.attach` é sucesso no
baseline vanilla e erro de scope-out apenas no candidato cristalino, conforme
divergência de export já autorizada no L0.

As correções v1→v2 foram estritamente do oráculo: `array.join` passou a usar
strings; mutadores não ligados deixaram de ser contados como sucesso;
callbacks de `arguments`/`dictionary` recebem só o valor; índices UTF-8 de
`str` foram corrigidos; `color.mix` usa variádicos; o vetor de `direction`
passou a oito resultados; metadata contextual foi colocada em markup dentro de
`context`; needles de contexto e a expectativa bilateral de `pdf.attach`
foram corrigidos. A validação integral também revelou os zeros iniciais
omitidos em v1 para `rocket` e `mako`; suas identidades abaixo são as sequências
integrais realmente expostas pelo vanilla ratificado.

## 5. Sentinelas determinísticas

| ID | Testemunha | Oráculo |
|---|---|---|
| S1 | `array.len((1,2,3))` e `(1,2,3).len()` | `function`, ambos `3` |
| S2 | `bytes.len(bytes("é"))` e forma ligada | `function`, ambos `2` bytes |
| S3 | `arguments.len(arguments(1,x:2))` | `2`, sendo um positional e um named |
| S4 | `alignment.left` | kind `alignment`, repr `left`, alias de `left` |
| S5 | `direction.rtl` | kind `direction`, repr `rtl`, alias de `rtl` |
| S6 | `color.map` e `.viridis` | `module`; filho `array`, 9 itens, `#440154`…`#fee825` |
| S7 | `str.from-unicode(0)` e `str.to-unicode(...)` | string de um scalar U+0000; volta a `0` |

## 6. Identidade integral dos 15 mapas

A sequência canônica é formada por tokens lowercase `0xrrggbbaa`, separados
por vírgula sem whitespace, e então SHA-256. O JSON contém uma sonda-paramétrica
que recolhe todos os valores, não apenas os extremos.

| Mapa | N | Primeiro | Último | SHA-256 canônico |
|---|---:|---|---|---|
| turbo | 256 | `0x23171bff` | `0x900c00ff` | `7909421397bc2bef00faf5a1064d7356ad4cab112ec1fd5e290b124a13529a3e` |
| cividis | 256 | `0x002051ff` | `0xfdea45ff` | `dece34103d5266311558bb44f96f02df7090edf8977744fc9a1213b2213b2f79` |
| rainbow | 256 | `0x7c4bbbff` | `0x7c4bbbff` | `74fe385a692d3da43ff8afc8f61896f76bc87a1a766f05f4897f5e8164db4ed7` |
| spectral | 11 | `0x9e0142ff` | `0x5e4fa2ff` | `1c62ea2e765ddf6d6b1c5189e772605e0203d6116bf384597df0cfa4cdab5f17` |
| viridis | 9 | `0x440154ff` | `0xfee825ff` | `3b9d02b0685ec2ae62958a287c4cdd68fdb08f642339aabaa8d96e60aeaf999b` |
| inferno | 11 | `0x000004ff` | `0xfcffa4ff` | `20de1b5440e58c3727d645e6f8444d9e6d1c73fb09f9e905a49c314dc58496da` |
| magma | 11 | `0x000004ff` | `0xfcfdbfff` | `c634397325d83383609535f10e2d964efd899be34a2cef337379a5368b09fa79` |
| plasma | 11 | `0x0d0887ff` | `0xf0f921ff` | `f0ddd8a6d9e12c3c3b5d705dfa2e7c5f7929ce242c8d94ab5e4a0f6b2e0d9f2d` |
| rocket | 256 | `0x03051aff` | `0xfaebddff` | `b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b` |
| mako | 256 | `0x0b0405ff` | `0xdef5e5ff` | `f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77` |
| coolwarm | 256 | `0x3b4cc0ff` | `0xb40426ff` | `13161dddf6ad860268eece02ce446b1025d3ab95871cc1a484a93aa05da4aed2` |
| vlag | 256 | `0x2369bdff` | `0xa9373bff` | `d015c8a4cb87aaebc49d0416edf430e5c76392e772602ca1234ce6fe748db81d` |
| icefire | 256 | `0xbde7dbff` | `0xffd4acff` | `0de0d1c2a9dd6a8a03cd3155b872ea4a2eeaa74a5454df8f511632aa5d91e55f` |
| flare | 256 | `0xedb081ff` | `0x4b2362ff` | `03f70bedc78f4fb3e22b3f7ed06a680d56d721321eca69218ab746d4c803f595` |
| crest | 256 | `0xa5cd90ff` | `0x2c3172ff` | `660b6381c0f53156c07b4f0f86988a280c06578f46a44b734c1f388cadeaeb98` |

## 7. Ataques discriminatórios

O oráculo rejeita deterministicamente as mutações do contrato por estas
testemunhas: omissão de projeção não ligada; método exposto como constante;
constante exposta como função; função estática convertida em método; `self`
removido/deslocado; positional/named trocado; required tornado optional;
default alterado; variadic removido ou alargado; named desconhecido aceito;
ligada e não ligada com semânticas diferentes; chamada real substituída por
presença; alias qualificado fabricado; módulo/filho com kind errado; filho de
`color.map` removido; ordem, cardinalidade, extremo ou valor interno de mapa
alterado; digest calculado com normalização diferente; `arguments.len` contando
só positional; U+0000 rejeitado; callback/erro/default incorreto; método
contextual aceito fora de contexto ou recusado dentro; erro `pdf.attach`
promovido a embedding não autorizado; e qualquer bloqueado apresentado como
sucesso.

As duas mutações v5 também são rejeitadas sem adicionar sondas: S3 falha se
`arguments` não for chamável ou perder/reordenar named, e a sonda de cor falha
se `color.cmyk` rejeitar Ratio, permutar CMYK ou divergir do binding global.
As mutações v6 sobre casts, unidades, ordem, alpha default ou aliases dos
quatro constructors heterogêneos são rejeitadas pela mesma sonda de cor: ela
executa as formas válidas exatas de HSL, HSV, Oklab e Oklch tanto no recorte
qualificado quanto nas relações globais já congeladas.

C-P1284-v7 não acrescenta mutação nem observável: os sete novos hashes são
resselos `Hash do Código`-only comprovados pelo contrato. A-P1284-v5 preserva,
portanto, exatamente a matriz 35+10 e as mesmas testemunhas de A-v4.

## 8. Controles bloqueados

| Path | Classificação exigida | Oráculo |
|---|---|---|
| `color.spot` | `Blocked(BLOCKED_ADR0127_PUBLIC_CONTRACT)` | ausência é controle; presença/stub é `Violated` |
| `color.spot.tint` | `Unknown(blocked_by_ancestor)` | nunca sucesso; implementação/crédito é `Violated` |
| `outline.entry` | `Blocked(BLOCKED_ADR0127_PUBLIC_CONTRACT)` | ausência é controle; elemento/dict/module/reuso é `Violated` |
| cinco `outline.entry.*` | `Unknown(blocked_by_ancestor)` | nunca sucesso; qualquer filho fabricado é `Violated` |
| `selector.before` | `Blocked(BLOCKED_ADR0127_PUBLIC_CONTRACT)` | ausência é controle; alias/composição aproximada é `Violated` |
| `selector.after` | `Blocked(BLOCKED_ADR0127_PUBLIC_CONTRACT)` | ausência é controle; alias/composição aproximada é `Violated` |
| defaults/metadata de `outline` | `Blocked` | preservar defaults cristalinos; mudança sem novo selo é `Violated` |

## 9. Classificação e score

Este é um selo de oráculos pré-candidato, não um veredito de implementação.
Logo os 169 paths autorizados estão **Unknown (não executados contra candidato)**
neste momento e nenhum deles é contabilizado como sucesso. O score de um
veredito posterior é holístico por path:

```text
score exigido = 169 / 169 Preserved
controles bloqueados = fora do denominador
Unknown = nunca sucesso
```

Um path só é `Preserved` se existência, classe de acesso, kind, valor/repr,
metadata, chamadas reais ligada/não ligada, resultado/erro/contexto e
alias/identidade modular coincidirem. Qualquer componente em falta torna-o
`Violated` ou, somente por impossibilidade documentada, `Unknown`.

## 10. Limitações

- A suíte é declarativa e executável, mas não instala um runner novo; o papel
  implementador/verificador deve materializar as fontes isoladas e coletar
  `typst query` conforme o protocolo do JSON.
- Para evitar duplicar milhares de literais, os mapas são selados por
  cardinalidade, extremos e digest integral derivado da fonte ratificada.
- Não houve varredura working-tree-wide nem `git diff --stat`, pois isso
  violaria a segregação pedida. O HEAD e os hashes integrais dos artefatos e
  inputs autorizados bastam para reproduzir este selo; mudanças concorrentes
  fora desses paths não são reivindicadas por este recibo.

**SEALED_ORACLES_FOR_IMPLEMENTATION**
