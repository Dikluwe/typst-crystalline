"""Bidirectional lineage including the existing dependency-free nucleus."""
import hashlib
from pathlib import Path
import re
ROOT=Path(__file__).resolve().parents[2]
SM=rb'(?m)^//! @prompt-hash ([0-9a-f]{8})\n'
PM=rb'(?m)^Hash do C\xc3\xb3digo: ([0-9a-f]{8})\n'
def hashes(prompt,source):
    p=(ROOT/prompt).read_bytes();s=(ROOT/source).read_bytes()
    assert len(re.findall(PM,p))==len(re.findall(SM,s))==1
    norm=re.sub(PM,b'',p);payload=norm
    pins=re.findall(rb'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$',p)
    if pins:
        assert len(pins)==1
        payload+=b'\0TEKT-PROMPT-NUCLEI-V1\0'
        for path,expected in pins:
            nucleus=(ROOT/path.decode()).read_bytes();assert b'[[depends]]' not in nucleus
            pin=hashlib.sha256(nucleus+b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
            assert pin.hex()==expected.decode()
            payload+=len(path).to_bytes(8,'big')+path+bytes([32])+pin
    return dict(prompt=prompt,source=source,norm_sha256=hashlib.sha256(norm).hexdigest(),
        effective_a=hashlib.sha256(payload).hexdigest(),code_b=hashlib.sha256(re.sub(SM,b'',s)).hexdigest(),
        recorded_a=re.search(SM,s).group(1).decode(),recorded_b=re.search(PM,p).group(1).decode())
