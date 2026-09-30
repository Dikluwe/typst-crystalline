from pathlib import Path
import hashlib,re,json
root=Path.cwd();pairs=[('03_infra/src/export/stream/text.rs','00_nucleo/prompts/infra/export/stream/text.md'),('03_infra/src/export/stream/text/runs.rs','00_nucleo/prompts/infra/export/stream/text/runs.md'),('01_core/src/entities/layout_types.rs','00_nucleo/prompts/entities/layout_types.md')];receipts={}
for consumer, entry in json.loads((root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/materialization-scope.json').read_text()).items():
 pair=(consumer,entry['owner'])
 if pair not in pairs:pairs.append(pair)
for pair in [('03_infra/src/export/stream/separator.rs','00_nucleo/prompts/infra/export/stream/separator.md'),('01_core/src/compiler/layout/text/style.rs','00_nucleo/prompts/compiler/layout/text/style.md')]:
 if pair not in pairs:pairs.append(pair)

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
(root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/scoped-seals.json').write_text(json.dumps(receipts,indent=2)+'\n')
