"""Create a byte-local measurement control for the pinned candidate."""
import hashlib
import json
from pathlib import Path
import stat
import struct
import subprocess
import sys

source, target, receipt = map(Path, sys.argv[1:])
expected = '3338608743aff8f36c864aa3377bcb7beff42492015f5607aa5a01c571dcd742'
code = bytearray(source.read_bytes())
assert hashlib.sha256(code).hexdigest() == expected
assert code[:6] == b'\x7fELF\x02\x01'
rows = subprocess.check_output(['nm', '-S', '-C', str(source)], text=True).splitlines()
symbol, = [row for row in rows if row.endswith('::lab_reserve_jump_enabled')]
address, size = (int(value, 16) for value in symbol.split()[:2])
assert size == 11
table, = struct.unpack_from('<Q', code, 32)
stride, count = struct.unpack_from('<HH', code, 54)
for index in range(count):
    kind, flags, position, virtual, _, extent, _, _ = struct.unpack_from(
        '<IIQQQQQQ', code, table + index * stride)
    if kind == 1 and flags & 1 and virtual <= address <= virtual + extent - size:
        offset = position + address - virtual
        break
else:
    raise RuntimeError('reservation gate outside executable segment')
assert code[offset:offset + size] == bytes.fromhex('488d4424ffc600018a00c3')
code[offset + 7] = 0
target.write_bytes(code)
target.chmod(stat.S_IMODE(source.stat().st_mode))
receipt.write_text(json.dumps(dict(candidate_sha256=expected,
    full_reservation_sha256=hashlib.sha256(code).hexdigest(), symbol=symbol.split(maxsplit=3)[3],
    byte_offset=offset + 7, old=1, new=0), indent=2) + '\n')
