"""Compare language-visible text/geometry/fonts after the timed run."""
from pathlib import Path
import argparse,csv,subprocess as sp,xml.etree.ElementTree as ET,re
p=argparse.ArgumentParser();p.add_argument('directory',type=Path);p.add_argument('--reference',default='bas');a=p.parse_args()
rows=list(csv.DictReader((a.directory/'measurements.csv').open()));groups={};count=0
for row in rows:
 key=tuple(row[x] for x in ['round','repeat','fixture','mode'])
 groups.setdefault(key,{})[row['variant']]=a.directory/row['case']/(row['mode']+'.pdf')
def signature(pdf):
 xml=sp.check_output(['pdftotext','-bbox',str(pdf),'-'])
 pages=[ET.tostring(el) for el in ET.fromstring(xml).iter() if el.tag.rsplit('}',1)[-1]=='page']
 fonts=[]
 for line in sp.check_output(['pdffonts',str(pdf)],text=True).splitlines()[2:]:
  fields=re.split(r'\s{2,}',line.strip());fields[0]=re.sub(r'^[A-Z]{6}\+','',fields[0]);fonts.append(fields[:6])
 return pages,fonts
for key,variants in groups.items():
 assert a.reference in variants,key
 expected=signature(variants[a.reference])
 marker=b'TEKT_EPOCH_'+(b'1' if key[3]=='main_edit' else b'0')
 assert marker in b''.join(expected[0]),('missing epoch',key)
 for name,pdf in variants.items():
  if name==a.reference:continue
  assert signature(pdf)==expected,(key,name)
  count+=1
print('Compared',count,'PDF pairs from',len(rows),'publications')
