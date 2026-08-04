#!/usr/bin/env python3
"""P959 Fase C.1 — distância tinta-a-tinta operador↔limite nas ocorrências do doc.

Mesmo método da auditoria externa: para cada glifo de operador grande,
encontra o limite acima/abaixo (mesma coluna) e mede o gap de tinta.
Uso: gaps959.py A.pdf B.pdf
"""
import re
import subprocess
import statistics
import sys

OPS = set('∑∏∐⋃⋂⨄⨅⨆∫∬∭∮∯∰⨁⨂⨀⋀⋁')

def words(path):
    out = subprocess.run(['pdftotext', '-bbox', path, '-'], capture_output=True, text=True).stdout
    return [
        (float(m.group(1)), float(m.group(2)), float(m.group(3)), float(m.group(4)), m.group(5))
        for m in re.finditer(
            r'<word xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">([^<]*)</word>',
            out,
        )
    ]

def gaps(path):
    ws = words(path)
    out = []
    for (x0, y0, x1, y1, t) in ws:
        if not (len(t) == 1 and t in OPS):
            continue
        cx = (x0 + x1) / 2
        above = [w for w in ws if abs((w[0] + w[2]) / 2 - cx) < 12 and w[3] <= y0 + 1 and w[3] > y0 - 40]
        below = [w for w in ws if abs((w[0] + w[2]) / 2 - cx) < 12 and w[1] >= y1 - 1 and w[1] < y1 + 40]
        if above:
            out.append(y0 - max(w[3] for w in above))
        if below:
            out.append(min(w[1] for w in below) - y1)
    return out

for name, path in (('cristalino', sys.argv[1]), ('vanilla', sys.argv[2])):
    g = gaps(path)
    g.sort()
    if g:
        print(
            f"{name}: {len(g)} gaps | min={g[0]:.2f} med={statistics.median(g):.2f} "
            f"max={g[-1]:.2f} | fora da banda 1.9-3.8pt: {sum(1 for x in g if x < 1.4 or x > 4.3)}"
        )
        print("   ", [f"{x:.1f}" for x in g])
