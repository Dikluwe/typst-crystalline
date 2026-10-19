from pathlib import Path
import re
log=Path('/tmp/tekt-bidi-materializacao/check.log').read_text()
changes={}
for m in re.finditer(r'error\[E0797\]:.*?--> ([^:]+):(\d+):',log,re.S):
 p=Path(m[1]);line=int(m[2])-1;changes.setdefault(p,set()).add(line)
for p,lines in changes.items():
 s=p.read_text().splitlines(True)
 for line in lines:
  # compiler points to whole expression; find introduced trailing .. before next constructor ends
  for i in range(line,min(line+35,len(s))):
   if re.search(r'\s\.\.\s*}',s[i]):s[i]=re.sub(r'\s\.\.\s*}', ' bidi: None, }',s[i]);break
 p.write_text(''.join(s))
