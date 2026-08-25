# Prompt L0 — módulo público `html` feature-gated
Hash do Código: 72916055

**Estado:** APROVADO NO GATE ADR-0127 EM 2026-08-25  
**Camada:** L1  
**Owners candidatos:** `compiler/stdlib/html.rs`, `compiler/eval/mod.rs`  
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

### Contrato público proposto para P1168

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

### Contrato público proposto

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

### Contrato público proposto

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

### Contrato público proposto

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

## P1173 — segundo lote global-only (APROVADO NO GATE ADR-0127 EM 2026-08-25)

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

P1173.1, se aprovado, materializa somente estes 12 bindings e a correção de
whitespace block medida. Todas as outras tags continuam fora.
