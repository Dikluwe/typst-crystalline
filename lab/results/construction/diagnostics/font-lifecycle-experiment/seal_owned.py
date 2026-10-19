#!/usr/bin/env python3
"""Seal only task-owned, changed pairs using installed tekt-linter framing.
Reference: /repos/Antigravity/tekt-linter/03_infra/{prompt_io,nucleus,hash_writer}.rs.
No nucleus bytes or pins are changed. Every dependency pin must already match.
"""
import hashlib, json, pathlib, re, tomllib
OUT=pathlib.Path(__file__).resolve().parent
ROOT=OUT.parents[2]

def strip_meta(data, marker):
    matching=[line for line in data.splitlines(keepends=True) if line.startswith(marker)]
    assert len(matching)==1, (marker, matching)
    assert re.fullmatch(re.escape(marker)+rb'[0-9a-f]{8}\r?\n?',matching[0])
    return data.replace(matching[0],b'',1)

def framed(data, domain, deps):
    h=hashlib.sha256(data);h.update(b'\0'+domain+b'\0')
    for name,digest in sorted(deps):
        encoded=name.encode();h.update(len(encoded).to_bytes(8,'big')+encoded+b' '+digest)
    return h.digest()

def nucleus(name, visiting=()):
    assert name not in visiting
    data=(ROOT/name).read_bytes();doc=tomllib.loads(data.decode());deps=[]
    for dep in doc.get('depends',[]):
        digest=nucleus(dep['path'],visiting+(name,));assert digest.hex()==dep['sha256'], name
        deps.append((dep['path'],digest))
    return framed(data,b'TEKT-NUCLEUS-DEPS-V1',deps)

rows=[]
for name in json.loads((OUT/'initial.json').read_text())['originals']:
    if not name.endswith('.rs'):continue
    source=ROOT/name;data=source.read_bytes()
    if data==(OUT/'originals'/(name+'.snapshot')).read_bytes():continue
    prompt_name=re.search(rb'^//! @prompt (.+)$',data,re.M).group(1).decode()
    assert prompt_name in json.loads((OUT/'initial.json').read_text())['originals']
    prompt=ROOT/prompt_name;pdata=prompt.read_bytes();deps=[]
    for path,pin in re.findall(rb'^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$',pdata,re.M):
        digest=nucleus(path.decode());assert digest.hex()==pin.decode(), path
        deps.append((path.decode(),digest))
    clean=strip_meta(pdata,'Hash do Código: '.encode())
    ph=(framed(clean,b'TEKT-PROMPT-NUCLEI-V1',deps) if deps else hashlib.sha256(clean).digest()).hex()[:8]
    ch=hashlib.sha256(strip_meta(data,b'//! @prompt-hash ')).hexdigest()[:8]
    updated_source=re.sub(rb'(?m)^//! @prompt-hash [0-9a-f]{8}$',b'//! @prompt-hash '+ph.encode(),data,count=1)
    updated_prompt=re.sub(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]{8}$','Hash do Código: '.encode()+ch.encode(),pdata,count=1)
    if updated_source!=data:source.write_bytes(updated_source)
    if updated_prompt!=pdata:prompt.write_bytes(updated_prompt)
    rows.append({'source':name,'prompt':prompt_name,'prompt_hash':ph,'code_hash':ch})
(OUT/'owned-seals.json').write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(rows,indent=2))
