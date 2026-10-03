set pagination off
set confirm off
python
import gdb
from collections import Counter
calls = 0
addresses = set()
kinds = Counter()
examples = []
class Probe(gdb.Breakpoint):
    def __init__(self):
        super().__init__('01_core/src/compiler/layout/measurement.rs:64')
        self.silent = True
    def stop(self):
        global calls
        calls += 1
        content = gdb.parse_and_eval('content')
        addresses.add(int(content))
        rendered = str(content.dereference())
        kind = rendered.split('Content::', 1)[1].split('(', 1)[0] if 'Content::' in rendered else 'unknown'
        kinds[kind] += 1
        if len(examples) < 3:
            examples.append((hex(int(content)), rendered[:180]))
        return False
Probe()
def done(event):
    print('PROBE calls=', calls, 'distinct_content_addresses=', len(addresses), 'kinds=', dict(kinds))
    for example in examples:
        print('PROBE example=', example)
gdb.events.exited.connect(done)
end
run
quit
