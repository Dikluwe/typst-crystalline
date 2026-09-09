"""Independent source/argv/channels and legacy-envelope checks for R2 sentinels."""
import base64
import collections
import importlib.util
import json
import os
from pathlib import Path
import re
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('review',D/'p1335-review-check.py')
c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)

def envelope(item,case):
    out,err,code=item['stdout'],item['stderr'],item['exit_code']
    if case.get('fixture'):
        for path in sorted({case['fixture'],os.path.relpath(case['fixture'],c.ROOT)},key=len,reverse=True):
            err=err.replace(path,'<source:'+case['source_sha256']+'>')
    if code==0 and case['route']=='eval':
        try:value=json.loads(out)
        except (ValueError,TypeError):return dict(kind='Unknown',reason='OUTPUT_NOT_JSON')
        if err:return dict(kind='Unknown',reason='SUCCESS_SIDE_DIAGNOSTIC',stderr=err)
        return dict(kind='value',value=value,stderr='',exit=0)
    if code==1 and case['route']=='compile':
        found=re.findall(r'^error: panicked with: '+re.escape(case.get('transport_marker','P1307R4:'))+r'([0-9,]*):END$',err,re.M)
        if found and len(re.findall(r'^error:',err,re.M))==1 and not re.search(r'^warning:',err,re.M):
            try:value=bytes(int(x) for x in found[0].split(',') if x).decode('utf-8')
            except (ValueError,UnicodeError):return dict(kind='Unknown',reason='INVALID_BYTE_TRANSPORT')
            return dict(kind='value',value=value,stderr='',exit=0)
    if code==1 and re.search(r'^error:',err,re.M):
        locations=re.findall(r'[┌└]─ ([^\n]+):(\d+):(\d+)',err)
        if any(p not in {'<input-expression>','<source:'+case['source_sha256']+'>'} for p,_,_ in locations):
            return dict(kind='Unknown',reason='DIAGNOSTIC_SOURCE_UNRESOLVED',stderr=err)
        return dict(kind='diagnostic',exit=code,stdout=out,stderr=err,messages=re.findall(r'^error: (.*)$',err,re.M),hints=re.findall(r'^\s*= hint: (.*)$',err,re.M))
    return dict(kind='Unknown',reason='CLI_OR_CONTEXT_EXECUTION_FAILURE',exit=code,stderr=err)

def run(phases):
    r=c.preflight();corpus=c.read('p1335-sentinels-cases-r2.json');frozen=c.read('p1335-sentinels-freeze-r2.json')
    cases={x['id']:x for x in corpus['cases']};r.require(len(cases)==len(corpus['cases']),'CASE_DUPLICATE',len(cases))
    for p,h in frozen['inputs'].items():r.require(c.digest(p)==h,'SUPPLEMENT_INPUT_PIN',p)
    historical=c.read('p1322-sentinels-cases.json');old={x['id']:x for x in historical['cases']}
    aliases={x['id']:x['canonical'] for x in corpus['aliases']}
    for id_,canonical in aliases.items():
        r.require(canonical in cases and id_ in cases[canonical]['aliases'],'ALIAS_BINDING',[id_,canonical])
        policy=cases[canonical].get('alias_policies',{}).get(id_)
        r.require(isinstance(policy,dict),'ALIAS_POLICY_MISSING',id_)
        if id_ in old:r.require(policy==old[id_],'ALIAS_HISTORY_NOT_EXACT',id_)
    for id_,case in cases.items():
        source=case.get('document',case['expression'])
        r.require(c.sha(source.encode())==case['source_sha256'],'CASE_SOURCE_HASH',id_)
        if case['route']=='compile':
            r.require(Path(case['fixture']).read_text().rstrip('\n')==source.rstrip('\n'),'COMPILE_SOURCE_IDENTITY',id_)
    maps={};summaries={}
    for phase in phases:
        matrix=c.read('p1335-sentinels-'+phase+'-r2.json')
        r.require(matrix['freeze_sha256']==c.digest(D/'p1335-sentinels-freeze-r2.json'),'SUPPLEMENT_FREEZE',phase)
        expected={(id_,p) for id_,case in cases.items() for p in case['profiles']}
        if phase=='focal':expected={(x['id'],p) for x in matrix['rows'] for p in cases[x['id']]['profiles']}
        rows={(x['id'],x['profile']):x for x in matrix['rows']}
        r.require(len(rows)==len(matrix['rows']) and set(rows)==expected,'SUPPLEMENT_COVERAGE',phase)
        counts=collections.Counter()
        for key,row in rows.items():
            case=cases[key[0]];r.require(row['universe']=='supplement','SUPPLEMENT_UNIVERSE',key)
            r.require(row['aliases']==case['aliases'],'ROW_ALIAS_BINDING',key)
            for side in ('vanilla','crystalline'):
                item=row[side];binary=frozen['binaries'][side]
                argv=[binary['path'],'--color=never',case['route']]
                if case['route']=='eval':
                    argv+=[case['expression']]
                    if case['format']=='json':argv+=['--format','json']
                else:argv += [case['fixture'],str(Path(corpus['temp'])/(c.sha(case['id'].encode())[:16]+f'-{key[1]}-{side}-{phase}.pdf')),'--format','pdf']
                if c.PROFILES[key[1]]:argv+=['--features',','.join(c.PROFILES[key[1]])]
                r.require(item['argv']==argv,'SUPPLEMENT_ARGV',[key,side])
                r.require(item['binary_path']==binary['path'] and item['binary_sha256']==binary['sha256'],'SUPPLEMENT_BINARY',[key,side])
                r.require(item['source_sha256']==case['source_sha256'] and item['cwd']==case['cwd'],'SUPPLEMENT_SOURCE',[key,side])
                r.require(item['features']==c.PROFILES[key[1]],'SUPPLEMENT_FEATURES',[key,side])
                for channel in ('stdout','stderr'):
                    raw=base64.b64decode(item[channel+'_base64'],validate=True)
                    r.require(c.sha(raw)==item[channel+'_sha256'] and raw.decode(errors='replace')==item[channel],'SUPPLEMENT_CHANNEL',[key,side,channel])
                if case.get('legacy_projection'):r.require(item.get('observable')==envelope(item,case),'LEGACY_ENVELOPE_RECONSTRUCTION',[key,side])
            calculated=c.classify(row['vanilla'],row['crystalline'],require_json=case['route']=='eval' and case['format']=='json')
            r.require(calculated==row['runtime_class'],'SUPPLEMENT_CLASS',[key,calculated,row['runtime_class']])
            if calculated=='EXECUTION_UNKNOWN':r.unknown('SUPPLEMENT_EXECUTION_UNKNOWN',key)
            counts[calculated]+=1
            if case.get('legacy_projection'):
                calculated=c.envelope_class(case['observations'][key[1]]['future_expected'],envelope(row['crystalline'],case))
                r.require(row['historical_preservation']==calculated,'HISTORICAL_PRESERVATION',key)
                calculated=c.envelope_class(envelope(row['vanilla'],case),envelope(row['crystalline'],case))
                r.require(row['language_projection']==calculated,'LANGUAGE_PROJECTION',key)
            for alias,policy in case.get('alias_policies',{}).items():
                if policy.get('legacy_projection'):
                    expected_alias=c.envelope_class(policy['observations'][key[1]]['future_expected'],envelope(row['crystalline'],policy))
                    r.require(row['alias_historical_preservation'].get(alias)==expected_alias,'ALIAS_PRESERVATION',[key,alias])
        r.require(dict(counts)==matrix['counts'],'SUPPLEMENT_COUNTS',phase)
        maps[phase]=rows;summaries[phase]=dict(counts)
    if 'normal' in maps:
        for phase in ('repeat','reverse'):
            if phase not in maps:continue
            for key,row in maps['normal'].items():
                other=maps[phase][key]
                for side in ('vanilla','crystalline'):
                    for field in ('exit_code','stdout','stderr','complete','reason_code','observable'):
                        r.require(row[side].get(field)==other[side].get(field),'SUPPLEMENT_INSTABILITY',[phase,key,side,field])
    return dict(at=c.utc(),role='D',manifest_sha256=c.digest(D/'p1335-manifest.json'),checker_sha256=c.digest(__file__),inputs=c.INPUTS,cases=len(cases),aliases=len(aliases),counts=summaries,limits=['Legacy projection uses explicit fixture identity substitution; raw channels remain fully compared and retained. Historical mismatch is not automatically product regression.'],**r.result())

if __name__=='__main__':print(json.dumps(run(sys.argv[1:] or ['normal','repeat','reverse']),ensure_ascii=False,indent=2))
