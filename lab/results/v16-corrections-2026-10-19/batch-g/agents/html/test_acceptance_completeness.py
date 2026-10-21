"""Lab-only completeness mutations; no product execution or changed expectations."""
import ast,collections,copy,hashlib,json
from pathlib import Path
P=Path(__file__).resolve().parents[2];F=P.parent/'figure-kind-implementation';script=P/'accept_final.py'
tree=ast.parse(script.read_text());ns={'Counter':collections.Counter}
exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef)],type_ignores=[]),str(script),'exec'),ns)
manifest=json.loads((F/'cases.json').read_text())['cases'];build={'product_sha256':'fixture-product'}
fixture={'passed':True,'changed_inputs':[],'binaries':{'product':{'sha256':'fixture-product'},'oracle':{'sha256':'9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'}},'cases':[{'name':c['name'],'disposition':c['disposition'],'source_sha256':c['source_sha256'],'expected':c['expected_at_closure'],'observed':c['oracle'] if 'policy' in c['expected_at_closure'] else c['expected_at_closure'],'passed':True} for c in manifest]}
rows=[]
def check(name,value,accept):
 try:ns['verify_f_policy_receipt'](value,manifest,build);actual=True
 except AssertionError:actual=False
 rows.append({'name':name,'accepted':actual,'expected':accept,'pass':actual==accept})
check('complete-F84-77-required-5-debt-2-negative',copy.deepcopy(fixture),True)
bad=copy.deepcopy(fixture);bad['cases'].pop();check('missing-F-row',bad,False)
bad=copy.deepcopy(fixture);bad['cases'][1]=copy.deepcopy(bad['cases'][0]);check('duplicated-F-row',bad,False)
bad=copy.deepcopy(fixture);bad['cases'][0]['disposition']='preserved-known-debt';check('required-F-reclassified',bad,False)
bad=copy.deepcopy(fixture);bad['cases'][0]['observed']={'returncode':0,'json':'wrong-language-value'};check('required-F-wrong-observation',bad,False)
classes=[{'class':kind} for kind,n in {'required-language':27,'required-error':12,'required-termination':1,'open-bilateral-gap':2,'refuted-fixture':1}.items() for _ in range(n)]
expected={'required-language':27,'required-error':12,'required-termination':1,'open-bilateral-gap':2,'refuted-fixture':1}
for name,actual,want in [('complete-Ref43-with40-required',classes,True),('missing-required-Ref',classes[1:],False),('extra-Ref',classes+[classes[0]],False)]:
 try:ns['require_distribution'](actual,'class',expected);accepted=True
 except AssertionError:accepted=False
 rows.append({'name':name,'accepted':accepted,'expected':want,'pass':accepted==want})
# Verify all six immutable actual matrix/oracle groups have exact ordered names.
groups=[('', 'render-order-policies-v2.json'),('additional-', 'additional-render-order-policies-v2.json'),('targeted-', 'targeted-render-order-policies-v2.json'),('origin-', 'origin-policies.json'),('origin-edge-', 'origin-edge-policies.json'),('show-origin-selected-', 'show-origin-selected-policies.json')]
base=F/'agents/entities/reference-acceptance';actual_classes=[]
for prefix,policy_file in groups:
 matrix=json.loads((base/(prefix+'matrix.json')).read_text());oracle=json.loads((base/(prefix+'oracle/receipt.json')).read_text());policy=json.loads((base/policy_file).read_text());ns['require_rows'](oracle['cases'],[c['name'] for c in matrix['cases']]);actual_classes.extend({'class':policy['cases'][c['name']]['class']} for c in matrix['cases'])
ns['require_distribution'](actual_classes,'class',expected)
print(json.dumps({'script_sha256':hashlib.sha256(script.read_bytes()).hexdigest(),'fixtures':rows,'actual_immutable_reference_groups':6,'actual_immutable_reference_rows':len(actual_classes),'actual_immutable_required_rows':sum(c['class'].startswith('required-') for c in actual_classes),'all_pass':all(r['pass'] for r in rows)},indent=2))
raise SystemExit(not all(r['pass'] for r in rows))
