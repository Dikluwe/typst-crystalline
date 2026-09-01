# ADV-P1284-v2 — recibo adversarial independente

**Estado:** `ADVERSARIAL_GATE_PASSED`  
**Contrato:** `C-P1284-v7`  
**Oráculo:** `A-P1284-v5`  
**Regime:** protocolo completo de materialização segregada  
**Papel:** adversário; sem autoridade para corrigir a solução  
**Momento:** `2026-08-30T12:46:00-03:00`  

## 1. Resultado

As 35 mutações negativas válidas do contrato possuem testemunha determinística
que as rejeita na candidata congelada. O score é:

```text
mutation_score = 35 rejeitadas / 35 válidas = 1.0
controles de gate = 10, fora do denominador
Unknown = nunca sucesso
```

Nenhuma mutação foi injetada no código: isso exigiria alterar a candidata,
operação proibida nesta fase. Conforme a autorização desta auditoria, cada
linha abaixo usa uma testemunha discriminatória já executada contra o binário
congelado ou uma auditoria estrutural reproduzível. A coluna `Forma` distingue
execução de linguagem (`live`), evidência executada pelo oráculo candidato
(`receipt`) e gate estrutural (`audit`). `Rejected` significa que a mudança
negativa faria a testemunha deixar de satisfazer o oráculo; não significa que
foi fabricada uma segunda build mutante.

## 2. Entradas e identidade

| Entrada | SHA-256 |
|---|---|
| contrato `p1284-contract-receipt.md` | `ea85130ae2954d0fb930e68ce82b5554780798ef5927bcab0cc7f109ab9c61f6` |
| recibo de oráculos `p1284-oracle-receipt.md` | `1c04efc1c2100ecc47b7891cb7962beaca64895fc769c3ca509efc084196519d` |
| suíte `p1284-probes.json` | `e8c6a0a2da6dbb784e29ba65e7cfa41b5eb17877e1f1f7fe5760e37d09f89d8f` |
| recibo candidato `p1284-candidate-oracle-receipt.json` | `d14ae6e871f5bb2efe421208774f00577d3142a7c09e1314c288535ee0fce431` |
| `p1284-summary.json` | `19f460ec8445b8869bf22f8354661d77c33e1ea0ffcb065a561dbc5f130a35f2` |
| inventário default | `41a16a2856b335275656af64649f9bbd85465ff9ebf91c979e3398ae3650db4e` |
| inventário HTML | `2a3f5f7d19497f702e8a410c1a0a94a793e5a4420d4a98203ef2268bbabf37f3` |
| probes default | `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb` |
| probes HTML | `828284bcb3f1676c52c9e32944506e4153656db7898b7b06ad07569463386db1` |
| candidata `target/release/typst` | `8b85f933b7cd1fa74e46e2c18902b8343d9064f11b2a76844a476a252835a57e` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

C-P1284-v7 difere normativamente de C-v6 em zero bytes: o contrato demonstra
que sete ressellos tocaram somente `Hash do Código`. A-P1284-v5 preserva o hash
canônico da projeção semântica da suíte em
`4159f8e5b0f683b85eac992f0a99c04e9241d14294721780927fbd4790f3c16c`.
A matriz adversarial e seus controles também permanecem textualmente idênticos
ao ADV-v1, com SHA-256 conjunto
`9548678b6ec73dd90dcfaab633117702ae3afff93be890df1a4b526e906009c9`.
Logo o repin altera somente identidades/proveniência, não testemunhas,
classificadores, numerador ou denominador.

O HEAD observado foi `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
A árvore estava não commitada. O inventário pinado registra integralmente em
`p1284-summary.json:provenance.git_status_short` e
`provenance.git_diff_head_stat` o estado usado para produzir as evidências e o
hash exato do produto cristalino. A seção 8 acrescenta a captura da árvore no
instante deste recibo.

## 3. Execução e evidência bruta

Foram usados estes formatos de comando:

```text
target/release/typst eval '<expressão>' --format json
target/release/typst compile PROBE.typ OUT.pdf
crystalline-lint .
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

As fontes de compile são as fontes seladas do JSON, materializadas de modo
isolado pelo runner que produziu o recibo candidato. Esse recibo contém exit
code, stdout e stderr normalizado por caso. A repetição adversarial por `eval`
foi feita diretamente, sem ficheiros auxiliares e com comparação JSON exata.

Resumo medido:

- recibo candidato: `36/36` sondas funcionais, controle mutador não ligado
  aceito como erro esperado, `15/15` mapas e `4/4` ausências bloqueadas;
- repetição adversarial: `19/19` relações semânticas;
- diagnósticos/casts negativos adicionais: `11/11`;
- mapas reextraídos por `repr`: `15/15`, com cardinalidade, primeiro, último e
  SHA-256 canônico integral iguais ao oráculo;
- inventário congelado: `169/169 active_bilateral` no perfil default e
  `169/169 active_bilateral` no HTML;
- `crystalline-lint .`: exit `0`, output SHA-256
  `3be7d707e590ecbf30ffa62e0ec661941203d1d961a5b3a0eae026721f2bac38`,
  `0` ocorrências de V3/V4/V5/V13/V14/V15/V26 e `0` linhas de severidade
  `error:`. Warnings e infos fora dessas travas não foram promovidos a sucesso
  nem ocultados.

O recibo candidato reexecutado declara `oracle_id: A-P1284-v5`, fixa a suíte
`e8c6a0a2…f89d8f`, fixa a candidata `8b85f933…a57e` e retorna
`all_passed: true`. Sua aritmética é `36 + 1 + 15 = 52`: 36 sondas funcionais,
o controle mutador aninhado e 15 identidades integrais; os quatro controles
de ausência continuam separados desse total.

Os 15 digests reextraídos foram, em ordem do contrato:

```text
turbo    7909421397bc2bef00faf5a1064d7356ad4cab112ec1fd5e290b124a13529a3e
cividis  dece34103d5266311558bb44f96f02df7090edf8977744fc9a1213b2213b2f79
rainbow  74fe385a692d3da43ff8afc8f61896f76bc87a1a766f05f4897f5e8164db4ed7
spectral 1c62ea2e765ddf6d6b1c5189e772605e0203d6116bf384597df0cfa4cdab5f17
viridis  3b9d02b0685ec2ae62958a287c4cdd68fdb08f642339aabaa8d96e60aeaf999b
inferno  20de1b5440e58c3727d645e6f8444d9e6d1c73fb09f9e905a49c314dc58496da
magma    c634397325d83383609535f10e2d964efd899be34a2cef337379a5368b09fa79
plasma   f0ddd8a6d9e12c3c3b5d705dfa2e7c5f7929ce242c8d94ab5e4a0f6b2e0d9f2d
rocket   b3d6e7d7762c8e4d3b27d86aa41391cf8d5093df122b328bc6482f2d37a1497b
mako     f74edb89a1207308a4534b6648d32f54439e0057fb5b4cb0346151610131da77
coolwarm 13161dddf6ad860268eece02ce446b1025d3ab95871cc1a484a93aa05da4aed2
vlag     d015c8a4cb87aaebc49d0416edf430e5c76392e772602ca1234ce6fe748db81d
icefire  0de0d1c2a9dd6a8a03cd3155b872ea4a2eeaa74a5454df8f511632aa5d91e55f
flare    03f70bedc78f4fb3e22b3f7ed06a680d56d721321eca69218ab746d4c803f595
crest    660b6381c0f53156c07b4f0f86988a280c06578f46a44b734c1f388cadeaeb98
```

## 4. Matriz das 35 mutações

| # | Mutação negativa | Testemunha/oráculo | Forma | Classificação |
|---:|---|---|---|---|
| 1 | omitir projeção não ligada | `array.len((1,2,3)) == (1,2,3).len() == 3` | live | Rejected |
| 2 | método de instância como constante | `type(array.len) == function` e chamada real | live | Rejected |
| 3 | constante como função | `type(alignment.left) == alignment`, valor e `repr(left)` | live | Rejected |
| 4 | estática como método com `self` | `type(array.range) == function`; `array.range(3) == (0,1,2)` | live | Rejected |
| 5 | remover/deslocar `self` | formas ligada/não ligada de array, direção e alinhamento | live | Rejected |
| 6 | positional/named trocados | `array.at(..., default:7)` mais erros de argumento | live | Rejected |
| 7 | required tornado optional | `array.at((1,2))` -> `missing argument: index` | live | Rejected |
| 8 | default alterado | `array.range(3)`, `array.at(default:7)` e alphas explícito/implícito | live | Rejected |
| 9 | variadic removido/alargado | `zip` com zero/um/múltiplos e extra positional rejeitado | live | Rejected |
| 10 | named desconhecido aceito | `bytes.len(..., bogus:1)` -> `unexpected argument: bogus` | live | Rejected |
| 11 | kind correto, comportamento errado | len de array/bytes, Unicode NUL e dispatch de chamadas | live | Rejected |
| 12 | ligada/não ligada divergem | `array.map`, `direction.inv`, `alignment.inv` em ambas as formas | live | Rejected |
| 13 | curto-circuito/ordem de callback | `(1,2).all(x => if x==1 {false} else {panic(...)}) == false` | live | Rejected |
| 14 | erro/classe/span alterado | recibo compile verbatim e 11 diagnósticos com exit `1`/needle exato | live + receipt | Rejected |
| 15 | valor/repr de direção/alinhamento | kinds, aliases e `repr(left/rtl)` | live | Rejected |
| 16 | alias duplicado divergente | `color.red == red`, `alignment.left == left`, `direction.rtl == rtl` | live | Rejected |
| 17 | `color.map` não módulo | `type(color.map) == module`; filho é array | live | Rejected |
| 18 | preset omitido | acesso e extração individual dos 15 nomes | live | Rejected |
| 19 | ordem/tamanho/cor de preset | digest da sequência integral canônica de cada mapa | live | Rejected |
| 20 | `arguments.len` conta só positional | `arguments(1,x:2).len() == 2`, ligada e não ligada | live | Rejected |
| 21 | named perdido em map/filter | `.map(...).named() == (x:3)` e `.filter(...).named() == (x:3)` | live | Rejected |
| 22 | confundir bytes/chars/clusters | `bytes("é").len()==2`, `"é".len()==2`, clusters `==1` | live | Rejected |
| 23 | contexto/Location ignorados | state/location/length presentes e ausentes no recibo compile | receipt | Rejected |
| 24 | unidade length/duration trocada | `72pt.inches()==1`, 24h=1 dia, 1 semana=7 dias | live | Rejected |
| 25 | default de outline alterado sem gate | L0 pinado; diff produtivo de outline contém só atualização de lineage | audit | Rejected |
| 26 | `pdf.attach` simula sucesso/perde metadata | kind function + erro explícito contendo `scope-out` | receipt | Rejected |
| 27 | `Unknown` promovido a sucesso | ancestrais ausentes; filhos continuam Unknown e fora do score | live + audit | Rejected |
| 28 | segundo consumer/L0 | `crystalline-lint`: V15=0 e V26=0 | audit | Rejected |
| 29 | `selector.before/after` sem gate/aproximados | ambos ausentes com erro de field; And/Or/Within funcionam separadamente | live | Rejected |
| 30 | nova API Rust pública desnecessária | diff dos consumers P1284 sem linha adicionada `pub`/enum/trait/struct; entities só lineage | audit | Rejected |
| 31 | `Arguments` não chamável/perde named | constructor, type, len, pos/named e preservação map/filter | live | Rejected |
| 32 | casts/ordem/range/alias CMYK errados | forma Ratio válida; Int e 101% rejeitados; global=qualificado | live | Rejected |
| 33 | casts HSL/HSV/range/default errados | Angle obrigatório, Float rejeitado, 101% rejeitado, alpha 100% | live | Rejected |
| 34 | casts/ordem/escala/clamp Oklab/Oklch errados | Ratio lightness, Angle, `40% -> repr 0.16`, chroma 2.0 aceito | live | Rejected |
| 35 | qualificada diverge da global | `repr` igual para HSL, HSV, Oklab e Oklch nas duas rotas | live | Rejected |

Os diagnósticos negativos adicionais tiveram primeiras linhas exatas:

```text
missing argument: index
unexpected argument
unexpected argument: bogus
expected string, found integer
expected ratio, found int
ratio must be between 0% and 100%
expected angle, found int
expected integer or ratio, found float
```

Comparações de cor usam `repr`, não igualdade casual do Rust ou igualdade
aproximada assada: isso preserva a política ADR-0107 e torna ordem, casts,
normalização e defaults observáveis na linguagem.

## 5. Dez controles fora do denominador

| # | Controle do contrato | Observação/classificador | Resultado |
|---:|---|---|---|
| 1 | `color.spot` ausente | erro `type color does not contain field spot` -> `Blocked` | correto |
| 2 | spot/tint implementado sem gate | nenhuma implementação observada; se presente seria `Violated` | correto |
| 3 | `outline.entry` ausente | erro de field em function -> `Blocked` | correto |
| 4 | stub de outline.entry | nenhum stub observado; se presente seria `Violated` | correto |
| 5 | filho de ancestral ausente | tint + cinco filhos de entry -> `Unknown(blocked_by_ancestor)` | correto |
| 6 | mapa ainda Unknown sem probe | todos os 15 foram sondados integralmente -> `Preserved`, nenhum Unknown residual | correto |
| 7 | selector.before/after ausentes | ambos -> `Blocked`, não Violated nem Unknown | correto |
| 8 | before/after implementados sem variantes/gate | nenhum membro observado; presença seria `Violated` | correto |
| 9 | before/after por alias/composição | ausência dos membros e teste separado das variantes existentes | correto |
| 10 | and/or/within existentes | chamadas ligada/não ligada equivalentes -> `Preserved` | correto |

Os controles 2, 4, 8 e 9 são guards contrafactuais: a candidata satisfaz o
controle precisamente por não expor a superfície proibida. Eles não entram no
numerador nem no denominador das 35 mutações.

## 6. Perfis e fechamento observável

A lista de 169 paths autorizados foi reconstruída mecanicamente do catálogo
selado: membros de cada família mais os 15 filhos de `color.map`. Para cada
path, `p1284-summary.json:feature_ledger` informa `active_bilateral` tanto em
default quanto em HTML. Resultado: `169/169` em cada perfil e nenhuma linha
autorizada fora desse estado.

Isso não converte a métrica global dos inventários em percentual de paridade.
Os inventários totais incluem outras famílias, extras históricos e metadata
fora de P1284. O fechamento reivindicado aqui limita-se ao fragmento C-v7/A-v5.

## 7. Independência e limitações

- O papel adversarial leu contrato, oráculo, recibo candidato, inventários,
  summary e somente os trechos produtivos necessários aos ataques 25/30.
- Não editou código produtivo, L0, testes candidatos, suíte de oráculos ou
  inventários. O único ficheiro criado foi este recibo.
- O ambiente é uma árvore compartilhada, não uma sandbox de leitura forte.
  Portanto a formulação correta é **executado com segregação procedimental,
  sem atestação física de isolamento**.
- O hash do binário prova a identidade executada, não que a working tree
  atual seja byte-a-byte a fonte que o produziu. A evidência causal do build é
  o recibo candidato reexecutado e o summary pinados; alterações concorrentes posteriores
  ficam fora deste veredito.
- As 35 mutações foram auditadas por poder discriminatório, não compiladas
  como 35 builds modificadas. Essa distinção impede simular mutation testing.
- `Unknown` aparece apenas nos seis filhos de ancestrais bloqueados e nunca é
  contado como sucesso. Os 15 mapas já não são Unknown porque todos possuem
  sonda integral.

## 8. Captura final da árvore

Preenchida depois da criação deste próprio recibo; editar seu conteúdo não
altera o conjunto de paths reportado por `git status --short`.

```text
HEAD = 53d21c5a602f4045a769a0ab0c935baa5ecd3b88
status_lines = 117
status_sha256 = 9670c09f12dabcbb9bc9f55887dc62cba91ce1601fed41d13923728998cf7c18
diff_stat_sha256 = d87e8175fd7626c6d21cf988b926b05d1e5c3141a0cc2d2b3b756e9c665d770b
diff_stat_summary = 75 files changed, 644076 insertions(+), 1171 deletions(-)
```

**ADVERSARIAL_GATE_PASSED — mutation_score 35/35 = 1.0**
