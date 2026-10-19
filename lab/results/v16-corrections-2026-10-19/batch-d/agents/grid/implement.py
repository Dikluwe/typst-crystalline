"""Apply reviewed nominal decisions only after the coordinator's baseline gate."""
from pathlib import Path
import re,json
rows={r['id']:r for r in json.load(open('lab/results/v16-corrections-2026-10-19/batch-d/cases.json'))}
def variants(enum,file):
 s=Path(file).read_text().split(f'pub enum {enum} {{',1)[1].split('\n}',1)[0]
 return {m.group(1):m.group(2) for m in re.finditer(r'^    ([A-Z]\w*)\s*([({,])',s,re.M)}
content=variants('Content','01_core/src/entities/content.rs');value=variants('Value','01_core/src/entities/value.rs')
def nominal(names,enum,domain,indent,expr):
 patterns=[f'{enum}::{n}'+{'(':'(..)','{':' { .. }',',':''}[domain[n]] for n in names]
 return ('\n'+indent+'| ').join([indent+patterns[0]]+patterns[1:])+' => '+expr+','
def replace(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old,s.count(old));p.write_text(s.replace(old,new))
def replace_case(n,old,expr,indent):
 r=rows[f'V16-{n:03}'];a=r['analysis'];names=a['nominal_complement'];assert set(names)|set(a['handled_variants'])==set(content)
 replace(a['file'],old,nominal(names,'Content',content,indent,expr))
replace_case(29,'        _ => None, // neutro: N16[β] — Content não-cell retorna None para stroke (sem override de célula)','None','        ')
replace_case(30,'            _ => (None, None, None, None, None), // neutro: N16[β] — Content não-cell retorna tupla de Nones para stroke/fill/align/inset/breakable','(None, None, None, None, None)','            ')
replace_case(31,'        _ => true, // Raw carriers have no explicit placement fields.','true','        ')
p='01_core/src/compiler/layout/grid/measurement_cache.rs'
replace(p,'''        Content::Styled(styled) if local_cell_transport_only(&styled.styles) => {
            token(&styled.body)
        }''','''        Content::Styled(styled) => {
            if local_cell_transport_only(&styled.styles) {
                token(&styled.body)
            } else {
                None
            }
        }''')
replace_case(32,'        _ => None, // No locatable, style scope, context, callback or unknown carrier.','None','        ')
p='01_core/src/compiler/layout/grid/tracks.rs'
s=Path(p).read_text();old='                            _ => None,';assert s.count(old)==2
new=nominal(rows['V16-033']['analysis']['nominal_complement'],'Content',content,'                            ','None');Path(p).write_text(s.replace(old,new))
replace(p,'''            Content::TableCell(e) if e.stage().is_some() => {
                Some(measurement_visual(e.visual()))
            }''','''            Content::TableCell(e) => {
                if e.stage().is_some() {
                    Some(measurement_visual(e.visual()))
                } else {
                    None
                }
            }''')
# Restrict the callback edit by the complete contextual suffix, not another None.
replace(p,'''            Content::ShowRecipe(e) => Some(measurement_visual(&e.body)),
            _ => None,''','''            Content::ShowRecipe(e) => Some(measurement_visual(&e.body)),
'''+nominal(rows['V16-035']['analysis']['nominal_complement'],'Content',content,'            ','None'))
p='01_core/src/compiler/layout/grid_placement.rs'
replace(p,'''        Content::TableCell(e) if table => {
            let (x, y, c, r) = e.placement();
            (x, y, c, r, e.source_span)
        }
        Content::GridCell(e) if !table => {
            (e.x, e.y, e.colspan, e.rowspan, Span::detached())
        }''','''        Content::TableCell(e) => {
            if table {
                let (x, y, c, r) = e.placement();
                (x, y, c, r, e.source_span)
            } else {
                (None, None, None, None, Span::detached())
            }
        }
        Content::GridCell(e) => {
            if table {
                (None, None, None, None, Span::detached())
            } else {
                (e.x, e.y, e.colspan, e.rowspan, Span::detached())
            }
        }''')
replace_case(36,'        _ => (None, None, None, None, Span::detached()),','(None, None, None, None, Span::detached())','        ')
p='01_core/src/entities/elements/table_cell.rs';s=Path(p).read_text();start=s.index('    pub(crate) fn style_inset(');end=s.index('\n}\n',start);body=s[start:end];assert body.count('            _ => None,')==1
body=body.replace('            _ => None,',nominal([n for n in value if n not in ['Length','Dict']],'Value',value,'            ','None'));Path(p).write_text(s[:start]+body+s[end:])
