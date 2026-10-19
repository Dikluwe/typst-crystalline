from pathlib import Path
import datetime, hashlib, json, subprocess
root = Path.cwd()
out = root / '00_nucleo/diagnosticos/auditoria-composicao-construcao-2026-09-29'
bin = root / 'target/release/typst'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
(out / 'single.typ').write_text('#set format.pdf(standard: "ua-1")\nHello\n')
(out / 'bundle.typ').write_text('#document("one.pdf")[#set format.pdf(standard: "ua-1")\nHello]\n')
(out / 'git-diff-stat.txt').write_text(subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True))
(out / 'git-status.txt').write_text(subprocess.check_output(['git', 'status', '--porcelain=v1', '-uall'], text=True))
base = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), 'working_tree': 'não commitada', 'binary_sha256': sha(bin), 'inputs_sha256': {'single.typ': sha(out / 'single.typ'), 'bundle.typ': sha(out / 'bundle.typ')}, 'runs': {}}
for name, src, dst, extra in [('single', out/'single.typ', out/'single.pdf', ['--format','pdf']), ('bundle', out/'bundle.typ', out/'bundle-out', ['--format','bundle','--features','bundle'])]:
    cmd = [str(bin), 'compile', str(src), str(dst), *extra]
    run = subprocess.run(cmd, capture_output=True, text=True)
    base['runs'][name] = {'command': cmd, 'exit': run.returncode, 'stdout': run.stdout, 'stderr': run.stderr}
    (out / f'{name}.stderr').write_text(run.stderr)
    if name == 'bundle' and run.returncode == 0:
        pdf = dst / 'one.pdf'
        info = subprocess.run(['pdfinfo', '-meta', str(pdf)], capture_output=True, text=True, check=True)
        base['runs'][name]['pdf_sha256'] = sha(pdf)
        base['runs'][name]['pdf_bytes'] = pdf.stat().st_size
        base['runs'][name]['xmp_pdfua1'] = '<pdfuaid:part>1</pdfuaid:part>' in info.stdout
        base['runs'][name]['xmp_title'] = '<dc:title>' in info.stdout
(out / 'probe.json').write_text(json.dumps(base, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({k:v for k,v in base.items() if k != 'runs'}, ensure_ascii=False))
print(json.dumps({k:{kk:vv for kk,vv in v.items() if kk not in ('command','stderr','stdout')} for k,v in base['runs'].items()}))
