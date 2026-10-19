import subprocess,hashlib,json,datetime,pathlib
root=pathlib.Path.cwd(); out=root/'lab/results/v16-corrections-2026-10-19/batch-c/agents/effects'; oracle=root/'lab/typst-original/target/release/typst'
sha=hashlib.sha256(oracle.read_bytes()).hexdigest(); assert sha=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
run=lambda a:subprocess.run(a,capture_output=True,text=True)
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':run(['git','rev-parse','HEAD']).stdout.strip(),'diffstat':run(['git','diff','HEAD','--stat']).stdout,'status':run(['git','status','--short']).stdout,'oracle':{'sha256':sha,'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','receipt':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/oracle-build.json.gz'},'probes':[]}
(out/'module.typ').write_text('*MODULE*')
cases={
 'warning-state-direct':'#{ let s = state("k", 0); s.update(1); return "x" }',
 'warning-state-block':'#{ let s = state("k", 0); [#s.update(1)]; return "x" }',
 'warning-state-strong':'#{ let s = state("k", 0); strong(s.update(1)); return "x" }',
 'warning-state-text':'#{ let s = state("k", 0); text(s.update(1)); return "x" }',
 'warning-state-value':'#{ state("k",0); [X]; return "x" }',
 'warning-counter-display':'#let f()={ counter("k").display(); return "x" }\n#context f()',
 'warning-context-hidden':'#{ context { panic("EXECUTED"); state("k",0).update(1) }; return "x" }',
 'display-markup':'#import "module.typ" as m\n#metadata(repr([#true #auto #none #(1.0) #decimal("1.50") #sym.alpha #sys.version #state("k",0) #counter("k") #m #((x)=>x) #(1,2)])) <probe>',
 'display-tail':'#import "module.typ" as m\n#metadata((true, auto, none, 1.0, decimal("1.50"), sym.alpha, sys.version, state("k",0), counter("k"), m, ((x)=>x), (1,2)).map(v=>repr({set text(size: 12pt);v}))) <probe>',
 'display-math':'#import "module.typ" as m\n#metadata(repr($ #true #auto #none #(1.0) #decimal("1.50") #sym.alpha #sys.version #state("k",0) #counter("k") #m #((x)=>x) #(1,2) $)) <probe>',
 'display-located':'#heading[*RICH*] <rich>\n#context {let c=query(<rich>).first();metadata((repr([#c]),repr({set text(size:12pt);c}),repr($ #c $)))}',
 'flow-located-par':'#show par: set text(fill:red)\n#par[A] <rich>\n#context {let c=query(<rich>).first(); c}',
 'flow-show-par':'#show strong: it => par(it.body)\n#show par: set text(fill:red)\n*ONE*\n\nTWO',
}
for name,src in cases.items():
 p=out/(name+'.typ'); p.write_text(src)
 args=[str(oracle),'query',str(p),'metadata','--field','value'] if name.startswith('display') else [str(oracle),'compile',str(p),str(out/(name+'.pdf'))]
 r=run(args); state['probes'].append({'name':name,'source':src,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'argv':args,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr})
 for suffix,body in [('stdout',r.stdout),('stderr',r.stderr)]: (out/(name+'.'+suffix)).write_text(body)
 print(name,r.returncode,r.stdout[:130],r.stderr[:100])
state['source_hashes']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [pathlib.Path('01_core/src/compiler/eval/expression_dispatch.rs'),pathlib.Path('01_core/src/compiler/eval/markup_stream.rs'),pathlib.Path('01_core/src/compiler/eval/rules/realize.rs'),pathlib.Path('lab/typst-original/crates/typst-library/src/foundations/value.rs'),pathlib.Path('lab/typst-original/crates/typst-eval/src/code.rs')]}
(out/'probes-refined.json').write_text(json.dumps(state,indent=2)+'\n')
