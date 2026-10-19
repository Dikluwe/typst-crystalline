from pathlib import Path
import json,re,hashlib,datetime,subprocess
P=Path('00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28')
def load(f):return json.loads((P/f).read_text())
before={r['case']:r for r in json.loads(Path('00_nucleo/diagnosticos/correcao-politica-hifenizacao-2026-09-28/before-pdf/resultados.json').read_text())};after={r['case']:r for r in load('regression-policy/resultados.json')};lex={r['case']:r for r in load('regression-lexical/resultados.json')};checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
def obs(run):return {k:run.get(k) for k in ['exit_code','pages','page_count','line_count','text']}
def words(run):return [w['text'] for p in run.get('pages',[]) for line in p['lines'] for w in line['words']]
def has_hyphen(run):return any(w.endswith(('-', '\u00ad')) for w in words(run))
def visible_lines(run):return [[line['text'].replace('\u00ad','-') for line in p['lines']] for p in run.get('pages',[])]
for k,a in after.items():
 check(k+': source unchanged',a['source_sha256']==before[k]['source_sha256'])
 check(k+': oracle observables unchanged',obs(a['runs']['vanilla'])==obs(before[k]['runs']['vanilla']))
for justify in [False,True]:
 for mode in ['absent','auto','false','true']:
  k=f'hyphen-{mode}-justify-{justify}';t=after[k]['runs']['tekt'];v=after[k]['runs']['vanilla'];expected=mode=='true' or (mode in ['absent','auto'] and justify)
  check(k+': option controls visible hyphenation',t['exit_code']==v['exit_code']==0 and has_hyphen(t)==has_hyphen(v)==expected)
  if not justify and mode!='true':check(k+': simple unhyphenated lines agree',visible_lines(t)==visible_lines(v))
for k,a in after.items():
 if k.startswith('invalid-') or k=='native-par-justify-auto':
  t=a['runs']['tekt'];v=a['runs']['vanilla']
  check(k+': compile error',t['exit_code']==v['exit_code']==1)
  check(k+': exact primary error',t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0])
  check(k+': causal line/column',re.findall(r'\.typ:(\d+):(\d+)',t['stderr'])==re.findall(r'\.typ:(\d+):(\d+)',v['stderr']))
for mode in ['false','auto','true']:
 a=after['native-hyphen-'+mode];check('native text '+mode+': option consumed',has_hyphen(a['runs']['tekt'])==(mode=='true'))
for mode in ['false','true']:
 a=after['native-par-justify-'+mode];check('native par '+mode+': auto policy consumed',has_hyphen(a['runs']['tekt'])==(mode=='true'))
for k,a in lex.items():
 t=a['runs']['tekt'];v=a['runs']['vanilla'];check(k+': compiles',t['exit_code']==v['exit_code']==0)
 if k=='show-par-sets-false':check(k+': selected false reaches layout',not has_hyphen(t) and not has_hyphen(v))
 else:check(k+': lexical override and restoration',words(t)==[w.replace('\u00ad','-') for w in words(v)])
 if k!='par-false-restores-true':check(k+': visible line breaks agree',visible_lines(t)==visible_lines(v))
# Preserve the pagination corrections; no full footnote/float parity assertion.
for k,pages in [('terminal',1),('terminal-continuation',2),('terminal-pagebreak',2),('terminal-pagebreak-content',2),('terminal-two-pagebreaks',3),('terminal-final-parbreak',1),('empty',1),('empty-explicit-pagebreak',2),('terminal-odd-break',3),('terminal-forced-line',1),('terminal-columns',1),('terminal-footnote',2),('terminal-bottom-float',2)]:
 t=after[k]['runs']['tekt'];v=after[k]['runs']['vanilla'];check(k+': correct pages',t.get('page_count')==v.get('page_count')==pages)
check('columns: complete lines now agree',visible_lines(after['terminal-columns']['runs']['tekt'])==visible_lines(after['terminal-columns']['runs']['vanilla']))
for k in ['tracking--0.333','tracking-0','tracking-0.333','tracking-3','terminal-footnote','terminal-bottom-float']:
 check(k+': unrelated geometry stable',obs(after[k]['runs']['tekt'])==obs(before[k]['runs']['tekt']))
# Existing unsupported string show-set is an explicit open item, not a new regression.
check('string show-set limitation remains unchanged',after['showset-hyphen-override']['runs']['tekt']['exit_code']==before['showset-hyphen-override']['runs']['tekt']['exit_code']==1)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();binary=sha('target/release/typst')
for folder in ['regression-policy','regression-lexical']:
 check(folder+': exact release identity',load(folder+'/proveniencia.json')['binary_sha256']['tekt']==binary)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':True,'bilateral_inputs':len(after)+len(lex),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'scope_sha256':{f:sha(f) for pair in load('scope.json').items() for f in pair}}
(P/'regression-acceptance.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'bilateral_inputs':result['bilateral_inputs']}));raise SystemExit(bool(result['failed']))
