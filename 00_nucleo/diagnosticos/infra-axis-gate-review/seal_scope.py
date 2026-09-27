"""Seal only the explicitly reviewed prompt/consumer pairs, never the repository."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[3]
DATA=Path(__file__).resolve().parent

def sha(data):return hashlib.sha256(data).digest()

def run():
    scope=json.loads((DATA/'provenance.json').read_text())['initial_fix_scope']
    for owner in (p for p in scope if p.endswith('.md')):
        prompt=ROOT/owner
        consumer=next(p for p in scope if p.endswith('.rs') and f'@prompt {owner}\n' in (ROOT/p).read_text())
        source=ROOT/consumer
        code=source.read_text();code=re.sub(r'(?m)^//! @updated .*$', '//! @updated 2026-09-27',code)
        text=prompt.read_text()
        # Refresh existing local symbol anchors; retain the owner rather than inventing targets.
        for match in list(re.finditer(r'`('+re.escape(consumer)+r'):(\d+)` — `([^`]+)`',text)):
            name=match[3]
            found=re.search(r'(?m)^.*\b(?:struct|enum|fn) '+re.escape(name)+r'\b',code)
            if found:
                anchor=next(i for i,line in enumerate(code.splitlines(),1) if re.search(r'\b(?:struct|enum|fn) '+re.escape(name)+r'\b',line))
                text=text.replace(match[0],f'`{consumer}:{anchor}` — `{name}`')
        clean_code=re.sub(r'(?m)^//! @prompt-hash [^\n]*\n','',code).encode()
        text=re.sub(r'(?m)^Hash do Código: .*$',f'Hash do Código: {sha(clean_code).hex()[:8]}',text)
        clean=re.sub(r'(?m)^Hash do Código: [^\n]*\n','',text).encode()
        deps=re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$',text)
        if deps:
            state=hashlib.sha256(clean);state.update(b'\0TEKT-PROMPT-NUCLEI-V1\0')
            for path,pin in sorted(deps):
                data=(ROOT/path).read_bytes()
                assert b'[dependencies]' not in data and b'[[dependencies]]' not in data,'recursive dependencies require explicit implementation'
                digest=sha(data+b'\0TEKT-NUCLEUS-DEPS-V1\0');assert digest.hex()==pin
                encoded=path.encode();state.update(len(encoded).to_bytes(8,'big'));state.update(encoded);state.update(b' ');state.update(digest)
            digest=state.digest()
        else:digest=sha(clean)
        code=re.sub(r'(?m)^//! @prompt-hash .*$',f'//! @prompt-hash {digest.hex()[:8]}',code)
        prompt.write_text(text);source.write_text(code)
if __name__=='__main__':run()
