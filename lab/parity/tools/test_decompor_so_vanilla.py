#!/usr/bin/env python3
"""Teste do script de medição do Passo 385 (decompor_so_vanilla.py).

Verifica as propriedades que o passo exige (critérios 6.1 / 6.2):
  - reconciliação fechada: os 4 baldes somam o total da entrada;
  - determinismo: classificar o mesmo item dá sempre o mesmo balde;
  - as regras-âncora classificam como esperado (mecânica, scope-out, etc.).

Roda standalone: `python3 lab/parity/tools/test_decompor_so_vanilla.py`
(sem framework; falha com AssertionError + mensagem).
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import decompor_so_vanilla as D

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))


def _classif(kind, trait, nome, path, lit_s=None, lit_m=None, scope=None):
    return D.classificar(
        {"kind": kind, "trait": trait, "nome": nome, "path": path},
        lit_s or set(), lit_m or set(), scope or set())


def test_regra_topologia():
    # crate de backend → balde 2 topologia, com âncora.
    b, motivo, crate = _classif("struct", "", "Pdf", "typst_pdf::pdf::Pdf")
    assert (b, motivo) == ("2", "topologia"), (b, motivo)
    assert crate == "typst_pdf"


def test_regra_trait_impl_e_macro():
    # impl de trait → mecânica (ADR-0107), independente do crate.
    b, motivo, _ = _classif("fn", "Debug", "Foo::fmt", "typst_library::x::Foo::fmt")
    assert (b, motivo) == ("M", "trait-impl"), (b, motivo)
    b, motivo, _ = _classif("macro", "", "elem", "typst_library::x::elem")
    assert (b, motivo) == ("M", "macro"), (b, motivo)


def test_regra_vtable_adr_0026():
    b, motivo, _ = _classif(
        "trait", "", "NativeElement",
        "typst_library::foundations::content::element::NativeElement")
    assert (b, motivo) == ("2", "vtable-ADR-0026"), (b, motivo)


def test_regra_renome_simbolo():
    # tipo cujo nome está na literatura → balde 1.
    b, motivo, chave = _classif(
        "struct", "", "HeadingElem", "typst_library::model::heading::HeadingElem",
        lit_s={"HeadingElem"})
    assert (b, motivo) == ("1", "literatura-simbolo"), (b, motivo)
    assert chave == "HeadingElem"


def test_metodo_de_tipo_registrado_e_mecanica():
    # método de tipo que está na literatura → mecânica desse tipo, não balde 1.
    b, motivo, _ = _classif(
        "fn", "", "Counter::resolve", "typst_library::introspection::counter::Counter::resolve",
        lit_s={"Counter"})
    assert (b, motivo) == ("M", "metodo-de-tipo-registrado"), (b, motivo)


def test_residuo_quando_sem_ancora():
    b, motivo, _ = _classif("struct", "", "Frobnicator",
                            "typst_library::text::frob::Frobnicator")
    assert (b, motivo) == ("3", "residuo"), (b, motivo)


def test_reconciliacao_fechada_na_entrada_real():
    entrada = os.path.join(REPO, "00_nucleo", "diagnosticos",
                           "entrada-lente-so-vanilla.2026-06-21.txt")
    if not os.path.exists(entrada):
        print("SKIP: entrada real ausente —", entrada)
        return
    itens = D.read_entrada(entrada)
    lit_s, lit_m = D.extrair_literatura(REPO)
    scope = D.extrair_scope_out_features(REPO)
    contagem = {"1": 0, "2": 0, "M": 0, "3": 0}
    for it in itens:
        b, _, _ = D.classificar(it, lit_s, lit_m, scope)
        contagem[b] += 1
    soma = sum(contagem.values())
    assert soma == len(itens), f"reconciliação falhou: {soma} != {len(itens)}"
    # determinismo: re-classificar dá idêntico.
    contagem2 = {"1": 0, "2": 0, "M": 0, "3": 0}
    for it in itens:
        b, _, _ = D.classificar(it, lit_s, lit_m, scope)
        contagem2[b] += 1
    assert contagem == contagem2, "classificação não-determinística"
    print(f"OK reconciliação: {contagem} soma={soma}")


if __name__ == "__main__":
    fns = [v for k, v in sorted(globals().items()) if k.startswith("test_")]
    for fn in fns:
        fn()
        print("PASS", fn.__name__)
    print(f"\n{len(fns)} testes passaram.")
