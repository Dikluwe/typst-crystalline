# Prompt L0 — `contracts/world` — O Contrato Supremo do Sistema

**Camada**: L1
**Ficheiro alvo**: `01_core/src/contracts/world.rs`
**Passo de origem**: Passo 1 (trait base), expandido em Passos 8, 15, 22
**ADRs relevantes**: ADR-0005 (padrão B3: World/TrackedWorld), ADR-0001 (comemo em L1),
                     ADR-0019 (ttf-parser → L3)

---

## Contexto e Objetivo

O `World` é o **trait supremo** do motor Cristalino — a fronteira arquitetural
entre o núcleo puro e determinístico (L1) e o sistema físico externo (L3).

O L1 **nunca realiza I/O directamente**. Precisa de fontes, ficheiros e a
data actual? Faz perguntas ao `World`. Quem implementa `World` (o `SystemWorld`
em L3) decide como obter esses dados — do disco, da rede, da memória ou de um
mock.

**Separação de `contracts/mod.rs`**: `mod.rs` é o agregador/facade (declara
`pub mod world`). Este ficheiro (`world.rs`) define o **contrato em si** — o
`trait World` e o `MockWorld` para testes.

---

## O Trait `World`

```rust
pub trait World: Send + Sync {
    /// A biblioteca de funções e valores padrão do Typst.
    /// Inclui as funções nativas registadas pela stdlib.
    fn library(&self) -> &Library;

    /// O catálogo de fontes disponíveis (nome, variante, índice).
    /// Usado pelo Layouter para resolver pedidos de fonte.
    fn book(&self) -> &FontBook;

    /// O ficheiro principal a compilar (ponto de entrada).
    fn main(&self) -> FileId;

    /// Obter o código-fonte de um ficheiro pelo seu id.
    /// Retorna FileError::NotFound se o ficheiro não existe.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Obter o conteúdo binário de um ficheiro (ex: imagens, fontes .ttf).
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Obter uma fonte (bytes + metadados) pelo índice no FontBook.
    /// None se o índice está fora dos limites.
    fn font(&self, index: usize) -> Option<Font>;

    /// A data actual com offset UTC em horas (None se indisponível).
    /// Usa i64 em vez de Duration — o tipo Duration do Typst não existe em L1.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

### Por que `Send + Sync`?

O compilador Typst suporta compilação paralela (Passos futuros). O `World`
deve ser passável entre threads sem `Arc<Mutex<>>` adicional.

---

## Padrão B3: `World` vs `TrackedWorld` (ADR-0005)

```
World (este trait) → o contrato puro: "o quê"
TrackedWorld       → adiciona rastreio incremental via comemo: "como"
```

- `World`: implementado por `SystemWorld` (L3) e `MockWorld` (testes)
- `TrackedWorld`: gerado pelo macro `#[comemo::track]` sobre `World`
- O `eval()` recebe `Tracked<dyn TrackedWorld>` — garante memoização
- O `World` puro compila **sem importar comemo** — confirmado pelo teste
  `world_pure_no_comemo_import_needed()`

---

## MockWorld nos Testes

```rust
struct MockWorld {
    library: Library,
    book:    FontBook,
    main_id: FileId,
}

impl World for MockWorld {
    fn library(&self) -> &Library  { &self.library }
    fn book(&self)    -> &FontBook { &self.book }
    fn main(&self)    -> FileId    { self.main_id }
    fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
    fn file(&self, _: FileId)   -> FileResult<Bytes>  { Err(FileError::NotFound) }
    fn font(&self, _: usize)    -> Option<Font>       { None }
    fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
}
```

O `MockWorld` mínimo — sem I/O, sem fontes reais — permite testar todo o
pipeline de eval sem dependências externas.

---

## Invariantes Críticos

| Regra | Consequência da violação |
|-------|--------------------------|
| L1 nunca importa `std::fs`, `std::net` etc | Compilação falha (import proibido) |
| L1 nunca chama I/O directamente | Teste sem mock quebra |
| `world.source(id)` pode falhar | Caller DEVE tratar `FileResult` |
| `world.font(idx)` pode retornar `None` | Layouter DEVE ter fallback de fonte |
| `today(None)` significa "sem offset" | `today(Some(0))` = UTC exacto |

---

## Critérios de Verificação

```
// Compilabilidade sem comemo
// world.rs não contém: use comemo; / extern crate comemo;
grep "comemo" 01_core/src/contracts/world.rs = vazio

// MockWorld implementa World correctamente
let w = mock()
World::main(&w) = FileId::from_raw(NonZeroU16::new(1).unwrap())
World::source(&w, FileId::from_raw(2)) = Err(FileError::NotFound)
World::file(&w, FileId::from_raw(2))   = Err(FileError::NotFound)
World::font(&w, 0)                     = None
World::today(&w, None)                 = None
World::today(&w, Some(2))              = None

// Trait bounds
// World: Send + Sync — verificado por compilação com mock em thread
std::thread::spawn(|| {
    let w: &dyn World = &mock();
    let _ = w.main();
}).join().unwrap();

// library() retorna Library válida
World::library(&mock()) // não panic

// book() retorna FontBook válido
World::book(&mock()).len() = 0  // mock começa sem fontes
```

---

## Relação com Outros Módulos

| Módulo | Como usa `World` |
|--------|-----------------|
| `rules/eval.rs` (L1) | Recebe `Tracked<dyn TrackedWorld>` — acessa `source()` |
| `contracts/mod.rs` (L1) | Agrega via `pub mod world` |
| `03_infra/src/world.rs` (L3) | Implementa `World` com I/O real (`SystemWorld`) |
| `03_infra/src/integration_tests.rs` (L3) | Cria mocks — não usa `MockWorld` de L1 |
