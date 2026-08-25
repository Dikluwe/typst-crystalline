# P1175 — auditoria do quarto lote HTML global-only

**Data:** 2026-08-25
**Estado:** concluída; parada no gate ADR-0127
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`
**Início:** `2026-08-25T15:14:52-03:00`
**Árvore:** working tree não commitado P1173.1–P1174.1; índice vazio
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Proveniência

No início, `git diff HEAD --stat` continha 6 ficheiros rastreados, 321
inserções e 19 remoções; os documentos untracked não entram nessa contagem.
`git diff --cached --quiet` terminou com exit 0. O binário cristalino tinha
SHA-256 `5760b826461b8dbe07085d9a350c8744eaad89d4b530fed6885ddc5a81b49f0e`.

## Composição e fonte

A comparação mecânica contra `html.div` encontrou, em cada candidata, 77
parâmetros integralmente iguais em nome, ordem e metadados, sem extras nem
ausências. As 12 estavam `present=false` no inventário cristalino.

| tags | específicos | body | void/raw | display/categoria |
|---|---:|---|---|---|
| `nav pre search section` | 0 | content opcional | não | block |
| `picture s samp small sub sup u var` | 0 | content opcional | não | inline/phrasing |

`typed.rs:73-114` e `:125-158` mostram a via única de assinatura/construção.
`tag.rs:125-158` exclui void/raw/escapable-raw. `property.rs:95-114` prova as
quatro block; `property.rs:198-211` e `tag.rs:298-349` provam as oito phrasing.
A fonte gerada de assets não está materializada; lockfile, inventário e
binário foram triangulados sem atribuir linhas inexistentes a `data.rs`.

## Sondas de linguagem

Para cada tag, o vanilla retornou `function`, chamada vazia
`elem(tag: "TAG", body: none)` e body `[X]` com attrs `id`, `class`, `hidden:
""` na ordem. Em todas, `href`, `value` e `start` terminaram com exit 1 e
`unexpected argument: NAME`. Representantes block e phrasing também rejeitaram
unknown named, segundo body posicional e body named nas classes medidas.

No cristalino, as 12 sondas falharam como field ausente: ABSENT em sintaxe e
assinatura. Os 40 bindings anteriores retornaram `function`, zero ausentes; a
feature off preservou o diagnóstico gated. `html.elem` aproxima o DOM, mas não
substitui binding tipado.

## Morfologia

A fixture principal cobriu nesting block, `picture`, `pre`, `search` e oito
phrasing siblings. Vanilla tipado e cristalino genérico ficaram byte-idênticos
(`cmp` exit 0): quatro boundaries block e um parágrafo contendo os oito inline.
O corpo formatado de `pre` tornou-se `A B C` nos dois; não há body raw a
nuclear neste lote.

Uma fixture adversarial com `picture` vazio divergiu. Vanilla emitiu entre os
dois elementos `<span style="white-space: pre-wrap">&#x20;</span>`; cristalino,
espaço literal. A repetição com `html.span`, já materializado, reproduziu a
mesma divergência, refutando a atribuição confortável a `picture`. A fonte
`convert.rs:33-55,597-681` confirma um passe transversal de proteção de espaço
sem suporte visual em ambos os lados. Isso é correção interna L3, especificada
separadamente no L0 do exporter.

## Decisão e gate

As 12 candidatas sobrevivem: contrato público global-only, realizável por
wrappers estáticos e `TYPED_TAGS`, sem entidade nova. P1175 atualizou o L0 do
módulo e o L0 L3 da correção transversal, mas não ressellou nem escreveu
testes/código.

Solicita-se aprovação ADR-0127 para exatamente `nav`, `picture`, `pre`, `s`,
`samp`, `search`, `section`, `small`, `sub`, `sup`, `u`, `var`, cada um com 76
globais e body content opcional. A aprovação não cobre outras tags ou mudanças
de contrato. A correção L3 segue fluxo contínuo após o gate público. Índice
permaneceu vazio; nenhum commit foi criado.
