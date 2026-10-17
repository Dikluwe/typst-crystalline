from pathlib import Path
for owner,contract in {
'compiler/layout':'Armazenar relative_region_size privado opcional; None no root. Representa tamanho real do contexto de porcentagens durante subframes, separado do sentinel de altura usado para evitar paginação. Não altera entidade ou assinatura pública.',
'compiler/layout/sub_frame':'Salvar/restaurar relative_region_size em LIFO. No subframe, largura é a região fornecida; altura explícita vence, ausente herda região real do pai em vez do sentinel. Um transform nativo terminal retorna seu fim físico sem margem abaixo; manter o caminho Matrix legado. Não mudar o algoritmo dos demais tipos.',
'compiler/layout/checkpoint':'Capturar/restaurar relative_region_size juntamente com o contexto de subframe; rollback não pode herdar dimensões da tentativa descartada.',
'compiler/layout/transform':'Move usa tamanho real do contexto de porcentagens, não a sentinela de paginação. As quatro variantes nativas são unidades de bloco no fluxo (rules.rs:725–739 upstream), inclusive no corpo de Place; dentro de Box a caixa externa estabelece a ocorrência inline. Terminar linha anterior, colapsar above/below de block vigentes, registrar fim/largura lógicos e conservar baseline interna do frame. Matrix Rust mantém fluxo legado. Reflow false mantém tamanho do body, não implica que a função nativa seja inline. Valores de origem e bbox são sobre o frame medido, independentes da margem externa. Refutar por irmãos, texto e subframes.'}.items():
 p=Path('00_nucleo/prompts')/(owner+'.md');p.write_text(p.read_text()+'''\n## Região relativa e fluxo das transformações — medição causal\n\n`transformacoes-2026-10-14/after-first/results.json.gz` registra o candidato e\nseus hashes: Move percentual fica fora da página porque a altura é o sentinel;\ntextos A/TEST/B ficam numa linha no Tekt e em blocos no pin. As imagens foram\ninspecionadas, sem realinhamento. Fonte pinada `typst-layout/src/rules.rs:725–739`\nusa BlockElem::single_layouter para as quatro funções, confirmando a semântica\nde bloco; não é uma convenção mecânica de enum. Transform Rust genérico não\né uma função da linguagem. Correção dentro de X4 aprovado, sem novo contrato\npúblico. A hipótese é refutada por deslocamento em região explícita/aninhada,\nespaçamento, perda de body ou vazamento após rollback.\n\n'''+contract+'\n')
p=Path('01_core/src/compiler/layout/mod.rs');s=p.read_text().replace('    pub(super) is_sub_frame: bool,','    pub(super) is_sub_frame: bool,\n    pub(super) relative_region_size: Option<(f64, f64)>,').replace('            is_sub_frame: false,','            is_sub_frame: false,\n            relative_region_size: None,');p.write_text(s)
p=Path('01_core/src/compiler/layout/checkpoint.rs');s=p.read_text().replace('    is_sub_frame: bool,','    is_sub_frame: bool,\n    relative_region_size: Option<(f64, f64)>,').replace('            is_sub_frame: self.is_sub_frame,','            is_sub_frame: self.is_sub_frame,\n            relative_region_size: self.relative_region_size,').replace('        self.is_sub_frame = checkpoint.is_sub_frame;','        self.is_sub_frame = checkpoint.is_sub_frame;\n        self.relative_region_size = checkpoint.relative_region_size;');p.write_text(s)
p=Path('01_core/src/compiler/layout/sub_frame.rs');s=p.read_text().replace('        let saved_is_sub_frame = self.is_sub_frame;','''        let saved_is_sub_frame = self.is_sub_frame;
        let saved_relative_region = self.relative_region_size;
        let parent_height = self.block_place_region.map(|r| r.h.0)
            .or(saved_relative_region.map(|size| size.1))
            .unwrap_or(self.regions.current.height - self.page_config.margin.top - self.page_config.margin.bottom);
        self.relative_region_size = Some((region.width, region.height.unwrap_or(parent_height)));''').replace('        self.is_sub_frame = saved_is_sub_frame;','        self.is_sub_frame = saved_is_sub_frame;\n        self.relative_region_size = saved_relative_region;')
s=s.replace('''        let cell_height = if !had_items
            && super::raw::terminal_block(content)''','''        let cell_height = if !had_items && super::transform::terminal_native(content) {
            self.flow_placement_end.map_or(0.0, |(end, _)| (end - start_y).max(0.0))
        } else if !had_items
            && super::raw::terminal_block(content)''');p.write_text(s)
p=Path('01_core/src/compiler/layout/transform.rs');s=p.read_text();s=s.replace('''    let available_w = l.available_width();
    let region_h = l.regions.current.height - l.page_config.margin.top - l.page_config.margin.bottom;''','''    let had_line = super::html_frame::items_have_material(&l.regions.current.current_line);
    l.flush_line();
    let spacing = |field: &str| match l.chain.custom(&format!("block.{field}")) {
        Some(Value::Length(v)) => Some(v.resolve_pt(font)), _ => None,
    };
    let above = spacing("above").or_else(|| spacing("spacing")).unwrap_or(font * super::vanilla_defaults::BLOCK_SPACING);
    let below = spacing("below").or_else(|| spacing("spacing")).unwrap_or(font * super::vanilla_defaults::BLOCK_SPACING);
    let mut top = if had_line {
        l.flow_placement_end.map_or(l.prev_line_baseline, |(end, _)| end) + above
    } else if l.block_chain_active {
        l.prev_line_baseline + l.prev_block_equation_descent + l.prev_block_below_pending.max(above)
    } else { l.regions.current.cursor_y.0 };
    let available_w = l.available_width();
    let region_size = l.block_place_region.map(|r| (r.w.0, r.h.0)).or(l.relative_region_size)
        .unwrap_or((available_w, l.regions.current.height - l.page_config.margin.top - l.page_config.margin.bottom));
    let region_h = region_size.1;''')
s=s.replace('v.rel * extent + v.abs.resolve_pt(font)','(if v.rel == 0.0 { 0.0 } else { v.rel * extent }) + v.abs.resolve_pt(font)').replace('styled_relative("dx")), available_w)','styled_relative("dx")), region_size.0)')
a=s.index('    if l.initial_baseline_pending {',s.index('fn layout_native'))
b=s.index('    if let Some(collector)',a)
s=s[:a]+'''    if !l.is_sub_frame && top + frame_h > l.page_bottom_limit() {
        l.new_page();
        top = l.page_config.margin.top;
    }
    let pos = Point { x: l.regions.current.line_start_x, y: Pt(top) };
'''+s[b:]
s=s.replace('''    l.regions.current.current_line.push(FrameItem::Group {
        paint_root: false, pos, matrix, clip_mask: None, inner_width: w, inner_height: h, items,
    });
    l.note_inline_extent(baseline.max(0.0), (frame_h - baseline).max(0.0));
    l.regions.current.cursor_x += Pt(frame_w);''','''    l.regions.current.current_items.push(FrameItem::Group {
        paint_root: false, pos, matrix, clip_mask: None, inner_width: w, inner_height: h, items,
    });
    let end = top + frame_h;
    l.regions.current.cursor_x = l.regions.current.line_start_x;
    l.regions.current.cursor_y = Pt(end + below);
    l.initial_baseline_pending = false;
    l.prev_line_baseline = end;
    l.prev_block_equation_descent = 0.0;
    l.prev_block_below_pending = below;
    l.prev_margin_is_parbreak = false;
    l.block_chain_active = true;
    l.last_block_descent_y = Some(end);
    l.record_flow_placement_end(end);
    l.record_flow_width_at(pos.x.0, frame_w);
    // The frame's baseline remains meaningful for a containing inline box.
    l.last_sub_frame_inline_extents = (baseline, frame_h - baseline);''')
s+='''

pub(super) fn terminal_native(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::Transform(e) => !matches!(e.operation, TransformOperation::Matrix(_)),
        Content::Source(e) => terminal_native(&e.body), Content::Styled(e) => terminal_native(&e.body),
        Content::ShowRecipe(e) => terminal_native(&e.body), Content::Located(e) => terminal_native(&e.body),
        Content::Realized(e) => terminal_native(&e.visual),
        Content::Sequence(parts) => parts.iter().rev().find(|c| !matches!(c, Content::Space | Content::SpaceFromNewline | Content::Empty | Content::Parbreak)).is_some_and(terminal_native),
        _ => false,
    }
}
''';p.write_text(s)
