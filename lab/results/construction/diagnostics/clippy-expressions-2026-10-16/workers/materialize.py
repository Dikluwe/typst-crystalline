from pathlib import Path
import gzip,json,re,datetime,hashlib,subprocess,collections
out=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16');b=json.load(gzip.open(out/'before.json.gz'));messages=json.loads((out/'initial-diagnostics.json').read_text());changes=collections.defaultdict(list)
def add(p,start,end,new,reason):
 old=b['files'][p]['text'].encode()[start:end].decode();changes[p].append(dict(start=start,end=end,old=old,new=new,reason=reason))
def replace(p,old,new,reason):
 raw=b['files'][p]['text'].encode();a=old.encode();assert raw.count(a)==1,(p,old[:90]);start=raw.index(a);add(p,start,start+len(a),new,reason)
for m in messages:
 primary=next(s for s in m['spans'] if s['is_primary']);p=primary['file_name'];rule=m['code']['code']
 if rule in ['clippy::regex_creation_in_loops','clippy::needless_range_loop']:continue
 suggestions=[c for c in m['children'] if any(s.get('suggested_replacement') is not None for s in c.get('spans',[]))];assert len(suggestions)==1,(p,rule)
 for s in suggestions[0]['spans']:
  new=s.get('suggested_replacement')
  if new is None:continue
  if p.endswith('integration_tests.rs') and rule=='clippy::collapsible_else_if':
   oldbranch="""                } else if c == '<' {
                    in_hex = true;
                } else if i + 1 < text.len() && c == '<' && bytes[i + 1] == b'<' {
                    depth += 1;
                    i += 1;
"""
   newbranch="""                } else if i + 1 < text.len() && c == '<' && bytes[i + 1] == b'<' {
                    depth += 1;
                    i += 1;
                } else if c == '<' {
                    in_hex = true;
"""
   assert oldbranch in new;new=new.replace(oldbranch,newbranch)
  assert '<item>' not in new
  add(p,s['byte_start'],s['byte_end'],new,rule)
p='03_infra/src/integration_tests.rs'
replace(p,'            let end = i.saturating_sub(2);\n            objects.insert(id, text[start..end].to_string());','            assert_eq!(depth, 0, "unterminated PDF dictionary observation");\n            let end = i.saturating_sub(2);\n            objects.insert(id, text[start..end].to_string());','observer: reject truncated body')
anchor='    /// Devolve os valores de `/Count` presentes apenas na árvore de bookmarks.\n';newtests=(out/'observer-tests-before-implementation.rs.txt').read_text();replace(p,anchor,newtests+anchor,'independent observer regressions retained')
p='04_wiring/tests/crystalline_lint.rs';replace(p,'use std::path::PathBuf;','use std::path::{Path, PathBuf};','borrowed Path helper import')
p='04_wiring/tests/p1293_contract.rs'
old='''            for index in 0..4 {
                if (summary.0[index] - expected[index]).abs() > 0.01 {
                    violations.push(format!(
                        "B-P07 {side}/{name} viewBox[{index}] expected {} +/- 0.01, got {}",
                        expected[index], summary.0[index]
                    ));
                }
            }'''
new='''            for (index, expected_value) in expected.iter().enumerate() {
                if (summary.0[index] - *expected_value).abs() > 0.01 {
                    violations.push(format!(
                        "B-P07 {side}/{name} viewBox[{index}] expected {} +/- 0.01, got {}",
                        expected_value, summary.0[index]
                    ));
                }
            }'''
replace(p,old,new,'enumerate all four expected coordinates; retain indexing of observation')
p='03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs'
replace(p,'    let mut starts = Vec::new();\n    for entry in refs.captures_iter',r'''    let start_re = Regex::new(r"/St\s+(\d+)").unwrap();
    let style_re = Regex::new(r"/S\s*/([A-Za-z]+)").unwrap();
    let mut starts = Vec::new();
    for entry in refs.captures_iter''','compile fixed patterns once per observation')
replace(p,'        let start = Regex::new(r"/St\\s+(\\d+)")\n            .unwrap()\n            .captures(&label)','        let start = start_re.captures(&label)','regex loop start capture unchanged')
replace(p,'        let style = Regex::new(r"/S\\s*/([A-Za-z]+)")\n            .unwrap()\n            .captures(&label)','        let style = style_re.captures(&label)','regex loop style capture unchanged')
# Before changing behavior, require actual RED evidence from both independent regressions.
r=json.loads((out/'observer-red-result.json').read_text());assert r['exit']==101
log=(out/'observer-red.log').read_text();assert '2 failed' in log and '0 passed' in log
for p,edits in changes.items():
 expected=b['files'][p]['text']
 if p.endswith('/integration_tests.rs'):expected=expected.replace(anchor,newtests+anchor)
 assert Path(p).read_text()==expected,p
 raw=b['files'][p]['text'].encode();ascending=sorted(edits,key=lambda e:(e['start'],e['end']));assert all(a['end']<=z['start'] for a,z in zip(ascending,ascending[1:])),p
 for e in sorted(edits,key=lambda e:(e['start'],e['end']),reverse=True):
  assert raw[e['start']:e['end']].decode()==e['old'];raw=raw[:e['start']]+e['new'].encode()+raw[e['end']:]
 Path(p).write_bytes(raw)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*changes],check=True)
(out/'edits.json').write_text(json.dumps(changes,ensure_ascii=False,indent=2)+'\n')
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files=list(changes),changes=sum(map(len,changes.values())),new_observer_tests=2),indent=2)+'\n')
print('materialized',len(changes),'consumers')
