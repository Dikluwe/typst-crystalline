from pathlib import Path
import shutil
root=Path.cwd();out=Path('/tmp/tekt-separator-baseline');out.mkdir(exist_ok=True)
for name in ['01_core','02_shell','03_infra','04_wiring','benches','.cargo']:
 if (root/name).exists():shutil.copytree(root/name,out/name,dirs_exist_ok=True,ignore=shutil.ignore_patterns('target'))
for name in ['Cargo.toml','Cargo.lock','crystalline.toml']:
 if (root/name).exists():shutil.copy2(root/name,out/name)
if not (out/'.git').exists():(out/'.git').symlink_to(root/'.git',target_is_directory=True)
before=root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/before'
for f in before.rglob('*.rs.txt'):
 target=out/str(f.relative_to(before))[:-4];target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(f.read_bytes())
