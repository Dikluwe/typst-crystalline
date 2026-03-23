# World e TrackedWorld — contratos de ambiente de compilação

**Camada**: L1 — contracts
**Criado em**: 2026-03-22
**Arquivos gerados**: `01_core/src/contracts/world.rs`

---

## Contexto

`World` é o contrato entre o compilador Typst e o ambiente de
execução — filesystem, fontes, biblioteca padrão, hora actual.
É o ponto de inversão de dependência central: L1 declara o que
precisa, L3 implementa como obter.

O original em `lab/typst-original/crates/typst/src/lib.rs` usa
`#[comemo::track]` directamente na trait. Em L1 cristalina,
separamos em dois contratos (ADR-0005, padrão B3):

- `World`: contrato puro, sem `comemo`, testável com mocks simples
- `TrackedWorld`: estende `World` com `#[comemo::track]` e blanket impl

**Verificação empírica obrigatória**: antes de escrever código,
verificar que `#[comemo::track]` em `TrackedWorld` com blanket impl
gera rastreio funcional. Ver secção de verificação abaixo.

---

## Restrições Estruturais

- `World` não usa `comemo` de forma alguma
- `TrackedWorld` usa `comemo` — autorizado via ADR-0001
- Todos os tipos nos métodos devem existir em L1 antes de compilar
  (usar stubs de `world_types.rs` — ver prompt correspondente)
- `offset: Option<i64>` em vez de `Option<Duration>` — `Duration`
  do Typst não existe em L1 neste passo
- V11 (DanglingContract) vai disparar até L3 ter implementação —
  adicionar a `[orphan_exceptions]` temporariamente

---

## World — contrato puro

```rust
/// Contrato entre o compilador Typst e o ambiente de execução.
/// Sem comemo — testável com qualquer impl simples.
pub trait World: Send + Sync {
    /// A biblioteca de funções e valores padrão do Typst.
    fn library(&self) -> &Library;

    /// O catálogo de fontes disponíveis.
    fn book(&self) -> &FontBook;

    /// O ficheiro principal a compilar.
    fn main(&self) -> FileId;

    /// Obter o source de um ficheiro pelo seu id.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Obter o conteúdo binário de um ficheiro pelo seu id.
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Obter uma fonte pelo índice no FontBook.
    fn font(&self, index: usize) -> Option<Font>;

    /// A data actual com offset em horas (None se indisponível).
    /// Usa i64 em vez de Duration — Duration do Typst não existe em L1.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

---

## TrackedWorld — contrato de performance

```rust
/// Extensão de World com rastreio de acessos para memoização incremental.
/// comemo autorizado em L1 via ADR-0001.
///
/// VERIFICAÇÃO EMPÍRICA: confirmar que #[comemo::track] aqui gera
/// rastreio funcional através do blanket impl antes de usar no pipeline.
#[comemo::track]
pub trait TrackedWorld: World {
    fn library(&self) -> &Library;
    fn book(&self) -> &FontBook;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId) -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}

/// Qualquer World é automaticamente TrackedWorld.
/// Compatibilidade com o ecossistema Typst sem mudanças nos implementadores.
/// Dessincronização com World detectada em tempo de compilação.
impl<T: World> TrackedWorld for T {
    fn library(&self) -> &Library      { World::library(self) }
    fn book(&self) -> &FontBook        { World::book(self) }
    fn source(&self, id: FileId) -> FileResult<Source> { World::source(self, id) }
    fn file(&self, id: FileId) -> FileResult<Bytes>    { World::file(self, id) }
    fn font(&self, index: usize) -> Option<Font>       { World::font(self, index) }
    fn today(&self, offset: Option<i64>) -> Option<Datetime> { World::today(self, offset) }
}
```

---

## Verificação empírica de TrackedWorld

Antes de escrever código de produção, criar um teste que confirma
que o rastreio funciona através do blanket impl:

```rust
#[cfg(test)]
mod tracking_verification {
    use super::*;

    struct MockWorld {
        library: Library,
        book: FontBook,
        main_id: FileId,
    }

    impl World for MockWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { self.main_id }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    #[test]
    fn mock_world_is_tracked_world() {
        // MockWorld implementa World → deve satisfazer TrackedWorld via blanket
        fn requires_tracked(_: &dyn TrackedWorld) {}
        let w = MockWorld { ... };
        requires_tracked(&w); // se compilar, blanket impl funciona
    }
}
```

Se este teste compilar, o blanket impl está funcional.
Se não compilar, reportar ao developer — a implementação de
`TrackedWorld` precisa de ser revista antes de avançar.

---

## Critérios de Verificação

```
Dado MockWorld que implementa World
Quando usado como &dyn TrackedWorld
Então compila sem erros — blanket impl funciona

Dado MockWorld::library() → &Library
Quando chamado via World trait
Então retorna a referência correcta

Dado MockWorld::source(id) → Err(NotFound)
Quando chamado via World trait
Então retorna FileError::NotFound

Dado World pura sem comemo
Quando os testes de L1 correm
Então não importam comemo — testabilidade confirmada

Dado qualquer struct que implementa World
Quando usada onde TrackedWorld é requerida
Então satisfaz automaticamente via blanket impl
```

---

## orphan_exceptions temporário

Enquanto L3 não tiver implementação de `World`:

```toml
# crystalline.toml
[orphan_exceptions]
"01_core/src/contracts/world.rs" = "implementação em L3, Passo 5"
```

Remover quando `SystemWorld` ou equivalente for migrado para L3.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação inicial — B3 + blanket impl (ADR-0005) | world.rs |
