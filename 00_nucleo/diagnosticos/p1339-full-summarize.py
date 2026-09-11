"""Measurement catalog and vanilla A.2 classification, never a sealed contract."""
import collections
import importlib.util
import json
import subprocess
import sys
from pathlib import Path

sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('probe',Path(__file__).with_name('p1339-full-probe.py'))
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

def read(suffix):
    return json.loads((m.D/('p1339-full-'+suffix+'.json')).read_text())

SOURCES={
    'angle':dict(path='lab/typst-original/crates/typst-library/src/layout/angle.rs',lines=['12-17','38-50','63-66','140-152','195-207','247-253'],language='Angle receiver -> float degrees/radians. Public function names bind to conversion docs, not Rust constructors.',mechanics='Scalar wrapper, enum units, internal constructors named deg/rad, scope macro and storage units are not normative target architecture.'),
    'float':dict(path='lab/typst-original/crates/typst-library/src/foundations/float.rs',lines=['12-21','32-39','97-112','114-148','151-184'],language='inf/nan values; signed-zero signum; NaN signum; float accepts integer cast; byte lengths 4/8, IEEE binary32/64, little default, size 8 default; byte output itself is language.',mechanics='f64/f32 Rust conversion operations and #[scope] macro need not be copied. Bytes returned by API must be compared exactly.'),
    'function':dict(path='lab/typst-original/crates/typst-library/src/foundations/func.rs',lines=['310-315','340-352','366-374','395-409','412-450'],language='with pre-applies ordered arguments, where selects direct element identity plus named field values; native/closure/With rejection and unknown fields public diagnostics.',mechanics='FuncInner enum, Arc<With>, Args carrier internals, retained vector implementation and static scope organization are mechanisms, not required representation.'),
    'version':dict(path='lab/typst-original/crates/typst-library/src/foundations/version.rs',lines=['11-24','38-47','109-133'],language='Positive at indexes pad zero, negative indexes use explicitly provided length; out-of-bounds diagnostic includes original index and length.',mechanics='EcoVec storage and checked integer arithmetic implementation are not language requirements.')}

def main():
    stages=['final','boundaries','show-final']
    catalog=[]; comparisons=[]; instability=[]; differences_by_profile=[]; receipts={}; costs={}
    for stage in stages:
        manifest=read(stage+'-manifest')
        vr=read(stage+'-vanilla-runs'); cr=read(stage+'-crystalline-before-runs')
        receipts[stage]={suffix:m.sha(m.D/('p1339-full-'+stage+'-'+suffix+'.json')) for suffix in ['manifest','vanilla-runs','crystalline-before-runs']}
        maps={name:{(r['id'],r['order'],r['profile']):r for r in obj['rows']} for name,obj in [('vanilla',vr),('crystalline-before',cr)]}
        costs[stage]={name:dict(processes=len(obj['rows']),seconds=sum(r['seconds'] for r in obj['rows']),start=obj['start'],end=obj['end']) for name,obj in [('vanilla',vr),('crystalline-before',cr)]}
        for case in manifest['cases']:
            id=case['id']; route=case.get('route','function.where')
            v=maps['vanilla'][(id,'normal','default')]; c=maps['crystalline-before'][(id,'normal','default')]
            family='angle' if route.startswith('angle.') else 'float' if route.startswith('float.') else 'version' if route=='version.at' else 'function'
            catalog.append(dict(stage=stage,case=case,vanilla_reference={k:v[k] for k in ['exit','stdout','stderr','execution']},antecedent_observation={k:c[k] for k in ['exit','stdout','stderr','execution']},source=SOURCES[family],classification_scope='public observable measurements; no approved contract expectations or final verdict'))
            for order in ['normal','reverse']:
                for profile in m.PROFILES:
                    key=(id,order,profile); a,b=maps['vanilla'][key],maps['crystalline-before'][key]
                    infra=(a['execution']!='Observed' or b['execution']!='Observed' or a['exit'] not in [0,1] or b['exit'] not in [0,1])
                    if any(marker in a['stderr'] or marker in b['stderr'] for marker in ['unrecognized subcommand','unexpected argument \'--features\'','failed to load file','unknown feature']):
                        infra=True
                    equal=all(a[k]==b[k] for k in ['exit','stdout','stderr'])
                    classification='Unknown' if infra else 'Preserved' if equal else 'Violated'
                    comparisons.append(dict(stage=stage,id=id,route=route,order=order,profile=profile,classification=classification,reason='infrastructure/unsupported observation' if infra else 'identical public channels and exit' if equal else 'public channels or exit differ',same_stdout=a['stdout']==b['stdout'],same_stderr=a['stderr']==b['stderr'],same_exit=a['exit']==b['exit']))
            for name,rows in maps.items():
                for profile in m.PROFILES:
                    n,r=rows[(id,'normal',profile)],rows[(id,'reverse',profile)]
                    if any(n[k]!=r[k] for k in ['execution','exit','stdout','stderr']):
                        instability.append(dict(stage=stage,id=id,binary=name,profile=profile))
                default=rows[(id,'normal','default')]
                for profile in list(m.PROFILES)[1:]:
                    other=rows[(id,'normal',profile)]
                    if any(default[k]!=other[k] for k in ['execution','exit','stdout','stderr']):
                        differences_by_profile.append(dict(stage=stage,id=id,binary=name,profile=profile))
    for source in SOURCES.values():
        source['sha256']=m.sha(m.ROOT/source['path'])
    unknown_obligations=[dict(id='actual-angle-NaN-receiver',routes=['angle.deg','angle.rad'],classification='Unknown',mandatory=True,reason='No common NaN Angle was constructible by measured public expressions: vanilla normalizes all three constructions to 0deg; antecedent keeps NaN Angle. Measuring conversion of normalized zero is not measuring conversion of NaN Angle.',witnesses=['angle-construction-deg-nan-multiply','angle-construction-rad-nan-multiply','angle-construction-deg-zero-infinite','angle-construction-deg-inf-minus-inf'],refutation='Provide a public fixture that yields actual NaN Angle in both pinned binaries, established independently of deg/rad conversion, then measure it.'),dict(id='plugin-function-receiver',routes=['function.where','function.with'],classification='Unknown',mandatory=False,reason='Native non-element receivers and closures measured; no wasm plugin fixture supplied or built. Slash phrase plugin/native interpreted as native representative; plugin-specific public behavior is not claimed.',refutation='A pinned minimal plugin fixture with verified function identity and safe public invocation would allow this optional dimension to be measured.')]
    counts=dict(collections.Counter(r['classification'] for r in comparisons))
    coverage={route:dict(cases=sum(c['case'].get('route','function.where')==route for c in catalog),classifications=dict(collections.Counter(r['classification'] for r in comparisons if r['route']==route))) for route in sorted({r['route'] for r in comparisons})}
    m.save('catalog',dict(at=m.now(),provenance=m.provenance(),regime='executado sem atestação de isolamento',receipts=receipts,cases=catalog,unknown_obligations=unknown_obligations,source_classification=SOURCES))
    m.save('comparison',dict(at=m.now(),provenance=m.provenance(),catalog_sha256=m.sha(m.D/'p1339-full-catalog.json'),receipts=receipts,counts=counts,coverage=coverage,comparisons=comparisons,order_instability=instability,profile_differences=differences_by_profile,unknown_obligations=unknown_obligations,costs=costs,status='MEASUREMENT_ONLY; not a seal, RED gate or final PASS; mandatory Unknown blocks progression'))
    print(json.dumps(dict(counts=counts,cases=len(catalog),instability=len(instability),profile_differences=len(differences_by_profile),coverage=coverage),indent=2))

if __name__=='__main__':
    main()
