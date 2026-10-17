from pathlib import Path
import hashlib,json,gzip,datetime,subprocess,re
root=Path.cwd();pkg=Path('00_nucleo/materialization/tiling-transporte-2026-10-14');out=Path('00_nucleo/diagnosticos/tiling-carrier-2026-10-14');out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
expected='d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d'
assert sha(pkg/'manifest.json')==expected
m=json.loads((pkg/'manifest.json').read_text());assert sha(pkg/'contract.md')==m['contract_sha256'];assert all(sha(pkg/n)==h for n,h in m['candidates'].items())
changed=[n for n,h in m['base_files'].items() if sha(n)!=h]
receipt=dict(recorded_utc=now(),questionItemId='["request_user_input_async","call_28239ea1b7764508bca3c64c50aaac73",0]',answer='Aprovar a proposta',manifest_sha256=expected,candidates_verified=True,changed_since_proposal=changed,scope='Autoriza T1–T3 do contrato aprovado; não autoriza publicação/release; T4 fora deste pacote.')
p=pkg/'approval-receipt.json';assert not p.exists();p.write_text(json.dumps(receipt,indent=2)+'\n')
src='01_core/src/entities/frame_paint.rs';mod='01_core/src/entities/mod.rs';owner='00_nucleo/prompts/entities/frame_paint.md';modowner='00_nucleo/prompts/entities/mod.md';nucleus='00_nucleo/prompts/_nuclei/tiling-frame-transport.toml'
paths=[src,mod,owner,modowner,nucleus]
r=dict(utc=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:None if not Path(n).exists() else dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
(out/'owners.json').write_text(json.dumps({src:owner,mod:modowner},indent=2))
# Canonical L0 first. Preserve the immutable approval package.
Path(nucleus).write_bytes((pkg/'candidates'/nucleus).read_bytes())
for dst in [owner,modowner]:
 candidate=(pkg/'candidates'/dst).read_text()
 if dst==modowner:
  assert sha(dst)==m['base_files'][dst], 'Merge new facade L0 changes before proceeding'
 candidate=candidate.replace('Contrato candidato — gate ADR-0127','Contrato ratificado — ADR-0127').replace('Transporte de célula preparada — candidato ADR-0127','Transporte de célula preparada — ratificado ADR-0127').replace('Contrato candidato exclusivo','Contrato exclusivo ratificado')
 candidate += '''\n## Ratificação e fronteira da etapa T1a — 2026-10-14\n\nO humano aprovou “Aprovar a proposta” para o manifesto\n`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`.\nO recibo está em `00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.\nA exigência de aguardar aprovação acima está satisfeita, sem alterar o pacote\naprovado. T1a instala os tipos puros e sua declaração na fachada. A migração\ndos campos de FrameItem e produtores/consumidores completa T1b; PDF/SVG/raster,\nrecursos e clipping completam T2; integrações geométricas completam T3.\nEste estado é explicitamente incompleto e não comprova correção do render.\nNenhum fallback legado é legitimado pelo novo carrier.\n\nNesta unidade, testar transporte integral de Solid/Gradient, retenção da célula\npreparada por Arc sem cópia proporcional à repetição, geometria completa de\ncontorno e equivalência dos defaults com Stroke::default. Não oferecer conversão\ntotal de Paint ou Tiling declarativo. Ausência de PartialEq nos novos carriers\nnão altera igualdade da linguagem. Testes de compilação verificam a fronteira.\n'''
 Path(dst).write_text(candidate)
# Pin the shared nucleus in the facade too.
p=Path(modowner);s=p.read_text();marker='\n\n**Layer:**';assert marker in s;s=s.replace(marker,'\n\nNúcleos Tekt:\n- '+nucleus+' sha256:'+m['nucleus_pin']+marker,1);p.write_text(s)
p=Path(mod);s=p.read_text();assert 'pub mod frame_paint;' not in s;s=s.replace('pub mod paint;','pub mod paint;\n// Prepared paint carriers; declarative Paint remains separate.\npub mod frame_paint;',1);p.write_text(s)
Path(src).write_text('//! Crystalline Lineage\n//! @prompt '+owner+'\n//! @prompt-hash 00000000\n//! @layer L1\n//! @updated 2026-10-14\n\n'+Path('/tmp/tekt-tiling-carrier-tests.txt').read_text())
print(json.dumps(receipt,indent=2))
