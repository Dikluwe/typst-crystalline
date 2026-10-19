# P1173.1 — materialização do segundo lote HTML global-only

**Data:** 2026-08-25  
**Vanilla ratificado:** `a51e02804`  
**Estado:** GREEN, antes de staging

## Proveniência

A medição de fecho foi iniciada em `2026-08-25T14:58:50-03:00`, sobre o commit
`ce49041de76eb64c011e990e9774a0339f979211`, com working tree não commitado. No
momento do registo, `git diff HEAD --stat` continha os dois L0s, L1 HTML, L3
exporter e o teste CLI: 5 ficheiros, 147 inserções e 14 remoções. O passo e este
diagnóstico são ignorados pela política Git do repositório e, por isso, não
aparecem nesse stat. O binário `/usr/local/bin/typst` tinha SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## RED → GREEN

Depois do primeiro resselo (`07cf1e0c` para L1 e `b71ecccc` para L3), os testes
novos falharam como previsto:

- L1: `html.abbr` ausente; 1 falha, 5244 filtrados;
- L3: espaço preservado entre `address` e `aside`; 1 falha, 851 filtrados.

A implementação acrescentou exatamente `abbr`, `address`, `article`, `aside`,
`b`, `bdi`, `bdo`, `cite`, `code`, `dfn`, `i` e `kbd` à tabela tipada. Todos
reutilizam os 76 atributos globais e body content opcional, sem específicos.
No exporter, a normalização local de bodies reconhece os elementos
`display:block` medidos na fonte vanilla e `br`; a lista de void serialization
permanece independente.

Os mesmos testes passaram em GREEN: L1 1/1 e L3 1/1. O teste CLI P1173.1
passou 1/1, com 64 filtrados.

O resselo final propagou `88cdcac8` ao consumidor L1 e `38187406` ao consumidor
L3, terminando com zero drift warnings.

## Validação integral

`cargo test --workspace` passou com 5245 testes core, 852 infra, 55 shell, 2
wiring, 65 CLI e 2 testes do lint; 3 doctests core permaneceram ignorados.
`cargo build --workspace`, `cargo fmt --all -- --check` e `crystalline-lint .`
terminaram com exit 0. Os warnings e infos emitidos são preexistentes e não
bloqueantes.

## Decalque de linguagem

A mesma fixture tipada foi compilada por `/usr/local/bin/typst` e por
`./target/debug/typst`, ambos com `--format html --features html`. `cmp` terminou
com exit 0: atributos, wrappers, conteúdo e whitespace ficaram byte-idênticos.
Essa igualdade é evidência conveniente do observável de linguagem nesta
fixture, não um requisito arquitetural de mecânica Rust.

## Escopo preservado

Nenhuma outra tag pública foi exposta. Raw text, demais void tags, atributos
específicos de outros elementos, CSS, MathML, frame e posições continuam fora
deste passo. O índice permaneceu vazio e nenhum commit foi criado.
