---

# P516 — Subsetting TrueType de Fontes no PDF

> **Passo:** 516
> **Data:** 2026-06-30
> **Foco:** Implementar o subsetting de fontes TrueType no PDF gerado pelo cristalino, reduzindo o tamanho do PDF e atingindo paridade de produção com o Typst 0.15.0. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação XL-size com validação via corpus P490+P500.
> **Tamanho:** XL (~1-2 semanas de implementação + 3 dias de validação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica; subsetting é mecânica, mas necessário para produção.
> **ADR-0108 ACEITE** — medir antes de decidir.
> **ADR-0109 ACEITE** — atomização de código.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P515 (fontdb + font fallback implementados), P514 (paridade de linguagem completa).

---

## 1. Contexto

O P515 implementou **fontdb** (descoberta de fontes) e **font fallback** (shaping por caractere). No entanto, o PDF gerado pelo cristalino **não subseta fontes** — ele embedda a fonte inteira ou não embedda nada.

O Typst 0.15.0 subseta fontes automaticamente:
- Coleta glifos usados no documento.
- Cria um subset TrueType/CFF contendo apenas glifos necessários.
- Embedda o subset no PDF (~10-50 KB por fonte vs. ~500 KB-2 MB para fonte completa).
- Gera CMap para mapeamento Unicode → glyph ID no subset.

Esta trilha (P516) é o **último passo** para paridade de produção completa.

---

## 2. Arquitetura Proposta

### 2.1 Pipeline de Subsetting

```
[Layout Engine] → [Coleta de Glifos Usados] → [Subsetting TrueType] → [PDF com Fontes Embedadas]
                     │                              │
                     ▼                              ▼
              HashMap<FontID, Vec<GlyphID>>    ttf-parser + subsetting
```

### 2.2 Componentes

| Componente | Responsabilidade | Tamanho |
|------------|------------------|---------|
| **GlyphCollector** | Coletar glifos usados durante o layout | S |
| **TrueTypeSubset** | Criar subset TrueType (remover glifos não usados) | XL |
| **CMapGenerator** | Gerar CMap (Unicode → glyph ID) | M |
| **PDFontEmbedder** | Embeddar fonte subsetada no PDF (pdf-writer) | M |

---

## 3. Implementação por Componente

### 3.1 GlyphCollector (S-size, ~1 dia)

**Arquivo alvo:** `src/infra/font/glyph_collector.rs` (novo)

```rust
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct GlyphCollector {
    // FontID -> Set de GlyphIDs usados
    pub used_glyphs: HashMap<fontdb::ID, HashSet<u16>>,
}

impl GlyphCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_glyph(&mut self, font_id: fontdb::ID, glyph_id: u16) {
        self.used_glyphs.entry(font_id).or_default().insert(glyph_id);
    }

    pub fn record_text(&mut self, text: &str, font_id: fontdb::ID, font_data: &[u8]) {
        let face = ttf_parser::Face::parse(font_data, 0).unwrap();
        for c in text.chars() {
            if let Some(glyph_id) = face.glyph_index(ttf_parser::GlyphId(c as u16)) {
                self.record_glyph(font_id, glyph_id.0);
            }
        }
    }

    pub fn get_used_glyphs(&self, font_id: fontdb::ID) -> Option<&HashSet<u16>> {
        self.used_glyphs.get(&font_id)
    }
}
```

**Integração com o layout engine:**

```rust
// src/rules/layout/mod.rs
pub fn layout_document(content: &Content, ctx: &mut LayoutContext) -> Document {
    let mut glyph_collector = GlyphCollector::new();

    // Durante o layout, registrar glifos usados
    for item in &content.items {
        if let Content::Text(text) = item {
            let font_id = ctx.font_db.query(&text.font_family, text.weight, text.style).unwrap();
            let font_data = ctx.font_db.face_data(font_id).unwrap();
            glyph_collector.record_text(&text.text, font_id, font_data);
        }
    }

    // Passar glyph_collector para o renderizador de PDF
    ctx.glyph_collector = Some(glyph_collector);

    // ... resto do layout
}
```

### 3.2 TrueTypeSubset (XL-size, ~1 semana)

**Arquivo alvo:** `src/infra/font/subset.rs` (novo)

**Nota:** O subsetting TrueType é complexo. Requer manipulação de:
- `glyf` table (outlines de glifos)
- `loca` table (offsets para glifos)
- `cmap` table (mapeamento Unicode → glyph ID)
- `head`, `hhea`, `hmtx`, `maxp`, `post`, `name`, `OS/2` tables (metadados)

**Abordagem recomendada:** Usar crate `subset` ou implementar manualmente.

**Opção A: Implementação manual (mais controle, mais trabalho):**

```rust
use ttf_parser::{Face, TableProvider};

pub struct TrueTypeSubset {
    pub original_data: Vec<u8>,
    pub used_glyphs: Vec<u16>,
    pub subset_data: Vec<u8>,
}

impl TrueTypeSubset {
    pub fn new(font_data: &[u8], used_glyphs: &[u16]) -> Result<Self, SubsetError> {
        let face = Face::parse(font_data, 0)?;

        // Criar mapeamento: old_glyph_id -> new_glyph_id
        let mut glyph_map: HashMap<u16, u16> = HashMap::new();
        glyph_map.insert(0, 0); // .notdef sempre incluído
        for (new_id, &old_id) in used_glyphs.iter().enumerate() {
            if old_id != 0 {
                glyph_map.insert(old_id, (new_id + 1) as u16);
            }
        }

        // Construir novo arquivo TrueType
        let mut builder = TrueTypeBuilder::new();

        // Copiar tables necessárias
        builder.copy_table("head", &face)?;
        builder.copy_table("hhea", &face)?;
        builder.copy_table("maxp", &face)?;
        builder.copy_table("post", &face)?;
        builder.copy_table("name", &face)?;
        builder.copy_table("OS/2", &face)?;

        // Subsetar glyf + loca
        builder.subset_glyf(&face, &glyph_map)?;
        builder.subset_loca(&face, &glyph_map)?;

        // Subsetar cmap
        builder.subset_cmap(&face, &glyph_map)?;

        // Subsetar hmtx
        builder.subset_hmtx(&face, &glyph_map)?;

        let subset_data = builder.finish()?;

        Ok(Self {
            original_data: font_data.to_vec(),
            used_glyphs: used_glyphs.to_vec(),
            subset_data,
        })
    }
}
```

**Opção B: Usar crate `font-tools` (menos controle, menos trabalho):**

```toml
# Cargo.toml
[dependencies]
font-tools = "0.1"  # ou similar
```

```rust
use font_tools::subset::subset_font;

pub fn create_subset(font_data: &[u8], glyph_ids: &[u16]) -> Result<Vec<u8>, SubsetError> {
    subset_font(font_data, glyph_ids)
}
```

**Recomendação:** Começar com **Opção B** (crate existente) para validar o pipeline. Se a crate não existir ou não for adequada, migrar para **Opção B** (implementação manual).

### 3.3 CMapGenerator (M-size, ~2 dias)

**Arquivo alvo:** `src/infra/pdf/cmap.rs` (novo)

```rust
use pdf_writer::{Content, Obj, Ref};

pub struct CMapGenerator {
    pub unicode_to_glyph: HashMap<u32, u16>,
}

impl CMapGenerator {
    pub fn from_face(face: &ttf_parser::Face, used_glyphs: &[u16]) -> Self {
        let mut unicode_to_glyph = HashMap::new();

        for &glyph_id in used_glyphs {
            // Mapear glyph_id -> unicode(s) via cmap
            if let Some(unicode) = face.glyph_unicode(ttf_parser::GlyphId(glyph_id)) {
                unicode_to_glyph.insert(unicode as u32, glyph_id);
            }
        }

        Self { unicode_to_glyph }
    }

    pub fn generate_cmap_stream(&self) -> Vec<u8> {
        // Gerar CMap stream no formato PDF
        // Exemplo simplificado:
        let mut cmap = String::new();
        cmap.push_str("/CIDInit /ProcSet findresource begin
");
        cmap.push_str("12 dict begin
");
        cmap.push_str("begincmap
");
        cmap.push_str("/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def
");
        cmap.push_str("/CMapName /Adobe-Identity-UCS def
");
        cmap.push_str("/CMapType 2 def
");
        cmap.push_str("1 begincodespacerange
");
        cmap.push_str("<0000> <FFFF>
");
        cmap.push_str("endcodespacerange
");

        let ranges: Vec<_> = self.unicode_to_glyph.iter().collect();
        cmap.push_str(&format!("{} beginbfchar
", ranges.len()));
        for (unicode, glyph) in ranges {
            cmap.push_str(&format!("<{:04X}> <{:04X}>
", unicode, glyph));
        }
        cmap.push_str("endbfchar
");
        cmap.push_str("endcmap
");
        cmap.push_str("CMapName currentdict /CMap defineresource pop
");
        cmap.push_str("end
");
        cmap.push_str("end
");

        cmap.into_bytes()
    }
}
```

### 3.4 PDFontEmbedder (M-size, ~2 dias)

**Arquivo alvo:** `src/infra/pdf/font_embed.rs` (modificar)

```rust
use pdf_writer::{PdfWriter, Ref, Name, Str, Dict};

pub struct FontEmbedder<'a> {
    writer: &'a mut PdfWriter,
    font_db: &'a FontDatabase,
    glyph_collector: &'a GlyphCollector,
}

impl<'a> FontEmbedder<'a> {
    pub fn embed_fonts(&mut self) -> HashMap<fontdb::ID, Ref> {
        let mut font_refs = HashMap::new();

        for (font_id, glyph_ids) in &self.glyph_collector.used_glyphs {
            let font_data = self.font_db.face_data(*font_id).unwrap();

            // Criar subset
            let subset = TrueTypeSubset::new(font_data, &glyph_ids.iter().copied().collect::<Vec<_>>()).unwrap();

            // Criar CMap
            let face = ttf_parser::Face::parse(&subset.subset_data, 0).unwrap();
            let cmap = CMapGenerator::from_face(&face, &glyph_ids.iter().copied().collect::<Vec<_>>());

            // Referências PDF
            let font_ref = self.writer.alloc_ref();
            let font_descriptor_ref = self.writer.alloc_ref();
            let cmap_ref = self.writer.alloc_ref();
            let stream_ref = self.writer.alloc_ref();

            // Font object
            let mut font = self.writer.indirect(font_ref).start::<Dict>();
            font.pair(Name(b"Type"), Name(b"Font"));
            font.pair(Name(b"Subtype"), Name(b"TrueType"));
            font.pair(Name(b"BaseFont"), Name(font_name.as_bytes()));
            font.pair(Name(b"Encoding"), Name(b"Identity-H"));
            font.pair(Name(b"DescendantFonts"), Array(vec![font_descriptor_ref]));
            font.pair(Name(b"ToUnicode"), cmap_ref);

            // Font descriptor
            let mut descriptor = self.writer.indirect(font_descriptor_ref).start::<Dict>();
            descriptor.pair(Name(b"Type"), Name(b"FontDescriptor"));
            descriptor.pair(Name(b"FontName"), Name(font_name.as_bytes()));
            descriptor.pair(Name(b"Flags"), 32i32); // Symbolic
            descriptor.pair(Name(b"FontBBox"), Array(vec![...]));
            descriptor.pair(Name(b"ItalicAngle"), 0.0);
            descriptor.pair(Name(b"Ascent"), ascent);
            descriptor.pair(Name(b"Descent"), descent);
            descriptor.pair(Name(b"CapHeight"), cap_height);
            descriptor.pair(Name(b"StemV"), 80.0);
            descriptor.pair(Name(b"FontFile2"), stream_ref);

            // Font stream (subset)
            let mut stream = self.writer.indirect(stream_ref).start::<Stream>();
            stream.set_data(&subset.subset_data);

            // CMap stream
            let mut cmap_stream = self.writer.indirect(cmap_ref).start::<Stream>();
            cmap_stream.set_data(&cmap.generate_cmap_stream());

            font_refs.insert(*font_id, font_ref);
        }

        font_refs
    }
}
```

---

## 4. Validação

### 4.1 Testes de Tamanho de PDF

```bash
# Antes do subsetting (P515)
ls -la /tmp/out-pre-subset.pdf  # ~2 MB

# Depois do subsetting (P516)
ls -la /tmp/out-post-subset.pdf  # ~200 KB (10x menor)

# Comparar com vanilla 0.15.0
ls -la /tmp/vanilla.pdf  # ~200 KB
```

### 4.2 Testes de Fontes Embedadas

```bash
# Verificar se fontes estão embeddadas
pdffonts /tmp/out.pdf
# Esperado: fontes listadas como "embedded" ou "subset"

# Verificar se CMap funciona
# Extrair texto do PDF
pdftotext /tmp/out.pdf - | grep "Hello"
```

### 4.3 Testes de Corpus

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)"     || echo "FAIL: $(basename $f)"
done
# Esperado: 37/37 OK
```

### 4.4 Benchmark

```bash
python3 tools/perf/benchmark-p507.py
# Esperado: PDFs menores, tempo de render similar
```

---

## 5. Critério de Fecho

- [ ] GlyphCollector implementado e integrado ao layout engine.
- [ ] TrueTypeSubset implementado (manual ou via crate).
- [ ] CMapGenerator implementado.
- [ ] PDFontEmbedder implementado (pdf-writer).
- [ ] PDF gerado tem fontes embeddadas como "subset" (verificado com `pdffonts`).
- [ ] Tamanho do PDF reduzido em ≥50% para documentos com fontes.
- [ ] Texto extraível do PDF (CMap funciona).
- [ ] Corpus P490+P500: 37/37 OK (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 + L1 + infra).
- [ ] Sentinela `p516_subsetting_truetype` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-producao-p516.md` produzido.

---

## 6. Próximo Passo (P517)

Com P516 fechado, o cristalino atinge **paridade de produção completa** com Typst 0.15.0.

**Recomendação:** P517 = **DEBT-42 Benchmark Revalidado** — medir performance com shaping + subsetting ativos, comparar com vanilla 0.15.0 em corpus representativo.

Alternativa: P517 = **Activar `with_system_fonts` por defeito na CLI** — S-size, desbloqueia funcionalidade para usuários reais.

Alternativa: P517 = **Lookahead Layout Engine** — inovação arquitetural.

---

## A. Apêndice — Referência de Tables TrueType

| Table | Descrição | Subsetting |
|-------|-----------|------------|
| `glyf` | Outlines de glifos | Remover glifos não usados |
| `loca` | Offsets para glifos | Recalcular offsets |
| `cmap` | Unicode → glyph ID | Remover entradas não usadas |
| `head` | Header da fonte | Ajustar numGlyphs |
| `hhea` | Header horizontal | Ajustar numGlyphs |
| `hmtx` | Métricas horizontais | Remover glifos não usados |
| `maxp` | Maximum profile | Ajustar numGlyphs |
| `post` | PostScript info | Manter |
| `name` | Nomes | Manter |
| `OS/2` | Métricas OS/2 | Manter |
| `cvt`, `fpgm`, `prep` | Hinting | Manter (se presente) |

---

## B. Apêndice — Comandos de Verificação

```bash
# Compilar
cargo build --release -p typst-wiring

# Testar subsetting
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done

# Verificar fontes no PDF
pdffonts /tmp/out.pdf

# Verificar tamanho
ls -la /tmp/out.pdf

# Extrair texto
pdftotext /tmp/out.pdf - | head

# Benchmark
python3 tools/perf/benchmark-p507.py
```
