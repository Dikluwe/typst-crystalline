import collections
import json
import os
import gdb

counts = collections.Counter()
recipes = collections.Counter()

class Probe(gdb.Breakpoint):
    def stop(self):
        inferior = gdb.selected_inferior()
        address = int(gdb.parse_and_eval('$rdi'))
        length = int(gdb.parse_and_eval('$rsi'))
        key = bytes(inferior.read_memory(address, length)).decode('utf-8', 'replace')
        counts[key] += 1
        if key == 'eval.table.local.recipe':
            value = int(gdb.parse_and_eval('$rdx'))
            recipes[bytes(inferior.read_memory(value, 32)).hex()] += 1
        return False

Probe('typst_infra::pipeline::paged::page_independent_style')
gdb.execute('run')
with open(os.environ['PAGE_RECIPE_PROBE_OUT'], 'w') as output:
    json.dump({'key_counts': counts, 'recipe_value_images': len(recipes),
               'recipe_value_image_counts': sorted(recipes.values(), reverse=True)}, output)
