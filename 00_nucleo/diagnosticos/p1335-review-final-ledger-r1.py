"""Final supplemental temporal qualification and transversal ledger accounting."""
import collections,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('s',D/'p1335-review-semantic.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);c=m.core

def main():
    r=c.preflight();summary=c.read('p1335-classification-summary-r1.json');current=c.read('p1335-sentinels-normal-r3.json')['rows'];previous=c.read('p1322-sentinels-normal.json')['rows']
    rows=m.table('p1335-classification-supplemental-transitions-final-r1.tsv');bykey={(x['probe_id'],x['profile']):x for x in rows};prior={(x['id'],x['profile']):x for x in previous}
    r.require(len(rows)==len(bykey) and set(bykey)=={(x['id'],x['profile']) for x in current},'SUPPLEMENT_TEMPORAL_COVERAGE',len(rows));counts=collections.Counter()
    for row in current:
        key=row['id'],row['profile'];t=bykey[key];old=prior.get(key)
        comparable=bool(old and old['expression']==row['expression'] and 'eval' in row['vanilla']['argv'] and old['vanilla']['cwd']==row['vanilla']['cwd'])
        literal=bool(comparable and all(old[side][ch]==row[side][ch] for side in ('vanilla','crystalline') for ch in ('exit_code','stdout_base64','stderr_base64')))
        before=bool(old and (old['runtime_class'] in c.MATCH or old.get('language_projection')=='Preserved'));now=row['runtime_class'] in c.MATCH or row.get('language_projection')=='Preserved'
        expected='ADDED_SUPPLEMENT' if not old else ('CLOSED_NOW' if comparable else 'CURRENT_CLOSED_RELOCATED_NO_LITERAL_TEMPORAL_CLAIM') if not before and now else 'REOPENED_CANDIDATE' if before and not now else 'UNCHANGED_COMPLETE_CHANNELS' if literal else 'CHANGED_COMPLETE_CHANNELS' if comparable else 'LOCATION_OR_ROUTE_CHANGED_NO_RAW_TEMPORAL_CLAIM'
        r.require(t['fixture_comparable']==str(comparable).lower() and t['complete_literal_equal']==str(literal).lower(),'SUPPLEMENT_TEMPORAL_INPUT',key)
        r.require(t['transition']==expected,'SUPPLEMENT_TEMPORAL_CLASS',[key,expected,t['transition']]);counts[expected]+=1
        if expected=='REOPENED_CANDIDATE':r.unknown('UNISOLATED_REGRESSION',key)
    r.require(dict(counts)==summary['temporal']['functional_supplement']==summary['supplement']['temporal'],'SUPPLEMENT_TEMPORAL_COUNTS',dict(counts))
    trans=m.table('p1335-classification-transversal-ledger-r1.tsv');tx=c.read('p1335-transversal-r3.json')
    identity=lambda x:(x['probe_id'],x['profile'],x['phase'],x['universe'])
    r.require(len({identity(x) for x in trans})==len(trans),'TRANSVERSAL_LEDGER_DUPLICATE',len(trans))
    r.require(dict(collections.Counter(x['universe'] for x in trans))==summary['transversal']['rows_by_universe'] and len(trans)==summary['transversal']['rows'],'TRANSVERSAL_LEDGER_COUNTS',None)
    r.require(dict(collections.Counter(x['current_language_class'] for x in trans))==summary['transversal']['classes'],'TRANSVERSAL_LEDGER_CLASSES',None)
    mapped={identity(x):x for x in trans}
    for g in tx['matrix']:
        for row in g['results']:
            entry=mapped[row['id'],g['profile'],g['phase'],'transversal'];r.require(entry['raw_channel_state']==row.get('channel_state','') and entry['projection_state']==row['estado'],'TRANSVERSAL_RAW_NOT_HIDDEN',[row['id'],g['profile'],g['phase']])
            if row['id']=='P1137-I-001':r.require(entry['current_language_class']=='OPEN_QUERY_DIAGNOSTIC_OR_CLI_CAPABILITY','QUERY_NOT_FULL_MATCH',g['profile'])
    for row in tx['extra']:
        entry=mapped[row['id'],row['profile'],row['phase'],'supplement-boundary'];actual=c.classify(row['vanilla'],row['crystalline']);r.require(entry['raw_channel_state']==actual and entry['current_language_class']==('CLOSED_MEASURED_BOUNDARY' if actual in c.MATCH else 'OPEN_LANGUAGE_BOUNDARY'),'BOUNDARY_LEDGER_CLASS',row['id'])
    return dict(at=c.utc(),role='D',checker_sha256=c.digest(__file__),inputs=c.INPUTS,supplement_temporal=dict(counts),transversal_rows=len(trans),limits='Mechanical raster/font differences remain explicitly scoped projections. This check does not independently rerender every export or certify global layout.',**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
