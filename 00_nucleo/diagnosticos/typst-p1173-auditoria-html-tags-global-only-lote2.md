# P1173 — auditoria do segundo lote HTML global-only

**Data:** 2026-08-25  
**Estado:** concluída; parada no gate ADR-0127  
**HEAD:** `9412ad4593608195b3946bf47d48e12ff7dc5f83`  
**Início:** `2026-08-25T14:50:37-03:00`  
**Árvore:** working tree não commitado P1168–P1172.1; índice vazio  
**Vanilla:** `a51e02804`; SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`

## Composição

O inventário P1140.26 foi comparado mecanicamente contra `html.div`. Todas as
12 candidatas têm 77 parâmetros, zero nomes adicionais e zero ausentes: 76
globais mais body.

| tags | específicos | body | categoria |
|---|---:|---|---|
| `address article aside` | 0 | content opcional | block |
| `abbr b bdi bdo cite code dfn i kbd` | 0 | content opcional | phrasing visível |

O lockfile mantém `typst-assets` em `94dcb99`; a fonte gerada não está
materializada, portanto índices `data.rs` não foram inventados. A triangulação
usa lockfile, inventário, `typed.rs:73-114`, `tag.rs` e binário ratificado.

## Sondas e baseline

Para as 12 tags, `type` retornou function; chamada vazia produziu
`elem(tag: "TAG", body: none)`; `id`, `class`, `hidden` foram serializados na
ordem e body `[X]` foi preservado. `href` falhou como `unexpected argument`.
No cristalino P1172.1, todos os 12 bindings são ABSENT.

As nove phrasing no topo produziram um único `<p>` nos dois caminhos,
comprovando a cobertura P1172.1. `article`, `address` e `aside` foram boundaries
block sem wrappers indevidos.

## Achado L3

Na fixture formatada, vanilla produziu
`<article><address>Addr</address><aside>Side</aside></article>`, enquanto o
cristalino preservou um espaço entre os block siblings. O restante coincide.
É extensão da normalização L3 de P1171, não mudança de assinatura ou fase.

## Decisão e gate

A implementação pública é mecânica: 12 wrappers global-only e entradas
estáticas. Nenhuma entidade ou AttrKind muda. P1173.1 também deve obter RED e
corrigir whitespace adjacente a block HtmlElem no exporter.

Solicita-se aprovação para exatamente os 12 bindings listados, cada um com 76
globais e body content opcional. A correção L3 é fluxo contínuo de paridade e
não amplia o consentimento. Todas as outras tags e atributos específicos ficam
fora. Não houve resselo, código, testes, staging ou commit nesta auditoria.
