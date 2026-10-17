from pathlib import Path
import json,subprocess,hashlib,datetime
out=Path(__file__).resolve().parents[1];source=Path('01_core/src/compiler/layout/paragraph.rs').read_text();part=source[source.index("enum ParagraphBody<'a>"):source.index('\npub(super) fn layout<',source.index("enum ParagraphBody<'a>"))];variants=json.load(open(out/'content-variants.json'));definitions=[]
for v in variants:
 n=v['name'];shape=v['shape']
 if shape=='(':definitions.append(n+'('+{'Source':'Box<Carrier>','Styled':'Box<Carrier>','Text':'String','Sequence':'Vec<Content>'}.get(n,'()')+')')
 elif shape=='{':definitions.append(n+' { '+('body: Box<Content>' if n=='Par' else 'marker: ()')+' }')
 else:definitions.append(n)
base='#![allow(dead_code)]\nstruct Carrier { body: Content }\nenum Content { '+','.join(definitions)+' }\n'+part;records=[]
for mode in ['control','content-future','projection-future']:
 text=base
 if mode=='content-future':text=text.replace('enum Content {','enum Content { TektParagraphFuture,',1)
 if mode=='projection-future':text=text.replace("enum ParagraphBody<'a> {","enum ParagraphBody<'a> {\n    TektParagraphFuture,",1)
 p=out/('probe-'+mode+'.rs');p.write_text(text);cmd=['rustc','--edition=2021','--crate-type=lib','--emit=metadata','--error-format=json',str(p),'-o',str(out/'probe.rmeta')];r=subprocess.run(cmd,capture_output=True,text=True);messages=[json.loads(line) for line in r.stderr.splitlines() if line.startswith('{')];errors=[e for e in messages if (e.get('code') or {}).get('code')=='E0004'];nodes=json.loads(subprocess.check_output(['/tmp/tekt-table-cells-latent',str(p),'matches'],text=True));functions=[]
 for error in errors:
  spans=[sp for sp in error['spans'] if sp['is_primary']];assert spans
  functions += [n['function'] for n in nodes if n['line']<=spans[0]['line_start']<=n['end_line']]
 expected={'control':[],'content-future':['paragraph_body'],'projection-future':['is_paragraph','is_empty_body','contains_empty_text']}[mode];assert sorted(functions)==sorted(expected),(mode,functions,r.stderr);assert r.returncode==(0 if mode=='control' else 1)
 assert all((e.get('code') or {}).get('code')=='E0004' for e in messages if e.get('level')=='error' and e.get('code'))
 records.append(dict(mode=mode,command=cmd,exit=r.returncode,functions=functions,stderr=r.stderr))
(out/'probe.rmeta').unlink(missing_ok=True);(out/'probe-result.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(source.encode()).hexdigest(),method='Exact four production matches and private projection extracted; all 120 Content variant shapes, unrelated payloads stubbed; structural compile-time proof, not a runtime or full workspace mutation.',records=records),indent=2)+'\n');print('Control PASS; future Content rejected at boundary; future projection rejected at all three predicates.')
