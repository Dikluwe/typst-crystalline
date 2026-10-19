from pathlib import Path
import re,json,hashlib,subprocess,datetime
R=Path.cwd(); D=R/'lab/results/v16-corrections-2026-10-19/figure-kind/agents/contracts'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def variants(p,name):
 s=p.read_text().split('pub enum '+name+' {',1)[1].split('\n}',1)[0]
 return re.findall(r'^    (\w+)(?=\(|\s*\{|,)',s,re.M)
v=variants(R/'01_core/src/entities/content.rs','Content'); entries={}
def put(names,policy,fields='',reason='Semantic source fields; never invoke callbacks or inspect captured environments.'):
 for n in names.split():
  assert n in v and n not in entries,n
  entries[n]={'variant':n,'policy':policy,'ordered_fields':fields,'reason':reason}
put('Image Raw','inferable-root','no Content-valued fields after nonmatching root','Auto recognizes exact native identity; a nonmatching explicit-kind root has no semantic Content fields.')
put('Table','root-then-fields','ordinary raw children in source order; separated header/footer/caption retained after that synthetic adapter view','Auto succeeds immediately at Table; explicit target of another kind continues into declared children, never resolved visual plan. Ordinary-cell target order is required; native constructor extracts and can merge headers, so original mixed header/cell order for non-Table supplement targets is explicit bounded debt. No Table lexical carrier added.')
put('Source ShowRecipe Located','transparent','body','Source/recipe/location transport is not an additional language element; inspect retained body without executing recipe/callback.')
put('Realized','semantic-origin','original','Inspect original language contribution; do not infer from later show-produced visual content.')
put('Sequence MathSequence','children','children in source order')
put('Par SmallCaps Heading Title Strong Emph ListItem EnumItem Link Equation MathStretch MathDelimited MathLr MathMid MathCancel MathUnderline MathOverline MathClassOverride MathLimitsOverride MathStyled Label Transform GridHeader GridFooter GridCell Align Place Styled PdfArtifact Underline Strike Overline Pad Hide Boxed Block Footnote TableHeader TableFooter Repeat Columns HtmlFrame','body','body')
put('HtmlElem','optional-body','body iff HtmlBody::Content; Unset/None do not contribute')
put('MathFrac','ordered-fields','num, den')
put('MathAttach','ordered-fields','base, present t, present b, present tl, present bl, present tr, present br','Only lexically present slots; not synthesized placement or resolved visual slots. Order verified in pinned math/attach.rs:23–48.')
put('MathRoot','ordered-fields','present index, radicand')
put('MathOverbrace MathUnderbrace MathUnderbracket MathUnderparen MathUndershell MathOverbracket MathOverparen MathOvershell','ordered-fields','body, explicit present annotation')
put('MathMatrix','array-fields','rows in row-major source order')
put('MathCases MathVec','array-fields','children in source order')
put('MathAccent','body','base')
put('MathUnderover','ordered-fields','base, present under, present over','Legacy synthetic aggregate has no single upstream element; preserve all represented semantic source slots, not render products.')
put('MathOp','body','text')
put('Figure','ordered-fields','body, present caption, resolved supplement Content if resolved_supplement is Some, otherwise lexical FigureSupplement::Content; never both','First inferable descendant wins. Do not descend into the function value, kind identity, or private preparation record.')
put('FigureCaption','ordered-fields','present separator, body','Field declaration order is position (noncontent), separator, body; preserve explicit known fields only.')
put('Ref Cite','optional-fields','present supplement','Reference/citation supplement is public Content, unlike the static visual visitor policy.')
put('Outline','optional-fields','explicit title only','Generated realized_body/captured recipes are not source language fields.')
put('OutlineEntry','ordered-fields','element, present fill','Generated prefix/location is not a public stored semantic child.')
put('Grid','ordered-fields','source_order descriptors resolve Cell/Header/Footer/HLine/VLine in original positional order; None synthetic legacy order cells,header,footer','New approved GridChildOrder transport retains actual source ordering without duplicated Content or repeated map callbacks. Before traversal validate complete in-bounds exactly-once descriptor coverage and Header/Footer presence; malformed Some returns SourceResult error invalid grid source order at Source span, never partial result/fallback. Never use layout placement order as semantic source order.')
put('PageRun','ordered-fields','static supplement, static header, static footer, static background, static foreground, body','Only explicit Content payloads represented by public page properties. No running callbacks; Order follows pinned layout/page.rs:324,367,404,474,492,501, not rendering order.')
put('Terms','array-fields','items in source order')
put('TermItem','ordered-fields','term, description')
put('Quote','ordered-fields','body, present attribution')
put('Document','ordered-fields','present title, body','Title precedes body in language constructor fields; authors/keywords/metadata are noncontent. Document is a distinct resource; no asset I/O.')
put('Stack','array-fields','children in source order')
put('TableCell','body','raw declared body','Do not use resolved/standalone visual; fields x/y/etc are noncontent and function-valued style properties remain opaque.')
put('Bibliography','optional-fields','explicit title only','No CSL rendering, record traversal, or I/O.')
put('FootnoteEntry','ordered-fields','note, present separator','Order verified in pinned model/footnote.rs:240,264; do not generate note content.')
put('Metadata','value-field','value','Descend through Content/LocatedContent and recursive Arrays only; Dictionaries are opaque even when nested in Arrays.')
put('StateUpdate','opaque-internal','','Pinned state.rs:401 marks update internal; audit state-update-fields proves Set(Table) stays opaque even though Rust contains Value. Public key is noncontent. Do not execute Func or inspect Set payload.')
put('State','opaque-internal','', 'Synthetic state initial-value marker has no public native Content field schema; preserving opacity is an explicit extension limitation, not proof its stored Value is childless.')
put('ContextBlock StateDisplay CounterDisplayCallback','opaque-callback','', 'Do not execute callback or descend through closure captures; later produced content is outside naive semantic inference.')
put('Dynamic','opaque-extension','', 'Current Element/DynElement exposes get_field but no enumeration; do not call dyn_map_content. Extension field inference remains explicit debt, not certified parity.')
put('SetPage','opaque-style-marker','', 'A progressive style delta is not a public Content semantic element; do not walk style map payloads.')
put('Empty Text Space SpaceFromNewline Parbreak MathIdent MathText MathPrimes MathAlignPoint Linebreak CounterDisplay CounterUpdate Shape Curve Flush Divider Asset SmartQuote PdfAttach HSpace VSpace Pagebreak Colbreak GridHLine GridVLine TableHLine TableVLine','no-content-fields','', 'Current public fields are noncontent values or resource/counter actions; no semantic child and no callback execution. Invalid synthetic geometry Value contents are not valid semantic dimensions.')
assert set(v)==set(entries),(set(v)-set(entries))
vv=variants(R/'01_core/src/entities/value.rs','Value'); ve=[]
for n in vv:
 policy={'Content':'recurse semantic content','LocatedContent':'use authoritative stored fields when available; known-empty stays authoritative; otherwise underlying semantic content','Array':'recurse values in order including nested Arrays','Dict':'opaque; no recursion into values, including nested Arrays'}.get(n,'opaque non-Content/non-Array value; no callback or contained runtime-object traversal')
 ve.append({'variant':n,'policy':policy})
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'classification':'proposed closed semantic inspection partition; extension/representation residuals explicit, not runtime parity claims','content_source_sha256':sha(R/'01_core/src/entities/content.rs'),'value_source_sha256':sha(R/'01_core/src/entities/value.rs'),'content':list(entries.values()),'value':ve}
(D/'semantic-partition.json').write_text(json.dumps(meta,indent=2)+'\n')
roles={
'entities/elements/figure':'Typed lexical/occurrence kind and supplement; pure Eq/Hash/getter/map/payload contracts.',
'entities/content/construction':'Keep existing public adapter signature; initialize new fields and existing numbering style transport.',
'entities/content/traversal':'Copy every new field in existing Figure reconstruction; do not replace generic visitor policies.',
'entities/element_payload':'Typed Figure kind and resolved supplement payload; migrate fixtures.',
'compiler/stdlib/figure_image':'Stop constructor inference; typed validation/presence/supplement cast/numbering transport; preserve image contract.',
'compiler/eval/observation/elements':'Exhaustive observation of every new Figure field and kind/supplement nominal states.',
'compiler/eval/bindings/field_access/content_methods':'Known Figure kind/supplement lexical and resolved projection, exact ordered fields, authoritative snapshots.',
'compiler/eval/rules/set':'Transport kind/supplement and numbering values in existing style keys; no repeated evaluation.',
'compiler/eval/rules/apply':'Preview selects rules; fold matched show-set then prepare final occurrence before callback/replacement; preserve guard and transaction.',
'compiler/eval/selector_matching':'Figure kind field comparison uses exact element function identity, never Func name equality; other selectors untouched.',
'compiler/eval/operators/equality':'Figure typed-kind filter equality uses exact identity; retain generic Func equality and separate language/carrier comparison.',
'compiler/realize/mod':'Register private figure and figure_kind units; preserve existing public facade and causal routes.',
'compiler/realize/recipes':'Reached Figure kind preview and final preparation including no-rule path; no hidden descendant prewalk.',
'compiler/layout/measurement':'Migrate direct Figure test literals without changing measurement behavior.',
'compiler/introspect/materialize_time':'Reconstruct all typed Figure fields directly instead of lossy legacy adapter; preserve temporal body/caption/supplement traversal and origins.',
'compiler/introspect/locatable':'Preserve occurrence tests and migrate legacy fixture kind intent if necessary.',
'entities/elements/grid':'Add exact GridChildOrder and source_order; copy metadata through maps while each existing child transforms once; no duplicated source Content.',
'compiler/stdlib/layout/grid':'Capture positional GridChildOrder before extraction; preserve all existing validation, phase, diagnostics and rendering.',
'entities/content/tests/representation':'Migrate Grid source_order fixture and preserve size/representation assertions.',
'entities/content/tests/traversal':'Migrate Grid source_order fixture and verify map count/order contracts.',
'compiler/layout/placement':'Copy Grid source_order on transient placement reconstruction; preserve geometry and source ordering.',
'compiler/introspect/convergence':'Migrate typed payload fixture; verify new identity changes convergence.',
'compiler/introspect/extract_payload':'Migrate payload fixture expectations; maintain entity delegation.',
'compiler/introspect':'Migrate legacy fixture expectations only as justified by typed payload and corrected count contracts.',
'compiler/layout/tests':'Regressions for actual caption/reference output and typed/location counter expectations.',
}
new={'compiler/realize/figure_kind':'Feature-owned closed semantic Content/Value traversal and exact kind identity/target lookup.','compiler/realize/figure':'Feature-owned pure kind preview and final occurrence kind/supplement preparation.'}
items=[]
for path,role in roles.items():
 s=R/'01_core/src'/f'{path}.rs'; p=R/'00_nucleo/prompts/core'/f'{path}.md'; assert s.exists() and p.exists(),path
 h=re.search(r'@prompt-hash (\w+)',s.read_text()).group(1)
 items.append({'source':str(s.relative_to(R)),'owner':str(p.relative_to(R)),'status':'existing','source_sha256':sha(s),'owner_sha256':sha(p),'source_prompt_hash':h,'role':role})
for path,role in new.items():items.append({'source':f'01_core/src/{path}.rs','owner':f'00_nucleo/prompts/core/{path}.md','status':'new-only-after-approval','role':role})
(D/'owner-manifest.json').write_text(json.dumps({'utc':meta['utc'],'head':meta['head'],'branch':meta['branch'],'diff_head_stat':meta['diff_head_stat'],'scope':'Kind/carrier/supplement/realization contract; union with independent counter owner manifest required for final integrated package. No global resealing.','pairs':items},indent=2)+'\n')
print('content',len(v),'value',len(vv),'pairs',len(items))
manifest=json.loads((D/'owner-manifest.json').read_text())
for source,owner,role in [
('02_shell/src/cli.rs','00_nucleo/prompts/shell/cli.md','Project typed kind and resolved supplement using L1 getters; no serialization-time name inference or new synthesized defaults. Audit legacy fixture kind intent and retain preexisting unrelated semantic-tree defaults as explicit debt.'),
('03_infra/src/export/tests/mod.rs','00_nucleo/prompts/infra/export/tests/mod.md','Migrate legacy Figure fixture to explicit typed element intent, preserving actual export assertions.')]:
 s,p=R/source,R/owner
 manifest['pairs'].append({'source':source,'owner':owner,'status':'existing','source_sha256':sha(s),'owner_sha256':sha(p),'source_prompt_hash':re.search(r'@prompt-hash (\w+)',s.read_text()).group(1),'role':role})
(D/'owner-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
lines=['# Proposed per-owner L0 amendments','', 'Status: diagnostic amendments only. Apply to the named exclusive owner after the integrated package is approved. Keep all unrelated current clauses and nucleus pins. Each existing source and owner hash is fixed in owner-manifest.json; new owners must exist and be sealed before their code. The common evidence, exact public declarations, preview/final order, semantic partition, bounded debts and acceptance are in design.md and must be copied into the responsible active owners rather than treating this diagnostic as direct code authority.','']
for i in manifest['pairs']:
 lines += ['## '+i['owner'],'', 'Exclusive consumer: `'+i['source']+'`.','',i['role'],'']
 if 'figure_kind.rs' in i['source']:
  lines += ['Own the exact `semantic-partition.json` catalogue, preorder first-match behavior, Value Content/LocatedContent/Array versus opaque Dictionary partition, no dynamic callback execution and exact Func identity. Tests distinguish every category; mutation of Content or Value must fail at this match. This unit does not own preparation, effects, counters or generic traversal.','']
 elif i['source'].endswith('compiler/realize/figure.rs'):
  lines += ['Own pure kind preview and final kind/supplement occurrence preparation; lexical omission/auto/explicit, known style/no-location supplement context and display result, source spans and retained versus validation sink semantics. Do not previsit hidden descendants or evaluate counters. New public declarations live only in the entity owner.','']
 elif i['source'].endswith('entities/elements/figure.rs'):
  lines += ['Replace the old three-field-only clause by the exact six-field declaration and FigureKind/FigureSupplement definitions in design.md. Preserve body/caption text and empty behavior, locatability and legacy numbering-in-chain rule. Manual FigureKind Eq/Hash is nominal and exact function identity; no name fallback. Caption omission/none and caption synthesized fields remain incomplete.','']
 elif i['source'].endswith('compiler/stdlib/figure_image.rs'):
  lines += ['Replace only the explicitly pending inference path. Remove `infer_kind_from_body` from construction; preserve valid function identity and omission/auto. Validate body→caption→kind→supplement→numbering before unknown/excess arguments, using existing occurrence origins. Supplement invalid: `expected content, function, none, or auto, found {type}`. Numbering invalid: `expected string, function, or none, found {type}`. Preserve independent image path and existing unsupported SVG/PDF behavior.','']
 elif i['source'].endswith('compiler/eval/rules/apply.rs') or i['source'].endswith('compiler/realize/recipes.rs'):
  lines += ['For reached Figure only, select from kind preview, fold matching show-set once, resolve final kind/supplement, then apply selected replacement before children. No repeated matching against partially changed kind. Respect RuleId/Location aliases and restore lexical frames on success/error; other element owners keep their phases.','']
 elif i['source'].endswith('compiler/eval/selector_matching.rs') or i['source'].endswith('compiler/eval/operators/equality.rs'):
  lines += ['Only Figure kind-bearing field comparisons gain typed identity, including nested Where/And/Or paths. Use exact constructor identity for figure itself as well as the Func-valued kind. Different native diagnostic names do not separate aliases; identical names do not merge different functions. Keep authoritative Some(empty) snapshots and all unrelated value/selector equality.','']
 elif i['source'].endswith('02_shell/src/cli.rs'):
  lines += ['Raw omitted kind/supplement must remain absent; prepared kind/supplement uses L1 values. Never turn custom Name("table") into the native table kind by text. Keep unrelated existing tree-only defaults explicitly historical/uncertified; this amendment does not synthesize missing caption position/separator or complete all Figure serialization.','']
 else:
  lines += ['Use the exact field/state definitions in design.md; copy all unrelated data and preserve existing functional assertions. A compile-only adjustment is not sufficient where semantic kind or counting expectations change: record the reason and language-level replacement assertion.','']
 lines += ['Acceptance: responsible RED→GREEN tests plus shared full build/test/path/lineage and differential lint gates. No catchall, suppression, weakened assertion, global hash repair or unapproved public change.','']
(D/'l0-amendments.md').write_text('\n'.join(lines)+'\n')
# Exact callsite inventory, including tests; no body text is rewritten.
calls=[]
for layer in ['01_core','02_shell','03_infra','04_wiring']:
 for path in (R/layer/'src').rglob('*.rs'):
  for n,line in enumerate(path.read_text().splitlines(),1):
   if 'Content::figure(' in line and not line.strip().startswith('//'):
    calls.append({'source':str(path.relative_to(R)),'line':n,'text':line.strip(),'action':'typed reconstruction, preserve all fields' if path.name=='materialize_time.rs' else 'typed native construction' if path.name=='figure_image.rs' else 'fixture: classify actual element intent versus custom Name; preserve assertions'})
(D/'legacy-call-sites.json').write_text(json.dumps({'utc':meta['utc'],'head':meta['head'],'source_state':meta['diff_head_stat'],'calls':calls},indent=2)+'\n')
print('final pairs',len(manifest['pairs']),'legacy calls',len(calls))
