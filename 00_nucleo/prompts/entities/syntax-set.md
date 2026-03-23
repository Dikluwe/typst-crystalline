# SyntaxSet — bitset de SyntaxKind

## Contexto

`SyntaxSet` é um bitset de `SyntaxKind` baseado em `u128`.
Representa conjuntos de tipos sintácticos usados pelo parser
para lookahead e classificação de contexto.

Inspirado no `TokenSet` do rust-analyzer.

## Origem

`lab/typst-original/crates/typst-syntax/src/set.rs`

Zero dependências externas — apenas `SyntaxKind` (L1 interno).

## Interface pública

```rust
pub struct SyntaxSet(u128);

impl SyntaxSet {
    pub const fn new() -> Self;
    pub const fn add(self, kind: SyntaxKind) -> Self;
    pub const fn remove(self, kind: SyntaxKind) -> Self;
    pub const fn union(self, other: Self) -> Self;
    pub const fn contains(&self, kind: SyntaxKind) -> bool;
}
```

Constantes pré-definidas: `STMT`, `MATH_EXPR`, `CODE_EXPR`,
`ATOMIC_CODE_EXPR`, `CODE_PRIMARY`, `ATOMIC_CODE_PRIMARY`,
`UNARY_OP`, `BINARY_OP`, `ARRAY_OR_DICT_ITEM`, `ARG`,
`PARAM`, `DESTRUCTURING_ITEM`, `PATTERN`, `PATTERN_LEAF`.

Macro interna: `syntax_set!(Kind1, Kind2, ...)` para construção
de constantes em tempo de compilação.

## Restrições

- `add` e `remove` só funcionam para `SyntaxKind` com
  discriminador < 128 (assert em tempo de compilação)
- `SyntaxSet` é `Copy` — não tem alocação

## Critérios de correcção

- `SyntaxSet::new().contains(k)` → `false` para qualquer `k`
- `set.add(k).contains(k)` → `true`
- `set.add(k).remove(k).contains(k)` → `false`
- `STMT.contains(SyntaxKind::Let)` → `true`
- `STMT.contains(SyntaxKind::Text)` → `false`
- `UNARY_OP.contains(SyntaxKind::Not)` → `true`
- `CODE_EXPR.contains(SyntaxKind::Ident)` → `true`
