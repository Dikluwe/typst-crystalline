"""One-shot manifested path migration; refuses a previously migrated source tree."""
from pathlib import Path
import re, json, os, hashlib, gzip, subprocess, datetime, shutil
R=Path('/repos/Antigravity/typst-crystalline'); O=R/'lab/results/prompt-layout-migration-2026-10-18'
inv=json.loads((R/'lab/results/prompt-organization-audit-2026-10-18/inventory.json').read_text())
a=json.loads((R/'lab/results/prompt-organization-audit-2026-10-18/semantic-name-candidates.json').read_text())['candidates']
b=json.loads(Path('/tmp/tekt-remaining-rust-renames.json').read_text())['candidates']
renames={x['current_source']:x['proposed_source'] for x in a+b}
assert len(renames)==85 and len(set(renames.values()))==85
for old,new in renames.items():assert (R/old).is_file() and not (R/new).exists(),(old,new)
def owner_path(source):
 parts=list(Path(source).parts);layer=parts.pop(0).split('_',1)[1]
 if parts[0]=='src':parts.pop(0)
 return str(Path('00_nucleo/prompts')/layer/Path(*parts).with_suffix('.md'))
owners={x['source']:x['owner'] for x in inv['owners']}
prompt_map={owner:owner_path(renames.get(source,source)) for source,owner in owners.items()}
assert len(prompt_map)==991 and len(set(prompt_map.values()))==991
for old,new in prompt_map.items():
 assert (R/old).is_file()
 assert not (R/new).exists() or new in prompt_map,(old,new)
fullmap={**renames,**prompt_map}
pattern=re.compile('|'.join(re.escape(x) for x in sorted(fullmap,key=len,reverse=True)))
def canonical(text):return pattern.sub(lambda m:fullmap[m[0]],text)
original={s:(R/s).read_text() for s in set(owners)|set(renames)}
with gzip.open(O/'before-rust.json.gz','wt') as f:json.dump(original,f)
with gzip.open(O/'before-prompts.json.gz','wt') as f:json.dump({p:(R/p).read_text() for p in prompt_map},f)
def resolve(source,rel):
 p=Path(source); candidates=[p.parent/rel]
 if p.name not in ('mod.rs','lib.rs','main.rs'):candidates.append(p.parent/p.stem/rel)
 found={str(Path(os.path.normpath(c))) for c in candidates if str(Path(os.path.normpath(c))) in renames}
 assert len(found)<=1,(source,rel,found)
 return next(iter(found),None)
module_changes={}
for source,txt in original.items():
 for m in re.finditer(r'\bmod\s+(p\d+\w*)\s*;',txt):
  stem=m[1];attrs=txt[max(0,m.start()-240):m.start()]
  am=re.search(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\n]*\]\s*)*$',attrs)
  rel=am[1] if am else stem+'.rs';old=resolve(source,rel)
  assert old,(source,stem,rel)
  module_changes.setdefault(source,{})[stem]=Path(renames[old]).stem
assert sum(len(x) for x in module_changes.values())==52
# Markdown links are resolved at the old location before any paths are moved.
link_re=re.compile(r'(?P<start>\]\()(?P<angle><)?(?P<url>[^\s)>]+)>?(?P<end>\))')
def markdown_links(text,old,new):
 def repl(m):
  url=m['url']
  if url.startswith(('#','/')) or re.match(r'\w[\w+.-]*:',url):return m[0]
  path,sep,anchor=url.partition('#'); target=Path(os.path.normpath(str(Path(old).parent/path)))
  target_s=str(target)
  # Only relocate real paths (including directory links); unresolved links are audited later.
  if not (R/target).exists():return m[0]
  mapped=fullmap.get(target_s,target_s)
  relative=os.path.relpath(mapped,Path(new).parent).replace(os.sep,'/')
  if sep:relative+='#'+anchor
  return ']('+('<'+relative+'>' if m['angle'] else relative)+')'
 return link_re.sub(repl,text)
# L0 first, including delegation names for the renamed out-of-line test modules.
prepared={}
for source,old in owners.items():
 new=prompt_map[old];txt=(R/old).read_text()
 txt=canonical(markdown_links(txt,old,new))
 if source in module_changes:
  for before,after in module_changes[source].items():
   txt=re.sub(r'\b'+re.escape(before)+r'\b',after,txt)
  names=', '.join('`'+x+'`' for x in module_changes[source].values())
  txt+='\n## Owned test module declarations\n\nThis consumer declares the following separately owned test modules: '+names+'. Their names describe their test responsibilities; each test module retains its own L0 owner. This module delegates the test bodies and does not absorb their ownership.\n'
 prepared[new]=txt
for old in prompt_map:(R/old).unlink()
for new,txt in prepared.items():(R/new).parent.mkdir(parents=True,exist_ok=True);(R/new).write_text(txt)
# Then rewrite and move source consumers; only paths and out-of-line module names change.
source_result={}
for old,txt in original.items():
 new=renames.get(old,old)
 def attr(m):
  target=resolve(old,m[1])
  if target is None:return m[0]
  return '#[path = "'+os.path.relpath(renames[target],Path(new).parent).replace(os.sep,'/')+'"]'
 txt=re.sub(r'#\[path\s*=\s*"([^"]+)"\]',attr,txt)
 for before,after in module_changes.get(old,{}).items():
  txt=re.sub(r'(\bmod\s+)'+re.escape(before)+r'(\s*;)',lambda m:m[1]+after+m[2],txt)
 txt=canonical(txt);source_result[new]=txt
for old in renames:(R/old).unlink()
for new,txt in source_result.items():(R/new).write_text(txt)
# Active references only. Historical results, fixtures and nuclei retain exact bytes.
tracked=subprocess.check_output(['git','ls-files','-z'],cwd=R).decode().split('\0')
reference_changes=[]
for old in tracked:
 if not old or old in original or old in prompt_map:continue
 p=R/old
 if not p.is_file() or p.is_symlink():continue
 if old.startswith('lab/') and not old.startswith(('lab/tools/','lab/benches/')):continue
 if '/_nuclei/' in old:continue
 if old.startswith('lab/tools/ci/test_'):continue
 if p.suffix not in {'.md','.toml','.yml','.yaml','.py','.sh','.rs','.json'}:continue
 try:txt=p.read_text()
 except UnicodeError:continue
 updated=canonical(txt)
 if p.suffix=='.md':updated=canonical(markdown_links(txt,old,old))
 if updated!=txt:p.write_text(updated);reference_changes.append(old)
# Remove empty former prompt directories, never nucleus data.
for p in sorted((R/'00_nucleo/prompts').rglob('*'),key=lambda x:len(x.parts),reverse=True):
 if p.is_dir() and not any(p.iterdir()):p.rmdir()
manifest={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkpoint':inv['head'],'tag':'tekt-before-prompt-layout-2026-10-18','source_renames':renames,'prompt_paths':prompt_map,'module_declarations':module_changes,'reference_changes':reference_changes,'prompt_moves':sum(a!=b for a,b in prompt_map.items()),'source_moves':len(renames),'note':'Pure path/module-name migration; no test bodies, behavioral algorithms or nucleus bytes changed. Inline test names and provenance fixtures retain historical IDs.'}
(O/'migration-map.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Migrated',manifest['source_moves'],'Rust files,',manifest['prompt_moves'],'prompt paths; module declarations',sum(len(x) for x in module_changes.values()))
