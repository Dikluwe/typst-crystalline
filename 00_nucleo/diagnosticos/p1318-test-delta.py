"""Record expected-message changes to old tests before the candidate."""
import importlib.util
import json
from pathlib import Path
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1318-record.py')
r=importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
baseline=json.loads((D/'p1318-measurement.json').read_text())['sources'][r.OWNER]
current=(r.ROOT/r.OWNER).read_text()
names=[
    'p1317_csv_utf8_native_origin_and_synthetic',
    'p1316_csv_bytes_parse_origin_not_named_or_call',
    'p1316_csv_parse_with_excess_uses_first_source_origin',
    'p1316_csv_synthetic_and_explicit_detached_preserved',
    'p1315_csv_native_bytes_ordinal_without_io',
    'p1313_csv_bytes_preserve_legacy_parsing',
]
def section(text,name):
    start=text.index('    fn '+name+'(')
    end=text.index('\n    #[test]',start)
    return text[start:end]
assert baseline['text'].split('#[cfg(test)]')[0]==current.split('#[cfg(test)]')[0]
r.save('test-delta',dict(state=r.state(),baseline_source_sha256=baseline['sha256'],
    red_source_sha256=r.sha(r.ROOT/r.OWNER),productive_prefix_unchanged=True,
    changed_old_tests=[dict(name=n,before=section(baseline['text'],n),after=section(current,n)) for n in names],
    new_tests=['p1318_csv_bytes_positions_follow_buffer_and_parser_offset',
               'p1318_csv_text_position_is_not_argument_origin',
               'p1318_csv_pure_and_path_do_not_acquire_bytes_suffix'],
    reason='Only native Bytes expected messages gain explicit measured positions; all old cases, causes, spans, hints/traces and pure/path controls remain. One extra assertion pins the pure UTF8 legacy message.',
    script_sha256=r.sha(__file__)))
