from pathlib import Path
p=Path('00_nucleo/prompts/infra/export/pdf_tiling.md');p.write_text('''# L0 — infra/export/pdf_tiling
Hash do Código: 00000000

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:4d947acae2d9bf8baff2639b7c9c1bdebb7b260e7ca4ac282d4f9d0e7a7feaee

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/pdf_tiling.rs`

## Medição anterior à decisão

O snapshot tiling-nativo-2026-10-14/before.json.gz registra fontes/UTC/HEAD/working tree. stream/shape.rs:156/204 converte Tiling a cor; builder.rs:277 reserva somente gradientes. O manifesto aprovado d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d exige célula preparada nativa. O pin vanilla 9f2b6e871 em typst-pdf/src/paint.rs:161–195 prepara stream de célula com raiz local e transformação corrigida.

## Decisão e contrato privado

Registrar padrões por ocorrência emprestada de Shape e identidade do Arc de célula, sem serializar endereços. IDs e nomes são locais determinísticos. Recorrer Groups com matriz local e raiz lógica paint_root; Link/Semantic apenas contém itens visuais. Cada célula aninhada tem canvas próprio e raiz identidade. O grafo de frames é finito e não contém referências de retorno mutáveis.

Emitir PatternType 1 PaintType 1 TilingType 1, BBox [0 0 cell.width cell.height], XStep/YStep = size + spacing positivos, recursos compartilhados, e stream vetorial da célula sem layout L3. A matriz converte coordenadas Y-up da célula para default user space do stream dono: flip(owner.height) * local_to_owner * placement * flip(cell.height). O helper afim de tiling decide Self/Parent e normaliza períodos. Rejeitar placement inválido explicitamente, sem sólido substituto.

O stream da célula usa itens emprestados, sem clonar ocorrências nem invalidar chaves de gradientes. Desabilitar tagging/MCIDs/UA1 durante a emissão e restaurar contexto antes da página; nenhuma anotação/destino é criado a partir da célula. A célula é recortada por BBox, inclusive overflow de conteúdo. Overlap de spacing negativo fica no padrão PDF nativo. Compartilhar fontes/imagens/spot/gradientes/patterns aninhados, sem ciclos em Resources: cada pattern deve referenciar apenas seus descendentes de tiling. Recursos de página incluem todos os padrões usados na página, inclusive descendentes.

## Aceitação

T1–T3 aprovado; este módulo não autoriza publicação. Conferir estrutura PDF/qpdf, limites/célula/spacing/offset/rotação/Parent afim/contorno/densidade, texto e imagens aninhados e isolamento semântico. Render e diagnósticos são a aceitação de linguagem; IDs e bytes não certificam paridade. Build e linhagem completos devem ficar verdes.
''')
p=Path('00_nucleo/prompts/infra/export/mod.md');s=p.read_text();s+='\nO módulo privado pdf_tiling possui a emissão/registro PatternType 1 segundo seu owner canônico; não altera API pública da fachada.\n';p.write_text(s)
p=Path('03_infra/src/export/mod.rs');s=p.read_text().replace('mod tiling;', 'mod tiling;\nmod pdf_tiling;');p.write_text(s)
p=Path('03_infra/src/export/pdf_tiling.rs');p.write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_tiling.md
//! @prompt-hash 00000000
//! @layer L3

use std::collections::HashMap;
use std::sync::Arc;
use typst_core::entities::frame_paint::{FramePaint, TilingFrame};
use typst_core::entities::layout_types::{FrameItem, PagedDocument, TransformMatrix};

pub(super) type Key = (usize, usize);
pub(super) fn key(item: &FrameItem, cell: &Arc<TilingFrame>) -> Key {
    (item as *const FrameItem as usize, Arc::as_ptr(cell) as usize)
}
struct Pattern<'a> {
    cell: &'a TilingFrame,
    matrix: TransformMatrix,
    id: usize,
    page: usize,
    children: Vec<usize>,
}
pub(super) struct Patterns<'a> {
    pub(super) names: HashMap<Key, String>,
    patterns: Vec<Pattern<'a>>,
}
fn flip(height: f64) -> TransformMatrix {
    TransformMatrix::translate(0.0,height).concat(&TransformMatrix::scale(1.0,-1.0))
}
impl<'a> Patterns<'a> {
    pub(super) fn scan(doc: &'a PagedDocument, next: &mut usize) -> Self {
        let mut result = Self { names: HashMap::new(), patterns: Vec::new() };
        for (page_index,page) in doc.pages.iter().enumerate() {
            let to_pdf = TransformMatrix::translate(page.bleed.left,page.bleed.bottom).concat(&flip(page.height));
            for items in [&page.background,&page.items,&page.foreground] {
                result.walk(items,page_index,to_pdf,TransformMatrix::identity(),next);
            }
        }
        result
    }
    fn walk(&mut self, items: &'a [FrameItem], page: usize, to_pdf: TransformMatrix, to_root: TransformMatrix, next: &mut usize) -> Vec<usize> {
        let mut found = Vec::new();
        for item in items {
            match item {
                FrameItem::Shape { pos, fill, stroke, .. } => {
                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let FramePaint::Tiling(cell) = paint {
                            let key = key(item,cell);
                            if let Some(name) = self.names.get(&key) {
                                found.push(name[1..].parse().expect("local pattern name"));
                                continue;
                            }
                            let placement = super::tiling::placement(cell,*pos,to_root).expect("invalid prepared tiling placement");
                            let matrix = to_pdf.concat(&placement).concat(&flip(cell.frame.size.height.0));
                            let index = self.patterns.len();
                            self.names.insert(key,format!("T{index}"));
                            let id = *next; *next += 1;
                            self.patterns.push(Pattern { cell, matrix, id, page, children: Vec::new() });
                            let children = self.walk(&cell.frame.items,page,flip(cell.frame.size.height.0),TransformMatrix::identity(),next);
                            self.patterns[index].children = children;
                            found.push(index);
                        }
                    }
                }
                FrameItem::Group { pos, matrix, paint_root, items, .. } => {
                    let local = TransformMatrix::translate(pos.x.0,pos.y.0).concat(matrix);
                    found.extend(self.walk(items,page,to_pdf.concat(&local),if *paint_root { TransformMatrix::identity() } else { to_root.concat(&local) },next));
                }
                FrameItem::Link { items, .. } | FrameItem::Semantic { items, .. } | FrameItem::TextReplacement { items, .. } => found.extend(self.walk(items,page,to_pdf,to_root,next)),
                _ => {}
            }
        }
        found.sort_unstable(); found.dedup(); found
    }
    pub(super) fn resources(&self, gradients: &str, page: usize) -> String {
        let entries = self.patterns.iter().enumerate().filter(|(_,p)| p.page == page)
            .map(|(i,p)| format!("/T{i} {} 0 R",p.id)).collect::<Vec<_>>().join(" ");
        if entries.is_empty() { return gradients.into(); }
        if gradients.is_empty() { format!("/Pattern << {entries} >>") }
        else { format!("{} {entries} >>",gradients.trim_end().strip_suffix(">>").expect("pattern dictionary")) }
    }
    pub(super) fn emit(&self, builder: &mut super::builder::PdfBuilder, page: usize, ctx: &mut super::stream::PageContext, resources: &str) {
        let (tags,ua1) = (ctx.tags,ctx.ua1);
        ctx.tags = super::PdfTags::Disabled; ctx.ua1 = false;
        for pattern in self.patterns.iter().filter(|p| p.page == page) {
            let size = pattern.cell.frame.size;
            let bytes = super::stream::build_cell_stream(&pattern.cell.frame.items,size.height.0,ctx);
            let m = pattern.matrix;
            let descendants = pattern.children.iter().map(|i| format!("/T{i} {} 0 R",self.patterns[*i].id)).collect::<Vec<_>>().join(" ");
            // Rebuild just the pattern dictionary, retaining fonts/images/spot.
            let mut cell_resources = resources.to_string();
            if let Some(start) = cell_resources.find("/Pattern <<") {
                let end = start + cell_resources[start..].find(">>").expect("pattern dictionary") + 2;
                let gradients = ctx.pat_refs.iter().map(|r| format!("/{} {} 0 R",r.name,r.pattern_obj_id)).collect::<Vec<_>>().join(" ");
                cell_resources.replace_range(start..end,&format!("/Pattern << {gradients} {descendants} >>"));
            }
            let mut object = format!("<< /Type /Pattern /PatternType 1 /PaintType 1 /TilingType 1 /BBox [0 0 {} {}] /XStep {} /YStep {} /Matrix [{} {} {} {} {} {}] /Resources << {} >> /Length {} >>\\nstream\\n",size.width.0,size.height.0,size.width.0+pattern.cell.spacing.width.0,size.height.0+pattern.cell.spacing.height.0,m.a,m.b,m.c,m.d,m.tx,m.ty,cell_resources,bytes.len()).into_bytes();
            object.extend_from_slice(&bytes); object.extend_from_slice(b"\\nendstream");
            builder.add_bytes(pattern.id,object);
        }
        ctx.tags=tags;ctx.ua1=ua1;
    }
}
''')
# Context field shared in all constructors.
p=Path('03_infra/src/export/stream.rs');s=p.read_text().replace('    pub font_scenario: FontScenario', '    pub tiling_names: Option<&\'a HashMap<super::pdf_tiling::Key, String>>,\n    pub font_scenario: FontScenario',1)
s=s.replace('            font_scenario:', '            tiling_names: None,\n            font_scenario:')
s+='''
pub(super) fn build_cell_stream(items: &[FrameItem], height: f64, ctx: &PageContext) -> Vec<u8> {
    let mut ops = String::new();
    page::emit_page_items(&mut ops,items,height,ctx,matches!(ctx.mode,StreamMode::Verbose));
    ops.into_bytes()
}
''';p.write_text(s)
# Hooks in all three builders.
for path in ['03_infra/src/export/builder.rs','03_infra/src/export/builder/fonts.rs']:
 p=Path(path);s=p.read_text().replace('let mut next_sub_id = first_grad_id + n_grads * 3;', 'let mut next_sub_id = first_grad_id + n_grads * 3;\n        let tilings = crate::export::pdf_tiling::Patterns::scan(doc, &mut next_sub_id);')
 s=s.replace('let pat_res = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);','let pat_res = tilings.resources(&pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs), i);')
 s=s.replace('let ctx = PageContext::','let mut ctx = PageContext::')
 s=s.replace('            let stream_bytes = self.maybe_oracle(build_page_stream(page, &ctx));', '            ctx.tiling_names = Some(&tilings.names);\n            tilings.emit(&mut self, i, &mut ctx, &resources_str);\n            let stream_bytes = self.maybe_oracle(build_page_stream(page, &ctx));')
 p.write_text(s)
p=Path('03_infra/src/export/stream/shape.rs');s=p.read_text().replace('typst_core::entities::paint::Paint','typst_core::entities::frame_paint::FramePaint').replace('use typst_core::entities::frame_paint::FramePaint;', 'use typst_core::entities::frame_paint::FramePaint as Paint;')
s=s.replace('paint.to_color()', 'typst_core::entities::paint::Paint::Gradient(gradient.clone()).to_color()',1)
a=s.index('        Paint::Tiling(_) => {');b=s.index('\n        }',a)+len('\n        }');s=s[:a]+'''        Paint::Tiling(cell) => {
            let name = ctx.tiling_names.and_then(|map| map.get(&crate::export::pdf_tiling::key(item,cell))).expect("prepared tiling must be registered");
            ops.push_str(&format!("/Pattern cs\\n/{name} scn\\n"));
        }'''+s[b:]
s=s.replace('paint.to_color()', 'typst_core::entities::paint::Paint::Gradient(g.clone()).to_color()',1)
a=s.index('        Paint::Tiling(_) => {');b=s.index('\n        }',a)+len('\n        }');s=s[:a]+'''        Paint::Tiling(cell) => {
            let name = ctx.tiling_names.and_then(|map| map.get(&crate::export::pdf_tiling::key(item,cell))).expect("prepared tiling must be registered");
            ops.push_str(&format!("/Pattern CS\\n/{name} SCN\\n{thickness:.2} w\\n"));
        }'''+s[b:];p.write_text(s)
# Gradients within prepared cells retain borrowed occurrence identity.
p=Path('03_infra/src/export/gradients/mod.rs');s=p.read_text().replace('use typst_core::entities::paint::Paint;', 'use typst_core::entities::frame_paint::FramePaint as Paint;')
needle='''                    for g in fill''';start=s.index(needle);s=s[:start]+'''                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let Paint::Tiling(cell) = paint {
                            let w=cell.frame.size.width.0;let h=cell.frame.size.height.0;
                            let cell_placement=ConicPlacement { to_pdf: TransformMatrix::translate(0.0,h).concat(&TransformMatrix::scale(1.0,-1.0)), parent: Rect{x:Pt(0.0),y:Pt(0.0),w:Pt(w),h:Pt(h)},canvas:(w,h) };
                            walk(&cell.frame.items,None,h,cell_placement,ptr_to_idx,refs,grad_objs,next_id,counter);
                        }
                    }
'''+s[start:]
start=s.index(needle,s.index('pub(super) fn pattern_resources_for_page'));s=s[:start]+'''                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let Paint::Tiling(cell) = paint { walk(&cell.frame.items,None,ptr_to_idx,refs,entries,seen); }
                    }
'''+s[start:];p.write_text(s)
