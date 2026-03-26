# Passo 7 — SystemWorld em L3

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/0001-estrategia-migracao.md` (Opção C para comemo)
- `00_nucleo/adr/0005-packagespec-world.md` (World trait, TrackedWorld, stubs)
- `00_nucleo/adr/0016-adiamento-eval-typst-library.md`
- `00_nucleo/adr/0019-rustc-hash.md` (revoga ADR-0007 — executar aqui)

Pré-condição: `cargo test -p typst-core` — 159 testes, zero violations.

### Stubs actuais em L1 (o que SystemWorld vai implementar)

| Tipo L1 | Estado | Nota |
|---------|--------|------|
| `Library` | stub `Library(())` | depende de Content — adiado |
| `FontBook` | stub `FontBook(())` | ttf_parser → L3 definitivo |
| `Font` | stub `Font(Vec<u8>)` | rustybuzz → L3 definitivo |
| `Bytes` | stub `Bytes(Vec<u8>)` | suficiente para o pipeline |
| `Datetime` | stub `Datetime { year, month, day }` | time crate → ADR-0017 adiada |
| `Source` | **real** | migrado no Passo 5 |
| `FileId` | **real** | migrado no Passo 1 |
| `FileResult<T>` | **real** | em world_types.rs |

`SystemWorld` em L3 vai implementar `World` de L1 retornando stubs
para os tipos ainda não migrados. Isso é correcto e esperado —
os stubs têm interfaces suficientes para o pipeline compilar e
os testes passarem.

---

## Executar ADR-0018 antes de continuar

ADR-0018 revoga ADR-0007 e reintroduz `rustc_hash` em L1.
Executar agora, antes de criar qualquer ficheiro em L3:

```bash
# 1. Adicionar rustc_hash a [l1_allowed_external] em crystalline.toml
# 2. Adicionar rustc-hash ao workspace Cargo.toml
# 3. Reverter parse.rs: std::collections → rustc_hash::FxHashMap/FxHashSet
# 4. Verificar
cargo test -p typst-core
crystalline-lint .
# ✓ Zero violations, testes mantêm-se
```

---

## Diagnóstico — typst-cli como referência

`typst-cli` é a implementação real de `World` no Typst original.
Usar como referência de interface, não como código a copiar.

```bash
# Onde SystemWorld está definido no original
find lab/typst-original -name "*.rs" | xargs grep -l "impl World for" | head -5

# Interface de SystemWorld original
grep -n "impl World for\|fn library\|fn book\|fn main\|fn source\|fn file\|fn font\|fn today" \
  lab/typst-original/cli/src/world.rs 2>/dev/null | head -30

# Dependências do world.rs original
grep "^use\|^extern" lab/typst-original/cli/src/world.rs 2>/dev/null \
  | grep -v "crate::\|super::\|std::" | head -20

# Estrutura de campos de SystemWorld original
grep -n "^pub struct\|^struct\|    [a-z].*:.*," \
  lab/typst-original/cli/src/world.rs 2>/dev/null | head -30
```

---

## Tarefa 1 — Prompt L0

**Criar**: `00_nucleo/prompts/infra/system-world.md`

`SystemWorld` é a implementação concreta de `World` para o
filesystem real. Vive em L3 porque:
- Lê ficheiros do disco (`std::fs`)
- Carrega fontes de paths do sistema
- Faz I/O de rede para resolver pacotes remotos

Interface mínima para este passo (filesystem + fontes stub):

```rust
pub struct SystemWorld {
    root:    PathBuf,
    sources: /* cache de Source */,
    fonts:   /* FontBook stub */,
    main:    FileId,
}

impl World for SystemWorld {
    fn library(&self) -> &Library;      // retorna stub
    fn book(&self)    -> &FontBook;     // retorna stub
    fn main(&self)    -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;   // retorna None
    fn today(&self, offset: Option<i64>) -> Option<Datetime>; // retorna None
}
```

Critérios de verificação:
```
Dado SystemWorld::new(root_path, main_path)
Quando main() for chamado
Então FileId válido

Dado SystemWorld com ficheiro existente no root
Quando source(id) for chamado com id do ficheiro
Então Ok(Source) com text() correcto

Dado SystemWorld com ficheiro inexistente
Quando source(id) for chamado
Então Err(FileError::NotFound)

Dado SystemWorld
Quando font(0) for chamado
Então None (stub — fontes reais no Passo 8)

Dado SystemWorld
Quando today(None) for chamado
Então None (stub — Datetime real no ADR-0017)
```

---

## Tarefa 2 — Criar SystemWorld em L3

**Criar**: `03_infra/src/world.rs`
**Actualizar**: `03_infra/src/lib.rs` — `pub mod world;`
**Actualizar**: `03_infra/Cargo.toml` — dependências necessárias

### Dependências de L3 para este passo

```toml
# 03_infra/Cargo.toml
[dependencies]
typst-core = { path = "../01_core" }
thiserror  = { workspace = true }
comemo     = { workspace = true }   # ADR-0001 — TrackedWorld
# Adicionar conforme necessário — uma por uma, com comentário de ADR
```

Para leitura de ficheiros (`std::fs`) não é necessária nenhuma
crate extra — `std::fs::read` é suficiente para `Bytes`.

Para cache de `Source` e resolução de `FileId`, uma
`HashMap<PathBuf, FileId>` e `Vec<Source>` são suficientes para
este passo.

### Estrutura mínima

```rust
// 03_infra/src/world.rs
pub struct SystemWorld {
    root:    std::path::PathBuf,
    main:    typst_core::entities::file_id::FileId,
    sources: Vec<typst_core::entities::source::Source>,
    paths:   std::collections::HashMap<std::path::PathBuf,
             typst_core::entities::file_id::FileId>,
    // library e font_book como stubs — substituídos quando migrados
    library: typst_core::entities::world_types::Library,
    font_book: typst_core::entities::world_types::FontBook,
}
```

`SystemWorld::new(root: PathBuf, main: PathBuf)`:
1. Resolver `main` relativo a `root`
2. Criar `FileId` para o ficheiro principal
3. Carregar `Source` do ficheiro principal via `std::fs::read_to_string`
4. Retornar erro se o ficheiro não existir

### Implementação de World

```rust
impl World for SystemWorld {
    fn library(&self) -> &Library {
        &self.library  // stub — Library(())
    }

    fn book(&self) -> &FontBook {
        &self.font_book  // stub — FontBook(())
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        // procurar em self.sources pelo id
        // se não encontrado: ler do disco, criar Source, cachear
        // se ficheiro não existe: Err(FileError::NotFound)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        // ler bytes brutos do ficheiro correspondente ao id
        // std::fs::read — I/O real aqui é correcto (L3)
    }

    fn font(&self, _index: usize) -> Option<Font> {
        None  // stub — fontes reais no Passo 8
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None  // stub — Datetime real após ADR-0017
    }
}
```

### TrackedWorld

`comemo::Tracked<dyn TrackedWorld>` é o que `eval()` vai receber.
O blanket impl em L1 (`impl<T: World> TrackedWorld for T`) garante
que `SystemWorld` já implementa `TrackedWorld` automaticamente.

Verificar que `SystemWorld` é `Send + Sync` — necessário para
`comemo::Tracked`:

```rust
// Se SystemWorld não for Sync automaticamente (ex: por Mutex internos),
// usar Arc<Mutex<...>> nos campos que precisem de mutabilidade.
// Para este passo, todos os campos são imutáveis após construção —
// Sync é automático.
```

---

## Tarefa 3 — MockWorld para testes de L3

Para testar `SystemWorld` sem filesystem real, criar um `MockWorld`
mínimo nos testes:

```rust
#[cfg(test)]
mod tests {
    use typst_core::contracts::world::World;
    use typst_core::entities::{
        file_id::FileId,
        source::Source,
        world_types::*,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        main_id: FileId,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                main_id: id,
                source: Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library { unimplemented!() }
        fn book(&self) -> &FontBook { unimplemented!() }
        fn main(&self) -> FileId { self.main_id }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.main_id {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    #[test]
    fn mock_world_source_roundtrip() {
        let world = MockWorld::new("Hello *world*");
        let src = world.source(world.main()).unwrap();
        assert_eq!(src.text(), "Hello *world*");
    }
}
```

---

## Tarefa 4 — Teste de integração parse→world

```rust
#[test]
fn parse_via_world() {
    use typst_core::rules::parse::parse;
    use typst_core::entities::syntax_kind::SyntaxKind;

    let world = MockWorld::new("= Heading\n\nParagraph.");
    let src = world.source(world.main()).unwrap();

    // Source internamente já chamou parse() — verificar resultado
    assert_eq!(src.root().kind(), SyntaxKind::Markup);
    assert!(!src.root().erroneous());

    // Verificar que Heading existe na árvore
    let has_heading = src.root()
        .children()
        .any(|n| n.kind() == SyntaxKind::Heading);
    assert!(has_heading);
}
```

---

## Verificação final

```bash
# ADR-0019 executada primeiro
cargo test -p typst-core   # 159+ testes
cargo test -p typst-infra  # testes de SystemWorld
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
# V11 warnings para World/TrackedWorld devem DESAPARECER após este passo
# (SystemWorld em L3 satisfaz o contrato)
```

**Critério de conclusão**: V11 warnings de `World`/`TrackedWorld`
eliminados — `SystemWorld` implementa `World` em L3, fechando o
contrato aberto desde o Passo 3.

---

## Ao terminar, reportar

- Se ADR-0019 foi executada sem problemas
- Campos reais de `SystemWorld` (pode diferir da estrutura proposta)
- Se V11 warnings desapareceram
- Se `MockWorld` nos testes compila e passa
- Quais dependências foram adicionadas a `03_infra/Cargo.toml`
- Número total de testes

Esta informação vai para o Passo 8
(fontes reais, FontBook e Font em L3).
