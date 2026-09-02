# P1293 — medição causal independente do carrier de `br: []`

## Estado e limite do recibo

```text
resultado: CAUSA_ISOLADA
primeiro_colapso_da_morfologia_interna_de_[]: eval_markup -> Content::sequence([]) -> Content::Empty
primeira_colisao_semantica_com_none: p1293_math_attach_slot
owner_minimo_do_carrier: entities/elements/math_attach
correcao_somente_em_layout/attach.rs: NAO
gate_ADR-0127: SIM, categoria 1
aprovacao_do_lote_B: NAO CONCEDIDA
```

Esta auditoria é independente, read-only sobre produto/L0/contrato/oracle/
testes/selo/manifesto/gates/veredito. O único write no repositório é este
recibo; probes próprios ficaram sob `/tmp/p1293-empty-markup-carrier-*`.
Foi usado o protocolo completo da skill `tekt-materializacao-segregada` por
segregação de capacidades e artefatos, sem alegação de isolamento técnico do
filesystem compartilhado. Não foram lidos contrato protegido, oracle,
receipts RED/discrimination, testes P1293 ou materialization adicional.

O achado central é mais estreito que “`[]` vira vazio”: o parser, o `Value` e
`Args` ainda distinguem `none` de conteúdo vazio. A colisão nasce somente no
constructor de `math.attach`, quando `Value::None` e
`Value::Content(Content::Empty)` são ambos convertidos em `Content::Empty` e
depois embrulhados em `Some`. A entidade `MathAttachElem` tem apenas
`Option<Content>` e, portanto, não comporta simultaneamente os três estados
observáveis exigidos: omitido, `none` explicitamente fornecido e conteúdo
presente cujo valor é vazio.

## Autoridade e proveniência

- input autorizado `p1293-implementation-receipt-b.md`: SHA-256
  `8752d98afc37119b87033b454fb183eae447b8640f386bc46bc34ec976309919`;
- `01_core/src/compiler/math/layout/attach.rs`: SHA-256
  `36a6699f507b3ddc4f1360c04bcc46c451dd42010f7e9051ed1536e738a7acd4`;
- HEAD/branch: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` / `Tekt`;
- instante final da medição antes da redação:
  `2026-09-01T23:58:54.895401286-03:00`;
- working tree: compartilhada e não commitada;
- SHA-256 do output exato de `git status --short`:
  `dd03dfc7ad1be1ba4277dc70422a2f706ccc5772a0a39660e3d33877ed7394bc`;
- SHA-256 do output exato de `git diff HEAD --stat`:
  `5704686946d2f947c88e3ca97438f95e70086536a097617fe27337c00b9e52e3`;
- diff stat: 50 tracked files, 4.463 inserções e 405 remoções;
- candidato executado `target/release/typst`: SHA-256
  `66f1bfd5494d546d6788d31b3738bbe78e990fb197cfd9834e56d3ee6024862f`;
- vanilla ratificado `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O binário candidato é posterior ao hash registrado no input B, mas o
`attach.rs` auditado continua byte-idêntico ao hash autorizado. Os consumers
relevantes foram registrados individualmente:

| Fonte cristalina | SHA-256 |
|---|---|
| `compiler/parse/code.rs` | `1e07e0815046bc82def55162836fd03c62f9b28bf52fac062bde4a79b6484ea2` |
| `compiler/parse/patterns.rs` | `4188a8a4ca9eb5a81fba3cc1e4c5775e940c61fc6308a4adf467dd04595eeeeb` |
| `entities/ast/expr.rs` | `1b86240612928aa97075b598deafadf3b43df348baabe218ee84a87c65c42bc0` |
| `compiler/eval/mod.rs` | `84888144e8722b93611ee35a3067df1de99a3d22c33f859679e68091a5a396d0` |
| `compiler/eval/call_dispatch.rs` | `4d7002a7475fb8999025fbb28a2381647557ad0617697c52113c34ed9cd1664b` |
| `entities/args.rs` | `c4390cee53010c891656e33dc08159a597f99b10f8f9a1a54e9eb67acd15155a` |
| `entities/value.rs` | `0fdd94c7fff7953f4902f43dc2ede76812d24c5783d489f0470b36a98919542b` |
| `compiler/stdlib/structural/math.rs` | `2ddc0acf10582966dbe8bb9caded4ce273608b433db95cd04efd6440e4bc4540` |
| `entities/elements/math_attach.rs` | `3dc6b20ee1d1e9ad420b78b0eeaabc7688d4f920e4c5923b268c81236f8d3e02` |
| `entities/content.rs` | `f31a13ee93df8039bf8727221c36fa64d50a0715f32814b3a8b74bd7a7a6f974` |
| `compiler/eval/repr.rs` | `8efc40fefab5836603ae757b8ee3a0e00c5d3b53f755703db811f04e39aa045a` |
| `compiler/math/layout/mod.rs` | `f612884fc3f39cdb509fa20e59d7d5f077be33dd49052b34aaf56e08a9d6c81c` |
| `compiler/math/layout/attach.rs` | `36a6699f507b3ddc4f1360c04bcc46c451dd42010f7e9051ed1536e738a7acd4` |

L0s vigentes lidos antes de classificar ownership:

| Prompt L0 | SHA-256 |
|---|---|
| `entities/elements/math_attach.md` | `53c54d6e23bf40497e066daf7a3e8ac9c2a5f86a60010c5df3ef5498ee3695b6` |
| `compiler/stdlib/structural/math.md` | `8a6c9255bd739a9adc710a007a6d99c0feb967d818221ee986dca161e6da04e5` |
| `compiler/math/layout/attach.md` | `24cb3d594438709e759d01cae6b0dc0c7c8162c95f81c78ed4cd094d2d1a0c83` |
| `compiler/eval/repr.md` | `07bdbec8bf68e17ab1fe360559059259e3f4f2731b9a0d15dc29a8b6c511e5de` |

## Probes públicos próprios

Comandos essenciais:

```text
target/release/typst compile <source> <candidate.pdf|svg> [--format svg]
/usr/local/bin/typst compile <source> <vanilla.pdf|svg> [--format svg]
pdftotext -layout <output.pdf> -
rg -o 'viewBox="[^"]+' <output.svg>
sha256sum <sources> <outputs> <binaries>
```

Fontes e outputs de morfologia:

| Artefato | SHA-256 |
|---|---|
| `/tmp/p1293-empty-markup-carrier-values.typ` | `2aa4a280781981023a55db46603839c1faa6f53157100eca41067f0f64daa3cc` |
| values candidato PDF | `9583b8de7f683295e16518e1503bcd82f8c5433de09aeece95d768c251660ca5` |
| values vanilla PDF | `dbe7c2bfaa94dd1a1f554cd1f1475391ff7f8ad5b6dec887e54fb9eb9a52734a` |
| `/tmp/p1293-empty-markup-carrier-raw-values.typ` | `d312701a83816711a1b030b3de3afb96c6c480b9ccbb59bb2dba67470ffb9367` |
| raw-values candidato PDF | `76376434321ae87fb6ba982d56edba691e6ee4d5e1d2ed02b9862797348c1785` |
| raw-values vanilla PDF | `6ab75948ad07af1c4025ae32c5a1dafee3f1bb85bba951d09d8fa22ed10e75c9` |
| `/tmp/p1293-empty-markup-carrier-widths.typ` | `26429e5969eb8c46977c4ce36a3e0201bc9736b78cb004b7f96a76eeabe96a6d` |
| `/tmp/p1293-empty-markup-carrier-field-access.typ` | `700dcf70d2975cd90ce03ce51fc02e36ff5d0516d0472f84aed02c09302bc942` |
| field-access vanilla PDF | `2ed89f99a4ef968d67c484cf0cfae31e703d6420948f8a9d278e52a7d243f309` |

O probe de valores, antes do constructor, produz bilateralmente:

| expressão | tipo de linguagem | candidato `repr` | vanilla `repr` |
|---|---|---|---|
| `none` | `none` | `none` | `none` |
| `[]` | `content` | `[]` | `[]` |
| `""` | `str` | `""` | `""` |
| `[] + []` | `content` | `sequence([], [])` | `[]` |
| `text(fill:red)[]` | `content` | `[]` | `styled(child: [], ..)` |
| `context []` | `content` | `context(...)` | `context()` |
| função que retorna `[]` | `content` | `[]` | `[]` |
| `if false { [] }` | `none` | `none` | `none` |

Portanto `none` e `[]` já são distinguíveis no nível da linguagem e em
`Value`; não é necessário carregar o tipo de AST até layout. A função que
retorna `[]` é a contraprova decisiva contra uma correção baseada em reconhecer
literal `[]` no parser ou no constructor.

O probe `repr(math.attach(...))` localiza a perda:

| slot `br` | candidato | vanilla |
|---|---|---|
| omitido | `attach(base: [x])` | `attach(base: [x])` |
| `none` | `attach(base: [x], br: none)` | igual |
| `[]` | `attach(base: [x], br: none)` | `attach(base: [x], br: [])` |
| `""` | `attach(base: [x], br: [])` | igual |
| `[] + []` | `attach(base: [x], br: sequence([], []))` | `attach(base: [x], br: [])` |
| styled empty | `attach(base: [x], br: [])` | `attach(base: [x], br: styled(child: [], ..))` |
| context empty | `attach(base: [x], br: context(...))` | `attach(base: [x], br: context())` |
| função que retorna `[]` | `attach(base: [x], br: none)` | `attach(base: [x], br: [])` |
| conditional que retorna `none` | `attach(base: [x], br: none)` | igual |

As larguras SVG auto-page são:

| `br` | candidato (pt) | vanilla (pt) | classe |
|---|---:|---:|---|
| omitido | 5.8080 | 5.8080 | ausente |
| `none` | 5.8080 | 5.8080 | ausente |
| `[]` | 5.8080 | 6.4240 | **colisão** |
| `""` | 6.4240 | 6.4240 | presente |
| `[] + []` | 6.4240 | 6.4240 | presente |
| styled empty | 6.4240 | 6.4240 | presente |
| context empty | 6.4240 | 6.4240 | presente |
| função que retorna `[]` | 5.8080 | 6.4240 | **colisão** |
| conditional que retorna `none` | 5.8080 | 5.8080 | ausente |

Todas as alturas são `7.5130pt`. A diferença observada é
`6.4240 - 5.8080 = 0.6160pt`, exatamente a contribuição pública
`SpaceAfterScript` já identificada no input. Os SVGs candidatos ausentes são
byte-idênticos, SHA-256
`391ec89b6fd55f1aa1f323a07be3dff00a431bed2d35bbda91510e12145547ec`;
os candidatos presentes são byte-idênticos, SHA-256
`f57fb6704c2c188055bca2637c92c64f517bda18e41d19f03a0e1cf5f4410abe`.
No vanilla, ausentes são SHA-256
`2021ac0847175d422949b767cca5315468ce2af98a24c5ec731817e9b53d5c1b`
e presentes SHA-256
`4e1cf65687856187a0c06da0b9cc42857ca818dbac9c57af6631e60079ed028e`.

## Trace cristalino, estágio por estágio

### 1. Parser e AST não colapsam os casos

- `compiler/parse/code.rs:107-111` reconhece a chamada e sua lista de args;
- `compiler/parse/patterns.rs:225-253,267-284` cria `Args` e um nó `Named`
  cujo segundo filho é a expressão;
- `compiler/parse/code.rs:253-261` cria `ContentBlock` para `[]`;
- `entities/ast/expr.rs:67,75,142,150` mantém `Expr::None` e
  `Expr::ContentBlock` como variantes distintas;
- `entities/ast/expr.rs:536-547,784-825` conserva nome e expressão em
  `Arg::Named`.

AST relevante: `br:none` chega como `Named(br, Expr::None)` e `br:[]` como
`Named(br, Expr::ContentBlock(empty Markup))`. Não há colisão sintática.

### 2. Eval normaliza markup vazio, mas ainda preserva o tipo do valor

- `compiler/eval/mod.rs:909` avalia `Expr::None` como `Value::None`;
- `compiler/eval/mod.rs:1080-1116` avalia `ContentBlock` via `eval_markup`;
- um body sem filhos deixa `parts=[]` e
  `compiler/eval/mod.rs:831-835` chama `Content::sequence(parts)`;
- `entities/content.rs:2778-2783` normaliza zero partes para
  `Content::Empty`.

Este é o primeiro ponto em que a morfologia interna do literal `[]` é
normalizada a `Content::Empty`, mas o resultado completo continua sendo
`Value::Content(Content::Empty)`, distinto de `Value::None`. Styled empty,
context empty e sequências não canônicas mantêm wrappers estruturais, como
confirmado pelos controles.

### 3. `Args` preserva presença e o `Value`

`compiler/eval/call_dispatch.rs:301-343` insere o resultado avaliado do named
sob a chave `br` sem cast: `none` continua `Value::None`; `[]` continua
`Value::Content(Content::Empty)`. `entities/args.rs:18-28` usa mapa de named e
`entities/value.rs:31-60` tem variantes separadas. O registro de
`math.attach` em `compiler/stdlib/structural/math.rs:1207-1211` e o dispatch
nativo em `compiler/eval/call_dispatch.rs:373-381,1483-1504` também não
colapsam os valores.

### 4. Primeiro ponto da colisão causal

Em `compiler/stdlib/structural/math.rs:637-644`:

```text
Value::None                         -> Content::Empty
Value::Content(Content::Empty)      -> Content::Empty
```

Depois, `:696-703` faz `br = Some(resultado)` para qualquer named presente.
Logo ambos viram exatamente `Some(Content::Empty)`. Este é o primeiro ponto
em que já não existe informação suficiente para reconstruir a distinção.

### 5. A entidade não tem cardinalidade para três estados

`entities/elements/math_attach.rs:17-25` declara os seis slots como
`Option<Content>`. Omissão usa `None`; named presente usa `Some(Content)`.
Como `none` explícito já consome `Some(Content::Empty)`, conteúdo presente
cujo valor é `Content::Empty` colide. `map_content` em `:63-75`, `plain_text`
em `:38-60` e os constructors em `entities/content.rs:1589-1618` propagam
essa representação sem criar a informação ausente.

### 6. `repr` e layout apenas tornam a colisão observável

- `compiler/eval/repr.rs:495-516` projeta todo slot
  `Some(Content::Empty)` como `none`, daí `br: []` virar `br: none`;
- `compiler/math/layout/mod.rs:667-675` passa somente `Option<&Content>` ao
  owner de attach;
- `compiler/math/layout/attach.rs:34-46` filtra todo
  `Some(Content::Empty)` como ausência;
- o mesmo arquivo em `:128-138` só cria MathBox para os slots restantes.

Assim, editar apenas `visible_slot` não pode resolver: fazê-lo preservar
`Content::Empty` corrigiria `[]`, mas faria `none` reservar spacing; filtrá-lo
preserva `none`, mas perde `[]`. Ambos já são o mesmo valor.

## Decalque do vanilla pinado

Hashes de fontes ratificadas relevantes:

| Fonte vanilla | SHA-256 |
|---|---|
| `typst-eval/src/code.rs` | `807f20641f9b3145a46fd160a192c2b83ec2a12bb33a21a519736aad0dd0effc` |
| `typst-eval/src/markup.rs` | `d37c3e17d8654c4ee62fcf2c51fea751344d95ce05eedcbd2d5b4dc7dba0c00e` |
| `typst-eval/src/call.rs` | `cb2fa9dfa313b60d39aae320d90f161685fe9bdfbc5ac7aa71419b36447f2862` |
| `typst-library/foundations/args.rs` | `b681b149809326f2479b99966232680771f8d95a170c82180d30cbd22274849b` |
| `foundations/none.rs` | `4ccf98e03b9fafda26da92c0a9f0b83db78bb9669f1b3b6cb968122d7b8cfb5b` |
| `foundations/content/mod.rs` | `70f2fe3abe9ed674452ab0d26729e76ac3ebef540b41420fb9feb5a1ab6dd0c9` |
| `foundations/content/field.rs` | `69faff47131d47a8b5323326a089bb8e3240c5ae282701e87857c8c050ed8cff` |
| `typst-macros/src/elem.rs` | `62dcaa15c4f09a6d00c4249c4b0d6c930f51fa56f72c1be8433b674a6014ede9` |
| `math/attach.rs` | `7efdda527260ad0ac91436b35abe5f98b85019ff8491192c1ed5077c17953f5f` |
| `math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| `typst-layout/math/scripts.rs` | `d3f8a9fc8a4f58eafdc3edbac9cdb67279ff0f023dd9b4967f209e81165f286b` |

O vanilla também avalia `[]` como `Content::empty`: `typst-eval/code.rs:328-336`
delega o content block; `typst-eval/markup.rs:17-29,86` chama
`Content::sequence`; `foundations/content/mod.rs:92-95,238-247` representa o
vazio como um `SequenceElem` vazio. Portanto a diferença não é “vanilla não
colapsa markup vazio”.

A diferença real é o carrier de campo gerado:

1. `typst-eval/call.rs:412-460` e
   `typst-library/foundations/args.rs:512-520` mantêm cada `Arg` com nome e
   `Spanned<Value>`;
2. `math/attach.rs:19-49` declara semanticamente cada slot como
   `Option<Content>`;
3. o macro `elem` transforma um campo opcional em
   `Settable<AttachElem, I>` (`typst-macros/elem.rs:297-306`);
4. o constructor usa `args.named()` para o campo opcional
   (`typst-macros/elem.rs:638-655`) e embrulha o resultado no `Settable`
   (`:574-605`);
5. `FromValue for Option<T>` mapeia `Value::None -> None` e conteúdo para
   `Some(content)` (`foundations/none.rs:107-114`);
6. `Settable` acrescenta o eixo unset/present e guarda internamente outro
   `Option<E::Type>` (`foundations/content/field.rs:439-481`).

Para `AttachElem.br`, `E::Type` já é `Option<Content>`. A cardinalidade física
efetiva é, portanto, `Option<Option<Content>>`:

| linguagem | `Settable` externo | valor semântico interno |
|---|---|---|
| omitido | `None` (unset) | default `None` |
| `br:none` | `Some(None)` | attachment ausente |
| `br:[]` | `Some(Some(Content::empty()))` | attachment presente, caixa zero |

`SettableFieldData::vtable` usa `is_set` e expõe só campos explicitamente
presentes (`field.rs:289-319`), razão pela qual o `repr` genérico conserva
`br:none` mas omite o default. `resolve_inner_attach` lê o valor semântico
resolvido em `math/ir/resolve.rs:486-504`: `none` chega `None`, enquanto `[]`
chega `Some(Content::empty())`. Em `:554-579`, somente o segundo produz um
`Some(MathItem)` zero-size. `typst-layout/math/scripts.rs:149-177` soma
`SpaceAfterScript` aos post-scripts presentes e calcula a largura por máximo.

`Smart<Content>` **não** participa de `AttachElem`; o auditado
`foundations/auto.rs:65-73` é outro eixo (`auto` versus custom). `Packed<T>`
também não cria presença: `foundations/content/packed.rs:14-18` é apenas o
wrapper type-erased do elemento que já contém os `Settable`. A distinção
relevante pertence ao `Settable` externo mais o `Option<Content>` interno.

## Classificação ADR-0107/0108 e owner

A diferença omitido/`none`/conteúdo vazio e sua projeção em `repr` é
morfologia da linguagem; a reserva de `SpaceAfterScript` é geometria
observável. `Option<Option<Content>>`, `Settable` e a posição do cast são
mecânica e não precisam ser copiadas literalmente, mas qualquer mecânica
escolhida precisa comportar os mesmos três estados sem inferi-los de largura,
tinta, texto, nome, AST ou constante de fixture.

O menor owner L0 1:1 que pode possuir o carrier é:

```text
00_nucleo/prompts/entities/elements/math_attach.md
  -> 01_core/src/entities/elements/math_attach.rs
```

Esse owner atualmente fixa publicamente `Option<Content>` e a codificação
`Some(Content::Empty) == none explícito`; logo está contradito pela medição.
Também estão contraditas as alegações de `layout/attach.md` de que há um
“carrier estrutural já existente” e de que nenhum campo/payload é necessário,
e a premissa de `eval/repr.md` de que a entidade já distingue os casos. O L0
de `stdlib/structural/math.md` repete a codificação insuficiente.

Uma solução sem sentinel ad hoc exige um carrier tipado de três estados no
payload da entidade — mecanicamente, um campo externo de presença sobre um
valor opcional, ou um tipo fechado equivalente. Isso muda tipo/campo público
de entidade/payload e as signatures dos constructors que hoje recebem
`Option<Content>`. É, portanto, **ADR-0127 categoria 1** e exige reabertura de
L0, resselo e confirmação humana antes de código. Não é correção interna que
possa continuar somente em `layout/attach.rs`.

Opções medidas:

- **carrier específico de `MathAttachElem`**: menor superfície; preserva os
  três estados na entidade e deixa constructor, repr e layout consumirem
  projeções tipadas. É o menor owner existente;
- **infraestrutura genérica equivalente ao `Settable<T>` do vanilla**:
  coerente para múltiplos elementos/defaults explícitos, mas muito mais ampla
  que o defeito medido; exigiria owner próprio e inventário de todos os
  consumers antes de adoção;
- **nova distinção global em `Content` para vazio sintático**: alarga contrato
  público, igualdade/hash/traversal e todos os matches exaustivos; não é o
  menor owner e altera compatibilidade/morfologia geral;
- **carregar AST/proveniência em `Args`**: desnecessário e refutado pela função
  que retorna `[]`; o `Value` já distingue conteúdo de `none`;
- **usar `Content::Sequence([])`, `Styled(Empty)`, bit nominal ou outra forma
  impossível como sentinel**: rejeitado por ser codificação ad hoc que pode
  colidir com conteúdo legítimo e não tem owner semântico.

## Consumers que uma correção tipada teria de coordenar

No mínimo, sem autorizar escrita:

1. `entities/elements/math_attach.rs`: definição, traversal e texto;
2. `entities/content.rs`: constructors públicos e reconstruções;
3. `compiler/stdlib/structural/math.rs`: cast dos named e construção;
4. `compiler/eval/math.rs`: attachments produzidos pela sintaxe;
5. `compiler/eval/repr.rs`: presença externa versus valor interno;
6. `compiler/math/layout/mod.rs` e `compiler/math/layout/attach.rs`: projeção
   semântica para `Option<&Content>` e spacing por presença;
7. as duas reconstruções recursivas de math style/default em
   `compiler/math/layout/mod.rs:2029-2037,2242-2250`.

`compiler/eval/bindings/field_access.rs` atualmente nem expõe `br` para
`MathAttach` no candidato: o probe público acima falha com “math.attach does
not have field br”. O mesmo source no vanilla imprime `none | br=none |
type=type(none)` e `markup | br=[] | type=content`. Isso é uma lacuna pública
adjacente, mas não é usado para explicar a colisão nem autorizado a entrar
silenciosamente nesta correção; precisa de medição/L0 próprios se for incluído.

Como a mesma invariante triestatal precisa ser consumida pelos owners de
entidade, constructor, repr e layout, ADR-0129 impede copiá-la normativamente
em quatro prompts independentes. O carrier produtivo continua pertencendo ao
L0 da entidade; um Núcleo Tekt dedicado à semântica compartilhada de presença
seria necessário se a obrigação for pinada por esses múltiplos prompts. O
Núcleo não legitimaria código nem substituiria os owners 1:1. Uma alternativa
genérica `Settable<T>` compartilhada por outros elementos reforçaria, não
eliminaria, essa necessidade e exigiria inventário/DAG/V26 antes do resselo.

## Hipótese causal e refutadores

Hipótese confirmada: a falha de `br: []` é causada exclusivamente pela perda
do eixo externo `argumento presente` versus valor interno opcional no
constructor `p1293_math_attach_slot`; `visible_slot` apenas recebe o resultado
já colidido.

Refutariam esta conclusão:

- um dump reproduzível de `Args` mostrando `none` e `[]` iguais antes de
  `p1293_math_attach_slot`;
- `br: []` continuar sem spacing depois de um carrier tipado preservar
  explicitamente `present-content(Content::Empty)` até `layout_attach`;
- `br:none` reservar spacing quando projetado semanticamente como ausência;
- vanilla resolver `br:[]` como `None`, ou não aplicar `SpaceAfterScript` a
  um `Some` zero-size;
- função que retorna conteúdo vazio comportar-se como `none` no vanilla;
- necessidade medida de alterar parser, fase eval/layout, fonte ou constante.

Nenhum refutador ocorreu. A causa está isolada, mas o lote continua sem
aprovação porque a correção exige reabertura pública/gate humano e ainda não
foi implementada ou verificada.

## Working tree observado

O `git diff HEAD --stat` observado foi:

```text
00_nucleo/prompts/compiler/eval.md                 |  46 +-
.../prompts/compiler/eval/bindings/field_access.md |  41 +-
00_nucleo/prompts/compiler/eval/call_dispatch.md   | 154 +++++-
00_nucleo/prompts/compiler/eval/math.md            |  40 +-
00_nucleo/prompts/compiler/eval/repr.md            | 142 +++++-
00_nucleo/prompts/compiler/eval/tests.md           |  50 +-
00_nucleo/prompts/compiler/layout/equation.md      | 114 ++++-
00_nucleo/prompts/compiler/layout/helpers.md       |  67 ++-
00_nucleo/prompts/compiler/layout/text.md          |  35 +-
00_nucleo/prompts/compiler/math/layout/_comum.md   | 373 +++++++++++++-
00_nucleo/prompts/compiler/math/layout/attach.md   | 318 +++++++++++-
00_nucleo/prompts/compiler/stdlib/foundations/float.md | 83 ++-
00_nucleo/prompts/compiler/stdlib/html.md           | 95 +++-
00_nucleo/prompts/compiler/stdlib/math_style.md     | 106 +++-
00_nucleo/prompts/compiler/stdlib/structural/math.md | 95 +++-
00_nucleo/prompts/entities/elements/math_attach.md |  52 +-
00_nucleo/prompts/entities/layout_types.md         | 156 +++++-
00_nucleo/prompts/entities/style_chain.md          |  37 ++
00_nucleo/prompts/infra/font_metrics.md             | 115 ++++-
00_nucleo/prompts/infra/shaper.md                   | 59 +++
01_core/src/compiler/eval/bindings/field_access.rs |  11 +-
01_core/src/compiler/eval/call_dispatch.rs         | 262 +++++++++-
01_core/src/compiler/eval/math.rs                  | 204 +++-----
01_core/src/compiler/eval/mod.rs                   |   2 +-
01_core/src/compiler/eval/repr.rs                  | 247 +++++++--
01_core/src/compiler/eval/tests.rs                 |  10 +-
01_core/src/compiler/layout/equation.rs            | 179 ++++++-
01_core/src/compiler/layout/helpers.rs             |  64 ++-
01_core/src/compiler/layout/text.rs                |   5 +-
01_core/src/compiler/math/layout/accent.rs         |   2 +-
01_core/src/compiler/math/layout/attach.rs         | 259 +++++++++-
01_core/src/compiler/math/layout/cancel.rs         |   2 +-
01_core/src/compiler/math/layout/cases.rs          |   2 +-
01_core/src/compiler/math/layout/frac.rs           |   2 +-
01_core/src/compiler/math/layout/matrix.rs         |   2 +-
01_core/src/compiler/math/layout/mod.rs            | 272 +++++++++-
01_core/src/compiler/math/layout/root.rs           |   2 +-
01_core/src/compiler/math/layout/spacing.rs        |   2 +-
01_core/src/compiler/math/layout/tests.rs          | 129 ++---
01_core/src/compiler/math/layout/underover.rs      |   2 +-
01_core/src/compiler/math/layout/vec.rs            |   2 +-
01_core/src/compiler/stdlib/foundations/float.rs   | 235 ++++++++-
01_core/src/compiler/stdlib/html.rs                |   2 +-
01_core/src/compiler/stdlib/math_style.rs          |  54 +-
01_core/src/compiler/stdlib/structural/math.rs     | 557 ++++++++++++++++++++-
01_core/src/entities/elements/math_attach.rs       |   2 +-
01_core/src/entities/layout_types.rs               |  18 +-
01_core/src/entities/style_chain.rs                |   6 +-
03_infra/src/font_metrics.rs                       |  99 +++-
03_infra/src/shaper.rs                             |  55 +-
50 files changed, 4463 insertions(+), 405 deletions(-)
```

Os untracked observados eram o arquivo literal `-`, os artefatos públicos
P1293 já presentes sob `00_nucleo/diagnosticos/`, o passo explicitamente não
lido `00_nucleo/materialization/typst-passo-1293.md`, o L0 de contrato e o
teste protegido. Nenhum deles foi alterado por esta auditoria.

## SHA do recibo

O SHA-256 final do arquivo completo é calculado e reportado na entrega; ele
não é auto-incluído porque isso tornaria o digest recursivo.
