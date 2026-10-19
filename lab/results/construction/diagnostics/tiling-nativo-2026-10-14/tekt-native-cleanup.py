from pathlib import Path
import re
roles={
'03_infra/src/export/render.rs':'Usar libm::hypotf pinado para a norma f32 das colunas da matriz (sx,ky) e (kx,sy), calculada uma vez por eixo; conservar ceil e limiar subpixel. Transportar a raiz lógica já atualizada em State ao chamar render_group, preservando transformação física/máscaras sem argumento booleano adicional.',
'03_infra/src/export/tiling.rs':'Posicionar helpers produtivos antes do módulo de testes, sem alteração de comportamento. O walker privado de recursos enumera explicitamente todas as folhas FrameItem, tornando futuras variantes erro de compilação.',
'03_infra/src/export/stream.rs':'Posicionar build_cell_stream antes do módulo de testes, sem alterar assinatura ou comportamento privado.',
'03_infra/src/export/pdf_tiling.rs':'A varredura enumera explicitamente as folhas FrameItem que não contêm paints, sem wildcard neutro; futuras variantes exigem decisão explícita.',
'03_infra/src/export/svg/paints.rs':'O comparador privado de cache enumera os seis pares de variantes distintas como false, sem wildcard que aceite silenciosamente futuras variantes. Identidade/igualdade do cache não é igualdade de linguagem.'}
for path,role in roles.items():
 owner=re.search(r'^//! @prompt (.+)$',Path(path).read_text(),re.M)[1];p=Path(owner)
 p.write_text(p.read_text()+'''\n## Higiene da migração nativa\n\nMedição: verified-v3/followup-clippy-inventory.json e followup-architecture.log, com candidate-final e workspace-pre próprios, registram quatro f32::hypot proibidos, um argumento adicional, dois itens produtivos depois de módulos de teste e dois wildcards neutros novos.\n\nDecisão interna em fluxo contínuo (ADR-0127): '''+role+''' Não elevar limites nem adicionar allow. Validar suíte e renders antes/depois, além de formato, doc e linhagem.\n''')
p=Path('03_infra/src/export/render.rs');s=p.read_text();s=s.replace('''            render_group(
                canvas,
                state,''','''            let state = State {
                local_to_root: if *paint_root { TransformMatrix::identity() } else {
                    state.local_to_root.concat(&TransformMatrix::translate(pos.x.0, pos.y.0)).concat(matrix)
                },
                ..state
            };
            render_group(
                canvas,
                state,''',1).replace('                *paint_root,\n','',1).replace('    paint_root: bool,\n','',1)
a=s.index('    new_state.local_to_root = if paint_root {');b=s.index('    let mask =',a);s=s[:a]+s[b:];s=s.replace('    let mut new_state = state\n', '    let new_state = state\n',1)
a=s.index('    let px = (pitch_x *');b=s.index('    let period =',a)
s=s[:a]+'''    let scale_x = f64::from(libm::hypotf(device.sx, device.ky));
    let scale_y = f64::from(libm::hypotf(device.kx, device.sy));
    let px = (pitch_x * scale_x).ceil().max(1.0) as u32;
    let py = (pitch_y * scale_y).ceil().max(1.0) as u32;
    // Supersampling resolves coverage even when an entire period is subpixel.
    let high_x = if pitch_x * scale_x < 1.0 { 16 } else { px };
    let high_y = if pitch_y * scale_y < 1.0 { 16 } else { py };
'''+s[b:];p.write_text(s)
p=Path('03_infra/src/export/tiling.rs');s=p.read_text();a=s.index('/// Resource-only traversal:');helper=s[a:];s=s[:a];i=s.index('#[cfg(test)]');s=s[:i]+helper+'\n'+s[i:];s=s.replace('            _ => visitor.visit_item(item),','''            FrameItem::Text { .. } | FrameItem::TextSeparator { .. } | FrameItem::TextShaped { .. }
            | FrameItem::Line { .. } | FrameItem::Glyph { .. } | FrameItem::Image { .. } => visitor.visit_item(item),''');p.write_text(s)
p=Path('03_infra/src/export/stream.rs');s=p.read_text();a=s.index('pub(super) fn build_cell_stream');helper=s[a:];s=s[:a];i=s.index('#[cfg(test)]\nmod ');s=s[:i]+helper+'\n'+s[i:];p.write_text(s)
p=Path('03_infra/src/export/pdf_tiling.rs');s=p.read_text().replace('                _ => {}','''                FrameItem::Text { .. } | FrameItem::TextSeparator { .. } | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. } | FrameItem::Glyph { .. } | FrameItem::Image { .. } => {}''',1);p.write_text(s)
p=Path('03_infra/src/export/svg/paints.rs');s=p.read_text();s=s.replace('        _ => false,','''        (Paint::Solid(_), Paint::Gradient(_) | Paint::Tiling(_))
        | (Paint::Gradient(_), Paint::Solid(_) | Paint::Tiling(_))
        | (Paint::Tiling(_), Paint::Solid(_) | Paint::Gradient(_)) => false,''',1);p.write_text(s)
