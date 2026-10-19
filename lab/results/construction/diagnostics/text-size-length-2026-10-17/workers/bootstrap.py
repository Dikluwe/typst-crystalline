from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,shutil
out=Path('00_nucleo/diagnosticos/text-size-length-2026-10-17');out.mkdir(exist_ok=False);(out/'workers').mkdir()
p=Path('00_nucleo/materialization/text-size-length-2026-10-17');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert sha(p/'SHA256SUMS')=='771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'
for line in (p/'SHA256SUMS').read_text().splitlines():h,f=line.split('  ',1);assert sha(p/f)==h
scope=json.load(open(p/'scope.json'));prior=json.load(gzip.open('00_nucleo/diagnosticos/tekt-v16-abs-2026-10-17/final-state.json.gz'));assert all(sha(f)==h for f,h in prior['source_hashes'].items());sources=scope['semantic_sources']+scope['adapter_sources']+scope['test_only_sources'];owners=scope['owners'];files={f:Path(f).read_text() for f in sources+list(owners.values())}
b=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),hashes=prior['source_hashes'],files=files,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),linter=dict(path='/tmp/tekt-linter-none-target/debug/crystalline-lint',sha256=sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')),approval=dict(answer='Aprovar proposta TS1',manifest=sha(p/'SHA256SUMS'),api=scope['public_changes']))
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
scope.update(sources=sources,status='TS1 APPROVED by human; RG1 remains pending');(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n');(out/'approval.json').write_text(json.dumps(b['approval'],indent=2)+'\n');(out/'empty.typ').write_text('')
old=Path('00_nucleo/diagnosticos/tekt-v16-abs-2026-10-17/workers')
for name in ['run.py','strict.py','suite.py','rustdoc.py']:
 s=(old/name).read_text().replace('tekt-v16-abs-2026-10-17','text-size-length-2026-10-17').replace('TEKT_ABS_MEMORY_LIMIT','TEKT_TS1_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
for f in owners.values():
 current=Path(f).read_text();candidate=(p/'candidates'/f).read_text();addition=candidate.split('## TS1 —',1)[1];addition='## TS1 —'+addition;addition=addition.replace('(PROPOSTO; gate ADR-0127)','(ratificado pelo humano, 2026-10-17)').replace('### Ratificação obrigatória','### Ratificação').replace('Este candidato altera dois contratos públicos, descritos em api.md. Não é\nL0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,\nnão resselo de implementação inexistente. Materializar somente após ratificar\nTS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.','O humano aprovou TS1 e os dois contratos públicos da proposta de manifesto\n771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba.\nImplementação e testes S1–S16 sob 12 GiB/swap zero; sem release/merge/push.\nResselo somente após implementação e grafo válido. RG1 continua independente.')
 pin='- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:8a628c506765c5d67144248103049ab49d2f1402312cb210c25ac9902b0f2682'
 if pin in candidate:
  if 'Núcleos Tekt:' in current:current=current.replace('Núcleos Tekt:','Núcleos Tekt:\n'+pin,1)
  else:current+='\n\nNúcleos Tekt:\n'+pin+'\n'
 current=current.replace('`Size(Pt)` — propriedade `text.size` em pontos tipográficos','`Size(Length)` — propriedade `text.size` declarada como abs + em').replace('pub size:   Option<f64>,       // pontos tipográficos','pub size:   Option<Length>,    // tamanho declarado abs + em')
 Path(f).write_text(current+'\n\n'+addition)
f='00_nucleo/prompts/_nuclei/text-size-affine.toml';Path(f).write_text((p/'candidates'/f).read_text());assert sha(f)=='8a628c506765c5d67144248103049ab49d2f1402312cb210c25ac9902b0f2682'
(out/'l0-before-tests.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),hashes={f:sha(f) for f in owners.values()},nucleus_sha=sha(f)),indent=2)+'\n');shutil.copy('/tmp/bootstrap-ts1.py',out/'workers/bootstrap.py');print('TS1 baseline, approval and 19 L0 owners saved; Par/Box retained')
