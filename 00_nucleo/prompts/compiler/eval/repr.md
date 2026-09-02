# Prompt L0 — `compiler/eval/repr` — representação morfológica
Hash do Código: d91654f3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/eval/repr.rs`
**Vanilla ratificado:** `a51e02804`

## Contrato

### P1293 — projeção dos três estados de slot

#### Medição anterior à decisão

`01_core/src/compiler/eval/repr.rs:496-514` hoje omite `None` e converte
`Some(Content::Empty)` em `none`. O recibo causal SHA-256
`80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`
mede que isso torna impossível representar `Present(Content::Empty)` como
markup vazio, embora o vanilla distinga `attach(base:[x], br:none)` de
`attach(base:[x], br:[])`.

#### Decisão

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

## P1293.reopen-B — projeção morfológica de `attach` e `binom`

### Medição anterior à decisão

Em `2026-09-01T15:51:21-03:00`, sobre
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não
commitada discriminada no recibo
`00_nucleo/diagnosticos/p1293-implementation-receipt-b.md` SHA-256
`ae74fa392fcb1b3b93d209347ba211bc207da72fb0ed6a0f1d14c1584937d8f1`,
o implementador executou os cinco REDs próprios do lote B antes de qualquer
implementação produtiva: `0 passed / 5 failed / 5375 filtered`, exit `101`.
As cinco falhas eram exclusivamente bindings ausentes. O consumer
`structural/math.rs`, que transportava os REDs, tinha SHA-256
`07b26199e1fe03f4fcab51591c1ded7da45f462156fcbb8c08bc983dbb446a88`;
o consumer deste owner permanecia baseline, SHA-256
`6582446811b7469bb5e632d1c4f130417d27ad30388138a22027ee3add083eb4`.

Medição direta do baseline, repetida em `2026-09-01T15:55:55-03:00` no
mesmo `HEAD` e estado não commitado: `repr_content` começa em
`01_core/src/compiler/eval/repr.rs:503`; `Content::Sequence` é projetado em
`repr.rs:558-565`; `MathSequence` em `repr.rs:583`; `MathText` em
`repr.rs:584`; `MathFrac` em
`repr.rs:585-587` ignora `line`; `MathAttach` em `repr.rs:588-614` projeta
scripts; e `MathDelimited` em `repr.rs:617-619` projeta apenas delimitador e
body. Logo os owners B já legitimados conseguem construir os payloads
canônicos, mas este owner ainda não consegue observar
`attach(base: ..., t: none)` nem
`binom(upper: ..., lower: (...,))` exigidos pelo contrato congelado.

O baseline do caminho sintático em `HEAD:01_core/src/compiler/eval/math.rs:963-1002`
mostra a forma causal preservada: cada lower ocupa uma posição par de
`Content::MathSequence`; entre lowers, e somente nas posições ímpares, o
constructor insere `Content::MathText(", ")`; o conjunto torna-se `den` de
`MathFrac(line:false)` e é envolvido por `MathDelimited('(', ..., ')')`.
Os owners P1293 de sintaxe e função qualificada obrigam ambos os caminhos a
delegar ao mesmo constructor puro. A conclusão histórica de que `Option` já
distinguia `none` de markup vazio foi refutada pelo recibo causal do carrier;
a decisão triestatal pinada no topo deste prompt substitui somente esse trecho.

### Decisão estreita

`Content::MathAttach` projeta-se como
`attach(base: <repr da base>[, <slot>: <repr>...])`. Somente slots presentes
são emitidos, na ordem pública fixa `t`, `b`, `tl`, `bl`, `tr`, `br`.
`ExplicitNone` projeta-se como `none`; `Omitted` é omitido; e
`Present(content)` usa sua `repr` canônica, inclusive `[]` para conteúdo
vazio. A forma
curta ou multilinha e o escaping pertencem aos formatters canônicos vigentes,
não a concatenação textual ad hoc.

`Content::MathDelimited` projeta-se como `binom` somente quando a estrutura é
exatamente `open == '('`, `close == ')'` e `body == Content::MathFrac` com
`line == false`, cujo `den` é a `MathSequence` canônica de cardinalidade
ímpar não vazia: lowers nas posições pares e exatamente
`Content::MathText(", ")` nas posições ímpares. Essa forma projeta
`binom(upper: <repr num>, lower: (<repr lower 0>, ...))`; singleton conserva a
vírgula final, e múltiplos lowers conservam cardinalidade e ordem. Os
separadores estruturais não viram itens do tuple. A forma fechada é a
construção canônica única autorizada pelos owners de `binom`; a decisão não
reconhece nome de função, texto do upper/lower ou identidade de testemunha.

Qualquer `MathFrac` com `line == true`, qualquer delimitador diferente,
qualquer body que não seja essa fração, e qualquer denominador que não
satisfaça integralmente a alternância estrutural conservam a representação
genérica vigente; não há recuperação permissiva nem heurística nominal.
Sintaxe e chamada qualificada convergem porque chegam ao mesmo payload, não
porque `repr` consulta a origem da chamada.

Esta correção é de morfologia da linguagem (ADR-0107) e corrige owner gap
interno em fluxo contínuo (ADR-0127). Não altera API pública, payload,
`Args`, casts, default, fase de eval/layout, igualdade, hash, traversal ou
render. Refutam a decisão: perda de `none` explícito; emissão de slot omitido;
troca de ordem/cardinalidade dos lowers; barra em binom; classificação de
fração/delimitador genérico como binom; diferença sintaxe/qualificada; ou
qualquer mudança de frame/SVG. Formas fora da estrutura fechada permanecem
`Unknown` para P1293, nunca sucesso implícito.

## P1293.reopen-B-independent-RED — `MathStyled` e folhas diretas

### Medição anterior à decisão

O julgamento independente recebido depois do recibo final B SHA-256
`d677c0b1e6d8796c6680787d27b3409c100ff13653ab8ff89d7154813866720c`
rejeitou a morfologia candidata em dois eixos já congelados pelo contrato:

- `MathStyled` produzido por `mono`/`script` era projetado apenas como `[x]`,
  apagando o wrapper público `styled(child: [x], ..)`;
- nos quatro pares P1293, folhas do caminho sintático apareciam como
  `x`/`T`/equivalentes, enquanto os mesmos campos no caminho qualificado
  apareciam como `[x]`/`[T]`/equivalentes.

O recibo vanilla independente SHA-256
`39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`
mede em `p1293-vanilla-measurement-receipt.md:195-204` que `mono`/`script`
preservam `styled(child: [x], ..)` e que sintaxe/chamada qualificada convergem.
Este owner, em `:248-272`, já reconhece as formas estruturais fechadas de
attach/binom, mas ainda delega cada campo à `repr` genérica sem fixar a projeção
da folha direta. O precedente interno medido em `:84-97` já resolve a mesma
fronteira para filhos diretos de `MathVec`: `MathIdent`/`MathText` usam o
formatter canônico de conteúdo textual sem mudar o payload.

Medido: wrapper e colchetes são morfologia pública; variantes/campos existentes
identificam integralmente os locais. Inferência: a causa cabe somente na
projeção deste owner. Refutam-na necessidade de provenance bit/payload novo,
mudança de entidade/fase/layout, folha aninhada que exija projeção recursiva ou
forma qualificada que continue divergente depois da projeção canônica.

### Decisão estreita

`Content::MathStyled` nas formas estruturais fechadas de `mono`
(`kind == Monospace`) e `script` (`kind == Script`, preservando o `cramped`
presente) mantém obrigatoriamente o constructor público `styled(...)`; nunca
se reduz à `repr` do body. O campo `child` é emitido primeiro pelo formatter
canônico de campos/construtores e os demais campos de estilo presentes
conservam seus nomes, valores e ordem canônicos. A forma observável inclui
`styled(child: [x], ..)` para a testemunha medida; `..` aqui denota os campos
de estilo canônicos presentes, não texto literal nem licença para omiti-los.

Somente nestes campos P1293, uma folha direta `Content::MathIdent` ou
`Content::MathText` projeta a sequência visível pelo formatter canônico de
conteúdo textual, obtendo `[<texto escapado>]` sem concatenar colchetes crus:

- `MathAttach.base` e cada slot presente `t,b,tl,bl,tr,br`;
- `binom.upper` e cada item real de `binom.lower`, excluindo os separadores
  estruturais `MathText(", ")`;
- `MathStyled.child` das formas mono/script acima.

`Content::Text` e qualquer variante estruturada conservam sua própria `repr`;
não há double-wrap, projeção recursiva ou regra para outros campos/elementos.
A seleção usa exclusivamente variantes, campos e valores estruturais
existentes: é proibido reconhecer nome/origem da função, texto da testemunha,
span ou forma sintática/qualificada.

Esta é morfologia da linguagem (ADR-0107) e correção interna de paridade em
fluxo contínuo (ADR-0127), sem novo gate humano. Payload, API pública,
entidade, `Args`, defaults, igualdade/hash/traversal, fase eval/layout e render
permanecem inalterados. Refutam a aceitação: wrapper `styled` apagado,
folha direta sem formatter textual, diferença sintaxe/qualificada, projeção
fora dos campos enumerados ou qualquer alteração de frame/SVG.

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

## P1293.reopen-C — forma curta/multiline de `HtmlElem`

### Medição anterior à decisão

O recibo residual independente P1293/C SHA-256
`4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mede sete positivos com campos, valores e ordem corretos, mas forma linear onde
o vanilla usa multiline. Em `01_core/src/compiler/eval/repr.rs:580-597`, o
`HtmlElem` une campos com `", "`; a disciplina canônica já existe em
`:1087-1140`. Contraprovas curtas permanecem iguais nos dois binários:
`col(span:2,id:"c")`, `wbr(id:"w")` e `button(value:"v")` continuam numa
linha. A divergência é morfologia pública de `repr`, não payload nem layout.

### Decisão estreita

`Content::HtmlElem` usa a mesma disciplina canônica de listas/campos já
aplicada por este owner: forma curta enquanto a representação integral cabe na
fronteira canônica e nenhum campo é multiline; caso contrário, um campo por
linha, indentação por nível e vírgula final. Tag, attrs e body conservam nomes,
valores e ordem; nesting reindenta sem achatar o filho. A decisão não torna todo
HtmlElem multiline e não especializa as sete testemunhas, tags ou quantidade
de atributos.

É correção de morfologia da linguagem em fluxo contínuo ADR-0127. Não altera
entidade, API, default, fase, escaping do target HTML, igualdade ou render.
Refutam-na qualquer forma curta medida que passe a quebrar, perda/reordenação de
campo, escolha por nome de tag ou divergência persistente na mesma largura.
