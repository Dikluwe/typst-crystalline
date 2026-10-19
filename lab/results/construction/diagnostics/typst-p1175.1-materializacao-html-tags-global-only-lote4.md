# P1175.1 — quarto lote HTML e proteção de espaço

**Data:** 2026-08-25
**Estado:** GREEN, antes de staging
**Vanilla:** `a51e02804`

## Proveniência

O fecho foi medido em `2026-08-25T15:22:56-03:00`, sobre o commit
`ce49041de76eb64c011e990e9774a0339f979211`, com working tree não commitado.
`git diff HEAD --stat` reportou 6 ficheiros rastreados, 582 inserções e 24
remoções; documentos untracked não aparecem no stat. O vanilla tinha SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`; o
cristalino reconstruído, `bb2c159e8867c590f35eb86de7a7afbb869d65809647139b73166333982b8800`.

## RED → GREEN

O primeiro resselo propagou `77cb66c7` ao L1 e `20cb2a91` ao L3. Os REDs
falharam exatamente em `html.nav` ausente (1 falha, 5246 filtrados) e espaço
literal entre dois `span` vazios (1 falha, 852 filtrados).

Foram acrescentados exatamente `nav`, `picture`, `pre`, `s`, `samp`, `search`,
`section`, `small`, `sub`, `sup`, `u`, `var` como wrappers global-only e
entradas em `TYPED_TAGS`. O teste L1 passou 1/1 e cobre globals/body/rejeição de
específico.

Em L3, a análise local de espaço distingue boundary, inline vazio e conteúdo
supportive. Block/`br` encerram o contexto; replaced elements e conteúdo
visível sustentam espaço; inline vazio não sustenta. Somente quando há inline
dos dois lados e falta suporte completo é emitido
`<span style="white-space: pre-wrap">&#x20;</span>`. Espaços de borda são removidos
e espaços entre inline com conteúdo permanecem literais. Os cinco testes HTML
P117x passaram juntos; o teste CLI P1175.1 também passou.

## Decalque

As fixtures principal e adversarial foram compiladas pelos dois binários com
`--format html --features html`. Os dois `cmp` terminaram com exit 0. Isso
cobre grouping block/phrasing, constructors tipados, `pre`, `picture` vazio,
proteção de espaço e ausência de proteção indevida nas formas medidas.

O resselo final propagou `53b80b37` ao L1 e `3fe1098c` ao L3, com zero drift
warnings.

## Escopo

Nenhuma outra tag foi exposta. `HtmlElem`, target e pipeline não mudaram. As
16 global-only restantes, específicos, outras void/raw, frame, CSS, MathML e
positions continuam fora. Índice vazio; nenhum commit criado.

## Validação integral

`cargo test --workspace` passou com 5247 testes core, 853 infra, 55 shell, 2
wiring, 67 CLI e 2 testes do lint; 3 doctests core permaneceram ignorados.
`cargo build --workspace`, `cargo fmt --all -- --check`, `crystalline-lint .`
e `git diff --check` terminaram com exit 0; restaram somente warnings e infos
preexistentes não bloqueantes.
