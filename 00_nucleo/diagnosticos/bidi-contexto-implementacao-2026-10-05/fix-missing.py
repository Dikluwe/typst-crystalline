from pathlib import Path
import re
log=Path('/tmp/tekt-bidi-materializacao/context-red.log').read_text()
for m in re.finditer(r'error\[E0063\].*?--> ([^:]+):(\d+):',log,re.S):
 p=Path(m[1]);s=p.read_text();a=sum(len(x) for x in s.splitlines(True)[:int(m[2])-1]);b=s.index('{',s.index('FrameItem::',a));depth=1;j=b+1
 # trim raw strings/comments not present in these four literal declarations
 while depth:
  if s[j]=='{':depth+=1
  elif s[j]=='}':depth-=1
  j+=1
 owner=re.search(r'@prompt ([^\n]+)',s)[1];q=Path(owner);old=q.read_text()
 if 'Migração do carrier bidi' not in old:q.write_text(old+'\n## Migração do carrier bidi\n\nMedição anterior à decisão: compiler RED (`context-red.log`, receipt da implementação 2026-10-05) identifica constructor de teste sem campo ratificado. Acrescentar bidi None para fixture sem origem certificada; aceitação mantém as asserções de linguagem existentes.\n')
 s=s[:j-1]+' bidi: None, '+s[j-1:];p.write_text(s)
