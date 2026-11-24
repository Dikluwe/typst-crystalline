set pagination off
set confirm off
set debuginfod enabled off
set disable-randomization off
maintenance set worker-threads 1
start
python
import gdb, os
from pathlib import Path
pid = gdb.selected_inferior().pid
exe = os.readlink(f"/proc/{pid}/exe")
base = next(int(line.split()[0].split("-")[0],16) for line in Path(f"/proc/{pid}/maps").read_text().splitlines() if line.split()[2] == "00000000" and line.endswith(exe))
for symbol in [0x2098ca0, 0x2646430, 0x2646820, 0x2646c10]:
    gdb.execute(f"break *{base + symbol + 0x113:#x}")
end
continue
x/5i $pc
python
import re
instructions = gdb.execute("x/5i $pc", to_string=True)
length, pending, captures = [int(v, 16) for v in re.findall(r"cmpq? +\$0x[0-9a-f]+,(0x[0-9a-f]+)\(%rbx\)", instructions)]
for name, offset in [("line_items", length), ("fractional_items", pending), ("registered_positions", captures)]:
    value = int(gdb.parse_and_eval(f"*(unsigned long*)($rbx+{offset})"))
    print(f"{name}={value}")
end
si
si
x/i $pc
kill
