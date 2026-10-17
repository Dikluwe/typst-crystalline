from pathlib import Path
import hashlib,json,re
R=Path(__file__).resolve().parents[3]
D=Path(__file__).resolve().parent
consumer='01_core/src/entities/font_book.rs'
owner='00_nucleo/prompts/entities/font_book.md'
p=R/consumer;q=R/owner
source=re.sub(r'(?m)^//! @updated .*$', '//! @updated 2026-10-09', p.read_text())
spec=q.read_text()
assert 'Núcleos Tekt:' not in spec
code_hash=hashlib.sha256(re.sub(r'(?m)^//! @prompt-hash [^\n]*\n','',source).encode()).hexdigest()[:8]
spec=re.sub(r'(?m)^Hash do Código: .*$',f'Hash do Código: {code_hash}',spec)
clean=re.sub(r'(?m)^Hash do Código: [^\n]*\n','',spec).encode()
prompt_hash=hashlib.sha256(clean).hexdigest()[:8]
source=re.sub(r'(?m)^//! @prompt-hash .*$',f'//! @prompt-hash {prompt_hash}',source)
p.write_text(source);q.write_text(spec)
receipt={'consumer':consumer,'owner':owner,'code_hash':code_hash,'prompt_hash':prompt_hash,'source_sha256':hashlib.sha256(source.encode()).hexdigest(),'l0_sha256':hashlib.sha256(spec.encode()).hexdigest()}
(D/'seal-font-buffer.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
