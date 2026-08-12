#!/usr/bin/env python3
"""Co-mudança por método em metrics.rs — critério 3 do Passo 1006.

Para cada commit que tocou o ficheiro (atravessando os renames rules/→engine/),
mapeia as linhas ADICIONADAS do pós-imagem ao método/item que as contém.
"""
import re
import subprocess
from collections import defaultdict

REPO = "/repos/Antigravity/typst-crystalline"


def sh(*args):
    return subprocess.run(
        args, cwd=REPO, capture_output=True, text=True, errors="replace"
    ).stdout


commits = sh("git", "log", "--follow", "--format=%h",
             "--", "01_core/src/engine/layout/metrics.rs").split()

# Regex para itens de topo dentro do trait/impl (indentados 4) e de topo (0).
ITEM = re.compile(r"^\s*(?:pub |pub\(crate\) )?fn ([a-z_0-9]+)")
TOP = re.compile(r"^(?:pub )?(?:trait|struct|impl|mod) ([A-Za-z_&<> ]+)")

per_method = defaultdict(set)   # method -> {commits}
per_commit = defaultdict(set)   # commit -> {methods}
subjects = {}

for c in commits:
    names = sh("git", "show", "--stat", "--format=", "--name-only", c).splitlines()
    path = next((n for n in names
                 if re.search(r"(rules|engine)/layout/metrics\.rs$", n)), None)
    if not path:
        continue
    subjects[c] = sh("git", "log", "-1", "--format=%s", c).strip()

    # pós-imagem do ficheiro nesse commit -> spans dos métodos
    blob = sh("git", "show", f"{c}:{path}").splitlines()
    spans = []          # (start_line_1based, name)
    for i, line in enumerate(blob, 1):
        m = ITEM.match(line)
        if m:
            spans.append((i, m.group(1)))
            continue
        m = TOP.match(line)
        if m:
            spans.append((i, "<" + m.group(1).strip() + ">"))

    def owner(lineno):
        best = "<topo-do-ficheiro>"
        for start, name in spans:
            if start <= lineno:
                best = name
            else:
                break
        return best

    diff = sh("git", "show", "--format=", "-U0", c, "--", path).splitlines()
    cur = None
    for line in diff:
        if line.startswith("@@"):
            m = re.search(r"\+(\d+)(?:,(\d+))?", line)
            cur = int(m.group(1)) if m else None
            continue
        if line.startswith("+") and not line.startswith("+++") and cur is not None:
            o = owner(cur)
            per_method[o].add(c)
            per_commit[c].add(o)
            cur += 1

print("=== MÉTODO/ITEM -> nº de commits que o alteraram ===")
for name, cs in sorted(per_method.items(), key=lambda kv: -len(kv[1])):
    subs = sorted(subjects[c][:34] for c in cs)
    print(f"{len(cs):3d}  {name}")
    for s in subs:
        print(f"       - {s}")

print()
print("=== CLUSTERS: métodos que mudam no MESMO commit ===")
for c, ms in sorted(per_commit.items(), key=lambda kv: subjects[kv[0]]):
    real = sorted(m for m in ms if not m.startswith("<"))
    if len(real) >= 2:
        print(f"{c} {subjects[c][:44]:46s} :: {', '.join(real)}")
