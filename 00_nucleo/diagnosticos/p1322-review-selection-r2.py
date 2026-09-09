"""Recheck current ranking and repeat D12 against the corrected cohort universe."""
import copy
import importlib.util
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def module(name,file):
    spec=importlib.util.spec_from_file_location(name,D/file)
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
semantic=module('semantic','p1322-review-semantic.py')
attacks=module('attacks','p1322-review-attacks.py')
c=semantic.core
selection=c.read('p1322-classification-selection-r2.json')
old=c.read('p1322-classification-selection.json')
def judge(data):
    r=semantic.verify_selection(data)
    cohorts={x['id']:x for x in data['cohorts']}
    h=cohorts.get('html-experimental-warning-hints',{})
    r.require(h.get('priority')==2,'HTML_WARNING_L0_CONTRADICTION',h.get('priority'))
    r.require(h.get('owners')==['04_wiring/src/main.rs'],'HTML_FIXED_WARNING_OWNER',h.get('owners'))
    r.require(h.get('paths')==['CLI.compile.html.warning'],'HTML_PATH_NOT_MULTIPLIED',h.get('paths'))
    r.require(h.get('regression_surface',{}).get('rank')==1,'HTML_FIXED_WARNING_SURFACE',h.get('regression_surface'))
    return r
result=judge(selection)
mutant=copy.deepcopy(selection)
mutant['selected']=copy.deepcopy(old['selected'])
attack=attacks.attack_result('D12',selection,mutant,judge,
    [{'replace':'selected','value':old['selected']['id'],'fault':'Lower priority old winner retained after actual HTML L0 contradiction added'}])
for name in ['p1322-classification-query-hint-addendum.md','p1322-classification-query-hint-addendum-r2.md',
             'p1322-review-channels.json','p1322-review-channels-objection.md']:
    c.INPUTS[str(D/name)]=c.digest(D/name)
receipt=dict(at=c.utc(),role='D',scope='Successor ranking and repeated frozen D12; no increase to unique 13-attack score',
    checker_sha256=c.digest(__file__),semantic_checker_sha256=c.digest(D/'p1322-review-semantic.py'),
    inputs=c.INPUTS,selected=selection['selected'],attack=attack,**result.result())
attacks.publish('p1322-review-selection-r2.json',receipt)
print(receipt['verdict'], 'D12 valid',attack['valid'],'rejected',attack['rejected'])
