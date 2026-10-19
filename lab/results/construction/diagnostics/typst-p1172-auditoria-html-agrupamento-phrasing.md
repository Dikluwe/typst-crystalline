# P1172 — auditoria de agrupamento phrasing HTML

**Data:** 2026-08-25  
**Estado:** concluída; fluxo contínuo ADR-0127  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Início:** `2026-08-25T14:40:08-03:00`  
**Árvore:** working tree não commitado P1168–P1171.1; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Fonte e intenção

`tag.rs:290-350` lista phrasing content. `tag.rs:504-546` declara que, quando
parágrafos são forçados, phrasing visível é agrupado; tags transparentes como
`a` são atualmente agrupadas incondicionalmente. `property.rs:62-76` marca
como none `area`, `datalist`, `link`, `meta`, `script` e `template` dentre o
conjunto phrasing. `typst-realize/src/lib.rs:1043-1065` usa a decisão como
trigger do agrupador PAR.

## Matriz

| fixture | vanilla | cristalino P1171.1 |
|---|---|---|
| `html.a` topo | `<p><a>X</a></p>` | `<a>X</a>` |
| `html.elem("a")` topo | mesmo wrapper | sem wrapper |
| `html.span` topo | `<p><span>X</span></p>` | sem wrapper |
| `span/div/a` | p, div, p | span, div, a |
| phrasing + parbreak + phrasing | dois p | dois nós sem p |
| `a/span` dentro de div | sem p interno | MATCH |
| `a` contendo div | a agrupado em p | sem p externo |

Repr dos dois compiladores preserva os mesmos HtmlElem/Space/Parbreak. Logo a
causa é o exporter L3, não parser, eval, entidade ou pipeline.

## Decisão ADR-0127

Corrigir `block_sequence` para distinguir HtmlElem groupable de boundary block.
É correção interna de paridade, sem API, default ou mudança de fase; segue em
fluxo contínuo. A tabela é mecânica L3; wrapper/boundaries são morfologia.
Custom tags permanecem não groupable. Novos constructors, raw, CSS, MathML e
pretty continuam fora.

Nenhum código, teste, hash, staging ou commit foi alterado na auditoria.
