#!/usr/bin/env python3
"""Pre-C coverage supplement: actual source-file import and retained origins."""
import sys,pathlib,importlib.util,json,datetime,concurrent.futures,argparse
sys.dont_write_bytecode=True
D=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('p1334_frozen_cli',D/'p1334-ab-cli.py'); frozen=importlib.util.module_from_spec(spec);spec.loader.exec_module(frozen)
BASE=frozen.BASE; VANILLA=frozen.VANILLA; digest=frozen.digest; command=frozen.command; observed=frozen.observed; save=frozen.save
FIXTURE=D/'p1334-ab-fixtures/origins.typ'
PREFIX='{import "00_nucleo/diagnosticos/p1334-ab-fixtures/origins.typ": '
CASES=[('cross-source-first-named-value',PREFIX+'named; named()}'),('cross-source-arguments-first-extra',PREFIX+'supplied; calc.abs(..supplied)}'),('cross-source-with-first-named-surplus',PREFIX+'extra; extra()}')]
CACHE=D/'p1334-ab-cross-baseline.json'; EXPECTED=D/'p1334-ab-cross-expected.json'
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--candidate');ap.add_argument('--freeze-from');ap.add_argument('--output',required=True);ap.add_argument('--order',choices=['normal','reverse'],default='normal');a=ap.parse_args()
 assert digest(D/'p1334-ab-cli.py')=='3afb07be4d498d46d9aee2e6c7ce573a3bb49075144f1f3b3624bb7dc9bdda5b'
 assert frozen.history.normhash()==frozen.NORM
 for role,path in [('BASE',BASE),('VANILLA',VANILLA)]:assert digest(path)==frozen.PINS[role]
 if a.freeze_from:
  assert not a.candidate
  baseline=json.loads(pathlib.Path(a.freeze_from).read_text());expected=[]
  for row in baseline['cases']:
   oracle=observed(row['results']['VANILLA'])
   assert oracle['exit']==1
   assert 'origins.typ' in oracle['stderr'] and 'while calling `abs`' in oracle['stderr'],row
   if row['case']=='cross-source-first-named-value':assert 'the argument `value` is positional' in oracle['stderr'] and 'try removing `value:`' in oracle['stderr']
   elif row['case']=='cross-source-arguments-first-extra':assert 'error: unexpected argument\n' in oracle['stderr']
   else:assert 'error: unexpected argument: bad\n' in oracle['stderr']
   expected.append({'case':row['case'],'profile':row['profile'],'classification':'signature-parity-cross-source','expected':oracle})
  save(a.output,{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reason':'Pre-C L0 coverage supplement, no candidate feedback; actual imported Sources. Existing 816 cells remain frozen.','manifest_sha256':digest(frozen.MANIFEST),'runner_sha256':digest(pathlib.Path(__file__)),'fixture_sha256':digest(FIXTURE),'baseline_sha256':digest(pathlib.Path(a.freeze_from)),'expectations':expected});return
 expected=json.loads(EXPECTED.read_text()) if a.candidate else None
 cache=json.loads(CACHE.read_text()) if a.candidate else None
 if a.candidate:
  assert digest(CACHE)==expected['baseline_sha256'];assert digest(FIXTURE)==expected['fixture_sha256'];assert digest(pathlib.Path(__file__))==expected['runner_sha256']
 ex={(r['case'],r['profile']):r for r in expected['expectations']} if expected else {}
 refs={(r['case'],r['profile']):r for r in cache['cases']} if cache else {}
 cases=CASES if a.order=='normal' else list(reversed(CASES))
 def measure(cell):
  n,e,p,flags=cell
  results=dict(refs[(n,p)]['results']) if a.candidate else {role:command([path,'--color','never','eval',*flags,e]) for role,path in [('BASE',BASE),('VANILLA',VANILLA)]}
  row={'case':n,'expression':e,'profile':p,'results':results}
  if a.candidate:
   results['CANDIDATE']=command([a.candidate,'--color','never','eval',*flags,e]);row.update(expected=ex[(n,p)]['expected'],candidate_matches_frozen_policy=observed(results['CANDIDATE'])==ex[(n,p)]['expected'])
  return row
 with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:rows=list(pool.map(measure,[(n,e,p,f) for n,e in cases for p,f in frozen.PROFILES]))
 receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'manifest_sha256':digest(frozen.MANIFEST),'public_baseline_sha256':digest(D/'p1334-baseline-public.json'),'runner_sha256':digest(pathlib.Path(__file__)),'fixture':{'path':str(FIXTURE),'sha256':digest(FIXTURE)},'binaries':{k:{'path':p,'sha256':digest(pathlib.Path(p))} for k,p in [('BASE',BASE),('VANILLA',VANILLA)]},'order':a.order,'reference_mode':'cached' if cache else 'fresh','cases':rows}
 if a.candidate:receipt.update(candidate_binary={'path':a.candidate,'sha256':digest(pathlib.Path(a.candidate))},expected_sha256=digest(EXPECTED),cache_sha256=digest(CACHE))
 save(a.output,receipt)
 failures=[r['case']+'/'+r['profile'] for r in rows if r.get('candidate_matches_frozen_policy') is False]
 print(json.dumps({'observations':len(rows),'candidate_failures':failures}))
 if failures:raise SystemExit(1)
if __name__=='__main__':main()
