#!/usr/bin/env python3
"""Read-only product probing; generated evidence stays in this diagnostic directory."""
import argparse, datetime, hashlib, json, os, pathlib, subprocess, sys, xml.etree.ElementTree as ET
ROOT = pathlib.Path(__file__).resolve().parents[3]
BASE = pathlib.Path(__file__).resolve().parent
VANILLA = ROOT / "lab/typst-original/target/release/typst"
CRYSTAL = ROOT / "target/debug/typst"
PIN = "9f2b6e8715237cb086899a42873660fe744622e8"
ORACLE_SHA = "7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea"
ENV = dict(os.environ, LC_ALL="C", TZ="UTC", NO_COLOR="1", CARGO_BUILD_JOBS="1")
ENV.pop("TYPST_ROOT", None)
ENV.pop("TYPST_FONT_PATHS", None)
ENV.pop("CRYSTALLINE_DOCUMENT_ID", None)
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path): return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def call(args, timeout=90):
    start=now()
    try:
        p=subprocess.run([str(a) for a in args], cwd=ROOT, env=ENV, capture_output=True, timeout=timeout)
        return dict(command=[str(a) for a in args], started=start, finished=now(),
                    exit=p.returncode, stdout=p.stdout.decode("utf-8","replace"), stderr=p.stderr.decode("utf-8","replace"))
    except subprocess.TimeoutExpired as e:
        return dict(command=[str(a) for a in args], started=start, finished=now(), exit=None, timeout=True,
                    stdout=(e.stdout or b"").decode("utf-8","replace"),stderr=(e.stderr or b"").decode("utf-8","replace"))
def save(path,data):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+"\n")
def state():
    paths=subprocess.check_output(["git","ls-files","--cached","--others","--exclude-standard","-z",
        "01_core","02_shell","03_infra","04_wiring","00_nucleo/prompts","00_nucleo/adr",
        "Cargo.toml","Cargo.lock","CLAUDE.md","AGENTS.md"],cwd=ROOT).decode().split("\0")
    fingerprints={}
    for name in sorted(set(paths)):
        p=ROOT/name
        if not name or name.endswith((".orig",".rej")) or not p.is_file(): continue
        fingerprints[name]=sha(p)
    identity=hashlib.sha256(json.dumps(fingerprints,sort_keys=True).encode()).hexdigest()
    receipt=dict(captured=now(),head=subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip(),
        status=subprocess.check_output(["git","status","--porcelain=v1","--untracked-files=all"],cwd=ROOT,text=True),
        productive_documentary_sha256=fingerprints,aggregate_sha256=identity,
        environment={k:ENV.get(k) for k in ["LC_ALL","TZ","NO_COLOR","CARGO_BUILD_JOBS","TYPST_ROOT","TYPST_FONT_PATHS","CRYSTALLINE_DOCUMENT_ID"]})
    receipt_id=hashlib.sha256(json.dumps(receipt,sort_keys=True).encode()).hexdigest()
    out=BASE/"receipts"/("state-"+receipt_id[:24]+".json");save(out,receipt)
    patch=subprocess.check_output(["git","diff","--binary","HEAD","--",
        "01_core","02_shell","03_infra","04_wiring","00_nucleo/prompts","00_nucleo/adr",
        "Cargo.toml","Cargo.lock","CLAUDE.md"],cwd=ROOT)
    patch_path=out.with_suffix(".patch");patch_path.write_bytes(patch)
    return {"path":str(out.relative_to(ROOT)), "sha256":sha(out),"source_identity":identity,"patch_sha256":sha(patch_path)}
def binaries():
    if sha(VANILLA)!=ORACLE_SHA: raise SystemExit("REFUSED: oracle bytes do not match the ratified pin receipt")
    return {"vanilla":{"path":str(VANILLA),"sha256":sha(VANILLA),"pin":PIN,
        "tree":call(["git","rev-parse",PIN+"^{tree}"]),"version":call([VANILLA,"--version"]),
        "recipe":"00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md"},
        "crystalline":{"path":str(CRYSTAL),"sha256":sha(CRYSTAL),"version":call([CRYSTAL,"--version"])}}
def build():
    before=state()
    r=call(["cargo","build","--locked","--bin","typst"],timeout=900)
    after=state()
    if before["source_identity"]!=after["source_identity"]: raise SystemExit("Source changed during build")
    record=dict(before=before,after=after,build=r,binaries=binaries())
    save(BASE/"receipts"/"build.json",record)
    print(json.dumps({"build_exit":r["exit"],"source_identity":after["source_identity"],"crystalline_sha256":record["binaries"]["crystalline"]["sha256"]}))
    if r["exit"]!=0: raise SystemExit("Build failed; inspect receipt")
def extract(pdf,out):
    r=call(["pdftotext","-bbox",pdf,"-"])
    save(out.with_suffix(".bbox-command.json"),r)
    if r["exit"]!=0: return {"error":"bbox extraction failed"}
    out.with_suffix(".bbox.xhtml").write_text(r["stdout"])
    root=ET.fromstring(r["stdout"]); pages=[]
    for page in root.iter():
        if page.tag.split("}")[-1]!="page": continue
        words=[]
        for word in page.iter():
            if word.tag.split("}")[-1]=="word":
                words.append(dict(text="".join(word.itertext()),**{k:float(v) for k,v in word.attrib.items()}))
        pages.append(dict(width=float(page.attrib["width"]),height=float(page.attrib["height"]),words=words))
    return {"pages":pages}
def raster(pdf,out):
    from PIL import Image
    png=out.with_suffix(".png")
    r=call(["mutool","draw","-q","-r","72","-o",png,pdf,"1"])
    if r["exit"]!=0: return {"command":r}
    im=Image.open(png).convert("RGB");colored=[]
    for y in range(im.height):
        for x in range(im.width):
            a,b,c=im.getpixel((x,y))
            if max(a,b,c)-min(a,b,c)>80 and min(a,b,c)<160:
                colored.append((x,y))
    box=[min(x for x,y in colored),min(y for x,y in colored),max(x for x,y in colored)+1,max(y for x,y in colored)+1] if colored else None
    return {"command":r,"sha256":sha(png),"colored_bbox_pixels_72dpi":box}
def probes(selected):
    s=state();bins=binaries();results={}
    for fixture in sorted((BASE/"fixtures").glob("*.typ")):
        if selected and fixture.stem not in selected: continue
        case={}
        for name,binary in [("crystalline",CRYSTAL),("vanilla",VANILLA)]:
            out=BASE/"outputs"/fixture.stem/name
            out.parent.mkdir(parents=True,exist_ok=True)
            pdf=out.with_suffix(".pdf")
            args=[binary,"compile",fixture,pdf,"--font-path","/usr/share/fonts/truetype/dejavu"]
            measurement_state=state()
            binary_before=sha(binary)
            r=call(args)
            measurement_end=state()
            obs={"run":r,"fixture_sha256":sha(fixture),"binary_sha256":binary_before,"state":measurement_state,
                 "end_state":measurement_end,"binary_stable":binary_before==sha(binary)}
            if r["exit"]==0 and pdf.exists():
                obs["pdf_sha256"]=sha(pdf);obs["text"]=extract(pdf,out)
                if fixture.stem.startswith(("clip","float")): obs["raster"]=raster(pdf,out)
            save(out.with_suffix(".json"),obs);case[name]=obs
        results[fixture.stem]=case
        print(json.dumps({"fixture":fixture.name,**{k:{"exit":v["run"]["exit"],"pages":len(v.get("text",{}).get("pages",[])),
            "words":[w["text"] for p in v.get("text",{}).get("pages",[]) for w in p["words"]],
            "color_bbox":v.get("raster",{}).get("colored_bbox_pixels_72dpi")}for k,v in case.items()}},ensure_ascii=False))
    end=state()
    stable=end["source_identity"]==s["source_identity"]
    record=dict(state=s,binaries=bins,results=results,end_state=end,stable=stable)
    run_id=hashlib.sha256(json.dumps(record,sort_keys=True).encode()).hexdigest()[:24]
    save(BASE/"receipts"/("probes-"+run_id+".json"),record)
    save(BASE/"receipts"/"latest-probes.json",record)
    if not stable: raise SystemExit("Source changed during probes; receipts retained")
if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("--build",action="store_true");p.add_argument("--only",nargs="*");args=p.parse_args()
    if args.build: build()
    else: probes(set(args.only or []))
