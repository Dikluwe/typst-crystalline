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
