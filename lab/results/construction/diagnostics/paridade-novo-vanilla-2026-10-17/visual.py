from pathlib import Path
import gzip,json,subprocess,hashlib,collections,datetime,shutil
from PIL import Image
out=Path(__file__).resolve().parent;d=json.load(gzip.open(out/'measurement.json.gz'));rows=[]
def run(cmd):
 r=subprocess.run(cmd,capture_output=True,text=True,timeout=45);return {'command':cmd,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
for c in d['cases']:
 if c['group'] not in ['tiling','table-svg']:continue
 for name,r in c['results'].items():
  if r['exit']!=0:continue
  args=r['command'];src=Path(args[3]);fmt=c['fmt'];dest=src.with_suffix('.observed.png');row={'group':c['group'],'label':c['label'],'binary':name,'source':str(src),'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
  if fmt=='pdf':row['conversion']=run(['pdftoppm','-r','144','-png','-singlefile',str(src),str(dest.with_suffix(''))])
  elif fmt=='svg':row['conversion']=run([shutil.which('rsvg-convert'),'--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(dest),str(src)])
  else:dest=src
  if row.get('conversion',{}).get('exit',0)!=0:rows.append(row);continue
  im=Image.open(dest).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);im=bg.convert('RGB');row['size']=list(im.size);row['raster_sha256']=hashlib.sha256(im.tobytes()).hexdigest()
  if c['group']=='tiling':
   roi=im.crop((40,40,120,100));pixels=list(roi.getdata());count=collections.Counter(pixels);row['roi']=[40,40,120,100];row['center']=im.getpixel((60,60));row['common_colors']=count.most_common(6);row['scan_y60_colors']=len(set(im.getpixel((x,60)) for x in range(42,118)))
   if c['label'].startswith('overlap-alpha'):expected=[255,15.9375,15.9375]
   elif c['label'].startswith('order'):expected=[159.375,63.75,79.6875]
   elif c['label'].startswith('subpixel'):expected=[255,191.25,191.25]
   else:expected=None
   if expected:
    row['analytic']=expected;errors=[max(abs(v-e) for v,e in zip(p,expected)) for p in pixels];row['max_analytic_error']=max(errors);row['pixels_over_2']=sum(e>2 for e in errors);row['roi_pixels']=len(pixels)
  else:
   # Bounds of solid red/blue ink are meaningful here; no pixel-identity gate.
   for color,pred in [('red',lambda p:p[0]>180 and p[1]<90 and p[2]<90),('blue',lambda p:p[2]>180 and p[0]<90 and p[1]<160)]:
    coords=[(x,y) for y in range(im.height) for x in range(im.width) if pred(im.getpixel((x,y)))]
    row[color]={'pixels':len(coords),'bounds':[min(x for x,y in coords),min(y for x,y in coords),max(x for x,y in coords),max(y for x,y in coords)] if coords else None}
  rows.append(row)
with gzip.open(out/'visual.json.gz','wt') as f:json.dump({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'rows':rows},f)
for r in rows:
 print(r['label'],r['binary'],r.get('center'),r.get('scan_y60_colors'),r.get('pixels_over_2'),r.get('red'),r.get('blue'),r.get('conversion',{}).get('exit',0),flush=True)
