from pathlib import Path
import hashlib,re,json
root=Path.cwd();receipts={};pairs=list(json.loads((root/"00_nucleo/diagnosticos/correcao-hardcode-paragrafo-2026-09-28/scope.json").read_text()).items())
for consumer,owner in pairs:
 source=root/consumer;prompt=root/owner;code=source.read_text();text=prompt.read_text()
 text=re.sub(r"(?m)^Núcleos Tekt:\n((?:- [^\n]+\n)+)",lambda m:"Núcleos Tekt:\n"+"".join(sorted(m.group(1).splitlines(keepends=True))),text)
 if consumer.startswith('03_infra/'):
  code=re.sub(r'(?m)^//! @updated .*$', '//! @updated 2026-09-28',code)
 clean_code=re.sub(r'(?m)^//! @prompt-hash [^\n]*\n','',code).encode();code_hash=hashlib.sha256(clean_code).hexdigest()[:8]
 text=re.sub(r'(?m)^Hash do Código: .*$',f'Hash do Código: {code_hash}',text)
 clean_prompt=re.sub(r'(?m)^Hash do Código: [^\n]*\n','',text).encode();deps=re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$',text)
 state=hashlib.sha256(clean_prompt)
 if deps:
  state.update(b'\0TEKT-PROMPT-NUCLEI-V1\0')
  for path,pin in sorted(deps):
   data=(root/path).read_bytes();assert b'[dependencies]' not in data and b'[[dependencies]]' not in data
   digest=hashlib.sha256(data+b'\0TEKT-NUCLEUS-DEPS-V1\0').digest();assert digest.hex()==pin
   encoded=path.encode();state.update(len(encoded).to_bytes(8,'big'));state.update(encoded);state.update(b' ');state.update(digest)
 prompt_hash=state.hexdigest()[:8];code=re.sub(r'(?m)^//! @prompt-hash .*$',f'//! @prompt-hash {prompt_hash}',code)
 prompt.write_text(text);source.write_text(code);receipts[consumer]={'owner':owner,'code_hash':code_hash,'prompt_hash':prompt_hash}
(root/'00_nucleo/diagnosticos/correcao-hardcode-paragrafo-2026-09-28/scoped-seals.json').write_text(json.dumps(receipts,indent=2)+'\n')
