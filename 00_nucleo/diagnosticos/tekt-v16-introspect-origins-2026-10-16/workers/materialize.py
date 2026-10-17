from pathlib import Path
import json,hashlib,re,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert baseline.get('structural_red') and baseline.get('source_transparency_red')
assert all(sha(p)==h for p,h in baseline['source_hashes'].items())
for enum in ['Value','Content']:assert json.load(open(out/('probe-baseline-'+enum+'-result.json')))['selected_e0004']==0
files={p:Path(p).read_text() for p in scope['sources']};record={'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changes':[],'files':{}}
def body_range(text,name):
 matches=list(re.finditer(r'^([ \t]*)(?:pub(?:\([^)]*\))? )?fn '+name+r'\(',text,re.M));assert len(matches)==1,name
 start=matches[0].start();opening=text.index('{',matches[0].end());depth=1;end=opening+1
 while depth:
  depth+=(text[end]=='{')-(text[end]=='}');end+=1
 return start,end,text[start:end]
def group(enum,handled,indent):return ('\n'+indent+'| ').join(spelling for name,spelling in scope['variants'][enum].items() if name not in handled)
def bool_arm(enum,handled,indent,consequence):return indent+group(enum,handled,indent)+' => '+consequence+','
for target in scope['targets']:
 source=target['source'];name=target['function'];text=files[source];start,end,body=body_range(text,name)
 if target['id']=='I1':
  assert body.rstrip().endswith('Ok(module)\n}')
  old='        _ => None,';assert body.count(old)==1
  after=body.replace(old,'        '+group('Value',{'Content'},'        ')+' => None,',1)
 elif target['id']=='I2':
  special={'Outline','Bibliography','Ref','Dynamic','Document','Asset'}
  positive='\n        | '.join(scope['variants']['Content'][n] for n in ['Outline','Bibliography','Ref','Dynamic','Document','Asset'])
  after='fn needs_fresh_index(content: &Content) -> bool {\n    let content = content.source_body();\n    match content {\n        '+positive+' => true,\n'+bool_arm('Content',special,'        ','false')+'\n    }\n}'
 elif target['id']=='I3':
  old='            other => {';assert body.count(old)==1
  after=body.replace(old,'            other @ (\n                '+group('Content',{'Located','Realized'},'                ')+ '\n            ) => {',1)
 elif target['id']=='I4':
  handlers={'Equation','Located','Styled','ShowRecipe','Realized'}
  after='''fn block_equation_origin(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::Equation(e) => e.block,
        Content::Located(e) => block_equation_origin(&e.body),
        Content::Styled(e) => block_equation_origin(&e.body),
        Content::ShowRecipe(e) => block_equation_origin(&e.body),
        Content::Realized(e) => block_equation_origin(&e.original),
'''+bool_arm('Content',handlers,'        ','false')+'\n    }\n}'
 elif target['id']=='I5':
  handlers={'Text','Empty','Space','SpaceFromNewline','Linebreak','Sequence','Styled','Strong','Emph','Located','ShowRecipe'}
  after='''fn textual_product(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::Text(_)
        | Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Linebreak(_) => true,
        Content::Sequence(parts) => parts.iter().all(textual_product),
        Content::Styled(e) => textual_product(&e.body),
        Content::Strong(e) => textual_product(&e.body),
        Content::Emph(e) => textual_product(&e.body),
        Content::Located(e) => textual_product(&e.body),
        Content::ShowRecipe(e) => textual_product(&e.body),
'''+bool_arm('Content',handlers,'        ','false')+'\n    }\n}'
 elif target['id']=='I6':
  handled=set(re.findall(r'Content::([A-Z]\w*)\s*(?:\(|\{)',body));assert len(handled)==21,handled
  old='''        // Todas as outras variantes não são locatable em M1.
        // Adicionar uma variant locatable nova exige edição explícita
        // deste match (compilador não força exaustividade aqui porque
        // usamos catch-all `_`, mas o L0 mandata revisão).
        _ => None, // neutro: N16[γ] — variantes não-locatable em M1 retornam None (fallback aberto sob evolução de AST)'''
  assert body.count(old)==1
  replacement='''        // As demais variantes atuais não fornecem payload próprio aqui.
        // OutlineEntry conserva seu caminho sintetizado; Source já foi normalizado.
        '''+group('Content',handled,'        ')+' => None,'
  after=body.replace(old,replacement,1).replace('    match content {','    let content = content.source_body();\n    match content {',1)
 else:raise AssertionError(target)
 assert after!=body;files[source]=text[:start]+after+text[end:]
 record['changes'].append({**target,'before':body,'after':after})
source='01_core/src/compiler/introspect/extract_payload.rs'
files[source]=files[source].replace('''/// Extrai o `ElementPayload` correspondente a um `Content`, se for
/// uma variante locatable (Heading/Figure/Cite em M1).''','''/// Extrai o payload próprio do nó, normalizando a origem transparente Source.
/// Delegações e exceções são prescritas no L0; OutlineEntry mantém caminho sintetizado.''',1)
source='01_core/src/compiler/introspect/locatable.rs';text=files[source]
old='''//! variant novo é adicionado a `Content`. Invariante:
//! `is_locatable(c) == extract_payload(c).is_some()` para todo c.'''
new='''//! variant novo é adicionado a `Content`. No domínio de produtores ordinários,
//! `is_locatable(c) == extract_payload(c).is_some()`. OutlineEntry mantém
//! alinhamento pendente com o caminho sintetizado; Dynamic delega ao elemento.'''
assert text.count(old)==1;text=text.replace(old,new,1)
old='''/// Equivalente a `extract_payload(c).is_some()` mas mais barato — sem
/// construção de payload nem cálculo de hash.''';new='''/// No domínio ordinário, equivale a `extract_payload(c).is_some()` sem
/// construir payload ou calcular hash. OutlineEntry usa caminho sintetizado;
/// Dynamic depende do contrato de kind/payload do elemento.'''
assert text.count(old)==1;files[source]=text.replace(old,new,1)
for source,text in files.items():
 assert Path(source).read_text()==baseline['files'][source]
 files[source]=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-16',text,count=1,flags=re.M)
for source,text in files.items():Path(source).write_text(text)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
for source in scope['sources']:record['files'][source]={'baseline':baseline['files'][source],'expected_after':Path(source).read_text()}
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'decisions':6,'source_normalization':True,'locatable_executable_code_unchanged':True}),flush=True)
