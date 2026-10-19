#!/usr/bin/env python3
"""Reproduce Figure boundary observations without building or mutating product code."""
import datetime,gzip,hashlib,json,pathlib,subprocess
R=pathlib.Path(__file__).resolve().parents[6]
D=pathlib.Path(__file__).resolve().parent
C=R/'lab/results/v16-corrections-2026-10-19/circle-origin'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args): return subprocess.check_output(['git',*args],cwd=R,text=True)
receipt=json.loads((C/'accepted-build-receipt.json').read_text())
statefile=pathlib.Path(receipt['state']['path']); assert sha(statefile)==receipt['state']['sha256']
state=json.loads(gzip.decompress(statefile.read_bytes()))
inputs={p:sha(R/p) for p in state['inputs']}; assert inputs==state['inputs'], 'Product inputs changed since accepted build'
product=pathlib.Path(receipt['product']['path']); assert sha(product)==receipt['product']['sha256']
oracle=R/'lab/typst-original/target/release/typst'; oracle_sha='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'; assert sha(oracle)==oracle_sha
old=json.loads((R/'lab/results/v16-corrections-2026-10-19/batch-f/fg1-proposal/oracle-boundaries.json').read_text())
cases=[dict(name=c['name'],source=c['source'],category='prior-boundary') for c in old['cases'] if c['name']!='counter-update-prefix']
show='#show figure: it => [#metadata((kind: repr(it.kind), kind-type: repr(type(it.kind)))) <probe>]\n'
extras={
 'explicit-table':show+'#figure([A], kind: table)\n',
 'explicit-raw':show+'#figure(table([A]), kind: raw)\n',
 'explicit-image':show+'#figure(table([A]), kind: image)\n',
 'explicit-heading-custom-supplement':show+'#figure([A], kind: heading, supplement: [Heading])\n',
 'explicit-closure-invalid':show+'#figure([A], kind: x => x)\n',
 'explicit-integer-invalid':show+'#figure([A], kind: 1)\n',
 'custom-table-none-supplement':show+'#figure([A],kind:"table",supplement:none)\n',
 'custom-table-callback-supplement':show+'#figure([A],kind:"table",supplement: it => [Custom])\n',
 'element-table-callback-target': '#figure([#raw("x")#table([A])],kind:table,supplement: it => [#metadata(repr(it.func())) <probe>])\n',
 'name-table-callback-target': '#figure([#raw("x")#table([A])],kind:"table",supplement: it => [#metadata(repr(it.func())) <probe>])\n',
 'counter-update-prefix-fixed':'#counter(figure.where(kind: table)).update(7)\n#figure(table([A]),caption:[C]) <fig>\n#context [#metadata(counter(figure.where(kind: table)).at(<fig>)) <probe>]\n',
 'counter-mixed-typed-tracks':'#counter(figure.where(kind: table)).update(7)\n#counter(figure.where(kind: "table")).update(20)\n#figure(table([A]),caption:[A]) <element>\n#figure([B],kind:"table",supplement:[Custom],caption:[B]) <name>\n#context [#metadata((element:counter(figure.where(kind: table)).at(<element>),name:counter(figure.where(kind: "table")).at(<name>))) <probe>]\n',
 'counter-interleaved-prefix':'#counter(figure.where(kind: table)).update(7)\n#figure(table([A]),caption:[A]) <first>\n#counter(figure.where(kind: table)).update(n => n + 10)\n#figure(table([B]),caption:[B]) <second>\n#context [#metadata((first:counter(figure.where(kind:table)).at(<first>),second:counter(figure.where(kind:table)).at(<second>))) <probe>]\n',
 'callback-demand-after-last-figure':'#figure(table([A]),caption:[C])\n#counter(figure.where(kind:table)).update(n=>panic("FG1_AFTER"))\n',
 'callback-demand-by-counter-at-no-caption':'#counter(figure.where(kind:table)).update(n=>panic("FG1_AT"))\n#figure(table([A])) <fig>\n#context [#metadata(counter(figure.where(kind:table)).at(<fig>)) <probe>]\n',
 'callback-numbering-function':'#counter(figure.where(kind:table)).update(n=>panic("FG1_NUMBERING"))\n#figure(table([A]),caption:[C],numbering:n=>[X])\n',
 'callback-hidden-parent':'#counter(figure.where(kind:table)).update(n=>panic("FG1_PARENT"))\n#show block: none\n#block(figure(table([A]),caption:[C]))\n',
 'filtered-show-set-explicit-auto': '#show figure.where(kind:table):set figure(kind:raw)\n'+show+'#figure(table([A]),kind:auto)\n',
 'filtered-show-set-explicit-table':'#show figure.where(kind:table):set figure(kind:raw)\n'+show+'#figure(table([A]),kind:table)\n',
 'shared-figure-kind-style':'#let f = figure(table([A]))\n'+show+'#block({set figure(kind:raw); f})\n#block({set figure(kind:image); f})\n',
 'metadata-dict-array-opaque':show+'#figure(metadata((key:(table([A]),))))\n',
 'metadata-array-dict-opaque':show+'#figure(metadata(((key:table([A])),)))\n',
 'metadata-raw-before-table':show+'#figure([#metadata(raw("x"))#table([A])])\n',
}
cases += [dict(name=k,source=v,category='new-boundary') for k,v in extras.items()]
manifest={"utc_start":utc(),"head":git('rev-parse','HEAD').strip(),"branch":git('branch','--show-current').strip(),"diff_head_stat":git('diff','HEAD','--stat'),"status":git('status','--short'),"runner_sha256":sha(__file__),"product_build_receipt_sha256":sha(C/'accepted-build-receipt.json'),"product_build_head":receipt['head'],"product_build_state_note":receipt['state_note'],"product_inputs_verified":len(inputs),"product_inputs":inputs,"product_sha256":sha(product),"oracle_sha256":oracle_sha,"oracle_commit":old['oracle_commit'],"oracle_tree":old['oracle_tree'],"cases":[]}
(D/'probes').mkdir(exist_ok=True)
for case in cases:
 p=D/'probes'/(case['name']+'.typ');p.write_text(case['source']);row={**case,'source_sha256':sha(p),'runs':{}}
 for name,binary in [('oracle',oracle),('product',product)]:
  argv=[str(binary),'query',str(p),'<probe>','--field','value'];start=utc();run=subprocess.run(argv,cwd=R,text=True,capture_output=True,timeout=35)
  row['runs'][name]={'argv':argv,'utc_start':start,'utc_end':utc(),'returncode':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
 manifest['cases'].append(row)
 print(case['name'],[(k,v['returncode'],v['stdout'].strip()[:140],v['stderr'].splitlines()[0] if v['returncode'] else '') for k,v in row['runs'].items()],flush=True)
manifest['utc_end']=utc();manifest['binaries_unchanged']=sha(product)==manifest['product_sha256'] and sha(oracle)==oracle_sha
manifest['input_changes_after']=[p for p,h in inputs.items() if sha(R/p)!=h]
(D/'boundaries.json').write_text(json.dumps(manifest,indent=2)+'\n')
