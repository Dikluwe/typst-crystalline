# P1170 — auditoria de `html.a`

**Data:** 2026-08-25  
**Estado:** concluída; parada no gate ADR-0127  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Árvore:** working tree não commitado P1168–P1169.1; índice vazio  
**Início:** `2026-08-25T14:14:21-03:00`  
**Vanilla:** `a51e02804`; binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Fontes e limite de proveniência

`lab/typst-original/Cargo.lock:3120-3123` fixa `typst-assets` em
`94dcb990edb36311ef85162baa26e4a8205e0bf0`. O inventário
`superficie-linguagem-p1140.26.json:25670+` lista `html.a` e os oito específicos
antes dos globais. `lab/typst-original/crates/typst-html/src/typed.rs:73-114`
define a formação da assinatura e body.

O checkout da dependência gerada não está disponível no filesystem. Uma
tentativa read-only de `cargo metadata` quis baixar crates e falhou por DNS;
nenhum ficheiro do repositório foi alterado por isso. Assim, índices e linhas
da tabela não são inventados: nomes vêm do inventário local, pin do lockfile e
casts/faixas das sondas do binário ratificado.

## Matriz medida

| atributo | cast vanilla | serialização | inválido observado |
|---|---|---|---|
| `download` | string livre | literal | bool/none: `expected string` |
| `href` | string livre | literal | int: `expected string` |
| `hreflang` | string livre | literal | none: `expected string` |
| `ping` | string ou array de strings | espaço | item com espaço tem erro próprio; item int espera string |
| `referrerpolicy` | none ou 8 tokens | none vira vazio | enum enumerada; tipo errado acrescenta `found` |
| `rel` | token ou array de 27 tokens | espaço | token inválido enumera os 27; tipo errado acrescenta `found` |
| `target` | 4 keywords ou string livre | literal | int enumera keywords ou string |
| `type` | string livre | literal | none: `expected string` |

`ping` e `rel` aceitam array vazio, produzindo atributo vazio, e shorthand
escalar. Itens de `ping` não podem conter espaço; itens de `rel` precisam ser
um token da enum. `referrerpolicy: none` preserva o atributo com valor vazio.
Nenhuma sonda indicou parsing ou normalização de URL/MIME/idioma.

## Morfologia e isolamento

`type(html.a)` é `function`; `html.a()` produz
`elem(tag: "a", body: none)` e `html.a[X]` preserva content. Ordem de named é
preservada. `value`, `data-x`, segundo body e body inteiro falham nas classes
esperadas. `html.ol(href: "x")` também rejeita o atributo, comprovando que os
específicos não vazam.

No cristalino P1169.1 com feature on, `html.a` é ABSENT e `html.elem("a")` é
apenas aproximação genérica; `html.ol` permanece presente. Feature off mantém
o diagnóstico gated e os dois hints. Classificação: binding ABSENT;
representação `HtmlElem` MATCH; casts/assinatura ABSENT; exporter do nó
aninhado MATCH.

## Export e achado separado

Uma fixture aninhada em `html.div` foi byte-idêntica entre `html.a` vanilla e
`html.elem("a")` cristalino (`cmp` exit 0). A fixture completa preservou ordem
e escaping de `&` e aspas.

No topo, porém, vanilla produziu `<body><p><a ...>...</a></p></body>` (390
bytes), enquanto o cristalino produziu `<body><a ...>...</a></body>` (383
bytes). Esta diferença é semântica de agrupamento de conteúdo phrasing na
realização HTML, não cast do constructor nem branch de serialização da tag.
Inferência: corrigir exige classificação de tags/realização anterior ao
export. Refutador: fonte ou sonda mostrando que o wrapper é inserido pelo
constructor `html.a` em vez da expansão do documento.

## Decisão posterior à medição

O contrato mínimo pode reutilizar `HtmlElem`, `HtmlBody`, os 76 globais e o
exporter genérico. É necessário um novo binding público com oito especificações
estáticas. Nenhuma mudança de entidade foi justificada. O agrupamento phrasing
fica fora de P1170.1 e precisa de L0/gate próprios.

São língua: binding, assinatura, casts, body, repr, diagnósticos e morfologia
DOM. São mecânica: arrays Rust, lookup e wrappers. O `cmp` é usado como decalque
do observável HTML, não como definição geral de paridade.

## Gate solicitado

Aprovar ou rejeitar exatamente:

1. novo binding público `html.a`;
2. oito named específicos com os casts medidos;
3. reutilização dos 76 globais e body content opcional;
4. nenhuma mudança de entidade, exporter, default ou pipeline;
5. agrupamento phrasing de topo explicitamente fora.

Não houve resselo, teste RED, código L1–L4, staging ou commit.
