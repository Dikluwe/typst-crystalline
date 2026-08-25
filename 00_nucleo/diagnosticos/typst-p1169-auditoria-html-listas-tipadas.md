# P1169 — auditoria de `html.ol` e `html.li`

**Data:** 2026-08-25  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83` + working tree P1168  
**Vanilla:** pin ratificado `a51e02804`  
**Estado:** `APROVADO NO GATE ADR-0127 EM 2026-08-25`

## Proveniência

Medição iniciada em `2026-08-25T13:59:58-03:00`. A working tree continha os
10 paths rastreados P1168 (793 inserções/23 remoções), o diagnóstico P1168 e o
passo P1169 novos. O índice estava vazio. SHA-256: vanilla
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
cristalino debug P1168
`f59bfc98fd6d5c95cda155208e0ab4dce0ed0a777a5f27dd0417587bc13f3a72`.
As versões impressas não são usadas como prova do pin.

## Fonte antes da decisão

| binding | índice específico | tipo pinado | globais | body/classe |
|---|---:|---|---:|---|
| `ol.reversed` | 181 | Presence | 76 | content opcional; normal |
| `ol.start` | 201 | Int | 76 | content opcional; normal |
| `ol.type` | 214 | Strings(229,234) = `1,a,A,i,I` | 76 | content opcional; normal |
| `li.value` | 220 | Int | 76 | content opcional; normal |

As entradas são literais em `data.rs:302-304,357-359`; a construção comum e
body vêm de `typed.rs:73-114,118-159`. Não há inferência a partir da spec HTML.

## Sondas vanilla

`repr((type(html.ol), type(html.li), html.ol, html.li))` devolveu
`(function, function, ol, li)`. Chamadas vazias devolveram
`elem(tag: "ol", body: none)` e `elem(tag: "li", body: none)`.

| sonda | resultado |
|---|---|
| `ol(reversed:true,start:-2,type:"A")[X]` | attrs `("", "-2", "A")` em ordem; body `[X]` |
| `ol(reversed:false)[X]` | atributo omitido |
| `li(value:-3)[X]` | `value: "-3"` |
| cinco valores de `ol.type` | todos aceites e preservados |
| `ol[#li(value:3)[X]]` | body é `elem(tag:"li", attrs:(value:"3"), body:[X])` |

Erros medidos:

- `reversed`: none/int/string → `expected boolean, found TYPE`;
- `start`/`value`: float/string/none → `expected integer, found TYPE`;
- `type: "x"|""` → enumeração exata dos cinco tokens;
- `type: bool|none` → mesma enumeração + `found TYPE`;
- `ol(value:1)`, `li(start:1)`, `data-x` → `unexpected argument: NAME`;
- `ol(1,2)` falha no primeiro posicional com `expected content, found integer`.

## DOM e nesting

A fixture compacta e a formatada produziram o mesmo DOM vanilla:

```html
<ol id="o" start="2" reversed type="A"><li value="3">X</li><li>Y <strong>Z</strong></li></ol>
```

Isso confirma serialização normal, atributo Presence sem igual/aspas,
nesting e ausência de wrapper. A eliminação do whitespace da fixture formatada
continua sendo o gap de eval/HTML já transferido a P1171.

## Estado cristalino P1168

- feature off: diagnóstico gated e dois hints, exit 1;
- feature on: `html.ol` e `html.li` ausentes, exit 1;
- `html.elem("ol")` aninhado com `html.elem("li")` preserva DOM/repr quando o
  utilizador fornece strings, mas não oferece os casts nem os bindings;
- `HtmlBody`, tabela global e exporter normal já são suficientes.

| binding | vanilla | P1168 | dependência | decisão |
|---|---|---|---|---|
| `html.ol` | função completa | ABSENT | 3 specs locais + wrapper/registro | entra P1169.1 |
| `html.li` | função completa | ABSENT | 1 spec local + wrapper/registro | entra P1169.1 |

Não há necessidade medida de alterar `HtmlElem`, `HtmlBody`, `Content`,
exporter, feature/default ou pipeline.

## Classificação e inferência

Bindings, casts, repr, diagnósticos, body e DOM são língua. A forma da tabela,
lookup e wrappers Rust é mecânica. Inferência: acrescentar metadados
específicos por tag à tabela P1168 basta. Refutador: RED futuro que demonstre
estado contextual ou diferença de exporter não presente nas sondas.

## Gate ADR-0127

O dono aprovou acrescentar exatamente `html.ol` e `html.li`, com
os 76 globais existentes e somente `ol.reversed: Presence`, `ol.start: Int`,
`ol.type: {1,a,A,i,I}` e `li.value: Int`, além de body content opcional já
modelado. A aprovação não abrange `a`, `br`, outras tags nem whitespace.
Nenhum código, teste, hash, staging ou commit pertence a P1169.
