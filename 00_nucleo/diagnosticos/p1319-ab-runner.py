"""Independent P1319 A/B. Only public executable observations and frozen L0."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re

ROOT=Path('/repos/Antigravity/typst-crystalline')
D=ROOT/'00_nucleo/diagnosticos'
FIX=Path('/tmp/p1319-ab-fixtures')
spec=importlib.util.spec_from_file_location('historical_runner',D/'p1318-ab-runner.py')
old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
old.FIX=FIX;old.CASES=D/'p1319-ab-cases.json'
old.BASE='/tmp/p1318-target.eAgQwp/release/typst'
old.BASE_SHA='0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c'
old.__file__=__file__
read,save,sha,obs,state,normative=old.read,old.save,old.sha,old.obs,old.state,old.normative
CASES=old.CASES
PROFILES=old.PROFILES

def prepare(a):
    assert normative()=='da59f9964566dc99d3341145fe8200df15ff8df172158c79d91542d47015e823'
    prior=read(D/'p1318-ab-cases.json')['cases'];cases=[]
    for c in prior:
        delta=(c['id'].startswith('history-new-file-') or c['id'].startswith('control-file-invalid-after-earlier-ls-'))
        cases.append(dict(id='history-'+c['id'],prior_id=c['id'],expr=c['expr'],cwd=c['cwd'],kind='file' if delta else 'literal',historical=True))
    raws={
        'header':b'\xff,b','data-lf':b'a,b\n\xff,2','data-crlf':b'a,b\r\n\xff,2',
        'data-cr':b'a,b\r\xff,2','no-lf-prefix':b'\r\r\xff,a',
        'bom':b'\xef\xbb\xbf\xff,b','empty-lines':b'\n\na,b\n\xff,2',
        'unequal-lf':b'a,b\n1\n\xff,2','unequal-crlf':b'a,b\r\n1\n\xff,2',
        'unequal-ls':'α\u2028😀,b\r\n1\n'.encode()+b'\xff,2',
        'unequal-cr':'α,😀\r1\r'.encode()+b'\xff,2',
        'quoted-lf':'"α\n😀",b\n'.encode()+b'\xff,2',
        'late-unicode':'α,😀\nβ,δ\n'.encode()+b'\xff,2',
        'unequal-same-record':b'a,b\n\xff',
        'valid':'α,😀\r\nβ,δ'.encode(),'text-error':b'a,b\r\n1',
    }
    FIX.mkdir(exist_ok=True);(FIX/'nested').mkdir(exist_ok=True);(FIX/'sub').mkdir(exist_ok=True)
    fixtures={}
    def fixture(relative,raw):
        p=FIX/relative
        assert not p.exists() or p.read_bytes()==raw
        if not p.exists():p.write_bytes(raw)
        fixtures[str(p)]=dict(bytes=list(raw),sha256=sha(p))
    for name,raw in raws.items():fixture(name+'.csv',raw)
    fixture('sub/data.csv',b'a,b\n\xff,2')
    fixture('data.csv',b'a,b\n1\n\xff,2')
    save(FIX/'sub/paths.typ','#let rooted = path("data.csv")\n#let string = "data.csv"\n')
    fixtures[str(FIX/'sub/paths.typ')]=dict(sha256=sha(FIX/'sub/paths.typ'))
    def add(id,expr,kind='file',**kw):cases.append(dict(id=id,expr=expr,cwd=str(FIX),kind=kind,historical=False,**kw))
    for mode in ('array','dictionary'):
        for name in raws:
            for route in ('str','path'):
                arg=json.dumps(name+'.csv');arg=f'path({arg})' if route=='path' else arg
                add('new-'+name+'-'+route+'-'+mode,f'csv({arg}, row-type: {mode})','literal' if name in ('valid','text-error') else 'file',check='success' if name=='valid' else None)
        for route in ('str','path'):
            arg='"sub/data.csv"';arg=f'path({arg})' if route=='path' else arg
            add('new-subdirectory-'+route+'-'+mode,f'csv({arg}, row-type: {mode})')
            arg='"nested/../unequal-ls.csv"';arg=f'path({arg})' if route=='path' else arg
            add('new-normalized-'+route+'-'+mode,f'csv({arg}, row-type: {mode})')
            arg='"/data-lf.csv"';arg=f'path({arg})' if route=='path' else arg
            add('new-absolute-'+route+'-'+mode,f'csv({arg}, row-type: {mode})')
        for route in ('str','path'):
            arg='"data-crlf.csv"';arg=f'path({arg})' if route=='path' else arg;opts='row-type: '+mode
            routes={
                'with':f'{{let f=csv.with({arg}, {opts}); f()}}',
                'args':f'{{let a=arguments({opts}, {arg}); csv(..a)}}',
                'named-first':f'csv({opts}, {arg})',
                'array-spread':f'csv(..({arg},), {opts})',
                'sink':f'{{let f(..a)=csv(..a); f({arg}, {opts})}}',
                'alias':f'{{let f=csv; f({arg}, {opts})}}',
            }
            for transport,expr in routes.items():add('new-'+transport+'-'+route+'-'+mode,expr)
            if route=='path':
                detached=f'{{let p={arg}; csv(..arguments(0).map((..a) => p), {opts})}}'
            else:detached=f'csv(..arguments(0).map((..a) => {arg}), {opts})'
            add('new-detached-'+route+'-'+mode,detached,normative_only='detached',position_probe=f'csv({arg}, {opts})')
            probe=f'csv({arg}, {opts})';expr=f'csv({arg}, 42, {opts})'
            add('new-excess-'+route+'-'+mode,expr,normative_only='excess',position_probe=probe)
        add('new-cross-file-rooted-'+mode,f'{{import "sub/paths.typ": rooted; csv(rooted, row-type: {mode})}}','literal',boundary='Measured eval import sandbox failure before CSV; no parsing/base-identity coverage claim.')
        # The existing string resolution debt is kept separate: root/data.csv wins
        # in this consumer, while the rooted Path above resolves sub/data.csv.
        add('new-cross-file-string-'+mode,f'{{import "sub/paths.typ": string; csv(string, row-type: {mode})}}','literal',boundary='Measured eval import sandbox failure before CSV; no parsing/base-identity coverage claim.')
    controls={
        'option-before-io':'csv("missing.csv", delimiter: "xx")',
        'row-before-io':'csv("missing.csv", row-type: str)',
        'invalid-delimiter':'csv("header.csv", delimiter: "xx")',
        'invalid-row':'csv(path("header.csv"), row-type: str)',
        'unknown':'csv("header.csv", nope: true)',
        'cast-before-options':'csv(42, delimiter: "xx")',
        'missing':'csv()','io':'csv("missing.csv")',
        'escape':'csv("../escape.csv")','backslash':'csv("nested\\\\file.csv")',
        'read':'read("header.csv")','read-binary':'read("header.csv", encoding: none)',
        'json':'json("header.csv")','yaml':'yaml("header.csv")','toml':'toml("header.csv")',
        'xml':'xml("header.csv")','cbor':'cbor("header.csv")','namespace':'csv.encode',
        'bytes':'csv(bytes((97,44,98,13,10,255,44,50)))',
        'valid-excess':'csv("valid.csv", 42)',
    }
    for name,expr in controls.items():add('control-'+name,expr,'literal')
    save(CASES,dict(schema='p1319-ab-cases-v1',cases=cases,fixtures=fixtures,policy='Exact P1318 replays before eight declared historical Path/Str invalid-buffer deltas. Native invalid-buffer diagnostics from vanilla integral; explicit normative detached/excess branches. All other observations literal baseline.'))
    print('cases',len(cases),'changes',sum(c['kind']=='file' for c in cases),flush=True)

def run(a):old.run(a)

def freeze(a):
    m=read(a.measurement);cases=read(CASES)['cases'];prior={(r['id'],r['profile']):r for r in read(D/'p1318-ab-candidate-runs.json')['rows'] if r['order']=='normal'}
    assert m['runner_sha256']==sha(D/'p1319-ab-runner-r0.py') and m['cases_sha256']==sha(D/'p1319-ab-cases-r0.json')
    assert m['calibration_revision']['runner_sha256']==sha(__file__) and m['calibration_revision']['cases_sha256']==sha(CASES) and m['l0_normative_sha256']==normative()
    by={(r['id'],r['profile'],r['product']):r for r in m['rows']};assert len(by)==len(m['rows'])
    expected=[]
    for c in cases:
        for profile in PROFILES:
            row=by[c['id'],profile,'baseline'];e=obs(row);full=False;position=None
            assert e['exit'] in (0,1),('Unknown process',c['id'])
            if c['historical']:
                p=prior[c['prior_id'],profile]
                assert row['cwd']==p['cwd'] and row['argv'][2]==p['argv'][2] and e==obs(p),('Unknown replay drift',c['id'])
            if c.get('check')=='success':assert e['exit']==0,('Unknown success',c['id'],e)
            if c['kind']=='file':
                first,sep,rest=e['stderr'].partition('\n')
                assert e['exit']==1 and not e['stdout'] and first.startswith('error: failed to parse CSV (') and first.endswith(')'),('Unknown parsing',c['id'],e)
                assert len(re.findall(r'(?m)^error: ',e['stderr']))==1
                v=obs(by[c['id'],profile,'vanilla']);pv=obs(by[c['id'],profile,'position-probe']) if c.get('normative_only') else v
                line=pv['stderr'].partition('\n')[0]
                match=re.fullmatch(re.escape(first[:-1])+r' in (.+):([1-9][0-9]*):([1-9][0-9]*)\)',line)
                assert match,('Unknown file oracle',c['id'],first,line)
                position=list(match.groups())
                if c.get('normative_only')=='detached':
                    assert v['stderr'].startswith('error: cannot access file system from here'),('Unknown detached boundary',c['id'],v)
                    assert '┌─' not in e['stderr'],('Unknown detached baseline',c['id'],e)
                    e['stderr']=line+sep+rest
                elif c.get('normative_only')=='excess':
                    assert v['stderr'].startswith('error: unexpected argument'),('Unknown excess',c['id'],v)
                    assert c['position_probe'] in pv['stderr']
                    e=dict(pv);e['stderr']=pv['stderr'].replace(c['position_probe'],c['expr'])
                else:e=v;full=True
            expected.append(dict(id=c['id'],profile=profile,kind=c['kind'],expr=c['expr'],cwd=c['cwd'],expected=e,position=position,full_vanilla_parity=full,normative_only=c.get('normative_only',False),baseline_red=e!=obs(row)))
    historical=read(D/'p1318-ab-freeze.json')
    files=[Path(__file__),CASES,Path(a.measurement),D/'p1319-ab-runner-r0.py',D/'p1319-ab-cases-r0.json',D/'p1319-ab-baseline-runs.json',D/'p1319-ab-focal-runs.json',D/'p1319-ab-calibration.py',D/'p1318-ab-runner.py',D/'p1318-ab-cases.json',D/'p1318-ab-candidate-runs.json',D/'p1318-ab-freeze.json']
    files += [Path(p) for p in historical['inputs'] if p.startswith('/tmp/')]
    files += [Path(p) for p in read(CASES)['fixtures']]
    files += [ROOT/'lab/typst-original/crates/typst-library/src'/p for p in ('loading/csv.rs','loading/mod.rs','diag.rs','foundations/path.rs')]
    files += [ROOT/'00_nucleo/adr'/p for p in ('typst-adr-0107-paridade-linguagem-nao-mecanica.md','typst-adr-0108-disciplina-anti-deriva.md','typst-adr-0127-gate-l0-paragem-vs-fluxo.md','typst-adr-0129-nucleos-tekt-l0-compartilhado.md')]
    skill=Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada')
    files += [skill/p for p in ('SKILL.md','references/papeis-e-capacidades.md','references/artefatos-e-gates.md')]
    save(a.output,dict(schema='p1319-ab-freeze-v1',utc=state()['utc'],regime='A/B executado sem atestação de isolamento técnico',capabilities=dict(executor='/root/p1319_tests',reads=['L0 inteiro','skill, refs, ADRs','vanilla loading/csv.rs, loading/mod.rs, diag.rs, foundations/path.rs','p1318-ab-* and historical fixtures','public binary execution','git HEAD/status/diff STAT'],writes=['00_nucleo/diagnosticos/p1319-ab-*','/tmp/p1319-ab-fixtures'],context='Task-scoped agent. No crystalline source, local tests, productive diff, or receipts embedding source read. Shared filesystem, technical isolation not attested.'),unknown_policy='Malformed/missing/duplicate, timeout/crash, identity ambiguity, unsupported construction or replay/input drift blocks. Never implicit PASS.',budget='One baseline complete, focal corrections only; two consecutive revisions without gain on same cause stop. Candidate normal/repeat/reverse once after release.',l0=dict(path=str(old.L0),raw_sha256=sha(old.L0),normative_sha256=normative(),exclusion='Only one canonical Hash do Código line'),inputs={str(p):sha(p) for p in files},binaries=m['binaries'],provenance=m['before'],expected=expected,calibration=dict(revisions=0,processes=m['processes'],wall_seconds=m['wall_seconds']),limitations=['CLI baseline eval does not expose package-path; Package identity requires local tests/source audit by another authority.','CLI cannot attest pure decode_csv, synthetic Rust Args without occurrences, impossible offsets/fallback/overflow, World call count or single parser invocation.','No refinement seal, mutation score, general parity or technical isolation attestation.']))
    print('freeze',sha(a.output),'observations',len(expected),'RED',sum(e['baseline_red'] for e in expected),flush=True)

def compare(a):old.compare(a)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['prepare','run','freeze','compare'])
    for name in ('binary','binary-sha256','ids','measurement','freeze','output'):p.add_argument('--'+name)
    p.add_argument('--orders',default='normal');args=p.parse_args();globals()[args.mode](args)
