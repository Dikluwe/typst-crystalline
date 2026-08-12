#!/usr/bin/env python3
"""Co-mudança por função/método num ficheiro — critério 3 do método do Passo 1002.

Para cada commit que tocou o ficheiro (atravessando renames), mapeia as linhas
ADICIONADAS do pós-imagem à função/item que as contém. Desconta o ruído de
resselo de linhagem (`@prompt-hash`/`@updated`), que no Passo 1006 se mostrou
dominante (30 de 51 commits).

Uso:
    python3 tools/analysis/cochange_metrics.py <caminho-actual> [regex-do-caminho-historico]

Exemplos:
    python3 tools/analysis/cochange_metrics.py 01_core/src/compiler/eval/bindings.rs '(rules|engine|compiler)/eval/bindings\\.rs$'
"""
import re
import subprocess
import sys
from collections import defaultdict

REPO = "/repos/Antigravity/typst-crystalline"

PATH = sys.argv[1] if len(sys.argv) > 1 else "01_core/src/compiler/layout/metrics.rs"
HIST = sys.argv[2] if len(sys.argv) > 2 else re.escape(PATH.split("/")[-1]) + "$"


def sh(*args):
    return subprocess.run(
        args, cwd=REPO, capture_output=True, text=True, errors="replace"
    ).stdout


commits = sh("git", "log", "--follow", "--format=%h", "--", PATH).split()

ITEM = re.compile(r"^\s*(?:pub(?:\([a-z:) ]+\))? )?fn ([a-z_0-9]+)")
TOP = re.compile(r"^(?:pub(?:\([a-z:) ]+\))? )?(?:trait|struct|enum|impl|mod) ([A-Za-z_&<> ]+)")
LINEAGE = re.compile(r"^[+-]//! @(prompt-hash|updated|prompt)\b")

per_method = defaultdict(set)
per_commit = defaultdict(set)
subjects = {}
lineage_only = 0
touched = 0

for c in commits:
    names = sh("git", "show", "--stat", "--format=", "--name-only", c).splitlines()
    path = next((n for n in names if re.search(HIST, n)), None)
    if not path:
        continue
    touched += 1
    subjects[c] = sh("git", "log", "-1", "--format=%s", c).strip()

    raw = sh("git", "show", "--format=", "-U0", c, "--", path).splitlines()
    body_changes = [
        l for l in raw
        if l[:1] in "+-" and not l.startswith(("+++", "---")) and not LINEAGE.match(l)
    ]
    if not body_changes:
        lineage_only += 1
        continue

    blob = sh("git", "show", f"{c}:{path}").splitlines()
    spans = []
    for i, line in enumerate(blob, 1):
        m = ITEM.match(line)
        if m:
            spans.append((i, m.group(1)))
            continue
        m = TOP.match(line)
        if m:
            spans.append((i, "<" + m.group(1).strip() + ">"))

    def owner(lineno):
        best = "<topo>"
        for start, name in spans:
            if start <= lineno:
                best = name
            else:
                break
        return best

    cur = None
    for line in raw:
        if line.startswith("@@"):
            m = re.search(r"\+(\d+)(?:,(\d+))?", line)
            cur = int(m.group(1)) if m else None
            continue
        if line.startswith("+") and not line.startswith("+++") and cur is not None:
            if not LINEAGE.match(line):
                o = owner(cur)
                per_method[o].add(c)
                per_commit[c].add(o)
            cur += 1

print(f"# {PATH}")
print(f"commits que tocaram o ficheiro: {touched}")
print(f"  dos quais só linhagem (@prompt-hash/@updated): {lineage_only}")
print(f"  com mudança de corpo: {touched - lineage_only}")
print()
print("=== FUNÇÃO -> nº de commits com mudança de corpo ===")
for name, cs in sorted(per_method.items(), key=lambda kv: -len(kv[1])):
    print(f"{len(cs):3d}  {name}")

print()
print("=== CLUSTERS: funções que mudam no MESMO commit (>=2) ===")
for c, ms in sorted(per_commit.items(), key=lambda kv: subjects[kv[0]]):
    real = sorted(m for m in ms if not m.startswith("<"))
    if len(real) >= 2:
        print(f"{c} {subjects[c][:44]:46s} :: {', '.join(real)}")
