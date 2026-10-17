from pathlib import Path
import re,hashlib,gzip,json
H=Path('00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05');before=json.loads(gzip.open(H/'before.json.gz','rt').read())['sources'];scope=[]
for folder in ['01_core/src','02_shell/src','03_infra/src','04_wiring/src']:
 for p in Path(folder).rglob('*.rs'):
  source=p.read_text()
  m=re.search(r'^//! @prompt (.+)$',source,re.M)
  if not m:continue
  owner=Path(m[1]);spec=owner.read_text();
  if before.get(str(p))==source and before.get(str(owner))==spec:continue
  body=re.sub(r'^//! @prompt-hash[^\n]*\n','',source,flags=re.M);codehash=hashlib.sha256(body.encode()).hexdigest()[:8]
  if 'Hash do Código:' in spec:spec=re.sub(r'Hash do Código: [a-f0-9]+',f'Hash do Código: {codehash}',spec)
  else:spec=spec.replace('\n',f'\nHash do Código: {codehash}\n',1)
  if owner.name=='bidi_context.md' and owner.parent.name=='entities':spec=spec.replace('**Status:** proposta aguardando confirmação ADR-0127; consumer ainda não materializado.','**Status:** contrato ratificado pelo humano; consumer materializado. Aprovação imutável em `00_nucleo/materialization/bidi-contexto-aprovacao-2026-10-05.json`.').replace('## Contrato público a confirmar','## Contrato público ratificado')
  owner.write_text(spec);scope.append({'source':str(p),'owner':str(owner),'code_hash':codehash})
# entity stub was already present in initial receipt: still changed now so included.
(H/'scope.json').write_text(json.dumps(scope,indent=2)+'\n');print(len(scope),'owners changed')
