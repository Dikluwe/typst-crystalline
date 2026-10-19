#!/usr/bin/env python3
"""Measure colored exported ink bounds, never SVG/bitmap byte parity."""
import argparse,datetime,hashlib,json,pathlib,subprocess,sys
from PIL import Image
H=pathlib.Path(__file__).resolve().parent;R=pathlib.Path('/repos/Antigravity/typst-crystalline');RSVG=pathlib.Path('/home/linuxbrew/.linuxbrew/bin/rsvg-convert')
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
def bbox(p):
 im=Image.open(p).convert('RGBA'); points=[];red=blue=0
 for y in range(im.height):
  for x in range(im.width):
   r,g,b,a=im.getpixel((x,y))
   isred=r>150 and g<100 and b<100 and a>127
   isblue=b>120 and r<100 and g<160 and a>127
   if isred or isblue:points.append((x,y))
   red+=isred;blue+=isblue
 if not points:return {'bbox':None,'width_px':0,'height_px':0,'red_pixels':red,'blue_pixels':blue}
 xs,ys=zip(*points);box=[min(xs),min(ys),max(xs)+1,max(ys)+1]
 return {'bbox':box,'width_px':box[2]-box[0],'height_px':box[3]-box[1],'red_pixels':red,'blue_pixels':blue}
def main():
 p=argparse.ArgumentParser();p.add_argument('--baseline',action='store_true');p.add_argument('--output',required=True);p.add_argument('--product',required=True);p.add_argument('--build-receipt',required=True);p.add_argument('--validation-state');a=p.parse_args();out=R/a.output;assert not out.exists()
 mpath=H/'paint-cases.json';m=json.loads(mpath.read_text());binaries={'product':R/a.product,'oracle':R/'lab/typst-original/target/release/typst'};br=R/a.build_receipt
 assert sha(binaries['product']) in br.read_text();assert sha(binaries['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
 if a.validation_state:
  import gzip
  sp=R/a.validation_state;inputs=json.loads(gzip.decompress(sp.read_bytes()) if sp.suffix=='.gz' else sp.read_bytes())['inputs'];assert all(sha(R/k)==v for k,v in inputs.items())
 else:assert a.baseline;inputs={}
 out.mkdir(parents=True);rec={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'baseline':a.baseline,'baseline_source_note':'Stable baseline bytes are bound independently of current working-tree edits.' if a.baseline else None,'manifest_sha256':sha(mpath),'runner_sha256':sha(__file__),'build_receipt':{'path':a.build_receipt,'sha256':sha(br)},'rasterizer':{'path':str(RSVG),'sha256':sha(RSVG)},'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},'cases':[]}
 if a.validation_state:rec['validation_state']={'path':a.validation_state,'sha256':sha(sp)}
 for c in m['cases']:
  src=out/(c['name']+'.typ');src.write_text(c['source']);item={'name':c['name'],'source_sha256':sha(src),'utc':utc(),'results':{}}
  for label,binary in binaries.items():
   svg=out/(c['name']+'-'+label+'.svg');png=svg.with_suffix('.png');argv=[str(binary),'compile',str(src),str(svg)];run=subprocess.run(argv,cwd=R,text=True,capture_output=True,timeout=60);assert run.returncode==0,(argv,run.stderr)
   raster=[str(RSVG),'--dpi-x','288','--dpi-y','288','--output',str(png),str(svg)];rr=subprocess.run(raster,text=True,capture_output=True,timeout=30);assert rr.returncode==0,rr.stderr
   ink=bbox(png);passed=all(abs(ink[key]-c['expected_ink_diameter_px'])<=m['tolerance_px'] for key in ('width_px','height_px')) and ink['red_pixels']>0 and ink['blue_pixels']>0
   item['results'][label]={'compile':{'argv':argv,'returncode':run.returncode,'stdout':run.stdout,'stderr':run.stderr},'raster':{'argv':raster,'returncode':rr.returncode,'stdout':rr.stdout,'stderr':rr.stderr},'svg_sha256':sha(svg),'png_sha256':sha(png),'ink':ink,'expected_ink_diameter_px':c['expected_ink_diameter_px'],'passed':passed}
  item['passed']=item['results']['oracle']['passed'] and (a.baseline or item['results']['product']['passed']);rec['cases'].append(item);(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n')
 rec.update(utc_end=utc(),diff_head_stat_after=git('diff','HEAD','--stat'),changed_inputs=[k for k,v in inputs.items() if sha(R/k)!=v],binary_sha256_after={k:sha(v) for k,v in binaries.items()})
 rec['passed']=all(c['passed'] for c in rec['cases']) and not rec['changed_inputs'] and all(rec['binaries'][k]['sha256']==v for k,v in rec['binary_sha256_after'].items()) and rec['manifest_sha256']==sha(mpath) and rec['rasterizer']['sha256']==sha(RSVG) and rec['build_receipt']['sha256']==sha(br) and (not a.validation_state or sha(sp)==rec['validation_state']['sha256']);(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n');print(json.dumps({'passed':rec['passed'],'cases':len(rec['cases'])}));return 0 if rec['passed'] else 1
if __name__=='__main__':sys.exit(main())
