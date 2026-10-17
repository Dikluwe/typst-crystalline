from pathlib import Path
import json,re
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');items=json.loads((out/'items.json').read_text());scope=json.loads((out/'scope.json').read_text());groups=[]
type_names={
 'cli':['WatchedChild'], 'p1291_callback_runtime':['Scratch'], 'p1292_contract':['Scratch','SvgTransform'],
 'p1293_contract':['Transcript','Scratch','DomNode','ParsedDom'],
 'p1462_format_svg':['TempDir','Checks'], 'p1463_format_png':['TempDir','Checks'],
 'p1464_format_pdf':['ProcessResult','Checks'], 'p1465_format_html':['ProcessResult','Checks'],
 'p1467_format_pdf_tagged_pretty':['ProcessResult'], 'p1468_format_pdf_pages':['ProcessResult'], 'p1469_format_pdf_standard':['ProcessResult']}
extra={'p1293_contract':['trim_transport','decode_entities','tag_end','parse_open_tag','push_dom_node','parse_dom','visit_elements'], 'p1462_format_svg':['NEXT_TEMP'], 'p1463_format_png':['NEXT_TEMP']}
imports={
 'cli':'use std::process::Child;\n',
 'p1291_callback_runtime':'use std::{env, fs};\nuse std::path::PathBuf;\n',
 'p1292_contract':'use std::{env, fs};\nuse std::path::PathBuf;\n',
 'p1293_contract':'use std::{env, fs};\nuse std::path::PathBuf;\nuse std::process::Output;\n',
 'p1462_format_svg':'use std::fs;\nuse std::path::PathBuf;\nuse std::sync::atomic::{AtomicU64, Ordering};\n',
 'p1463_format_png':'use std::fs;\nuse std::path::PathBuf;\nuse std::sync::atomic::{AtomicU64, Ordering};\n'}
for source in [p for p in scope['sources'] if '/tests/' in p]:
 stem=Path(source).stem;selected=[r for r in items if r['source']==source and ((r['kind'] in ['struct_item','enum_item','impl_item'] and r['name'] in type_names[stem]) or r['name'] in extra.get(stem,[]))];assert selected and not any('#[test]' in r['text'] for r in selected)
 names={r['name'] for r in selected};assert set(type_names[stem])<=names
 parent=Path(source).read_bytes()
 for r in sorted(selected,key=lambda r:r['start'],reverse=True):assert parent[r['start']:r['end']].decode()==r['text'];parent=parent[:r['start']]+parent[r['end']:]
 exports=sorted(n for n in names if re.search(r'\b'+re.escape(n)+r'\b',parent.decode()))
 helper='03_infra/tests/support/wiring/'+Path(source).name;owner='00_nucleo/prompts/infra/test-support/wiring/'+stem+'.md';oldowner=next(r['owner'] for r in scope['rows'] if r['source']==source)
 group=dict(source=source,owner=oldowner,helper=helper,helper_owner=owner,types=type_names[stem],items=selected,exports=exports,imports=imports.get(stem,''),warnings=[r for r in scope['rows'] if r['source']==source]);groups.append(group)
 assert sum(r['kind'] in ['enum_item','impl_item'] and not r['text'].startswith('impl Drop') for r in selected)==len(group['warnings'])
(out/'classification.json').write_text(json.dumps(groups,ensure_ascii=False,indent=2)+'\n')
print([(Path(g['source']).stem,len(g['warnings']),g['exports']) for g in groups])
