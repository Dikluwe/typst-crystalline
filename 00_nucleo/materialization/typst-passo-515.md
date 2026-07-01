---

# P515 — Trilha 5: Shaping Real com rustybuzz + fontdb + Subsetting

> **Passo:** 515
> **Data:** 2026-06-30
> **Foco:** Implementar o subsistema de shaping de texto no cristalino, integrando rustybuzz para posicionamento de glifos, fontdb para descoberta de fontes do sistema, e subsetting de fontes no PDF gerado. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação arquitetural XL-size com validação via corpus P490+P500.
> **Tamanho:** XL (~2-3 semanas de implementação + 1 semana de validação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica; shaping é mecânica, mas necessário para produção.
> **ADR-0108 ACEITE** — medir antes de decidir.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P514 (paridade de linguagem completa), P507 (benchmark que revelou ausência de shaping).

---

## 1. Contexto

O P514 confirmou que o cristalino atinge **paridade de linguagem completa** com Typst 0.15.0 (37/37 documentos compilam, 3550 testes passam). No entanto, o P507 revelou que o cristalino **não faz shaping real**:

- `FrameItem::Text` é sequencial sem posicionamento de glifo via rustybuzz.
- PDF gerado não tem fontes embeddadas (subsetting ausente).
- `fontdb` não é inicializado (descoberta de fontes do sistema ausente).

Esta trilha (P515) é o **único caminho** para paridade de produção completa.

---

## 2. Arquitetura Proposta

### 2.1 Componentes

```
┌─────────────────────────────────────────────────────────────┐
│  Fase 1: Descoberta de Fontes (fontdb)                       │
│  - Inicializar fontdb::Database                               │
│  - Carregar fontes do sistema (system fonts)                  │
│  - Carregar fontes do projeto (project fonts)               │
│  - Mapear família → fonte (font matching)                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Fase 2: Shaping (rustybuzz)                                │
│  - Para cada run de texto:                                  │
│    1. Determinar fonte (font matching por caractere)        │
│    2. Criar face rustybuzz (hb_face_t)                      │
│    3. Criar buffer rustybuzz (hb_buffer_t)                  │
│    4. Adicionar texto ao buffer                             │
│    5. Configurar script, language, direction                │
│    6. Executar shaping (hb_shape)                           │
│    7. Extrair glifos posicionados (glyphs + advances)       │
│  - Suportar: kerning, ligatures, contextual alternates        │
│  - Suportar: fallback de fontes (font fallback)              │
│  - Suportar: text direction (LTR, RTL, vertical)           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Fase 3: Layout com Glifos Posicionados                     │
│  - Substituir FrameItem::Text sequencial por                │
│    FrameItem::ShapedText { glyphs: Vec<Glyph> }             │
│  - Cada Glyph: id, x_advance, y_advance, x_offset, y_offset│
│  - Calcular posições absolutas de cada glifo                │
│  - Suportar: line breaking (linebreak crate)               │
│  - Suportar: hyphenation (hyphenation crate)               │
│  - Suportar: bidi (unicode-bidi crate)                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  Fase 4: Subsetting de Fontes no PDF                        │
│  - Coletar todos os glifos usados no documento              │
│  - Para cada fonte: criar subset (ttf-parser + subsetting)  │
│  - Embed fontes subsetadas no PDF (pdf-writer)             │
│  - Gerar CMap para mapeamento Unicode → glyph ID            │
│  - Suportar: CFF e TrueType outlines                        │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Dependências de Crates

| Crate | Versão | Uso |
|-------|--------|-----|
| `rustybuzz` | 0.14+ | Shaping (Harfbuzz em Rust) |
| `fontdb` | 0.21+ | Descoberta de fontes |
| `ttf-parser` | 0.24+ | Parsing de fontes TrueType/OpenType |
| `pdf-writer` | 0.12+ | Geração de PDF (já usado) |
| `linebreak` | 0.5+ | Line breaking Unicode |
| `hyphenation` | 0.10+ | Hyphenation (opcional) |
| `unicode-bidi` | 0.3+ | Bidirectional text (opcional) |

---

## 3. Implementação por Fase

### 3.1 Fase 1: fontdb (M-size, ~3 dias)

**Arquivo alvo:** `src/infra/font/fontdb.rs` (novo)

```rust
use fontdb::{Database, Family, Query, Source};

pub struct FontDatabase {
    db: Database,
    system_loaded: bool,
}

impl FontDatabase {
    pub fn new() -> Self {
        let mut db = Database::new();
        // Carregar fontes do sistema
        db.load_system_fonts();

        Self { db, system_loaded: true }
    }

    pub fn load_project_fonts(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.db.load_font_file(path).ok();
        }
    }

    pub fn query(&self, family: &str, weight: Weight, style: Style) -> Option<fontdb::ID> {
        let query = Query {
            families: &[Family::Name(family)],
            weight: weight.into(),
            style: style.into(),
            ..Default::default()
        };
        self.db.query(&query)
    }

    pub fn face_data(&self, id: fontdb::ID) -> Option<&[u8]> {
        self.db.face_source(id).and_then(|source| {
            match source {
                Source::Binary(data) => Some(data.as_ref()),
                Source::File(path) => std::fs::read(path).ok().map(|v| v.leak().as_ref()),
            }
        })
    }
}
```

**Integração com o compilador:**

```rust
// src/infra/pipeline.rs (ou equivalente)
pub struct CompileContext {
    // ... campos existentes
    pub font_db: FontDatabase,
}

impl CompileContext {
    pub fn new() -> Self {
        Self {
            // ... inicialização existente
            font_db: FontDatabase::new(),
        }
    }
}
```

### 3.2 Fase 2: rustybuzz (L-size, ~1 semana)

**Arquivo alvo:** `src/infra/font/shaper.rs` (novo)

```rust
use rustybuzz::{Face, Buffer, Direction, Script, Language, GlyphInfo, GlyphPosition};

pub struct ShapedGlyph {
    pub glyph_id: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
}

pub struct ShapedRun {
    pub glyphs: Vec<ShapedGlyph>,
    pub font_id: fontdb::ID,
    pub text: String,
}

pub fn shape_text(
    text: &str,
    font_id: fontdb::ID,
    font_data: &[u8],
    face_index: u32,
    script: Script,
    language: Option<Language>,
    direction: Direction,
) -> Result<ShapedRun, ShapingError> {
    // Criar face rustybuzz
    let face = Face::from_slice(font_data, face_index)
        .ok_or(ShapingError::InvalidFont)?;

    // Criar buffer
    let mut buffer = Buffer::new();
    buffer.add_str(text);
    buffer.set_direction(direction);
    buffer.set_script(script);
    if let Some(lang) = language {
        buffer.set_language(lang);
    }

    // Executar shaping
    rustybuzz::shape(&face, &mut buffer, &[]);

    // Extrair glifos
    let glyph_infos = buffer.glyph_infos();
    let glyph_positions = buffer.glyph_positions();

    let glyphs: Vec<ShapedGlyph> = glyph_infos.iter().zip(glyph_positions.iter())
        .map(|(info, pos)| ShapedGlyph {
            glyph_id: info.glyph_id,
            x_advance: pos.x_advance,
            y_advance: pos.y_advance,
            x_offset: pos.x_offset,
            y_offset: pos.y_offset,
            cluster: info.cluster,
        })
        .collect();

    Ok(ShapedRun {
        glyphs,
        font_id,
        text: text.to_string(),
    })
}
```

**Font fallback:**

```rust
pub fn shape_with_fallback(
    text: &str,
    preferred_family: &str,
    font_db: &FontDatabase,
) -> Vec<ShapedRun> {
    let mut runs = Vec::new();
    let mut current_run = String::new();
    let mut current_font_id: Option<fontdb::ID> = None;

    for c in text.chars() {
        let font_id = font_db.query(preferred_family, Weight::NORMAL, Style::Normal)
            .or_else(|| font_db.find_font_with_char(c));

        match (current_font_id, font_id) {
            (Some(prev), Some(curr)) if prev == curr => {
                current_run.push(c);
            }
            _ => {
                if !current_run.is_empty() {
                    runs.push(shape_run(&current_run, current_font_id.unwrap(), font_db));
                }
                current_run = c.to_string();
                current_font_id = font_id;
            }
        }
    }

    if !current_run.is_empty() && current_font_id.is_some() {
        runs.push(shape_run(&current_run, current_font_id.unwrap(), font_db));
    }

    runs
}
```

### 3.3 Fase 3: Layout com Glifos Posicionados (M-size, ~4 dias)

**Arquivo alvo:** `src/rules/layout/text.rs` (modificar)

```rust
// Antes (P514):
pub struct FrameItem {
    pub content: FrameItemContent,
    pub pos: Point,
}

pub enum FrameItemContent {
    Text(TextItem),        // Texto sequencial (sem shaping)
    // ... outros
}

// Depois (P515):
pub enum FrameItemContent {
    Text(TextItem),        // Mantido para compatibilidade (fallback)
    ShapedText(ShapedTextItem), // Texto com glifos posicionados
    // ... outros
}

pub struct ShapedTextItem {
    pub glyphs: Vec<Glyph>,
    pub font_id: fontdb::ID,
    pub size: Length,
    pub color: Color,
}

pub struct Glyph {
    pub id: u32,
    pub x: f64,      // posição absoluta X
    pub y: f64,      // posição absoluta Y
    pub width: f64,  // largura do glifo
    pub height: f64, // altura do glifo
}
```

**Modificação do layout engine:**

```rust
fn layout_text_item(
    item: &TextItem,
    ctx: &mut LayoutContext,
) -> Vec<FrameItem> {
    let font_db = &ctx.font_db;
    let shaped_runs = shape_with_fallback(&item.text, &item.font_family, font_db);

    let mut frame_items = Vec::new();
    let mut x = 0.0;
    let y = 0.0;

    for run in shaped_runs {
        let font_data = font_db.face_data(run.font_id).unwrap();
        let face = ttf_parser::Face::parse(font_data, 0).unwrap();

        for glyph in run.glyphs {
            let glyph_id = ttf_parser::GlyphId(glyph.glyph_id as u16);
            let bbox = face.glyph_bounding_box(glyph_id).unwrap_or_default();

            let scale = item.size.to_pt() / face.units_per_em() as f64;

            frame_items.push(FrameItem {
                content: FrameItemContent::Glyph(GlyphItem {
                    id: glyph.glyph_id,
                    font_id: run.font_id,
                    x: x + glyph.x_offset as f64 * scale,
                    y: y + glyph.y_offset as f64 * scale,
                    width: (bbox.x_max - bbox.x_min) as f64 * scale,
                    height: (bbox.y_max - bbox.y_min) as f64 * scale,
                }),
                pos: Point::new(x, y),
            });

            x += glyph.x_advance as f64 * scale;
        }
    }

    frame_items
}
```

### 3.4 Fase 4: Subsetting de Fontes no PDF (L-size, ~1 semana)

**Arquivo alvo:** `src/infra/pdf/font_subset.rs` (novo)

```rust
use pdf_writer::{PdfWriter, Ref, Name, Str};
use ttf_parser::Face;

pub struct FontSubset {
    pub original_data: Vec<u8>,
    pub glyph_ids: Vec<u16>,
    pub unicode_map: HashMap<u16, char>, // glyph_id -> unicode
}

impl FontSubset {
    pub fn from_usage(font_data: &[u8], used_glyphs: &[u32]) -> Result<Self, SubsetError> {
        let face = Face::parse(font_data, 0)?;

        let mut glyph_ids = Vec::new();
        let mut unicode_map = HashMap::new();

        for &glyph_id in used_glyphs {
            let gid = ttf_parser::GlyphId(glyph_id as u16);
            glyph_ids.push(glyph_id as u16);

            // Mapear glyph_id -> unicode (via cmap)
            if let Some(unicode) = face.glyph_unicode(gid) {
                unicode_map.insert(glyph_id as u16, unicode);
            }
        }

        // Criar subset (simplificado — usar crate font-tools ou implementar manual)
        let subset_data = create_font_subset(font_data, &glyph_ids)?;

        Ok(Self {
            original_data: subset_data,
            glyph_ids,
            unicode_map,
        })
    }

    pub fn embed_in_pdf(&self, writer: &mut PdfWriter, font_ref: Ref) {
        // Criar font object no PDF
        writer.indirect(font_ref).start::<Font>();
        // ... implementação específica do pdf-writer
    }
}
```

**Integração com o renderizador de PDF:**

```rust
// src/infra/pdf/mod.rs (modificar)
pub fn render_pdf(document: &Document, ctx: &CompileContext) -> Vec<u8> {
    let mut writer = PdfWriter::new();

    // Coletar todos os glifos usados
    let mut used_glyphs: HashMap<fontdb::ID, Vec<u32>> = HashMap::new();
    for page in &document.pages {
        for item in &page.items {
            if let FrameItemContent::Glyph(glyph) = &item.content {
                used_glyphs.entry(glyph.font_id).or_default().push(glyph.id);
            }
        }
    }

    // Criar subsets e embeddar
    let mut font_refs = HashMap::new();
    for (font_id, glyphs) in used_glyphs {
        let font_data = ctx.font_db.face_data(font_id).unwrap();
        let subset = FontSubset::from_usage(font_data, &glyphs).unwrap();
        let font_ref = Ref::new(writer.xref.len() as i32 + 1);
        subset.embed_in_pdf(&mut writer, font_ref);
        font_refs.insert(font_id, font_ref);
    }

    // ... resto da renderização
    writer.finish()
}
```

---

## 4. Validação

### 4.1 Testes de Shaping

```typst
// test-shaping-basic.typ
#text("Hello World", font: "Linux Libertine")
#text("fi fl ffi", font: "Linux Libertine")  // ligatures
#text("مرحبا", font: "Amiri")  // RTL
#text("日本語", font: "Noto Sans CJK")  // CJK
```

### 4.2 Testes de Font Fallback

```typst
// test-font-fallback.typ
#text("Hello 日本語 مرحبا", font: "Linux Libertine")
// Deve usar Linux Libertine para "Hello", Noto Sans CJK para "日本語", Amiri para "مرحبا"
```

### 4.3 Testes de PDF

```bash
# Verificar se fontes estão embeddadas
pdffonts /tmp/out.pdf
# Esperado: fontes listadas como "embedded" ou "subset"

# Verificar se glifos estão posicionados corretamente
# Comparar visualmente com vanilla 0.15.0
```

### 4.4 Benchmark Revalidado

```bash
# Re-executar P507 com shaping ativo
python3 tools/perf/benchmark-p507.py
# Esperado: ratio cristalino/vanilla mais próximo de 1.0 (ou < 2.0)
```

---

## 5. Critério de Fecho

- [ ] Fase 1: fontdb inicializado com fontes do sistema.
- [ ] Fase 2: rustybuzz integrado, shaping funciona para texto latino.
- [ ] Fase 2b: Font fallback funciona (CJK, RTL, emoji).
- [ ] Fase 3: Layout engine usa glifos posicionados (FrameItem::Glyph).
- [ ] Fase 4: Fontes subsetadas e embeddadas no PDF.
- [ ] Testes de shaping passam (latino, ligatures, RTL, CJK).
- [ ] Testes de font fallback passam.
- [ ] PDF gerado tem fontes embeddadas (verificado com `pdffonts`).
- [ ] Benchmark revalidado: ratio cristalino/vanilla < 2.0.
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 + L1 + L3 + infra).
- [ ] Sentinela `p515_trilha_5_shaping` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-producao-p515.md` produzido.

---

## 6. Próximo Passo (P516)

Com P515 fechado, o cristalino atinge **paridade de produção completa** com Typst 0.15.0.

**Recomendação:** P516 = **DEBT-42 Benchmark Revalidado** — medir performance com shaping ativo, comparar com vanilla 0.15.0 em corpus representativo.

Alternativa: P516 = **Lookahead Layout Engine** — inovação arquitetural (não paridade).

---

## A. Apêndice — Referência de Crates

```toml
# Cargo.toml
[dependencies]
rustybuzz = "0.14"
fontdb = "0.21"
ttf-parser = "0.24"
pdf-writer = "0.12"  # já usado
linebreak = "0.5"
# hyphenation = "0.10"  # opcional
# unicode-bidi = "0.3"  # opcional
```

---

## B. Apêndice — Comandos de Verificação

```bash
# Instalar dependências
cargo add rustybuzz fontdb ttf-parser linebreak

# Compilar
cargo build --release -p typst-wiring

# Testar shaping
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)"     || echo "FAIL: $(basename $f)"
done

# Verificar fontes no PDF
pdffonts /tmp/out.pdf

# Benchmark
python3 tools/perf/benchmark-p507.py
```
