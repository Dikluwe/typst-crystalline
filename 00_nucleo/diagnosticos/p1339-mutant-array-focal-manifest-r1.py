"""Prospective manifest generator, metadata only: does not import executor."""
import ast,datetime,hashlib,json,pathlib
ROOT=pathlib.Path(__file__).resolve().parents[2];D=ROOT/'00_nucleo/diagnosticos'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def pin(p):return {'path':str(pathlib.Path(p).resolve()),'sha256':sha(p)}
def read(p):return json.loads(pathlib.Path(p).read_text())
executor=D/'p1339-mutant-array-focal-r1.py';ast.parse(executor.read_text())
registry=read(D/'p1339-mutant-array-registry-r1.json')
oracle_path=D/'p1339-contract-array-oracles-focal-r1.json';oracle=read(oracle_path)
assert sha(oracle_path)=='9fde62b1664bdda6d83aece3580f9acbebeb2768113a204ebeb8889117b3c533'
assert sha(D/'p1339-mutant-array-registry-r1.json')=='d48373e9944f1f1cc9e8417a7379d71d00016d2e3a1f38fa46a9e2d1c61df9ee'
protected={p['path']:p for p in registry['input_pins']}
for relative,h in registry['l0_raw_pins'].items():
    p=ROOT/relative;assert sha(p)==h;protected[str(p)]=pin(p)
for p in [executor,pathlib.Path(__file__),oracle_path,D/'p1339-mutant-array-registry-r1.json',pathlib.Path(registry['binary']['path']),pathlib.Path(registry['baseline_archive']['path'])]:
    protected[str(p)]=pin(p)
plan=[{'role':'control','mode':0,'id':c['id'],'family':None} for c in oracle['cases']]
plan += [{'role':'mutant','mode':m['mode'],'id':i,'family':m['id']} for m in registry['modes'] for i in m['prospective_witness_ids']]
plan += [{'role':'invalid_mode','mode':255,'id':'ARR-P01-empty','family':None}]
assert len(plan)==55
manifest={
    'schema':'p1339-array-mutation-focal-manifest-v1','author':'/root/p1336_tests',
    'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'regime':'executado sem atestacao de isolamento','status':'PROSPECTIVE_PENDING_INDEPENDENT_GO_NO_EXECUTION',
    'authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
    'executor':pin(executor),'protected_inputs':list(protected.values()),'process_plan':plan,
    'command':['python3',str(executor),'--go','<independent GO JSON path>'],
    'required_go_fields':{'verdict':'GO_ARRAY_MUTANT_FOCAL_55','executor_sha256':sha(executor),'manifest_sha256':'actual SHA256 of this manifest, filled only by independent verifier'},
    'argv_template':[registry['binary']['path'],'eval','<unchanged frozen source>'],
    'profile':'default','order':'normal','stdin':'empty bytes',
    'environment':{'overrides':{'NO_COLOR':'1','PYTHONDONTWRITEBYTECODE':'1','P1339_ARRAY_MUTANT':'exact mode in process_plan','RUST_BACKTRACE':'0'},'inherited_relevant_values':'recorded for every row including PATH, locale, HOME, TZ, SOURCE_DATE_EPOCH, all TYPST_/FONTCONFIG_/XDG_ keys','core_limit':0},
    'timeout_seconds':30,'worker_count':1,
    'integrity':'Pins plus full isolated source/config inventory before/after; actual executed binary before/after every process; runner/manifest/GO pins; raw argv/channels/base64/UTC/exit/signal/environment; git before/after excludes restricted directories.',
    'control_gate':'Execute all28 controls first. Exactly27 FrozenLiteralDefault must match full UTF8 byte channels and exit; E08 expected=null remains UnknownNoFrozenExpected. Any other control mismatch/Unknown pauses before mutants; save all raw controls.',
    'negative_gate':'Only actual normal exit0/1 full raw differences count as observed differences. Abnormal transport and E08 never yield rejection credit. Every family needs an independently reviewed distinguishing witness; comparator never produces independent verdict or mutation score.',
    'budget':'55-process adversary tranche of same Array focal cycle1, following56 reference/baseline observations. Not a new cycle, no budget reset. Maximum second corrective cycle remains separately authorized only.',
    'no_credit':['E08 Unknown blocks its mandatory final dimension','Private bridge is not part of this executor and not yet run','No candidate, expanded matrix,70original float-dependent probes,global4718,old20 or W/F closure inferred'],
}
path=D/'p1339-mutant-array-focal-manifest-r1.json'
with path.open('x') as f:json.dump(manifest,f,ensure_ascii=False,indent=2);f.write('\n')
print(json.dumps({'manifest':pin(path),'executor':pin(executor),'processes':len(plan)}))
