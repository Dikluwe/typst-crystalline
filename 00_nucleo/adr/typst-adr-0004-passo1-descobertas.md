# ⚖️ ADR-0004: Descobertas do Passo 1 — FileId, ecow, V14 com self::

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-22

---

## Contexto

O Passo 1 migrou `FileId`, `SyntaxKind` e `Span` para
`01_core/entities/`. Três decisões e uma descoberta de linter
foram registadas.

---

## Decisão 1 — Interner global de FileId vai para L3

O `FileId` original em `typst-syntax` usa um interner global
(`static AtomicU16` ou equivalente) para gerar IDs únicos por
ficheiro. V13 (MutableStateInCore) detecta estado global mutável
em L1.

**Decisão**: o interner global é removido de L1. `FileId` em L1
é apenas o valor — um newtype sobre `NonZeroU16`:

```rust
pub struct FileId(NonZeroU16);
```

A geração de IDs únicos (o interner) fica em L3, implementada
como componente injectável. L1 recebe `FileId` como valor opaco
— nunca os gera.

**Impacto**: qualquer código que cria `FileId` a partir de um
path precisa de passar por L3. L1 apenas consome `FileId`.

---

## Decisão 2 — ecow não está em [l1_allowed_external]

`SyntaxNode` e `PackageSpec` dependem de `ecow::EcoString` — uma
string clone-on-write optimizada usada internamente pelo Typst
para eficiência de memória no CST.

`ecow` não foi declarado em `[l1_allowed_external]` no
`crystalline.toml`. V14 vai disparar quando estes módulos forem
migrados.

**Decisão**: avaliar `ecow` antes do Passo 2 e decidir:
- **A**: adicionar `ecow` a `[l1_allowed_external]` — é um
  utilitário de domínio (string especializada), não infraestrutura
- **B**: substituir `EcoString` por `String` ou `Arc<str>` em L1,
  usando `EcoString` apenas em L3 para a representação interna

A opção A é mais pragmática — `ecow` é essencialmente uma
optimização de `Arc<str>` sem I/O. A opção B requer adapters
entre representações. Registar a decisão num ADR de Passo 2.

---

## Decisão 3 — Source depende de parse() → Passo 4

`Source` no Typst original contém o resultado do parse junto com
o texto — não é um valor puro de "texto em memória". A função
`parse()` é chamada internamente no construtor de `Source`.

Como `parse()` é o núcleo do pipeline (Passo 4), `Source` fica
bloqueada até lá. Por agora, `Source` em L1 será um placeholder
com apenas os campos de texto, sem o CST.

---

## Descoberta — V14 dispara para `pub use self::X::Y`

**Comportamento observado**: `pub use self::entities::FileId;`
num `mod.rs` de L1 dispara V14 (ExternalTypeInContract).

**Causa**: o linter resolve `self::entities::FileId` como import
com path `self::entities::FileId`. O prefixo `self::` não é
reconhecido como referência interna à crate — é tratado como
`Layer::Unknown`.

**Padrão correcto em L1**:

```rust
// ❌ Dispara V14
pub use self::entities::FileId;
pub use self::span::Span;

// ✓ Correcto — apenas declarações de módulo
pub mod entities;
pub mod span;
pub mod syntax_kind;
```

Consumers de `typst-core` usam `typst_core::entities::FileId`
directamente. Re-exports via `self::` em `lib.rs` não são
necessários e devem ser evitados em L1.

**Nota**: este é um gap no `RustParser` do `crystalline-lint` —
`self::` deveria ser reconhecido como referência interna. Registar
como issue no repositório do linter para correcção futura. Por
agora, o padrão correcto é não usar re-exports com `self::` em L1.

---

## Decisões do Passo 2 (após diagnósticos)

### ecow — Opção C aprovada (Newtype opaco)

**Decisão revista** após consulta externa (Gemini/rust-analyzer pattern).

A Opção A foi inicialmente recomendada porque `EcoString` aparece
na interface pública de `SyntaxNode`. Mas isso confunde custo de
migração com correcção arquitectural.

`EcoString` é uma optimização de performance do compilador — clone
O(1) para strings do CST. Isso é conhecimento de infraestrutura,
não de domínio. O domínio não sabe nem deve saber que strings
precisam de ser clonadas eficientemente.

**Opção C — Newtype opaco em L1:**

```rust
// 01_core/entities/syntax_text.rs
/// String de domínio para texto de tokens sintácticos.
/// Representação interna opaca — pode mudar sem alterar a interface.
pub struct SyntaxText(Arc<str>);

impl SyntaxText {
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}
```

L1 define o que é uma string de domínio. L3 faz a conversão
`EcoString → Arc<str>` ao construir `SyntaxNode`. Se amanhã
a conversão degradar performance, `SyntaxText` pode adoptar
`ecow` internamente como detalhe privado — sem mudar a interface
pública de L1.

**`ecow` não entra em `[l1_allowed_external]`.**

Interface pública de `SyntaxNode` em L1:
```rust
pub fn text(&self) -> &SyntaxText    // não EcoString
pub fn into_text(self) -> SyntaxText // não EcoString
pub struct SyntaxError {
    pub message: SyntaxText,         // não EcoString
    pub hints: Vec<SyntaxText>,      // não EcoVec<EcoString>
}
```

Em L3, ao construir `SyntaxNode` a partir do parser:
```rust
// Adapter na fronteira L3→L1
impl From<EcoString> for SyntaxText {
    fn from(s: EcoString) -> Self {
        SyntaxText(Arc::from(s.as_str()))
    }
}
```

### PackageSpec — Opção C (DTO pattern) para Passo 3

`serde` e `unscanny` não entram em L1. O padrão correcto:

- **L1**: `PackageSpec` puro, sem derives de `serde`
- **L3**: `PackageSpecDto` com `Serialize`/`Deserialize`
- **Conversão**: L3 faz parse → Dto → `Into<PackageSpec>`

Chato de escrever, mas mantém L1 imune a mudanças em bibliotecas
de terceiros. Análise e implementação no Passo 3.

---

## Estado actual de 01_core/entities/

| Módulo | Tipos | Estado |
|--------|-------|--------|
| `file_id.rs` | `FileId(NonZeroU16)` | ✓ migrado |
| `syntax_kind.rs` | `SyntaxKind` (enum `#[repr(u8)]`) | ✓ migrado |
| `span.rs` | `Span`, `Spanned<T>` | ✓ migrado |
| `syntax_text.rs` | `SyntaxText(Arc<str>)` | ✓ criado (Opção C) |
| `syntax_node.rs` | `SyntaxNode`, `SyntaxError`, `LinkedNode` | ✓ migrado (sem ecow) |
| `syntax_set.rs` | `SyntaxSet` | ✓ migrado |
| `package_spec.rs` | `PackageSpec` | ✗ adiado para Passo 3 |
| `source.rs` | `Source` | ✗ bloqueado (parse(), Passo 4) |

---

## Referências

- ADR-0001 — estratégia de migração, [l1_allowed_external]
- V13 (MutableStateInCore) — estado global em L1
- V14 (ExternalTypeInContract) — externos não autorizados em L1
- crystalline-lint issue: `self::` reconhecido como Layer::Unknown
