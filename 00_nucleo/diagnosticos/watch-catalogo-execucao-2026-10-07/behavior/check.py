from pathlib import Path
import importlib.util,json,os,subprocess,time,zlib,struct
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('run',R/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r);b=r.b
r.OUT=D;r.WORK=Path('/tmp/watch-catalogo-2026-10-07/behavior');r.WORK.mkdir(parents=True,exist_ok=True)
b.BIN['previous']=Path('/tmp/watch-catalogo-2026-10-07/previous-96f22a1d')
# New image bytes and project font changes must become visible on the next source edit.
def behavior(engine):
    w=r.WORK/('resources-'+engine);w.mkdir(exist_ok=True);fonts=w/'fonts';fonts.mkdir(exist_ok=True)
    from fontTools.ttLib import TTFont
    unique=w/'unique-font.ttf';tt=TTFont(r.ROOT/'03_infra/fixtures/fonts/NotoSans-Regular.ttf')
    for rec in tt['name'].names:
        if rec.nameID in (1,4,6,16):
            value='WatchPriorityFourFamily-Regular' if rec.nameID==6 else 'Watch Priority Four Family'
            rec.string=value.encode(rec.getEncoding())
    tt.save(unique);tt.close()
    for old in fonts.glob('*'):old.unlink()
    font=fonts/'project.ttf';font.write_bytes(unique.read_bytes())
    img=w/'image.png'
    def png(color):
        def chunk(t,d):return struct.pack('>I',len(d))+t+d+struct.pack('>I',zlib.crc32(t+d)&0xffffffff)
        return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',20,20,8,2,0,0,0))+chunk(b'IDAT',zlib.compress((b'\0'+bytes(color)*20)*20))+chunk(b'IEND',b'')
    img.write_bytes(png((255,0,0)))
    main=w/'main.typ';pdf=w/'main.pdf';lp=w/'watch.log'
    src='#set text(font: "Watch Priority Four Family")\nResource Probe\n#image("image.png", width: 20pt)\n'
    main.write_text(src);pdf.unlink(missing_ok=True)
    cmd=[str(b.BIN[engine]),'watch',str(main),str(pdf),'--root',str(w),'--font-path',str(fonts),'--pdf-tagged=false','--pretty=false']
    if engine=='vanilla':cmd+=['--jobs','1']
    rows=[]
    def raster():
        dest=w/'raster';subprocess.run(['pdftoppm','-f','1','-singlefile','-scale-to','500',str(pdf),str(dest)],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);return b.sha(dest.with_suffix('.ppm'))
    def fontnames():return subprocess.check_output(['pdffonts',str(pdf)],text=True)
    with lp.open('w') as log:
        p=subprocess.Popen(cmd,cwd=r.ROOT,env=b.ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{b.CPU}))
        try:
            b.wait_pdf(p,pdf,None);time.sleep(.2);initial=raster();oldfonts=fontnames()
            prev=b.generation(pdf);img.write_bytes(png((0,0,255)));b.wait_pdf(p,pdf,prev);time.sleep(.2);new=raster();assert initial!=new;rows.append({'case':'image-byte-edit','raster_before':initial,'raster_after':new,'changed':True})
            # Font paths are not source/asset dependencies; source edit forces acquisition.
            prev=b.generation(pdf);font.write_bytes((r.ROOT/'03_infra/fixtures/fonts/NimbusSans-Regular.otf').read_bytes());time.sleep(.3)
            triggered=b.generation(pdf)!=prev;rows.append({'case':'font-edit-alone','publication_triggered':triggered})
            prev=b.generation(pdf);main.write_text(src+'// force cycle 1\n');b.wait_pdf(p,pdf,prev);time.sleep(.2);names=fontnames();assert names!=oldfonts if engine!='vanilla' else True;rows.append({'case':'replace-font-next-cycle','fonts_before':oldfonts,'fonts_after':names,'catalog_refreshed':names!=oldfonts})
            font.unlink();(fonts/'added.ttf').write_bytes(unique.read_bytes());prev=b.generation(pdf);main.write_text(src+'// force cycle 2\n');b.wait_pdf(p,pdf,prev);time.sleep(.2);names=fontnames();rows.append({'case':'delete-and-add-font-next-cycle','fonts':names,'custom_font_restored':'WatchPriorityFourFamily' in names});assert 'WatchPriorityFourFamily' in names if engine!='vanilla' else True
        finally:p.terminate();p.wait(timeout=5)
    return {'command':cmd,'initial_source':src,'unique_font_sha256':b.sha(unique),'fonttools_version':__import__('fontTools').__version__,'cases':rows,'log':lp.read_text()}
before=r.state('before');resources={};mutations={}
for e in ('previous','tekt','vanilla'):
    resources[e]=behavior(e);r.save('resources.json',resources)
    mutations[e]=r.mutation(e);r.save('mutations.json',mutations)
after=r.state('after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
print('Resources and recovery passed for prior/current/pinned vanilla.')
