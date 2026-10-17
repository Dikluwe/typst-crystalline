from pathlib import Path
import re,json
inv=json.loads(Path('00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json').read_text())
changed=[]
# Tokenization masks comments and literals but preserves byte character positions.
pattern=re.compile(r'//[^\n]*|/\*.*?\*/|"(?:\\.|[^"\\])*"|\b[rR](#+)?".*?"\1|\'(?:\\.|[^\'\\])\'',re.S)
for item in inv['files']:
 p=Path(item['path']);s=p.read_text();masked=pattern.sub(lambda m:' '*len(m.group()),s); edits=[]
 for m in re.finditer(r'FrameItem::(?:Text|TextSeparator|TextShaped)\s*\{',masked):
  start=m.end();depth=1;j=start
  while depth and j<len(masked):
   if masked[j]=='{':depth+=1
   elif masked[j]=='}':depth-=1
   j+=1
  body=masked[start:j-1]
  if re.search(r'\bbidi\b|\.\.',body):continue
  after=masked[j:].lstrip(); ispat=after.startswith(('=>','=',')','|','if '))
  # constructor fields always have a colon; bare patterns commonly do not.
  if not re.search(r'\w+\s*:',body):ispat=True
  tail=s[start:j-1].rstrip();addition=('' if tail.endswith(',') else ',')+(' .. ' if ispat else ' bidi: None, ')
  edits.append((j-1,addition))
 if edits:
  owner=Path(item['owner']); old=owner.read_text() # Read current owner before update.
  if p.name!='layout_types.rs':
   owner.write_text(old+'\n## Migração do carrier bidi ratificado (2026-10-05)\n\nMedição anterior à decisão: `00_nucleo/diagnosticos/bidi-contexto-2026-10-05/inventory.json` identifica as referências textuais desta unidade; os campos opcionais ratificados estão em `00_nucleo/materialization/bidi-contexto-l0-manifest-2026-10-05.json`. Mecânica interna, sem mudança de fase. Constructors sem origem lógica certificada usam `bidi: None`; patterns de leitura ignoram o campo com `..`. Transformações que mantêm a folha conservam o span; splits exatos conservam subspans. Não certificar por igualdade de texto nem fundir spans não contíguos. Testes e export mantêm os observáveis anteriores.\n')
  for pos,add in reversed(edits):s=s[:pos]+add+s[pos:]
  p.write_text(s);changed.append(str(p))
p=Path('01_core/src/entities/layout_types.rs');s=p.read_text();s=s.replace('Text { pos: Point, text: EcoString, style: TextStyle },','Text { pos: Point, text: EcoString, style: TextStyle, bidi: Option<crate::entities::bidi_context::BidiSpan> },')
s=s.replace('        origin: TextSeparatorOrigin,\n    },','        origin: TextSeparatorOrigin,\n        bidi: Option<crate::entities::bidi_context::BidiSpan>,\n    },',1)
s=s.replace('        units_per_em: u16,\n    },','        units_per_em: u16,\n        bidi: Option<crate::entities::bidi_context::BidiSpan>,\n    },',1);p.write_text(s)
Path('/tmp/tekt-bidi-materializacao/migration-files.json').write_text(json.dumps(changed,indent=2))
