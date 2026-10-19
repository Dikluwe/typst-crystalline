# F — Inventário 1b: o sistema de estilos do typst VANILLA (upstream)

> Diagnóstico read-only. Descreve **factualmente** como o sistema de estilos
> do typst original (em quarentena, `lab/typst-original/`) está modelado, com
> detalhe suficiente para desenhar opções cristalinas ("o F" / StyleChain) mais
> tarde. **Não copia código.** Toda afirmação carrega `lab/...:linha`.
>
> Critério do dono (registado noutro lado): a fidelidade ao vanilla é
> **COMPORTAMENTAL** (output renderizado + semântica de linguagem), **não**
> estrutural ao nível de Rust. Esta secção de contrato está no fim.

Fonte primária: `lab/typst-original/crates/typst-library/src/foundations/styles.rs`
(1108 linhas). Restantes refs ao longo do texto.

---

## 0. Mapa de ficheiros relevantes

| Assunto | Ficheiro |
|--------|----------|
| `Styles`, `Style`, `Property`, `Recipe`, `StyleChain`, `Fold`, `Resolve`, `NativeRuleMap`, `NativeShowRule` | `lab/typst-original/crates/typst-library/src/foundations/styles.rs` |
| `StyledElem`, `Content::styled*`, `Content::set` | `lab/typst-original/crates/typst-library/src/foundations/content/mod.rs` |
| `Field`, `Settable`, `SettableField`, `SettableProperty`, `RefableProperty`, vtables de campo | `lab/typst-original/crates/typst-library/src/foundations/content/field.rs` |
| `Element::set`, traits `NativeElement`/`Construct`/`Set`/`Synthesize`/`ShowSet` | `lab/typst-original/crates/typst-library/src/foundations/content/element.rs` |
| Avaliação AST de `set`/`show` | `lab/typst-original/crates/typst-eval/src/engine.rs` |
| Realização (consome StyledElem + recipes + show rules) | `lab/typst-original/crates/typst-realize/src/lib.rs` |
| Macro `#[elem]` (gera `set`, `construct`, impls de campo, `#[ghost]`) | `lab/typst-original/crates/typst-macros/src/elem.rs` |
| `Selector::matches` | `lab/typst-original/crates/typst-library/src/foundations/selector.rs` |
| `TextElem` (font/size/weight/... como `#[ghost]`) | `lab/typst-original/crates/typst-library/src/text/mod.rs` |
| `HeadingElem` (numbering settável + Synthesize/ShowSet) | `lab/typst-original/crates/typst-library/src/model/heading.rs` |

---

## 1. A StyleChain real: estruturas de dados

### 1.1 `Styles` — a lista de propriedades (alocada)

`Styles(EcoVec<LazyHash<Style>>)` — `styles.rs:24`. É um `#[ty(cast)]`,
`Default + Clone + PartialEq + Hash` (`styles.rs:22-24`). Operações chave:

- `set(field, value)` (`styles.rs:52-58`): empurra um `Property::new(field, value)`.
- `push(impl Into<Style>)` (`styles.rs:61-63`): adiciona ao fim, embrulhado em `LazyHash`.
- `apply(outer)` (`styles.rs:71-74`): aplica estilos **outer** *in-place* — `outer`
  fica à frente, `self` ao fim (precedência interna vence). `apply_one` insere um
  único estilo na posição 0 (`styles.rs:77-79`).
- `spanned/outside/liftable` (`styles.rs:82-111`): marcam metadados em todas as
  entradas Property/Recipe (span de origem, "fora de show rule", "liftável até a
  página").
- `root(...)` (`styles.rs:140-164`): filtra os estilos partilhados que podem subir
  ao nível raiz (marginais de página, footnotes) — só mantém `outside && (initial
  || liftable)`. **Comportamento relevante:** `set text(red)` antes de `set page`
  faz o footer ficar vermelho; `text(red)[..]` (construtor) não é liftável.

### 1.2 `Style` — entrada única (enum de 3 variantes)

`enum Style { Property(Property), Recipe(Recipe), Revocation(RecipeIndex) }`
(`styles.rs:215-226`). Ou seja, **set rule, show rule e revogação de show rule
partilham o mesmo enum** e a mesma lista. Métodos: `property()`, `recipe()`,
`element()` (a qual elemento o estilo se aplica), `liftable()`, `outside()`
(`styles.rs:228-291`). `Revocation` só é usada para recipes de regex hoje
(`styles.rs:220-225`).

### 1.3 `Property` — uma propriedade de set rule / construtor

Campos (`styles.rs:317-330`): `elem: Element` (a qual elemento pertence),
`id: u8` (índice do campo), `value: Block` (valor type-erased), `span`,
`liftable: bool`, `outside: bool`. `is(elem, id)` / `is_of(elem)` para matching
(`styles.rs:350-357`).

`Block` (`styles.rs:383-436`) é `Box<dyn Blockable>` — armazenamento type-erased
com `dyn_hash`/`dyn_clone`. `Blockable` é auto-derivado para qualquer `T: Any +
Clone + Hash + Debug + Send + Sync` (`styles.rs:417-436`). O hash inclui o
`TypeId` (`styles.rs:429`). Downcast por `TypeId`, com panic se o tipo divergir
(`styles.rs:393-398`, `block_wrong_type` em `styles.rs:974-982`). **Implicação de
desenho:** o valor da propriedade é apagado de tipo e re-tipado por `(Element,
u8)` — não há enum fechado de "todas as propriedades possíveis".

### 1.4 `StyleChain` — a cadeia (não-alocante)

```text
StyleChain<'a> { head: &'a [LazyHash<Style>], tail: Option<&'a Self> }
```
(`styles.rs:563-569`). É uma lista ligada de *links*, cada link uma slice de
estilos. **Não funde eagerly**: cada acesso percorre da camada mais interna para a
mais externa procurando match e fundindo (docstring `styles.rs:556-562`).

- `new(root)` (`styles.rs:573-575`): começa a cadeia com os estilos raiz.
- `chain(local)` (`styles.rs:683-688` + trait `Chainable` `styles.rs:788-822`):
  empurra `local` como primeiro link; `local` tem precedência sobre `self`; em
  propriedades folded `local` contribui o valor *interno*. Slice vazia => devolve
  `*outer` (não cria link) — `styles.rs:802-810`.
- `entries()` (`styles.rs:691-693`, iterador `Entries` `styles.rs:824-845`):
  itera entradas. **Ordem:** dentro de cada link itera `next_back()` (do fim para
  o início), e percorre links do mais interno para o mais externo — ou seja a
  entrada empurrada por último/mais interna vem primeiro. `recipes()` filtra só
  recipes (`styles.rs:696-698`).
- `links()` (`styles.rs:701-703`, iterador `Links` `styles.rs:848-858`): itera os
  links (head, depois tail recursivamente).
- `pop()` (`styles.rs:725-727`): remove o último link.
- `to_map()` (`styles.rs:706-710`): materializa a cadeia numa `Styles` (com
  reverse para repor a ordem natural).
- Igualdade por **ponteiro** (`PartialEq` em `styles.rs:776-785`): duas chains são
  iguais sse `head` e `tail` apontam para a mesma memória. Isto é usado por comemo
  para memoização e por `trunk` (ver abaixo).

### 1.5 Resolução do cascade: get / fold / resolve

A consulta de um campo é feita por **`Field<E, I>`** (zero-sized, `field.rs:18`)
que codifica no sistema de tipos a qual elemento `E` e índice `I` se refere.

- `get_cloned(field)` (`styles.rs:599-610`): se `E::FOLD` existe, chama
  `get_folded`; senão `get_unfolded(...).cloned().unwrap_or_else(E::default)`.
- `get_unfolded` (`styles.rs:645-647`) procura o **primeiro** match na chain
  (`find` -> `properties().next()`, `styles.rs:666-676`) — primeiro = mais interno
  = maior precedência. `properties()` filtra entradas por `(elem, id)`.
- `get_folded` (`styles.rs:651-663`): recolhe **todos** os valores da chain para
  aquele campo (do interno ao externo), faz `reduce(fold)` e depois `fold(folded,
  default)`. Ver `Fold` abaixo.
- `get_ref` (`styles.rs:616-621`): referência sem clone — só para `RefableProperty`
  (não folded).
- `resolve(field)` (`styles.rs:624-633`): `get_cloned` seguido de `.resolve(self)`
  — usa a própria chain para resolver (ex.: comprimentos relativos a em-size).
- Defaults: cada propriedade tem `default()`/`default_ref()` com slot estático
  `OnceLock` (`field.rs:320-334`). Não é estado mutável — é cache lazy de uma
  constante.

### 1.6 `Fold` — fusão de valores acumuláveis

Trait `Fold { fn fold(self, outer: Self) -> Self }` (`styles.rs:890-893`). Deve
ser associativa (`styles.rs:888-889`). Impls: `bool` (interno vence,
`styles.rs:895-899`), `Option<T>` (None interno explícito é respeitado, *não* faz
`or`, `styles.rs:901-910`), `Vec`/`SmallVec`/`OneOrMultiple` (concatenam outer +
inner, `styles.rs:912-931`), `Depth` (soma, `styles.rs:965-972`).
`AlternativeFold::fold_or` (`styles.rs:947-962`): variante onde `None` significa
"não especificado" — um `Some` vence sobre `None`, dois `Some` fundem.
Marcação `#[fold]` na macro liga `with_fold` (`field.rs:266-272`, `field.rs:371-376`).

### 1.7 `Resolve` — resolução contextual

Trait `Resolve { type Output; fn resolve(self, styles) -> Output }`
(`styles.rs:861-867`). Permite que um valor "cru" (ex.: `Em`, comprimento
relativo) seja convertido em valor absoluto usando a própria chain (ex.: font
size corrente). `Option<T>` propaga (`styles.rs:869-875`).

---

## 2. Como o vanilla representa `Styled` / `Set*`

### 2.1 `StyledElem` — o nó portador de estilos na árvore de conteúdo

`StyledElem { child: Content, styles: Styles }` (`content/mod.rs:744-752`), ambos
`#[required]`. É um **elemento de conteúdo** como qualquer outro (gerado por
`#[elem]`). `PartialEq` ignora os styles, compara só `child` (`content/mod.rs:763-767`).

**Como conteúdo recebe estilo** (`content/mod.rs:340-384`):
- `Content::set(field, value)` -> `self.styled(Property::new(field, value))`
  (`content/mod.rs:341-347`).
- `styled(style)` (`content/mod.rs:350-357`): se o conteúdo já é um `StyledElem`,
  faz `apply_one` no seu mapa; senão embrulha num novo `StyledElem`.
- `styled_with_map(styles)` (`content/mod.rs:360-371`): idem para um mapa inteiro;
  fusão por `apply` se já for `StyledElem`.

Ou seja, **`Styled` é um elemento-wrapper na árvore**; os `Set*` são **`Property`
entries dentro de `StyledElem.styles`** — não há tipos `SetHeading`/`SetText`
distintos. A distinção "qual elemento/qual campo" vive nos campos `elem: Element`
e `id: u8` da `Property`.

### 2.2 Avaliação de um `#set` (AST -> Styles)

`impl Eval for ast::SetRule` (`typst-eval/src/engine.rs:11-35`):
1. Avalia condição opcional (`set ... if cond`); se falsa devolve `Styles::new()`
   vazio (`rules.rs:15-19`).
2. Avalia o target, exige que seja uma `Func` que é um element function
   (`rules.rs:21-31`).
3. Avalia args e chama `target.set(&mut engine, args)` (`rules.rs:32-33`),
   marcando `.spanned(span).liftable()` — **set rules são sempre liftáveis**.

`Element::set` (`content/element.rs:65-69`) delega ao vtable `set` (gerado pela
macro) e devolve `Styles`. A função `set` gerada (`macros/elem.rs:598-624`)
constrói uma `Styles` vazia e, para cada campo settável presente nos args, faz
`styles.set(Self::<field>, value)`. Logo um `#set heading(numbering: "1.")`
produz uma `Styles` com **uma** `Property { elem: HeadingElem::ELEM, id: <id de
numbering>, value: "1." }`.

Esse `Styles` é depois aplicado ao "resto" do scope: o avaliador embrulha o
conteúdo seguinte num `StyledElem` com esses estilos (via `styled_with_map`). Na
realização, a chain transporta a `Property` até ao `HeadingElem`.

### 2.3 Como `#set heading(numbering:...)` chega ao heading

`HeadingElem.numbering: Option<Numbering>` é um **campo settável normal**, *sem*
`#[ghost]` e *sem* `#[required]` (`model/heading.rs:134`). Logo:
- Pode vir do construtor (`heading(numbering: ..)[..]`) **ou** de um set rule.
- O acesso `self.numbering.get_ref(styles)` (em `Synthesize`,
  `heading.rs:251-252`) usa `Settable::get_ref` (`field.rs:484-493`): se o campo
  está set na instância usa-o; **senão cai na StyleChain** (`styles.get_ref`).

O caminho concreto na realização:
1. `verdict`/`prepare` correm uma vez por elemento (`realize/lib.rs:419-571`).
2. `prepare` chama `Synthesize::synthesize` com `styles.chain(map)`
   (`lib.rs:549-551`) — heading lê o numbering da chain e gera campos
   sintetizados.
3. `elem.materialize(styles.chain(map))` (`lib.rs:555`) **copia campos da chain
   para dentro do elemento** (vtable `materialize`, `field.rs:298-302`): se o
   campo não está set na instância, escreve o valor da chain. A partir daí o
   elemento "carrega" o numbering.

**Resumo de desenho:** o set não muta o heading diretamente — alimenta a chain;
o heading lê da chain com fallback `instância -> chain -> default`, e a
materialização fixa o valor no elemento antes da layout.

---

## 3. Propriedades de texto: via chain, NÃO via leaf

`TextElem` (`text/mod.rs:94-95`, `#[elem(Debug, Construct, PlainText, Repr)]`).
O **único** campo de dados real é `text: EcoString` (`#[required]`,
`text/mod.rs:753-755`). Praticamente todas as propriedades tipográficas são
`#[ghost]`:

- `font: FontList` (`text/mod.rs:168-170`, default `Libertinus Serif`)
- `style: FontStyle` (`text/mod.rs:210-211`)
- `weight: FontWeight` (`text/mod.rs:231-232`)
- `size: TextSize` (`text/mod.rs:261-264`, `#[fold]`, default 11pt)
- `fill: Paint` (`text/mod.rs:284-286`, default preto)
- `lang: Lang` (`text/mod.rs:438-440`, default inglês)
- `features: FontFeatures` (`text/mod.rs:744-746`, `#[fold]`)
- internos como `delta: WeightDelta` (`#[fold]`, `text/mod.rs:763-767`) e
  `ItalicToggle` (`#[fold]`, `text/mod.rs:769-773`).
- `body: Content` é `#[external] #[required]` (`text/mod.rs:748-751`) — só existe
  na doc/assinatura do construtor.

**O que `#[ghost]` significa** (macro `elem.rs:408-411`, `elem.rs:526-541`): o
campo gera um `impl SettableProperty` mas **não vive na struct do elemento** —
existe **só na StyleChain** (`SettablePropertyData::vtable`). `has` é sempre
`false`, `get`/`get_from_styles` lêem da chain (`field.rs:378-405`). A regra da
macro: "cannot have public ghost fields and an auto-generated constructor"
(`elem.rs:176-181`) — por isso TextElem tem `Construct` manual.

**Conclusão de desenho (crítico para o F):** no vanilla, font/size/weight/fill/lang
**não estão no `TextElem` (a leaf)** — estão na StyleChain como propriedades ghost.
A leaf de texto carrega apenas a string. Toda a tipografia é resolvida no momento
de layout consultando `styles.get(TextElem::font)` etc. Um `#set text(red)`
produz uma `Property { elem: TextElem::ELEM, id: fill, value: red }` na chain; o
`TextElem` em si nunca é tocado.

---

## 4. Show rules & recipes: modelo de dados e transformação

### 4.1 `Recipe`

`Recipe { selector: Option<Selector>, transform: Transformation, span, outside }`
(`styles.rs:445-460`). `selector == None` => show rule sem seletor (`show: rest =>
..`), **aplicado eagerly** ao resto do scope (`styles.rs:448-451`,
`content/mod.rs:321-333`). Com seletor, é guardado na chain como `Style::Recipe`
e aplicado durante a realização.

`Transformation` (`styles.rs:530-538`): `Content(Content)` (substituição),
`Func(Func)` (função aplicada ao match), `Style(Styles)` (**show-set**: aplica
estilos ao conteúdo). `Recipe::apply` (`styles.rs:488-510`) despacha:
- Content -> clona o conteúdo de substituição.
- Func -> chama a função com o match; se há seletor, adiciona tracepoint
  `Show(...)` e `.display()` o resultado.
- Style -> `content.styled_with_map(styles)` (envolve em StyledElem).

### 4.2 Avaliação de um `#show`

`impl Eval for ast::ShowRule` (`typst-eval/src/engine.rs:37-64`): avalia seletor
(`ShowableSelector`), avalia transform — caso especial: `show x: set y(..)` é
detectado e vira `Transformation::Style` (`rules.rs:53-56`, **isto é o show-set**).
Cria `Recipe::new(selector, transform, span)`. Há *guard rails* de migração:
`show page` avisa que não tem efeito (`rules.rs:67-77`); `show par: set block(
spacing)` avisa para usar `set par(spacing)` (`rules.rs:80-95`).

### 4.3 Aplicação na realização

`verdict` (`realize/lib.rs:419-512`) por elemento:
1. Pré-síntese num clone para poder casar `figure.where(kind: table)` antes da
   síntese real (`lib.rs:428-441`).
2. Itera `styles.recipes()` (`lib.rs:449`). Para cada recipe que **casa**
   (`selector.matches(elem, Some(styles))`, `lib.rs:451-456`):
   - Se `Transformation::Style` (show-set) e ainda não preparado -> `map.apply(
     transform)` e continua (acumula estilos, `lib.rs:458-464`).
   - Senão, e ainda não há step: calcula `RecipeIndex(depth - r)`
     (`lib.rs:472`); se o elemento já está *guarded* nesse índice salta
     (`lib.rs:473-475`); senão regista `ShowStep::Recipe(recipe, index)`
     (`lib.rs:478`). Se já preparado, `break`; senão continua a procurar
     show-sets (`lib.rs:483-485`).
3. Se nenhum recipe de utilizador casou, considera a **show rule nativa**
   (`NativeRuleMap::get(target, elem)`, `lib.rs:489-493`).

**Guards (idempotência):** cada elemento regista que recipes já lhe foram
aplicadas (`elem.is_guarded(index)`, `output.guarded(guard)` em `lib.rs:366`).
Isto evita reaplicar a mesma show rule em loop. A indexação é do topo da chain
(`RecipeIndex`, `styles.rs:525-527`; docstring `lib.rs:443-447`).

`visit_show_rules` (`lib.rs:335-415`) executa o step: `recipe.apply` (utilizador,
`lib.rs:361-368`) ou `rule.apply` (nativa, `lib.rs:371-375`). Erros em show rules
**não abortam** — produz conteúdo vazio e acumula erros (`lib.rs:378-385`). Depois
re-visita o conteúdo realizado com o `map` (show-set styles) encadeado
(`lib.rs:404`).

### 4.4 Show rules nativas

`NativeRuleMap` (`styles.rs:985-1059`): `FxHashMap<(Element, Target), NativeShowRule>`.
Há uma rule por combinação elemento×target (`Paged`/`Html`/`Bundle`,
`styles.rs:1010`). `ShowFn<T> = fn(&Packed<T>, &mut Engine, StyleChain) ->
SourceResult<Content>` (`styles.rs:990-994`). `NativeShowRule` é type-erased via
`transmute` seguro (`Packed<T>` é wrapper transparente de `Content`,
`styles.rs:1063-1108`). Regista panics em duplicados (`styles.rs:1038-1046`).

### 4.5 ShowSet / Synthesize nativos

- `ShowSet::show_set(&self, styles) -> Styles` (`element.rs:253-257`): show-set
  **nativo** com acesso aos campos do elemento (mais potente que o do
  utilizador). Aplicado em `prepare` (`lib.rs:542-544`). Ex.: heading
  (`heading.rs:277`).
- `Synthesize::synthesize(&mut self, engine, styles)` (`element.rs:243-247`):
  deriva campos a partir de outros campos/queries, antes de qualquer show rule.
  Ex.: heading resolve `numbering` + location -> número (`heading.rs:237-262`).

---

## 5. Ordem global (pipeline) — visão de desenho

1. **Eval:** `set`/`show` viram `Styles`/`Recipe`; o conteúdo subsequente é
   embrulhado em `StyledElem` (set) ou recebe recipe (show). (`typst-eval/rules.rs`)
2. **Realize/visit:** percorre a árvore com uma `StyleChain` acumulada.
   `StyledElem` empurra um novo link na chain (`visit_styled`, `lib.rs:573+`;
   chamada em `lib.rs:256-258`).
3. **Por elemento:** `verdict` -> `prepare` (location, **show-set**, **synthesize**,
   **materialize**) -> aplica show step -> re-visita. (`lib.rs:241-414`)
4. **Layout/export:** lê propriedades resolvidas da chain
   (`styles.get(TextElem::size)` etc.) por target.

Pontos de desenho a reter para o F:
- A chain é **não-alocante e percorrida por acesso** (lazy), não um mapa fundido.
- Igualdade por ponteiro => crucial para memoização comemo e para `trunk`/`root`.
- Set, show e show-set vivem todos na **mesma lista** (`Style` enum).
- Propriedades são **type-erased por `(Element, u8)`**, não um enum fechado.
- Resolução de campo: **instância -> chain -> default**, com fold opcional.

---

## Contrato de fidelidade de comportamento

O critério do dono: fidelidade é **comportamental** (output renderizado +
semântica de linguagem), **não** estrutura Rust. Qualquer opção cristalina do F
pode reorganizar tipos/camadas livremente desde que preserve os seguintes
comportamentos **observáveis**:

### C1 — Precedência e cascade
- Um valor mais **interno** (mais próximo do elemento) vence o mais externo
  (`get_unfolded` pega o primeiro = mais interno, `styles.rs:645-676`).
- Campo definido no **construtor** da instância vence o set rule (`Settable::
  get_cloned` instância-antes-de-chain, `field.rs:468-480`).
- Ausência em todos -> **default** do campo (`field.rs:320-334`).

### C2 — Fold (acumulação)
- Propriedades `#[fold]` **acumulam** ao longo da chain, não substituem.
  Em particular: `text size` em em-relativo compõe (`text/mod.rs:261-264`);
  `Vec`/features concatenam (`styles.rs:912-931`); `Depth` soma. A fusão é
  associativa e o `default` participa como elo externo (`styles.rs:651-663`).
- `Option` folded: um `None` **explícito** interno é respeitado (não cai para o
  outer) — `styles.rs:901-910`. (Exceto `fold_or`, semântica "unspecified".)

### C3 — Set de numbering propaga (heading/figure/equation)
- `#set heading(numbering: "1.")` deve numerar headings que não têm numbering
  próprio; instância com numbering próprio vence (`heading.rs:134`,
  `field.rs:484-493`). O mesmo padrão settável-com-fallback-na-chain vale para
  figure e equation (campos settáveis equivalentes). Síntese do número corre antes
  das show rules (`heading.rs:237-262`, `lib.rs:549-551`).

### C4 — Show rules
- `show <sel>: <func/content>` transforma elementos que casam o seletor; aplicada
  **uma vez** por elemento (guards de idempotência, `lib.rs:471-475`,
  `lib.rs:366`).
- `show <sel>: set <elem>(..)` (**show-set**) aplica estilos ao alvo
  (`rules.rs:53-56`, `lib.rs:458-464`).
- `show: rest => ..` (sem seletor) aplica eagerly ao resto do scope
  (`content/mod.rs:321-333`).
- Erros em show rules não abortam a compilação; acumulam (`lib.rs:378-385`).
- `show page` não tem efeito (warn) — config de página é via `set page`
  (`rules.rs:67-77`).

### C5 — Propriedades de texto resolvidas da chain
- font/size/weight/style/fill/lang/features de `#set text(...)` afetam **todo** o
  texto no scope, resolvido no layout a partir da chain (não da leaf)
  (`text/mod.rs:168-746`). O resultado renderizado (fonte/tamanho/cor/idioma)
  é o contrato; a leaf carregar só a string é detalhe estrutural livre.

### C6 — Config de página / documento e "lifting"
- `set page(...)` e `set document(...)` aplicam-se ao nível raiz mesmo emitidos
  dentro do fluxo; set rules são **liftáveis** e direct-constructors não
  (`rules.rs:33`, `styles.rs:103-164`, `lib.rs:585-617`).
- `set document(...)` dentro de containers é erro (`lib.rs:589-598`); set de
  formato só no target Bundle (`lib.rs:599-606`).
- O comportamento de "footer fica vermelho com `set text(red); set page(..)`"
  mas não com `text(red)[..]` (`styles.rs:124-139`) é observável e deve manter-se.

### C7 — Resolução contextual (`Resolve`)
- Comprimentos relativos (em, %) resolvem usando a chain corrente (font size,
  etc.) no ponto de acesso (`styles.rs:624-633`, `styles.rs:861-867`). O valor
  absoluto final é o contrato.

### C8 — Idioma / semântica de linguagem
- `set text(lang: ..)` define a locale do documento via primeiro set rule de topo
  (`lib.rs:607-611`) e afeta hyphenation/smartquotes/local names — semântica de
  linguagem que conta como fidelidade comportamental.

---

## Dúvidas (para o dossiê §perguntas)

1. **Granularidade da opção F:** o contrato C1–C8 é puramente comportamental. Até
   que ponto a opção cristalina pode abandonar o modelo "type-erased `(Element,
   u8)` + `Box<dyn Blockable>`" a favor de um enum fechado de propriedades sem
   quebrar C2 (fold genérico sobre tipos arbitrários) e a extensibilidade por
   elementos do utilizador? (vanilla suporta `#[elem]` de 3os; o cristalino tem
   conjunto fechado?)

2. **Igualdade por ponteiro da StyleChain** (`styles.rs:776-785`) é load-bearing
   para memoização comemo e para `trunk`/`root`. Uma opção F com chain
   estruturalmente diferente preserva a *equivalência de memoização* esperada, ou
   isto é detalhe interno livre? (Precisa decisão: comemo é parte do contrato
   comportamental ou só de performance?)

3. **Guards de show rule** (`is_guarded`/`RecipeIndex`, `lib.rs:471-475`):
   indexados a partir do topo de uma chain que "cresce para baixo". É um invariante
   subtil. Qualquer F que não use lista-ligada precisa replicar a semântica de
   "esta recipe já foi aplicada a este elemento" — confirmar que é contrato C4 e
   não só otimização.

4. **`#[ghost]` vs campo settável real:** TextElem usa ghost (só na chain);
   heading.numbering é settável real (instância OU chain). Esta distinção é
   relevante para a opção F? Há comportamento observável que dependa de um campo
   ser ghost (ex.: `heading.numbering` é introspetável como field; `text.size`
   não está na leaf)? Verificar via introspection/query antes de fixar o desenho.

5. **`Revocation`** (`styles.rs:220-225`) só é usado hoje para recipes de regex.
   É contrato comportamental que tem de existir no F, ou pode ser modelado de
   outra forma desde que regex-show-rules revogadas se comportem igual?
