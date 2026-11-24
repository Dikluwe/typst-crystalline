from pathlib import Path

source = Path(__file__).resolve().parent / 'source'
l0 = source / '00_nucleo/prompts/core/compiler/layout/page_running_requests.md'
with l0.open('a') as stream:
    stream.write(
        '\nInitial census: generic Content equality refused every checkpoint, '
        'including the fixed control\'s proven cut. The closed source grammar '
        'allows only literal text changes. Compare every admitted Text leaf '
        'as exact text, after unwrapping source wrappers. Keep carrier, environment, '
        'reference and boundary proofs for other nodes. A value mismatch refuses '
        'the cut. Verify edits before and after the historical page 3.\n'
    )
p = source / '01_core/src/compiler/layout/page_running_requests.rs'
s = p.read_text()
old = '|| (adaptive && a.content != b.content)'
new = '''|| (adaptive && match (a.content.source_body(), b.content.source_body()) {
                    (Content::Text(old), Content::Text(new)) => old != new,
                    (Content::Text(_), _) | (_, Content::Text(_)) => true,
                    _ => false,
                })'''
assert s.count(old) == 1
p.write_text(s.replace(old,new,1))
