#!/usr/bin/env python3
"""Apply the quarantined experiment to a separate RAM pipeline copy.

Usage: apply_overlay.py PIPELINE_RAM [EXPERIMENT_RAM]
PIPELINE_RAM must be under /dev/shm/tekt-joint-discovery-20261105 and distinct
from workspace/. EXPERIMENT_RAM is an already prepared RAM copy of this crate;
if omitted, a fresh copy is made at D/experiment (an existing copy is preserved).
No compilation, root lock updates or dependency version changes are performed.
"""
from pathlib import Path
import shutil
import sys
E=Path(__file__).resolve().parent
D=Path('/dev/shm/tekt-joint-discovery-20261105').resolve()
P=Path(sys.argv[1]).resolve()
assert P.is_relative_to(D) and P not in (D,D/'workspace'), 'Use a separate RAM pipeline copy'
assert (P/'03_infra/Cargo.toml').is_file(), 'Prepare a separate pipeline copy first'
C=Path(sys.argv[2]).resolve() if len(sys.argv)>2 else D/'experiment'
assert C.is_relative_to(D) and C not in (D,D/'workspace',P), 'Experiment must be a separate RAM copy'
if not C.exists():
    shutil.copytree(E,C,ignore=shutil.ignore_patterns('target','__pycache__'))
assert (C/'Cargo.toml').is_file() and (C/'vendor/fontdb/Cargo.toml').is_file()
for src in (C/'overlays').rglob('*.rs'):
    target=P/src.relative_to(C/'overlays')
    assert target.is_file(),target
    shutil.copyfile(src,target)
p=P/'03_infra/Cargo.toml'; s=p.read_text()
assert 'joint_font_discovery' not in s,'Experiment already linked'
assert '[dependencies]\n' in s
s=s.replace('[dependencies]\n','[dependencies]\njoint_font_discovery = { path = "'+str(C)+'" }\n',1)
p.write_text(s)
p=P/'Cargo.toml'; s=p.read_text()
patch='fontdb = { path = "'+str(C/'vendor/fontdb')+'" }\n'
if '[patch.crates-io]' in s:
    section=s.split('[patch.crates-io]',1)[1].split('\n[',1)[0]
    assert 'fontdb' not in section,'Existing fontdb patch requires review'
    s=s.replace('[patch.crates-io]','[patch.crates-io]\n'+patch,1)
else:
    s+='\n[patch.crates-io]\n'+patch
p.write_text(s)
print('Applied only to',P,'using RAM experiment',C)
print('Enable the observational candidate with E_JOINT_DISCOVERY=1; C performs all builds/checks.')
