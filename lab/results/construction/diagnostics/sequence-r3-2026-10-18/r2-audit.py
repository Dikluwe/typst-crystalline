from pathlib import Path
import json,gzip,re
p=Path(__file__).resolve().parent
load=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
primary={
 'show-once':'variables from outside the function are read-only and cannot be modified',
 'stored-value':'panicked with: EARLY',
 'show-panic':'panicked with: X',
 'show-block-bad-mutation':'variables from outside the function are read-only and cannot be modified',
 'other-parent-hides-sequence':'panicked with: HIDDEN',
}
open_text={'symbol-identity': ('text', 'symbol'), 'symbol-equality': ('true', 'false')}
rows=[];diagnostics=[]
for c in load('after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')];label=c['label']
 assert a['exit']==b['exit'],c
 if label in primary:
  assert a['stderr'].splitlines()[0]=='error: '+primary[label],c
  assert b['stderr'].splitlines()[0]=='error: maximum show rule depth exceeded',c
  assert '.typ:' in a['stderr'],c
  status='primary-error-preserved'
 elif label in open_text:
  assert (a['text'],b['text'])==open_text[label],c
  status='open-morphology'
 else:
  assert a['text']==b['text'],c
  if a['exit']:
   assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],c
   # Primary message/span/hints are acceptance; generic show trace remains audited separately.
   assert a['stderr'].split('\n  while ')[0].rstrip()==b['stderr'].split('\n  while ')[0].rstrip(),c
   if a['stderr']!=b['stderr']:diagnostics.append({'label':label,'tekt':a['stderr'],'vanilla':b['stderr'],'classification':'open-generic-show-trace'})
  status='accepted'
 rows.append({'label':label,'status':status})
text_rows=[]
for c in load('text-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit']==0,c
 if c['label']=='local-text-scope':
  assert (a['text'],b['text'])==('C','B'),c
  status='open-literal-precedence'
 else:
  assert a['text']==b['text'],c
  status='accepted'
 text_rows.append({'label':c['label'],'status':status})
target_rows=[]
html_scaffolding={'show':('Y','Y Y'),'hidden-child':('Y','Y Y'),'local-rule':('LOCAL','LOCAL LOCAL'),'sequence-hidden-table':('Y','Y Y')}
for c in load('target-controls.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit'],c
 status='accepted-text-and-primary-error'
 if c['target']=='html' and c['label'] in html_scaffolding:
  assert a['exit']==b['exit']==0,c
  assert (a['text'],b['text'])==html_scaffolding[c['label']],c
  status='internal-empty-footnote-leak-not-copied'
 elif not a['exit']:assert a['text']==b['text'],c
 else:assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],c
 target_rows.append({'label':c['label'],'target':c['target'],'status':status})
slots=[]
for c in load('empty-slots-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit']==0 and a['text']==b['text'],c
 slots.append(c['label'])
emission=[]
for c in load('emission-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit'],c
 assert a['text']==b['text'],c
 if a['exit']:assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],c
 status='accepted'
 emission.append({'label':c['label'],'status':status})
footnotes=[]
for c in load('footnotes-controls.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit'],c
 status='accepted'
 if c['label']=='empty-footnotes' and c['target']=='html':
  assert (a['text'],b['text'])==('BODY','BODY FOOTER'),c
  status='internal-empty-footnote-leak-not-copied'
 elif not a['exit']:assert a['text']==b['text'],c
 else:assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],c
 footnotes.append({'label':c['label'],'target':c['target'],'status':status})
eof=[]
for c in load('eof-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit']==0 and a['text']==b['text'],c
 eof.append(c['label'])
logical=[]
for c in load('logical-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit'],c
 if not a['exit']:
  assert a['text']==b['text']=='Y',c
  status='accepted'
 else:
  assert a['stderr'].splitlines()[0]=='error: selector não suportado em show rule',c
  assert b['stderr'].splitlines()[0]=='error: this selector cannot be used with show',c
  status='open-composite-selector-diagnostic'
 logical.append({'label':c['label'],'status':status})
original=[]
for c in load('original-after.json.gz')['cases']:
 a,b=[c['results'][k] for k in ('tekt','vanilla')]
 assert a['exit']==b['exit']==0,c
 if c['label']=='styled-children':
  assert (a['text'],b['text'])==('([A B], [ ])','(styled(child: [A B], ..), [ ])'),c
  status='open-styled-repr'
 else:
  assert a['text']==b['text'],c
  status='accepted'
 original.append({'label':c['label'],'status':status})
result={'pdf':rows,'text':text_rows,'targets':target_rows,'slots':slots,'logical':logical,'original':original,'eof':eof,'emission':emission,'footnotes':footnotes,'diagnostic_details':diagnostics,'method':'Compare language values via explicit repr/assert probes and normalized extracted text; compare status and primary error/span/hints. This is not proof of equal layout geometry, PDF bytes, color or Symbol morphology. Every known divergence remains individually classified.'}
(p/'r2-audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2).encode()))
print(json.dumps({'pdf_cases':len(rows),'pdf_accepted':sum(x['status']=='accepted' for x in rows),'primary_error_preserved':sum(x['status']=='primary-error-preserved' for x in rows),'open_morphology':sum(x['status']=='open-morphology' for x in rows),'text_cases':len(text_rows),'target_cases':len(target_rows),'slot_cases':len(slots),'emission_cases':len(emission),'footnotes_controls':len(footnotes),'open_diagnostic_details':[x['label'] for x in diagnostics]}))
