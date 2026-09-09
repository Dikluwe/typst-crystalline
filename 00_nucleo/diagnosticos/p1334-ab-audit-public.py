#!/usr/bin/env python3
"""Read-only audit of authorized public A/B receipts; writes only new evidence."""
import pathlib,json,hashlib,datetime,subprocess
D=pathlib.Path(__file__).resolve().parent
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(n):return json.loads((D/n).read_text())
def save(n,t):
 p=D/n;assert n.startswith('p1334-ab-') and not p.exists()
 subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+x+'\n' for x in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
def obs(r):return {k:r[k] for k in ['exit','stdout','stderr']}
freeze=read('p1334-ab-freeze-r1.json')
assert digest(D/'p1334-ab-freeze-r1.json')=='684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5'
for section in ['inputs','artifacts']:
 for name,pin in freeze[section].items():assert digest(D/name)==pin,(section,name)
for owner in freeze['owners']:
 p=D.parents[1]/owner['prompt'];lines=p.read_text().splitlines(keepends=True)
 assert hashlib.sha256(''.join(x for x in lines if not x.startswith('Hash do Código:')).encode()).hexdigest()==owner['norm_sha256'],owner['prompt']
report={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'manifest_sha256':freeze['manifest_sha256'],'freeze':{'file':'p1334-ab-freeze-r1.json','sha256':digest(D/'p1334-ab-freeze-r1.json')},'role':'B auditing only authorized public CLI outputs after freeze; no productive runtime or private RED/GREEN receipts read','verdict':'PASS','suites':[],'protected_hashes_verified':len(freeze['inputs'])+len(freeze['artifacts'])}
candidates=set();all_count=0
for suite,prefix,expected_name,cache_name,runner_name in [
 ('main','p1334-ab-cli','p1334-ab-cli-expected.json','p1334-ab-cli-baseline.json','p1334-ab-cli.py'),
 ('cross-source','p1334-ab-cross-cli','p1334-ab-cross-expected.json','p1334-ab-cross-baseline.json','p1334-ab-cross-cli.py')]:
 expected=read(expected_name);cache=read(cache_name)
 assert expected['baseline_sha256']==digest(D/cache_name)
 assert expected['runner_sha256']==digest(D/runner_name)
 ex={(r['case'],r['profile']):r for r in expected['expectations']};refs={(r['case'],r['profile']):r for r in cache['cases']}
 assert len(ex)==len(expected['expectations'])==len(refs)
 details={'suite':suite,'unique_cells':len(ex),'expected_sha256':digest(D/expected_name),'baseline_sha256':digest(D/cache_name),'runs':[]};first=None;normal_keys=None
 for order in ['normal','repeat','reverse']:
  n=prefix+'-'+order+'.json';run=read(n)
  assert run['manifest_sha256']==freeze['manifest_sha256']
  assert run['runner_sha256']==digest(D/runner_name)
  assert run['expected_sha256']==digest(D/expected_name)
  assert run['cache_sha256']==digest(D/cache_name)
  assert run['order']==('reverse' if order=='reverse' else 'normal')
  assert run['binaries']==cache['binaries']
  candidate=run['candidate_binary'];candidates.add((candidate['path'],candidate['sha256']))
  keys=[(r['case'],r['profile']) for r in run['cases']]
  assert len(keys)==len(set(keys))==len(ex) and set(keys)==set(ex)
  if order=='normal':normal_keys=keys
  elif order=='repeat':assert keys==normal_keys
  else:
   # Reversal is of expression order; profiles retain canonical order.
   groups=[normal_keys[i:i+4] for i in range(0,len(normal_keys),4)]
   assert keys==[k for group in reversed(groups) for k in group]
  actual={};earliest=None;latest=None
  for row in run['cases']:
   k=(row['case'],row['profile']);want=ex[k]['expected'];results=row['results']
   assert row['expected']==want
   assert obs(results['CANDIDATE'])==want,k
   assert row['candidate_matches_frozen_policy'] is True,k
   assert row['expression']==refs[k]['expression']
   for role in ['BASE','VANILLA']:assert results[role]==refs[k]['results'][role],(k,role)
   c=results['CANDIDATE'];assert c['argv'][0]==candidate['path']
   assert c['argv'][1:]==results['BASE']['argv'][1:]
   assert c['at']>freeze['utc']
   earliest=min(earliest,c['at']) if earliest else c['at'];latest=max(latest,c['at']) if latest else c['at']
   actual[k]=obs(c)
  if first is None:first=actual
  else:assert actual==first
  details['runs'].append({'file':n,'sha256':digest(D/n),'cells':len(keys),'matched':len(keys),'mismatches':0,'first_command_utc':earliest,'last_command_utc':latest})
  all_count+=len(keys)
 report['suites'].append(details)
assert len(candidates)==1
path,pin=next(iter(candidates));report['candidate_binary_reported_consistently']={'path':path,'sha256':pin,'qualification':'Read from six public receipts; B did not execute or inspect the candidate executable.'}
history=read('p1333-ab-cli-expected.json');main=read('p1334-ab-cli-expected.json')
assert digest(D/'p1333-ab-cli-expected.json')==main['historical_expectations_sha256']
old={(r['case'],r['profile']):r for r in history['expectations']};new={(r['case'],r['profile']):r for r in main['expectations']}
migrations={(r['case'],r['profile']):r for r in main['migration_ledger'] if r['historical']}
assert len(old)==712
for k,row in old.items():
 if k in migrations:assert row['expected']==migrations[k]['before'] and new[k]['expected']==migrations[k]['after']
 else:assert row['expected']==new[k]['expected']
report.update(total_observations=all_count,unique_cells=sum(s['unique_cells'] for s in report['suites']),historical_cells=712,historical_migrated_cells=len(migrations),historical_preserved_cells=712-len(migrations),normal_repeat_reverse_identical=True,reference_observations_unchanged=True,unknown_cells=0)
report['limits']=['Shared filesystem, no technical isolation attestation or refinement seal.','This receipt attests only literal CLI comparisons in the frozen corpus and protected artifact integrity, not global abs/calc parity.','Native RED/GREEN and build/workspace/lint receipts were private and were not inspected by B.','Historical stale Some fixtures remain local robustness outside causal parity.','Candidate executable identity is the hash reported consistently by public runner receipts; no independent binary execution or content inspection by B.']
save('p1334-ab-public-audit.json',json.dumps(report,indent=2,ensure_ascii=False)+'\n')
lines=['# P1334 — recibo B dos resultados CLI públicos','',f"Veredito: **PASS**, limitado ao corpus congelado. Auditoria UTC `{report['utc']}`.",'',f"Manifesto SHA-256 `{freeze['manifest_sha256']}`. Freeze R1 SHA-256 `{report['freeze']['sha256']}`. Auditoria reproduzível: `python3 00_nucleo/diagnosticos/p1334-ab-audit-public.py` (evidências imutáveis; execução subsequente verifica e recusa sobrescrever). JSON detalhado: `p1334-ab-public-audit.json`, SHA-256 `{digest(D/'p1334-ab-public-audit.json')}`.",'',f"As {report['unique_cells']} células distintas passaram nas três ordens: {all_count} comparações integrais de exit/stdout/stderr, zero diferenças e zero Unknown. Suite principal: 816 células por rodada. Suplemento real cross-source: 12 por rodada. Resultados normal/repetido/invertido coincidem por caso e perfil, com reversão efetiva da ordem de expressões.",'',f"As 712 células históricas estão presentes: {len(migrations)} migram somente pelos guards autorizados no ledger congelado, e {712-len(migrations)} preservam exatamente as expectativas anteriores. Conferidos {report['protected_hashes_verified']} hashes declarados de inputs/artefatos e as três normas L0 sem a linha Hash do Código. As observações BASE/VANILLA nos seis recibos são cópias integrais dos caches congelados, incluindo argv e UTC originais.",'',f"Os seis recibos identificam o candidato `{path}`, SHA-256 `{pin}`. Cada comando candidato registra argv e UTC posterior ao freeze. O estado inicial de referência é HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado, com diff/stat e inventário preservados no baseline público pinado; esta auditoria não infere a identidade do candidato a partir desse HEAD.",'','| Recibo público | Células | SHA-256 |','|---|---:|---|']
for suite in report['suites']:
 for r in suite['runs']:lines.append(f"| {r['file']} | {r['cells']} | `{r['sha256']}` |")
lines+=['','Regime A/B sem atestação técnica de isolamento. Não há selo de refinamento nem alegação de paridade geral. B leu somente os outputs públicos autorizados e inputs/freezes, sem ler runtime produtivo ou recibos privados RED/GREEN. A identidade do executável é a declarada consistentemente nos recibos; B não executou nem inspecionou seu conteúdo. Este recibo não certifica gates nativos, build, workspace ou lint. Fixtures Some históricos incoerentes continuam classificados apenas como robustez local fora do domínio causal.','']
save('p1334-ab-receipt.md','\n'.join(lines))
print(json.dumps({'verdict':'PASS','unique_cells':report['unique_cells'],'comparisons':all_count,'receipt_sha256':digest(D/'p1334-ab-receipt.md'),'audit_sha256':digest(D/'p1334-ab-public-audit.json')}))
