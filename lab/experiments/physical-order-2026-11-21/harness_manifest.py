"""Pin Python harness modules loaded by the watch protocol."""
import hashlib
import json
import os
from pathlib import Path
import types

import observe

root = Path(__file__).resolve().parents[3]
seen = set()
pending = [observe, observe.cycle, observe.driver, observe.measure]
for _ in range(4):
    nxt = []
    for module in pending:
        if not isinstance(module, types.ModuleType) or id(module) in seen:
            continue
        seen.add(id(module))
        for value in vars(module).values():
            if isinstance(value, types.ModuleType):
                path = getattr(value, '__file__', None)
                if path and str(path).startswith(str(root)):
                    nxt.append(value)
    pending = nxt
paths = {Path(module.__file__) for module in (observe, observe.cycle, observe.driver, observe.measure)
         if getattr(module, '__file__', None)}
paths.update(Path(value.__file__) for value in vars(observe.cycle).values()
             if isinstance(value, types.ModuleType) and getattr(value, '__file__', None)
             and str(value.__file__).startswith(str(root)))
paths.update(Path(value.__file__) for value in vars(observe.cycle.cycle).values()
             if isinstance(value, types.ModuleType) and getattr(value, '__file__', None)
             and str(value.__file__).startswith(str(root)))
paths.update(Path(__file__).parent.glob('*.py'))
record = {str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest()
          for path in sorted(paths) if path.is_file() and str(path).startswith(str(root))}
out = Path(__file__).with_name('harness.sha256.json')
out.write_text(json.dumps(record, indent=2) + '\n')
print(out, len(record))
