"""Freeze the explicit legacy expectation delta before productive changes."""
import importlib.util
from pathlib import Path
import re
import subprocess

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('record', D/'p1319-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
before = subprocess.check_output(['git','show','HEAD:'+r.OWNER],cwd=r.ROOT,text=True)
after = (r.ROOT/r.OWNER).read_text()
clean = lambda s: re.sub(r'^.*@prompt-hash.*\n', '', s, flags=re.M)
assert clean(before.split('#[cfg(test)]')[0]) == clean(after.split('#[cfg(test)]')[0])
def body(text, name):
    start = text.index('    fn '+name+'(')
    end = text.find('\n    #[test]', start)
    return text[start:end if end!=-1 else len(text)]
names = ['p1318_csv_pure_and_path_do_not_acquire_bytes_suffix',
         'p1317_csv_utf8_path_text_without_origin_change']
r.save('test-delta', dict(state=r.state(), source_sha256=r.sha(r.ROOT/r.OWNER),
    productive_prefix_unchanged_except_lineage=True,
    old_test_changes={n:dict(before=body(before,n),after=body(after,n)) for n in names},
    new_tests=['p1319_csv_binary_files_keep_cause_path_and_argument_origin',
               'p1319_csv_str_resolves_once_and_invalid_options_do_not_read'],
    instrumentation='MockWorld tracks resolve/read_path and forbids source when requested; no old assertions removed.',
    script_sha256=r.sha(__file__)))
