set pagination off
set confirm off
python
import gdb
rows = []
class Probe(gdb.Breakpoint):
    def __init__(self):
        super().__init__('03_infra/src/font_metrics/caches.rs:175')
        self.silent = True
    def stop(self):
        rows.append((str(gdb.parse_and_eval('text')),
                     hex(int(gdb.parse_and_eval('self.shaped_width_cache.ptr.pointer'))),
                     int(gdb.parse_and_eval('key.size_bits')),
                     int(gdb.parse_and_eval('key.font_hash'))))
        return False
Probe()
def done(event):
    for row in rows:
        print('EPOCH', row)
gdb.events.exited.connect(done)
end
run
quit
