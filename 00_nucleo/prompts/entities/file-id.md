# Prompt L0 — `entities/file_id`

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/file_id.rs`
**Criado em**: 2026-03-22 (Passo 1)
**Atualizado em**: 2026-04-12 (restauro — expandido com decisão arquitetural e critérios completos)
**ADRs relevantes**: ADR-0001 (Opção A adaptada — interner em L3, não em L1)

---

## Contexto e Objetivo

Num ecossistema que suporta múltiplos ficheiros e pacotes externos (imports,
módulos), um ficheiro fonte não pode ser identificado apenas pelo seu caminho
local. O `FileId` é a **chave global e unívoca** que representa a origem de
qualquer texto fonte ou nó da AST no motor Cristalino.

`FileId` é um handle opaco (`NonZeroU16`) que identifica um ficheiro no
compilador. Em projetos com muitos ficheiros, o tipo deve ser leve de copiar
e utilizável como chave em caches de alta performance (ex: VFS — Virtual File
System).

---

## Decisão Arquitetural (ADR-0001 adaptada)

O Typst original (`path.rs`) usa um interner global
`static INTERNER: LazyLock<RwLock<...>>` que mapeia `RootedPath → NonZeroU16`.
Isso viola **V13** (estado mutável global em L1).

**Decisão no Cristalino**: em L1, `FileId` é apenas o handle opaco — um
`NonZeroU16` sem semântica de path. O interner (mapeamento de
`RootedPath → FileId`) fica em **L3** onde estado global é permitido.

`VirtualPath`, `RootedPath`, `VirtualRoot` dependem de `ecow::EcoString`
(não autorizado em L1 no parser/entities — ADR-0015) e ficam em L3.

Este design garante que:
- `FileId` é `Copy` e cabe em 2 bytes
- Não há estado global em L1 (V13 não dispara)
- L1 pode usar `FileId` como chave em `HashMap` sem depender de L3
- A resolução de `FileId → path real` é delegada ao `SystemWorld` (L3)

---

## Restrições Estruturais

- Camada **L1**: zero I/O, zero estado global (V13).
- Nenhuma dependência externa — apenas `std::num::NonZeroU16` e `std::fmt`.
- `NonZeroU16` garante que `Option<FileId>` tem o mesmo tamanho que `FileId`
  (null pointer optimization).
- `from_raw`/`into_raw` são a única API de construção — o handle é opaco
  deliberadamente (a semântica de path fica em L3).

---

## Instrução

```rust
/// Handle opaco que identifica um ficheiro no compilador Typst.
///
/// Em L1, apenas o handle — o interner (RootedPath → NonZeroU16) vive em L3.
/// Garante: Copy, 2 bytes, sem estado global, utilizável como chave em cache.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileId(NonZeroU16);

impl FileId {
    /// Constrói a partir de um número raw.
    /// Usar apenas com números obtidos via `into_raw`.
    pub const fn from_raw(v: NonZeroU16) -> Self

    /// Extrai o número raw subjacente.
    pub const fn into_raw(self) -> NonZeroU16
}

impl Debug for FileId  // → "FileId(n)"
```

---

## Critérios de Verificação

```
// roundtrip
FileId::from_raw(NonZeroU16::new(42).unwrap()).into_raw()
    = NonZeroU16::new(42).unwrap()

// igualdade
FileId::from_raw(1) == FileId::from_raw(1)   = true
FileId::from_raw(1) != FileId::from_raw(2)   = true

// Copy
let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
let copy = id;
id == copy                                    = true   // ambos válidos após copy

// Propriedades
Copy + Clone + Eq + PartialEq + Hash + Debug
Zero dependências externas
V13 não dispara (sem LazyLock/RwLock em L1)
```

---

## Relação com outros tipos

- **`Span`** — codifica `FileId` nos 16 bits mais significativos de um `NonZeroU64`
- **`Source`** — em L1 usa `FileId` como chave ao construir a árvore; em L3 mapeia `FileId → path real`
- **`SystemWorld` (L3)** — mantém o interner `HashMap<RootedPath, FileId>` e responde a `world.source(id)`

---

## Resultado Esperado

- `01_core/src/entities/file_id.rs` com `FileId` e testes co-localizados
- Zero violações V13

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 1: handle opaco, interner delegado a L3 | `file_id.rs` |
| 2026-04-12 | Restauro — expandido com decisão ADR-0001, relações com Span/Source, critérios completos | `file-id.md` |
