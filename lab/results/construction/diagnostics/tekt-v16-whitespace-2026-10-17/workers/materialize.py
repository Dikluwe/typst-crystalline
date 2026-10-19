from pathlib import Path
import datetime,gzip,json,re,subprocess
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));t=json.load(open(out/'tests-before-code.json'));scope=json.load(open(out/'scope.json'));assert json.load(open(out/'baseline-result.json'))['exit']!=0;assert json.load(open(out/'probe-baseline-result.json'))['selected_matches']==0;assert '6 passed; 7 failed;' in (out/'baseline-functional-red.log').read_text();files={p:Path(p).read_text() for p in scope['sources']};assert files==t['expected_files'];record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={},functional_edits=[])
for source,s in files.items():
 raw=s.encode();edits=[]
 for r in scope['targets']:
  if r['source']!=source:continue
  a=r['pattern_start_byte'];old=r['pattern_token'].encode();assert raw[a:a+len(old)]==old;edits.append((a,a+len(old),r['replacement'].encode()))
 for a,z,v in sorted(edits,reverse=True):raw=raw[:a]+v+raw[z:]
 files[source]=raw.decode()
def change(source,old,new,count=1):
 assert files[source].count(old)==count,(source,old,files[source].count(old));files[source]=files[source].replace(old,new);record['functional_edits'].append(dict(source=source,old=old,new=new,count=count))
core=scope['sources'][0];html=scope['sources'][2];variants=scope['variants']['Content'];complement=' | '.join(pattern for name,pattern in variants.items() if name not in ['Source','Located','Realized','Styled']);helper='''// Classify a boundary through already supported visual carriers without rewriting it.
fn space_boundary_content(mut content: &Content) -> &Content {
    loop {
        content = match content {
            Content::Source(e) => &e.body,
            Content::Located(e) => &e.body,
            Content::Realized(e) => &e.visual,
            Content::Styled(e) => &e.body,
            '''+complement+''' => return content,
        };
    }
}

'''
change(core,'struct SpaceRun {',helper+'struct SpaceRun {');change(core,'matches!(&parts[end], Content::Empty)','matches!(space_boundary_content(&parts[end]), Content::Empty)');change(core,'matches!(&parts[index], Content::Linebreak(_))','matches!(space_boundary_content(&parts[index]), Content::Linebreak(_))',2);change(html,'    match visual_content(content) {','    let content = visual_content(content);\n    match content {')
for source,s in files.items():
 s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);record['files'][source]=dict(unformatted=s);Path(source).write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',source],check=True);record['files'][source]['expected_after']=Path(source).read_text()
(out/'materialization.json').write_text(json.dumps(record,indent=2)+'\n');print('12 original arms and new boundary helper materialized',flush=True)
