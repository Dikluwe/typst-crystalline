set pagination off
set confirm off
python
import gdb
count = 0
class Probe(gdb.Breakpoint):
    def __init__(self):
        super().__init__('03_infra/src/font_metrics/caches.rs:175')
        self.silent = True
    def stop(self):
        global count
        if str(gdb.parse_and_eval('text')) == '"C000"':
            count += 1
            print('C000-MISS', count,
                  'cache=', hex(int(gdb.parse_and_eval('self.shaped_width_cache.ptr.pointer'))),
                  'size_bits=', int(gdb.parse_and_eval('key.size_bits')))
            print(gdb.execute('bt 20', to_string=True))
        return False
Probe()
end
run
quit
