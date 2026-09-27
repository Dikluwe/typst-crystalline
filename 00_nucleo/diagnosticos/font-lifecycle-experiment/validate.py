#!/usr/bin/env python3
"""Untimed structural/text/font controls, distinct from timing admission."""
import datetime,hashlib,json,re,subprocess
from pathlib import Path
OUT=Path(__file__).resolve().parent
def call(args):return subprocess.check_output(args,text=True,stderr=subprocess.STDOUT)
rows=[];comparisons=[];target=OUT/'validation';target.mkdir(exist_ok=True)
for p in sorted((OUT/'samples').glob('plain-*/samples.json')):
    receipt=json.loads((OUT/'receipts'/(p.parent.name+'.json')).read_text());assert receipt['valid']
    samples=json.loads(p.read_text())['samples'];by={}
    for name in ['before','lazy','interval','vanilla']:
        accepted=[r for r in samples if r['variant']==name];sample=accepted[0];pdf=OUT/sample['output'];label=f"r{sample['round']}-{sample['fixture']}-{name}"
        commands=[['pdfinfo','-box',str(pdf)],['pdffonts',str(pdf)],['pdftotext','-layout',str(pdf),'-'],['pdfimages','-list',str(pdf)]]
        outputs=[]
        for n,cmd in enumerate(commands):
            text=call(cmd);(target/(label+f'-{n}.txt')).write_text(text);outputs.append(text)
        pages=int(re.search(r'^Pages:\s+(\d+)',outputs[0],re.M).group(1))
        boxes=call(['pdfinfo','-f','1','-l',str(pages),'-box',str(pdf)]);(target/(label+'-boxes.txt')).write_text(boxes)
        geometry=[x.strip() for x in boxes.splitlines() if re.match(r'Page\s+\d+\s+(size|MediaBox|CropBox):',x)]
        fonts=[]
        for value in outputs[1].splitlines()[2:]:
            if value.strip():fonts.append(re.sub(r'-Identity-H$','',re.sub(r'^[A-Z]{6}\+','',value.split()[0])))
        normalized=' '.join(outputs[2].split())
        row={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'variant':name,'fixture':sample['fixture'],'round':sample['round'],'sample':sample['label'],'pages':pages,'geometry':geometry,'font_names':sorted(fonts),'text_normalized':normalized,'text_sha256':hashlib.sha256(normalized.encode()).hexdigest(),'commands':commands,'pdf_sha256':sample['output_sha256'],'all_sample_pdf_hashes':sorted({r['output_sha256'] for r in accepted}),'stderr_nonempty':[(r['label'],(OUT/r['output']).with_suffix('.stderr').read_text()) for r in accepted if (OUT/r['output']).with_suffix('.stderr').stat().st_size]}
        rows.append(row);by[name]=row
    for a,b in [('before','lazy'),('lazy','interval'),('interval','vanilla')]:
        x,y=by[a],by[b]
        comparisons.append({'fixture':sample['fixture'],'round':sample['round'],'comparison':a+'-'+b,'pages_match':x['pages']==y['pages'],'geometry_match':x['geometry']==y['geometry'],'normalized_text_match':x['text_normalized']==y['text_normalized'],'font_resource_names_match':x['font_names']==y['font_names'],'text_font_control_match':(not x['text_normalized'] and not y['text_normalized']) or x['font_names']==y['font_names'],'exact_pdf_match_auxiliary_only':x['pdf_sha256']==y['pdf_sha256']})
(OUT/'output-validation.json').write_text(json.dumps({'tools':{name:call([name,'-v']) for name in ['pdfinfo','pdffonts','pdftotext']},'rows':rows,'comparisons':comparisons},indent=2)+'\n')
assert all(all(row[k] for k in ['pages_match','geometry_match','normalized_text_match','text_font_control_match']) for row in comparisons), 'Output control failed'
print('Output controls passed for all rounds, fixtures and variants')
