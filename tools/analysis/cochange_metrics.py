#!/usr/bin/env python3
"""Co-mudança por função/método num ficheiro — critério 3 do método do Passo 1002.

Para cada commit que tocou o ficheiro (atravessando renames), mapeia as linhas
ADICIONADAS do pós-imagem à função/item que as contém. Desconta o ruído de
resselo de linhagem (`@prompt-hash`/`@updated`), que no Passo 1006 se mostrou
dominante (30 de 51 commits).

Desconta também o **artefacto de atribuição de fronteira** (medido em
`stdlib/text.rs`, 2026-08-13): a fronteira de um item vai até ao banner do item
seguinte, e só as linhas a partir da linha de declaração contam como mudança de
corpo. Sem as duas correcções, um passo que acrescenta uma função nova faz a
função *anterior* aparecer como co-mudança, com o corpo intacto — em
`stdlib/text.rs` isso produzia três clusters falsos de quatro.

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
per_banner = defaultdict(set)
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
    in_tests = False
    for i, line in enumerate(blob, 1):
        # Itens dentro de `#[cfg(test)] mod tests` são rotulados `test:<nome>`: um
        # ficheiro com suite própria (structural.rs tinha 56 testes) enche a saída de
        # nomes de teste que não são unidades de fronteira. Rotular em vez de excluir
        # — a linha de um teste tem de continuar a ter dono, senão é atribuída ao
        # último item de topo e inventa co-mudança (medido em 2026-08-13).
        if re.match(r"^(#\[cfg\(test\)\]|mod tests\b)", line):
            in_tests = True
        m = ITEM.match(line)
        if m:
            spans.append((i, ("test:" if in_tests else "") + m.group(1)))
            continue
        m = TOP.match(line)
        if m:
            spans.append((i, "<" + m.group(1).strip() + ">"))

    # Correcção do artefacto de atribuição de fronteira: o intervalo de um item
    # começa no seu banner de comentário (`// ── ... ──`, doc-comments, atributos),
    # não na linha da declaração. Sem isto, as linhas do banner de uma função NOVA
    # caem depois do `}` da função anterior e são-lhe atribuídas — a função anterior
    # aparece a "co-mudar" com corpo intacto. Medido em `stdlib/text.rs`: produzia
    # três clusters falsos (`overline`+`smallcaps`, `smallcaps`+`sub`/`super`,
    # `highlight`+`super`), todos desaparecidos com a fronteira corrigida.
    def banner_start(decl_line, prev_decl):
        start, j = decl_line, decl_line - 1
        while j > prev_decl:
            s = blob[j - 1].strip()
            if s == "" or s.startswith("//") or s.startswith("#["):
                start, j = j, j - 1
            else:
                break
        return start

    # (banner_start, decl_line, nome) por item, em ordem de linha.
    starts = []
    for idx, (decl, name) in enumerate(spans):
        prev = spans[idx - 1][0] if idx else 0
        starts.append((banner_start(decl, prev), decl, name))

    def owner(lineno):
        """Devolve (nome, 'corpo'|'banner'). Só 'corpo' é sinal de co-mudança:
        uma linha no banner de um item pode ser a linha em branco que sobrou do
        item anterior, e contá-la como mudança inventa co-mudança."""
        best = ("<topo>", "corpo")
        for start, decl, name in starts:
            if start <= lineno:
                best = (name, "corpo" if lineno >= decl else "banner")
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
                name, where = owner(cur)
                if where == "corpo":
                    per_method[name].add(c)
                    per_commit[c].add(name)
                else:
                    per_banner[name].add(c)
            cur += 1

print(f"# {PATH}")
print(f"commits que tocaram o ficheiro: {touched}")
print(f"  dos quais só linhagem (@prompt-hash/@updated): {lineage_only}")
print(f"  com mudança de corpo: {touched - lineage_only}")
print()
print("=== FUNÇÃO -> nº de commits com mudança de corpo ===")
for name in sorted(set(per_method) | set(per_banner), key=lambda n: -len(per_method.get(n, ()))):
    banner = len(per_banner.get(name, ()))
    extra = f"   (só banner, não conta: {banner})" if banner else ""
    print(f"{len(per_method.get(name, ())):3d}  {name}{extra}")

print()
print("=== CLUSTERS: funções que mudam no MESMO commit (>=2) ===")
for c, ms in sorted(per_commit.items(), key=lambda kv: subjects[kv[0]]):
    real = sorted(m for m in ms if not m.startswith("<"))
    if len(real) >= 2:
        print(f"{c} {subjects[c][:44]:46s} :: {', '.join(real)}")
