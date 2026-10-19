from pathlib import Path
import hashlib,json,gzip,datetime,subprocess
path=Path('01_core/src/entities/content/traversal.rs');current=path.read_text();source=subprocess.check_output(['git','show','HEAD:'+str(path)],text=True);baseline=json.loads(gzip.decompress((Path(__file__).resolve().parent/'before.json.gz').read_bytes()));assert hashlib.sha256(source.encode()).hexdigest()==baseline['source_sha256'][str(path)];method=source.index('    pub(crate) fn map_content_pre<F>');start=source.index('        let processed = match self {',method);opening=source.index('{',start)
# Scan Rust delimiters without interpreting braces/commas in strings or comments.
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
assert not body[last:].strip(),repr(body[last:])
output=[];wrapped=[]
for chunk in chunks:
 arrow=next((i for i,c,d in points(chunk) if chunk[i:i+2]=='=>' and d==(0,0,0)),None)
 assert arrow is not None,chunk
 pattern=chunk[:arrow];expr=chunk[arrow+2:].strip()
 # A block containing only an explicitly typed Ok(...)? already has the needed Result.
 normalized=expr
 if expr.startswith('{') and expr.endswith('}'):
  inner=expr[1:-1].strip()
  if inner.startswith('Ok::<Content,') and inner.endswith('?'):
   normalized=inner[:-1].rstrip()
   result_expression=True
  else:result_expression=False
 else:result_expression=False
 if 'map_content_pre' in expr or 'map_slot(' in expr:
  mapped='rebuild_pre_node(|| '+(normalized if result_expression else 'Ok('+expr+')')+')';wrapped.append(pattern.strip().splitlines()[-1])
 elif expr.startswith('unreachable!'):mapped=expr
 else:mapped=normalized if result_expression else 'Ok('+expr+')'
 output.append(pattern+'=> '+mapped+',')
helper='''        // As reconstruções ficam em frames separados do dispatcher recursivo.
        #[inline(never)]
        fn rebuild_pre_node(
            action: impl FnOnce() -> crate::entities::source_result::SourceResult<Content>,
        ) -> crate::entities::source_result::SourceResult<Content> {
            action()
        }
'''
tail=source[opening+closing+1:];assert tail.startswith(';\n        Ok(processed)\n    }'),tail[:120];tail=tail[len(';\n        Ok(processed)'):]
changed=source[:start]+helper+'        match self {'+''.join(output)+'\n        }'+tail
assert len(chunks)>50 and len(wrapped)>35,(len(chunks),len(wrapped))
old_start=source.index('    pub(crate) fn map_content_pre<F>')
old_end=source.index('    /// Forma canônica',old_start)
new_start=changed.index('    pub(crate) fn map_content_pre<F>')
new_end=changed.index('    /// Forma canônica',new_start)
current_start=current.index('    pub(crate) fn map_content_pre<F>')
current_end=current.index('    /// Forma canônica',current_start)
changed=current[:current_start]+changed[new_start:new_end]+current[current_end:]
path.write_text(changed)
p=Path(__file__).resolve().parent
(p/'outline-edit.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'before_sha256':hashlib.sha256(source.encode()).hexdigest(),'after_sha256':hashlib.sha256(changed.encode()).hexdigest(),'arms':len(chunks),'outlined':wrapped},indent=2).encode()))
print({'arms':len(chunks),'outlined':len(wrapped)})
