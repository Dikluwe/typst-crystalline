#!/usr/bin/env python3
"""Teste do script do eixo 1 do Passo 386 (falta_migrar.py).

Verifica (critérios 6.3 / 6.6):
  - Lista A extrai só linhas ausente/parcial da Tabela A, com categoria;
  - Lista B dá exatamente um veredicto por candidato (reconciliação);
  - determinismo: re-rodar dá contagens idênticas.

Roda standalone: `python3 lab/parity/tools/test_falta_migrar.py`.
"""

import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import falta_migrar as F


def test_lista_A_so_ausente_parcial():
    A = F.parse_lista_A()
    assert A, "Lista A vazia — parser quebrou?"
    classes = set(r["classe"] for r in A)
    assert classes <= {"ausente", "parcial"}, classes
    # toda linha tem categoria não-vazia
    assert all(r["categoria"] and r["categoria"] != "?" for r in A), \
        "linha sem categoria"


def test_norm_feature():
    assert F._norm_feature("`lorem`") == "lorem"
    assert F._norm_feature("~~`footnote(body)`~~") == "footnote"
    assert F._norm_feature("`Value::Bytes`") == "bytes"
    assert F._norm_feature("`gradient(...)`") == "gradient"


def test_lista_B_um_veredicto_por_candidato():
    B = F.lista_B()
    assert B, "Lista B vazia"
    vereditos = {"divida-confirmada", "nao-divida-migrado",
                 "parcial-ja-listado", "scope-out", "lacuna-inventario"}
    for r in B:
        assert r["veredicto"] in vereditos, r["veredicto"]
    # determinismo: re-rodar dá a mesma contagem
    c1 = Counter(r["veredicto"] for r in B)
    c2 = Counter(r["veredicto"] for r in F.lista_B())
    assert c1 == c2, ("não-determinístico", c1, c2)
    print("OK veredictos:", dict(c1), "soma=", sum(c1.values()))


def test_indice_features_tem_classes_conhecidas():
    idx = F.indice_features()
    assert idx, "índice vazio"
    valores = set(idx.values())
    assert valores <= {"implementado", "parcial", "ausente", "scope-out"}, valores
    # sanity: features conhecidas presentes
    assert idx.get("lorem") == "ausente", idx.get("lorem")


if __name__ == "__main__":
    fns = [v for k, v in sorted(globals().items()) if k.startswith("test_")]
    for fn in fns:
        fn()
        print("PASS", fn.__name__)
    print(f"\n{len(fns)} testes passaram.")
