from pathlib import Path
import gzip,json,hashlib,re,subprocess,shutil,datetime,importlib.util
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
before=json.load(gzip.open(out/'before.json.gz'));root=Path(before['source']);gitroot=Path('/tmp/tekt-linter-none-git')
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
green=json.load(open(out/'green-result.json'));assert green.get('end') and 'candidate_hashes' in green
assert all(x['exit']==0 for x in green['commands'])
current={str(p.relative_to(root)):sha(p) for p in root.rglob('*') if p.is_file()}
assert current==green['candidate_hashes']
previous=before['linter_hashes'];assert not set(previous)-set(current)
changed=sorted(p for p in current if previous.get(p)!=current[p])
docs=json.load(open(out/'docs-before-code.json'))
expected={'03_infra/rs_parser.rs','00_nucleo/prompts/parsers/rust.md','00_nucleo/assessments/0037-self-complexity.md','00_nucleo/assessments/0037-self-complexity.tsv'}
expected.update(docs['sources']);expected.update(x['owner'] for x in docs['sources'].values())
expected.update('tests/fixtures/'+n+'/00_nucleo/prompts/.gitkeep' for n in ['v01_fail','v01b_fail','vexcl_pass','vnest_fail'])
assert set(changed)==expected,(len(changed),set(changed)^expected)
public=lambda s:re.findall(r'^\s*pub(?:\([^)]*\))?\s+[^\n]+',s,re.M)
assert public(before['files']['03_infra/rs_parser.rs'])==public((root/'03_infra/rs_parser.rs').read_text())
norm=lambda s:'\n'.join(x for x in s.splitlines() if not x.lstrip().startswith(('///','//!')))
for source,item in docs['sources'].items():assert norm((root/source).read_text())==norm(item['before'])
spec=importlib.util.spec_from_file_location('lineage',Path.cwd()/'tools/ci/check_reverse_lineage.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
for source in [p for p in changed if p.endswith('.rs')]:assert module.check_pair(root,root/source)==[],source
assert all(sha(p)==h for p,h in before['project_hashes'].items())
# Local review commit only: canonical checkout and remote are unchanged.
assert not gitroot.exists()
subprocess.run(['git','clone','--shared','--quiet','--no-checkout','/repos/Antigravity/tekt-linter',str(gitroot)],check=True)
git=lambda *args:subprocess.check_output(['git','-C',str(gitroot),*args],text=True)
branch='codex/v16-none-prelude'
git('remote','set-url','origin','https://github.com/Dikluwe/tekt-linter.git')
git('checkout','--quiet','-b',branch,before['base_commit'])
for path in changed:
 dest=gitroot/path;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/path,dest)
git('add','--',*[p for p in changed if not p.endswith('.gitkeep')])
git('add','--force','--',*[p for p in changed if p.endswith('.gitkeep')])
assert set(git('diff','--cached','--name-only').splitlines())==expected
name=subprocess.check_output(['git','-C','/repos/Antigravity/tekt-linter','config','user.name'],text=True).strip()
email=subprocess.check_output(['git','-C','/repos/Antigravity/tekt-linter','config','user.email'],text=True).strip()
git('-c','user.name='+name,'-c','user.email='+email,'commit','--no-gpg-sign','--quiet','-m','fix(rust): distinguish None constructors from V16 bindings')
commit=git('rev-parse','HEAD').strip();tree=git('rev-parse','HEAD^{tree}').strip()
assert git('rev-parse','HEAD^').strip()==before['base_commit']
assert not git('status','--porcelain')
for path,h in current.items():assert sha(gitroot/path)==h,path
(out/'candidate.patch').write_text(git('diff','--binary',before['base_commit'],'HEAD'))
git('bundle','create',str(out/'candidate.bundle'),before['base_commit']+'..HEAD')
subprocess.run(['git','-C',str(gitroot),'archive','--format=tar.gz','--output='+str(out/'candidate-source.tar.gz'),'HEAD'],check=True)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'base_commit':before['base_commit'],'commit':commit,'tree':tree,'branch':branch,'checkout':str(gitroot),'changed':changed,'file_hashes':{p:current[p] for p in changed},'source_hashes':current,'binary':green['binary'],'project_sources_unchanged':True,'reverse_changed_pairs_valid':True,'documentation_only_in_11_sources':True,'canonical_checkout_unchanged':subprocess.check_output(['git','-C','/repos/Antigravity/tekt-linter','status','--porcelain'],text=True)=='','published':False,'consumer_pin_changed':False}
assert record['canonical_checkout_unchanged']
(out/'package-result.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({k:record[k] for k in ['commit','tree','branch','changed','published','consumer_pin_changed']}),flush=True)
