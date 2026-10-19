# P1176 — auditoria das tags HTML residuais normais

**Data:** 2026-08-25
**Estado:** concluída; parada no gate ADR-0127
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`
**Início:** `2026-08-25T15:27:39-03:00`
**Árvore:** working tree não commitado P1173.1–P1175.1; índice vazio
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Proveniência

`git diff HEAD --stat` iniciou com 6 ficheiros rastreados, 582 inserções e 24
remoções; documentos untracked não entram na contagem. O índice estava vazio.
O cristalino medido tinha SHA-256
`bb2c159e8867c590f35eb86de7a7afbb869d65809647139b73166333982b8800`.

## Composição e sondas

As três listas têm 77 parâmetros integralmente iguais a `html.div`, zero
extras/ausentes e `present=false` no inventário cristalino.

| tag | específicos | body | void/raw | display/categoria |
|---|---:|---|---|---|
| `datalist` | 0 | content opcional | não | none, phrasing |
| `noscript` | 0 | content opcional | não | inline, phrasing |
| `summary` | 0 | content opcional | não | block |

Para cada tag, vanilla retornou `function`, `body: none`, body `[X]` e globals
ordenados. `href`, `value` e `start` deram `unexpected argument`; as classes de
unknown/segundo body/body named também coincidiram nas representantes.
`noscript` gerou a mesma entidade sob targets `paged` e `html`.

No cristalino pré-código, as três são fields ausentes. A tabela `TYPED_TAGS`
contém 52 bindings e `repr(type(...))` confirmou os 52 como `function`, zero
ausentes. Feature off preservou o diagnóstico gated.

## Morfologia e achados L3

Na fixture principal, `datalist` ficou fora de `<p>`, `noscript` agrupou com
phrasing e `summary` ficou block. A única diferença foi espaço após `summary`
dentro de `details`: vanilla removeu, cristalino preservou.

Na fixture vazia, vanilla não criou parágrafos de whitespace ao redor de
`datalist`/`summary`; cristalino criou spans protegidos isolados. Entre dois
`noscript` vazios, vanilla protegeu o espaço como P1175.1.

Uma contraprova dentro de `div` foi byte-idêntica: `datalist` com conteúdo
preservou espaços literais, e vazio entre inline vazios preservou spans
pre-wrap. Isso refuta classificá-lo como block. A causa é boundary de formação
de parágrafo no topo, separada da análise inline.

## Decisão e gate

As três tags sobrevivem como bindings global-only. A entidade atual basta. O
L0 L3 especifica `summary` block e descarte de espaço junto a HtmlElem
não agrupável em `block_sequence`, sem alterar `datalist` dentro de bodies.

P1176 não ressellou nem escreveu testes/código. Solicita aprovação ADR-0127
para exatamente `html.datalist`, `html.noscript` e `html.summary`, cada um com
76 globais e body content opcional. As correções L3 são fluxo contínuo após o
gate público. Famílias documento, tabela e ruby ficam fora. Índice vazio;
nenhum commit criado.
