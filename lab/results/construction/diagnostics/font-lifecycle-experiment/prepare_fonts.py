"""Create one shared, uniquely named math face, without editing its source font."""
import hashlib, json
from pathlib import Path
from fontTools import __version__
from fontTools.ttLib import TTFont
OUT=Path(__file__).resolve().parent
source=Path('/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts/NewCMMath-Regular.otf')
font=TTFont(source,recalcTimestamp=False)
for record in font['name'].names:
    if record.nameID in {1,2,3,4,6,16,17}:
        value={1:'Baseline Math',2:'Regular',3:'BaselineMath-Regular',4:'Baseline Math Regular',6:'BaselineMath-Regular',16:'Baseline Math',17:'Regular'}[record.nameID]
        record.string=value.encode(record.getEncoding())
(OUT/'fonts').mkdir(exist_ok=True)
target=OUT/'fonts'/'BaselineMath-Regular.otf';font.save(target);font.close()
license_source=source.parent/'LICENSE'; candidates=[source.parents[2]/'LICENSE',source.parents[2]/'NOTICE']
for p in candidates:
    if p.exists():(OUT/'fonts'/p.name).write_bytes(p.read_bytes())
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(OUT/'shared-math-font.json').write_text(json.dumps({'source':str(source),'source_sha256':h(source),'output':str(target),'output_sha256':h(target),'fonttools_version':__version__,'operation':'Only name table IDs 1,2,3,4,6,16,17 changed to avoid embedded/system family collision; same resulting bytes supplied to both engines.'},indent=2)+'\n')
