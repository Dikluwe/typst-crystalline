"""Batch-one nominal resolution and direct variant-to-payload declaration map."""
import datetime, hashlib, json, pathlib, re, runpy
ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
helpers=runpy.run_path(str(D/'p1339-mutant-closed-state-inventory-supplement.py'))
decl,pin,REG=helpers['declaration'],helpers['pin'],helpers['REG']
closure=json.loads((D/'p1339-mutant-closed-state-inventory-closure.json').read_text())
supp=json.loads((D/'p1339-mutant-closed-state-inventory-supplement.json').read_text())
extra=[]
for qualified,path,name in [
 ('rust_decimal::Decimal',REG/'rust_decimal-1.42.1/src/decimal.rs','Decimal'),
 ('regex::Regex',REG/'regex-1.12.3/src/regex/string.rs','Regex'),
 ('regex_automata::meta::regex::Regex',REG/'regex-automata-0.4.14/src/meta/regex.rs','Regex'),
 ('regex_automata::meta::regex::RegexI',REG/'regex-automata-0.4.14/src/meta/regex.rs','RegexI'),
 ('regex_automata::meta::regex::CachePool',REG/'regex-automata-0.4.14/src/meta/regex.rs','CachePool'),
 ('regex_automata::meta::regex::CachePoolFn',REG/'regex-automata-0.4.14/src/meta/regex.rs','CachePoolFn'),
 ('crate::compiler::eval::EvalTarget',ROOT/'01_core/src/compiler/eval/mod.rs','EvalTarget'),
 ('crate::compiler::eval::flow::FlowEvent',ROOT/'01_core/src/compiler/eval/flow.rs','FlowEvent'),
]:
 x=decl(path,name);x['qualified_name']=qualified;extra.append(x)
all_declarations=closure['declarations']+supp['declarations']+extra
names={}
for x in all_declarations:names.setdefault(x['name'],[]).append(x)
aliases={x['token']:x['target'] for x in supp['aliases']}
aliases['IndependentStyle']='citationberg::IndependentStyle through hayagriva re-export'
aliases['InnerDecimal']='rust_decimal::Decimal (1.42.1)'
excluded=set(closure['stdlib_or_generic_tokens_not_expanded'])
maps=[]
for root in ['Value','Content']:
 item=next(x for x in closure['declarations'] if x['name']==root)
 variants=item['direct_variants']
 for index,variant in enumerate(variants):
  stop=variants[index+1]['line'] if index+1<len(variants) else item['last_line']
  lines=[x for x in item['declaration'] if variant['line']<=x['line']<stop]
  payload='\n'.join(x['text'] for x in lines)
  payload=re.sub(r'^    '+re.escape(variant['name'])+r'\b','    ',payload,count=1)
  tokens=set(re.findall(r'\b[A-Z][A-Za-z0-9_]*\b',payload))
  mapped={}
  for token in sorted(tokens):
   if token in excluded: mapped[token]={'kind':'stdlib/generic container; payload parameters remain mapped separately'}
   elif token in aliases:mapped[token]={'kind':'qualified alias','target':aliases[token]}
   elif token in names:
    refs={(x['path'],x['first_line']):{'path':x['path'],'line':x['first_line'],'qualified_name':x.get('qualified_name'),
          'sha256':x.get('source_sha256',x.get('sha256'))} for x in names[token]}
    mapped[token]={'kind':'declaration references; use exact payload qualification for homonyms','references':list(refs.values())}
   else:raise ValueError(('unresolved direct variant payload',root,variant['name'],token))
  maps.append({'owner':root,'variant':variant['name'],'source_path':item['path'],'source_sha256':item['source_sha256'],
               'line':variant['line'],'payload_declaration':lines,'nominal_payload_tokens':mapped})
record={
 'schema':'p1339-closed-direct-payload-nominal-resolution-v1','batch':1,
 'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'executor':'/root/p1336_tests','role':'declarative transport/signature adapter only',
 'regime':'executado sem atestação de isolamento','inherited_context':'P1336; P1339 baseline only, no candidate',
 'status':'Nominal map and structural witness inputs, not executed coverage or verdict',
 'inputs':{n:pin(D/n) for n in [
  'p1339-mutant-closed-state-inventory.json','p1339-mutant-closed-state-inventory-supplement.json',
  'p1339-mutant-closed-state-inventory-closure.json','p1339-budget-redesign-r1.json',
  'p1339-verifier-budget-redesign-acceptance-r1.md','p1339-l0-freeze.json','p1339-authority-manifest-r2.json']},
 'generator':pin(pathlib.Path(__file__)),'cargo_lock':pin(ROOT/'Cargo.lock'),
 'provenance':closure['head'],'additional_qualified_declarations':extra,
 'direct_payload_map':maps,'qualified_aliases':aliases,
 'external_boundary_witness_inputs':[
  {'subject':'InnerDecimal','source':pin(ROOT/'01_core/src/entities/decimal.rs'),'import_line':11,
   'target':'rust_decimal::Decimal','fields':['flags:u32','hi:u32','lo:u32','mid:u32'],
   'limit':'Closed integer representation declaration; normalization/observation equivalence remains L0 plus verifier, not invented here.'},
  {'subject':'crate::entities::regex::Regex.compiled','source':pin(ROOT/'01_core/src/entities/regex.rs'),
   'target':'Arc<regex::Regex>, not recursive crate::entities::regex::Regex',
   'graph':'regex::Regex { meta:regex_automata::meta::Regex, pattern:Arc<str> }; meta::Regex { imp:Arc<RegexI>, pool:CachePool }; RegexI { strat:Arc<dyn Strategy>, info:RegexInfo }; CachePool=Pool<Cache,CachePoolFn>; CachePoolFn=Box<dyn Fn()->Cache+Send+Sync+UnwindSafe+RefUnwindSafe>.',
   'limit':'Actual external compiled engine/cache boundary, not a Typst Func. No Arc-address sameness or generic opaque-leaf claim. Regex language-observable projection and exclusion of mutable compilation caches need the independent normative structural witness; declaration graph is not that verdict.'},
 ],
 'cost_history':{'product_processes':0,'future_api_executions':0,'focal_batches':0,
  'causes':['Ancestral global variant-name subtraction omitted payload homonyms.',
            'First supplement resolved only reported tokens; closure successor repairs all enum payload positions.',
            'Current supplement resolves newly exposed InnerDecimal and explicitly distinguishes external regex::Regex; previous artifacts unchanged.']},
 'limits':['Lexical declaration reachability is not rustc type resolution; every qualified ambiguity retains its separate declaration.',
           'Direct 38 Value and 98 Content variants are all mapped to declared nominal payloads, including unit variants.',
           'External service/trait/macro/compiled-engine boundaries remain explicitly named witness obligations; not recursively equated by Eq/Debug/Arc.',
           'Independent oracle must assign concrete tests or normative structural witnesses for every variant/field; this map does not assert acceptance.'],
}
assert len(maps)==136
out=D/'p1339-mutant-closed-state-inventory-resolution.json'
with out.open('x') as f:json.dump(record,f,ensure_ascii=False,indent=2);f.write('\n')
print(json.dumps(pin(out)))
