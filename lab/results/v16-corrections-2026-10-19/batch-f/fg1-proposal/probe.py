#!/usr/bin/env python3
import pathlib,json,datetime,hashlib,subprocess
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[4]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
oracle=ROOT/'lab/typst-original/target/release/typst'
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
show='#show figure: it => [#metadata((kind: repr(it.kind), kind-type: repr(type(it.kind)))) <probe>]\n'
cases=[
('metadata-content',show+'#figure(metadata(table([A])))'),
('metadata-array',show+'#figure(metadata((1, (table([A]),))))'),
('metadata-dict',show+'#figure(metadata((nested: table([A]),)))'),
('metadata-before-raw',show+'#figure([#metadata(table([A]))#raw("x")])'),
('metadata-dict-before-raw',show+'#figure([#metadata((nested: table([A]),))#raw("x")])'),
('context-opaque',show+'#figure(context table([A]))'),
('show-product-opaque','#show emph: it => table([A])\n'+show+'#figure(emph([A]))'),
('set-kind',show+'#set figure(kind: raw)\n#figure(table([A]))'),
('explicit-auto',show+'#set figure(kind: raw)\n#figure(table([A]),kind:auto)'),
('filtered-show-set','#show figure.where(kind: table): set figure(kind: raw)\n'+show+'#figure(table([A]))'),
('custom-name-no-supplement',show+'#figure([A],kind:"table")'),
('custom-name-supplement',show+'#figure([A],kind:"table",supplement:[Custom])'),
('custom-name-numbering-none',show+'#figure([A],kind:"table",numbering:none)'),
('callback-with-caption','#counter(figure.where(kind: table)).update(n=>panic("FG1_CALLBACK"))\n#figure(table([A]),caption:[C])'),
('callback-no-caption','#counter(figure.where(kind: table)).update(n=>panic("FG1_CALLBACK"))\n#figure(table([A]))'),
('callback-numbering-none','#counter(figure.where(kind: table)).update(n=>panic("FG1_CALLBACK"))\n#figure(table([A]),caption:[C],numbering:none)'),
('callback-hidden','#counter(figure.where(kind: table)).update(n=>panic("FG1_CALLBACK"))\n#show figure: none\n#figure(table([A]),caption:[C])'),
('counter-update-prefix','#counter(figure.where(kind: table)).update(7)\n#figure(table([A]),caption:[C]) <fig>\n#context metadata(counter(figure.where(kind: table)).at(<fig>)) <probe>'),
]
record={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','oracle_sha256':sha(oracle),'runner_sha256':sha(pathlib.Path(__file__)),'cases':[]}
for name,source in cases:
 p=HERE/'probes'/f'{name}.typ';p.parent.mkdir(exist_ok=True);p.write_text(source+'\n');argv=[str(oracle),'query',str(p),'<probe>','--field','value'];t=utc();r=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=20);record['cases'].append({'name':name,'source':source+'\n','source_sha256':sha(p),'argv':argv,'utc_start':t,'utc_end':utc(),'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr})
record['utc_end']=utc();record['oracle_sha256_after']=sha(oracle);out=HERE/'oracle-boundaries.json';assert notout if False else True
assert not out.exists();out.write_text(json.dumps(record,indent=2)+'\n');print(json.dumps([(c['name'],c['returncode'],c['stdout'].strip(),c['stderr'].splitlines()[0]) for c in record['cases']],indent=2))
