# P1167 — auditoria do primeiro lote de tags HTML tipadas

**Data:** 2026-08-25  
**Baseline cristalina:** `30a6f11bcb8344f083a88edc8de89b3a1b5d6d07` + working tree P1165/P1166  
**Vanilla:** pin ratificado `a51e02804`  
**Estado:** `APROVADO NO GATE ADR-0127 EM 2026-08-25`

## Proveniência

Medição principal em `2026-08-25T13:22:01-03:00`. A árvore tinha 33 ficheiros
rastreados alterados (443 inserções/42 remoções) e 11 ficheiros novos antes
deste diagnóstico; `git diff --cached --stat` estava vazio. A lista exata é a
saída de `git status --short` reproduzida no passo P1167; corresponde somente
ao trabalho ainda não commitado P1165/P1166 e ao próprio passo P1167.

SHA-256 dos binários: vanilla
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
cristalino debug
`0d0cb57333432866b534aa063ed124266ab1c08403928a648fa0eb53679cb276`.
As versões impressas foram, respetivamente, `typst 0.15.1 (e0e8ca4d)` e
`typst 0.15.1 (30a6f11b)`; a primeira não é usada como prova do pin.

## Medição antes da decisão

`typst-html/src/typed.rs:31-69` cria uma função pública para cada entrada da
tabela. `:74-114` acrescenta todos os atributos declarados como named
opcionais e acrescenta `body: content` posicional opcional somente a tags não
void; `:118-159` converte cada atributo antes de criar o mesmo `HtmlElem` usado
por `html.elem`. A tabela pinada declara que os primeiros 76 atributos são
globais (`data.rs:1702-1703`) e `ElemInfo::attributes` os concatena aos
específicos (`typst-assets/src/html.rs:25-32`). Logo até uma tag sem atributos
específicos exige a assinatura global completa; aceitar apenas strings seria
uma API diferente.

As entradas `data.rs:157-159,172-174,207-234,377-379,462-469,557-559`
confirmam que `div`, `span`, `p`, `h1..h6`, `strong`, `em` e `ul` não têm
atributos específicos. `a` tem 8 (`:16-20`), `ol` tem 3 (`:357-360`) e `li`
tem 1 (`:302-305`). `tag.rs:125-142` classifica `br` como void e nenhuma das
outras candidatas como void; nenhuma candidata é raw (`:145-147`).

Uma única sonda ratificada sobre os 16 bindings devolveu tipo `function` para
todos. Chamadas vazias deram `elem(tag: "TAG", body: none)` para as 15 tags
normais e `elem(tag: "br")` para `br`. Body `[x]` foi preservado nas 15 tags
normais; `html.br[x]` falhou com `unexpected argument` no body.

Sondas de casts confirmaram:

- `class: ("a", "b")` vira `class: "a b"`;
- atributos Presence entram como `""` quando `true` e são omitidos quando
  `false` (`hidden`, `reversed`);
- inteiros são decimalizados, incluindo `tabindex: -2` e `start: -2`;
- `id: 1` falha com `expected string, found integer`;
- `ol(type: "x")` falha enumerando `"1", "a", "A", "i", "I"`;
- named desconhecido falha com `unexpected argument: unknown`.

A fixture aninhada emitiu no vanilla:

```html
<div id="outer" class="a b"><h1>H</h1><p>P <strong>S</strong> <em>E</em> <span>X</span></p><ul><li>U</li></ul><ol start="2" reversed type="A"><li value="3">O</li></ol><a href="/x" rel="next nofollow" target="frame">L</a> A<br>B</div>
```

No cristalino P1166, feature off manteve o diagnóstico gated e dois hints;
feature on, `html.div` falhou com `module 'html' does not contain field
"div"`. O escape hatch genérico conseguiu construir a fixture
`html.elem("div")[A html.elem("br") B]`, mas emitiu `<br></br>`, confirmando
que isso não equivale à API tipada nem à semântica void.

## Inventário de assinatura

Todos os 16 candidatos recebem os mesmos 76 named globais opcionais. A matriz
exata, agrupada pelo cast Typst, está fixada no L0
`prompts/compiler/stdlib/html.md`. Além deles:

| binding | específicos | body | classe |
|---|---|---|---|
| `div span p h1 h2 h3 h4 h5 h6 strong em ul` | nenhum | `content`, opcional | normal |
| `ol` | `reversed: bool(Presence)`, `start: int`, `type: {1,a,A,i,I}` | `content`, opcional | normal |
| `li` | `value: int` | `content`, opcional | normal |
| `a` | `download/href/hreflang/type: str`; `ping: str array`; `referrerpolicy`; `rel: enum array`; `target: enum|string` | `content`, opcional | normal |
| `br` | nenhum | ausente | void |

`data-*` não consta dos 76 globais e é rejeitado pela API tipada; isso não é
inferido da especificação HTML, mas da tabela pinada e do lookup fechado.

## Decisão do lote

| família | P1166 | dependência adicional | decisão |
|---|---|---|---|
| `div span p h1..h6 strong em ul` | ABSENT; representação serve | tabela/casts dos 76 globais | **ENTRA P1168** |
| `ol li` | ABSENT | atributos específicos e casts de lista | adiar P1169 |
| `a` | ABSENT | 8 específicos, enums/listas próprios | adiar P1170 |
| `br` | ABSENT; exporter incorreto | classificação void declarativa geral | adiar P1171 |

O lote aprovado proposto contém exatamente 12 bindings. É coerente porque
todos são normais, não raw, têm zero atributo específico, aceitam o mesmo body
e partilham uma única tabela global. A representação `HtmlElem` já preserva
os observáveis após o cast; não é necessária mudança em `Content` nem no
contrato público da entidade.

Inferência: uma tabela estática de metadados/casts em L1 é suficiente e evita
funções ad hoc. Refutador: qualquer sonda que exija reflexão de assinatura não
representável pelo dispatcher atual ou conversão dependente de contexto. A
organização concreta da tabela é mecânica; assinatura, casts, repr,
diagnósticos e DOM são língua (ADR-0107).

## Gate ADR-0127

O dono aprovou acrescentar, sob `Feature::Html`, exatamente os
bindings públicos `html.div`, `html.span`, `html.p`, `html.h1`, `html.h2`,
`html.h3`, `html.h4`, `html.h5`, `html.h6`, `html.strong`, `html.em` e
`html.ul`, cada qual com os 76 named globais tipados e `body: content`
posicional opcional. A aprovação não abrange `ol`, `li`, `a`, `br` nem outras
tags. Nenhum hash, teste ou código L1-L4 foi alterado neste passo.
