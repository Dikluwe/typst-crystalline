#!/usr/bin/env python3
"""Independent black-box CSV validation corpus; never opens crystalline sources."""
import argparse, concurrent.futures, datetime, hashlib, json, pathlib, re, subprocess, time
ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
OUT = ROOT / '00_nucleo/diagnosticos'
BASE = '/tmp/p1319-target.VqXtmj/release/typst'
VANILLA = '/usr/local/bin/typst'
PROFILES = {'default': [], 'html': ['--features','html'], 'a11y': ['--features','a11y-extras'], 'html-a11y': ['--features','html,a11y-extras']}
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def norm_sha(p):
    raw=pathlib.Path(p).read_bytes()
    normalized,n=re.subn(rb'^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n',b'',raw,flags=re.M)
    if n!=1: raise RuntimeError('Expected exactly one canonical code hash')
    return hashlib.sha256(normalized).hexdigest()
def command(args): return subprocess.run(args,cwd=ROOT,text=True,capture_output=True)
def save(name, obj):
    p = OUT / ('p1321-ab2-' + name + '.json')
    p.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n')
    return sha(p)
def state():
    return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':command(['git','rev-parse','HEAD']).stdout.strip(),'diff_stat':command(['git','diff','HEAD','--stat']).stdout,'status':command(['git','status','--short']).stdout}
def corpus():
    c=[]
    def add(name,expr,policy='vanilla'): c.append({'id':name,'expr':expr,'policy':policy})
    good='bytes("a,b\\n1,2")'; bad='bytes("a,b\\n1")'; absent='"p1321-ab2-nonexistent.csv"'
    for name,args in [('none',''),('delimiter','delimiter: "ab"'),('unknown','zeta: 1'),('source','source: 42'),('source_unknown','zeta: 1, source: 42'),('unknown_source','source: 42, zeta: 1'),('source_options','delimiter: "ab", source: 42')]: add('missing_'+name,'csv('+args+')')
    for name,v in [('int','42'),('bool','true'),('none','none'),('dict','(a: 1)'),('array','(1, 2)'),('type','array'),('float','1.5'),('content','[x]')]:
        add('cast_'+name,'csv('+v+')')
        if name in ['int','bool','dict','type']: add('cast_unknown_'+name,'csv(zeta: 1, '+v+', delimiter: "ab")')
    for name,opt in [('empty','delimiter: ""'),('long','delimiter: "ab"'),('unicode','delimiter: "é"'),('int','delimiter: 3'),('row_str','row-type: "array"'),('row_type','row-type: str'),('row_int','row-type: 3')]:
        add('option_'+name,'csv('+good+', '+opt+')')
        add('option_unknown_'+name,'csv(zeta: 1, '+good+', '+opt+')')
    for name,tail in [('extra','17'),('unknown','zeta: 1'),('z_a','zeta: 1, alpha: 2'),('a_z','alpha: 2, zeta: 1'),('extra_unknown','17, zeta: 1'),('unknown_extra','zeta: 1, 17'),('source_named','source: 17')]:
        add('remaining_'+name,'csv('+good+', '+tail+')')
    for name,src in [('missingfile',absent),('unequal',bad),('utf8','bytes((255,))')]:
        add('precedence_'+name+'_extra','csv('+src+', 17)')
        add('precedence_'+name+'_unknown','csv('+src+', zeta: 1)')
        add('precedence_'+name+'_options','csv('+src+', row-type: str, delimiter: "ab", zeta: 1)')
    for name,a,b in [('delimiter_firstbad','delimiter: "ab"','delimiter: ";"'),('delimiter_lastbad','delimiter: ";"','delimiter: "ab"'),('delimiter_good','delimiter: ";"','delimiter: ","'),('row_firstbad','row-type: str','row-type: array'),('row_lastbad','row-type: array','row-type: str'),('row_good','row-type: array','row-type: dictionary'),('cross','row-type: str','delimiter: "ab"')]:
        add('with_'+name,'{ let f = csv.with('+a+'); f('+good+', '+b+') }')
    add('syntax_duplicate','csv('+good+', delimiter: ";", delimiter: ",")','baseline')
    routes = {
       'alias':lambda args:'{ let f = csv; f('+args+') }',
       'args':lambda args:'{ let a = arguments('+args+'); csv(..a) }',
       'sink':lambda args:'{ let f(..a) = csv(..a); f('+args+') }',
       'with':lambda args:'{ let f = csv.with('+args+'); f() }',
       'map':lambda args:'{ let a = arguments('+args+'); csv(..a.map(x => x)) }',
    }
    for route,wrap in routes.items():
        for name,args in [('missing','zeta: 1'),('source','source: 42'),('cast','zeta: 1, 42'),('excess',good+', 17, zeta: 1'),('unknown',good+', zeta: 1, alpha: 2')]: add(route+'_'+name,wrap(args))
    add('spread_dict_unknown','csv('+good+', ..(zeta: 1, alpha: 2))')
    add('spread_array_excess','csv(..('+good+', 17))')
    add('args_dup_delimiter','{ let a = arguments(delimiter: "ab"); csv('+good+', ..a, delimiter: ",", zeta: 1) }')
    add('sink_dup_row','{ let f(..a) = csv('+good+', ..a, row-type: array); f(row-type: str, zeta: 1) }')
    for name,args in [('array',good),('dict',good+', row-type: dictionary'),('semicolon','bytes("a;b\\n1;2"), delimiter: ";"'),('empty','bytes(())'),('quoted','bytes("\\\"a,b\\\",c")')]: add('valid_'+name,'csv('+args+')','baseline')
    for name,args in [('unequal',bad),('utf8','bytes((255,))'),('multiline','bytes("\\\"a\\nb\\\",c\\n1")'),('crlf','bytes("a,b\\r\\n1")'),('path_good','"00_nucleo/diagnosticos/p1320-fixtures/good.csv"'),('path_bad','"00_nucleo/diagnosticos/p1320-fixtures/unequal.csv"'),('io',absent)]: add('preserve_'+name,'csv('+args+')','baseline')
    for name,expr in [('source','csv(sym.alpha)'),('delimiter','csv('+good+', delimiter: sym.alpha)'),('read','read(42)'),('json','json(42)'),('yaml','yaml(bytes("a: 1"))'),('toml','toml(bytes("a = 1"))'),('xml','xml(bytes("<a/>"))'),('cbor','cbor(cbor.encode((a: 1)))'),('encode','json.encode((a: 1), pretty: false)'),('missingfield','csv.encode(1)')]: add('protected_'+name,expr,'baseline')
    add('normative_symbol_source','csv(sym.alpha, zeta: 1)','symbol-source')
    add('normative_symbol_delimiter','csv('+good+', delimiter: sym.alpha, zeta: 1)','symbol-delimiter')
    return c
def run_one(binary,case,profile):
    argv=[binary,'--color=never','eval',*PROFILES[profile],case['expr']]
    started=time.monotonic()
    try:
        r=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=25)
        return {'id':case['id'],'profile':profile,'argv':argv,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'seconds':time.monotonic()-started}
    except subprocess.TimeoutExpired: return {'id':case['id'],'profile':profile,'argv':argv,'unknown':'timeout'}
def run_all(binary,cases):
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as e:
        fs=[e.submit(run_one,binary,c,p) for c in cases for p in PROFILES]
        return [f.result() for f in fs]
def obs(r): return {k:r.get(k) for k in ['exit','stdout','stderr','unknown']}
def symbolic(case):
    expr=case['expr']; col=expr.index('sym.alpha'); msg='expected path, string, or bytes, found symbol' if case['policy']=='symbol-source' else 'expected string, found symbol'
    return {'exit':1,'stdout':'','stderr':f'error: {msg}\n  ┌─ <input-expression>:1:{col}\n  │\n1 │ {expr}\n  │ '+ ' '*col+'^'*9+'\n\n','unknown':None}
def main():
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['baseline','freeze','candidate']);p.add_argument('--binary'); a=p.parse_args()
    if a.mode=='baseline':
        cases=corpus(); save('cases',cases)
        r={'state':state(),'binaries':{b:sha(b) for b in [BASE,VANILLA]},'runs':{}}
        for label,binary in [('baseline',BASE),('vanilla',VANILLA)]: r['runs'][label]=run_all(binary,cases)
        save('baseline',r)
        baseline=r['runs']['baseline']; vanilla=r['runs']['vanilla']
        print(json.dumps({'cases':len(cases),'processes':len(baseline)+len(vanilla),'different':[(b['id'],b['profile']) for b,v in zip(baseline,vanilla) if obs(b)!=obs(v)]}))
    elif a.mode=='freeze':
        cases=json.loads((OUT/'p1321-ab2-cases.json').read_text());r=json.loads((OUT/'p1321-ab2-baseline.json').read_text());expect=[]
        for c in cases:
            for profile in PROFILES:
                label='baseline' if c['policy']=='baseline' else 'vanilla'
                ref=next(x for x in r['runs'][label] if x['id']==c['id'] and x['profile']==profile)
                expect.append({'id':c['id'],'profile':profile,'expected':symbolic(c) if c['policy'].startswith('symbol-') else obs(ref),'policy':c['policy']})
        save('expectations',expect)
        paths=[OUT/'p1321-ab2-cases.json',OUT/'p1321-ab2-baseline.json',OUT/'p1321-ab2-expectations.json',pathlib.Path(__file__)]
        paths.extend(ROOT/p for p in ['lab/typst-original/crates/typst-library/src/loading/csv.rs','lab/typst-original/crates/typst-library/src/foundations/args.rs','lab/typst-original/crates/typst-macros/src/func.rs','00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md','00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md','00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md','00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md'])
        paths.extend((ROOT/'00_nucleo/diagnosticos/p1320-fixtures').rglob('*'))
        paths.extend(pathlib.Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada')/p for p in ['SKILL.md','references/papeis-e-capacidades.md','references/artefatos-e-gates.md'])
        paths=[x for x in paths if x.is_file()]
        norm=ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md'
        print(save('freeze',{'state':state(),'regime':'A/B executado sem atestacao de isolamento','inputs':{str(x):sha(x) for x in paths},'norm':{'path':str(norm),'raw':sha(norm),'without_code_hash':norm_sha(norm),'exclusion':'Only the single canonical Hash do Código line'},'binaries':r['binaries'],'budget':'one full baseline; at most two focal adjustments with gain; one candidate normal/repeat/reverse','unknown':'never success; timeout or unobservable required case blocks','capabilities':'Only L0, listed ADRs, skill refs, vanilla csv/args/macrosfunc, textual p1320 fixtures, binaries, own artifacts; no crystalline source, tests, real diff or previous diagnostics read.'}))
    else:
        freeze=json.loads((OUT/'p1321-ab2-freeze.json').read_text())
        for path,digest in freeze['inputs'].items():
            if sha(path)!=digest: raise RuntimeError('Protected input changed: '+path)
        if norm_sha(freeze['norm']['path'])!=freeze['norm']['without_code_hash']: raise RuntimeError('Norm changed')
        cases=json.loads((OUT/'p1321-ab2-cases.json').read_text()); expected=json.loads((OUT/'p1321-ab2-expectations.json').read_text())
        table={(x['id'],x['profile']):x for x in expected}; report={'state':state(),'candidate':a.binary,'sha256':sha(a.binary),'runs':{},'failures':{}}
        for order,rows in [('normal',cases),('repeat',cases),('reverse',list(reversed(cases)))]:
            runs=run_all(a.binary,rows);report['runs'][order]=runs
            report['failures'][order]=[{'id':x['id'],'profile':x['profile'],'actual':obs(x),'expected':table[(x['id'],x['profile'])]['expected']} for x in runs if obs(x)!=table[(x['id'],x['profile'])]['expected']]
        save('candidate',report);print(json.dumps({k:len(v) for k,v in report['failures'].items()}))
if __name__=='__main__':main()
