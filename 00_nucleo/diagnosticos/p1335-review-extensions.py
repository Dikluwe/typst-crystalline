"""Audit current normative extension evidence, separate from runtime numerator."""
import importlib.util,json,re,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('c',D/'p1335-review-check-r1.py');c=importlib.util.module_from_spec(s);s.loader.exec_module(c)

def main():
    r=c.preflight();evidence=c.read('p1335-inventory-extension-evidence.json');normal=c.read('p1335-matrix-normal.json')['results'];source={}
    for key,item in evidence['inputs'].items():
        for kind in ('prompt','owner'):
            pin=item[kind];p=c.ROOT/pin['path'];r.require(p.is_file() and c.digest(p)==pin['sha256'],'EXTENSION_SOURCE_PIN',[key,kind])
        source[key]=(c.ROOT/item['prompt']['path']).read_text()
        declared=re.findall(r'^\s*//[!/]?\s*@prompt\s+(\S+)\s*$',(c.ROOT/item['owner']['path']).read_text(),re.M)
        r.require(declared==[item['prompt']['path']],'EXTENSION_OWNER_LINK',key)
    accepted=[];pending=[]
    for entry in evidence['entries']:
        p=entry['path']
        for claim in entry['measured_current_claims']:
            txt=(c.ROOT/claim['path']).read_text();actual='\n'.join(txt.splitlines()[claim['line']-1:claim['end_line']])
            r.require(actual==claim['text'] and c.digest(c.ROOT/claim['path'])==claim['sha256'],'EXTENSION_CLAIM_LITERAL',[p,claim['path'],claim['line']])
        if p in ('calc.deg','calc.rad','calc.log10'):
            r.require(entry['documentary_finding']=='NO_BINDING_AUTHORIZATION_FOUND_IN_CURRENT_OWNER','UNAUTHORIZED_CALC_EXTENSION',p);pending.append(p)
        else:
            r.require(entry['documentary_finding'].startswith('EXPLICIT_') and len(entry['measured_current_claims'])>0,'EXTENSION_MISSING_EXPLICIT_CLAIM',p);accepted.append(p)
        r.require(entry['parity_credit']==False,'EXTENSION_CREDIT',p)
        r.require(all(x['runtime_class']=='CRYSTALLINE_ONLY' for x in normal if x['path']==p) and sum(x['path']==p for x in normal)==4,'EXTENSION_CURRENT_PUBLIC_OBSERVATION',p)
    r.require(len(accepted)==len(set(accepted)) and len(pending)==3,'EXTENSION_DUPLICATE',None)
    return dict(at=c.utc(),role='D',checker_sha256=c.digest(__file__),inputs=c.INPUTS,accepted_documented_paths=sorted(accepted),pending_without_current_binding_authorization=pending,
        reasoning='Current L0 explicitly retains global names, flat aliases or named extensions for accepted paths; native implementation presence was not used as intent. Calc unit references/internal log10 formula do not authorize qualified public bindings. Adjustment may exclude accepted C-only cells but never add numerator credit.',
        limits='Documentary read is current claim/context review plus exact source/header validation; no architecture change or functional extension certification. Full unique-owner/hash gates checked separately.',**r.result())

if __name__=='__main__':print(json.dumps(main(),ensure_ascii=False,indent=2))
