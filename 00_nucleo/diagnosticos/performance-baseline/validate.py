"""Untimed output controls for accepted CLI blocks; never a performance test."""
import datetime, hashlib, json, re, subprocess
from pathlib import Path
OUT=Path(__file__).resolve().parent

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def call(args):return subprocess.check_output(args,text=True,stderr=subprocess.STDOUT)

def run():
    target=OUT/'validation';target.mkdir(exist_ok=True)
    rows=[]
    for samples in sorted((OUT/'e2e'/'plain').glob('r*/*/samples.json')):
        block=json.loads(samples.read_text())
        if not block['valid']:continue
        for engine in ['tekt','vanilla']:
            accepted=[s for s in block['samples'] if s['engine']==engine]
            first=accepted[0];pdf=OUT/first['output'];label=f"r{first['round']}-{samples.parent.name}-{engine}"
            commands=[['pdfinfo','-box',str(pdf)],['pdffonts',str(pdf)],['pdftotext','-layout',str(pdf),'-'],['pdfimages','-list',str(pdf)]]
            outputs=[]
            for n,cmd in enumerate(commands):
                value=call(cmd);(target/(label+f'-{n}.txt')).write_text(value);outputs.append(value)
            match=re.search(r'^Pages:\s+(\d+)',outputs[0],re.M);pages=int(match.group(1))
            boxes=call(['pdfinfo','-f','1','-l',str(pages),'-box',str(pdf)]);(target/(label+'-boxes.txt')).write_text(boxes)
            geometry=[x.strip() for x in boxes.splitlines() if re.match(r'Page\s+\d+\s+(size|MediaBox|CropBox):',x)]
            fonts=[]
            for value in outputs[1].splitlines()[2:]:
                if value.strip():fonts.append(re.sub(r'-Identity-H$','',re.sub(r'^[A-Z]{6}\+','',value.split()[0])))
            normalized=' '.join(outputs[2].split())
            rows.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'fixture':samples.parent.name,'round':first['round'],'engine':engine,'sample':first['label'],'pdf_sha256':sha(pdf),'pages':pages,'geometry':geometry,'font_names':sorted(fonts),'text_normalized':normalized,'text_normalized_sha256':hashlib.sha256(normalized.encode()).hexdigest(),'commands':commands,'all_sample_pdf_hashes':sorted(set(s['output_sha256'] for s in accepted)),'stderr_nonempty':[(s['label'],(OUT/s['output']).with_suffix('.stderr').read_text()) for s in accepted if (OUT/s['output']).with_suffix('.stderr').stat().st_size]})
    comparisons=[]
    for fixture,round_number in sorted(set((r['fixture'],r['round']) for r in rows)):
        pair={r['engine']:r for r in rows if r['fixture']==fixture and r['round']==round_number}
        a,b=pair['tekt'],pair['vanilla']
        comparisons.append({'fixture':fixture,'round':round_number,'pages_match':a['pages']==b['pages'],'geometry_match':a['geometry']==b['geometry'],'normalized_text_match':a['text_normalized']==b['text_normalized'],'font_resource_names_match':a['font_names']==b['font_names'],'text_font_control_match':(not a['text_normalized'] and not b['text_normalized']) or a['font_names']==b['font_names']})
    (OUT/'output-validation.json').write_text(json.dumps({'tools':{'pdfinfo':call(['pdfinfo','-v']),'pdffonts':call(['pdffonts','-v']),'pdftotext':call(['pdftotext','-v'])},'rows':rows,'comparisons':comparisons},indent=2)+'\n')
    print(json.dumps(comparisons,indent=2))
if __name__=='__main__':run()
