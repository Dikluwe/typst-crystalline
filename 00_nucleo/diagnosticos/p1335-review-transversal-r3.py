"""Independent transcript/profile/order and warning-contract review."""
import base64
import collections
import copy
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('review',D/'p1335-review-check.py')
c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)

def effective(args,features):
    out=[];i=0
    while i<len(args):
        if args[i]=='--features':i+=2;continue
        if args[i].startswith('--features='):i+=1;continue
        out.append(args[i]);i+=1
    return out+(['--features',','.join(features)] if features else [])

def stable(x):
    if isinstance(x,dict):return {k:stable(v) for k,v in x.items() if k!='diff_artifact'}
    if isinstance(x,list):return [stable(v) for v in x]
    return x

def run(focal):
    r=c.preflight();data=c.read('p1335-transversal-focal-r3.json' if focal else 'p1335-transversal-r3.json')
    frozen=c.read('p1335-transversal-freeze-r3.json');corpus=c.read('p1335-transversal-cases.json')
    r.require(data['freeze_sha256']==c.digest(D/'p1335-transversal-freeze-r3.json'),'TRANSVERSAL_FREEZE',focal)
    for p,h in {**frozen['inputs'],**data['artifacts']}.items():r.require(Path(p).is_file() and c.digest(p)==h,'TRANSVERSAL_PIN',p)
    cases={x['id']:x for x in corpus['matrix']['cases']}
    ids=set(cases) if not focal else {'P1137-I-001','P1138-X-004','P1138-L-001'}
    phases=['focal'] if focal else ['normal','repeat','reverse']
    rows={};summaries={};projected_raw_diff=[]
    def check_row(row,profile,phase,source_root,directory):
        case=cases[row['id']];key=(row['id'],profile,phase)
        if not set(case.get('required_features',[]))<=set(c.PROFILES[profile]):
            r.require(row['estado']=='DISABLED_BY_PROFILE','FEATURE_GATE',key);return
        if row['estado']=='UNKNOWN':r.unknown('TRANSVERSAL_UNKNOWN',key)
        for side,role,decl in [('vanilla','oracle','oraculo'),('crystalline','crystalline','cristalino')]:
            o=row.get(role,{})
            source=source_root/case['fonte_typ'] if case['fonte_typ'] else None
            substitutions=dict(source=str(source or ''),output=str(directory/role),root=str(c.ROOT),fixtures=str(source_root/'fixtures'))
            argv=[frozen['binaries'][side]['path'],*[x.format(**substitutions) for x in effective(case[decl]['args'],c.PROFILES[profile])]]
            r.require(o.get('command')==argv,'TRANSVERSAL_ARGV',[key,side])
            r.require(o.get('binary_sha256')==frozen['binaries'][side]['sha256'],'TRANSVERSAL_BINARY',[key,side])
            r.require(o.get('source_sha256')==(c.digest(source) if source else None),'TRANSVERSAL_SOURCE',[key,side])
            r.require(o.get('cwd')==str(c.ROOT) and o.get('features')==c.PROFILES[profile],'TRANSVERSAL_ENV',[key,side])
            if not o.get('complete') or o.get('harness_error') or (o.get('exit_code') not in (0,1) and not c.known_query_rejection(o)) or any(not isinstance(o.get(k),str) for k in ('stdout','stderr')):
                r.unknown('TRANSVERSAL_EXECUTION_UNKNOWN',[key,side])
            if c.known_query_rejection(o):r.require(row['estado']=='DIFFERENCE','QUERY_CAPABILITY_NOT_JSON_MATCH',key)
        differing=[k for k in ('exit_code','stdout','stderr') if row['oracle'][k]!=row['crystalline'][k]]
        r.require(row['differing_channels']==differing and row['channel_state']==('RAW_DIFFERENCE' if differing else 'RAW_EQUAL'),'RAW_CHANNEL_RECONSTRUCTION',key)
        if row['estado']=='MATCH' and differing:projected_raw_diff.append(dict(id=key[0],profile=profile,phase=phase,channels=differing))
    for group in data['matrix']:
        phase,profile=group['phase'],group['profile']
        r.require({x['id'] for x in group['results']}==ids,'TRANSVERSAL_CASE_COVERAGE',[phase,profile])
        count=collections.Counter(x['estado'] for x in group['results'])
        r.require(dict(count)=={k:v for k,v in group['counts'].items() if v},'TRANSVERSAL_COUNTS',[phase,profile]);summaries[phase+'/'+profile]=dict(count)
        for row in group['results']:
            key=(row['id'],profile,phase);r.require(key not in rows,'TRANSVERSAL_DUPLICATE',key);rows[key]=row
            check_row(row,profile,phase,Path(corpus['temp']),Path(data['output_root'])/phase/profile/row['id'])
    expected={(id_,profile,phase) for id_ in ids for profile in c.PROFILES for phase in phases}
    r.require(set(rows)==expected,'TRANSVERSAL_CROSS_PRODUCT',len(rows))
    if not focal:
        for key,row in rows.items():
            if key[2]=='normal':continue
            normal=rows[key[0],key[1],'normal']
            r.require(normal['estado']==row['estado'] and stable(normal.get('observed'))==stable(row.get('observed')),'TRANSVERSAL_PROJECTION_STABILITY',key)
            if 'oracle' in row:
                for side in ('oracle','crystalline'):
                    for channel in ('exit_code','stdout','stderr'):r.require(row[side][channel]==normal[side][channel],'TRANSVERSAL_CHANNEL_STABILITY',[key,side,channel])
        for row in data['location']:
            check_row(row,row['profile'],row['phase'],Path(corpus['internal']),Path(data['output_root'])/'location'/row['phase']/row['profile']/row['id'])
    fixed=base64.b64decode(corpus['warning_base64']).decode();warnings={x['id']:x for x in corpus['warning_cases']}
    wexpected={id_ for id_ in warnings} if not focal else {'html-compile','html-error-order','gate-default'}
    wrows={}
    for row in data['warnings']:
        case=warnings[row['id']];key=(row['id'],row['phase']);r.require(key not in wrows,'WARNING_DUPLICATE',key);wrows[key]=row
        sides={'crystalline'} if '--html-serialization' in case['argv'] else {'vanilla','crystalline'}
        r.require(set(row['observations'])==sides and row['bilateral']==(len(sides)==2),'WARNING_SIDES',key)
        for side,o in row['observations'].items():
            output=Path(data['output_root'])/'warning'/f"{row['phase']}-{side}-{row['id']}.{case['format']}"
            args=[x.replace(str(D/'p1323-fixtures'),corpus['html']).replace('{output}',str(output)) for x in case['argv']]
            r.require(o['command']==[frozen['binaries'][side]['path'],*args],'WARNING_ARGV',[key,side])
            r.require(o['stdin_base64']==case['stdin_base64'] and o['binary_sha256']==frozen['binaries'][side]['sha256'],'WARNING_IDENTITY',[key,side])
            err=o['stderr'];valid=(err.startswith(fixed) and (not case['error_expected'] or err[len(fixed):].startswith('error:'))) if case['warning_expected'] else fixed.splitlines()[0] not in err
            r.require(o['warning_contract']==valid,'WARNING_CONTRACT_CLASS',[key,side])
            if side=='crystalline':r.require(valid,'CRYSTALLINE_WARNING_CONTRACT_NOT_PRESERVED',[key,side])
            if case['warning_expected']:r.require(err.count(fixed.splitlines()[0])==1,'WARNING_DUPLICATED',[key,side])
            if not o['complete']:r.unknown('WARNING_EXECUTION_UNKNOWN',[key,side])
    r.require(set(wrows)=={(id_,phase) for id_ in wexpected for phase in phases},'WARNING_COVERAGE',len(wrows))
    controls=[]
    genuine=next((row['crystalline'] for row in rows.values() if 'crystalline' in row and c.known_query_rejection(row['crystalline'])),None)
    if genuine:
        for mode in ('genuine','truncated','missing','generic','crash','other_command'):
            item=copy.deepcopy(genuine)
            if mode=='truncated':item['stderr']=item['stderr'].splitlines()[0]
            if mode=='missing':item.pop('stderr')
            if mode=='generic':item['stderr']='error: failed\n'
            if mode=='crash':item['exit_code']=-9
            if mode=='other_command':item['command'][1]='eval'
            actual='PUBLIC_CLI_ABSENT' if c.known_query_rejection(item) else 'Unknown'
            expected_control='PUBLIC_CLI_ABSENT' if mode=='genuine' else 'Unknown'
            r.require(actual==expected_control,'QUERY_POLICY_CONTROL',mode);controls.append(dict(mode=mode,expected=expected_control,actual=actual))
    return dict(at=c.utc(),role='D',manifest_sha256=c.digest(D/'p1335-manifest.json'),checker_sha256=c.digest(__file__),inputs=c.INPUTS,counts=summaries,projected_match_raw_differences=projected_raw_diff,query_controls=controls,
        limits=['Checks raw channels, identity and declared projections; does not certify global layout/export or substitute visual QA.','C-only serialization controls cannot be bilateral parity.'],**r.result())

if __name__=='__main__':print(json.dumps(run('focal' in sys.argv),ensure_ascii=False,indent=2))
