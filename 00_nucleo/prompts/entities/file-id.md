# FileId — identificador de ficheiro

## Contexto

`FileId` é um handle opaco (`NonZeroU16`) que identifica um ficheiro
no compilador Typst. O original (`path.rs`) usa um interner global
(`static INTERNER: LazyLock<RwLock<...>>`) que viola V13 —
estado mutável global em L1.

## Decisão arquitectural (ADR-0001 Opção A adaptada)

Em L1, `FileId` é apenas o handle — `NonZeroU16` opaco.
O interner (mapeamento de `RootedPath → FileId`) fica em L3.

`VirtualPath`, `RootedPath`, `VirtualRoot` dependem de
`ecow::EcoString` (externo não autorizado em L1) — ficam
fora deste passo.

## Interface pública

```rust
pub struct FileId(NonZeroU16);

impl FileId {
    pub const fn from_raw(v: NonZeroU16) -> Self;
    pub const fn into_raw(self) -> NonZeroU16;
}
```

## Critérios de correcção

- `FileId::from_raw(v).into_raw() == v` para qualquer `NonZeroU16`
- `Copy`, `Clone`, `Eq`, `PartialEq`, `Hash`, `Debug`
- Zero dependências externas
- V13 não dispara
