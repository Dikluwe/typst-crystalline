from pathlib import Path
import hashlib, json, re

root = Path(__file__).resolve().parents[3]
pairs = {
    '01_core/src/compiler/math/layout/large_operator.rs': '00_nucleo/prompts/compiler/math/layout/large_operator.md',
    '01_core/src/compiler/math/layout/attach.rs': '00_nucleo/prompts/compiler/math/layout/attach.md',
    '01_core/src/compiler/math/layout/tests.rs': '00_nucleo/prompts/compiler/math/layout/tests.md',
    '01_core/src/entities/layout_types.rs': '00_nucleo/prompts/entities/layout_types.md',
    '03_infra/src/export/stream.rs': '00_nucleo/prompts/infra/export/stream.md',
    '03_infra/src/export/stream/text.rs': '00_nucleo/prompts/infra/export/stream/text.md',
    '03_infra/src/export/stream/draw.rs': '00_nucleo/prompts/infra/export/stream/draw.md',
}
receipts = {}
for consumer, owner in pairs.items():
    code_path, prompt_path = root / consumer, root / owner
    code, prompt = code_path.read_text(), prompt_path.read_text()
    assert f'//! @prompt {owner}\n' in code, consumer
    clean_code = re.sub(r'(?m)^//! @prompt-hash [^\n]*\n', '', code).encode()
    code_hash = hashlib.sha256(clean_code).hexdigest()[:8]
    prompt = re.sub(r'(?m)^Hash do Código: .*$', f'Hash do Código: {code_hash}', prompt)
    clean_prompt = re.sub(r'(?m)^Hash do Código: [^\n]*\n', '', prompt).encode()
    deps = re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$', prompt)
    state = hashlib.sha256(clean_prompt)
    if deps:
        state.update(b'\0TEKT-PROMPT-NUCLEI-V1\0')
        for path, pin in sorted(deps):
            data = (root / path).read_bytes()
            digest = hashlib.sha256(data + b'\0TEKT-NUCLEUS-DEPS-V1\0').digest()
            assert digest.hex() == pin, path
            encoded = path.encode()
            state.update(len(encoded).to_bytes(8, 'big'))
            state.update(encoded)
            state.update(b' ')
            state.update(digest)
    prompt_hash = state.hexdigest()[:8]
    code = re.sub(r'(?m)^//! @prompt-hash .*$', f'//! @prompt-hash {prompt_hash}', code)
    prompt_path.write_text(prompt)
    code_path.write_text(code)
    receipts[consumer] = {'owner': owner, 'code_hash': code_hash, 'prompt_hash': prompt_hash}
(root / '00_nucleo/diagnosticos/math-large-identidade-2026-09-30/scoped-seals.json').write_text(json.dumps(receipts, indent=2) + '\n')
