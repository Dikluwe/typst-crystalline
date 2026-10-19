# P1177 — auditoria da família HTML ruby

**Data da medição:** 2026-08-25T15:48:52-03:00  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**HEAD cristalino:** `ce49041de76eb64c011e990e9774a0339f979211`  
**Estado:** decisão redigida; parado no gate ADR-0127

## Proveniência

O binário vanilla medido foi `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O cristalino foi `./target/debug/typst`, SHA-256
`e68cd9e8e59fcd0c4ba86a49d16268d7dd3e4a6004ff381d4b9e10905fe42ce3`.
A versão impressa foi respectivamente `0.15.1 (e0e8ca4d)` e
`0.15.1 (ce49041d)`; ela não substitui o pin.

A working tree não estava commitada: seis paths tracked, 809 inserções e 24
remoções acumuladas de P1173.1–P1176.1, além dos passos/diagnósticos untracked
correspondentes e `typst-passo-1177.md`. O índice estava vazio
(`git diff --cached --quiet` exit 0).

## Assinatura medida

| tag | parâmetros | específicos | body | void/raw | display | top-level |
|---|---:|---:|---|---|---|---|
| `ruby` | 77 | 0 | Content opcional | não | Ruby | agrupável |
| `rp` | 77 | 0 | Content opcional | não | None | boundary |
| `rt` | 77 | 0 | Content opcional | não | RubyText | boundary |

As 77 entradas coincidem integralmente com `html.div`: 76 globais mais body.
Para cada tag, vanilla confirmou `function`, `body: none`, body `[X]` e attrs
`id`, `class`, `hidden`. `href`, `value`, `start` e unknown named são
inesperados; segundo body e body named falham. `rp` e `rt` isolados constroem
normalmente, refutando validação parental. No cristalino, as três funções
continuam ausentes e os 55 bindings anteriores permanecem no dispatcher.

## Morfologia diferencial

Nesting e tags vazias coincidiram entre vanilla tipado e cristalino genérico.
No topo, `ruby` foi envolvido por `<p>`, enquanto `rp` e `rt` isolados ficaram
fora. Entre siblings phrasing, a família preservou a ordem e não criou wrapper
interno.

A divergência reproduzível foi:

```html
vanilla:   <ruby><rp>(</rp><rt>T</rt><rp>)</rp></ruby>
cristalino:<ruby><rp>(</rp> <rt>T</rt> <rp>)</rp></ruby>
```

Com filhos vazios, o cristalino chegou a proteger o espaço; vanilla o
descartou. As contraprovas `ruby[A rt[T]]` e `ruby[A span[X] B]` preservaram
seus espaços nos dois compiladores. A regra observada é portanto localizada
ao espaço cujos dois vizinhos imediatos são `rp`/`rt`, não ao body inteiro de
`ruby`. A inferência é que o descarte pertence à realização ruby; seria
refutada por fixture ratificada que preserve tal espaço.

## Decisão

Propor exatamente `html.ruby`, `html.rp` e `html.rt`, reutilizando o
constructor global-only e `HtmlElem`. Separadamente, nuclear em L3 o descarte
de `Content::Space` entre dois filhos `rp`/`rt` dentro de `ruby`, antes da
proteção de espaços. Não alterar entidade, parent validation, defaults ou fase.

Os L0s foram atualizados sem hash novo. Por se tratar de três bindings
públicos, P1177 para aqui. Após aprovação, P1177.1 fará resselo, RED, código e
validação GREEN somente deste escopo.
