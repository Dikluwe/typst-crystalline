# Passo 20 — export_pdf() e PDF mínimo válido

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/entities/layout_types.rs` — `PagedDocument`, `Frame`, `FrameItem`
- `lab/typst-original/crates/typst-pdf/Cargo.toml` — deps de referência

Pré-condição: `cargo test` — 325 testes (303 L1 + 22 L3), zero violations.

**Invariante de L3 purity**: `export_pdf` não contém lógica de decisão
sobre o texto — obedece às posições e conteúdos ditados pelo `PagedDocument`.
Nenhuma regra tipográfica aqui. Transformações geométricas (inversão de Y)
são a única excepção — são transformações de coordenadas, não lógica de layout.

---

## Tarefa 1 — Diagnóstico de typst-pdf

```bash
# Deps de typst-pdf — crate externa de PDF ou manual?
grep "^[a-z].*=" lab/typst-original/crates/typst-pdf/Cargo.toml \
  | grep -v "typst\|workspace" | head -20

# Tamanho
find lab/typst-original/crates/typst-pdf/src -name "*.rs" \
  | xargs wc -l | tail -3
```

Se o original usa `pdf-writer`, `krilla`, ou outra crate → criar
ADR-0027 antes de continuar. Se não precisarmos dela (stub manual), sem ADR.

---

## Tarefa 2 — export_pdf() em L3

**Criar**: `03_infra/src/export.rs`
**Actualizar**: `03_infra/src/lib.rs` — `pub mod export;`

### Geometria: inversão do eixo Y

PDF usa coordenadas cartesianas (y=0 no canto inferior esquerdo, y cresce
para cima). O cristalino usa coordenadas de tela (y=0 no topo, y cresce
para baixo). Primeira transformação do `PdfBuilder`:

```
y_pdf = page_height - y_cristalino
```

### Escaping de texto em PDF

Os caracteres `(`, `)`, e `\` têm significado especial nos string
literals PDF e **devem ser escapados**, não substituídos:

```
( → \(
) → \)
\ → \\
```

Texto não-ASCII (codepoints > 127) → substituir por `?` (placeholder).
UTF-16BE com BOM para Unicode completo é trabalho do Passo 21.

```rust
fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '('  => out.push_str("\\("),
            ')'  => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _    => out.push('?'),   // placeholder para não-ASCII
        }
    }
    out
}
```

### PdfBuilder — acumulação de objectos e offsets

```rust
// 03_infra/src/export.rs

use typst_core::entities::layout_types::{FrameItem, PagedDocument};

pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> {
    PdfBuilder::new().build(doc)
}

struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,  // (id, conteúdo serializado)
}

impl PdfBuilder {
    fn new() -> Self {
        Self { objects: Vec::new() }
    }

    fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    fn build(mut self, doc: &PagedDocument) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        // IDs: 1=Catalog, 2=Pages, 3..3+n=Pages, 3+n..3+2n=Streams, 3+2n=Font
        let first_page   = 3usize;
        let first_stream = first_page + n;
        let font_id      = first_stream + n;

        // Obj 1: Catalog
        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        // Obj 2: Pages
        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        self.add(2, format!(
            "<< /Type /Pages /Kids [{kids}] /Count {n} >>"
        ));

        // Uma entrada por página
        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.size.width.val();
            let h = page.size.height.val();

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.1} {h:.1}] \
                   /Contents {stream_id} 0 R \
                   /Resources << /Font << /F1 {font_id} 0 R >> >> >>"
            ));

            // Content stream
            let stream_bytes = build_page_stream(page);
            let len = stream_bytes.len();
            let mut stream_obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            stream_obj.extend_from_slice(&stream_bytes);
            stream_obj.extend_from_slice(b"\nendstream");
            self.objects.push((stream_id, stream_obj));
        }

        // Helvetica — Standard Type1, sem embedding necessário
        self.add(font_id, format!(
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica \
               /Encoding /WinAnsiEncoding >>"
        ));

        self.serialize()
    }

    fn serialize(self) -> Vec<u8> {
        // Header — %PDF-1.7 + comentário binário (4 bytes > 127)
        // O comentário binário indica aos leitores que o ficheiro é binário
        let mut out: Vec<u8> = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();

        // Serializar objectos e registar offset de cada um
        let mut offsets: std::collections::HashMap<usize, usize> = Default::default();
        let mut ids_in_order: Vec<usize> = Vec::new();

        for (id, content) in &self.objects {
            offsets.insert(*id, out.len());
            ids_in_order.push(*id);
            out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
            out.extend_from_slice(content);
            out.extend_from_slice(b"\nendobj\n");
        }

        // xref table
        let xref_start = out.len();
        let max_id = offsets.keys().copied().max().unwrap_or(0);
        out.extend_from_slice(b"xref\n");
        out.extend_from_slice(format!("0 {}\n", max_id + 1).as_bytes());
        // Entrada 0: free list sentinel
        out.extend_from_slice(b"0000000000 65535 f \n");
        // Entradas 1..=max_id
        for id in 1..=max_id {
            let off = offsets.get(&id).copied().unwrap_or(0);
            // Cada entrada xref tem exactamente 20 bytes: "%010d %05d %c \n"
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }

        // Trailer
        out.extend_from_slice(format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            max_id + 1, xref_start
        ).as_bytes());

        out
    }
}

fn build_page_stream(
    page: &typst_core::entities::layout_types::Frame,
) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, font_size } => {
                // Inversão do eixo Y: y_pdf = page_height - y_cristalino
                let pdf_y = page_height - pos.y.val();
                let safe  = escape_pdf_string(text.as_str());

                if safe.is_empty() { continue; }

                ops.push_str(&format!(
                    "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n({safe}) Tj\nET\n",
                    font_size.val(), pos.x.val(), pdf_y
                ));
            }
        }
    }

    ops.into_bytes()
}

fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '('  => out.push_str("\\("),
            ')'  => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _    => out.push('?'),
        }
    }
    out
}
```

---

## Tarefa 3 — Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::{
        entities::content::Content,
        rules::layout::layout,
    };

    #[test]
    fn pdf_header_correcto() {
        let doc = layout(&Content::text("Hello"));
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"),
            "deve começar com %PDF-1.7");
    }

    #[test]
    fn pdf_termina_com_eof() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let tail = std::str::from_utf8(
            &pdf[pdf.len().saturating_sub(20)..]
        ).unwrap_or("");
        assert!(tail.contains("%%EOF"), "deve terminar com %%EOF");
    }

    #[test]
    fn pdf_tem_estrutura_valida() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref"),      "deve ter xref");
        assert!(s.contains("trailer"),   "deve ter trailer");
        assert!(s.contains("startxref"), "deve ter startxref");
        assert!(s.contains("/Catalog"),  "deve ter Catalog");
        assert!(s.contains("/Pages"),    "deve ter Pages");
        assert!(s.contains("Helvetica"), "deve ter Helvetica");
    }

    #[test]
    fn pdf_contem_texto_ascii() {
        let doc = layout(&Content::text("Hello world"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("Hello") || s.contains("world"),
            "texto ASCII deve aparecer no PDF");
    }

    #[test]
    fn pdf_documento_vazio_valido() {
        let doc = typst_core::entities::layout_types::PagedDocument::new(vec![]);
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        assert!(String::from_utf8_lossy(&pdf).contains("%%EOF"));
    }

    #[test]
    fn escaping_caracteres_especiais() {
        // Texto com ( ) \ deve ser escapado, não substituído por ?
        let escaped = escape_pdf_string("Hello (world) 100\\%");
        assert!(escaped.contains("\\("), "( deve ser escapado");
        assert!(escaped.contains("\\)"), ") deve ser escapado");
        assert!(escaped.contains("\\\\"), "\\ deve ser escapado");
        assert!(!escaped.contains("\\%"), "% não precisa de escape em PDF strings");
    }

    #[test]
    fn inversao_eixo_y_texto_no_topo() {
        // Texto posicionado no topo cristalino (y pequeno)
        // deve ter y_pdf grande (próximo de page_height)
        use typst_core::entities::layout_types::{
            Frame, FrameItem, PagedDocument, Point, Pt, Size,
        };
        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Text {
            pos:       Point { x: Pt(72.0), y: Pt(84.0) },  // topo da página
            text:      "Top".into(),
            font_size: Pt(12.0),
        });
        let doc = PagedDocument::new(vec![frame]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        // y_pdf = 842 - 84 = 758 → deve aparecer no stream
        assert!(s.contains("758.0") || s.contains("758"),
            "y_pdf deve ser 842-84=758: {}", &s[..s.len().min(500)]);
    }

    #[test]
    fn pdf_mediabox_dimensoes_a4() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("595") && s.contains("842"),
            "MediaBox deve ter dimensões A4 (595x842 pt)");
    }
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found

# Verificação manual — gerar e inspecionar o PDF
# (adicionar um binário de teste temporário ou usar um teste de integração)
```

Critérios de conclusão:
- PDF começa com `%PDF-1.7` ✓
- PDF termina com `%%EOF` ✓
- Secções `xref`, `trailer`, `startxref` presentes ✓
- `Helvetica` referenciada ✓
- `(`, `)`, `\` escapados (não substituídos por `?`) ✓
- Inversão de Y correcta — texto no topo cristalino → y_pdf grande ✓
- Dimensões A4 no MediaBox ✓
- Zero violations ✓
- Testes não regridem (325 base + novos) ✓

**Critério de ouro**: abrir o PDF gerado num leitor (Chrome, evince,
mupdf) e ver "Hello" na posição correcta da página. Documentar no
relatório se foi possível verificar manualmente.

---

## Ao terminar, reportar

**Do diagnóstico:**
- Crate PDF usada pelo original — ADR-0027 necessária?
- Tamanho de typst-pdf (estimativa de complexidade do Passo 21)

**Da implementação:**
- Se o PDF gerado abre num leitor (sim/não, leitor usado)
- Se a inversão de Y colocou o texto na posição visual correcta
- Se o escaping de `(`, `)`, `\` foi validado pelo teste

**Número total de testes e zero violations.**

**Go/No-Go para o Passo 21:**
- **GO — embedding de fontes**: PDF abre e mostra texto; Passo 21
  embebe subsets de fontes reais via ttf-parser para produzir PDF
  tipograficamente correcto (sem `?` para não-ASCII)
- **GO — mais Content**: se Heading/Strong/Emph são urgentes para
  documentos reais antes de melhorar as fontes; Passo 21 adiciona
  variantes ao enum Content e ao Layouter
- **NO-GO — PDF inválido**: xref offset errado ou estrutura incorrecta
  impede a abertura; Passo 21 corrige com ferramenta de validação
  (ex: `pdfinfo` do poppler)
