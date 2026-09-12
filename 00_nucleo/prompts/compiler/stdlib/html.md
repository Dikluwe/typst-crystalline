# Prompt L0 — módulo público `html` feature-gated
Hash do Código: 3c9cdb86

**Estado:** APROVADO NO GATE ADR-0127 EM 2026-08-25  
**Camada:** L1  
**Ficheiro proprietário:** `01_core/src/compiler/stdlib/html.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0128

## Medição anterior à decisão

No vanilla ratificado `a51e02804`, `typst-library/src/lib.rs:369` registra
`html` com `Feature::Html`; feature off, acessar o nome produz diagnóstico
específico e hints; feature on, `type(html)` é `module`. Em
`typst-html/src/lib.rs:34-41`, o módulo contém `elem`, `frame` e as funções
tipadas registradas por `typed.rs:31-43`. A tabela pinada contém 112 tags. O
cristalino não registra `html` em `eval/mod.rs:1391-1844`.

## Contrato do primeiro corte (P1166, incompleto)

Com `Feature::Html` ausente, o binding existe como definição gated, mas seu
acesso falha com o diagnóstico vanilla medido; não cai em `unknown variable`.
Com a feature presente, `html` é `Value::Module` e expõe inicialmente apenas:

```typst
html.elem(tag, attrs: (:), body: none)
```

`tag` é string posicional obrigatória e valida a gramática de nome HTML;
`attrs` é dictionary named opcional de nomes válidos para strings; body é
content posicional opcional. O resultado é conteúdo com morfologia observável
`elem(tag: ..., attrs: ..., body: ...)`, preservando unset de attrs/body.
Tag/atributo inválido é erro no argumento. A sonda ratificada de P1165 criou
`html.elem` em `eval` com target paged por default; portanto a construção do
valor não é restringida ao target HTML. O consumo/export é específico do
target.

O módulo permanece deliberadamente incompleto. P1167 inicia tags tipadas;
P1168+ completa as 112 assinaturas; passo posterior implementa `html.frame`.
Não criar aliases silenciosos nem expor a tabela incompleta como paridade.

## Invariantes

- L1 não lê CLI, env nem filesystem; recebe features por valor/contrato puro.
- Selecionar formato/target HTML não habilita a feature.
- A tabela de tags/atributos é dado de linguagem, não import de `lab`.
- O dispatcher permanece estático; nenhuma vtable é necessária.

## Critérios de língua do primeiro corte

- feature off: `type(html)` falha com diagnóstico e hints medidos;
- feature on: `type(html) == module`;
- `repr(html.elem("article", attrs: (lang: "pt"))[Olá])` coincide;
- tag/atributo inválido coincide em classe, span e mensagem medida;
- output HTML preserva tag, atributos escapados e body sem passar por layout
  paginado;
- `html.frame` e tags tipadas continuam ausentes e declaradas como scope-out.

## P1167 — lote tipado global-only (APROVADO NO GATE ADR-0127 EM 2026-08-25)

### Medição

No pin ratificado, `typed.rs:74-114` dá a cada constructor todos os atributos
da entrada e um body `content` posicional opcional quando a tag não é void.
`typst-assets/src/html.rs:25-32` concatena a cada entrada os primeiros 76 attrs
de `data.rs`, declarados globais em `data.rs:1702-1703`. As entradas medidas
de `div`, `span`, `p`, `h1..h6`, `strong`, `em` e `ul` têm lista específica
vazia e não são void/raw. Sondas ratificadas confirmaram `function`, repr
`elem(tag: "TAG", body: none)`, body content, casts estritos, erro de named
desconhecido e DOM aninhado.

### Contrato público

Sob `Feature::Html`, acrescentar exatamente:

```text
html.div html.span html.p html.h1 html.h2 html.h3 html.h4 html.h5 html.h6
html.strong html.em html.ul
```

Cada função retorna content, não é settable/contextual, aceita `body: content`
posicional opcional e os seguintes named opcionais. A lista abaixo é a
assinatura integral; não aceitar `data-*` nem qualquer named fora dela.

- `str`: `aria-activedescendant`, `aria-details`, `aria-errormessage`,
  `aria-keyshortcuts`, `aria-label`, `aria-placeholder`,
  `aria-roledescription`, `aria-valuetext`, `id`, `is`, `itemid`, `nonce`,
  `slot`, `style`, `title`.
- `int`: `aria-colcount`, `aria-colindex`, `aria-colspan`, `aria-level`,
  `aria-posinset`, `aria-rowcount`, `aria-rowindex`, `aria-rowspan`,
  `aria-setsize`, `tabindex`.
- `float`: `aria-valuemax`, `aria-valuemin`, `aria-valuenow`.
- booleano serializado `true|false`: `aria-atomic`, `aria-busy`,
  `aria-disabled`, `aria-modal`, `aria-multiline`, `aria-multiselectable`,
  `aria-readonly`, `aria-required`, `draggable`, `spellcheck`,
  `writingsuggestions`.
- Presence (`true` produz atributo vazio; `false` omite): `autofocus`,
  `inert`, `itemscope`.
- listas separadas por espaço, aceitando também shorthand escalar:
  `accesskey: char`, `aria-controls: str`, `aria-describedby: str`,
  `aria-flowto: str`, `aria-labelledby: str`, `aria-owns: str`, `class: str`,
  `itemprop: str`, `itemref: str`, `itemtype: str`; `aria-relevant` restringe
  itens a `additions|additions text|all|removals|text`.
- unions boolean/string: `aria-checked` e `aria-pressed` aceitam boolean ou
  `mixed`; `aria-expanded`, `aria-hidden` e `aria-selected` aceitam boolean
  ou `none` (serializado `undefined`).
- enums/unions ARIA: `aria-autocomplete: none|inline|list|both`;
  `aria-current: bool|page|step|location|date|time`; `aria-haspopup:
  bool|menu|listbox|tree|grid|dialog`; `aria-invalid: bool|grammar|spelling`;
  `aria-live: assertive|off|polite`; `aria-orientation:
  horizontal|undefined|vertical`; `aria-sort:
  none|ascending|descending|other`.
- globais restantes: `autocapitalize: on|off|none|sentences|words|characters`;
  `autocorrect: on|off`; `contenteditable: bool|plaintext-only`; `dir:
  ltr|rtl|auto`; `enterkeyhint: enter|done|go|next|previous|search|send`;
  `hidden: Presence|until-found`; `inputmode:
  none|text|tel|email|url|numeric|decimal|search`; `lang: str|none` (none vira
  string vazia); `popover: auto|manual`; `role: none` ou um dos 80 tokens
  ARIA da faixa pinada `ATTR_STRINGS[237..317]`; `translate: bool` serializado
  `yes|no`.

Unset não cria atributo. A ordem no `repr`/DOM segue a ordem fornecida pelos
argumentos depois da conversão. Cast inválido mantém a classe de diagnóstico
vanilla; named desconhecido é `unexpected argument: NAME`. Chamada sem body
preserva `body: none`; body fornecido preserva conteúdo, inclusive HtmlElem
aninhado. O resultado usa a mesma morfologia `elem` de `html.elem`.

### Divisão explícita

P1168 completa somente estes 12 bindings e a infraestrutura reutilizável dos
76 globais. P1169 mede/materializa `ol` e `li`; P1170, `a`; P1171 introduz a
tabela void e `br`. As demais tags e `html.frame` continuam incompletas. Esta
secção foi aprovada pelo dono em 2026-08-25 e aguarda resselo no início de
P1168.

### Correção descoberta durante P1168 — gate aprovado em 2026-08-25

A sonda ratificada distinguiu `html.div()` (`body: none`) de
`html.elem("div")` (body unset). Até aprovação e materialização do estado
triádico definido em `entities/html.md`. O dono aprovou a alteração pública e
P1168 materializou-a antes do GREEN integral. Os 12 bindings e os 76 casts
continuam dentro do primeiro gate aprovado.

## P1169 — listas ordenadas tipadas (APROVADO E MATERIALIZADO EM 2026-08-25)

### Medição

No `typst-assets` pinado, `data.rs:302-304` associa `li` somente ao atributo
220 (`value: Int`) e `:357-359` associa `ol` aos atributos 181/201/214
(`reversed: Presence`, `start: Int`, `type: Strings(229,234)`). A faixa de
strings contém exatamente `"1"`, `"a"`, `"A"`, `"i"`, `"I"`.
`typed.rs:73-114` acrescenta os 76 globais e body content opcional porque
ambas as tags são normais, não void/raw.

Sondas no vanilla ratificado confirmaram:

- `type(html.ol) == function` e `type(html.li) == function`;
- chamadas vazias preservam `body: none`;
- `reversed: true` vira valor vazio e `false` omite o atributo;
- `start` e `value` aceitam `int` irrestrito, incluindo zero/negativos, e o
  decimalizam;
- `type` aceita somente os cinco tokens pinados;
- atributos específicos da outra tag, `data-*` e named desconhecido são
  `unexpected argument`;
- nesting produz `ol`/`li` explícitos sem wrapper adicional.

### Contrato público

Sob `Feature::Html`, acrescentar exatamente:

```typst
html.ol(
  reversed: bool = false,
  start: int,
  type: "1" | "a" | "A" | "i" | "I",
  ..76 atributos globais P1168,
  body: content,
)

html.li(
  value: int,
  ..76 atributos globais P1168,
  body: content,
)
```

Todos os named são opcionais/unset; a notação `= false` acima descreve apenas
a ausência material de Presence quando omitido/false, não um valor armazenado
nem parâmetro obrigatório. Body é posicional opcional e, quando omitido,
materializa `HtmlBody::None` e repr `body: none`.

`ol.reversed=true` produz par vazio, repr `reversed: ""` e DOM `reversed`;
false/omitido não cria par. `start`/`value` usam todo `i64` Typst, sem impor
positividade HTML. A ordem de attrs no repr/DOM segue a ordem dos named após
omissões. Casts inválidos usam as classes vanilla: `expected boolean`,
`expected integer` ou `expected "1", "a", "A", "i", or "I"`; valor de tipo
errado acrescenta `found TYPE`. Named restante é `unexpected argument: NAME`.

Implementação deve estender os metadados estáticos existentes com atributos
específicos por tag; não duplicar os 76 globais, não aceitar fallback string e
não criar branches no exporter.

### Divisão explícita

P1169.1 completou somente `ol` e `li`. `a` permanece P1170; `br`, tabela void
e whitespace adjacente permanecem P1171. Todas as outras tags, raw, frame,
CSS, MathML e positions continuam incompletas. Esta secção foi ressellada e
materializada somente em P1169.1. O dono aprovou exatamente esse contrato em
2026-08-25.

## P1170 — anchor tipado (APROVADO E MATERIALIZADO EM 2026-08-25)

### Medição

O lockfile do vanilla ratificado fixa `typst-assets` em `94dcb99`
(`lab/typst-original/Cargo.lock:3120-3123`). O inventário local P1140.26 mede
`html.a` como função e enumera oito named específicos antes dos 76 globais:
`download`, `href`, `hreflang`, `ping`, `referrerpolicy`, `rel`, `target` e
`type`. `typed.rs:73-114` acrescenta os atributos da entrada e body content
posicional opcional a tags normais.

Sondas no binário vanilla ratificado `a51e02804` mediram:

- `download`, `href`, `hreflang` e `type`: string livre;
- `ping`: string ou array de strings, serializado com espaço; item de array
  não pode conter espaço;
- `referrerpolicy`: `none` (serializa string vazia) ou exatamente
  `no-referrer`, `no-referrer-when-downgrade`, `same-origin`, `origin`,
  `strict-origin`, `origin-when-cross-origin`,
  `strict-origin-when-cross-origin`, `unsafe-url`;
- `rel`: token ou array de tokens, separados por espaço, entre exatamente
  `alternate`, `canonical`, `author`, `bookmark`, `dns-prefetch`, `expect`,
  `external`, `help`, `icon`, `manifest`, `modulepreload`, `license`, `next`,
  `nofollow`, `noopener`, `noreferrer`, `opener`, `pingback`, `preconnect`,
  `prefetch`, `preload`, `prev`, `privacy-policy`, `search`, `stylesheet`,
  `tag`, `terms-of-service`;
- `target`: `_blank`, `_self`, `_parent`, `_top` ou string livre;
- chamada vazia produz `elem(tag: "a", body: none)` e body aceita content;
- named desconhecido, `data-*` e atributos de `ol`/`li` são rejeitados.

A fonte checkout de `typst-assets` não está materializada localmente; portanto
os tipos acima são medição binária, não atribuição de `file:line` à tabela. O
pin é provado pelo lockfile. Uma futura disponibilidade da fonte que divirja
dos casts medidos refuta a caracterização da tabela, não o observável medido.

### Contrato público

Sob `Feature::Html`, acrescentar exatamente `html.a` com os oito named acima,
os mesmos 76 globais P1168 e body content posicional opcional. Todos os named
são opcionais/unset. O constructor produz o `HtmlElem` triádico existente:
body omitido é `HtmlBody::None`; body fornecido é `Content`.

Casts e diagnósticos seguem as classes medidas. Strings livres não validam
URL, idioma ou MIME. Arrays de `ping` e `rel` preservam ordem e aceitam vazio;
o shorthand escalar é aceito. A ordem final dos atributos segue a chamada.
Não aceitar fallback, nono atributo, `data-*`, normalização de URL nem
resolução de `Content::Link`.

O nó `a` aninhado em `div` foi byte-idêntico via exporter genérico. No topo,
o vanilla envolve o anchor em `<p>` e o cristalino não; essa diferença pertence
à realização/agrupamento de conteúdo phrasing, não ao cast do constructor.
Ela não será corrigida implicitamente na materialização do binding: exige L0 e
gate próprios antes de alterar o exporter ou a fase de realização.

### Divisão explícita

P1170.1 materializou somente o binding e casts de `html.a`. `br`,
tabela void e whitespace permanecem P1171. Agrupamento phrasing de HTML de
topo, demais tags, raw, frame, CSS, MathML e positions continuam incompletos.

## P1171 — `br` void (APROVADO E MATERIALIZADO EM 2026-08-25)

### Medição

`typst-html/src/tag.rs:123-141` classifica exatamente 13 tags void: `area`,
`base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `source`,
`track`, `wbr`. Em `typed.rs:73-114`, tag void não recebe parâmetro body.

O vanilla ratificado mediu `type(html.br) == function`; `html.br()` produz
`elem(tag: "br")`, sem field body. O constructor aceita os mesmos 76 globais
e nenhum específico. Qualquer posicional é `unexpected argument`; named
desconhecido e `data-*` também são rejeitados.

`html.elem("br")[X]` pode ser construído como
`elem(tag: "br", body: [X])`, mas a compilação HTML falha com
`HTML void elements must not have children`. Sem filho, o DOM é `<br>` — sem
slash e sem `</br>`. A classificação é derivada da tag e não exige campo novo.

### Contrato público

Sob `Feature::Html`, acrescentar exatamente
`html.br(..76 atributos globais P1168)`, todos opcionais/unset, zero atributos
específicos e nenhum body. O resultado usa `HtmlElem` existente com tag `br`,
attrs opcionais e `HtmlBody::Unset`, portanto repr omite body.

Uma tabela interna das 13 tags void deve ser reutilizada pelo exporter para
emitir somente start tag e rejeitar body content. Não duplicar a lista nem
adicionar booleano público por nó.

### Whitespace e divisão

Eval vanilla e cristalino preservam o mesmo `Content::Space` para newline
entre expressões HTML. A divergência é L3: dentro de `div` formatado, vanilla
remove espaços nas bordas e preserva um espaço entre dois `span`; o cristalino
preserva também as bordas. Entre dois `div` no topo ambos omitem o espaço. Em
torno de `br`, vanilla produz `A<br>B` no caso formatado.

P1171.1 pode corrigir essa normalização no exporter, pois a fase causal já é
L3. Deve usar regras medidas, não `trim()` global. Agrupamento phrasing de topo
permanece fora. Outras tags void, demais tags, raw, frame, CSS, MathML e
positions continuam incompletos.

## P1173 — segundo lote global-only (MATERIALIZADO EM P1173.1)

O inventário ratificado P1140.26 mede cada candidata com exatamente 77
parâmetros, lista idêntica a `html.div`: 76 globais mais body. Sondas vanilla
confirmaram function, `body: none`, body content, casts/ordem globais e
rejeição de `href`. Nenhuma é void ou raw.

Sob `Feature::Html`, acrescentar exatamente:

```text
html.abbr html.address html.article html.aside html.b html.bdi html.bdo
html.cite html.code html.dfn html.i html.kbd
```

Cada função aceita somente os 76 globais P1168 e body content posicional
opcional. Não há específicos, aliases ou fallback. `address`, `article` e
`aside` são block; as outras nove são phrasing visíveis já cobertas por
P1172.1. A fixture revelou whitespace extra entre block siblings no body de
`article`; a correção é L3 interna e não amplia a assinatura.

P1173.1 materializa somente estes 12 bindings e a correção de whitespace block
medida. Todas as outras tags continuam fora.

## P1174 — terceiro lote global-only (MATERIALIZADO EM P1174.1)

### Medição anterior à decisão

O inventário ratificado P1140.26 compara cada candidata com `html.div`: todas
têm exatamente a mesma lista ordenada de 77 parâmetros e os mesmos metadados,
isto é, 76 globais mais body. `typed.rs:73-114` deriva a assinatura da entrada,
acrescentando body content somente quando a tag não é void; `tag.rs:125-158`
exclui as 12 das classes void, raw e escapable-raw.

Sondas no binário ratificado confirmaram para todas `function`, chamada vazia
com `body: none`, body content e atributos `id`, `class`, `hidden` na ordem. As
três sentinelas específicas `href`, `value` e `start` foram rejeitadas em cada
tag como `unexpected argument`. Named desconhecido, segundo body posicional e
body named também foram rejeitados nas representantes block e phrasing.

`property.rs:82-120` classifica `dd`, `dl`, `dt`, `figcaption`, `figure`,
`footer`, `header`, `hgroup`, `legend`, `main` e `menu` como block;
`property.rs:194` classifica `mark` como inline, e `tag.rs:298-335` inclui
`mark` em phrasing. Fixtures tipada vanilla e genérica cristalina foram
byte-idênticas: `mark` no topo recebeu wrapper `<p>` e as outras onze foram
boundaries block. `dd` e `legend` isolados foram aceitos, portanto as regras
contextuais de conteúdo não pertencem ao constructor neste recorte.

### Contrato público

Sob `Feature::Html`, expor exatamente:

```text
html.dd html.dl html.dt html.figcaption html.figure html.footer html.header
html.hgroup html.legend html.main html.mark html.menu
```

Cada função aceita somente os mesmos 76 atributos globais de P1168 e body
content posicional opcional. Não há atributos específicos, aliases, fallback,
validação de parent/contexto, regra void/raw nem alteração de entidade.
Chamada sem body produz `HtmlBody::None`; body fornecido produz Content.

P1174.1 materializa somente estes 12 bindings pelo dispatcher estático
existente. O exporter e `HtmlElem` não mudam. Todas as outras tags,
atributos específicos, raw, void, frame, CSS, MathML e positions continuam
fora.

## P1175 — quarto lote global-only (MATERIALIZADO EM P1175.1)

### Medição anterior à decisão

O inventário ratificado P1140.26 compara `nav`, `picture`, `pre`, `s`, `samp`,
`search`, `section`, `small`, `sub`, `sup`, `u` e `var` com `html.div`: cada
lista tem os mesmos 77 parâmetros, na mesma ordem e com metadados idênticos —
76 globais mais body. `typed.rs:73-114` deriva essa assinatura;
`tag.rs:125-158` exclui as 12 de void, raw e escapable-raw.

Sondas no vanilla ratificado confirmaram para todas `function`, `body: none`,
body Content e conversão ordenada de `id`, `class`, `hidden`. `href`, `value`
e `start` foram rejeitados em cada tag; unknown named, segundo body posicional
e body named também foram rejeitados nas representantes block e phrasing.

`property.rs:95-114` classifica `nav`, `pre`, `search` e `section` como block;
`property.rs:198-211` classifica as outras oito como inline, e
`tag.rs:298-349` confirma phrasing. A fixture principal tipada/genérica foi
byte-idêntica. `pre` não introduziu body raw: o whitespace do Content Typst já
estava normalizado da mesma forma nos dois caminhos. `picture` aceita vazio e
texto sem regra própria de constructor.

### Contrato público

Sob `Feature::Html`, expor exatamente:

```text
html.nav html.picture html.pre html.s html.samp html.search html.section
html.small html.sub html.sup html.u html.var
```

Cada função aceita somente os 76 globais P1168 e body content posicional
opcional. Não há específicos, aliases, fallback, parent validation, regra
void/raw ou estado novo. Body omitido produz `HtmlBody::None`; body fornecido
produz Content.

P1175.1 materializa somente estes 12 bindings pelo dispatcher estático
existente. A correção de whitespace vazia descrita no L0 L3 é fluxo
contínuo de paridade e não amplia o consentimento. As outras 16 tags
global-only inventariadas, demais tags, atributos específicos, frame, CSS,
MathML e positions continuam fora.

## P1176 — lote residual normal (MATERIALIZADO EM P1176.1)

### Medição anterior à decisão

O inventário P1140.26 compara `datalist`, `noscript` e `summary` com
`html.div`: as três têm a mesma lista ordenada de 77 parâmetros e metadados,
isto é, 76 globais mais body. `typed.rs:73-114,125-158` confirma a via única;
`tag.rs:125-158` exclui void/raw/escapable-raw.

Sondas vanilla confirmaram `function`, `body: none`, body Content e attrs
`id`, `class`, `hidden` ordenados. `href`, `value`, `start`, unknown named,
segundo body e body named foram rejeitados nas classes medidas. `noscript`
produziu a mesma entidade com target `paged` e `html`; não há constructor
dependente de scripting/target.

`property.rs:67` classifica `datalist` como display none, mas
`tag.rs:298-332` o inclui em phrasing; no topo, vanilla o mantém fora do
parágrafo, enquanto dentro de `div` ele participa do contexto inline.
`property.rs:195` classifica `noscript` inline/phrasing. `property.rs:144-146`
classifica `summary` block. As divergências encontradas são regras L3 de
parágrafo/whitespace, não de assinatura.

### Contrato público

Sob `Feature::Html`, expor exatamente:

```text
html.datalist html.noscript html.summary
```

Cada função aceita somente os 76 globais P1168 e body content posicional
opcional. Não há específicos, aliases, fallback, validação contextual, regra
void/raw nem estado novo. Body omitido produz `HtmlBody::None`; body fornecido
produz Content.

P1176.1 materializa somente estes três bindings. As correções
L3 descritas no L0 do exporter seguem fluxo contínuo e não ampliam o
consentimento. Documento/raw, tabela, ruby, demais tags, específicos, frame,
CSS, MathML e positions permanecem fora.

## P1177 — família ruby (MATERIALIZADO EM P1177.1)

### Medição anterior à decisão

No vanilla ratificado, `typed.rs:73-114` constrói `ruby`, `rp` e `rt` pela
mesma via das tags normais. O inventário pinado compara cada uma com
`html.div`: 77 parâmetros idênticos e na mesma ordem, isto é, os 76 globais
de P1168 mais body Content posicional opcional. As três ficam fora das tabelas
void/raw/escapable-raw de `tag.rs:125-158`.

Sondas no binário ratificado confirmaram para as três `function`, chamada
vazia com `body: none`, body Content e attrs globais `id`, `class`, `hidden`.
`href`, `value`, `start`, named desconhecido, segundo body e body named foram
rejeitados. `rp` e `rt` isolados são aceitos: o constructor não valida parent
nem content model.

`property.rs:100-101` distingue somente o display: `ruby` é Ruby, `rt` é
RubyText e `rp` conserva o default None. `tag.rs:290-350,543-546` torna apenas
`ruby` agrupável em parágrafo. Fixtures tipadas/genéricas confirmaram nesting,
vazios e top-level: `ruby` entra em `<p>`; `rp` e `rt` isolados são boundaries.
A única divergência foi whitespace entre filhos consecutivos `rp`/`rt` dentro
de `ruby`, especificada separadamente no L0 L3.

### Contrato público

Sob `Feature::Html`, expor exatamente:

```text
html.ruby html.rp html.rt
```

Cada função aceita somente os 76 atributos globais P1168 e body Content
posicional opcional. Não há específicos, aliases, fallback, validação de
parent/contexto, regra void/raw nem estado novo. Body omitido produz
`HtmlBody::None`; body fornecido produz Content. A representação `HtmlElem`
existente é suficiente.

P1177.1 materializa somente estes três bindings pelo dispatcher estático
existente e a correção L3 explicitada no seu próprio L0. Documento/raw,
tabela, outros constructors, frame, CSS, MathML e positions permanecem fora.


## Sete constructors tipados residuais\n\n### Superfície e body

Sob `Feature::Html`, acrescentar exatamente as funções de nomes curtos:

```text
html.button html.col html.iframe html.select
html.template html.video html.wbr
```

`button`, `iframe`, `select`, `template` e `video` recebem no máximo um body
Content posicional opcional: omissão produz `HtmlBody::None`, fornecimento
produz `HtmlBody::Content`. `col` e `wbr` não recebem body nem qualquer
positional e usam `HtmlBody::Unset`. Chamada vazia, extra, body named e cast
inválido conservam os erros e spans medidos; nenhum named é ignorado.

Todos aceitam os 76 atributos globais já selados. Além deles, somente os 48
específicos abaixo pertencem a cada tag:

| Tag | Contrato de casts específicos |
|---|---|
| `button` | `command`: um de `toggle-popover|show-popover|hide-popover|close|request-close|show-modal` ou string; `commandfor`, `form`, `formaction`, `name`, `popovertarget`, `value`: string; `disabled`, `formnovalidate`: Presence; `formenctype`: `application/x-www-form-urlencoded|multipart/form-data|text/plain`; `formmethod`: `GET|POST|dialog`; `formtarget`: `_blank|_self|_parent|_top` ou string; `popovertargetaction`: `toggle|show|hide`; `type`: `submit|reset|button` |
| `col` | `span`: inteiro estritamente positivo |
| `iframe` | `allow`, `src`, `srcdoc`: string; `allowfullscreen`: Presence; `height`, `width`: inteiro não negativo; `loading`: `lazy|eager`; `name`: `_blank|_self|_parent|_top` ou string; `referrerpolicy`: `none` ou `no-referrer|no-referrer-when-downgrade|same-origin|origin|strict-origin|origin-when-cross-origin|strict-origin-when-cross-origin|unsafe-url`; `sandbox`: lista ordenada do conjunto fechado descrito abaixo |
| `select` | `autocomplete`: lista ordenada do conjunto fechado descrito abaixo; `disabled`, `multiple`, `required`: Presence; `form`, `name`: string; `size`: inteiro estritamente positivo |
| `template` | `shadowrootclonable`, `shadowrootcustomelementregistry`, `shadowrootdelegatesfocus`, `shadowrootserializable`: Presence; `shadowrootmode`: `open|closed` |
| `video` | `autoplay`, `controls`, `loop`, `muted`, `playsinline`: Presence; `crossorigin`: `anonymous|use-credentials`; `height`, `width`: inteiro não negativo; `poster`, `src`: string; `preload`: valores de linguagem `none` ou `auto`, ou a string literal `"metadata"` |
| `wbr` | nenhum específico |

O conjunto fechado de `sandbox` é: `allow-downloads`, `allow-forms`,
`allow-modals`, `allow-orientation-lock`, `allow-pointer-lock`, `allow-popups`,
`allow-popups-to-escape-sandbox`, `allow-presentation`, `allow-same-origin`,
`allow-scripts`, `allow-top-navigation`,
`allow-top-navigation-by-user-activation`,
`allow-top-navigation-to-custom-protocols`.

O conjunto fechado de `autocomplete` é: `shipping`, `billing`, `name`,
`honorific-prefix`, `given-name`, `additional-name`, `family-name`,
`honorific-suffix`, `nickname`, `username`, `new-password`,
`current-password`, `one-time-code`, `organization-title`, `organization`,
`street-address`, `address-line1`, `address-line2`, `address-line3`,
`address-level4`, `address-level3`, `address-level2`, `address-level1`,
`country`, `country-name`, `postal-code`, `cc-name`, `cc-given-name`,
`cc-additional-name`, `cc-family-name`, `cc-number`, `cc-exp`,
`cc-exp-month`, `cc-exp-year`, `cc-csc`, `cc-type`,
`transaction-currency`, `transaction-amount`, `language`, `bday`, `bday-day`,
`bday-month`, `bday-year`, `sex`, `url`, `photo`, `home`, `work`, `mobile`,
`fax`, `pager`, `tel`, `tel-country-code`, `tel-national`, `tel-area-code`,
`tel-local`, `tel-local-prefix`, `tel-local-suffix`, `tel-extension`, `email`,
`impp`.

Presence `true` serializa valor vazio; `false` omite o atributo. Inteiro fora
do domínio, enum inválido, tipo alheio, named desconhecido, `data-*` e atributo
específico de outra tag são erros no span do valor/nome medido. Atributo
omitido não é serializado. Após omissões, `repr` e DOM preservam a ordem de
chamada; listas usam espaço entre tokens.

### Feature, target e DOM

Feature e target permanecem eixos ortogonais (ADR-0128): feature off rejeita o
binding tanto em target paged quanto na exportação HTML; feature on disponibiliza
módulo, função e chamada em target paged e HTML. Selecionar target HTML nunca
habilita `Feature::Html`.

O exporter genérico deve produzir DOM equivalente: atributos escapados; body e
nesting preservados; `col` e `wbr` somente com start tag, sem end tag; texto de
body escapado segundo o contrato vigente. Este lote não cria comportamento de
browser, CSS, mídia ou rede, não cria variante de `Content`, não duplica
`HtmlElem`, não aceita named como string livre e não materializa `html.frame`
ou tag fora da lista.

Aceitação cobre as sete funções, 48 casts específicos, globais representativos,
body/void, ordem, escaping, nesting e os quatro quadrantes feature/target.
`Unknown` nunca é sucesso; crash, timeout, cast ambíguo ou DOM não observado
é falha.

## P1178 — família de documento (MATERIALIZADO EM P1178.1)

### Medição anterior à decisão

No asset pinado `94dcb99`, `files/html/data.rs:76-80,236-240,256-260,
536-540` dá listas específicas vazias a `body`, `head`, `html` e `title`.
`src/html.rs:20-27` concatena os 76 globais, e `typed.rs:73-114` acrescenta
body Content posicional opcional porque nenhuma é void nem raw. `title` é
escapable-raw em `tag.rs:149-153`, mas `typed.rs:88-101,145-153` usa body
string somente para `script`/`style`; portanto sua assinatura continua
Content.

Sondas no vanilla ratificado confirmaram, para as quatro, `function`, chamada
vazia com `body: none`, body `[X]` e attrs `id`, `class`, `hidden` ordenados.
`href`, `value`, `start`, named desconhecido, segundo body e body named foram
rejeitados. Cada constructor também produziu o mesmo `HtmlElem` em target
paged, inclusive fora da hierarquia normativa: não há validação de parent,
unicidade ou target durante a construção.

A validação e composição documentais acontecem depois, no exporter. `html` e
`body` possuem comportamento morfológico próprio; `head` e `title` também
exigem classificações L3 descritas no L0 do exporter. A representação
`HtmlElem` atual preservou todos os estados necessários.

### Contrato público

Sob `Feature::Html`, expor exatamente:

```text
html.html html.head html.body html.title
```

Cada função aceita somente os 76 atributos globais P1168 e body Content
posicional opcional. Não há específicos, aliases, fallback, validação de
parent/unicidade/target, regra void nem estado novo. Body omitido produz
`HtmlBody::None`; body fornecido produz Content. `title` não aceita string por
uma via especial: texto/string convertido em Content segue o contrato normal.

P1178.1 materializa somente estes quatro bindings pelo dispatcher estático e
as correções L3 explicitadas no próprio L0. Tabela, `script`/`style`, outros
constructors, frame, CSS, MathML e positions permanecem fora.


## Casts, mensagens e união exata de `video.preload`\n\n### Decisão estreita

Os casts P1293 usam nomes públicos completos nos erros locais: `string`,
`integer` e `boolean`. Falha de tipo permanece `expected <tipo>, found
<tipo público>`; depois de tipo válido, zero/negativo em domínio estritamente
positivo produz `number must be positive`, e negativo em domínio não negativo
produz `number must be at least zero`. Enums que incluem o valor de linguagem
`none` o listam sem aspas; não o substituem por string homônima.

`video.preload` aceita exatamente três polos válidos:

- `Value::None` → atributo textual `none`;
- `Value::Auto` → atributo textual `auto`;
- `Value::Str("metadata")` → atributo textual `metadata`.

`Value::Str("none")` e `Value::Str("auto")` são inválidos. O erro fechado é
`expected none, auto, or "metadata"`, seguido do tipo/valor conforme a
disciplina diagnóstica medida. Não há coerção por spelling nem fallback string.
`iframe.referrerpolicy:none` continua distinto: aceita o valor `none` e
serializa string vazia conforme seu contrato vigente.

Mensagens e aceitação são semântica observável (ADR-0107). A correção é local a
este owner e segue fluxo contínuo ADR-0127; spans pertencem ao owner
`call_dispatch`, e repr/export ao seus owners 1:1. Não alterar os 76 globais,
48 específicos, ordem, body/void, feature/target, entidade, API, default ou
fase. Refutam esta decisão qualquer aceitação das strings `"none"`/`"auto"`,
rejeição dos valores `none`/`auto`, mudança de `"metadata"` ou efeito fora dos
constructors P1293.
