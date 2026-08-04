#!/usr/bin/env python3
"""P956 Fase C.1 — comparação directa de operadores de blocos de texto.

Extrai os blocos de texto de dois PDFs (já descomprimidos com mutool clean -d),
normaliza (remove BDC/EMC do vanilla, mapeia nomes de fonte, Tj/TJ equivalentes)
e compara a sequência de operadores por bloco, em ordem de ocorrência.
"""
import re
import sys
from collections import Counter

OPS = ["cm", "cs", "scn", "CS", "SCN", "BT", "Tr", "Tf", "Tm", "Td", "TJ", "Tj",
       "ET", "q", "Q", "rg", "w", "Tc"]

def content_streams(pdf_bytes):
    out = []
    for m in re.finditer(rb"stream\r?\n(.*?)endstream", pdf_bytes, re.S):
        s = m.group(1)
        if b"BT" in s:
            out.append(s.decode("latin-1"))
    return out

def text_blocks(stream):
    """Divide o stream em blocos que contêm BT…ET, com o envelope à volta."""
    # Remove marked content (eixo acessibilidade — fora do escopo P956)
    stream = re.sub(r"/\w+\s*<<[^>]*>>\s*BDC", " ", stream)
    stream = re.sub(r"\bEMC\b", " ", stream)
    blocks = []
    # janela: de q(ou início) antes do BT até Q(ou fim) depois do ET
    for m in re.finditer(r"(q\s)?.*?BT\s.*?ET\s*(Q)?", stream, re.S):
        blocks.append(m.group(0))
    return blocks

def signature(block):
    """Sequência ordenada de nomes de operadores no bloco (TJ/Tj → TJ)."""
    toks = re.findall(r"\b(cm|cs|scn|CS|SCN|BT|Tr|Tf|Tm|Td|TJ|Tj|ET|q|Q|rg|Tc|w)\b", block)
    return tuple("TJ" if t == "Tj" else t for t in toks)

def numbers(block):
    return re.findall(r"-?\d+\.?\d*", block)

def main(path_a, path_b):
    A = [b for s in content_streams(open(path_a, "rb").read()) for b in text_blocks(s)]
    B = [b for s in content_streams(open(path_b, "rb").read()) for b in text_blocks(s)]
    print(f"blocos: A={len(A)} B={len(B)}")
    sig_a = Counter(signature(b) for b in A)
    sig_b = Counter(signature(b) for b in B)
    print("\n── assinaturas A (cristalino verbose):")
    for s, n in sig_a.most_common(10):
        print(f"  {n:5d}  {' '.join(s)}")
    print("\n── assinaturas B (vanilla):")
    for s, n in sig_b.most_common(10):
        print(f"  {n:5d}  {' '.join(s)}")
    # emparelhamento sequencial simples
    n = min(len(A), len(B))
    match = sum(1 for i in range(n) if signature(A[i]) == signature(B[i]))
    print(f"\nemparelhamento sequencial: {match}/{n} blocos com assinatura idêntica")
    shown = 0
    for i in range(n):
        if signature(A[i]) != signature(B[i]) and shown < 5:
            print(f"\n── divergência no bloco {i}:")
            print("  A:", " ".join(signature(A[i])))
            print("  B:", " ".join(signature(B[i])))
            print("  A raw:", " ".join(A[i].split())[:160])
            print("  B raw:", " ".join(B[i].split())[:160])
            shown += 1

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
