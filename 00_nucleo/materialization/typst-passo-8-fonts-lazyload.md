# Passo 8 — FontBook, Font e lazy-loading em L3

## Contexto

Pré-condição: `cargo test` — 170 testes (159 L1 + 11 L3), zero violations.

O Passo 7 criou `SystemWorld` com três limitações deliberadas:
1. `source()` só serve o ficheiro principal — ficheiros importados retornam `NotFound`
2. `font()` retorna `None` — sem carregamento de fontes
3. `FontBook` e `Font` são stubs opacos em L1

Este passo resolve as limitações 1 e 2. A limitação 3 (stubs em L1)
é parcialmente resolvida: `FontBook` e `Font` ganham implementações
reais em L3, mas os stubs em L1 mantêm-se — L3 usa os seus próprios
tipos internos e converte na fronteira.

---

## Diagnóstico obrigatório

```bash
# FontBook original — onde vive e o que contém
find lab/typst-original -name "book.rs" | grep font
grep -n "^pub struct FontBook\|^pub fn\|^impl FontBook" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs | head -30

# Dependências de book.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Font original
grep -n "^pub struct Font\|^pub fn\|^impl Font" \
  lab/typst-original/crates/typst-library/src/text/font/mod.rs | head -30

# Dependências de font/mod.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/text/font/mod.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como typst-cli carrega fontes
grep -n "font\|FontBook\|book" \
  lab/typst-original/cli/src/world.rs | head -40

# Paths de fontes do sistema (onde typst-cli procura)
grep -n "font_path\|system_font\|font_dir\|fonts()" \
  lab/typst-original/cli/src/world.rs | head -20
```

**Reportar output antes de continuar.**

Se `book.rs` ou `font/mod.rs` tiverem externos não cobertos pelas
ADRs 0001–0018 — criar ADR (0019+) antes de continuar.

---

## Tarefa 1 — Lazy-loading de ficheiros em SystemWorld

**Problema actual**: `SystemWorld::source()` só serve o ficheiro
principal carregado em `new()`. Ficheiros importados (`#import`)
retornam `FileError::NotFound`.

**Solução**: lazy-loading com cache protegido por `Mutex`.

```rust
// 03_infra/src/world.rs — estrutura actualizada
use std::sync::Mutex;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct SystemWorld {
    root:      PathBuf,
    main:      FileId,
    // Cache lazy: FileId → Source
    sources:   Mutex<HashMap<FileId, Source>>,
    // Mapeamento path → FileId (para resolver imports)
    file_ids:  Mutex<HashMap<PathBuf, FileId>>,
    // Contador para geração de FileId únicos
    next_id:   Mutex<u16>,
    // Stubs mantêm-se
    library:   Library,
    font_book: FontBook,  // substituído na Tarefa 2
}
```

`source()` actualizado:
```rust
fn source(&self, id: FileId) -> FileResult<Source> {
    let cache = self.sources.lock().unwrap();
    if let Some(src) = cache.get(&id) {
        return Ok(src.clone());
    }
    drop(cache);

    // Resolver path a partir do id
    // Ler do disco
    // Criar Source, inserir no cache, retornar
}
```

Para resolver `FileId → PathBuf` e `PathBuf → FileId`, manter
o mapeamento inverso em `file_ids`.

`file()` (bytes brutos) também beneficia do mesmo padrão de cache
se ficheiros binários forem lidos repetidamente.

**Nota sobre Send + Sync**: `Mutex<T>` torna `SystemWorld` `Sync`
automaticamente se `T: Send`. `HashMap`, `Source`, `PathBuf` são
todos `Send` — sem problema.

---

## Tarefa 2 — FontBook e Font em L3

Com base no diagnóstico, criar implementações reais em L3.

### Se ttf_parser for necessário (esperado)

Adicionar a `03_infra/Cargo.toml`:
```toml
ttf-parser = "0.x"  # ADR-0019 (criar se necessário)
```

Criar ADR antes de adicionar — `ttf_parser` é L3 definitivo
(parsing de ficheiros binários de fonte).

### FontSlot — padrão de carregamento lazy de fontes

O Typst original usa um padrão de slot para carregamento lazy:

```rust
// 03_infra/src/fonts.rs
struct FontSlot {
    path:   PathBuf,
    index:  u32,        // índice dentro do ficheiro (TrueType collection)
    font:   OnceLock<Option<Font>>,  // carregado na primeira chamada
}
```

`SystemWorld::font(index)` indexa um `Vec<FontSlot>` e carrega
na primeira chamada via `OnceLock`.

### Paths de fontes do sistema

O Typst original procura fontes em:
- Paths fornecidos pelo utilizador
- Paths do sistema (`/usr/share/fonts`, `~/.fonts`, etc. em Linux;
  equivalentes em macOS e Windows)
- Fontes embebidas (subset de fontes Typst)

Para este passo, suporte mínimo:
- Paths fornecidos em `SystemWorld::new(root, main, font_paths)`
- Opcional: paths do sistema se `fontdb` ou equivalente for usado

### Prompt L0 para fonts.rs

**Criar**: `00_nucleo/prompts/infra/fonts.md`

Documentar:
- `FontSlot` com `OnceLock` para lazy-loading
- Interface de descoberta de fontes (paths → slots)
- Conversão `ttf_parser::Face → Font stub de L1` na fronteira L3→L1

---

## Tarefa 3 — Actualizar SystemWorld::font()

Após `FontSlot` existir:

```rust
fn font(&self, index: usize) -> Option<Font> {
    self.font_slots.get(index)?.load()
}
```

Onde `FontSlot::load()` abre o ficheiro, faz parse com `ttf_parser`,
e converte para o `Font` de L1 (ainda stub — `Font(Vec<u8>)` com
os bytes raw da fonte).

**Fronteira L3→L1**: `Font(Vec<u8>)` em L1 recebe os bytes raw
da fonte. Quando `Font` real migrar para L1 (após análise de
`rustybuzz`), a fronteira muda sem alterar a interface de `World`.

---

## Tarefa 4 — Actualizar Prompt L0 de SystemWorld

**Actualizar**: `00_nucleo/prompts/infra/system-world.md`

Reflectir:
- Lazy-loading com `Mutex`
- `font_paths` como parâmetro de construção
- `FontSlot` como detalhe de implementação

---

## Verificação final

```bash
cargo test -p typst-core   # 159 testes — não devem regredir
cargo test -p typst-infra  # testes de SystemWorld + fonts
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Testes obrigatórios para lazy-loading:
```rust
#[test]
fn source_ficheiro_inexistente() {
    use std::num::NonZeroU16;
    let world = SystemWorld::new(root, main).unwrap();
    let id = FileId::from_raw(NonZeroU16::new(99).unwrap());
    assert!(matches!(world.source(id), Err(FileError::NotFound)));
}

#[test]
fn source_ficheiro_principal_cached() {
    let world = SystemWorld::new(root, main).unwrap();
    let id = world.main();
    // Segunda chamada — deve vir do cache, não do disco
    let s1 = world.source(id).unwrap();
    let s2 = world.source(id).unwrap();
    assert_eq!(s1.text(), s2.text());
}
```

Para testes de fontes, usar um ficheiro `.ttf` minimal de teste
(pode ser gerado programaticamente com `ttf_parser` ou incluído
como fixture em `03_infra/tests/fixtures/`).

---

## Ao terminar, reportar

- Externos novos adicionados a `03_infra/Cargo.toml` e ADRs criadas
- Se lazy-loading com `Mutex` compilou sem problemas de Send+Sync
- Se `font()` retorna `Some(Font)` para fontes reais ou continua stub
- Número total de testes
- Zero violations confirmado

Esta informação determina se `eval()` pode avançar com um
`SystemWorld` suficientemente completo para compilar documentos
simples (sem fontes especiais).
