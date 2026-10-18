from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,statistics,os
p=Path(__file__).resolve().parent;r=p.parents[2];out=p/'outputs/ram';out.mkdir(parents=True,exist_ok=True)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
bins={'before':p/'bin/typst-before','after':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
base='#set page(width:200pt,height:300pt,margin:10pt)\n#set text(font:"Libertinus Serif",size:10pt,hyphenate:false)\n'
body='A *B* C alpha beta gamma delta epsilon zeta eta theta.\n\n'*500
cases={'plain':base+body,'textual':base+'#show "A":[Z]\n'+body}
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='candidate-sources.json.gz',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},cases=[])
os.environ['CRYSTALLINE_PDF_FIXED_EPOCH']='1792281600'
for label,source in cases.items():
 f=out/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),runs=[])
 for repeat,actors in enumerate([list(bins),list(reversed(bins))]):
  for actor in actors:
   target=out/(label+'-'+actor+'-'+str(repeat)+'.pdf');timing=target.with_suffix('.time');cmd=['/usr/bin/time','-f','%M %e %U %S','-o',str(timing),str(bins[actor]),'compile',str(f),str(target)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=120);assert z.returncode==0,(label,actor,z.stderr);values=timing.read_text().strip().split();text=' '.join(subprocess.check_output(['pdftotext',str(target),'-'],text=True).split());run=dict(actor=actor,repeat=repeat,command=cmd,exit=z.returncode,stderr=z.stderr,max_rss_kib=int(values[0]),elapsed_seconds=float(values[1]),user_seconds=float(values[2]),system_seconds=float(values[3]),text_sha256=hashlib.sha256(text.encode()).hexdigest(),pdf_sha256=sha(target));row['runs'].append(run);print(label,actor,repeat,run['max_rss_kib'],run['elapsed_seconds'],flush=True)
 assert len({x['text_sha256'] for x in row['runs']})==1,'Do not compare memory without equal emitted text'
 row['summary']={actor:dict(median_rss_kib=statistics.median(x['max_rss_kib'] for x in row['runs'] if x['actor']==actor),median_elapsed_seconds=statistics.median(x['elapsed_seconds'] for x in row['runs'] if x['actor']==actor)) for actor in bins};record['cases'].append(row)
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(b)==record['binaries'][n]['sha256'] for n,b in bins.items());record['limitations']='Exploratory two repetitions per actor/fixture, alternating order; process max RSS, not whole-product certification or cold-cache benchmark. Same source/font settings and emitted text; no claim of universal memory improvement.'
(p/'ram-probe.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2).encode()))
