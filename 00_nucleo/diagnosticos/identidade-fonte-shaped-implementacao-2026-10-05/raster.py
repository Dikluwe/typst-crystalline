from pathlib import Path
import subprocess,hashlib,json,datetime
T=Path('/tmp/tekt-font-bound');H=Path('00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05');d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'dpi':600,'threshold':180,'precision_pt_per_pixel':.12,'renderer':subprocess.run(['pdftoppm','-v'],capture_output=True,text=True).stderr,'cases':{}}
for name in ['emph-mixed','arabic-emph-repeat','arabic-strong','latin-emph']:
 case={}
 for engine in ['after','vanilla']:
  pdf=T/(name+'-'+engine+'.pdf');prefix=T/(name+'-'+engine+'-raster');cmd=['pdftoppm','-f','1','-l','1','-singlefile','-r','600','-gray',str(pdf),str(prefix)];subprocess.run(cmd,check=True,capture_output=True)
  pgm=prefix.with_suffix('.pgm');raw=pgm.read_bytes();header=raw.split(b'\n',3);assert header[0]==b'P5' and header[2]==b'255';width,height=map(int,header[1].split());pixels=header[3];assert len(pixels)==width*height
  points=[(i%width,i//width) for i,p in enumerate(pixels) if p<180];x0=min(x for x,y in points);x1=max(x for x,y in points);y0=min(y for x,y in points);y1=max(y for x,y in points)
  case[engine]={'pdf_sha256':hashlib.sha256(pdf.read_bytes()).hexdigest(),'pgm_sha256':hashlib.sha256(raw).hexdigest(),'command':cmd,'ink_bbox_pixels':[x0,y0,x1,y1],'ink_bbox_pt':[x0*.12,y0*.12,(x1+1)*.12,(y1+1)*.12],'dark_pixels':len(points)}
  # Portable small PNG for visual inspection, no external Python package.
  crop=bytes(pixels[y*width+x] for y in range(y0-10,y1+11) for x in range(x0-10,x1+11));cw=x1-x0+21;ch=y1-y0+21
  import zlib,struct
  chunk=lambda tag,data:struct.pack('>I',len(data))+tag+data+struct.pack('>I',zlib.crc32(tag+data)&0xffffffff)
  png=b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',cw,ch,8,0,0,0,0))+chunk(b'IDAT',zlib.compress(b''.join(b'\0'+crop[y*cw:(y+1)*cw] for y in range(ch))))+chunk(b'IEND',b'');prefix.with_suffix('.png').write_bytes(png)
 d['cases'][name]=case;print(name,{k:v['ink_bbox_pt'] for k,v in case.items()},flush=True)
(H/'raster.json').write_text(json.dumps(d,indent=2)+'\n');(H/'raster.py').write_text(Path(__file__).read_text())
