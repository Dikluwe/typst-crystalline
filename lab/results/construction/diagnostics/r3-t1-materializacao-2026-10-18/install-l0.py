from pathlib import Path
import json,gzip,hashlib,datetime
p=Path(__file__).resolve().parent;r=Path.cwd();proposal=r/'00_nucleo/materialization/textual-realization-2026-10-18'
receipt=json.loads((proposal/'approval-receipt.json').read_text());assert receipt['answer']=='Aprovar R3-T1'
assert hashlib.sha256((proposal/'manifest.json').read_bytes()).hexdigest()==receipt['approved_manifest_sha256']
baseline=json.loads((proposal/'baseline-owners.json').read_text())
nucleus='00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml'
target=r/nucleus;assert not target.exists();target.write_bytes((proposal/'candidates'/nucleus).read_bytes())
pin='- '+nucleus+' sha256:'+baseline['new_nucleus_effective_sha256']
rows=[]
for owner in baseline['owners']:
 f=r/owner['path'];before=f.read_text();candidate=(proposal/owner['candidate']).read_text();marker='## R3-T1 — candidato, NÃO vigente antes do gate de ordem (2026-10-18)';assert candidate.count(marker)==1 and marker not in before
 addition=marker+candidate.split(marker,1)[1]
 addition=addition.replace(marker,'## R3-T1 — realização causal ratificada (2026-10-18)').replace('### Decisão sujeita a aprovação','### Decisão ratificada').replace('Aplicar o núcleo textual-recipe-frontier pinado abaixo, somente depois da\nconfirmação humana do manifesto da revisão. Ele supersede neste escopo as','Aplicar o núcleo textual-recipe-frontier pinado neste L0. A confirmação humana\ndo manifesto 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302\nratificou esta ordem. Ele supersede neste escopo as')
 assert 'Núcleos Tekt:\n' in before;after=before.replace('Núcleos Tekt:\n','Núcleos Tekt:\n'+pin+'\n',1).rstrip()+'\n\n'+addition
 f.write_text(after);rows.append(dict(path=owner['path'],before_sha256=hashlib.sha256(before.encode()).hexdigest(),after_sha256=hashlib.sha256(after.encode()).hexdigest(),approved_baseline_sha256=owner['sha256'],intervening_clauses_preserved=hashlib.sha256(before.encode()).hexdigest()!=owner['sha256']))
(p/'l0-installed.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope='Eleven existing owners installed additively before tests/code; the new recipes owner will be installed before creating its consumer after the functional RED.',approval_receipt=str(proposal/'approval-receipt.json'),owners=rows,nucleus=nucleus),indent=2)+'\n')
print('installed',len(rows),'existing L0s and approved nucleus; preserved intervening bodies')
