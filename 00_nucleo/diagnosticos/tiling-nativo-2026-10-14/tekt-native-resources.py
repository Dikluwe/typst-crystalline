from pathlib import Path
import subprocess
roles={
'03_infra/src/export/font_binding.rs':'Percorrer recursos visuais também dentro de fill/stroke TilingFrame para binding e partição de fontes. Usar walker privado de recursos; manter o visitor semântico L1 sem descida em paint.',
'03_infra/src/export/fonts.rs':'Coletar codepoints, glyph IDs, clusters e mapeamentos também das células preparadas para subsetting correto. A coleta de recursos não insere conteúdo semântico na página.',
'03_infra/src/export/images/scan.rs':'Validar, registrar e referenciar imagens também nas células e em background/foreground, com deduplicação de bytes por Arc e erros de decode existentes.',
'03_infra/src/export/builder/fonts.rs':'Coletar glyphs das células preparadas e integrar registro/emissão de PatternType 1 nos caminhos CID/multifont, com mesmos recursos e fontes, sem semântica ou links duplicados.',
'03_infra/src/export/builder/resources.rs':'Coletar colorantes spot nos paints preparados e células aninhadas, preservando PageFill declarativo. Emissão de gradientes segue contrato vigente.',
'03_infra/src/shaper/traversal.rs':'Executar shaping visual das células preparadas uma vez por ocorrência via Arc::make_mut e caches locais existentes, preservando size/spacing/placement. A célula usa seu próprio contexto bidi, sem inserir suas identidades na página.',
'03_infra/src/pipeline/resources.rs':'Resolver fontes usadas em texto/glifos das células preparadas de fill e stroke, recursivamente, preservando decisões de variações e fallback existentes.'}
for source,role in roles.items(): subprocess.run(['python3','/tmp/tekt-native-spec.py',source,role],check=True)
p=Path('00_nucleo/prompts/infra/export/tiling.md');s=p.read_text();s+='''\n## Percurso privado de recursos preparado\n\nMedição: images/scan.rs:57,139,273, fonts.rs:80 e font_binding.rs:124 usam o visitor semântico, que em layout_types.rs:1370 visita Shape sem entrar em paint. A troca de transporte ocultaria recursos antes presentes em Group.\n\nDecisão: este owner também oferece walker privado exclusivo de recursos visuais, delegando folhas ao visitor e descendo Group/Link/Semantic/TextReplacement e fill/stroke TilingFrame. Não chama callbacks de semântica/link de célula; não altera FrameVisitor público nem navegação/MCIDs. Coleta de recursos sem I/O aqui; decoders e fontes permanecem nos owners atuais.\n''';p.write_text(s)
p=Path('03_infra/src/export/tiling.rs');s=p.read_text();s+='''
/// Resource-only traversal: prepared paints are deliberately not page content.
pub(super) fn walk_resources<V: typst_core::entities::frame_visitor::FrameVisitor>(visitor: &mut V, items: &[typst_core::entities::layout_types::FrameItem]) {
    use typst_core::entities::layout_types::FrameItem;
    use typst_core::entities::frame_paint::FramePaint;
    for item in items {
        match item {
            FrameItem::Group { items, .. } | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } | FrameItem::TextReplacement { items, .. } => walk_resources(visitor,items),
            FrameItem::Shape { fill, stroke, .. } => {
                visitor.visit_shape(item);
                for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                    if let FramePaint::Tiling(cell) = paint { walk_resources(visitor,&cell.frame.items); }
                }
            }
            _ => visitor.visit_item(item),
        }
    }
}
''';p.write_text(s)
for source in ['03_infra/src/export/font_binding.rs','03_infra/src/export/fonts.rs','03_infra/src/export/images/scan.rs','03_infra/src/export/builder/fonts.rs']:
 p=Path(source);s=p.read_text().replace('use typst_core::entities::frame_visitor::{walk_frame_items, FrameVisitor};','use typst_core::entities::frame_visitor::FrameVisitor;\nuse crate::export::tiling::walk_resources as walk_frame_items;').replace('use typst_core::entities::frame_visitor::walk_frame_items;', 'use crate::export::tiling::walk_resources as walk_frame_items;')
 if source.endswith('scan.rs') or source.endswith('builder/fonts.rs'):
  import re
  s=re.sub(r'(        |    )walk_frame_items\(([^\n]+), &page.items\);',r'\1for items in [&page.background, &page.items, &page.foreground] { walk_frame_items(\2, items); }',s)
 p.write_text(s)
p=Path('03_infra/src/export/builder/resources.rs');s=p.read_text();s=s.replace('        fn paint(&mut self, paint:', '''        fn prepared(&mut self, paint: &typst_core::entities::frame_paint::FramePaint) {
            use typst_core::entities::frame_paint::FramePaint;
            match paint {
                FramePaint::Solid(c) => self.color(c),
                FramePaint::Gradient(g) => self.paint(&typst_core::entities::paint::Paint::Gradient(g.clone())),
                FramePaint::Tiling(c) => walk_frame_items(self, &c.frame.items),
            }
        }
        fn paint(&mut self, paint:''',1).replace('self.paint(fill);','self.prepared(fill);').replace('self.paint(&stroke.paint);','self.prepared(&stroke.paint);');p.write_text(s)
p=Path('03_infra/src/shaper/traversal.rs');s=p.read_text();a=s.index('        // intencional: nós já shaped');s=s[:a]+'''        FrameItem::Shape { fill, stroke, .. } => {
            for paint in fill.iter_mut().chain(stroke.iter_mut().map(|s| &mut s.paint)) {
                if let typst_core::entities::frame_paint::FramePaint::Tiling(cell) = paint {
                    let cell = std::sync::Arc::make_mut(cell);
                    let mut shaped = Vec::new();
                    for child in cell.frame.items.drain(..) {
                        shaped.extend(shape_item_inner(world, child, cache, face_cache, false, None));
                    }
                    cell.frame.items = shaped;
                }
            }
        }
'''+s[a:];s=s.replace('        | FrameItem::Image { .. }\n        | FrameItem::Shape { .. } => {}','        | FrameItem::Image { .. } => {}');p.write_text(s)
p=Path('03_infra/src/pipeline/resources.rs');s=p.read_text();a=s.index('            FrameItem::Line { .. }',s.index('fn collect_fonts_in_items'));s=s[:a]+'''            FrameItem::Shape { fill, stroke, .. } => {
                for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                    if let typst_core::entities::frame_paint::FramePaint::Tiling(cell) = paint {
                        collect_fonts_in_items(&cell.frame.items,metrics,seen);
                    }
                }
            }
'''+s[a:];s=s.replace('            FrameItem::Line { .. }\n            | FrameItem::Image { .. }\n            | FrameItem::Shape { .. } => {}','            FrameItem::Line { .. } | FrameItem::Image { .. } => {}',1);p.write_text(s)
