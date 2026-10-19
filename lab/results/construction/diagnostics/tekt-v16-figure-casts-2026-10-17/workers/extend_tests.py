from pathlib import Path
import json,shutil,datetime,hashlib
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-resources.json'))['exit']==101;assert '1 passed; 4 failed;' in (out/'red.log').read_text()
p=Path('01_core/src/compiler/stdlib/figure_image.rs');s=p.read_text();assert 'fn figure_domain_located_values' not in s;pos=s.rfind('\n}');p.write_text(s[:pos]+Path('/tmp/figure-located-test.txt').read_text()+s[pos:]);shutil.copy('/tmp/figure-located-test.txt',out/'figure-located-test.txt');(out/'located-tests-before-code.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(p.read_bytes()).hexdigest()),indent=2)+'\n')
