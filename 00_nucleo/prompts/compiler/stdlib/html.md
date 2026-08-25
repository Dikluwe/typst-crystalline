# Prompt L0 — módulo público `html` feature-gated
Hash do Código: 297ade9e

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
