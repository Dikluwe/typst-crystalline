import hashlib,json,subprocess,datetime,pathlib,gzip
root=pathlib.Path.cwd();out=root/'lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors'
bin=root/'lab/typst-original/target/release/typst';sha=hashlib.sha256(bin.read_bytes()).hexdigest();assert sha=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
receipt=root/'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md';r=receipt.read_text();assert 'bd5ebec285505889e8ff1d4f68804ab147a699f3' in r and 'cac836c07e4dec2a89219370428292393a3216b5' in r
expressions=[]
for f in ['table','table.cell','heading','figure','page','math.equation','par']:
 for method in ['or','and','within','before','after']:
  expressions += [f'selector(heading).{method}({f})',f'selector(heading).{method}(selector({f}))']
expressions += ['selector(heading).or("x")','selector(heading).or(regex("x"))','selector(heading).or(<x>)','selector(heading).or(1)','selector(heading).or(x => x)','selector(heading).or(table.with(columns: 1))','selector(page) == selector(page)','{let p = page; selector(p) == selector(page)}','selector(page) == page.where()','selector("equation")','selector("par")','selector("table")','selector("state")','selector("context_block")','{show selector(math.equation): it => [MATCH]; $x$}','{show selector(table): it => [MATCH]; table([x])}','{show selector(par): it => [MATCH]; par[x]}','{show selector(page): it => [MATCH]; page[x]}','{show selector(math.frac): it => [MATCH]; $1/2$}','{show selector(math.sqrt): it => [MATCH]; $sqrt(x)$}']
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'oracle_sha256':sha,'receipt_sha256':hashlib.sha256(receipt.read_bytes()).hexdigest(),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','results':[]}
for exp in expressions:
 argv=[str(bin),'eval',exp];p=subprocess.run(argv,text=True,capture_output=True);state['results'].append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
(out/'oracle-probes.json').write_text(json.dumps(state,indent=2)+'\n')
for x in state['results']:print(x['argv'][-1],'=>',x['stdout'].strip() or x['stderr'].splitlines()[0])
