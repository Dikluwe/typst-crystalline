import os
import sys
from pathlib import Path
import subprocess
import json
import datetime

if os.environ.get('TEKT_HTML_IDENTITY_MEMORY_LIMIT') != '12G':
    os.execvp('systemd-run', ['systemd-run', '--user', '--quiet', '--wait', '--pipe',
        '--working-directory=' + os.getcwd(), '-p', 'MemoryHigh=10G', '-p', 'MemoryMax=12G', '-p', 'MemorySwapMax=0',
        '/usr/bin/env', 'TEKT_HTML_IDENTITY_MEMORY_LIMIT=12G', 'CARGO_BUILD_JOBS=1',
        'PATH=' + os.environ['PATH'], 'python3', __file__, *sys.argv[1:]])

out = Path('00_nucleo/diagnosticos/html-feature-identity-2026-10-17')
cg = Path('/sys/fs/cgroup') / Path('/proc/self/cgroup').read_text().strip().split('::', 1)[1].lstrip('/')
def memory():
    return {key: (cg / key).read_text().strip() for key in
            ['memory.max', 'memory.high', 'memory.swap.max', 'memory.current', 'memory.peak', 'memory.events']}
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'label': sys.argv[1], 'command': sys.argv[2:], 'memory_before': memory(), 'cargo_jobs': os.environ['CARGO_BUILD_JOBS']}
assert record['memory_before']['memory.max'] == str(12 * 1024**3)
assert record['memory_before']['memory.high'] == str(10 * 1024**3)
assert record['memory_before']['memory.swap.max'] == '0'
try:
    result = subprocess.run(sys.argv[2:])
    record['exit'] = result.returncode
finally:
    record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(), memory_after=memory())
    (out / (sys.argv[1] + '-resources.json')).write_text(json.dumps(record, indent=2) + '\n')

sys.exit(result.returncode)
