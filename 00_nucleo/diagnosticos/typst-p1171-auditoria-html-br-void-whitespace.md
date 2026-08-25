# P1171 — auditoria de `html.br`, void e whitespace

**Data:** 2026-08-25  
**Estado:** concluída; parada no gate ADR-0127  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Início:** `2026-08-25T14:27:12-03:00`  
**Árvore:** working tree não commitado P1168–P1170.1; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Fonte e assinatura

`lab/typst-original/crates/typst-html/src/tag.rs:123-141` enumera 13 tags void:
`area base br col embed hr img input link meta source track wbr`.
`typed.rs:73-114` omite body para void.

| sonda | vanilla | cristalino P1170.1 | estado |
|---|---|---|---|
| `type(html.br)` | function | campo ausente | ABSENT |
| `html.br()` | `elem(tag: "br")` | ausente | ABSENT |
| `html.br(id: "b")` | attr preservado | ausente | ABSENT |
| `html.br[X]` | unexpected argument | ausente | ABSENT |
| `html.elem("br")` | mesmo repr | mesmo repr | MATCH eval |
| `html.elem("br")[X]` | constrói; export rejeita | `<br>X</br>` | PARTIAL |

`br` possui zero específicos, reutiliza os 76 globais e não possui body. A
representação atual é suficiente: typed void deve produzir `HtmlBody::Unset`.

## Serialização void

A fixture ratificada produziu `<div>A<br>B<br id="b">C</div>`. Vanilla não
usa slash XHTML nem end tag. Body em `html.elem("br")[X]` falhou no export com
`HTML void elements must not have children`. O cristalino fecha todo HtmlElem
genericamente. `Content::Linebreak` já produz `<br>` e não deve regredir.

Decisão: tabela interna única das 13 tags consumida por L3; nenhum campo/flag
público em `HtmlElem`. O binding L1 escolhe constructor sem body.

## Whitespace

As sondas repr dos dois binários contêm `Content::Space` entre expressões em
linhas diferentes. A camada causal é expansão/export L3, não parser/eval.

| fixture | vanilla DOM | cristalino DOM |
|---|---|---|
| dois `span` mesma linha | sem espaço | igual |
| dois `span` formatados em `div` | um espaço entre spans, zero nas bordas | espaços extras nas bordas |
| dois `div` em linhas | zero espaço entre divs | igual |
| texto/`br` formatado | `<div>A<br>B</div>` | binding ausente |

Inferência: realização HTML remove whitespace de borda de corpos normais e
whitespace adjacente a `br`, preservando espaço entre inline siblings.
Refutador: matriz ampliada mostrar dependência de categoria/tag ou sentinelas
não representadas no Content. P1171.1 deve manter testes diferenciais e evitar
trim global.

## Decisão e gate

Binding, ausência de body, repr, diagnóstico de filho void e DOM são língua;
tabela Rust e lookup são mecânica. Solicita-se aprovação para exatamente:

1. binding público `html.br`, 76 globais, zero específicos e zero body;
2. `HtmlBody::Unset`, sem mudança de entidade;
3. tabela interna das 13 tags void em L3, emissão sem end tag e erro para body;
4. correção interna L3 do whitespace local medido, sem mudança de fase;
5. agrupamento phrasing de topo explicitamente fora.

Não houve alteração L1–L4, teste, hash, staging ou commit nesta auditoria.
