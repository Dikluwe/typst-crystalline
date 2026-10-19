from pathlib import Path
import subprocess,re,json,hashlib
out=Path('00_nucleo/diagnosticos/native-element-catalogue-2026-10-17')
pin='9f2b6e8715237cb086899a42873660fe744622e8'
cases=[('std.title','model/title.rs','TitleElem','native_title'),('std.document','model/document.rs','DocumentElem','native_document'),('std.divider','model/divider.rs','DividerElem','native_divider'),('std.list.item','model/list.rs','ListItem','native_list_item'),('std.enum.item','model/enum.rs','EnumItem','native_enum_item'),('std.terms.item','model/terms.rs','TermItem','native_terms_item'),('std.footnote.entry','model/footnote.rs','FootnoteEntry','native_footnote_entry'),('std.place.flush','layout/place.rs','FlushElem','native_flush'),('std.math.op','math/op.rs','OpElem','native_op')]+[('std.curve.'+name,'visualize/curve.rs','Curve'+name.title(),'native_curve_'+name) for name in ['move','line','quad','cubic','close']]+[('std.asset','../typst-bundle/src/format.rs','AssetElem','native_asset')]
rows=[]
for expr,part,struct,callback in cases:
 path=('crates/typst-library/src/'+part) if not part.startswith('../') else 'crates/typst-bundle/src/format.rs'
 raw=subprocess.check_output(['git','show',pin+':'+path],text=True)
 start=raw.index('pub struct '+struct+' {');end=(raw.index('}',start)+1 if raw[raw.index('{',start)+1:raw.index('}',start)].strip()=='' else raw.index('\n}',start)+2); attr=raw.rfind('#[elem',0,start)
 block=raw[start:end];a=raw[attr:start];fields=re.findall(r'^    pub (\w+):',block,re.M)
 rows.append(dict(expression=expr,struct=struct,callback=callback,source=path,line=raw[:start].count('\n')+1,sha256=hashlib.sha256(raw.encode()).hexdigest(),attributes=a.strip(),block=block,fields=[s.replace('_','-') for s in fields],locatable='Locatable' in a,abi=6 if callback=='native_op' else 4))
(out/'batch-schema.json').write_text(json.dumps(dict(pin=pin,cases=rows),indent=2)+'\n')
for x in rows: print(x['expression'],x['attributes'],x['fields'],x['locatable'])
