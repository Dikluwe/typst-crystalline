# SyntaxKind — tipo dos nós da árvore sintática

## Contexto

`SyntaxKind` é um enum `#[repr(u8)]` que classifica cada nó
da árvore sintática do Typst. Copiado directamente de
`lab/typst-original/crates/typst-syntax/src/kind.rs`.

## Origem

`lab/typst-original/crates/typst-syntax/src/kind.rs`

Nenhuma dependência externa — zero alterações necessárias.

## Interface pública

Enum com variantes para markup, math e code mode.
Métodos booleanos: `is_keyword`, `is_trivia`, `is_grouping`,
`is_terminator`, `is_block`, `is_stmt`, `is_error`.
Método `name() -> &'static str`.

## Critérios de correcção

- Compilação com zero erros
- `SyntaxKind::Let.is_keyword()` → `true`
- `SyntaxKind::Text.is_keyword()` → `false`
- `SyntaxKind::Error.is_error()` → `true`
- `SyntaxKind::Space.is_trivia()` → `true`
- `SyntaxKind::Text.name()` → `"text"`
