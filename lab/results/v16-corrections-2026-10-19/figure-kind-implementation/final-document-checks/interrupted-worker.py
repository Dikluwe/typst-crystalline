"""Check current report links and repository path/lineage gates after consolidation."""
from pathlib import Path
import importlib.util
import json
import re
import subprocess
from urllib.parse import unquote

ROOT=Path.cwd();P=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
out=P/'final-document-checks'
assert not out.exists();out.mkdir()
inputs=v.frozen_inputs()
record={'utc_start':v.utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':inputs,'documents':[],'commands':[]}
documents=[P/'README.md',ROOT/'00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md']
missing=[]
for document in documents:
    links=[]
    for raw in re.findall(r'\[[^\]]*\]\(([^)]+)\)',document.read_text()):
        target=raw.strip().strip('<>')
        if re.match(r'^[a-zA-Z][a-zA-Z0-9+.-]*:',target) or target.startswith('#'):
            continue
        target=unquote(target.split('#',1)[0])
        path=(document.parent/target).resolve()
        links.append({'target':raw,'resolved':str(path),'exists':path.exists()})
        if not path.exists():missing.append({'document':str(document),'target':raw})
    record['documents'].append({'path':str(document.relative_to(ROOT)),'sha256':v.digest(document),'links':links})
record['missing_links']=missing
assert not missing,missing
commands=[('layout',['python3','lab/tools/ci/check_mature_layout.py']),('strict-lineage',['lab/target/linter/debug/crystalline-lint','.','--checks','v1,v5,v15,v26','--fail-on','warning']),('reverse-lineage',['python3','lab/tools/ci/check_reverse_lineage.py']),('diff-check',['git','diff','--check'])]
for name,command in commands:
    start=v.utc();run=subprocess.run(command,capture_output=True,text=True)
    (out/(name+'.log')).write_text(run.stdout+run.stderr)
    record['commands'].append({'name':name,'argv':command,'utc_start':start,'utc_end':v.utc(),'exit':run.returncode})
    assert run.returncode==0,(name,run.stderr)
record.update(utc_end=v.utc(),changed_inputs=[k for k in set(inputs)|set(v.frozen_inputs()) if inputs.get(k)!=v.frozen_inputs().get(k)])
(out/'receipt.json').write_text(json.dumps(record,indent=2)+'\n')
assert not record['changed_inputs']
print('Current document links and final path/lineage checks passed')
