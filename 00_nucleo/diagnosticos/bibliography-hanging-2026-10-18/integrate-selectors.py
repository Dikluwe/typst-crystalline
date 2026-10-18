from pathlib import Path
base=Path('01_core/src/compiler/layout')
f=base/'paragraph_select.rs';s=f.read_text();a=s.index('pub(super) fn select(');b=s.index('\n#[cfg(test)]',a);old=s[a:b]
shaped=old.replace('fn select(','fn select_shaped(',1).replace('    width: f64,','    shape: super::paragraph_indent::Shape,',1)
shaped=shaped.replace('        while start < n {\n            let mut end', '        while start < n {\n            let width = shape.width(start == 0);\n            let mut end')
shaped=shaped.replace('        for start in first_active..end {\n            let line', '        for start in first_active..end {\n            let width = shape.width(start == 0);\n            let line')
facade=old[:old.index('    let n =')]+'''    select_shaped(p, super::paragraph_indent::Shape::uniform(width), font, justify, strategy, costs)
}
'''
s=s[:a]+facade+'\n'+shaped+s[b:];f.write_text(s)
f=base/'paragraph_hyphen.rs';s=f.read_text();a=s.index('pub(super) fn select(');b=s.index('\n#[cfg(test)]',a);old=s[a:b]
shaped=old.replace('fn select(','fn select_shaped(',1).replace('    width: f64,','    shape: super::paragraph_indent::Shape,',1)
shaped=shaped.replace('        for end in 1..n {\n            let mut m', '        for end in 1..n {\n            let mut width = shape.width(start == 0);\n            let mut m')
shaped=shaped.replace('                    start = fitting;\n                    m =', '                    start = fitting;\n                    width = shape.width(start == 0);\n                    m =')
shaped=shaped.replace('            for start in first_active..end {\n                let m', '            for start in first_active..end {\n                let width = shape.width(start == 0);\n                let m')
facade=old[:old.index('    let points =')]+'''    select_shaped(p, super::paragraph_indent::Shape::uniform(width), font, justify, strategy, costs)
}
'''
s=s[:a]+facade+'\n'+shaped+s[b:];f.write_text(s)
f=base/'paragraph_prepare.rs';s=f.read_text();old='    style.dir != Some(Dir::RTL) && !style.math && !l.smallcaps';assert s.count(old)==1;s=s.replace(old,'    (style.dir != Some(Dir::RTL) || (l.bidi_cursor.is_none() && style.tracking.is_none()))\n        && !style.math && !l.smallcaps');f.write_text(s)
f=base/'paragraph_plan.rs';s=f.read_text()
needle='    let Some(p) = paragraph_prepare::prepare(l, body) else { return false };';assert s.count(needle)==1;s=s.replace(needle,needle+'\n    super::paragraph_indent::start(l);\n    let shape = super::paragraph_indent::shape(l, width);')
needle='    let regional_only = implicit_regional_only(l);';positions=[m for m in range(len(s)) if s.startswith(needle,m)];assert len(positions)==2
idx=positions[1];s=s[:idx]+'    super::paragraph_indent::start_scope(l, parts);\n    let shape = super::paragraph_indent::shape(l, width);\n'+s[idx:]
s=s.replace('    let mut prepared = Vec::new();\n    for segment', '    let mut prepared = Vec::new();\n    let mut first_material = true;\n    for segment')
needle='        let Some(p) = paragraph_prepare::prepare_parts(l, segment) else { return false };';assert s.count(needle)==1
s=s.replace(needle,needle+'''\n        let shape = if !first_material && l.pending_bib_escape.is_some() {
            super::paragraph_indent::Shape::uniform(shape.width(false))
        } else { shape };
        first_material = false;''')
# Calls retain the uniform facade where no new shape is needed.
s=s.replace('super::paragraph_hyphen::select(&p, width, font, justify, strategy, costs)', '''if shape.is_uniform() { super::paragraph_hyphen::select(&p, shape.width(true), font, justify, strategy, costs) }
            else { super::paragraph_hyphen::select_shaped(&p, shape, font, justify, strategy, costs) }''')
s=s.replace('super::paragraph_hyphen::select(\n                &p, width, font, justify, strategy, costs,\n            )', '''if shape.is_uniform() { super::paragraph_hyphen::select(&p, shape.width(true), font, justify, strategy, costs) }
            else { super::paragraph_hyphen::select_shaped(&p, shape, font, justify, strategy, costs) }''')
s=s.replace('paragraph_select::select(&p, width, font, justify, strategy, costs)', '''if shape.is_uniform() { paragraph_select::select(&p, shape.width(true), font, justify, strategy, costs) }
            else { paragraph_select::select_shaped(&p, shape, font, justify, strategy, costs) }''')
f.write_text(s)
f=base/'paragraph_emit.rs';s=f.read_text();s=s.replace('    let saved = l.style.clone();','    super::paragraph_indent::start(l);\n    let saved = l.style.clone();');needle='    let m = measure(p, line);';assert s.count(needle)==1;s=s.replace(needle,'    let width = super::paragraph_indent::current_width(l, width);\n'+needle);f.write_text(s)
# Zero-shape calls keep their previous sizing authority exactly.
f=base/'paragraph_indent.rs';s=f.read_text();s=s.replace('    l.paragraph_indent.spec.map_or(fallback, |_| {\n        let (left,right)=bounds(l);right-left\n    })','''    l.paragraph_indent.spec.map_or(fallback, |spec| {
        if spec.first == 0.0 && spec.continuation == 0.0 { fallback }
        else { let (left,right)=bounds(l);right-left }
    })''');f.write_text(s)
f=base/'text.rs';s=f.read_text().replace('        super::text_baseline::admit(layouter);\n        super::paragraph_indent::admit(layouter);','        super::text_baseline::admit(layouter);');f.write_text(s)
