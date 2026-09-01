# Prompt L0 — `compiler/eval/repr` — representação morfológica
Hash do Código: 19c40081


**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/eval/repr.rs`
**Vanilla ratificado:** `a51e02804`

## Contrato

Produzir `repr` exaustivo e determinístico para `Value`, `Content` e
`Selector`, preservando a morfologia pública da linguagem, escaping e formas
constructoras. A aceitação é textual/linguística; a estrutura Rust é mecânica.

Labels usam `<nome>` somente quando o nome satisfaz a gramática literal; caso
contrário usam `label(<repr string>)`. Raiz matemática sem índice usa
`root(radicand: <repr>)`; com índice conserva `root(<index>, <radicand>)` até
medição específica. Symbols multi-codepoint preservam todos os scalar values,
incluindo variation selectors e ZWJ, sem conversão para `char`, normalização ou
substituição pelo nome canônico.

## P1292 — morfologia de cancel/underline/vec/flush

### Medição anterior à decisão

O recibo vanilla P1292 mediu defaults e formas explícitas. Medição adicional
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
`/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
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
`/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O cristalino foi compilado dessa árvore, sem reutilizar binário antigo, por
`CARGO_TARGET_DIR=/tmp/p1290-contract-target cargo build --release --bin typst`;
o binário resultante tem SHA-256
`acc52526e1c1cfde21c4583857330a6f64540f46caf1b7fa89c666da6deee64a`
e reporta `typst 0.15.1 (53d21c5a)`.

Os casos reproduzíveis estão em
`00_nucleo/diagnosticos/p1290-contract-probes.typ:1`. O comando bilateral é
`<binario> query 00_nucleo/diagnosticos/p1290-contract-probes.typ '<p1290-contract>' --field value --one --pretty`.
Em `p1290-contract-probes.typ:4`, o vanilla mede `()`, `(0,)` e
`(0, 1, 2)` para vazio, singleton e curto. Em
`p1290-contract-probes.typ:8`, corpos internos ASCII de 49 e 50 caracteres
ficam lineares, enquanto 51 quebra. As sequências de zeros confirmam a mesma
fronteira por largura, não por cardinalidade: 17 itens produzem corpo de 49 e
ficam lineares; 18 produzem 52 e quebram. Em
`p1290-contract-probes.typ:13`, um item já multilinha força o container externo
a quebrar; cada nível acrescenta dois espaços, cada item termina em vírgula e
o parêntese final volta à indentação do seu container.

Em `p1290-contract-probes.typ:17`, o vanilla mede
`math.csc`, `math.dim` e `math.arcsin` como, respectivamente,
`op(text: [csc], limits: false)`, `op(text: [dim], limits: false)` e
`op(text: [arcsin], limits: false)`; `math.lim` mede
`op(text: [lim], limits: true)`. `math.op("foo", limits: false)` e
`math.op("foo", limits: true)` conservam o booleano no mesmo segundo campo.
Conteúdo com markup mede forma multilinha com `text:` antes de `limits:`;
aspas e barra invertida em string permanecem na morfologia do conteúdo.
`math.sum` mede `symbol("∑", ("integral", "⨋"))`, portanto não é testemunha
de `MathOp`.

A invalidação registrada em
`00_nucleo/diagnosticos/p1290-seal-invalidation.json:1`, SHA-256
`c2e81e5db2810d720d80e2340cc79a600b12c3eea3725d20f342edd3e2d03d0e`,
refutou o caso anterior `math.op([a \# "quote"], ...)`: o markup cristalino
original já entrega aspas curvas como `Content::Text`, sem a proveniência
`SmartQuote` que o vanilla ainda representa. Em
`2026-08-31T11:43:44-03:00`, usando os mesmos binários congelados, o probe
substituto `math.op([a \# \*], limits: false)` mediu no vanilla
`op(\n  text: sequence([a], [ ], [#], [ ], [*]),\n  limits: false,\n)` e no
cristalino original `op(sequence([a], [ ], [#], [ ], [*]))`. Assim, `\#` e
`\*` observam escaping sem introduzir proveniência destruída; a divergência
restante pertence integralmente aos nomes/ordem dos campos deste owner. O
comando bilateral permanece o documentado acima após substituir o caso em
`p1290-contract-probes.typ:27`.

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

Arrays/tuplas serializam cada item pela sua própria `repr` e usam `", "`
entre itens no modo linear. Vazio é `()`; singleton conserva a vírgula
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

## Restrições e aceitação

P1224: `Stroke` simples conserva a forma histórica `thickness + paint`.
Quando qualquer dimensão complexa diverge do default, usa dict morfológico com
campos explícitos `paint`, `thickness` quando não-default, `cap`, `join`,
`dash: (array:, phase:)` e `miter-limit`. A ordem do dash é preservada;
`DashLength::LineWidth` usa `"dot"`. Não usar `Debug` dos enums.

P1225 permite que o módulo pai exponha o formatter de `Stroke` por uma função
pública estreita para o serializer L2. P1285 mede que o contrato de
serialização do vanilla usa `repr` para todo `Value` não estruturado
(`lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362`)
e, conforme gate ADR-0127 confirmado pelo dono em 2026-08-30, autoriza o módulo pai a expor uma
fachada pública total `repr_value_for_serialization(&Value) -> String`.
`repr_value` permanece implementado neste owner e o módulo continua privado;
a fachada não autoriza `Debug`, serialização em L1 nem dependência de formato.

L1 puro, sem I/O. Toda nova variante pública exige medição anterior contra o
vanilla e branch explícito; não usar `Debug` como fallback. Testes focais
cobrem escaping, labels, conteúdo matemático, selectors e symbols.

O constructor nativo `native_repr` pertence a
`compiler/stdlib/foundations/repr.md`; este owner especifica a serialização
efetiva chamada por ele.
