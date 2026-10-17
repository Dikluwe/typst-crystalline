import datetime, hashlib, json, os, subprocess, tempfile, xml.etree.ElementTree as ET
from pathlib import Path
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
FIXTURES = ROOT / '00_nucleo/diagnosticos/conjunto-prioridades-vanilla-2026-10-08/fixtures'
FONTS = ROOT / '00_nucleo/diagnosticos/performance-baseline/fonts'
BINARIES = {'tekt': ROOT / 'target/release/typst', 'vanilla': ROOT / 'lab/typst-original/target/release/typst'}
CASES = ['context', 'text-dense', 'table-alias', 'table-local', 'image']
NS = {'x': 'http://www.w3.org/1999/xhtml'}
ENV = os.environ.copy()
ENV.update({'RAYON_NUM_THREADS': '1', 'CRYSTALLINE_PDF_FIXED_EPOCH': '0', 'SOURCE_DATE_EPOCH': '0', 'TZ': 'UTC'})

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def words(pdf):
    root = ET.fromstring(subprocess.check_output(['pdftotext', '-bbox', str(pdf), '-']))
    return [{**{k: float(v) for k,v in word.attrib.items()}, 'text': word.text} for word in root.findall('.//x:word', NS)]
def raster(pdf, dpi, prefix):
    out = prefix.with_name(prefix.name + f'-{dpi}')
    subprocess.run(['pdftoppm', '-f', '1', '-singlefile', '-r', str(dpi), '-png', str(pdf), str(out)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return Path(str(out) + '.png')

result = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'head': subprocess.check_output(['git','rev-parse','HEAD'], cwd=ROOT, text=True).strip(), 'git_diff_head_stat_sha256': None, 'binaries': {name: sha(path) for name,path in BINARIES.items()}, 'fixtures': {name: sha(FIXTURES / f'{name}.typ') for name in CASES}, 'font_dir': str(FONTS), 'cases': {}}
stat = subprocess.check_output(['git','diff','HEAD','--stat'], cwd=ROOT)
(HERE/'git-diff-head-stat.txt').write_bytes(stat)
result['git_diff_head_stat_sha256'] = hashlib.sha256(stat).hexdigest()
status = subprocess.check_output(['git','status','--short'], cwd=ROOT)
(HERE/'git-status-short.txt').write_bytes(status)
result['git_status_short_sha256'] = hashlib.sha256(status).hexdigest()
with tempfile.TemporaryDirectory(prefix='item4-measure-') as td:
    td = Path(td)
    for name in CASES:
        group = {}
        for engine, binary in BINARIES.items():
            pdf = td / f'{name}-{engine}.pdf'
            cmd = [str(binary), 'compile', '--root', str(FIXTURES), '--font-path', str(FONTS), '--pdf-tagged=false', '--pretty=false']
            if engine == 'vanilla': cmd += ['--jobs', '1']
            cmd += [str(FIXTURES / f'{name}.typ') if engine == 'vanilla' else f'{name}.typ', str(pdf)]
            subprocess.run(cmd, env=ENV, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            group[engine] = {'pdf_sha256': sha(pdf), 'words': words(pdf), 'pdf': pdf}
        vw, tw = group['vanilla']['words'], group['tekt']['words']
        equal_words = [w['text'] for w in vw] == [w['text'] for w in tw]
        max_bbox_delta = max((abs(a[k]-b[k]) for a,b in zip(vw,tw) for k in ('xMin','xMax','yMin','yMax')), default=0) if equal_words else None
        rasters = {}
        for dpi in (36,72,144,288):
            v = Image.open(raster(group['vanilla']['pdf'],dpi,td/f'{name}-vanilla')).convert('RGB')
            t = Image.open(raster(group['tekt']['pdf'],dpi,td/f'{name}-tekt')).convert('RGB')
            diff = ImageChops.difference(v,t)
            rasters[str(dpi)] = {'size': v.size, 'changed_pixels': sum(1 for px in diff.getdata() if px != (0,0,0)) if v.size == t.size else None, 'bbox': diff.getbbox() if v.size == t.size else None}
        result['cases'][name] = {'word_sequence_equal': equal_words, 'word_count': len(vw), 'max_word_bbox_delta_pt': max_bbox_delta, 'pdf_sha256': {e:g['pdf_sha256'] for e,g in group.items()}, 'rasters': rasters}
        print(name, result['cases'][name]['max_word_bbox_delta_pt'], {k:v['changed_pixels'] for k,v in rasters.items()}, flush=True)
(HERE/'measurements.json').write_text(json.dumps(result,indent=2)+'\n')
