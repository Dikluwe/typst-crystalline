#!/usr/bin/env python3
"""Validação de paridade EXIF — imagens JPEG orientadas cristalino vs vanilla.

Convenção de rasterização do projecto (P778/P779):
    mutool draw -o <out>.png -r 300 <in>.pdf

Esta convenção foi estabelecida porque `pdftoppm -r 150` (usado numa versão
anterior deste script) introduziu resíduos de AE (195/138/0) que não existem
com outras ferramentas/resoluções nem reflectem diferenças reais entre
os PDFs cristalino e vanilla. Ver relatório P778.
"""

import subprocess
import sys
from pathlib import Path

# Ferramenta e resolução fixas para medições de paridade de imagem.
RASTERIZER_CMD = ["mutool", "draw", "-r", "300"]

ROOT = Path(__file__).resolve().parent.parent.parent.parent / "temp_p776"
PROJECT = ROOT.parent
VENV_PYTHON = PROJECT / "lab" / ".venv" / "bin" / "python"
CRYS_BIN = PROJECT / "target" / "release" / "typst"
VAN_BIN = PROJECT / "lab" / "typst-original" / "target" / "release" / "typst"


def run(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True, **kw)


def generate_images():
    from PIL import Image
    script = """
from PIL import Image
w, h = 200, 100
for orient in range(1, 9):
    img = Image.new('RGB', (w, h))
    for x in range(w):
        for y in range(h):
            r = int(255 * x / (w - 1))
            g = int(255 * y / (h - 1))
            b = 128
            img.putpixel((x, y), (r, g, b))
    exif = img.getexif()
    exif[0x0112] = orient
    path = f'orient{orient}.jpg'
    img.save(path, 'JPEG', quality=95, exif=exif)
    print(path)
"""
    r = run([str(VENV_PYTHON), "-c", script], cwd=ROOT)
    if r.returncode != 0:
        print("Erro ao gerar imagens:", r.stderr)
        sys.exit(1)
    print(r.stdout)


def generate_typst_files():
    for orient in range(1, 9):
        src = ROOT / f"orient{orient}.typ"
        src.write_text(f'#image("orient{orient}.jpg")\n')


def compile_crys():
    for orient in range(1, 9):
        src = ROOT / f"orient{orient}.typ"
        out = ROOT / f"crys-orient{orient}.pdf"
        r = run([str(CRYS_BIN), str(src), str(out)], cwd=ROOT)
        if r.returncode != 0:
            print(f"Erro ao compilar crys orient {orient}:", r.stderr)
            sys.exit(1)


def compile_vanilla():
    for orient in range(1, 9):
        src = ROOT / f"orient{orient}.typ"
        out = ROOT / f"vanilla-orient{orient}.pdf"
        r = run([str(VAN_BIN), "compile", str(src), str(out)], cwd=ROOT)
        if r.returncode != 0:
            print(f"Erro ao compilar vanilla orient {orient}:", r.stderr)
            sys.exit(1)


def rasterize(pdf, png):
    cmd = list(RASTERIZER_CMD) + ["-o", str(png), str(pdf), "1"]
    r = run(cmd)
    if r.returncode != 0:
        print(f"Erro ao rasterizar {pdf}:", r.stderr)
        sys.exit(1)
    return png


def compare_pngs(a, b):
    r = run(["compare", "-metric", "AE", str(a), str(b), "null:"], cwd=ROOT)
    text = r.stdout + r.stderr
    for line in text.splitlines():
        try:
            return int(line.strip())
        except ValueError:
            continue
    return None


def pdfimages_info(pdf):
    r = run(["pdfimages", "-list", str(pdf)], cwd=ROOT)
    lines = r.stdout.strip().splitlines()
    if len(lines) >= 2:
        return lines[-1]
    return ""


def main():
    generate_images()
    generate_typst_files()
    compile_crys()
    compile_vanilla()

    print("\n=== Comparação por orientação ===")
    all_zero = True
    for orient in range(1, 9):
        crys_pdf = ROOT / f"crys-orient{orient}.pdf"
        van_pdf = ROOT / f"vanilla-orient{orient}.pdf"
        crys_png = ROOT / f"crys-orient{orient}.png"
        van_png = ROOT / f"vanilla-orient{orient}.png"
        rasterize(crys_pdf, crys_png)
        rasterize(van_pdf, van_png)
        ae = compare_pngs(crys_png, van_png)
        print(f"Orient {orient}: AE={ae}")
        print(f"  crys pdfimages: {pdfimages_info(crys_pdf)}")
        print(f"  van  pdfimages: {pdfimages_info(van_pdf)}")
        if ae != 0:
            all_zero = False

    if all_zero:
        print("\nOK: todas as orientações têm AE=0.")
        return 0
    else:
        print("\nAVISO: existem orientações com AE > 0.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
