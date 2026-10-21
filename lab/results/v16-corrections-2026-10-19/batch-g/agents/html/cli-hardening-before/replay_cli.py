"""Replay immutable HTML language policies against a source-bound G build; no Cargo."""
from pathlib import Path
import argparse
import datetime
import hashlib
from html.parser import HTMLParser
import importlib.util
import json
import re
import subprocess

ROOT=Path.cwd()
HERE=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()

class DOM(HTMLParser):
    void={'area','base','br','col','embed','hr','img','input','link','meta','param','source','track','wbr'}
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root={'tag':'#document','attrs':[],'children':[]};self.stack=[self.root]
    def handle_starttag(self,tag,attrs):
        node={'tag':tag,'attrs':sorted(attrs),'children':[]}
        self.stack[-1]['children'].append(node)
        if tag not in self.void:self.stack.append(node)
    def handle_startendtag(self,tag,attrs):
        self.handle_starttag(tag,attrs)
        if tag not in self.void:self.handle_endtag(tag)
    def handle_endtag(self,tag):
        for index in range(len(self.stack)-1,0,-1):
            if self.stack[index]['tag']==tag:
                del self.stack[index:];return
    def handle_data(self,data):
        children=self.stack[-1]['children']
        if children and isinstance(children[-1],str):children[-1]+=data
        else:children.append(data)

def semantic_dom(html):
    parser=DOM();parser.feed(html);parser.close()
    def normalize(node):
        if isinstance(node,str):return node
        children=[normalize(child) for child in node['children']]
        if node['tag']=='table':
            grouped=[];rows=[]
            def flush():
                if rows:grouped.append({'tag':'tbody','attrs':[],'children':list(rows)});rows.clear()
            for child in children:
                if isinstance(child,dict) and child['tag']=='tr':rows.append(child)
                else:flush();grouped.append(child)
            flush();children=grouped
        return {'tag':node['tag'],'attrs':node['attrs'],'children':children}
    return normalize(parser.root)

def diagnostics(stderr):
    result=[]
    for line in stderr.splitlines():
        match=re.match(r'^(error|warning): (.*)$',line)
        if match:
            result.append({'severity':match[1],'message':match[2],'hints':[],'origin':None,'trace':[]});continue
        if not result:continue
        if '= hint: ' in line:result[-1]['hints'].append(line.split('= hint: ',1)[1])
        elif '\u250c\u2500 ' in line:
            origin=re.search(r':(\d+):(\d+)\s*$',line)
            if origin:result[-1]['origin']=[int(origin[1]),int(origin[2])]
        elif line.startswith('help:') or line.startswith('  = help:'):result[-1]['trace'].append(line)
    return result

def payload(diags,severity):
    return [{k:d[k] for k in ('severity','message','hints')} for d in diags if d['severity']==severity]

parser=argparse.ArgumentParser()
parser.add_argument('--build-receipt',type=Path,required=True)
parser.add_argument('--product',type=Path,default=Path('lab/target/product/debug/typst'))
parser.add_argument('--output',type=Path,required=True)
args=parser.parse_args()
assert not args.output.exists();args.output.mkdir(parents=True)
policy_path=HERE/'cli-policies.json';policy_sha=sha(policy_path);policy=json.loads(policy_path.read_text())
for name,digest in policy['oracle_receipt_sha256'].items():assert sha(ROOT/name)==digest
build=json.loads(args.build_receipt.read_text());assert build['exit']==0 and not build['changed_inputs']
assert sha(args.product)==build['product_sha256']
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
assert build['inputs']==v.frozen_inputs()
record={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),
        'binary_sha256':sha(args.product),'build_receipt_sha256':sha(args.build_receipt),
        'inputs':build['inputs'],'policy_sha256':policy_sha,'runner_sha256':sha(Path(__file__)),
        'oracle_commit':policy['oracle_commit'],'oracle_tree':policy['oracle_tree'],
        'oracle_sha256':policy['oracle_sha256'],'cases':[]}
for case in policy['cases']:
    source=args.output/(case['name']+'.typ');source.write_text(case['source'])
    assert sha(source)==case['source_sha256']
    output=args.output/(case['name']+('-out' if case['bundle'] else '.html'))
    command=[str(args.product),'compile','--features','html,bundle' if case['bundle'] else 'html',
             '--format','bundle' if case['bundle'] else 'html',str(source),str(output)]
    started=utc();run=subprocess.run(command,text=True,capture_output=True,timeout=60)
    html_files={str(p.relative_to(output)):p.read_text() for p in output.rglob('*.html')} if output.is_dir() else ({output.name:output.read_text()} if output.exists() else {})
    observed_html=next(iter(html_files.values())) if len(html_files)==1 else None
    actual_diagnostics=diagnostics(run.stderr);oracle_diagnostics=diagnostics(case['oracle']['stderr'])
    exit_match=run.returncode==case['oracle']['exit']
    dom_match=run.returncode==0 and case['oracle']['exit']==0 and observed_html is not None and case['oracle']['html'] is not None and semantic_dom(observed_html)==semantic_dom(case['oracle']['html'])
    error_match=payload(actual_diagnostics,'error')==payload(oracle_diagnostics,'error')
    warning_match=payload(actual_diagnostics,'warning')==payload(oracle_diagnostics,'warning')
    main_match=exit_match and (dom_match if case['oracle']['exit']==0 else error_match and not html_files)
    required=case['policy']['class']=='required-closure'
    required_pass=main_match and (warning_match or not case['policy']['warning_payload_required'])
    record['cases'].append({'name':case['name'],'policy':case['policy'],'source_sha256':sha(source),
        'argv':command,'utc_start':started,'utc_end':utc(),'exit':run.returncode,'stdout':run.stdout,
        'stderr':run.stderr,'html_files':html_files,'semantic_dom':semantic_dom(observed_html) if observed_html else None,
        'diagnostics':actual_diagnostics,'oracle_diagnostics':oracle_diagnostics,
        'checks':{'exit':exit_match,'semantic_dom':dom_match,'error_payload':error_match,'warning_payload':warning_match,
                  'complete_diagnostic_fields':actual_diagnostics==oracle_diagnostics},
        'required':required,'required_pass':required_pass if required else None,
        'strict_comparison':main_match and actual_diagnostics==oracle_diagnostics})
record.update(utc_end=utc(),required_failures=[c['name'] for c in record['cases'] if c['required'] and not c['required_pass']],
              strict_failures=[c['name'] for c in record['cases'] if not c['strict_comparison']],
              explicit_gap_cases=[c['name'] for c in record['cases'] if not c['required']])
after=v.frozen_inputs();record['changed_inputs']=[n for n in after.keys()|build['inputs'].keys() if after.get(n)!=build['inputs'].get(n)]
record['binary_unchanged']=sha(args.product)==record['binary_sha256']
record['policy_unchanged']=sha(policy_path)==policy_sha
assert not record['changed_inputs'] and record['binary_unchanged'] and record['policy_unchanged']
(args.output/'receipt.json').write_text(json.dumps(record,indent=2)+'\n')
print('HTML required',sum(c['required'] and c['required_pass'] for c in record['cases']),'/',sum(c['required'] for c in record['cases']),
      '; explicit Link gaps',len(record['explicit_gap_cases']),'; strict differences',record['strict_failures'])
raise SystemExit(1 if record['required_failures'] else 0)
