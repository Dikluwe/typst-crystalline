"""Independent current boundary/math receipt validation, outside global denominator."""
import collections,importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('review',D/'p1335-review-check.py');c=importlib.util.module_from_spec(s);s.loader.exec_module(c)

def main():
    r=c.preflight(); summaries={}; details={}
    for name,freeze_name,cases_name,rows_key,cases_key in [
        ('p1335-transversal-r3.json','p1335-transversal-freeze-r3.json','p1335-transversal-cases.json','extra','extra'),
        ('p1335-math-controls.json','p1335-math-controls-freeze.json','p1335-math-controls-freeze.json','rows','cases'),
        ('p1335-math-controls-r1.json','p1335-math-controls-freeze-r1.json','p1335-math-controls-freeze-r1.json','rows','cases')]:
        data=c.read(name);freeze=c.read(freeze_name);cases=c.read(cases_name)[cases_key]
        r.require(data['freeze_sha256']==c.digest(D/freeze_name),'BOUNDARY_FREEZE',name)
        for path,pin in freeze['inputs'].items():r.require(c.digest(path)==pin,'BOUNDARY_INPUT_PIN',path)
        baseline=c.read('p1335-baseline.json')['state']['product_inventory']
        for phase in ('before','after'):r.require(data[phase]['product_inventory']==baseline,'BOUNDARY_SOURCE_STATE',[name,phase])
        maps={};summaries[name]={};details[name]=[]
        for phase in ('normal','repeat','reverse'):
            rows=[x for x in data[rows_key] if x['phase']==phase]
            matrix=dict(results=rows,counts=dict(collections.Counter(x['runtime_class'] for x in rows)),pairs=len(cases)*len(c.PROFILES),probes=len(cases))
            rr,bykey,counts=c.verify_matrix(matrix,dict(probes=cases),freeze['binaries'],phase)
            r.checks+=rr.checks;r.violations+=rr.violations;r.unknowns+=rr.unknowns;maps[phase]=bykey;summaries[name][phase]=counts
            for key,row in bykey.items():
                for side in ('vanilla','crystalline'):
                    r.require(row[side]['cwd']==str(c.ROOT),'BOUNDARY_CWD',[name,phase,key,side])
                if phase=='normal' and key[1]=='default':details[name].append(dict(id=key[0],expression=row['expression'],runtime_class=row['runtime_class'],vanilla_stdout=row['vanilla']['stdout'],crystalline_stdout=row['crystalline']['stdout']))
        for phase in ('repeat','reverse'):
            for key,row in maps[phase].items():
                normal=maps['normal'][key]
                r.require(row['runtime_class']==normal['runtime_class'],'BOUNDARY_CLASS_STABILITY',[name,phase,key])
                for side in ('vanilla','crystalline'):
                    for channel in ('stdout_base64','stderr_base64','exit_code'):r.require(row[side][channel]==normal[side][channel],'BOUNDARY_RAW_STABILITY',[name,phase,key,side,channel])
    return dict(at=c.utc(),role='D',checker_sha256=c.digest(__file__),inputs=c.INPUTS,counts=summaries,default_language_witnesses=details,scope='Supplemental eval only; single-letter generic control is refuted grammar and not successful function dispatch; no principal denominator or global math closure.',**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
