#!/usr/bin/env python3
"""compare.py — comparação geométrica glifo a glifo entre dois PDFs (cristalino vs vanilla).

Passo 948. Extrai de cada PDF, via caminhada directa do content stream (pikepdf),
a posição absoluta, o texto (ToUnicode) e a fonte/tamanho de cada operador de
desenho de glifo (Tj/TJ). Emparelha glifos por secção (cabeçalhos numerados
"N." na margem esquerda), alinha por ordem de leitura com âncora no codepoint
(difflib) e reporta deltas de posição relativos à origem da secção — imune a
diferenças legítimas de tamanho/posição de página.

Uso:
    compare.py A.pdf B.pdf [--limiar 0.5] [--json saida.json] [--secoes N,M,...]

Auto-teste rápido:
    compare.py A.pdf A.pdf        # deltas devem ser ~0 em tudo
"""

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from difflib import SequenceMatcher

import pikepdf


# ─── extracção ───────────────────────────────────────────────────────────────

@dataclass
class Glyph:
    text: str          # codepoint(s) via ToUnicode ("∅" se não mapeado)
    x: float
    y: float
    size: float
    font: str


def _load_tounicode(font_obj) -> dict:
    """Parse bfchar/bfrange de um /ToUnicode CMap → {cid: str}."""
    out = {}
    tuni = font_obj.get("/ToUnicode")
    if tuni is None:
        return out
    try:
        txt = bytes(tuni.read_bytes()).decode("latin-1")
    except Exception:
        return out

    def hexs(h):
        b = bytes.fromhex(h)
        return b.decode("utf-16-be", "replace")

    for block in re.finditer(r"beginbfchar(.*?)endbfchar", txt, re.S):
        for m in re.finditer(r"<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>", block.group(1)):
            out[int(m.group(1), 16)] = hexs(m.group(2))
    for block in re.finditer(r"beginbfrange(.*?)endbfrange", txt, re.S):
        for m in re.finditer(
            r"<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>", block.group(1)
        ):
            lo, hi, dst = int(m.group(1), 16), int(m.group(2), 16), m.group(3)
            base = int(dst, 16)
            for cid in range(lo, hi + 1):
                out[cid] = hexs(f"{base + cid - lo:0{len(dst)}X}")
        for m in re.finditer(
            r"<([0-9A-Fa-f]+)>\s*<([0-9A-Fa-f]+)>\s*\[(.*?)\]", block.group(1), re.S
        ):
            lo = int(m.group(1), 16)
            for i, dst in enumerate(re.findall(r"<([0-9A-Fa-f]+)>", m.group(3))):
                out[lo + i] = hexs(dst)
    return out


def _load_widths(font_obj) -> tuple:
    """Larguras de glifo (/W do CIDFont descendente + /DW) → ({cid: w_du}, dw_du)."""
    dw = 1000
    widths = {}
    try:
        desc = font_obj["/DescendantFonts"][0]
        dw = int(desc.get("/DW", 1000))
        w = desc.get("/W")
        if w is not None:
            i = 0
            w = list(w)
            while i < len(w):
                start = int(w[i])
                nxt = w[i + 1]
                if isinstance(nxt, pikepdf.Array):
                    for k, val in enumerate(nxt):
                        widths[start + k] = float(val)
                    i += 2
                else:
                    end, val = int(nxt), float(w[i + 2])
                    for cid in range(start, end + 1):
                        widths[cid] = val
                    i += 3
    except Exception:
        pass
    return widths, dw


def _mat_mul(a, b):
    return [
        a[0] * b[0] + a[1] * b[2],
        a[0] * b[1] + a[1] * b[3],
        a[2] * b[0] + a[3] * b[2],
        a[2] * b[1] + a[3] * b[3],
        a[4] * b[0] + a[5] * b[2] + b[4],
        a[4] * b[1] + a[5] * b[3] + b[5],
    ]


def extract_glyphs(pdf_path: str) -> list:
    """Caminha o content stream e devolve [Glyph] com posições absolutas
    (y normalizado para "para baixo positivo" — a orientação de cada PDF é
    detectada pela posição dos primeiros glifos face à MediaBox)."""
    pdf = pikepdf.open(pdf_path)
    glyphs = []
    for page in pdf.pages:
        res = page["/Resources"]
        fonts = {}
        widths = {}
        for name, fobj in res.get("/Font", {}).items():
            fonts[str(name)] = _load_tounicode(fobj)
            widths[str(name)] = _load_widths(fobj)
        contents = page.get("/Contents")
        if contents is None:
            continue
        mb = [float(v) for v in page["/MediaBox"]]
        page_h = mb[3] - mb[1]
        ctm = [1, 0, 0, 1, 0, 0]
        stack = []
        tm = [1, 0, 0, 1, 0, 0]
        tlm = [1, 0, 0, 1, 0, 0]
        cur_font, cur_size = None, 0.0
        page_glyphs = []

        for operands, op in pikepdf.parse_content_stream(contents):
            op = str(op)
            if op == "q":
                stack.append(list(ctm))
            elif op == "Q":
                if stack:
                    ctm = stack.pop()
            elif op == "cm":
                ctm = _mat_mul(ctm, [float(v) for v in operands])
            elif op == "BT":
                tm = [1, 0, 0, 1, 0, 0]
                tlm = list(tm)
            elif op in ("Td", "TD"):
                tlm = _mat_mul(tlm, [1, 0, 0, 1, float(operands[0]), float(operands[1])])
                tm = list(tlm)
            elif op == "Tm":
                tm = [float(v) for v in operands]
                tlm = list(tm)
            elif op == "Tf":
                cur_font = str(operands[0])
                cur_size = float(operands[1])
            elif op in ("Tj", "TJ", "'", '"'):
                items = list(operands[0]) if op == "TJ" else [operands[-1]]
                cmap = fonts.get(cur_font, {})
                w_tab, w_def = widths.get(cur_font, ({}, 1000))
                # posição corrente no espaço de texto (translação de tm)
                tx, ty = tm[4], tm[5]
                for it in items:
                    if isinstance(it, (int, float)):
                        # ajuste TJ: milésimos de em, para trás na escrita
                        tx -= float(it) / 1000.0 * cur_size
                        continue
                    if not isinstance(it, (bytes, pikepdf.String)):
                        continue
                    raw = bytes(it)
                    for i in range(0, len(raw), 2):
                        cid = int.from_bytes(raw[i : i + 2], "big")
                        gx = ctm[0] * tx + ctm[2] * ty + ctm[4]
                        gy = ctm[1] * tx + ctm[3] * ty + ctm[5]
                        page_glyphs.append(
                            Glyph(cmap.get(cid, "∅"), gx, gy, cur_size, cur_font or "")
                        )
                        tx += w_tab.get(cid, w_def) / 1000.0 * cur_size
        # normalização de orientação: se os primeiros glifos (ordem do stream,
        # que começa no topo da página) estão na metade INFERIOR da página
        # (y grande), o espaço é bottom-origin — inverter para y-para-baixo.
        if page_glyphs and page_h > 0:
            top_probe = page_glyphs[: min(100, len(page_glyphs))]
            if sum(g.y for g in top_probe) / len(top_probe) > page_h / 2:
                for g in page_glyphs:
                    g.y = page_h - g.y
        glyphs.extend(page_glyphs)
    return glyphs


# ─── secções ─────────────────────────────────────────────────────────────────

@dataclass
class Section:
    num: int
    glyphs: list = field(default_factory=list)


def split_sections(glyphs: list) -> list:
    """Divide por cabeçalhos 'N.' na margem esquerda (x < 60). Dois formatos
    suportados: run único 'N. Título' (vanilla) ou dígitos separados 'N'+'.'
    (cristalino emite glifo a glifo). Sem cabeçalhos: secção 0 única."""
    heads = []
    i = 0
    while i < len(glyphs):
        g = glyphs[i]
        if g.x >= 60:
            i += 1
            continue
        if re.match(r"^\d{1,2}\.(\s|$)", g.text):
            heads.append(i)
            i += 1
            continue
        if re.fullmatch(r"\d", g.text):
            # run de dígitos (1-2) seguido de '.': '1'+'.' ou '1'+'0'+'.'
            j = i + 1
            while j < len(glyphs) and re.fullmatch(r"\d", glyphs[j].text) and j - i < 2:
                j += 1
            if (
                j < len(glyphs)
                and glyphs[j].text == "."
                and all(abs(glyphs[k].y - g.y) < 3.0 for k in range(i, j + 1))
            ):
                heads.append(i)
                i = j + 1  # consumir dígitos + ponto — não re-detectar '0' de '10'
                continue
        i += 1
    if not heads:
        return [Section(0, glyphs)]
    secs = []
    for j, start in enumerate(heads):
        end = heads[j + 1] if j + 1 < len(heads) else len(glyphs)
        m = re.match(r"^\d{1,2}", glyphs[start].text)
        if re.fullmatch(r"\d", glyphs[start].text) and re.fullmatch(
            r"\d", glyphs[start + 1].text
        ):
            num_txt = glyphs[start].text + glyphs[start + 1].text
        else:
            num_txt = m.group(0)
        secs.append(Section(int(num_txt), glyphs[start:end]))
    return secs


# ─── emparelhamento ──────────────────────────────────────────────────────────

def _lines(glyphs: list) -> list:
    """Agrupa em linhas por proximidade de y (3pt), ordenadas por y e depois x
    dentro da linha."""
    ordered = sorted(glyphs, key=lambda g: (g.y, g.x))
    lines = []
    for g in ordered:
        if lines and abs(g.y - lines[-1][-1].y) < 3.0:
            lines[-1].append(g)
        else:
            lines.append([g])
    return [sorted(line, key=lambda g: g.x) for line in lines]


def _pair_seq(la: list, lb: list):
    """Emparelha duas sequências de glifos por difflib sobre os codepoints.
    Devolve (pares, não_emparelhados_a, não_emparelhados_b)."""
    sa = "".join(g.text[0] if g.text else "∅" for g in la)
    sb = "".join(g.text[0] if g.text else "∅" for g in lb)
    sm = SequenceMatcher(a=sa, b=sb, autojunk=False)
    pairs, un_a, un_b = [], [], []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            for k in range(i2 - i1):
                pairs.append((la[i1 + k], lb[j1 + k]))
        elif tag == "replace":
            n = min(i2 - i1, j2 - j1)
            for k in range(n):
                pairs.append((la[i1 + k], lb[j1 + k]))
            un_a.extend(la[i1 + n : i2])
            un_b.extend(lb[j1 + n : j2])
        elif tag == "delete":
            un_a.extend(la[i1:i2])
        elif tag == "insert":
            un_b.extend(lb[j1:j2])
    return pairs, un_a, un_b


def pair_glyphs(a_glyphs: list, b_glyphs: list):
    """Emparelha por sequência de leitura da secção (difflib sobre codepoints),
    e devolve para cada par a origem da sua "construção" (cluster de linha) no
    respectivo lado — os deltas ficam relativos à construção sem depender de
    emparelhamento linha-a-linha (frágil quando os dois compiladores agrupam
    as mesmas construções em números diferentes de linhas)."""
    la = [g for line in _lines(a_glyphs) for g in line]
    lb = [g for line in _lines(b_glyphs) for g in line]
    pairs, un_a, un_b = _pair_seq(la, lb)
    # origem da construção de cada glifo: min x/y do seu cluster de linha
    orig = {}
    for glyphs in (a_glyphs, b_glyphs):
        for line in _lines(glyphs):
            ox = min(g.x for g in line)
            oy = min(g.y for g in line)
            for g in line:
                orig[id(g)] = (ox, oy)
    out = []
    for ga_, gb_ in pairs:
        oxa, oya = orig[id(ga_)]
        oxb, oyb = orig[id(gb_)]
        out.append((ga_, gb_, (oxa, oya), (oxb, oyb)))
    return out, un_a, un_b


# ─── relatório ───────────────────────────────────────────────────────────────

def compare(path_a: str, path_b: str, limiar: float, only_sections=None) -> dict:
    ga, gb = extract_glyphs(path_a), extract_glyphs(path_b)
    secs_a = {s.num: s for s in split_sections(ga)}
    secs_b = {s.num: s for s in split_sections(gb)}
    report = {"a": path_a, "b": path_b, "limiar": limiar, "sections": []}
    for num in sorted(set(secs_a) & set(secs_b)):
        if only_sections and num not in only_sections:
            continue
        sa, sb = secs_a[num], secs_b[num]
        pairs, un_a, un_b = pair_glyphs(sa.glyphs, sb.glyphs)
        rows = []
        for ga_, gb_, (oxa, oya), (oxb, oyb) in pairs:
            dx = (ga_.x - oxa) - (gb_.x - oxb)
            dy = (ga_.y - oya) - (gb_.y - oyb)
            rows.append(
                {
                    "texto": ga_.text if ga_.text == gb_.text else f"{ga_.text}|{gb_.text}",
                    "ax": round(ga_.x - oxa, 3), "ay": round(ga_.y - oya, 3),
                    "bx": round(gb_.x - oxb, 3), "by": round(gb_.y - oyb, 3),
                    "dx": round(dx, 3), "dy": round(dy, 3),
                    "flag": abs(dx) > limiar or abs(dy) > limiar,
                }
            )
        if not rows:
            continue
        abs_dx = sorted(abs(r["dx"]) for r in rows)
        abs_dy = sorted(abs(r["dy"]) for r in rows)
        report["sections"].append(
            {
                "secao": num,
                "glifos_a": len(sa.glyphs),
                "glifos_b": len(sb.glyphs),
                "emparelhados": len(rows),
                "nao_emparelhados_a": len(un_a),
                "nao_emparelhados_b": len(un_b),
                "max_abs_dx": round(abs_dx[-1], 3),
                "max_abs_dy": round(abs_dy[-1], 3),
                "mediana_abs_dx": round(abs_dx[len(abs_dx) // 2], 3),
                "mediana_abs_dy": round(abs_dy[len(abs_dy) // 2], 3),
                "acima_limiar": sum(1 for r in rows if r["flag"]),
                "detalhe": [r for r in rows if r["flag"]],
            }
        )
    return report


def main():
    ap = argparse.ArgumentParser(description="Comparação geométrica de glifos entre dois PDFs (P948).")
    ap.add_argument("pdf_a")
    ap.add_argument("pdf_b")
    ap.add_argument("--limiar", type=float, default=0.5, help="limiar de divergência em pt (default 0.5)")
    ap.add_argument("--json", dest="json_out", help="escreve relatório completo em JSON")
    ap.add_argument("--secoes", help="só estas secções, ex.: 4,5,21")
    args = ap.parse_args()
    only = {int(x) for x in args.secoes.split(",")} if args.secoes else None

    rep = compare(args.pdf_a, args.pdf_b, args.limiar, only)
    if args.json_out:
        with open(args.json_out, "w") as f:
            json.dump(rep, f, ensure_ascii=False, indent=1)

    print(f"A: {rep['a']}\nB: {rep['b']}  (limiar {rep['limiar']}pt)")
    print(f"{'sec':>4} {'glifos':>12} {'emparelh.':>10} {'s/par A':>8} {'s/par B':>8}"
          f" {'med|dx|':>8} {'med|dy|':>8} {'max|dx|':>8} {'max|dy|':>8} {'>lim':>5}")
    for s in sorted(rep["sections"], key=lambda s: -max(s["max_abs_dx"], s["max_abs_dy"])):
        print(
            f"{s['secao']:>4} {s['glifos_a']:>6}/{s['glifos_b']:<5} {s['emparelhados']:>10}"
            f" {s['nao_emparelhados_a']:>8} {s['nao_emparelhados_b']:>8}"
            f" {s['mediana_abs_dx']:>8} {s['mediana_abs_dy']:>8}"
            f" {s['max_abs_dx']:>8} {s['max_abs_dy']:>8} {s['acima_limiar']:>5}"
        )
    total_flag = sum(s["acima_limiar"] for s in rep["sections"])
    total = sum(s["emparelhados"] for s in rep["sections"])
    print(f"\nTotal: {total_flag}/{total} glifos acima do limiar em {len(rep['sections'])} secções")


if __name__ == "__main__":
    sys.exit(main())
