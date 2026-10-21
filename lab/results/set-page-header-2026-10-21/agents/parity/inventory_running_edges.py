from pathlib import Path
import re,json,hashlib,datetime,subprocess
ROOT=Path.cwd();D=ROOT/'lab/results/set-page-header-2026-10-21/agents/parity'
def mask_literals(s):
 out=list(s);i=0
 def blank(a,b):
  for k in range(a,b):
   if out[k]!='\n':out[k]=' '
 while i<len(s):
  if s.startswith('//',i):
   j=s.find('\n',i);j=len(s) if j<0 else j;blank(i,j);i=j
  elif s.startswith('/*',i):
   j=i+2;depth=1
   while j<len(s) and depth:
    if s.startswith('/*',j):depth+=1;j+=2
    elif s.startswith('*/',j):depth-=1;j+=2
    else:j+=1
   blank(i,j);i=j
  elif re.match(r'(?:br|r)#+"|(?:br|r)"',s[i:]):
   m=re.match(r'(?:br|r)(#*)"',s[i:]);end='"'+m[1];j=s.find(end,i+m.end());j=len(s) if j<0 else j+len(end);blank(i,j);i=j
  elif s[i]=='"':
   j=i+1
   while j<len(s):
    if s[j]=='\\':j+=2
    elif s[j]=='"':j+=1;break
    else:j+=1
   blank(i,j);i=j
  elif s[i]=="'" and (m:=re.match(r"'(?:\\(?:u\{[^}]*\}|x[0-9a-fA-F]{2}|.)|[^'\n\\])'",s[i:])):
   j=i+m.end();blank(i,j);i=j
  else:i+=1
 return ''.join(out)
def cfg_test_ranges(mask):
 ranges=[]
 for m in re.finditer(r'#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]',mask):
  j=m.end()
  # Skip attributes attached to that test item.
  while True:
   a=re.match(r'\s*#\s*\[',mask[j:])
   if not a:break
   j+=a.end();depth=1
   while depth and j<len(mask):
    if mask[j]=='[':depth+=1
    if mask[j]==']':depth-=1
    j+=1
  b=re.search('[{;]',mask[j:])
  if not b:continue
  k=j+b.start()
  if mask[k]==';':ranges.append((m.start(),k+1));continue
  depth=1;k+=1
  while k<len(mask) and depth:
   if mask[k]=='{':depth+=1
   if mask[k]=='}':depth-=1
   k+=1
  ranges.append((m.start(),k))
 return ranges
rows=[]
patterns=r'\.\s*(layout_content(?:_body|_once)?|layout_sub_frame\w*)\s*\('
for p in sorted((ROOT/'01_core/src/compiler/layout').rglob('*.rs')):
 rel=str(p.relative_to(ROOT))
 if p.name=='tests.rs' or '/tests/' in rel or p.name.endswith('_tests.rs'):continue
 s=p.read_text();masked=mask_literals(s);test=cfg_test_ranges(masked);hits=[]
 for m in re.finditer(patterns,masked):
  if any(a<=m.start()<b for a,b in test):continue
  line=s.count('\n',0,m.start())+1
  hits.append({'line':line,'method':m[1],'source_line':s.splitlines()[line-1].strip()})
 if hits:
  owner='00_nucleo/prompts/core/compiler/layout/'+str(p.relative_to(ROOT/'01_core/src/compiler/layout')).removesuffix('.rs')+'.md'
  rows.append({'source':rel,'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'owner':owner,'owner_sha256':hashlib.sha256((ROOT/owner).read_bytes()).hexdigest(),'sites':hits})
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'method':'Lexically mask comments/string/char/raw literals; exclude cfg(test) item ranges and known test source units; collect direct Content/subframe dispatch calls. Includes synth/measurement calls deliberately; classification requires source audit. Not an exhaustive inventory of free-function delegation or indirect cloned/deferred Content transport.','owner_count':len(rows),'call_site_count':sum(len(x['sites']) for x in rows),'owners':rows}
(D/'descendant-call-inventory.json').write_text(json.dumps(record,indent=2)+'\n')
print(record['owner_count'],record['call_site_count'])
for x in rows:print(x['source'],','.join(str(z['line']) for z in x['sites']))
