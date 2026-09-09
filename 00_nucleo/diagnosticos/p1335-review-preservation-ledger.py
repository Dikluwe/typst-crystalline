"""Reconstruct historical preservation; IDs alone never authorize arbitrary new bytes."""
import base64,collections,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('semantic',D/'p1335-review-semantic.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);c=m.core

def main():
    r=c.preflight();reviewed=c.read('p1335-classification-preservation-reconciliation.json');freeze=c.read('p1335-classification-preservation-freeze-r3.json')
    normal=c.read('p1335-sentinels-normal-r3.json');loc=c.read('p1335-location-control.json');lf=c.read('p1335-location-control-freeze.json')
    for field in ('freeze','measurement','location_control'):
        pin=reviewed[field];r.require(c.digest(c.ROOT/pin['path'])==pin['sha256'],'PRESERVATION_INPUT_PIN',field)
    for path,pin in lf['inputs'].items():r.require(c.digest(path)==pin,'LOCATION_INPUT_PIN',path)
    r.require(loc['freeze_sha256']==c.digest(D/'p1335-location-control-freeze.json'),'LOCATION_FREEZE',None)
    pairs={(x['id'],x['profile']):x for x in normal['rows']};expected={(e['historical_id'],e['profile']):e for e in freeze['rows']}
    rows={(x['historical_id'],x['profile']):x for x in reviewed['rows']};r.require(set(rows)==set(expected) and len(rows)==len(reviewed['rows']),'PRESERVATION_COVERAGE',len(rows))
    authorized={'p1324.table-present-call':'body','p1324.with-present-call':'body','p1324.dictionary-negative':'absent','p1324.content-negative':'absent','p1324.closure-named':'nope','p1324.closure-anonymous':'missing','p1324.closure-with':'missing','p1325.closure-boundary':'nope'}
    location_rows={(x['id'],x['profile'],x['phase']):x for x in loc['rows']};lcases={x['id']:x for x in lf['cases']}
    r.require(len(location_rows)==len(loc['rows']) and set(location_rows)=={(id_,profile,phase) for id_ in lcases for profile in c.PROFILES for phase in ('normal','repeat','reverse')},'LOCATION_COVERAGE',len(location_rows))
    for key,row in location_rows.items():
        case=lcases[key[0]];o=row['observation'];argv=[lf['binary']['path'],'--color=never','eval']
        if c.PROFILES[key[1]]:argv+=['--features',','.join(c.PROFILES[key[1]])]
        argv+=[case['expression']]
        r.require(o['argv']==argv and o['binary_sha256']==lf['binary']['sha256'] and o['source_sha256']==c.sha(case['expression'].encode()) and o['cwd']==str(c.ROOT),'LOCATION_IDENTITY',key)
        r.require(o['features']==c.PROFILES[key[1]] and o['phase']==key[2] and o['side']=='crystalline','LOCATION_PROFILE',key)
        for ch in ('stdout','stderr'):
            r.require(o[ch+'_sha256']==c.sha(o[ch].encode()) and o[ch+'_base64']==base64.b64encode(o[ch].encode()).decode(),'LOCATION_CHANNEL_PIN',[key,ch])
        if not o['complete'] or o.get('reason_code') or o['exit_code'] not in (0,1):r.unknown('LOCATION_UNKNOWN',key)
        e=expected[key[:2]]['expected_final_literal'];r.require(row['expected']==e and all(o['exit_code' if ch=='exit' else ch]==e[ch] for ch in ('exit','stdout','stderr')),'LOCATION_LITERAL',key)
    statuses=[]
    for key,e in expected.items():
        entry=rows[key];current=pairs[e['canonical_id'],e['profile']]['crystalline'];actual=dict(exit=current['exit_code'],stdout=current['stdout'],stderr=current['stderr'])
        r.require(all(entry.get(k)==v for k,v in e.items()),'PRESERVATION_EXPECTATION_UNCHANGED',key)
        r.require(entry['current_literal']==actual,'PRESERVATION_CURRENT_LITERAL',key)
        if actual==e['expected_final_literal']:status='PRESERVED_LITERAL'
        elif key[0] in authorized:
            field=authorized[key[0]];expr=e['current_expression'];pos=expr.rfind('.'+field)+1
            message=e['expected_final_literal']['stderr'].splitlines()[0]
            if 'closure-' in key[0]:message='error: cannot access fields on user-defined functions'
            desired=message+'\n  ┌─ <input-expression>:1:'+str(pos)+'\n  │\n1 │ '+expr+'\n  │ '+' '*pos+'^'*len(field)+'\n\n'
            r.require(actual==dict(exit=1,stdout='',stderr=desired),'AUTHORIZED_CHANGE_EXACT_NOT_ID_ONLY',key)
            status='AUTHORIZED_LATER_CORRECTION'
            auth=entry['authorization'];r.require(c.digest(c.ROOT/auth['l0'])==auth['sha256'] and c.digest(c.ROOT/auth['report']['path'])==auth['report']['sha256'],'AUTHORIZED_CHANGE_PIN',key)
        elif e['needs_location_control']:
            status='PRESERVED_LITERAL_AT_ORIGINAL_LOCATION'
            r.require(all((*key,phase) in location_rows for phase in ('normal','repeat','reverse')),'PRESERVATION_LOCATION_REQUIRED',key)
        else:status='UNEXPLAINED_TEMPORAL_CHANGE';r.unknown(status,key)
        r.require(entry['status']==status,'PRESERVATION_CLASS',key);statuses.append(status)
    r.require(reviewed['counts']==dict(collections.Counter(statuses)),'PRESERVATION_COUNTS',None)
    ledger=m.table('p1335-classification-supplemental-ledger.tsv');rr=m.verify_supplement(ledger,normal['rows']);c.merge_review(r,rr)
    return dict(at=c.utc(),role='D',checker_sha256=c.digest(__file__),inputs=c.INPUTS,counts=dict(collections.Counter(statuses)),location_observations=len(location_rows),scope='Historical final expected preservation plus exact authorized span/message transformations; not blanket functional parity.',**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
