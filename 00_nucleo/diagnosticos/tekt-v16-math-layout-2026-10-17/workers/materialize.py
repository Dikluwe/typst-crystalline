from pathlib import Path
import json,gzip,re,hashlib,subprocess,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));t=json.load(open(out/'tests-before-code.json'));baseline=json.load(open(out/'baseline-result.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert baseline['functional_red'] and baseline['sources_unchanged'];assert all(sha(p)==h for p,h in baseline['source_hashes'].items());assert json.load(open(out/'probe-baseline-result.json'))['selected']==0
files={p:Path(p).read_text() for p in scope['sources']};record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':{},'functional_edits':[]}
# Close the measured fallback spans. Tests were appended, so the old offsets remain exact.
for source in scope['existing_sources']:
 raw=files[source].encode();edits=[]
 for r in scope['targets']:
  if r['source']!=source:continue
  if r['optional_content']:
   old=r['match_source'].encode();assert raw.count(old)==1;a=raw.index(old);z=a+len(old)
   new='let c = c?;\n            match c.source_body() {\n                Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),\n                '+r['replacement']+' => None,\n            }'
  else:a,z=r['pattern_start_byte'],r['pattern_end_byte'];assert raw[a:z]==b'_';new=r['replacement']
  edits.append((a,z,new.encode()))
 for a,z,new in sorted(edits,reverse=True):raw=raw[:a]+new+raw[z:]
 files[source]=raw.decode()
def apply(source,old,new):
 s=files[source];assert s.count(old)==1,(source,old[:100],s.count(old));files[source]=s.replace(old,new,1);record['functional_edits'].append({'source':source,'old':old,'new':new})
def function_range(s,name):
 matches=list(re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?fn '+re.escape(name)+r'(?:<[^\n]+>)?\(',s,re.M));assert len(matches)==1,(name,len(matches));a=matches[0].start();o=s.index('{',matches[0].end());z=o+1;depth=1
 while depth:depth+=(s[z]=='{')-(s[z]=='}');z+=1
 return a,z
def within(source,name,old,new,count=1):
 s=files[source];a,z=function_range(s,name);body=s[a:z];assert body.count(old)==count,(source,name,body.count(old),old);after=body.replace(old,new);apply(source,body,after)
base='01_core/src/compiler/math/layout/'
p=base+'attach.rs';within(p,'layout_attach','        let base_box = self.layout_node(base, style);','        let base = base.source_body();\n        let base_box = self.layout_node(base, style);');within(p,'layout_attach','            loop {\n                match b {','            loop {\n                b = b.source_body();\n                match b {');within(p,'layout_attach','match &e.text {','match e.text.source_body() {',2);within(p,'content_is_text_like','match c {','match c.source_body() {')
p=base+'accent.rs';within(p,'extract_base_char','match c {','match c.source_body() {');within(p,'layout_accent_resolved','let one_char = |c: &Content| match c {','let one_char = |c: &Content| match c.source_body() {');within(p,'atom_identity','match base {','match base.source_body() {')
s=files[p];a,z=function_range(s,'dotless_base');old=s[a:z];new=old.replace('    match content {','    let transformed = match content.source_body() {',1);assert new!=old;assert new.rstrip().endswith('}\n}');end=new.rfind('    }');new=new[:end]+'    };\n    transformed.with_source_span(content.source_span())'+new[end+5:];apply(p,old,new)
p=base+'underover.rs'
for name in ['piece_char','nested_spreader','base_char_for_attachment']:within(p,name,'match content {','match content.source_body() {')
within(p,'layout_underover_with_attach','match base {','match base.source_body() {')
p=base+'matrix.rs';within(p,'split_cell_on_align_point','match cell {','let parts = match cell.source_body() {');within(p,'split_cell_on_align_point','matches!(item, Content::MathAlignPoint(_))','matches!(item.source_body(), Content::MathAlignPoint(_))');s=files[p];a,z=function_range(s,'split_cell_on_align_point');old=s[a:z];end=old.rfind('    }');new=old[:end]+'    };\n    parts.into_iter().map(|part| part.with_source_span(cell.source_span())).collect()'+old[end+5:];apply(p,old,new);within(p,'edge_node','    if let Content::MathSequence(items) = cell {','    let cell = cell.source_body();\n    if let Content::MathSequence(items) = cell {')
p=base+'spacing.rs';within(p,'is_textual_spaced_item','match content {','match content.source_body() {');within(p,'is_textual_spaced_item','                child,','                child.source_body(),');within(p,'is_bare_pipe','match content {','match content.source_body() {')
p=base+'sequence_composition.rs';within(p,'layout_sequence','matches!(n, Content::MathAlignPoint(_) | Content::Linebreak(_))','matches!(n.source_body(), Content::MathAlignPoint(_) | Content::Linebreak(_))');within(p,'layout_sequence','matches!(n, Content::HSpace(e) if e.weak)','matches!(n.source_body(), Content::HSpace(e) if e.weak)');within(p,'layout_sequence','!match next {','!match next.source_body() {');within(p,'layout_sequence','let Content::MathText(text) = node else {','let Content::MathText(text) = node.source_body() else {');within(p,'layout_sequence','(Content::MathText(trimmed.into()), true)','(Content::MathText(trimmed.into()).with_source_span(node.source_span()), true)');old='''                if let Some(Content::Text(text)) = spacing_view.get(index + 1).cloned() {
                    spacing_view[index + 1] = Content::MathText(text);
                }''';new='''                if let Some(next) = spacing_view.get(index + 1) {
                    if let Content::Text(text) = next.source_body() {
                        let replacement = Content::MathText(text.clone()).with_source_span(next.source_span());
                        spacing_view[index + 1] = replacement;
                    }
                }''';within(p,'layout_sequence',old,new)
p=base+'stretch.rs';within(p,'leaf_char','match content {','match content.source_body() {')
p=base+'mod.rs';within(p,'layout_equation_measured','match &transformed {','match transformed.source_body() {');within(p,'single_math_leaf_char','match content {','match content.source_body() {');within(p,'should_tighten_text_frame','matches!((node, next),','matches!((node.source_body(), next.map(Content::source_body)),');within(p,'needs_grid_layout','matches!(c, Content::MathAlignPoint(_) | Content::Linebreak(_))','matches!(c.source_body(), Content::MathAlignPoint(_) | Content::Linebreak(_))')
# The forwarding branch preserves the original input, avoiding a bound normalized alias.
within(p,'partition_grid','match node {','match node.source_body() {');part_complement=' | '.join(v for n,v in scope['variants']['Content'].items() if n not in ['Linebreak','MathAlignPoint']);within(p,'partition_grid','''            other => {
                lines.last_mut().unwrap().last_mut().unwrap().push(other.clone());
            }''','''            '''+part_complement+''' => {
                lines.last_mut().unwrap().last_mut().unwrap().push(node.clone());
            }''')
within(p,'flatten_math_sequence_nodes','    for node in nodes {\n        match node {','    for node in nodes {\n        let start = out.len();\n        let span = node.source_span();\n        match node.source_body() {');within(p,'flatten_math_sequence_nodes','                        child,','                        child.source_body(),');s=files[p];a,z=function_range(s,'flatten_math_sequence_nodes');old=s[a:z];needle='''            other => out.push(other.clone()),
        }
    }''';replacement='''            other => out.push(other.clone()),
        }
        if !span.is_detached() {
            for child in &mut out[start..] {
                if child.source_span().is_detached() {
                    *child = child.clone().with_source_span(span);
                }
            }
        }
    }''';assert old.count(needle)==1;apply(p,old,old.replace(needle,replacement,1));oldcomment='// neutro: N16[γ] — fallback aberto: nós não listados assumem rota de texto math; novas variantes de container/layout exigirão braço explícito sob evolução de AST';apply(p,oldcomment,'// Complemento nominal atual: estes nós seguem a rota de texto math')

for source,s in files.items():
 s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);Path(source).write_text(s);record['files'][source]={'unformatted':s}
subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',*scope['sources']],check=True)
for source in files:record['files'][source]['expected_after']=Path(source).read_text()
record['partition_complement']=part_complement;(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('36 fallbacks closed, Source repairs applied, partition forwarding nominal; no public API changes',flush=True)
