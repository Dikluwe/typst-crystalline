# Span e Spanned — localização na fonte

## Contexto

`Span` é um valor de 8 bytes (`NonZeroU64`) que codifica um
`FileId` (16 bits altos) e um número (48 bits baixos).

Dois sabores:
- **numbered span**: número único por nó AST (estável durante edição)
- **raw range span**: encode de `Range<usize>` (para ficheiros não-Typst)

`Spanned<T>` embrulha qualquer valor com o seu span.

## Origem

`lab/typst-original/crates/typst-syntax/src/span.rs`

Dependências: apenas `std` e `FileId` (L1 interno).

## Interface pública

```rust
pub struct Span(NonZeroU64);
pub struct Spanned<T> { pub v: T, pub span: Span }
```

Métodos: `detached()`, `from_number()`, `from_range()`,
`from_raw()`, `into_raw()`, `is_detached()`, `id()`, `range()`, `or()`, `find()`.

## Critérios de correcção

- `Span::detached().is_detached()` → `true`
- `Span::detached().id()` → `None`
- `Span::from_range(id, 0..10).range()` → `Some(0..10)`
- `Span::from_number(id, n).id()` → `Some(id)`
- `Spanned::new(42, Span::detached()).v` → `42`
