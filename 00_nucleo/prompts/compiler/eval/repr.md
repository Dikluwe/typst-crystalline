# Prompt L0 — `compiler/eval/repr` — representação morfológica
Hash do Código: 8cda0c57

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

## P1494 — repr idêntica dos espaços (RATIFICADO; gate ADR-0127 confirmado em 2026-09-24)

### Medição anterior à decisão

No HEAD 6da06e849d75972651a342ad6aad337313451c79, com working tree
limpo nas sondas, repr.rs:679 representa Content::Space como "[ ]".
O controle bilateral de igualdade [newline] == [espaço] passa; na fonte
vanilla pinada typst-library/src/text/space.rs:10-45, had_newline é interno
e ambas as origens têm repr "[ ]".

### Decisão ratificada

O match exaustivo repr_content representa Content::SpaceFromNewline
exatamente como Content::Space: "[ ]". A origem não aparece em repr_value,
serializações que a usam nem texto plano. Representação do valor e
normalização da realização são operações distintas. O gate ADR-0127
da nova variante pública de Content foi confirmado em 2026-09-24.


## P1404 — repr da caixa fracionária interna

`Content::Boxed` com `width_fraction: Some(fr)` representa-se como
`box(width: {fr}fr, body: {body})`, preservando a morfologia pública de
`outline.entry.inner`. Caixas sem esse carrier conservam o repr histórico.

## P1307-R5 — snapshot de conteúdo consultado (proposta; gate ADR-0127 pendente)

### Medição anterior à decisão

Baseline histórico R5:
HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree não
commitado com diff/stat integral. A medição independente R5,
preserva fontes, horários e executáveis; referência upstream `a51e02804`.

O caso independente R4 `r2.construct-LocatedContent` conserva como controle
a dívida baseline `heading(level: 1)[[Probe]]`, embora seu vanilla_literal
já contenha todos os campos realizados. R5 fornece os campos que faltavam;
`foundations/content/mod.rs:625–638` mostra
que repr pública não inclui label/location. O formatter cristalino `:35`
aplicava o mesmo repr_content ao raw e ao consultado.

### Decisão proprietária

LocatedContent Some usa os campos congelados, excluindo label, na ordem do
snapshot. Para Heading, emitir a forma de constructor completo:

```text
heading(
  level: 1,
  depth: 1,
  offset: 0,
  numbering: none,
  supplement: [Section],
  outlined: true,
  bookmarked: auto,
  hanging-indent: auto,
  body: [Probe],
)
```

O corpo e cada valor usam repr canônica recursiva, com indentação de filhos
multiline conforme as regras existentes do formatter. Não codificar Probe,
Section ou os valores do exemplo no formatter. Não emitir label, Location,
Arc, set_fields, Debug ou estilos baked. Raw Content e LocatedContent None
conservam repr_content atual; isto é correção estreita de conteúdo realizado,
não remoção de todas as dívidas morfológicas de Content.

Revoga-se apenas para esse snapshot a exclusão de Content na seção R3.
O sucessor de `r2.construct-LocatedContent` deve exigir o vanilla_literal
já medido em cada perfil, deixando o arquivo R4 intacto e preservando os
demais controles. Fallbacks que usam repr, inclusive CBOR de LocatedHeading,
recebem deliberadamente essa nova string. Isso não converte CBOR Content em
mapa nem declara paridade CBOR: o delta é string antiga→string realizada,
medido/separado da saída estruturada vanilla. Aprovação R5 cobre esse delta;
antes do candidato seu envelope tem de ser validado independentemente.

Aceitação: default, numbering distinto, língua distinta, label ignorada em
repr mas preservada em fields; raw controle sem mudança; igualdade não usa
esta string. A fachada pública de repr conserva assinatura.

---



**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/eval/repr.rs`
**Vanila ratificado vigente:** `9f2b6e8715237cb086899a42873660fe744622e8`

## Contrato

### Projeção dos três estados de slot

Nos seis campos, `Omitted` não emite named; `ExplicitNone` emite exatamente
`<slot>: none`; `Present(content)` usa `repr_p1293_math_field(content)`, de
modo que `Present(Content::Empty)` emita `[]`. A ordem permanece
`t,b,tl,bl,tr,br`, e folhas/formatters/multiline/escaping continuam sob as
regras P1293 vigentes. É proibido inferir estado pelo conteúdo ou por origem.
Sintaxe e chamada qualificada devem convergir quando carregam o mesmo estado.

Produzir `repr` exaustivo e determinístico para `Value`, `Content` e
`Selector`, preservando a morfologia pública da linguagem, escaping e formas
constructoras. A aceitação é textual/linguística; a estrutura Rust é mecânica.

Labels usam `<nome>` somente quando o nome satisfaz a gramática literal; caso
contrário usam `label(<repr string>)`. Raiz matemática segue a medição P1426
abaixo. Symbols multi-codepoint preservam todos os scalar values,
incluindo variation selectors e ZWJ, sem conversão para `char`, normalização ou
substituição pelo nome canônico.

## P1490 — morfologia do `Stroke` totalmente automático

### Medição anterior à decisão

Em 2026-09-24T13:41:18-03:00, no vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8` (binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`),
as quatro formas nomeadas produziram, respectivamente:

```text
repr(stroke())                                  -> "stroke()"
repr(stroke(thickness: 1pt))                    -> "1pt"
repr(stroke(paint: black))                      -> "luma(0%)"
repr(stroke(paint: black, thickness: 1pt))      -> "1pt + luma(0%)"
```

`visualize/stroke.rs:306-320` confirma que a forma `stroke()` corresponde aos
dois slots automáticos, enquanto as outras três formas dependem da presença
causal dos slots. No cristalino pré-P1490, medido na mesma árvore não
commitada pelo binário SHA-256
`ba164438e499d0ae1980993bdfc751539f43bbedf029875a102495a508ef3442`,
somente o primeiro resultado divergia: `"1pt + black"`. Os bits existentes
`StrokeFields.paint` e `StrokeFields.thickness` distinguem os quatro estados;
não é necessário inferir presença pelos valores resolvidos nem acrescentar
campo de entidade.

### Decisão proprietária

Quando `cap`, `join`, `dash` e `miter-limit` não são explícitos, a projeção
do par automático com os valores default emite exatamente `stroke()`. Somente
paint explícito emite a `repr` canônica do paint; somente thickness explícito
emite o comprimento; ambos explícitos emitem `<thickness> + <paint>`. A forma
complexa parentizada e o fallback histórico para strokes internos com valores
não-default permanecem inalterados. Layout/render continuam a consumir os
valores resolvidos.

A aceitação cobre as quatro formas acima, composição em tuple e round-trip de
`stroke()` por `eval(repr(...))`. Os shorthands explícitos são reintroduzidos
pelos slots nomeados que o constructor cristalino aceita; o caso combinado
recompõe os dois slots a partir das suas reprs individuais. O round-trip da
suite vanilla por argumento posicional (`stroke(eval(repr(case)))`) continua
fora deste passo, porque o constructor cristalino ainda rejeita posicionais.
Esta é correção de morfologia da linguagem, não igualdade do valor resolvido
nem de bytes renderizados.

## P1426 — `repr` de raiz por presença (APROVADO no gate ADR-0127)

`MathRootElem` com `index_explicit:false` representa
`root(radicand: <repr>)`. Com presença verdadeira, `Some(index)` representa
`root(index: <repr>, radicand: <repr>)` e `None` representa
`root(index: none, radicand: <repr>)`. O formatter usa o bit causal; não infere
presença pelo valor e não altera escaping ou multiline dos filhos.

Esta materialização foi confirmada explicitamente pelo dono em 2026-09-16.

## P1425 — `repr` de `math.limits` por presença (APROVADO no gate ADR-0127)

Para `MathLimitsOverride` com `limits:true`, emitir sempre primeiro
`body: <repr>`. Emitir `inline: <bool>` somente quando
`inline_explicit == true`, inclusive para o valor default `true`; quando o bit
é falso, omitir o campo. Usar o formatter canônico curto/multiline e esta
ordem. `limits:false` continua `scripts(body: <repr>)` e ignora o valor
mecânico de `inline` e o bit, que por contrato é falso.

Não inferir presença comparando `inline` com `true`, nem pela origem sintática.
O carrier é a fonte única. A regra cobre chamada qualificada e sintaxe bare;
outros elementos math ficam fora de escopo. Esta mudança foi confirmada pelo
dono no gate P1425 em 2026-09-16.

## P1292 — morfologia de cancel/underline/vec/flush

### Medição anterior à decisão

O registro vanilla P1292 mediu defaults e formas explícitas. Medição adicional
no mesmo binário pinado confirmou que um named explicitamente igual ao default
permanece no `repr`: por exemplo `inverted:false`, `angle:auto`,
`align:center` e `gap:0.2em`. Logo comparar apenas valor com default perderia
morfologia; os owners de entidade transportam bits de presença.

### Decisão

- `MathCancel` usa `cancel(body: <body>)`. Acrescenta somente named presentes,
  nesta ordem: `length`, `inverted`, `cross`, `angle`, `stroke`, `background`.
  A forma longa usa o formatter multiline vigente. Callback usa sua `repr`
  pública, nunca `Debug`; stroke usa o formatter canônico de `Stroke`.
- `MathUnderline` usa exatamente `underline(body: <body>)`.
- `MathVec` imprime named presentes em ordem `delim`, `align`, `gap` e sempre
  termina com `children: <tupla>`. Vazio é `()`, singleton conserva vírgula.
  Delimiter é par `(<left>, <right>)`, usando `none` para o sentinel ausente;
  gap usa forma integral de relative length (`0% + 1em`).
- `Flush` usa exatamente `flush()`.

Defaults omitidos não são materializados. Defaults explicitamente escritos
são preservados pelos bits. Spans, flags mecânicas de layout e nomes Rust não
aparecem. Esta é morfologia de linguagem (ADR-0107); mensagem de erro permanece
observável no owner construtor (ADR-0108).

### Amendment-6 — projeção morfológica dos filhos sintáticos de `MathVec`

#### Medição anterior à decisão

Em `2026-09-01T02:15:43-03:00`, no vanilla ratificado
`/usr/local/bin/typst`,
os pares sintaxe/chamada qualificada mediram:

- `$vec(a,b)$.body` e `math.vec([a],[b])` →
  `vec(children: ([a], [b]))`;
- `$vec(1,23)$.body` e `math.vec([1],[23])` →
  `vec(children: ([1], [23]))`;
- `$vec("foo","bar")$.body` e `math.vec([foo],[bar])` →
  `vec(children: ([foo], [bar]))`;
- `$vec(alpha,beta)$.body` e `math.vec([α],[β])` →
  `vec(children: ([α], [β]))`;
- os grupos `$vec((a+b),(c))$.body` e
  `math.vec($(a+b)$.body,$(c)$.body)` conservam igualmente a morfologia
  estruturada `lr(body: ...)` de cada filho;
- `$vec(#strong[a],#emph[b])$.body` e
  `math.vec(strong[a],emph[b])` conservam
  `strong(body: [a])` e `emph(body: [b])`, sem wrapper textual adicional.

No candidato medido, texto quoted e markup já convergem, mas folhas sintáticas
diretas ainda aparecem como `a`, `"1"` e `"α"`; substituir a folha armazenada
por `Text` corrige o `repr` à custa de regressão no render matemático já
bilateral. Logo a diferença é de projeção pública neste owner, não de entidade,
eval ou layout. Um identificador multigrapheme não ligado como `foo` é erro de
scope na linguagem; o caso medido usa texto quoted. `alpha`/`beta` cobrem o
identificador multigrapheme resolvido para símbolo.

#### Decisão estreita

Somente ao formatar os itens diretos do campo `MathVec.children`, uma folha
`Content::MathIdent` ou `Content::MathText` projeta a sua sequência visível
pelo formatter canônico de conteúdo textual e, portanto, usa morfologia
`[<texto>]`. A projeção usa o escaping de conteúdo vigente; não concatena
colchetes crus. O valor continua armazenado como a mesma variante matemática:
igualdade, hash, span, traversal, classificação, estilo e layout não mudam.

Todas as demais variantes-filho usam sua `repr` própria sem projeção nem
recursão especial. `Content::Text` já produz `[texto]`; markup permanece
`strong(...)`/`emph(...)`; grupos e demais estruturas continuam pertencendo
ao formatter da respectiva variante. A regra não se aplica a tuples genéricas,
`Equation`, outros elementos matemáticos ou folhas aninhadas dentro de um
filho estruturado. Não cria campo, wrapper, provenance bit nem branch no
renderer.

Esta decisão preserva os resultados públicos A-D: apenas torna a morfologia
do filho-folha sintático de vec idêntica à chamada qualificada, sem alterar
children, named, presença, cardinalidade ou ordem. A divergência candidata já
existente na forma interna de um grupo `MathLr` não autoriza ampliar P1292; se
um vetor exigir igualdade vanilla exata desse filho estruturado, deve parar no
owner genérico correspondente, não ser mascarado por brackets neste branch.

Refutam o amendment: qualquer mudança de SVG/frame/classificação; mudança da
igualdade/hash do elemento; double-wrap de `Text`/markup; brackets crus que não
sigam escaping; projeção recursiva de grupo; ou diferença entre sintaxe e
chamada qualificada para as folhas medidas. Casos de conteúdo não medidos e
qualquer variante estruturada cujo formatter próprio divergir permanecem
`Unknown`, nunca sucesso implícito.

## Medição P1290 — arrays/tuplas e `MathOp` (precede a decisão)

Baseline medida em árvore não commitada, `HEAD`
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, em
`2026-08-31T10:40:23-03:00`. O vanilla ratificado foi
`/usr/local/bin/typst`.
O cristalino foi compilado dessa árvore sem reutilizar binário antigo e
reportou `typst 0.15.1 (53d21c5a)`.

Na medição bilateral preservada, o vanilla produz `()`, `(0,)` e
`(0, 1, 2)` para vazio, singleton e curto. Em
corpos internos ASCII de 49 e 50 caracteres
ficam lineares, enquanto 51 quebra. As sequências de zeros confirmam a mesma
fronteira por largura, não por cardinalidade: 17 itens produzem corpo de 49 e
ficam lineares; 18 produzem 52 e quebram. Em
um item já multilinha força o container externo
a quebrar; cada nível acrescenta dois espaços, cada item termina em vírgula e
o parêntese final volta à indentação do seu container.

Na medição preservada, o vanilla mede `math.csc`, `math.dim` e
`math.arcsin` como, respectivamente,
`op(text: [csc], limits: false)`, `op(text: [dim], limits: false)` e
`op(text: [arcsin], limits: false)`; `math.lim` mede
`op(text: [lim], limits: true)`. `math.op("foo", limits: false)` e
`math.op("foo", limits: true)` conservam o booleano no mesmo segundo campo.
Conteúdo com markup mede forma multilinha com `text:` antes de `limits:`;
aspas e barra invertida em string permanecem na morfologia do conteúdo.
`math.sum` mede `symbol("∑", ("integral", "⨋"))`, portanto não é testemunha
de `MathOp`.

A invalidação funcional refutou o caso anterior
`math.op([a \# "quote"], ...)`: o markup cristalino
original já entrega aspas curvas como `Content::Text`, sem a proveniência
`SmartQuote` que o vanilla ainda representa. Em
`2026-08-31T11:43:44-03:00`, usando os mesmos binários congelados, o probe
substituto `math.op([a \# \*], limits: false)` mediu no vanilla
`op(\n  text: sequence([a], [ ], [#], [ ], [*]),\n  limits: false,\n)` e no
cristalino original `op(sequence([a], [ ], [#], [ ], [*]))`. Assim, `\#` e
`\*` observam escaping sem introduzir proveniência destruída; a divergência
restante pertence integralmente aos nomes/ordem dos campos deste owner.

Esta medição observa morfologia pública, não a mecânica do algoritmo. Não se
infere do comportamento qual helper, estrutura Rust ou unidade interna de
contagem o vanilla usa. A fronteira de 50 está fechada para os casos ASCII
medidos; a unidade aplicável a representações não-ASCII permanece `Unknown`
até probe específico. Também permanecem `Unknown` membros de `math` e formas
de conteúdo não enumerados no contrato P1290. Permanece ainda `Unknown` a
proveniência de `SmartQuote` que uma fase anterior já resolveu em glifos de
texto: este owner não pode distinguir legitimamente uma smart quote resolvida
de texto curvo literal. Evidência que refutaria a classificação: um probe
bilateral reproduzível no escopo enumerado que não siga as strings ou a
fronteira acima; tal achado invalida este congelamento e exige nova Fase A.

## Decisão P1290 — contrato morfológico congelado

Arrays/tuplas serializam os itens visíveis pela sua própria `repr` e usam `", "`
entre itens no modo linear. P1305-r2, abaixo, refina explicitamente a regra
anterior de representar todos os itens: arrays acima de 40 itens elidem somente
a representação excedente. Vazio é `()`; singleton conserva a vírgula
`(<item>,)`. Quando o corpo interno unido contém quebra de linha ou excede a
fronteira ASCII medida de 50 caracteres, a forma é multilinha: `(`, newline,
um item por linha com dois espaços adicionais por nível e vírgula final,
seguido de newline e `)` na indentação do container. Nesting reindenta todas
as linhas do item, sem achatar representações internas. Casos `Unknown` não
podem ser promovidos implicitamente a preservados.

Todo `Content::MathOp` usa a forma construtora
`op(text: <repr do conteúdo>, limits: <bool>)`, sempre com ambos os nomes de
campo, nessa ordem, preservando o valor real de `limits`. A mesma regra cobre
operadores nativos simples, nomes compostos e `math.op` criado pelo usuário;
é proibida especialização nominal de `csc`, `dim` ou de qualquer conjunto
fechado de testemunhas. Quando a representação dos campos exige quebra,
aplica-se a mesma disciplina multilinha de dois espaços por nível e vírgula
final. Variantes que não sejam `MathOp`, como o `symbol` observado para
`math.sum`, conservam o contrato da sua própria variante.

Escaping de conteúdo no escopo deste owner é coberto pelo caso medido sem
smartquotes `math.op([a \# \*], limits: false)`. Quando `repr` recebe apenas
os glifos curvos já resolvidos, deve representar o conteúdo efetivamente
recebido; é proibido reconstruir `SmartQuote` por reconhecimento de glifo,
heurística textual ou especialização nominal. P1290 não autoriza mudar a fase
de resolução, editar outro owner ou ampliar contrato público para recuperar
essa proveniência; tal mudança seria novo escopo e gate humano ADR-0127.

## Projeção morfológica de `attach`, `binom` e estilos math

`MathAttach` usa sempre os seis campos na ordem `t,b,tl,bl,tr,br`: omite
`Omitted`, imprime `none` para `ExplicitNone` e delega `Present` à
projeção normal de conteúdo. `binom` representa upper e lower na ordem,
preservando o lower variádico.

`MathStyled` usa a identidade pública canônica do kind. `mono(body)` e
`script(body, cramped: ...)` tratam body como positional e mantêm a
morfologia das folhas de conteúdo, inclusive texto, símbolo e conteúdo vazio.
A projeção não infere presença por `Content::Empty` nem reescreve payload.

## Forma de `HtmlElem`

A forma curta é usada apenas quando o valor cabe integralmente nela. Caso
contrário, `HtmlElem` é bloco multiline com `tag`, `attrs` e `body`
nessa ordem, indentação estrutural e vírgula por campo. Ordem de atributos,
escaping e morfologia do body são preservados; não normalizar whitespace para
aceitar ambas as formas.

## Restrições e aceitação

A representação textual é observável de `repr`, mas não autoriza mudar
payload, API, eval, layout ou export. Testes cobrem os três estados de slot,
lower variádico, mono/script e formas curta/multiline de HTML.

## P1305-r2 — arrays longos e módulos nomeados

### Medição anterior à decisão

Sobre HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b` mais o diff P1303
não commitado, a medição independente preserva status, diff/stat, fixtures,
argv/cwd e saídas integrais.
Executada em `2026-09-07T13:48:28.580684+00:00` até
`2026-09-07T13:51:19.266604+00:00`, usa o vanilla ratificado `a51e02804`
e baseline fresco.

Em `01_core/src/compiler/eval/repr.rs:36-41`, o baseline representa todos
os itens. Em `lab/typst-original/crates/typst-library/src/foundations/array.rs:1188-1199`,
o limite é 40 e o texto da cauda informa a quantidade omitida. Os probes
`array-repr-40`, `array-repr-41` e `array-repr-42` medem, respectivamente,
ausência de marcador, `.. (1 items omitted)` e `.. (2 items omitted)` no
vanilla; nesting conserva reindentação. `ascii-body-49/50/51` reconfirma a
fronteira P1290; `array-data-*` conserva a sequência completa depois de repr.
Os controles `fields-dict-41` e `fields-args-41` já divergem em forma do
vanilla, mas preservam todos os seus fields/argumentos no baseline; não são
autorização de elisão neste passo.

Em `repr.rs:56`, o wrapper Module é `module(nome)`. O vanilla imprime o
nome armazenado entre `<module ` e `>` (`foundations/module.rs:174-179`).
`named-modules`, `ordinary-collisions`, `reexport-collision`,
`imported-route`, `nested-import-route` e `document-route` distinguem global,
aliases e módulos ordinários. O carrier não distingue global de um arquivo
`std.typ` que reexporta `std: *`; a contraprova anterior está na medição histórica.
O dono autorizou ampliar exclusivamente os owners de construção do global;
seus L0 passam a guardar o nome público `global` no objeto, mantendo o
binding lexical `std`. Este formatter não reconstrói origem por conteúdo.

### Decisão e aceitação

Todo `Value::Array`, independentemente de origem, representa os primeiros
`min(len, 40)` itens na ordem original. Quando `len > 40`, acrescenta uma
peça final literal `.. (<len - 40> items omitted)`, inclusive o plural
`items` quando a quantidade é um. Não há peça adicional em `len <= 40`.
Essa lista de peças segue a disciplina canônica P1290: vazio `()`, singleton
com vírgula, forma curta/multilinha, dois espaços por nível, vírgula final e
reindentação de cada linha. Arrays aninhados aplicam a regra em cada array
real. O marcador não é um valor inserido no array.

Somente a projeção de `Value::Array` sofre elisão. O helper genérico de
listas não adquire limite; campos de constructors, sequências de conteúdo,
argumentos, dicionários e outras listas que o reutilizam mantêm seus contratos.
Se um field contém um array real, a própria repr desse valor segue a regra
de array. Comprimento, ordem, lookup, valores omitidos e serialização
estruturada integral permanecem inalterados; a fachada textual usa esta repr.
Não selecionar mapas de cor ou nomes de testemunhas.

Todo Module nomeado usa exatamente `<module <name()>>`, sendo `<name()>`
substituído pelo nome armazenado. Global já chega nomeado `global`; import
ordinário de `std.typ`, inclusive reexport integral, permanece `std`.
Aliases não mudam esse nome. Não converter `std` em `global` no formatter,
inspecionar bindings para reconhecer stdlib, nem consultar paths, spans,
identidades de ponteiros, nomes de fixtures ou origem sintática. Anonimato
de plugin permanece fora de escopo; não alterar Module.

A expectativa histórica do teste `repr_value_module` neste owner deve ser
retificada de `module(mylib)` para `<module mylib>`, com autoria independente
de oráculo. Refutam o contrato limite incorreto, marcador/quantidade/forma
divergente, truncamento dos dados, elisão em helper genérico, regressão
P1290, conversão nominal de import ordinário ou discrepância entre rotas do
global. Política `Unknown`: nenhum caso obrigatório desconhecido satisfaz
aceitação. Morfologia é linguagem (ADR-0107); esta correção de paridade
segue ADR-0127 em fluxo contínuo dentro da ampliação autorizada, sem nova API,
entidade, fase, default de produto ou edição de diagnóstico neste owner.

## P1307-R3 — projeções canônicas e ordem causal de arguments

### Medição anterior à decisão

A medição histórica cobre State/Counter com dados disponíveis, mas omitidos por
`01_core/src/compiler/eval/repr.rs:122-123`; Location usa três pontos em
`:87`, e `:60-78,1033-1040` confunde With nativo com sua função interna.
A medição R2
confirma `location(..)` pela rota contextual e `(..) => ..` para o encoder
parcial, nos quatro perfis. Sua proveniência é HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` mais o working tree P1306, diff
e fontes integrais no baseline R2.

Na fonte ratificada `a51e02804`, sob
`lab/typst-original/crates/typst-library/src/`, `introspection/state.rs:370-374`
usa chave e init; `introspection/counter.rs:519-523,557-565` distingue as
chaves; `foundations/func.rs:460-470` torna todo With anônimo na repr.
`foundations/args.rs:455-459` percorre a sequência causal; o cristalino
`:108-120` projeta named antes de positional. A correção da ordem é decisão
de fonte, ainda requer medição bilateral independente antes do RED.
`compiler/stdlib/loading.rs:263` já usa este formatter no fallback CBOR:
alterar a repr propaga à string codificada desse fallback.

### Classificação e contrato proposto

São projeções públicas da linguagem, não Debug nem igualdade Rust.
O único consumer continua `01_core/src/compiler/eval/repr.rs`.

- State já construído: `state(<repr da chave string>, <repr do init>)`;
  preservar escaping e multiline do filho, sem reaplicar um pretty-printer
  ao constructor inteiro ou omitir seus dados.
- Counter já construído: Page produz `counter(page)`, Str usa string
  escapada/quotada, Selector delega ao formatter canônico do selector.
  `page` não é `"page"`; não corrigir constructors nem selectors compostos
  incidentalmente. A forma do filho não paritário continua dívida explícita.
- Location: `location(..)`, opaca e independente de sua identidade interna.
- Todo `FuncRepr::With`, inclusive de nativa, usa `(..) => ..`; a função
  original mantém sua repr, nome e namespace. Não mudar `Func::name()`.
  A regra não é um reconhecimento nominal dos três encoders.
- Args com `occurrences: Some` usa valores e nomes na ordem causal, inclusive
  named repetidos. `None` conserva a projeção legada named-first. Não imprime
  spans, discriminante ou metadados. Usa o carrier do owner Args, nunca
  reconstrói a origem por igualdade dos valores.

A fachada `repr_value_for_serialization` mantém sua assinatura e delegação.
Loading reutiliza-a somente no fallback; não duplica estes formatters.
As mudanças propagam deliberadamente aos fallbacks textuais existentes,
incluindo CBOR para State/Counter/Location/With e Args cuja ordem muda.
Não se promete preservação byte-a-byte desses casos. Medir e congelar seus
deltas antes do candidato; preservar CBOR nas demais classes, especialmente
a dívida Symbol/Content. O corpus CBOR R2 não prova preservação geral.

É inferência que os dados existentes, acrescidos do carrier causal aprovado,
bastam; perda de dados do filho, identidade por fixture ou necessidade de
nova entidade além de Args a refutam. Fora do escopo: default de constructor
State, aceitação de Label por Counter, repr de selector composto e correção
geral de Content. Este conjunto aguarda gate público ADR-0127 de P1307-R3;
a redação não autoriza código nem atesta RED/GREEN. Testes precisam cobrir
repr direta, fallback dos formatos, parcial de outras nativas e ordem de
Args com duplicatas antes de alegar aceitação.

## P1308 — largura de arguments e migração de expectativas

Medição anterior à decisão,
fixa baseline e binários e reproduz, em quatro perfis e três ordens, a
fronteira horizontal de 50 bytes e vertical a partir de 51 bytes. A fonte
ratificada `foundations/args.rs:455-459` delega ao pretty-printer de sequência.
O baseline já satisfaz esta fronteira; a expectativa histórica inline longa
não a satisfaz. Classificação: morfologia pública, não mecânica Rust.

Args com occurrences presentes conserva ordem causal e todos os valores,
usando o pretty-printer de sequência sem vírgula de singleton, largura de
50 bytes e forma multiline quando excedida; não elide argumentos. Args
sintético sem occurrences conserva a projeção legada já especificada.
A regra longa e sua propagação aos fallbacks são vigentes. Quando o baseline
já a cumpre não há mudança produtiva. A expectativa de Location neste consumer
é `location(..)`; nenhum outro expected é relaxado.

## P1339 — grupo público de filtros de elemento

### Medição anterior à decisão

No HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, consumer intacto,
`repr.rs:1040-1069` tem match exaustivo sobre Selector e representa cada
Where antigo separadamente. A medição histórica, UTC
`2026-09-10T00:04:21.279738+00:00`–`00:04:21.402736+00:00`, mede
grupo vazio `strong.where(:)` e grupo ordenado único para heading, em vez
de chamadas Where encadeadas. A medição preserva o estado e os resultados.
Em `lab/typst-original/crates/typst-library/src/foundations/selector.rs:309-321`,
a apresentação distingue ausência do filtro de filtro presente vazio.

### Decisão de integração

O novo braço Selector::Element aprovado no gate P1339 imprime o nome
canônico da função seguido de `.where` e da apresentação de um grupo named.
Vazio imprime `(:)`; não vazio usa os campos na ordem armazenada, com
`nome: repr_value(valor)`, escaping e disciplina curta/multilinha do formatter
canônico de named. Nenhum campo é elidido. Values aninhados mantêm seus
formatters próprios, inclusive elisão de arrays reais e reindentação.
Não usar Debug de Func/Selector, endereço de função ou o nome lexical de alias.
Não reordenar campos, achatar o grupo para Kind ou encadear vários `.where`.

A mesma projeção chega a CounterKey::Selector e aos fallbacks textuais
existentes por delegação, sem novo serializer nem nova assinatura pública.
Esses deltas de strings para os novos valores devem integrar o contrato antes
do candidato; não afirmar preservação de bytes dos fallbacks para eles.
As variantes anteriores, incluindo as formas históricas de And/Or/Where,
mantêm sua representação. P1339 não corrige repr geral de seletores compostos.

Aceitação: vazio, campo único, ordem oposta, spread com sobrescrita, alias,
filhos multiline, valores escapados e projeção em Counter/fallback. Refuta a
decisão qualquer perda de grupo, ordem ou valor, ou alteração do repr de
variantes anteriores. Morfologia é linguagem; escolher um helper é mecânica.

## P1397 — projeção canônica de classe e estilo matemáticos

### Medição anterior à decisão

O estado causal é o HEAD limpo
`c5af9f2af02bd830ec223f777a8d144356b03597`, observado em
`2026-09-14T16:55:44-03:00`: este L0 tinha
e seu consumer tinha.
O binário vanila ratificado tinha;
o cristalino baseline.

Sob limite de endereço virtual de 512 MiB, timeout de 60 s, execução serial e
swap zero, as sondas `eval --format raw` mediram:

- `repr(math.dif)`: o vanila produziu a sequência multiline com
  `class(class: "unary", body: styled(child: [d], ..))`; o cristalino produziu
  `sequence(h(amount: 0.17em, weak: true), class("unary", "d"))`;
- `repr(math.Dif)`: a mesma diferença, com `[D]` no vanila e `"D"` no
  cristalino;
- `repr(math.class("unary", [x]))`: o vanila produziu
  `class(class: "unary", body: [x])`; o cristalino,
  `class("unary", [x])`;
- `math.bb`, `math.frak`, `math.serif` e `math.inline` aplicados a `[x]`
  produzem `styled(child: [x], ..)` no vanila, enquanto o cristalino baseline
  os achata para `[x]`; `math.mono` e `math.script` já preservam o wrapper nos
  dois lados e são controles positivos da forma;
- `repr(math.lim)` já coincide como `op(text: [lim], limits: true)` e é controle
  negativo de escopo.

As execuções válidas locais de `dif`, `Dif`, `class`, `bb` e `lim` no vanila
usaram no máximo 18.036 KiB de RSS; as correspondentes válidas no cristalino,
33.036 KiB. `math.upright` no cristalino terminou exit 1 porque o binding não
existe nesse módulo e foi descartado: não sustenta decisão nem aceitação.

A intenção não é inferida apenas dessas saídas. Na fonte ratificada,
`lab/typst-original/crates/typst-library/src/math/op.rs:49-54` constrói `dif` e
`Dif` como espaço fino fraco seguido de `ClassElem(Unary, upright(...))`;
`math/mod.rs:141-149` declara os campos públicos obrigatórios `class` e `body`;
`foundations/content/mod.rs:626-640` representa elementos pelos nomes e campos
declarados, enquanto `:759-788` dá a todo `StyledElem` a forma explícita
`styled(child: ..., ..)`. Os arquivos têm, respectivamente,
`a8a64e505979b8a6fce3bb5fef978677a40200199e9dc36738a3343ae1104957`
e `a477eb1013d08d18a39da3c3341d666b37a4a9bab8d14d93e3327aff78674245`.

### Classificação e decisão

Essas strings são morfologia pública de `repr` (ADR-0107), não igualdade,
estrutura Rust, passos de layout ou bytes exportados. Para todo
`Content::MathClassOverride`, representar sempre
`class(class: <repr da classe string>, body: <repr do corpo>)`, nessa ordem.
Ambos os campos são obrigatórios e nunca posicionais; o formatter canônico
decide forma curta ou multiline sem especialização nominal de `dif`, `Dif`,
`unary` ou de qualquer testemunha.

Para todo `Content::MathStyled` que transporta qualquer eixo matemático
explícito, preservar o wrapper `styled(child: <repr do corpo>, ..)`. Isso inclui
variante tipográfica, tamanho, `cramped`, `bold` e `italic`, inclusive valor
igual ao default e variante Plain. Não projetar o kind como chamada nominal,
não achatar para o filho e não reconhecer glifos ou nomes. O corpo usa sua
representação canônica recursiva; a estrutura de estilo permanece opaca pelos
três pontos, como no vanila.

`math.dif` e `math.Dif` são testemunhas compostas obrigatórias; `math.class`
prova a regra geral do primeiro wrapper; `bb`, `frak`, `serif` e `inline`
provam eixos antes achatados; `mono` e `script` comprovam preservação; `lim`
comprova ausência de deriva em `MathOp`. A sequência exterior usa o formatter
canônico vigente, portanto quebra em múltiplas linhas quando as peças corrigidas
assim exigirem. É proibido codificar a quebra ou as letras das testemunhas.

Trata-se de correção interna de paridade no owner de representação, sem nova
API, novo default de produto, troca de fase ou quebra deliberada; segue fluxo
contínuo ADR-0127: L0 primeiro, RED, candidato, GREEN e revalidação.
A hipótese é refutada se a revisão ratificada não transportar os campos acima,
se uma sonda bilateral válida produzir outra forma para qualquer caso fechado,
ou se preservar o wrapper mudar payload, igualdade, hash, eval ou layout.

### Limites e aceitação

P1397 autoriza somente os braços `MathClassOverride` e `MathStyled` deste
consumer, expectativas test-only diretamente contraditórias e metadata de
linhagem. Não autoriza alterar entidades, construção de `dif`/`Dif`, stdlib,
parser, eval de conteúdo, layout, export, `MathOp`, outros braços de `repr` ou
helpers genéricos fora do necessário para essas duas projeções.

Ficam fora de escopo: criar o binding cristalino `math.upright`, corrigir
outras ausências da stdlib, provar paridade geral de matemática, inspecionar
styles internos no texto público, ou fazer o wrapper depender da origem
sintática. Casos não medidos permanecem `Unknown`. Identidade ambígua, falha
de parse/build/execução, timeout, OOM, swap, saída ausente ou ilegível também
são `Unknown`; `Unknown` obrigatório bloqueia e nunca conta como sucesso.

Aceitação exige todas as testemunhas e controles acima, repetição e ordem
inversa estáveis, mutação discriminatória integral e zero regressão observada
fora dos dois braços. Sondas CLI rodam uma por vez sob `RLIMIT_AS` de 512 MiB e
timeout de 60 s. Build/check/lint rodam serialmente, uma job, sob `RLIMIT_DATA`
de 2 GiB; swap deve ser zero e RSS acima de 1.782.579 KiB interrompe a
sequência. É proibido executar `cargo test` amplo ou compilar o lib-test
monolítico para P1397.

## P1399 — opacidade morfológica de blocos contextuais

### Medição anterior à decisão

O estado causal é o HEAD limpo
`bdec21bf935cbf7f61070756cd0f218e441b7e07`, observado em
`2026-09-14T17:54:52-03:00`: este L0 tinha
e seu único consumer produtivo tinha.
O binário vanila ratificado, commit
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, tinha;
o cristalino baseline recém-construído tinha.
A construção serial sob `RLIMIT_DATA` de 2 GiB terminou em 8,84 s, com RSS
máximo de 319.792 KiB e swap zero.

Entre `2026-09-14T17:54:52-03:00` e
`2026-09-14T17:55:08-03:00`, sondas bilaterais seriais sob `RLIMIT_AS` de
512 MiB mediram:

- `repr(context [x])`, `repr(context [different])`,
  `repr(context {1 + 2})` e `repr(context text.lang)` produziram
  `context()` no vanila e `context(...)` no cristalino baseline;
- `repr((x) => x + 1)` produziu `(..) => ..` nos dois lados;
- `repr([x])` produziu `[x]` nos dois lados.

Na fonte ratificada,
`lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:626-640`
delega a representação de um elemento à representação específica do handle
ou aos seus fields públicos; o arquivo tem.
Em
`lab/typst-original/crates/typst-library/src/foundations/context.rs:65-83`,
o único field de `ContextElem` é `func`, marcado interno; o arquivo tem.
Assim, nenhum field integra sua forma pública e a representação resultante é
o constructor vazio `context()`. No cristalino baseline,
`01_core/src/compiler/eval/repr.rs:1016` codifica `context(...)` para o braço
`Content::ContextBlock`.

### Classificação e decisão

O texto retornado por `repr` é morfologia pública da linguagem (ADR-0107),
não a estrutura Rust do bloco, a identidade da closure, a avaliação do corpo,
os passos de layout ou bytes de export. A fonte declara a intenção por
visibilidade de field e pela regra geral de representação de elementos; a
decisão não é inferida somente das saídas medidas.

Para todo `Content::ContextBlock`, sem exceção, `repr` deve produzir exatamente
`context()`. A forma é independente do corpo encapsulado, da closure interna,
de identidade, span, origem sintática e conteúdo avaliado. É proibido emitir
reticências, inspecionar ou representar o corpo, reconhecer nominalmente as
quatro testemunhas, ou decidir a forma por igualdade, hash, endereço, id ou
origem.

A representação genérica de funções permanece `(..) => ..` e a representação
de conteúdo textual comum permanece sob seu owner, inclusive `[x]` para o
controle medido. Nenhum helper geral de constructor, função ou conteúdo muda
por P1399. A fachada pública de `repr` e todas as assinaturas permanecem
inalteradas.

Trata-se de correção interna de paridade morfológica em um braço privado do
consumer existente, sem nova API, entidade, default de produto, mudança de
fase ou quebra deliberada. Segue fluxo contínuo ADR-0127: L0 primeiro, RED,
candidato, GREEN e revalidação. A hipótese é refutada se a fonte
ratificada expuser field público de `ContextElem`, se uma sonda bilateral
válida de qualquer `ContextBlock` produzir forma pública distinta de
`context()`, ou se a correção exigir alterar entidade, eval, layout, closure
ou helper geral.

### Limites, política de desconhecido e aceitação

P1399 autoriza somente o braço `Content::ContextBlock` em
`01_core/src/compiler/eval/repr.rs`, expectativas test-only diretamente
contraditórias e metadata de linhagem. Não autoriza alterar entidades,
construção, parser, eval, layout, representação de closure, representação de
outras variantes nem helpers gerais.

Identidade ambígua, falha de parse, build ou execução, timeout, OOM, swap,
stream truncado ou ilegível e budget esgotado produzem `Unknown` fail-closed.
`Unknown` em caso obrigatório bloqueia e nunca conta como sucesso. Aceitação
exige a regra geral para corpos textuais distintos, corpo de código e acesso
contextual; preservação dos dois controles; repetição e ordem inversa
estáveis; e rejeição de toda mutação negativa válida.

Oráculos rodam serialmente, um processo por vez, com `RLIMIT_AS` de 512 MiB,
timeout interno de 10 s, timeout externo de 60 s e teto de 64 KiB para cada
stream. Build, check e lint rodam sequencialmente, uma job, sob `RLIMIT_DATA`
de 2 GiB. É proibido executar `cargo test` ou compilar o lib-test monolítico.
Swap não zero ou RSS acima de 1.782.579 KiB interrompe a sequência. Duas
revisões consecutivas sem ganho discriminatório pela mesma causa reabrem o
desenho; o budget nunca autoriza relaxar `Unknown` ou o contrato.

## P1401 — representação de seletores contextuais (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O baseline P1401 não possui arms de repr para `Before` ou `After`. O vanila
pinado mede `heading.before(heading)` e `heading.after(heading)` no default, e
expõe `inclusive: false` somente quando explicitamente falso.

### Decisão proposta

Repr produz `<base>.before(<end>)` e `<base>.after(<start>)` quando inclusive é
true; quando false, acrescenta `, inclusive: false`. Base e limite usam o repr
recursivo público, sem Debug nem detalhes de Box. O texto é observável da
linguagem e aguarda confirmação ADR-0127.

# P1402 — repr de entry

`Content::OutlineEntry` usa nome `entry` e lista somente campos assentes na
morfologia corrente. Ocorrências consultadas usam o snapshot realizado e
imprimem `level`, `element` e `fill` nessa ordem.

## P1427 — morfologia pública de fração (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O cristalino imprime `(num)/(den)`. O vanilla imprime
`frac(num: ..., denom: ...)`; inclui `style` somente quando explicitamente
definido, inclusive `vertical`, e nunca expõe os bits internos de parênteses.

### Decisão proposta

Uma `MathFrac` pública com `line=true` usa campos `num`, `denom` e, se
presente, `style`, nessa ordem. Folhas math diretas usam a projeção Content
vigente. Bits causais não aparecem. O reconhecimento canônico de `binom`
continua prioritário para `line=false` e não ganha campo `style`.

## P1429 — morfologia pública de acento (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O cristalino imprime dois posicionais. O vanilla imprime campos `base` e
`accent`, com o codepoint já normalizado, e acrescenta `size`/`dotless`
somente quando explicitamente definidos, inclusive nos valores default.

### Decisão proposta

Imprimir `accent(base: <repr>, accent: <string escapada>)`, seguido por
`size: <relative>` e `dotless: <bool>` na ordem canônica somente quando os
respectivos bits de presença estiverem ativos. Cascata aplicada no layout não
fabrica presença no elemento. Posição superior/inferior e dotless realizado
não aparecem como campos derivados.

## P1430 — morfologia pública de `cases` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O cristalino imprime apenas `cases`. O vanilla imprime sempre
`children: (...)` e inclui `delim`, `reverse` e `gap`, nessa ordem, somente
quando cada campo foi explicitamente fornecido; explicitude igual ao default
continua observável.

### Decisão proposta

Imprimir `cases(delim?, reverse?, gap?, children: <tuple>)`. Zero children usa
`()`, um usa a vírgula terminal e muitos preservam ordem. Delimitadores
ausentes aparecem como `none`, nunca como sentinel; gap usa o repr relativo.
Cascata não fabrica campos explícitos e a separação de `&` para layout não
altera a morfologia armazenada.

## P1431 — morfologia pública de `mat` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O cristalino imprime apenas `matrix`. O vanilla imprime sempre `rows` como
tuple de tuples retangular e inclui, na ordem, `delim`, `align`, `augment`,
`row-gap` e `column-gap` somente quando causalmente explícitos. `gap` é
projetado nos dois gaps, nunca como campo próprio. Augment vertical único com
stroke auto usa forma inteira; os demais usam dict canônico.

### Decisão proposta

Projetar `mat(delim?, align?, augment?, row-gap?, column-gap?, rows: ...)`,
preservando `none`, offsets assinados, stroke auto/customizado, células vazias
de padding e explicitude igual ao default. Cascata não fabrica presença no
carrier. A quebra horizontal/vertical segue o helper canônico de repr sem
alterar a ordem dos campos.

## P1436 — morfologia pública de `lr` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O cristalino perde a variante e imprime body ou delimitadores crus. O vanilla
imprime sempre `lr(body: ...)` e inclui `size` antes do body somente quando o
argumento foi explicitamente fornecido, inclusive `100%`.

### Decisão proposta

Projetar `MathLr` como `lr(size?: <relative>, body: <content>)`, nesta ordem.
Body vazio usa `[]`; múltiplos preservam o `sequence` com vírgulas inseridas;
um `lr` interior usa recursivamente seu formatter próprio. Valor vindo apenas
de set rule não fabrica o campo `size`. Não reconhecer identidade `lr` por
forma de `MathDelimited`, e manter prioritário o formatter canônico de
`binom` sobre seu carrier mecânico atual.

## P1437 — morfologia pública de `mid` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O cristalino perde a identidade bare e não possui forma qualificada. O vanilla
imprime sempre `mid(body: <content>)`, inclusive vazio, sequência e nesting.

### Decisão proposta

Projetar `MathMid` exatamente como `mid(body: <content>)`. Body vazio usa
`[]`; conteúdo composto preserva seu formatter; `mid` interior recorre ao
mesmo formatter. Não inferir identidade por glifo, classe ou posição em `lr`.

## P1439 — morfologia pública de `overbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O cristalino imprime `xover("⏞")` ou `xover("⏞")over(n)`. O vanilla imprime
`overbrace(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathOverbrace` como `overbrace(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. O campo annotation só aparece quando
`annotation_explicit` for verdadeiro; seu valor explícito `none` não se
confunde com omissão. Nesting recorre ao mesmo formatter e identidade nunca é
inferida do glifo U+23DE ou do carrier genérico.

## P1440 — morfologia pública de `overbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino imprime `xover("⎴")` ou `xover("⎴")over(n)`. O vanilla imprime
`overbracket(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathOverbracket` como `overbracket(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. O campo annotation só aparece quando
`annotation_explicit` for verdadeiro; seu valor explícito `none` não se
confunde com omissão. Nesting recorre ao mesmo formatter e identidade nunca é
inferida do glifo U+23B4, de `MathOverbrace` ou do carrier genérico.

## P1441 — `repr` de `MathOverline` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O vanilla projeta `overline(body: [x])`; a forma cristalina atual
`overline[x]` identifica a decoração textual e não é equivalente.

### Decisão proposta

Projetar `MathOverline` sempre como `overline(body: <content>)`. Nesting
recorre ao mesmo formatter e a identidade nunca é inferida de
`Content::Overline`, de `MathUnderline` ou da geometria renderizada.

## P1442 — morfologia pública de `overparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino imprime uma forma opaca `overparen(x)` sem campos; o vanilla
imprime `overparen(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathOverparen` como `overparen(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. O campo só aparece quando
`annotation_explicit` for verdadeiro; seu valor explícito `none` não se
confunde com omissão. Nesting recorre ao mesmo formatter e a identidade nunca
é inferida de U+23DC ou do carrier genérico.

## P1443 — morfologia pública de `overshell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino imprime uma forma opaca `overshell(x)` sem campos; o vanilla
imprime `overshell(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathOvershell` como `overshell(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. O campo só aparece quando
`annotation_explicit` for verdadeiro; seu valor explícito `none` não se
confunde com omissão. Nesting recorre ao mesmo formatter e a identidade nunca
é inferida de U+23E0 ou do carrier genérico.

## P1444 — morfologia pública de `primes` (MATERIALIZADO)

### Medição anterior à decisão

O vanilla projeta `primes(count: n)` inclusive para zero e para contagens sem
codepoint dedicado. O cristalino projeta apenas texto quando a sintaxe existe
e não possui binding qualificado.

### Decisão proposta

Projetar `MathPrimes` sempre como `primes(count: <inteiro>)`. Não substituir
por glyph, string, sequence ou Debug e não inferir count do texto. Dentro de
`MathAttach`, cada slot recorre normalmente ao formatter e conserva nesting.
## P1446 — `repr` de `math.stretch` (PROPOSTA; gate ADR-0127)

Representar `MathStretch` como `stretch(body: <content>)`; quando
`size_explicit`, acrescentar `, size: <relative-length>` depois do body. O
default omitido não fabrica campo, enquanto `size: 100%` explícito permanece
visível. Usar a forma canônica `rel% + abs` já vigente para relative lengths e
a morfologia de content dos carriers math, preservando nesting.

Medição adicional pós-RED: `repr(50% + 0pt)` no vanilla conserva
`50% + 0pt`. Como percentagem literal pura viaja em `Value::Ratio`, todo
`Value::Relative` representa causalmente `Rel<Length>` e imprime sempre as
duas componentes, inclusive absoluto zero. Isto faz fields/dicts manterem o
tipo relativo sem introduzir variante ad hoc.

## P1448 — morfologia pública de `underbrace` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino imprime `xunder("⏟")` ou `xunder("⏟")under(n)`. O vanilla imprime
`underbrace(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathUnderbrace` como `underbrace(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. O campo annotation só aparece quando
`annotation_explicit` for verdadeiro; `none` explícito não se confunde com
omissão. Nesting recorre ao mesmo formatter e identidade nunca é inferida do
glifo U+23DF ou do carrier genérico.

## P1449 — morfologia pública de `underbracket` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino imprime `xunder("⎵")` ou `xunder("⎵")under(n)`. O vanilla imprime
`underbracket(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathUnderbracket` como `underbracket(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem. Annotation aparece somente quando
explicitamente fornecida; nesting recorre ao formatter e identidade nunca é
inferida de U+23B5 ou do carrier genérico.

## P1450 — morfologia pública de `underparen` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino não possui repr próprio; o vanilla imprime
`underparen(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathUnderparen` como `underparen(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem; nesting preserva identidade.

## P1451 — morfologia pública de `undershell` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O cristalino não possui repr próprio; o vanilla imprime
`undershell(body: ...)` e conserva `annotation: none` quando fornecido.

### Decisão proposta

Projetar `MathUndershell` como `undershell(body: <content>, annotation?:
<content-ou-none>)`, nessa ordem; nesting preserva identidade.

## P1466 — filho estruturado de `math.vec`

Quando `MathVec.children` contém uma variante estruturada, o formatter recorre
à repr pública vigente dessa variante. Em particular, `MathFrac` com linha é
`frac(num: [a], denom: [1])`; não restaurar a sintaxe mecânica antiga
`(a)/("1")`. Folhas matemáticas diretas continuam projetadas como conteúdo e
markup não é duplicado.

O teste deve comparar a morfologia canônica completa do vec, incluindo
`strong(body: [b])` e `[quoted]`, sem congelar Debug Rust ou árvore interna.
## P1482 — repr de cor spot (CONFIRMADO em 2026-09-23; ADR-0127)

### Medição anterior à decisão

Vanilla `visualize/color.rs:2300-2303,2320-2323,1399-1415`
compõe a repr de `Color::Spot` a partir do colorant e do tint. Sondas
`repr(color.spot("Brand", red).tint(50%))` dão
`color.spot("Brand", rgb("#ff4136")).tint(50%)`; com `none`,
preservam `color.spot(none, ...)`. No cristalino,
`compiler/eval/repr.rs:329-397` só formata oito variantes de processo.

### Decisão proprietária

Formatar `Color::Spot` com `repr` do `SpotColorant` P1473 e o
percentual canônico de `Ratio`, sem projetar o fallback. `Value::Color`,
`rect(fill:)`, stroke e serialização JSON via eval preservam essa
morfologia; `to-hex()` continua sendo projeção explícita separada.
Nenhum ponteiro, Arc, nome de enum Rust ou byte de PDF entra na repr.
