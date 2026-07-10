# Prompt L0 — infra/system-world
Hash do Código: 81c2aa25

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
    /// **P450** — carrega e parseia um ficheiro `.bib` relativo a current_file.
    pub fn load_bibliography(&self, current_file: FileId, path: &str) -> Result<Vec<BibEntry>, String>
}

impl World for SystemWorld {
    fn library(&self) -> &Library;
    fn book(&self)    -> &FontBook;
    fn main(&self)    -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String>;
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

## Resolução de pacotes (P681, P-β de P678)

`resolve_package(spec)` procura o pacote **offline**, sem rede, na cache local do
utilizador, seguindo a ordem de prioridade confirmada em P678:

1. **Data dir** — `$XDG_DATA_HOME/typst/packages` ou `~/.local/share/typst/packages`.
2. **Cache dir** — `$XDG_CACHE_HOME/typst/packages` ou `~/.cache/typst/packages`.

Para cada base, o candidato é `{base}/{namespace}/{name}/{version}` (ex.:
`~/.cache/typst/packages/preview/cetz/0.2.2`). O primeiro que existir como
directório é usado:

1. Lê `{candidato}/typst.toml` e extrai `[package].entrypoint` (via `toml::Value`;
   `toml` é workspace dep adicionada a `03_infra/Cargo.toml`). Sem `entrypoint`
   → erro claro.
2. `entrypoint_path = candidato.join(entrypoint)`; `id = register_file(entrypoint_path)`
   (FileId estável, canonicalizado); `source(id)` devolve o `Source` do entrypoint.
   Imports internos do pacote resolvem relativamente ao `current_file` (entrypoint),
   cujo `directory_of` é o directório do entrypoint dentro da cache — logo os
   `#import "..."` internos ao pacote funcionam sem tratamento especial.

Erros (todos `Result::Err` com mensagem clara, distinta de "pacotes não suportados"):

- Pacote não presente em nenhuma base →
  `pacote '@preview/nome:versao' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)`.
- `typst.toml` ilegível/inválido ou sem `[package].entrypoint` → mensagem específica.
- Entrypoint não existe no disco → mensagem específica.

É **I/O em L3** (permitido): lê o filesystem fora do projecto (cache do utilizador).
L1 (`World` trait, `eval_module_import`) só declara/consome o contrato.

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
