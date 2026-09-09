#!/usr/bin/env python3
"""R2 successor: original R1 cases and expectations remain immutable."""
import argparse, hashlib, json, pathlib, runpy
HERE=pathlib.Path(__file__).resolve().parent
R1=runpy.run_path(str(HERE/'p1321-ab2-runner.py'),run_name='ab2_r1_library')
BASE='/tmp/p1321-r1-preserved.Tdbgy8/typst'
VANILLA='/usr/local/bin/typst'
PREFIX='p1321-ab2-r2-'
EXCLUDED={'r2_math_csv_direct':'Exploratory syntax is not a bilateral CSV call: vanilla math namespace does not resolve bare csv. Alias and With math controls cover the required generic CSV routes.'}
def read(name):return json.loads((HERE/name).read_text())
def save(name,obj):
    p=HERE/(PREFIX+name+'.json');p.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n');return R1['sha'](p)
def controls():
    rows=[]
    def add(name,expr,policy='baseline'):rows.append({'id':'r2_'+name,'expr':expr,'policy':policy})
    add('user_csv_missing','{ let csv(x) = x; csv() }')
    add('user_csv_alias','{ let csv(x) = x; let alias = csv; alias() }')
    add('user_csv_with','{ let csv(x) = x; csv.with()() }')
    add('user_csv_value','{ let csv(..a) = a.len(); csv(1, zeta: 2) }')
    add('read_missing','read()')
    add('json_missing','json()')
    add('cbor_missing','cbor()')
    add('default_float','float.is-nan()')
    add('encode_json_missing','json.encode()')
    add('encode_toml_missing','toml.encode()')
    add('encode_yaml_missing','yaml.encode()')
    add('encode_with','json.encode.with().with()()')
    add('panic_direct','panic("control-r2")')
    add('panic_with','panic.with("control-r2").with()()')
    add('user_panic','{ let panic(x) = x; panic() }')
    add('csv_with_chain','csv.with(delimiter: ";").with(row-type: dictionary)()','vanilla')
    add('csv_with_source_origin','csv.with(source: 42).with(delimiter: ";")()','vanilla')
    add('math_csv_direct','$csv()$','vanilla')
    add('math_csv_alias','{ let reader = csv; $reader()$ }','vanilla')
    add('math_csv_with','{ let reader = csv.with(delimiter: ";").with(); $reader()$ }','vanilla')
    return rows
def inputs():
    old=read('p1321-ab2-freeze.json'); paths={**old['inputs']}
    for p in ['p1321-ab2-freeze.json','p1321-ab2-candidate.json','p1321-ab2-r2-cases.json','p1321-ab2-r2-measurement.json','p1321-ab2-r2-expectations.json','p1321-ab2-r2-runner.py']:
        paths[str(HERE/p)]=R1['sha'](HERE/p)
    return paths
def verify(freeze):
    for p,d in freeze['inputs'].items():
        if R1['sha'](p)!=d:raise RuntimeError('Frozen input changed: '+p)
    for p,d in freeze['norms'].items():
        if R1['norm_sha'](p)!=d['without_code_hash']:raise RuntimeError('Norm changed: '+p)
def compare(runs,expected):
    table={(x['id'],x['profile']):x['expected'] for x in expected}
    return [{'id':x['id'],'profile':x['profile'],'actual':R1['obs'](x),'expected':table[(x['id'],x['profile'])]} for x in runs if R1['obs'](x)!=table[(x['id'],x['profile'])]]
def main():
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['measure','freeze','focal','full']);p.add_argument('--binary');a=p.parse_args()
    if a.mode=='measure':
        cases=controls();save('cases',cases)
        r={'state':R1['state'](),'binaries':{b:R1['sha'](b) for b in [BASE,VANILLA]},'runs':{label:R1['run_all'](binary,cases) for label,binary in [('baseline',BASE),('vanilla',VANILLA)]}}
        print(save('measurement',r))
        for c in cases:
            if c['id'] in EXCLUDED:continue
            print(c['id'],[(label,next(x for x in run if x['id']==c['id'] and x['profile']=='default')['stderr'].splitlines()[:1]) for label,run in r['runs'].items()])
    elif a.mode=='freeze':
        r=read(PREFIX+'measurement.json');cases=read(PREFIX+'cases.json'); expected=[]
        for c in cases:
            if c['id'] in EXCLUDED:continue
            for profile in R1['PROFILES']:
                ref=next(x for x in r['runs'][c['policy']] if x['id']==c['id'] and x['profile']==profile)
                expected.append({'id':c['id'],'profile':profile,'policy':c['policy'],'expected':R1['obs'](ref)})
        save('expectations',expected)
        norms={str(R1['ROOT']/p):{'raw':R1['sha'](R1['ROOT']/p),'without_code_hash':R1['norm_sha'](R1['ROOT']/p)} for p in ['00_nucleo/prompts/compiler/stdlib/loading.md','00_nucleo/prompts/compiler/eval/call_dispatch.md']}
        print(save('freeze',{'state':R1['state'](),'regime':'A/B executado sem atestacao de isolamento','predecessor':R1['sha'](HERE/'p1321-ab2-freeze.json'),'original_expected':R1['sha'](HERE/'p1321-ab2-expectations.json'),'original_failure':R1['sha'](HERE/'p1321-ab2-candidate.json'),'inputs':inputs(),'norms':norms,'binaries':r['binaries'],'exploratory_exclusions':EXCLUDED,'budget':'new controls only before freeze; original 8 failed cases plus original boundaries and new controls focal; only after focal green one full normal/repeat/reverse','unknown':'required Unknown blocks; never accepted','scope':'CSV call-span transport by native identity and recursive With; original diagnostics unchanged'}))
    else:
        freeze=read(PREFIX+'freeze.json');verify(freeze)
        old=read('p1321-ab2-cases.json');new=[c for c in read(PREFIX+'cases.json') if c['id'] not in EXCLUDED];expected=read('p1321-ab2-expectations.json')+read(PREFIX+'expectations.json')
        r={'state':R1['state'](),'freeze':R1['sha'](HERE/(PREFIX+'freeze.json')),'binary':a.binary,'sha256':R1['sha'](a.binary),'runs':{},'failures':{}}
        if a.mode=='focal':
            failures={x['id'] for x in read('p1321-ab2-candidate.json')['failures']['normal']}
            boundary={'missing_source','cast_int','remaining_unknown','with_delimiter_firstbad','protected_read','protected_json','preserve_unequal','normative_symbol_source'}
            groups=[('focal',[x for x in old if x['id'] in failures|boundary]+new)]
        else:
            focal=read(PREFIX+'focal.json')
            if focal['sha256']!=r['sha256'] or any(focal['failures'].values()):raise RuntimeError('Focal must pass on this candidate before full run')
            groups=[('normal',old+new),('repeat',old+new),('reverse',list(reversed(old+new)))]
        for label,cases in groups:
            runs=R1['run_all'](a.binary,cases);r['runs'][label]=runs;r['failures'][label]=compare(runs,expected)
        verify(freeze);print(save(a.mode,r));print(json.dumps({label:len(fail) for label,fail in r['failures'].items()}))
if __name__=='__main__':main()
