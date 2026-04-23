# Passo 112.B — Inventário das APIs L3 consumíveis pelo 04_wiring

**Data**: 2026-04-23
**Propósito**: mapear o que o 04_wiring pode invocar para compor
uma CLI real.

---

## APIs públicas do `03_infra`

### `world.rs` — `SystemWorld`

```rust
pub struct SystemWorld { /* root, main, slots, font_slots, font_book, library */ }

impl SystemWorld {
    pub fn new(root: impl Into<PathBuf>, main: impl AsRef<Path>) -> Result<Self, SystemWorldError>;
    pub fn with_fonts(self, font_slots: Vec<FontSlot>) -> Self;
    pub fn register_file(&self, path: PathBuf) -> FileId;
    pub fn root(&self) -> &Path;
}

impl World for SystemWorld { /* library, book, main, source, file, font, today */ }

pub struct SystemWorldError { /* MainNotFound, Io, ... */ }
```

**Crítico**: SystemWorld é um `World` **real, production-ready**.
Lê ficheiros do filesystem, cache de sources, gere FileIds,
suporta fontes via `with_fonts`. **Não é MockWorld.**

### `export.rs` — PDF

```rust
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8>;
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8>;
pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String>;
```

- `export_pdf` — PDF básico com Helvetica Type1 (Latin-1 apenas).
- `export_pdf_with_font` — TTF embebida, suporte Unicode completo
  (ADR-0027).

### `layout.rs` — Layout com fontes

```rust
pub fn layout_with_font(
    content: &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument;
```

Wrapper que escolhe entre `FontBookMetrics` (se bytes válidos) e
`FixedMetrics` (L1 fallback).

### `fonts.rs` — Descoberta de fontes

```rust
pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot>;
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo>;
pub fn build_font_book(slots: &[FontSlot]) -> FontBook;
```

Para CLI com `--font-path`.

### Outros

- `font_metrics::FontBookMetrics` — `pub struct`, implementação
  de métricas (usada internamente + por layout).
- `image_sizer::ImageSizeImageSizer` — struct vazia; implementa o
  sizer de imagens usando `imagesize`.

---

## APIs **privadas** a testes (não consumíveis pelo 04_wiring)

`03_infra/src/integration_tests.rs` é `#[cfg(test)] mod integration_tests`
— **tudo lá dentro é test-only**.

### Helpers útil mas privados

1. **`do_eval_with_sink(world: &SystemWorld, source: &Source) ->
   (SourceResult<Module>, Vec<SourceDiagnostic>)`**
   - Orquestra `eval()` com boilerplate comemo + drena Sink.
   - **Blocker directo**: esta é exactamente a função que a CLI
     precisa de chamar. Hoje é `fn` (não `pub fn`) dentro de
     `mod integration_tests`.

2. **`format_diagnostic(diag, source, source_path) -> String`**
   (Passo 111, ADR-0045)
   - Formato gcc/clang `path:linha:coluna: severity: mensagem`.
   - Idem: test-only hoje.

3. **`drain_diagnostics_to_stderr(diags, source, source_path)`**
   (Passo 111, ADR-0045)
   - Loop + eprint! com o formatter.
   - Idem: test-only hoje.

4. **`compile_to_pdf(src: &str) -> Vec<u8>`**
   - Pipeline completo em testes: `world_from_str` → eval → introspect
     → layout → export. Usa temp dir.
   - Idem: test-only.

### Implicação

**Opção A**: copiar a lógica destes helpers para dentro de 04_wiring
(re-implementar em L4).

**Opção B**: **promover** os helpers de test-only para `pub` em
`03_infra` (mover de `integration_tests.rs` para módulos públicos).

Opção B é arquitecturalmente mais limpa — cedo ou tarde a CLI
vai replicá-los. Fazer de uma vez evita divergência.

---

## Cadeia comemo — ponto de atenção

`03_infra` tem `comemo` **apenas em `dev-dependencies`**:

```toml
[dev-dependencies]
comemo = { workspace = true }
```

**Implicações**:

- A **produção** do `03_infra` não liga comemo — não pode chamar
  `eval()` sem refactor.
- Para promover `do_eval_with_sink` de test-only a public, é
  preciso mover comemo para `[dependencies]` regulares.
- 04_wiring **já tem acesso indirecto** a comemo via `typst-core`
  (que tem comemo normal).

### Alternativa

A CLI pode chamar `typst-core` directamente e manter `03_infra`
sem comemo em produção. O wrapper `do_eval_with_sink` mudaria
para L4 (ou L2 se materializasse). Mas isso duplica boilerplate:
`Routines::new()`, `Traced::default()`, `Sink::new()`,
`Route::root()`, `.track()`/`.track_mut()`.

Decisão depende do escopo escolhido em 112.D. Custo de adicionar
comemo a `[dependencies]` de `03_infra`: neste passo, **zero**
(comemo já é workspace dep).

---

## Chain completa para CLI mínima

Para compilar `input.typ` → `output.pdf`, a CLI precisa:

```text
1. SystemWorld::new(root, main)           [L3]
2. world.source(world.main())              [L3]
3. eval(routines, world, traced, sink, route, source)  [L1]
   + drain sink                            [L3 helper ou L4 inline]
4. module.content()                        [L1]
5. introspect(content)                     [L1]
6. layout(content, initial_state)          [L1]
   ou layout_with_font(content, font_data, size)  [L3]
7. export_pdf(&doc)                        [L3]
   ou export_pdf_with_font(&doc, font_data)       [L3]
8. fs::write(output_path, pdf_bytes)       [L4]
```

**Todos os componentes existem**. Bloqueio é só organizational:
onde vive o wrapper `do_eval_with_sink` + `drain_diagnostics_to_stderr`.

---

## Conclusões 112.B

1. **SystemWorld real existe** — CLI pode ler ficheiros. Sem
   necessidade de "materializar World primeiro".
2. **Export PDF está estável** (ADR-0027). `export_pdf` +
   `export_pdf_with_font` cobrem os dois cenários (sem fonte / com
   fonte).
3. **Helpers úteis existem** mas são **privados a testes**. Passo
   de construção tem de os promover (Opção B recomendada).
4. **Comemo em `03_infra` precisa virar `[dependencies]` regular**
   se helpers forem expostos — custo zero.
5. **Formatter do Passo 111 é reutilizável** — precisa só promover
   para `pub`.
6. **Zero bloqueios arquitecturais** — CLI mínima é viável hoje.
