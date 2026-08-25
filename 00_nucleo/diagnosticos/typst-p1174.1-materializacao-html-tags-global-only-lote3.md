# P1174.1 — materialização do terceiro lote HTML global-only

**Data:** 2026-08-25
**Estado:** GREEN, antes de staging
**Vanilla ratificado:** `a51e02804`

## Proveniência

A medição de fecho foi registrada em `2026-08-25T15:10:48-03:00`, sobre o
commit `ce49041de76eb64c011e990e9774a0339f979211`, com working tree não
commitado. `git diff HEAD --stat` reportou 6 ficheiros rastreados, 321
inserções e 19 remoções; documentos ainda untracked não entram nesse stat. O
vanilla `/usr/local/bin/typst` tinha SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`; o
cristalino reconstruído tinha
`5760b826461b8dbe07085d9a350c8744eaad89d4b530fed6885ddc5a81b49f0e`.

## RED → GREEN

Após aprovação do gate e primeiro resselo (`d6fe85a3`), o teste L1 novo falhou
em `html.dd` ausente: 1 falha e 5245 testes filtrados. Foram acrescentados
exatamente `dd`, `dl`, `dt`, `figcaption`, `figure`, `footer`, `header`,
`hgroup`, `legend`, `main`, `mark` e `menu` como wrappers de
`native_typed_html(tag, &[], args)` e entradas em `TYPED_TAGS`.

O mesmo teste passou 1/1 em GREEN, com 5245 filtrados. Ele também prova
sentinelas globais, body Content e rejeição de `href`. O teste CLI passou 1/1,
com 65 filtrados, cobrindo repr e DOM estrutural do lote.

O resselo final propagou `0e2ccb7f` ao consumidor L1 e terminou com zero drift
warnings.

## Decalque

A fixture tipada de P1174 foi compilada pelo vanilla e pelo cristalino com
`--format html --features html`. `cmp` terminou com exit 0. Assim, bindings,
atributos, nesting, grouping phrasing de `mark` e boundaries block das outras
onze tags coincidem no observável medido. Nenhuma mudança L3 foi necessária.

## Escopo preservado

Nenhuma tag além das 12 aprovadas foi exposta. A entidade `HtmlElem`, o
exporter, raw, demais void tags, atributos específicos, frame, CSS, MathML e
positions permaneceram inalterados por P1174.1. O índice permaneceu vazio e
nenhum commit foi criado.

## Validação integral

`cargo test --workspace` passou com 5246 testes core, 852 infra, 55 shell, 2
wiring, 66 CLI e 2 testes do lint; 3 doctests core permaneceram ignorados.
`cargo build --workspace`, `cargo fmt --all -- --check`, `crystalline-lint .`
e `git diff --check` terminaram com exit 0. Permaneceram somente warnings e
infos preexistentes não bloqueantes.
