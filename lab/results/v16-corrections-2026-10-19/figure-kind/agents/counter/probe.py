import pathlib,subprocess,json,hashlib,datetime
R=pathlib.Path.cwd(); H=R/'lab/results/v16-corrections-2026-10-19/figure-kind/agents/counter'; O=R/'lab/typst-original/target/release/typst'; P=R/'lab/target/product/debug/typst'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert sha(O)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
end='\n#context [#metadata((typed: counter(figure.where(kind: image)).final(), all: counter(figure).final())) <probe>]\n'
cases={
 'uncaptioned-then-caption':'#figure([A])\n#figure([B], caption: [B])'+end,
 'uncaptioned-only-explicit-demand':'#figure([A])'+end,
 'disabled-then-caption':'#figure([A], numbering: none)\n#figure([B],caption:[B])'+end,
 'hidden-then-caption':'#show figure.where(kind: raw): none\n#figure([A],kind:raw)\n#figure([B],caption:[B])\n#context [#metadata((raw: counter(figure.where(kind:raw)).final(), image: counter(figure.where(kind:image)).final(), all:counter(figure).final())) <probe>]\n',
 'manual-interleave':'#figure([A])\n#counter(figure.where(kind:image)).update(7)\n#figure([B],caption:[B]) <b>\n#counter(figure.where(kind:image)).update(n=>n*2)\n#figure([C],caption:[C]) <c>\n#context [#metadata((b: counter(figure.where(kind:image)).at(<b>), c:counter(figure.where(kind:image)).at(<c>), final: counter(figure.where(kind:image)).final())) <probe>]\n',
 'uncaptioned-reference':'#figure([A]) <f>\n@f'+end,
}
record={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'runner_sha256':sha(pathlib.Path(__file__)),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','oracle_sha256':sha(O),'product_sha256':sha(P),'cases':[]}
for name,src in cases.items():
 f=H/(name+'.typ');f.write_text(src)
 row={'name':name,'source_sha256':sha(f),'runs':[]}
 for kind,b in [('oracle',O),('product',P)]:
  args=[str(b),'query',str(f),'<probe>','--field','value'];t=utc();p=subprocess.run(args,text=True,capture_output=True,timeout=25)
  row['runs'].append({'kind':kind,'argv':args,'utc_start':t,'utc_end':utc(),'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 record['cases'].append(row)
record['utc_end']=utc();(H/'counter-probes.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps([(x['name'],[(r['kind'],r['returncode'],r['stdout'],r['stderr'][:120]) for r in x['runs']]) for x in record['cases']],indent=2))
