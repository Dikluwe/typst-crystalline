#!/usr/bin/env python3
"""
falta_migrar.py — Passo 386 (diagnóstico, eixo 1)

Extrai, de forma DETERMINÍSTICA, "o que falta refatorar" de duas fontes
complementares e reconcilia-as:

  Lista A (top-down, autorada): linhas `ausente`/`parcial` da Tabela A
    (vista user-facing, secções A.1–A.9) do Inventário 148, com categoria,
    ressalva (coluna Nota) e roadmap (Referência + Tabela C).

  Lista B (bottom-up, rede de segurança): candidatos de língua do resíduo
    balde 3 do Passo 385 (reusa decompor_so_vanilla), filtrados per ADR-0107,
    cruzados com o índice de features do Inventário 148.

A CONTAGEM é script (critério 6.6 do passo). O LLM/humano só dá o VEREDICTO
final de cada candidato da Lista B (§B.3) — aqui o script propõe o cruzamento
determinístico e marca o que precisa de julgamento humano.

Uso:
  python3 falta_migrar.py --lista-A        # imprime Lista A (TSV)
  python3 falta_migrar.py --lista-B        # imprime Lista B (veredictos)
  python3 falta_migrar.py --resumo         # contagens dos dois eixos
"""

import argparse
import os
import re
import sys
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
INV = os.path.join(REPO, "00_nucleo", "diagnosticos",
                   "typst-cobertura-vanilla-vs-cristalino.md")

sys.path.insert(0, HERE)
import decompor_so_vanilla as D  # noqa: E402


# ---------------------------------------------------------------------------
# Parsing de tabelas markdown do Inventário 148.
# ---------------------------------------------------------------------------
def _split_row(line):
    # "| a | b | c |" -> ["a","b","c"]
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    return cells


def _strikethrough(s):
    return s.strip().startswith("~~")


def parse_lista_A():
    """Linhas ausente/parcial da Tabela A (A.1–A.9). Cada linha:
    dict(categoria, feature, classe, ressalva, referencia, resolvido)."""
    with open(INV, encoding="utf-8") as f:
        linhas = f.readlines()

    # janela: de "## Tabela A" até "## Tabela B"
    ini = next(i for i, l in enumerate(linhas) if l.startswith("## Tabela A"))
    fim = next(i for i, l in enumerate(linhas) if l.startswith("## Tabela B"))

    categoria = "?"
    header = None
    col = {}
    out = []
    for l in linhas[ini:fim]:
        m = re.match(r"^###\s+A\.\d+\s+—\s+(.*)$", l)
        if m:
            categoria = m.group(1).strip()
            header = None
            continue
        if not l.lstrip().startswith("|"):
            continue
        cells = _split_row(l)
        if set("".join(cells)) <= set("-: "):  # linha separadora
            continue
        if header is None:
            header = [c.lower() for c in cells]
            col = {}
            for i, h in enumerate(header):
                if h.startswith("feature") or h in ("variant", "símbolo", "simbolo"):
                    col["feature"] = i
                elif h.startswith("cristalino"):
                    col["classe"] = i
                elif h.startswith("refer"):
                    col["ref"] = i
                elif h.startswith("nota"):
                    col["nota"] = i
            continue
        if "classe" not in col or col["classe"] >= len(cells):
            continue
        classe_cell = cells[col["classe"]]
        mcl = re.search(r"`(ausente|parcial)`", classe_cell)
        if not mcl:
            continue
        feature = cells[col.get("feature", 0)]
        out.append({
            "categoria": categoria,
            "feature": feature.replace("~~", "").strip(),
            "classe": mcl.group(1),
            "ressalva": cells[col["nota"]] if "nota" in col and col["nota"] < len(cells) else "",
            "referencia": cells[col["ref"]] if "ref" in col and col["ref"] < len(cells) else "",
            "resolvido": _strikethrough(feature),
        })
    return out


def parse_tabela_C():
    """Roadmap por feature da Tabela C (vista cruzada parciais/ausentes).
    Retorna dict feature_normalizada -> (bloqueantes, roadmap, resolvido)."""
    with open(INV, encoding="utf-8") as f:
        linhas = f.readlines()
    try:
        ini = next(i for i, l in enumerate(linhas) if l.startswith("## Tabela C"))
        fim = next(i for i, l in enumerate(linhas[ini + 1:], ini + 1)
                   if l.startswith("## "))
    except StopIteration:
        return {}
    out = {}
    for l in linhas[ini:fim]:
        if not l.lstrip().startswith("|"):
            continue
        cells = _split_row(l)
        if len(cells) < 3 or set("".join(cells)) <= set("-: "):
            continue
        if cells[0].lower().startswith("feature"):
            continue
        feat = _norm_feature(cells[0])
        if not feat:
            continue
        out[feat] = {
            "bloqueantes": cells[1],
            "roadmap": cells[2],
            "resolvido": _strikethrough(cells[0]),
        }
    return out


def _norm_feature(s):
    """Normaliza um nome de feature para join: tira backticks, ~~, args,
    fica com o identificador-chave em minúsculas."""
    s = s.replace("~~", "")
    m = re.search(r"`([^`]+)`", s)
    if m:
        s = m.group(1)
    s = re.split(r"[\(\s]", s.strip())[0]
    s = s.strip("`* ").lower()
    # tira prefixo de namespace comum (text./layout./Value::)
    s = re.sub(r"^(value::|content::|shapekind::)", "", s)
    return s


# ---------------------------------------------------------------------------
# Índice de features do Inventário (TODAS as classes) p/ cruzar a Lista B.
# ---------------------------------------------------------------------------
def indice_features():
    """feature_normalizada -> classe (implementado/parcial/ausente/scope-out).
    Varre o ficheiro inteiro; última ocorrência vence (entrada mais específica)."""
    idx = {}
    with open(INV, encoding="utf-8") as f:
        for l in f:
            if not l.lstrip().startswith("|"):
                continue
            cells = _split_row(l)
            if len(cells) < 3:
                continue
            mcl = None
            for c in cells:
                mcl = re.search(r"`(implementado⁺?|parcial|ausente|scope-out)`", c)
                if mcl:
                    break
            if not mcl:
                continue
            feat = _norm_feature(cells[0])
            if feat and len(feat) > 1:
                cls = mcl.group(1).replace("⁺", "")
                idx[feat] = cls
    return idx


# ---------------------------------------------------------------------------
# Lista B — candidatos de língua do resíduo 385, cruzados com o inventário.
# ---------------------------------------------------------------------------
EXEC_MOD = ("typst_layout::", "typst_eval::vm", "typst_eval::call",
            "typst_eval::flow", "typst_eval::access", "typst_eval::math",
            "typst_realize::", "typst_library::math::ir",
            "typst_library::introspection::introspector",
            "typst_library::introspection::convergence",
            "typst_library::introspection::query_",
            "typst_library::introspection::locator",
            "typst_library::introspection::location")


def candidatos_lingua():
    """Resíduo balde 3 do 385, filtrado a candidatos de língua (ADR-0107):
    descarta mecânica de execução; mantém tipos de nível-de-língua e free-fns."""
    entrada = os.path.join(REPO, "00_nucleo", "diagnosticos",
                           "entrada-lente-so-vanilla.2026-06-21.txt")
    itens = D.read_entrada(entrada)
    lit_s, lit_m = D.extrair_literatura(REPO)
    scope = D.extrair_scope_out_features(REPO)
    cand = []
    for it in itens:
        b, _, _ = D.classificar(it, lit_s, lit_m, scope)
        if b != "3":
            continue
        path = it["path"] or ""
        if any(path.startswith(m) for m in EXEC_MOD):
            continue  # mecânica de execução — fora (ADR-0107)
        kind = it["kind"]
        nome = it["nome"]
        is_type = kind in ("struct", "enum", "type", "trait")
        is_freefn = kind == "fn" and "::" not in nome
        if is_type or is_freefn:
            cand.append(it)
    return cand


def lista_B():
    idx = indice_features()
    cand = candidatos_lingua()
    vereditos = []
    for it in cand:
        leaf = it["nome"].split("::")[-1]
        chave = _norm_feature(leaf)
        cls = idx.get(chave)
        if cls is None:
            # tenta o módulo-folha (ex.: bytes -> Value::Bytes)
            mod_leaf = (it["path"] or "").split("::")
            alt = mod_leaf[-2] if len(mod_leaf) >= 2 else ""
            cls = idx.get(_norm_feature(alt))
        if cls in ("implementado", "parcial"):
            v = "nao-divida-migrado" if cls == "implementado" else "parcial-ja-listado"
        elif cls == "scope-out":
            v = "scope-out"
        elif cls == "ausente":
            v = "divida-confirmada"
        else:
            v = "lacuna-inventario"  # sem entrada → revisão humana
        vereditos.append({**it, "chave": chave, "classe_inv": cls or "—", "veredicto": v})
    return vereditos


# ---------------------------------------------------------------------------
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lista-A", action="store_true")
    ap.add_argument("--lista-B", action="store_true")
    ap.add_argument("--resumo", action="store_true")
    args = ap.parse_args()

    if args.lista_A or args.resumo:
        A = parse_lista_A()
        C = parse_tabela_C()
        ativos = [r for r in A if not r["resolvido"]]
        resolvidos = [r for r in A if r["resolvido"]]
        print(f"# Lista A — {len(A)} linhas ausente/parcial na Tabela A "
              f"({len(ativos)} ativas, {len(resolvidos)} resolvidas/struck)")
        by_cat = Counter(r["categoria"] for r in ativos)
        by_cls = Counter(r["classe"] for r in ativos)
        print(f"#   por classe: {dict(by_cls)}")
        print(f"#   por categoria: {dict(by_cat)}")
        if args.lista_A:
            print("\ncategoria\tclasse\tfeature\troadmap(C)\tressalva")
            for r in sorted(A, key=lambda x: (x["categoria"], x["classe"], x["feature"])):
                road = C.get(_norm_feature(r["feature"]), {}).get("roadmap", "")
                flag = " [RESOLVIDO?]" if r["resolvido"] else ""
                print(f"{r['categoria']}\t{r['classe']}{flag}\t{r['feature']}\t"
                      f"{road}\t{r['ressalva'][:80]}")

    if args.lista_B or args.resumo:
        B = lista_B()
        cnt = Counter(r["veredicto"] for r in B)
        print(f"\n# Lista B — {len(B)} candidatos de língua julgados")
        for v, n in cnt.most_common():
            print(f"#   {v:24s} {n}")
        if args.lista_B:
            print("\nveredicto\tkind\tchave\tclasse_inv\tpath")
            for r in sorted(B, key=lambda x: (x["veredicto"], x["path"])):
                print(f"{r['veredicto']}\t{r['kind']}\t{r['chave']}\t"
                      f"{r['classe_inv']}\t{r['path']}")


if __name__ == "__main__":
    main()
