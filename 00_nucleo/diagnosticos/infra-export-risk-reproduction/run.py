"""Run isolated observations; never modify productive sources or L0s.

Use the bundled Python (Pillow/pypdf), from any working directory.
Outputs are generated diagnostic data, not new productive tests.
"""
import datetime, hashlib, http.server, io, json, os, pathlib, shutil, struct
import subprocess, sys, tarfile, tempfile, threading, zlib
from PIL import Image, ImageDraw
from pypdf import PdfReader

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
OUT = HERE / "results"
OUT.mkdir(exist_ok=True)
COMMANDS = []
PIN = "9f2b6e8715237cb086899a42873660fe744622e8"
VANILLA_SHA = "7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea"

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def save(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")

def command(argv, timeout=120, env=None):
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    p = subprocess.run(list(map(str, argv)), cwd=ROOT, text=True, capture_output=True,
                       timeout=timeout, env=env)
    COMMANDS.append(dict(argv=list(map(str, argv)), started=started, exit_code=p.returncode,
                         stdout=p.stdout, stderr=p.stderr))
    save("commands.json", COMMANDS)
    return p

def snapshot():
    status = subprocess.check_output(["git", "status", "--porcelain=v1", "-uall"], cwd=ROOT, text=True)
    # Explicit user restriction: no git diff --stat or line-count metrics.
    changed = subprocess.check_output(["git", "diff", "HEAD", "--name-status"], cwd=ROOT, text=True)
    paths = subprocess.check_output(["git", "ls-files", "--cached", "--others", "--exclude-standard"], cwd=ROOT, text=True).splitlines()
    protected = {p: sha(ROOT / p) for p in sorted(set(paths))
                 if (ROOT / p).is_file() and not p.startswith(str(HERE.relative_to(ROOT)))
                 and p != "00_nucleo/diagnosticos/infra-export-risk-reproduction.md"}
    return dict(time=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                head=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                working_tree="uncommitted", porcelain=status, tracked_changes=changed,
                protected_sha256=protected)

def png_chunk(tag, data):
    return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", zlib.crc32(tag + data))

def make_inputs():
    image = Image.new("RGB", (2, 2), "red")
    buf = io.BytesIO(); image.save(buf, format="PNG"); png = buf.getvalue()
    (OUT / "red.png").write_bytes(png)
    (OUT / "no-density.png").write_bytes(png)
    # Insert valid metadata after IHDR. Other pixels/geometry stay constant.
    for name, density in [("zero-phys", 0), ("positive-phys", 2835)]:
        chunk = png_chunk(b"pHYs", struct.pack(">IIB", density, density, 1))
        (OUT / (name + ".png")).write_bytes(png[:33] + chunk + png[33:])
    # Valid little-endian TIFF IFD with XResolution=0/1 and inch units.
    tiff = (b"II" + struct.pack("<HIH", 42, 8, 2)
            + struct.pack("<HHII", 0x11A, 5, 1, 38)
            + struct.pack("<HHI", 0x128, 3, 1) + struct.pack("<HH", 2, 0)
            + struct.pack("<I", 0) + struct.pack("<II", 0, 1))
    (OUT / "zero-exif.png").write_bytes(png[:33] + png_chunk(b"eXIf", tiff) + png[33:])
    for name, density in [("zero-jfif", 0), ("positive-jfif", 72)]:
        buf = io.BytesIO(); image.save(buf, format="JPEG", dpi=(density, density))
        jpg = bytearray(buf.getvalue()); assert jpg[6:11] == b"JFIF\0"
        jpg[13] = 1; jpg[14:18] = struct.pack(">HH", density, density)
        (OUT / (name + ".jpg")).write_bytes(jpg)
    jpg = (OUT / "positive-jfif.jpg").read_bytes()
    exif = b"Exif\0\0" + tiff
    (OUT / "zero-exif.jpg").write_bytes(jpg[:2] + b"\xff\xe1" + struct.pack(">H", len(exif)+2) + exif + jpg[2:])
    # Verify the fixtures genuinely decode; do not rely only on a header scanner.
    save("input-validation.json", {p.name:dict(size=Image.open(p).size, format=Image.open(p).format)
                                  for p in OUT.glob("*.png") if p.name in
                                  ["red.png", "no-density.png", "zero-phys.png", "positive-phys.png", "zero-exif.png"]})

def raster_observations():
    result = {}
    for name in ["group-clip", "group-no-clip", "image-clip", "image-no-clip", "cap-butt", "cap-round", "cap-square"]:
        im = Image.open(OUT / (name + ".png")).convert("RGB")
        mask = Image.new("1", im.size)
        mask.putdata([r < 240 or g < 240 or b < 240 for r,g,b in im.getdata()])
        result[name] = dict(size=im.size, painted_bbox_pixels=mask.getbbox(),
                           inside_pixel=im.getpixel((30,30)), outside_clip_pixel=im.getpixel((70,70)))
    for name in ["group-clip", "group-no-clip", "image-clip", "image-no-clip", "cap-butt", "cap-round", "cap-square"]:
        output = OUT / (name + ".svg-render.png")
        p = command(["rsvg-convert", "-w", "200", "-h", "200", "-o", output, OUT/(name+".svg")])
        assert p.returncode==0,p.stderr
        im = Image.open(output).convert("RGB"); mask = Image.new("1", im.size)
        mask.putdata([r<240 or g<240 or b<240 for r,g,b in im.getdata()])
        result[name+"-svg"] = dict(size=im.size, painted_bbox_pixels=mask.getbbox(),
                                   outside_clip_pixel=im.getpixel((70,70)))
    save("raster-observations.json", result)
    import xml.etree.ElementTree as ET
    save("svg-observations.json", {name:dict(ET.parse(OUT / (name + ".svg")).getroot().find("{http://www.w3.org/2000/svg}line").attrib)
                                  for name in ["cap-butt", "cap-round", "cap-square"]})

def pdf_observations():
    results = {}
    for path in OUT.glob("tag-*.pdf"):
        reader = PdfReader(path); page = reader.pages[0]
        data = page.get_contents().get_data().decode("latin1")
        root = reader.trailer["/Root"]
        tree = root.get("/StructTreeRoot")
        elements = []
        def visit(node):
            if isinstance(node, list):
                for child in node: visit(child)
            elif hasattr(node, "get_object"):
                node = node.get_object()
                if hasattr(node, "get"):
                    if node.get("/S"): elements.append(dict(role=str(node["/S"]), k=str(node.get("/K"))))
                    if node.get("/K") is not None: visit(node["/K"])
        if tree: visit(tree.get_object())
        relationships = []
        if tree:
            nums = tree.get_object()["/ParentTree"].get_object().get("/Nums", [])
            for key, entries in zip(nums[::2], nums[1::2]):
                relationships.append(dict(key=int(key), nodes=[dict(role=str(ref.get_object().get("/S")),
                    mcid=str(ref.get_object().get("/K")),
                    page_matches=ref.get_object().get("/Pg")==page.indirect_reference) for ref in entries]))
        results[path.name] = dict(extracted_text=page.extract_text(), content=data,
                                 structure=elements, parent_relationships=relationships, parent_tree=str(tree.get_object().get("/ParentTree")) if tree else None)
        command(["qpdf", "--check", path])
        command(["pdftoppm", "-r", "72", "-png", "-singlefile", path, path.with_suffix("")])
    save("pdf-observations.json", results)
    reader = PdfReader(OUT/"tag-body-ordinary.pdf")
    cs = reader.pages[0]["/Resources"]["/ColorSpace"]["/c0"]
    profile = cs[1].get_object()
    save("icc-success-control.json", dict(filter=str(profile.get("/Filter")),
         decoded_signature=profile.get_data()[36:40].decode(), decoded_sha256=hashlib.sha256(profile.get_data()).hexdigest()))

def download_observations(binary):
    result = {}
    for mode in ["normal", "race", "bad-archive"]:
        with tempfile.TemporaryDirectory(prefix="infra-export-risk-") as temp:
            cache = pathlib.Path(temp) / "cache"
            buffer = io.BytesIO()
            with tarfile.open(fileobj=buffer, mode="w:gz") as tar:
                data = b"loser-payload"; info = tarfile.TarInfo("marker.txt"); info.size = len(data)
                tar.addfile(info, io.BytesIO(data))
            payload = b"not gzip" if mode == "bad-archive" else buffer.getvalue()
            (OUT / ("invalid-package.tar.gz" if mode == "bad-archive" else "local-package.tar.gz")).write_bytes(payload)
            requests = []
            class Handler(http.server.BaseHTTPRequestHandler):
                def do_GET(self):
                    requests.append(self.path)
                    if mode == "race":
                        # Download has already created its temp and passed the is_dir check.
                        # A competing publisher wins BEFORE this response is released.
                        parent = cache / "preview/risk-probe"
                        winner = parent / ".local-winner"
                        winner.mkdir(); (winner / "marker.txt").write_text("winner-payload")
                        winner.rename(parent / "1.0.0")
                    self.send_response(200); self.send_header("Content-Length", str(len(payload)))
                    self.end_headers(); self.wfile.write(payload)
                def log_message(self, *args): pass
            server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
            thread = threading.Thread(target=server.serve_forever, daemon=True); thread.start()
            env = dict(os.environ)
            for key in ["HTTPS_PROXY", "https_proxy", "HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"]: env.pop(key, None)
            try:
                p = command([binary, "download", cache, f"http://127.0.0.1:{server.server_port}"], timeout=20, env=env)
                inventory = {str(p.relative_to(cache)):p.read_text() for p in cache.rglob("marker.txt")}
                result[mode] = dict(exit_code=p.returncode, reply=p.stdout, requests=requests, files=inventory,
                                    remaining_temp_dirs=[str(p.relative_to(cache)) for p in cache.rglob(".tmp-*")])
            finally:
                server.shutdown(); server.server_close(); thread.join(timeout=5)
    save("download-observations.json", result)

def vanilla_observations(vanilla):
    # Call only after identity verification; no PATH oracle.
    results = {}
    for name in ["quoted-control", "escaped-quote", "braced-quote", "negative-volume", "positive-sign-volume", "braced-negative-volume", "negative-year", "braced-negative-year"]:
        typ = OUT / (name + ".typ")
        typ.write_text(f'#set page(width: 200pt, height: 120pt)\n#bibliography("{name}.bib", full: true)\n')
        p = command([vanilla, "compile", typ, typ.with_suffix(".vanilla.pdf")], timeout=30)
        extracted = PdfReader(typ.with_suffix(".vanilla.pdf")).pages[0].extract_text() if p.returncode==0 else None
        results[name] = dict(exit_code=p.returncode, diagnostic=p.stderr, extracted_text=extracted)
    for name in ["no-density.png", "zero-phys.png", "positive-phys.png", "zero-exif.png", "zero-jfif.jpg", "positive-jfif.jpg", "zero-exif.jpg"]:
        typ = OUT / (name + ".typ")
        typ.write_text(f'#set page(width: 100pt, height: 100pt, margin: 0pt)\n#image("{name}")\n')
        p = command([vanilla, "compile", typ, typ.with_suffix(".vanilla.svg")], timeout=30)
        results[name] = dict(exit_code=p.returncode, diagnostic=p.stderr)
    save("vanilla-observations.json", results)

def main():
    before = snapshot(); save("provenance-before.json", before)
    executables = {}
    for name in ["cargo", "rustc", "qpdf", "pdftoppm", "rsvg-convert", "python3"]:
        p = shutil.which(name)
        if p: executables[name] = dict(path=p, realpath=os.path.realpath(p), sha256=sha(p))
    vanilla = ROOT / "lab/typst-original/target/release/typst"
    assert sha(vanilla)==VANILLA_SHA, "Vanilla binary differs from the P1489 pinned recipe"
    executables["vanilla"] = dict(path=str(vanilla), sha256=sha(vanilla), pin=PIN,
        tree="6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4", recipe="00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md")
    make_inputs()
    p = command(["cargo", "build", "--offline", "--manifest-path", HERE/"Cargo.toml", "--target-dir", ROOT/"target", "--quiet"], timeout=300)
    if p.returncode: raise RuntimeError(p.stderr[-3000:])
    binary = ROOT / "target/debug/infra-export-risk-probe"
    executables["probe"] = dict(path=str(binary), sha256=sha(binary))
    fault = ROOT / "target/debug/icc_fault"
    executables["icc-fault-probe"] = dict(path=str(fault), sha256=sha(fault))
    executables["python-runtime"] = dict(path=sys.executable, realpath=os.path.realpath(sys.executable), sha256=sha(sys.executable))
    for argv in [["rustc", "--version", "--verbose"], ["cargo", "--version"], ["qpdf", "--version"],
                 ["pdftoppm", "-v"], ["rsvg-convert", "--version"], [vanilla, "--version"]]:
        command(argv)
    save("executables.json", executables)
    p = command([binary, OUT]); assert p.returncode==0,p.stderr
    save("public-api-observations.json", [json.loads(s) for s in p.stdout.splitlines()])
    raster_observations(); pdf_observations(); download_observations(binary)
    p = command([fault, OUT/"icc-forced-error.stream"]); assert p.returncode==0,p.stderr
    stream = (OUT/"icc-forced-error.stream").read_bytes()
    header,payload = stream.split(b"\nstream\n",1); payload=payload.rsplit(b"\nendstream",1)[0]
    try:
        zlib.decompress(payload); decoded = "success"
    except zlib.error as error: decoded = str(error)
    save("icc-observations.json", dict(forced_dependency_failure=True,
        production_reachability="inconclusive: no natural compressor failure reproduced",
        header=header.decode(), stored_length=len(payload), icc_signature=payload[36:40].decode(),
        flate_decode=decoded))
    vanilla_observations(vanilla)
    cells = [("PDF body", "tag-body-ua1.png"), ("PDF background", "tag-background-ua1.png"),
             ("PDF foreground", "tag-foreground-ua1.png"), ("Raster group clip", "group-clip.png"),
             ("SVG group clip", "group-clip.svg-render.png"), ("Raster image clip", "image-clip.png"),
             ("Raster Square", "cap-square.png"), ("SVG Square", "cap-square.svg-render.png"),
             ("SVG image clip", "image-clip.svg-render.png")]
    sheet = Image.new("RGB", (600, 690), "white"); draw = ImageDraw.Draw(sheet)
    for i,(label,name) in enumerate(cells):
        x=(i%3)*200; y=(i//3)*230
        draw.text((x+3,y+3),label,fill="black")
        image = Image.open(OUT/name).convert("RGB"); image.thumbnail((200,200))
        sheet.paste(image,(x,y+25))
    sheet.save(OUT/"observation-preview.png")
    after = snapshot(); save("provenance-after.json", after)
    save("protected-changes.json", {p:dict(before=digest, after=after["protected_sha256"].get(p))
         for p,digest in before["protected_sha256"].items() if after["protected_sha256"].get(p)!=digest})
    save("artifact-sha256.json", {str(p.relative_to(HERE)):sha(p) for p in HERE.rglob("*") if p.is_file()
         and p.name!="artifact-sha256.json"})
    print("Diagnostic observations recorded in", OUT)

if __name__=="__main__": main()
