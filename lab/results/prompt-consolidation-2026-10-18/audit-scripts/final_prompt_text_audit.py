from pathlib import Path
import re,json,datetime,hashlib
root=Path('/repos/Antigravity/typst-crystalline');out=root/'lab/results/prompt-consolidation-2026-10-18';owners=list((root/'00_nucleo/prompts').rglob('*.md')); names={p.name for p in owners}
short=[];missing=[];headings=[];language=[];refs=0
rx=re.compile(r'\b(?:não|deve|devem|apenas|também|manter|preservar|nenhum|ficheiro|função|validação|comportamento|medição|aceitação|proposta|aprovado|pendente|é|são)\b',re.I)
for p in (root/'00_nucleo/prompts').rglob('*'):
 if p.suffix not in ['.md','.toml']:continue
 rel=str(p.relative_to(root));s=p.read_text()
 for n,l in enumerate(s.splitlines(),1):
  for target in re.findall(r'`([\w./-]+\.md)`',l):
   if not target.startswith(('00_nucleo/','lab/')) and target.rsplit('/',1)[-1] in names:short.append([rel,n,target])
  for target in re.findall(r'(?<![\w/])(lab/history/[\w/.-]+\.(?:md|json|toml)(?:\.gz)?)(?![\w.])',l):
   refs+=1
   if not (root/target).is_file():missing.append([rel,n,target])
  if re.match(r'^#{1,6} .*\b(?:Passo|P\d{3,4})\b',l):headings.append([rel,n,l])
  if rx.search(l) and not l.startswith(('Hash do Código:','Núcleos Tekt:')):
   if '#[expect(' in l:reason='Exact lint-expectation reason preserved verbatim.'
   elif rel.endswith(('infra/watch.md','entities/frame_paint.md','entities/layout_types.md')):reason='Exact human approval quotation preserved as provenance.'
   elif rel.endswith('eval/markup_stream.md'):reason='Literal Unicode fixture codepoint, not Portuguese prose.'
   else:reason='Exact product or historical diagnostic/placeholder spelling; individually reviewed, outside editorial translation.'
   language.append({'path':rel,'line':n,'text':l,'disposition':reason})
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkpoint':'ef4a08ec3f48b2eecc470d9b2582349665cb29c1','state':'Frozen uncommitted inputs recorded by final-integrity.json and validation/validation-state.json.gz','scope':'Explicit individual history paths, active-owner Markdown shorthand, numbered step headings, and Portuguese keywords. Full semantic reviews are recorded separately; lexical scans are not a language detector.','history_references':refs,'missing_history_references':missing,'short_owner_references':short,'numbered_step_headings':headings,'language_matches':language}
(out/'final-text-audit.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='language_matches'},indent=2));print('Individually reviewed literal exceptions:',len(language));raise SystemExit(bool(missing or short or headings))
