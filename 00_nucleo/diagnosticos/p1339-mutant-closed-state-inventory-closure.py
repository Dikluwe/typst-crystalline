"""Extract baseline carrier declarations, never implementations or predicates."""
import datetime, hashlib, json, pathlib, re, subprocess
ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT/'00_nucleo/diagnosticos'
OUTPUT = D/'p1339-mutant-closed-state-inventory-closure.json'
assert not OUTPUT.exists()
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
sources = sorted((ROOT/'01_core/src/entities').rglob('*.rs'))
declarations = []
for path in sources:
    text = path.read_text()
    lines = text.splitlines()
    for i,line in enumerate(lines):
        m = re.match(r'^(?:pub(?:\([^)]*\))?\s+)?(enum|struct|type)\s+(\w+)\b',line)
        if not m:
            continue
        # Only column-zero declarations. Nested test modules/functions are indented.
        # Exclude explicitly test-gated top-level declarations as well.
        previous = '\n'.join(lines[max(0,i-4):i])
        if '#[cfg(test)]' in previous:
            continue
        kind,name = m.groups()
        depth = 0
        opened = False
        code = []
        block = False
        for j in range(i,len(lines)):
            raw = lines[j]
            out = ''
            pos = 0
            while pos < len(raw):
                if block:
                    end = raw.find('*/',pos)
                    if end < 0: break
                    block = False; pos = end+2; continue
                if raw.startswith('//',pos): break
                if raw.startswith('/*',pos): block=True; pos+=2; continue
                out += raw[pos]; pos += 1
            clean = out.rstrip()
            if clean.strip(): code.append({'line':j+1,'text':clean})
            depth += clean.count('{')-clean.count('}')
            opened |= '{' in clean
            if (opened and depth == 0) or (not opened and ';' in clean):
                break
        else:
            raise RuntimeError(f'Unclosed declaration: {path}:{i+1}')
        joined='\n'.join(x['text'] for x in code)
        members=[]
        if kind=='enum':
            # Every top-level variant starts at one Rust indentation level.
            for row in code[1:]:
                vm=re.match(r'^    ([A-Z]\w*)\s*(?:[,({]|$)',row['text'])
                if vm:members.append({'name':vm.group(1),'line':row['line']})
        # A variant identifier is a label only in that precise syntactic
        # position. Its payload may have the identical nominal type name.
        payload_lines=[re.sub(r'^    [A-Z]\w*\b', '    ', row['text']) if kind=='enum' and index else row['text']
                       for index,row in enumerate(code)]
        payload_text='\n'.join(payload_lines)
        declarations.append({'name':name,'kind':kind,'path':str(path.relative_to(ROOT)),
                             'source_sha256':sha(path),'first_line':i+1,'last_line':j+1,
                             'declaration':code,'direct_variants':members,
                             'type_tokens':sorted(set(re.findall(r'\b[A-Z][A-Za-z0-9_]*\b',payload_text)))})
by_name={}
for item in declarations:by_name.setdefault(item['name'],[]).append(item)
seeds=['Value','IntrospectedContent','Content','Args','ArgOccurrence','Selector','SourceDiagnostic',
       'Severity','Tracepoint','SourceResult','State','Counter','CounterKey','Func','FuncRepr',
       'ClosureRepr','ClosureParam','NativeFunc','NativeFuncWithEngine','ElementFunc','Module','ModuleInner']
excluded={'Arc','Box','Vec','HashMap','IndexMap','BTreeMap','HashSet','Option','Result','String','EcoString',
          'EcoVec','Self','Some','None','Ok','Err','Fn','FnMut','FnOnce','Send','Sync','Debug','Clone',
          'T','F','S','N','K','V','E','R','C','I','D','P','NonZeroUsize','NonZeroU8','NonZeroU16','NonZeroU32',
          'Cow','PhantomData','FxBuildHasher','Tracked','TrackedMut','SmallVec','OnceLock','LazyLock'}
pending=list(seeds); selected={}; unresolved=set()
while pending:
    name=pending.pop()
    if name in excluded:continue
    items=by_name.get(name)
    if not items:
        unresolved.add(name);continue
    for item in items:
        key=f"{item['path']}:{item['first_line']}:{item['name']}"
        if key in selected:continue
        selected[key]=item
        tokens=set(item['type_tokens'])-{item['name']}
        pending.extend(tokens)
head=subprocess.run(['git','rev-parse','HEAD'],cwd=ROOT,capture_output=True,text=True,check=True).stdout.strip()
record={
    'schema':'p1339-baseline-declaration-inventory-closure-v1','at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'batch':1,
    'ancestry':{name:sha(D/name) for name in ['p1339-mutant-closed-state-inventory.json','p1339-mutant-closed-state-inventory-supplement.json','p1339-budget-redesign-r1.json','p1339-verifier-budget-redesign-acceptance-r1.md']},
    'causal_correction':'Original reachability removed every token equal to any variant name globally. This successor removes only each enum variant label at its declaration position, keeping its payload type even when names coincide. Old outputs and their omissions are preserved.',
    'head':head,'state':'working tree before productive P1339 candidate; baseline declarations only',
    'authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
    'closed_harness_authority_sha256':sha(D/'p1339-closed-harness-authority.json'),
    'executor':'/root/p1336_tests','regime':'executado sem atestação de isolamento',
    'inherited_context':'P1336; permitted P1339 baseline signatures/carriers; no candidate read',
    'seeds':seeds,'declarations':list(selected.values()),
    'direct_seed_variant_counts':{n:len(by_name[n][0]['direct_variants']) for n in seeds if by_name[n][0]['kind']=='enum'},
    'unresolved_type_tokens':sorted(unresolved),'stdlib_or_generic_tokens_not_expanded':sorted(excluded),
    'name_ambiguities':{n:[{'path':i['path'],'line':i['first_line']} for i in items] for n,items in by_name.items() if len(items)>1 and any(i in selected.values() for i in items)},
    'limitations':[
        'This is a declaration inventory, not a compiler-resolved type graph or comparator-coverage proof.',
        'Reachability follows type-name tokens and retains every ambiguous declaration; aliases, external crates, traits and macro-generated carriers need explicit structural witness, never implicit coverage.',
        'All direct Value/Content/Args/Selector/SourceDiagnostic/State/Counter declarations are included with exact source lines and hashes. Bodies, PartialEq/Debug logic and expectations are excluded.',
        'No claim that every nested field is tested or covered; oracle and verifier must map concrete test or normative structural witness to each variant/field.',
    ],
    'extractor_sha256':sha(__file__),
}
with OUTPUT.open('x') as f:json.dump(record,f,ensure_ascii=False,indent=2);f.write('\n')
print(json.dumps({'output':str(OUTPUT),'sha256':sha(OUTPUT),'declarations':len(selected),
                  'seed_variant_counts':record['direct_seed_variant_counts'],'unresolved_type_tokens':len(unresolved)}))
