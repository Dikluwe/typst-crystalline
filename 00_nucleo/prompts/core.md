# Core — typst-core (L1)
Hash do Código: 51a9e58f

Crate de domínio puro do compilador Typst cristalino.

## Módulos existentes (Passo 1)

### `entities/`

Tipos de domínio base migrados de `typst-syntax`:

| Módulo | Tipo(s) | Origem original |
|--------|---------|-----------------|
| `entities/file_id` | `FileId` | `typst-syntax/src/path.rs` |
| `entities/syntax_kind` | `SyntaxKind` | `typst-syntax/src/kind.rs` |
| `entities/span` | `Span`, `Spanned<T>` | `typst-syntax/src/span.rs` |

## Decisões arquitecturais

- `FileId` é apenas o handle opaco (`NonZeroU16`); o interner global
  (`static LazyLock<RwLock<...>>`) do original viola V13 e fica para L3.
- `ecow::EcoString` não está em `[l1_allowed_external]`; tipos que dependem
  dela (`SyntaxNode`, `PackageSpec`, `VirtualPath`) não migram neste passo.
- `comemo` está autorizado em L1 via ADR-0001; não foi necessário neste passo.

## Dependências externas autorizadas

`thiserror`, `comemo` — ver ADR-0001 e `crystalline.toml`.

## Bloqueantes identificados (para Passo 2)

- `SyntaxNode` precisa de `ecow::EcoString` → decisão antes de migrar
- `PackageSpec` precisa de `ecow`, `serde`, `unscanny` → múltiplos externos
- `Source` depende de `parse()` → Passo 4
