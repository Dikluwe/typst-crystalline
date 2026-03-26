# Prompt L0 — infra/system-world

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/world.rs`
**ADRs relevantes**: ADR-0001 (comemo/TrackedWorld), ADR-0005 (World trait), ADR-0017 (stubs)

## Contexto

`SystemWorld` é a implementação concreta de `World` (L1) para o
filesystem real. Vive em L3 porque lê ficheiros do disco (`std::fs`)
e terá I/O de fontes (Passo 8) e rede (Passo 9+).

O `World` trait original em `typst-cli` usa `LazyHash<Library>`,
`FontStore`, `FileStore`, `typst-kit` — todos evitados neste passo.
A implementação mínima usa `std::fs` directamente.

## Interface pública

```rust
pub struct SystemWorld { ... }

impl SystemWorld {
    pub fn new(root: PathBuf, main: PathBuf) -> Result<Self, SystemWorldError>
}

impl World for SystemWorld {
    fn library(&self) -> &Library;
    fn book(&self)    -> &FontBook;
    fn main(&self)    -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

## Comportamento

- `library()` — retorna stub `Library(())`
- `book()` — retorna stub `FontBook(())`
- `main()` — `FileId` do ficheiro principal
- `source(id)` — lê do cache ou do disco via `std::fs::read_to_string`;
  retorna `FileError::NotFound` se não existir
- `file(id)` — lê bytes brutos via `std::fs::read`;
  retorna `FileError::NotFound` se não existir
- `font(_)` — `None` (stub — fontes reais no Passo 8)
- `today(_)` — `None` (stub — Datetime real após ADR-0017)

## Critérios de Verificação

```
Dado SystemWorld::new(root, main) com ficheiro existente
Quando main() for chamado
Então FileId válido

Dado SystemWorld com ficheiro existente
Quando source(main_id) for chamado
Então Ok(Source) com text() igual ao conteúdo do ficheiro

Dado SystemWorld
Quando source(id_inexistente) for chamado
Então Err(FileError::NotFound)

Dado SystemWorld
Quando font(0) for chamado
Então None

Dado MockWorld("Hello *world*")
Quando source(main()) for chamado
Então Ok(Source) com text() == "Hello *world*"
```
