from pathlib import Path
import json,statistics,subprocess,hashlib,xml.etree.ElementTree as ET
R=Path(__file__).resolve().parent
save=lambda name,x:(R/name).write_text(json.dumps(x,indent=2)+'\n')
rows=json.loads((R/'samples.json').read_text());assert len(rows)==45

def dist(round_values):
 vals=[x for values in round_values for x in values];mean=statistics.mean(vals);sd=statistics.stdev(vals);rm=[statistics.mean(v) for v in round_values]
 return {'n':len(vals),'samples':vals,'mean_ms':mean,'median_ms':statistics.median(vals),'min_ms':min(vals),'max_ms':max(vals),'stdev_ms':sd,'cv':sd/mean,'round_means_ms':rm,'stable':sd/mean<=.2 and all(abs(x/mean-1)<=.1 for x in rm)}
summary={};pairs={};phase={};states=[]
for case in ['latin','dense','placed-shape']:
 for policy in ['fresh','retained','prewarmed']:
  rr=sorted([x for x in rows if x['case']==case and x['policy']==policy],key=lambda x:x['round']);assert len(rr)==5
  summary[case+'/'+policy]={
   'session':dist([[x['result']['compiled_session_ms']] for x in rr]),
   'cold_cycle':dist([[x['result']['cycles'][0]['cycle_ms']] for x in rr]),
   'subsequent_cycles':dist([[c['cycle_ms'] for c in x['result']['cycles'][1:]] for x in rr]),
  }
  phase[case+'/'+policy]={}
  for part in ['setup_ms','pipeline_ms','teardown_ms','eval_ms','layout_ms','render_ms','first_book_ms']:
   phase[case+'/'+policy][part]={which:dist([[c[part] or 0 for c in x['result']['cycles'][slice(0,1) if which=='first' else slice(1,None)]] for x in rr]) for which in ['first','subsequent']}
  for x in rr:
   result=x['result'];assert len(result['cycles'])==6
   cs=result['cycles'];assert len({c['source_sha256'] for c in cs})==6
   assert all(c['book_calls']>0 for c in cs)
   assert len({c['catalog_sha256'] for c in cs})==1
   if policy!='fresh':assert len({c['book_pointer'] for c in cs})==1
   states.append({'round':x['round'],'case':case,'policy':policy,'source_edits_observed':True,'font_book_demand_preserved':True,'retained_pointer_constant':policy!='fresh','catalog_faces':cs[0]['catalog_faces'],'first_book_ms':[c['first_book_ms'] for c in cs]})
 for policy in ['retained','prewarmed']:
  pp=[]
  for round in range(5):
   fresh=next(x['result'] for x in rows if x['case']==case and x['policy']=='fresh' and x['round']==round)
   alt=next(x['result'] for x in rows if x['case']==case and x['policy']==policy and x['round']==round)
   a=fresh['compiled_session_ms'];b=alt['compiled_session_ms'];pp.append({'round':round,'fresh_ms':a,'alternative_ms':b,'reduction_ms':a-b,'reduction_percent':100*(1-b/a)})
  pairs[case+'/'+policy]={'samples':pp,'median_reduction_percent':statistics.median(x['reduction_percent'] for x in pp),'min_reduction_percent':min(x['reduction_percent'] for x in pp),'max_reduction_percent':max(x['reduction_percent'] for x in pp),'all_positive':all(x['reduction_ms']>0 for x in pp)}
save('summary.json',summary);save('paired-session-effects.json',pairs);save('phase-summary.json',phase);save('retention-controls.json',states)

# Independent output checks after all measurements. Bbox signatures contain every
# page, word, exact text and coordinates. Raster checks cover all control PDFs and
# measured round zero; later rounds receive full bbox/page/font/warning checks.
control_dir=R/'output-controls';control_dir.mkdir(exist_ok=False)
commands=[]
def command(argv):
 commands.append(argv);return subprocess.check_output(argv)
def inspect(p,raster=False):
 bbox=command(['pdftotext','-bbox',str(p),'-']).decode(); tree=ET.fromstring(bbox)
 pages=[]
 for e in tree.iter():
  if e.tag.rsplit('}',1)[-1]=='page':
   words=[{'text':w.text or '',**w.attrib} for w in e.iter() if w.tag.rsplit('}',1)[-1]=='word']
   pages.append({'size':e.attrib,'words':words})
 fonts=command(['pdffonts',str(p)]).decode().splitlines()[2:]
 fonts=sorted(' '.join(l.split()[:-2]) for l in fonts if l.strip())
 sig={'pages':pages,'fonts':fonts};fingerprint=hashlib.sha256(json.dumps(sig,sort_keys=True).encode()).hexdigest()
 pixels=[]
 if raster:
  label=str(p.relative_to(R)).replace('/','_').replace('.pdf','');dest=control_dir/label
  command(['pdftoppm','-r','36','-png',str(p),str(dest)])
  from PIL import Image
  for q in sorted(control_dir.glob(label+'-*.png')):
   with Image.open(q) as im: pixels.append({'size':im.size,'rgb_sha256':hashlib.sha256(im.convert('RGB').tobytes()).hexdigest()})
 return {'signature_sha256':fingerprint,'pages':len(pages),'fonts':fonts,'raster_36dpi':pixels,'first_page_words':[w['text'] for w in pages[0]['words']] if pages else []}
checks=[];passed=True
for case in ['latin','dense','placed-shape']:
 references=[]
 for revision in range(6):
  ref=inspect(R/'controls'/case/'direct'/f'edit{revision}.pdf',True);references.append(ref)
  for policy in ['fresh','retained','prewarmed']:
   q=R/'controls'/case/policy/f'edit{revision}.pdf';actual=inspect(q,True)
   good=actual['signature_sha256']==ref['signature_sha256'] and actual['raster_36dpi']==ref['raster_36dpi'];passed &=good
   checks.append({'pdf':str(q.relative_to(R)),'equivalent_to_direct':good,'observed':actual})
  for round in range(5):
   expected_faces=None;expected_warnings=None
   for policy in ['fresh','retained','prewarmed']:
    record=next(x for x in rows if x['round']==round and x['case']==case and x['policy']==policy)
    cycle=record['result']['cycles'][revision];faces=cycle['used_faces'];warnings=cycle['warning_count']
    if expected_faces is None:expected_faces=faces;expected_warnings=warnings
    else:assert faces==expected_faces and warnings==expected_warnings
    q=R/record['raw']/f'edit{revision}.pdf';actual=inspect(q,round==0)
    good=actual['signature_sha256']==ref['signature_sha256'] and (round!=0 or actual['raster_36dpi']==ref['raster_36dpi']);passed &=good
    checks.append({'pdf':str(q.relative_to(R)),'equivalent_to_direct':good,'raster_checked':round==0,'observed':actual})
 if case!='placed-shape':assert len({r['signature_sha256'] for r in references})==6,'rendered revisions must differ'
save('output-validation.json',{'all_passed':passed,'scope':'All pages/word geometry/text/fonts for every measured PDF; all-page raster at 36dpi for controls and measured round0 only. Same Tekt behavior, not vanilla parity.','checks':checks});save('output-control-commands.json',commands)
assert passed,'output discrepancy prevents effect acceptance'
print('stable cohorts',sum(v['stable'] for group in summary.values() for v in group.values()),'of',27)
for case in ['latin','dense','placed-shape']:
 for policy in ['fresh','retained','prewarmed']:
  x=summary[case+'/'+policy];print(case,policy,[(k,__import__('builtins').round(v['median_ms'],3),v['stable']) for k,v in x.items()])
print('Output validation passed')
