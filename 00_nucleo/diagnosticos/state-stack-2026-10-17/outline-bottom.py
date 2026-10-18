from pathlib import Path
import subprocess,hashlib,json,gzip,datetime
path=Path('01_core/src/entities/content/traversal.rs');current=path.read_text();source=subprocess.check_output(['git','show','HEAD:'+str(path)],text=True)
p=Path(__file__).resolve().parent;baseline=json.loads(gzip.decompress((p/'before.json.gz').read_bytes()));assert hashlib.sha256(source.encode()).hexdigest()==baseline['source_sha256'][str(path)]
method=source.index('    fn map_content_inner<F>');start=source.index('        let processed = match self {',method);opening=source.index('{',start)
def points(text):
 b=p=q=0;string=False;escape=False;line=False;block=0;i=0
 while i<len(text):
  c=text[i];n=text[i+1:i+2]
  if line:
   if c=='\n':line=False
  elif block:
   if c=='/' and n=='*':block+=1;i+=1
   elif c=='*' and n=='/':block-=1;i+=1
  elif string:
   if escape:escape=False
   elif c=='\\':escape=True
   elif c=='"':string=False
  elif c=='/' and n=='/':line=True;i+=1
  elif c=='/' and n=='*':block=1;i+=1
  elif c=='"':string=True
  else:
   yield i,c,(b,p,q)
   if c=='{':b+=1
   elif c=='}':b-=1
   elif c=='(':p+=1
   elif c==')':p-=1
   elif c=='[':q+=1
   elif c==']':q-=1
  i+=1
relative=source[opening:];closing=next(i for i,c,d in points(relative) if c=='}' and d==(1,0,0));body=relative[1:closing]
chunks=[];last=0
for i,c,d in points(body):
 if c==',' and d==(0,0,0):chunks.append(body[last:i]);last=i+1
assert not body[last:].strip()
output=[];wrapped=[]
for chunk in chunks:
 arrow=next(i for i,c,d in points(chunk) if chunk[i:i+2]=='=>' and d==(0,0,0));pattern=chunk[:arrow];expr=chunk[arrow+2:].strip()
 recursive='map_content' in expr
 if expr.startswith('unreachable!'):mapped=expr
 elif recursive:
  result=expr[:-1] if expr.endswith('?') else 'Ok('+expr+')'
  mapped='rebuild_bottom_up_node(|| '+result+')';wrapped.append(pattern.strip().splitlines()[-1])
 else:mapped='Ok('+expr+')'
 output.append(pattern+'=> '+mapped+',')
helper="""        #[inline(never)]
        fn rebuild_bottom_up_node(
            action: impl FnOnce() -> crate::entities::source_result::SourceResult<Option<Content>>,
        ) -> crate::entities::source_result::SourceResult<Option<Content>> {
            action()
        }
"""
after=source[opening+closing+1:];assert after.startswith(';\n\n        if !transform_root'),after[:100]
changed=source[:start]+helper+'        let processed = match self {'+''.join(output)+'\n        }?'+after
new_start=changed.index('    fn map_content_inner<F>');new_end=changed.index('    /// Private pre-order traversal.',new_start)
current_start=current.index('    fn map_content_inner<F>');current_end=current.index('    /// Private pre-order traversal.',current_start)
changed=current[:current_start]+changed[new_start:new_end]+current[current_end:]
assert len(wrapped)>40
path.write_text(changed)
(p/'outline-bottom-edit.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'before_sha256':hashlib.sha256(current.encode()).hexdigest(),'after_sha256':hashlib.sha256(changed.encode()).hexdigest(),'arms':len(chunks),'outlined':wrapped},indent=2).encode()))
print({'arms':len(chunks),'outlined':len(wrapped)})
