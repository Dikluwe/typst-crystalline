# Passo 8 — FontBook, Font e lazy-loading em L3 (v2)

## Contexto

Pré-condição: `cargo test` — 170 testes (159 L1 + 11 L3), zero violations.

Diagnóstico concluído externamente. Resultados relevantes:

**`book.rs`**: `FontBook { info: Vec<FontInfo> }` — usa `ttf_parser`
para extracção de metadata, `unicode_segmentation`, `serde`.

**`font/mod.rs`**: `Font { data: Bytes, face: OnceLock<...> }` —
usa `ttf_parser::Face` e `rustybuzz::Face`. Núcleo pesado de shaping.

**`cli/src/world.rs`**: usa `Vec<FontSlot>` com `fontdb` para
descoberta de fontes do sistema. Para este passo, usar apenas
`font_paths: Vec<PathBuf>` explícitos — `fontdb` adiado (ADR-0020).

ADR-0019 (`ttf-parser` e `rustybuzz` em L3) deve ser executada
antes de qualquer código.

---

## Correcção crítica ao Passo 8 v1 — TOCTOU em source()

O padrão proposto na v1 tinha uma falha de concorrência
(Time-of-Check to Time-of-Use):

```rust
// ❌ ERRADO — janela de concorrência entre drop(cache) e leitura
let cache = self.sources.lock().unwrap();
if let Some(src) = cache.get(&id) { return Ok(src.clone()); }
drop(cache);
// outro thread pode entrar aqui com o mesmo id
// ambos lêem o disco, ambos inserem no cache
```

**Solução correcta — padrão Slot com OnceLock**:

O mapa é bloqueado apenas para procurar ou inserir um `SourceSlot`.
O `OnceLock` dentro do slot garante que o disco é lido exactamente
uma vez, mesmo com acesso concorrente ao mesmo `FileId`.

```rust
struct SourceSlot {
    path: PathBuf,
    source: OnceLock<FileResult<Source>>,
}

impl SourceSlot {
    fn get(&self) -> FileResult<Source> {
        self.source.get_or_init(|| {
            let text = std::fs::read_to_string(&self.path)
                .map_err(|_| FileError::NotFound)?;
            // FileId precisa de ser passado — ver estrutura abaixo
            Ok(/* Source::new(id, text) */)
        }).clone()
    }
}
```

O mapa `HashMap<FileId, SourceSlot>` é protegido por `Mutex`
apenas para inserir novos slots. Uma vez que o slot existe no mapa,
o `OnceLock` trata do acesso concorrente sem bloqueio do mapa
inteiro.

---

## Tarefa 0 — Executar ADR-0019

```bash
# Adicionar a 03_infra/Cargo.toml
# ttf-parser e rustybuzz — ver versões actuais no Cargo.toml original
cat lab/typst-original/Cargo.toml | grep -E "ttf-parser|rustybuzz"

cargo build  # verificar que compila
crystalline-lint .  # verificar zero violations
```

---

## Tarefa 1 — SourceSlot e lazy-loading de ficheiros

**Actualizar**: `03_infra/src/world.rs`

Estrutura actualizada de `SystemWorld`:

```rust
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use std::path::PathBuf;
use typst_core::entities::{
    file_id::FileId,
    source::Source,
    world_types::{Bytes, FileError, FileResult, Font, FontBook, Library},
};

struct SourceSlot {
    id:     FileId,
    path:   PathBuf,
    source: OnceLock<FileResult<Source>>,
}

impl SourceSlot {
    fn get(&self) -> FileResult<Source> {
        self.source.get_or_init(|| {
            let text = std::fs::read_to_string(&self.path)
                .map_err(|_| FileError::NotFound)?;
            Ok(Source::new(self.id, text))
        }).clone()
    }
}

pub struct SystemWorld {
    root:        PathBuf,
    main:        FileId,
    // Slots de ficheiros Typst — lazy loading via OnceLock
    slots:       Mutex<HashMap<FileId, SourceSlot>>,
    path_to_id:  Mutex<HashMap<PathBuf, FileId>>,
    next_id:     Mutex<u16>,
    // Fontes — lazy loading via FontSlot (Tarefa 2)
    font_slots:  Vec<FontSlot>,
    font_book:   FontBook,  // populado na construção
    library:     Library,
}
```

`SystemWorld::new(root: PathBuf, main: PathBuf, font_paths: Vec<PathBuf>)`:
1. Resolver `main` relativo a `root`
2. Criar `FileId` para o ficheiro principal (next_id = 1)
3. Criar `SourceSlot` para o ficheiro principal e inserir no mapa
4. Descobrir fontes nos `font_paths` e criar `FontSlot` para cada uma
5. Popular `FontBook` com metadata das fontes descobertas

`source()` actualizado:
```rust
fn source(&self, id: FileId) -> FileResult<Source> {
    let slots = self.slots.lock().unwrap();
    if let Some(slot) = slots.get(&id) {
        // Slot existe — libertar o lock antes de carregar
        let slot_ref = slot as *const SourceSlot;
        drop(slots);
        // SAFETY: slot vive em self.slots que tem lifetime de self
        return unsafe { (*slot_ref).get() };
    }
    // Id desconhecido — não há informação de path para carregar
    Err(FileError::NotFound)
}
```

**Nota**: para importações dinâmicas (ficheiros que o compilador
descobre durante eval), será necessário um método `slot_for_path()`
que cria novos slots. Isso é Passo 9+ — por agora, `NotFound` para
IDs não registados é correcto.

---

## Tarefa 2 — FontSlot e carregamento lazy de fontes

**Criar**: `03_infra/src/fonts.rs`
**Actualizar**: `03_infra/src/lib.rs` — `pub mod fonts;`

```rust
// 03_infra/src/fonts.rs
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fonts.md
//! @prompt-hash <hash>
//! @layer L3
//! @updated 2026-03-23

use std::path::PathBuf;
use std::sync::OnceLock;
use ttf_parser::Face;
use typst_core::entities::world_types::Font;

/// Slot de fonte com carregamento lazy.
/// A fonte só é carregada do disco na primeira chamada a `get()`.
pub struct FontSlot {
    pub path:  PathBuf,
    pub index: u32,         // índice em TrueType collections (.ttc)
    font:      OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn new(path: PathBuf, index: u32) -> Self {
        Self { path, index, font: OnceLock::new() }
    }

    /// Carrega a fonte do disco (apenas na primeira chamada).
    /// Retorna None se o ficheiro não existir ou não for válido.
    pub fn get(&self) -> Option<Font> {
        self.font.get_or_init(|| {
            let data = std::fs::read(&self.path).ok()?;
            // Validar que é uma fonte OpenType/TrueType válida
            Face::parse(&data, self.index).ok()?;
            // Passar bytes opacos para L1 — ttf_parser não escapa a fronteira
            Some(Font(data))
        }).clone()
    }
}

/// Descobre fontes nos paths fornecidos.
/// Retorna Vec<FontSlot> com uma entrada por face de fonte encontrada.
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot> {
    let mut slots = Vec::new();
    for path in font_paths {
        if path.is_dir() {
            // Iterar recursivamente sobre .ttf, .otf, .ttc
            discover_fonts_in_dir(path, &mut slots);
        } else if is_font_file(path) {
            // Ficheiro único — pode ter múltiplas faces (.ttc)
            let count = face_count(path).unwrap_or(1);
            for index in 0..count {
                slots.push(FontSlot::new(path.clone(), index));
            }
        }
    }
    slots
}

fn is_font_file(path: &std::path::Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("ttf" | "otf" | "ttc" | "otc")
    )
}

fn face_count(path: &std::path::Path) -> Option<u32> {
    let data = std::fs::read(path).ok()?;
    // ttf_parser pode determinar quantas faces existem num .ttc
    Some(ttf_parser::fonts_in_collection(&data).unwrap_or(1))
}

fn discover_fonts_in_dir(dir: &std::path::Path, slots: &mut Vec<FontSlot>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            discover_fonts_in_dir(&path, slots);
        } else if is_font_file(&path) {
            let count = face_count(&path).unwrap_or(1);
            for index in 0..count {
                slots.push(FontSlot::new(path.clone(), index));
            }
        }
    }
}
```

---

## Tarefa 3 — FontBook populado com metadata real

`FontBook` em L1 é ainda `FontBook(())` — stub opaco. Para este
passo, `SystemWorld` mantém internamente a informação de fontes
em `font_slots` e `FontBook` permanece como stub.

`world.book()` continua a retornar `&self.font_book` (stub).
Quando `FontBook` real migrar para L1, este método passa a retornar
informação real. Por agora, o pipeline compila sem `FontBook` real.

---

## Tarefa 4 — Actualizar SystemWorld::font()

```rust
fn font(&self, index: usize) -> Option<Font> {
    self.font_slots.get(index)?.get()
}
```

---

## Tarefa 5 — Prompts L0

**Criar**: `00_nucleo/prompts/infra/fonts.md`
Documentar: `FontSlot`, `discover_fonts`, fronteira `Font(Vec<u8>)`.

**Actualizar**: `00_nucleo/prompts/infra/system-world.md`
Reflectir: `SourceSlot` com `OnceLock`, `font_paths`, `FontSlot`.

---

## Verificação final

```bash
cargo test -p typst-core   # 159 — não devem regredir
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Testes obrigatórios:

```rust
// Em 03_infra/src/world.rs #[cfg(test)]

#[test]
fn source_ficheiro_principal_disponivel() {
    // Criar ficheiro temporário e SystemWorld
    // world.source(world.main()) → Ok com texto correcto
}

#[test]
fn source_id_desconhecido_retorna_not_found() {
    // FileId sem slot registado → Err(FileError::NotFound)
}

#[test]
fn source_chamada_repetida_usa_cache() {
    // Duas chamadas ao mesmo id → mesmo texto, sem I/O duplo
    // (verificar via contagem de chamadas ou timestamp de modificação)
}

// Em 03_infra/src/fonts.rs #[cfg(test)]

#[test]
fn font_slot_ficheiro_invalido_retorna_none() {
    let slot = FontSlot::new(PathBuf::from("/nao/existe.ttf"), 0);
    assert!(slot.get().is_none());
}

#[test]
fn discover_fonts_directorio_vazio() {
    let dir = tempdir(); // directório temporário vazio
    assert!(discover_fonts(&[dir.path().to_path_buf()]).is_empty());
}
```

Para testes de fontes reais, incluir um ficheiro `.ttf` minimal
em `03_infra/tests/fixtures/` — pode ser um subset do Liberation
Sans ou similar com licença permissiva.

---

## Ao terminar, reportar

- Versões de `ttf-parser` e `rustybuzz` adicionadas
- Se `SourceSlot` com `OnceLock` compilou sem problemas de Send+Sync
- Se `font()` retorna `Some(Font)` para fontes reais (com fixture)
- Se `source()` para ficheiros importados ainda retorna `NotFound`
  (esperado — lazy registration é Passo 9)
- Número total de testes
- Zero violations confirmado

Esta informação determina se o pipeline está pronto para
`eval()` end-to-end com documentos simples.
