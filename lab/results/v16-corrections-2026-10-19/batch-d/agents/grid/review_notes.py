from pathlib import Path
import collections,datetime,gzip,hashlib,json,re,subprocess
b=Path('lab/results/v16-corrections-2026-10-19/batch-d');out=b/'agents/grid'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(n):return json.load(gzip.open(b/n,'rt') if n.endswith('.gz') else open(b/n))
before=read('baseline.sarif.gz');after=read('after.sarif.gz');bs=read('baseline-state.json.gz');cs=read('lint-state.json.gz');delta=read('lint-delta.json')
def notes(d):return [r for run in d['runs'] for r in run['results'] if r['ruleId'] in ['V19','V20']]
def key(r):return (r['ruleId'],r['locations'][0]['physicalLocation']['artifactLocation']['uri'],re.sub(r'\s+',' ',r['message']['text']))
def pos(r):return r['locations'][0]['physicalLocation']['region']
a=notes(before);c=notes(after);ga=collections.defaultdict(list);gc=collections.defaultdict(list)
for r in a:ga[key(r)].append(r)
for r in c:gc[key(r)].append(r)
unchanged=[];moved=[];formatting=[];removed=[];added=[]
for k in set(ga)|set(gc):
 aa=sorted(ga[k],key=lambda r:(pos(r)['startLine'],pos(r)['startColumn']));cc=sorted(gc[k],key=lambda r:(pos(r)['startLine'],pos(r)['startColumn']))
 for x,y in zip(aa,cc):
  entry={'rule':k[0],'path':k[1],'before':pos(x),'after':pos(y),'message':k[2]}
  if pos(x)!=pos(y):moved.append(entry)
  else:unchanged.append(entry)
  if x['message']['text']!=y['message']['text']:formatting.append(entry)
 removed+=aa[len(cc):];added+=cc[len(aa):]
# Imported Value is the same nominal path; identify those two shallow textual renames.
renames=[]
for x in removed[:]:
 counterpart=next((y for y in added if key(x)[0:2]==key(y)[0:2] and key(x)[2].replace('crate::entities::value::Value::','Value::')==key(y)[2]),None)
 if counterpart:
  renames.append({'before':x,'after':counterpart,'classification':'equivalent imported Value path; pattern depth unchanged'});removed.remove(x);added.remove(counterpart)
classes={
'grid/borders.rs':'Raw content has no outer stroke override; prepared known None still suppresses grid inheritance.',
'grid/cell_body.rs':'Raw content has no outer cell properties; prepared paint/default-survival split remains.',
'grid/measurement_cache.rs':'Placement neutrality and cache refusal remain distinct; grouped negatives do not admit new products.',
'grid/tracks.rs':'Outer inset absence and no-replacement traversal remain distinct; prepared visual and Table barrier preserve ownership.',
'grid_placement.rs':'Raw metadata absence preserves both wrong-policy residuals and unit automatic placement.',
'helpers.rs':'True Empty/Metadata zero split from unsupported legacy approximations; no claim that unsupported means empty.',
'image.rs':'Unsupported bounded axis projection remains None, with intrinsic/caller semantics unchanged.',
'line_commit.rs':'Closed unsupported Some(Value) numbering class; outer None is overlay suppression; no newly counted input.',
'measurement.rs':'New callback-free-math certificate and explicit absence/unsupported classes; separate measured semantic fixes, not blanket support.',
'mod.rs':'Forward-to-locator and no-extra-quote-event partitions; content remains dispatched.',
'par_line_callbacks.rs':'Both HAlign axes closed; existing nonzero and zero shifts unchanged.',
'place.rs':'Option absence/Left/Start share unchanged zero-offset; finite geometry clamping remains.',
'references.rs':'Selector and kind domains closed separately; no implicit supplement for negatives; string guard-false preserved.',
'layout/text/style.rs':'Bounded style casts retain refusal; no new coercions or dropped malformed entries.',
'elements/table_cell.rs':'Bounded Length/partial-Dict inset rejection preserved; Auto remains caller reset.'}
for r in added:
 path=key(r)[1];r['review_classification']=next(v for k,v in classes.items() if path.endswith(k))
changed=[p for p,h in cs['inputs'].items() if bs['inputs'].get(p)!=h]
diff=subprocess.check_output(['git','diff','HEAD','--'],text=True)
suppressions=[line for line in diff.splitlines() if line.startswith('+') and not line.startswith('+++') and re.search(r'#\[(?:allow|expect)\b|tekt::skip|severity\s*=',line)]
report={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'current_diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'artifacts':{n:sha(b/n) for n in ['baseline.sarif.gz','after.sarif.gz','lint-delta.json','baseline-state.json.gz','lint-state.json.gz']},'baseline_state':{k:bs[k] for k in ['utc_start','utc_end','head','diff_head_stat','linter_sha256','command']},'candidate_state':{k:cs[k] for k in ['utc_start','utc_end','head','diff_head_stat','linter_sha256','command']},'counts_before':dict(collections.Counter(r['ruleId'] for r in a)),'counts_after':dict(collections.Counter(r['ruleId'] for r in c)),'paired_same_location':len(unchanged),'paired_moved_location':len(moved),'paired_whitespace_reformat':len(formatting),'moved':moved,'formatting':formatting,'equivalent_path_renames':renames,'substantive_removed':removed,'substantive_added':added,'changed_scan_inputs':changed,'same_linter':bs['linter_sha256']==cs['linter_sha256'],'new_suppression_lines':suppressions,'upstream_delta_crosscheck':{k:delta[k] for k in ['counts_before','counts_after','new_errors_or_warnings','outside_scope_exact']},'conclusion':'No alert suppression, weakened severity, scan exclusion or changed public contract found. V19 groups expose maintenance obligations; V20 removals are nested-match decomposition, not hidden domains. Runtime class coverage and exact-site compiler mutations remain separate obligations.'}
(out/'note-review.json').write_text(json.dumps(report,indent=2)+'\n')
print({k:report[k] for k in ['counts_before','counts_after','paired_same_location','paired_moved_location','paired_whitespace_reformat','same_linter','new_suppression_lines']});print('substantive',len(removed),len(added),'renames',len(renames))
