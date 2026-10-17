from pathlib import Path
import json,subprocess
out=Path('00_nucleo/diagnosticos/tekt-guards-2026-10-16');scope=json.loads((out/'scope.json').read_text());before={p:Path(p).read_text() for p in scope['sources']};replay=before.copy();edits=[]
def edit(path,old,new):
 p=next(p for p in scope['sources'] if p.endswith(path));assert replay[p].count(old)==1,(p,old);replay[p]=replay[p].replace(old,new);edits.append(dict(path=p,old=old,new=new,count=1))
def segment(path,start,end,convert):
 p=next(p for p in scope['sources'] if p.endswith(path));t=replay[p];a=t.index(start);b=t.index(end,a);old=t[a:b];edit(path,old,convert(old))
edit('bindings/field_access/fields.rs','''                None if is_presence_content(c.content())
                    || is_matrix_or_cell_content(c.content()) =>
                {
                    return declared_content_field_access(c.content(), field, span);
                }
                None => c.content().get_field(field),''','''                None => {
                    if is_presence_content(c.content()) || is_matrix_or_cell_content(c.content()) {
                        return declared_content_field_access(c.content(), field, span);
                    }
                    c.content().get_field(field)
                }''')
edit('bindings/field_access/fields.rs','''        Value::Content(c) if is_presence_content(&c) || is_matrix_or_cell_content(&c) => {
            declared_content_field_access(&c, field, span)
        }
        Value::Content(c) => c.get_field(field).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("{} does not have field \\"{field}\\"", c.elem_name()),
            )]
        }),''','''        Value::Content(c) => {
            if is_presence_content(&c) || is_matrix_or_cell_content(&c) {
                declared_content_field_access(&c, field, span)
            } else {
                c.get_field(field).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("{} does not have field \\"{field}\\"", c.elem_name()),
                    )]
                })
            }
        }''')
def native(old):
 guard=old.split('            if ',1)[1].split(' =>\n        {',1)[0]
 return '        FuncRepr::Native(native) => {\n            if '+guard+' {\n                args.span = call_span;\n            }\n        }\n'
segment('call_dispatch/arguments.rs','        FuncRepr::Native(native)\n','        _ => {}',native)
def realize(old):
 start=old.index('            let previous =');end=old.index('        }\n        Value::Content(content) =>')
 body=old[start:end]
 return '''        Value::Content(content) => {
            if has_recipes || crate::compiler::realize::bare_math::needs_preparation(&content) {
'''+body+'''            } else {
                Some(content.wrap_page_columns())
            }
        }
'''
segment('compiler/eval/mod.rs','        Value::Content(content)\n','        _ => None,',realize)
def augment_repr(old):
 start=old.index('            Some(augment) => format!(');fallback=old[start:].removeprefix('            Some(augment) => ').rstrip().removesuffix(',')
 return '''            Some(augment) => {
                if augment.hline.is_empty() && augment.vline.len() == 1 && augment.stroke.is_none() {
                    augment.vline[0].to_string()
                } else {
'''+fallback+'''\n                }
            }
'''
segment('eval/repr/math.rs','            Some(augment)\n','        };\n        fields.push(format!("augment:',augment_repr)
edit('grid/measurement_cache.rs','''        Content::Text(text)
            if !text.is_empty() && text.bytes().all(|b| b.is_ascii_graphic()) =>
        {
            Some(text)
        }''','''        Content::Text(text) => {
            if !text.is_empty() && text.bytes().all(|b| b.is_ascii_graphic()) {
                Some(text)
            } else {
                None
            }
        }''')
edit('grid/measurement_cache.rs','''        Content::GridCell(e) if simple_placement(content) && e.align.is_none() => {
            token(&e.body)
        }''','''        Content::GridCell(e) => {
            if simple_placement(content) && e.align.is_none() {
                token(&e.body)
            } else {
                None
            }
        }''')
edit('grid/measurement_cache.rs','''        Content::TableCell(e) if simple_placement(content) && e.align.is_none() => {
            match &e.resolved {
                Some(r) if r.align.is_none() => token(&r.visual),
                Some(_) => None,
                None => token(&e.body),
            }
        }''','''        Content::TableCell(e) => {
            if simple_placement(content) && e.align.is_none() {
                match &e.resolved {
                    Some(r) if r.align.is_none() => token(&r.visual),
                    Some(_) => None,
                    None => token(&e.body),
                }
            } else {
                None
            }
        }''')
edit('layout/realized.rs','''            other if needs_fresh_index(other) || extract_payload(other).is_some() => {
                complete = false
            }
            _ => {}''','''            other => {
                if needs_fresh_index(other) || extract_payload(other).is_some() {
                    complete = false;
                }
            }''')
def table(old):
 guard=old.split('            if ',1)[1].split(' =>\n        {')[0]
 return '''        Content::TableCell(cell) => {
            if cell.resolved.is_some() {
                content.clone()
            } else if '''+guard+''' {
                visual_content(&cell.body)
            } else {
                content.clone()
            }
        }
'''
segment('layout/table.rs','        Content::TableCell(cell) if cell.resolved.is_some() => content.clone(),','        Content::Sequence(items) =>',table)
edit('layout/text_tracking.rs','''            FrameItem::Text { text, style, .. }
            | FrameItem::TextShaped { text, style, .. }
                if !text.is_empty() && !style.math && style.dir == Some(Dir::RTL) =>
            {
                Pt(style.tracking.map(|t| t.resolve_pt(style.size.0)).unwrap_or(0.0))
            }''','''            FrameItem::Text { text, style, .. }
            | FrameItem::TextShaped { text, style, .. } => {
                if !text.is_empty() && !style.math && style.dir == Some(Dir::RTL) {
                    Pt(style.tracking.map(|t| t.resolve_pt(style.size.0)).unwrap_or(0.0))
                } else {
                    Pt(0.0)
                }
            }''')
edit('layout/text_tracking.rs','''            FrameItem::TextReplacement { items, .. } => continued_rtl_end(items),
            FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }''','''            FrameItem::TextReplacement { items, .. } => continued_rtl_end(items),
            FrameItem::Glyph { .. }''')
edit('stdlib/figure_image.rs','use crate::entities::layout_types::VAlign;','use crate::entities::layout_types::{Align2D, VAlign};')
for pos in ['Top','Bottom']:
 edit('stdlib/figure_image.rs',f'Some(Value::Align(a)) if a.h.is_none() && a.v == Some(VAlign::{pos}) =>',f'Some(Value::Align(Align2D {{ h: None, v: Some(VAlign::{pos}) }})) =>')
# Two identically structured casters, with indentation adjusted by rustfmt.
p=next(p for p in scope['sources'] if p.endswith('stdlib/foundations/color.rs'))
for indent in ['            ','        ']:
 old=indent+'''Value::Relative(relative)
'''+indent+'''    if relative.abs.is_zero() && (0.0..=1.0).contains(&relative.rel) =>
'''+indent+'''{
'''+indent+'''    Ok(relative.rel as f32)
'''+indent+'''}
'''+indent+'''Value::Relative(relative) if relative.abs.is_zero() => {
'''+indent+'''    Err(vec![SourceDiagnostic::error(
'''+indent+'''        Span::detached(),
'''+indent+'''        "ratio must be between 0% and 100%",
'''+indent+'''    )])
'''+indent+'''}'''
 new=indent+'''Value::Relative(relative) if relative.abs.is_zero() => {
'''+indent+'''    if (0.0..=1.0).contains(&relative.rel) {
'''+indent+'''        Ok(relative.rel as f32)
'''+indent+'''    } else {
'''+indent+'''        Err(vec![SourceDiagnostic::error(
'''+indent+'''            Span::detached(),
'''+indent+'''            "ratio must be between 0% and 100%",
'''+indent+'''        )])
'''+indent+'''    }
'''+indent+'''}'''
 edit('stdlib/foundations/color.rs',old,new)
edit('stdlib/pdf.rs','''        Some(Value::Int(value)) if *value > 0 && *value <= u32::MAX as i64 => {
            *value as u32
        }
        Some(Value::Int(_)) => return super::err("number must be positive"),''','''        Some(Value::Int(value)) => {
            if *value > 0 && *value <= u32::MAX as i64 {
                *value as u32
            } else {
                return super::err("number must be positive");
            }
        }''')
edit('structural/math/matrix.rs','''            .map(|value| match value {
                Value::Array(values) if array_mode && !delimited_mode => {
                    Ok(values.iter().map(math_matrix_cell).collect::<Vec<_>>())
                }
                _ if delimited_mode && !array_mode => math_matrix_delimited_row(value)''','''            .map(|value| match (array_mode, delimited_mode, value) {
                (true, false, Value::Array(values)) => {
                    Ok(values.iter().map(math_matrix_cell).collect::<Vec<_>>())
                }
                (false, true, _) => math_matrix_delimited_row(value)''')
edit('structural/table_grid.rs','''        Some(Value::Int(value)) if *value > 0 && *value <= u32::MAX as i64 => {
            *value as u32
        }
        Some(Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "number must be positive",
            )])
        }''','''        Some(Value::Int(value)) => {
            if *value > 0 && *value <= u32::MAX as i64 {
                *value as u32
            } else {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "number must be positive",
                )]);
            }
        }''')
edit('src/entities/color.rs','''            Self::Srgb { r, g, b, a }
                if r == 0.0 && g == 0.454902 && b == 0.85098 && a == 1.0 =>
            {
                f.write_str("blue")
            }
            Self::Srgb { r, g, b, a } => f
                .debug_struct("Srgb")
                .field("r", &r)
                .field("g", &g)
                .field("b", &b)
                .field("a", &a)
                .finish(),''','''            Self::Srgb { r, g, b, a } => {
                if r == 0.0 && g == 0.454902 && b == 0.85098 && a == 1.0 {
                    f.write_str("blue")
                } else {
                    f.debug_struct("Srgb")
                        .field("r", &r)
                        .field("g", &g)
                        .field("b", &b)
                        .field("a", &a)
                        .finish()
                }
            }''')
edit('content/traversal.rs','''                None if !e.span.is_detached() && e.body.source_span().is_detached() => {
                    None
                }
                None => Some(e.body.clone().with_source_span(e.span)),''','''                None => {
                    if !e.span.is_detached() && e.body.source_span().is_detached() {
                        None
                    } else {
                        Some(e.body.clone().with_source_span(e.span))
                    }
                }''')
def augment_field(old):
 start=old.index('                Some(a) => Value::Dict(');fallback=old[start:].removeprefix('                Some(a) => ').rstrip().removesuffix(',')
 return '''                Some(a) => {
                    if a.stroke.is_none() && a.hline.is_empty() && a.vline.len() == 1 {
                        Value::Int(a.vline[0])
                    } else {
'''+fallback+'''\n                    }
                }
'''
segment('elements/math_matrix.rs','                Some(a)\n','            }),\n            _ => None,',augment_field)
edit('gradient/interpolation.rs','''            (Color::Spot(a), Color::Spot(b))
                if a.colorant == b.colorant && a.colorant.as_ref() == &colorant =>
            {
                Color::Spot(crate::entities::color::SpotColor {
                    colorant: a.colorant,
                    tint: Ratio(a.tint.get() + (b.tint.get() - a.tint.get()) * t as f64),
                })
            }''','''            (Color::Spot(a), Color::Spot(b)) => {
                if a.colorant == b.colorant && a.colorant.as_ref() == &colorant {
                    Color::Spot(crate::entities::color::SpotColor {
                        colorant: a.colorant,
                        tint: Ratio(a.tint.get() + (b.tint.get() - a.tint.get()) * t as f64),
                    })
                } else {
                    interpolate_oklab(Color::Spot(a).to_process(), Color::Spot(b).to_process(), t)
                }
            }''')
def link(old):
 body=old.split(' => {\n',1)[1].rsplit('                }',1)[0]
 return '''                FrameItem::Link { items, .. } => {
                    if ua1 && !inside_artifact {
'''+body+'''                    } else {
                        for child in items {
                            collect_top(child, page_index, next, out, inside_artifact, ua1);
                        }
                    }
                }
'''
segment('builder/metadata.rs','                FrameItem::Link { items, .. } if ua1 && !inside_artifact => {','                FrameItem::TextReplacement { items, .. }',link)
edit('builder/metadata.rs','''                FrameItem::TextReplacement { items, .. }
                | FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. } => {''','''                FrameItem::TextReplacement { items, .. }
                | FrameItem::Group { items, .. } => {''')
edit('builder/metadata.rs','''                _ if ua1 && !inside_artifact => {
                    if let Some(node) = ua1_leaf_node(item, page_index, next) {
                        out.push(node);
                    }
                }
                _ => {}''','''                _ => {
                    if ua1 && !inside_artifact {
                        if let Some(node) = ua1_leaf_node(item, page_index, next) {
                            out.push(node);
                        }
                    }
                }''')
def raw(old):
 body=old.split('        {\n',1)[1].rsplit('        }',1)[0]
 return '''        Content::Raw(atom) => {
            if !atom.block
                && atom.lang.as_deref() == Some("typc")
                && matches!(atom.text.as_str(), "auto" | "true" | "false")
            {
'''+body+'''            } else {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "HTML export does not yet support this content",
                ));
            }
        }
'''
segment('html/inline.rs','        Content::Raw(atom)\n','        Content::Text(text) => {',raw)
edit('export/stream.rs','''                    | FrameItem::Image { .. }
                        if ua1 && !inside_artifact =>
                    {
                        out.insert(item as *const FrameItem as usize, *next);
                        *next += 1;
                    }''','''                    | FrameItem::Image { .. } => {
                        if ua1 && !inside_artifact {
                            out.insert(item as *const FrameItem as usize, *next);
                            *next += 1;
                        }
                    }''')
edit('tests/p1428_math_frac.rs','numeric if numeric.starts_with("#x") || numeric.starts_with("#X") => {','numeric if numeric.get(..2).is_some_and(|prefix| prefix.eq_ignore_ascii_case("#x")) => {')
# Stage every replacement in memory before changing any source.
assert len({e['path'] for e in edits})==21
for p,t in before.items():assert Path(p).read_text()==t,p
for p,t in replay.items():Path(p).write_text(t)
(out/'materialization.json').write_text(json.dumps(dict(before=before,edits=edits,unformatted=replay),ensure_ascii=False,indent=2)+'\n')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
print(json.dumps(dict(files=len(replay),replacements=len(edits))))
