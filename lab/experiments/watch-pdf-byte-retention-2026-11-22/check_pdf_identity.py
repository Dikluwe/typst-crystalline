"""Compare watch publications after removing only PDF trailer identifiers."""
import json
import os
import subprocess
import re
from pathlib import Path

work = Path(os.environ['TEKT_LAB_WORK']) / 'witness'
result = {}
for arm in ('off', 'doc', 'bytes', 'baseline', 'vanilla'):
    root = work / f'00-{arm}'
    documents = {}
    for state in ('startup', 'comment', 'comment-2', 'visible', 'out'):
        pdf_path = root / f'{state}.pdf'
        data = json.loads(subprocess.check_output(['qpdf', '--json', str(pdf_path)]))
        ids = data['qpdf'][-1]['trailer']['value'].pop('/ID', None)
        raw = pdf_path.read_bytes()
        normalized = re.sub(rb'<xmpMM:(InstanceID|DocumentID)>[^<]+</xmpMM:\1>',
                            rb'<xmpMM:\1>ID</xmpMM:\1>', raw)
        normalized = re.sub(rb'/ID \[ \([^)]*\) \([^)]*\) \]', b'/ID [ (ID) (ID) ]', normalized)
        documents[state] = {'id': ids, 'without_trailer_id': data,
                            'normalized': normalized, 'size': len(raw)}
    result[arm] = {
        'comment_1_same_except_id': documents['startup']['without_trailer_id'] == documents['comment']['without_trailer_id'],
        'comment_2_same_except_id': documents['startup']['without_trailer_id'] == documents['comment-2']['without_trailer_id'],
        'comment_1_id_changed': documents['startup']['id'] != documents['comment']['id'],
        'comment_2_id_changed': documents['startup']['id'] != documents['comment-2']['id'],
        'visible_id_changed': documents['startup']['id'] != documents['visible']['id'],
        'visible_equals_final': documents['visible'] == documents['out'],
        'comment_1_same_bytes_except_ids': documents['startup']['normalized'] == documents['comment']['normalized'],
        'comment_2_same_bytes_except_ids': documents['startup']['normalized'] == documents['comment-2']['normalized'],
        'sizes': {state: value['size'] for state, value in documents.items()},
        'ids': {state: value['id'] for state, value in documents.items()},
    }
destination = Path(os.environ['TEKT_RESULT_DIR']) / 'pdf-identity.json'
destination.write_text(json.dumps(result, indent=2) + '\n')
