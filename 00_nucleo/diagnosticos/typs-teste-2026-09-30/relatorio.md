# Sonda dos dez `.typ` locais — Tekt × vanilla pinado

Medição: 2026-09-30T12:45:36.827811+00:00 UTC, renderização 120 dpi. HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitada; a lista exata de arquivos modificados antes de arquivar esta sonda está em `working-tree-status.txt` e `working-tree-stat.txt`. `results.json` fixa SHA-256 dos dois binários, das dez entradas e dos PDFs, comandos implícitos `BIN compile INPUT OUTPUT`, stderr, páginas, dimensões, extração `pdftotext -layout` e diferenças raster. O vanilla é `lab/typst-original/target/release/typst`; o Tekt foi recompilado por `cargo build -q` nesta rodada.

| Arquivo | exit T/V | páginas T/V | extração textual | raster 120 dpi | Δ altura T−V |
|---|---:|---:|---|---|---:|
| `sec_04.typ` | 0/0 | 1/1 | igual | 0.055% pixels Δ>20 | +0.000 pt |
| `sec_07.typ` | 0/0 | 1/1 | igual | 0.000% pixels Δ>20 | +0.000 pt |
| `sec_09.typ` | 0/0 | 1/1 | difere | idêntico | +0.000 pt |
| `sec_10.typ` | 0/0 | 1/1 | difere | 0.038% pixels Δ>20 | +0.000 pt |
| `sec_12.typ` | 0/0 | 1/1 | difere | 0.204% pixels Δ>20 | +0.000 pt |
| `sec_16.typ` | 0/0 | 1/1 | igual | idêntico | +0.000 pt |
| `sec_17.typ` | 0/0 | 1/1 | igual | idêntico | +0.000 pt |
| `sec_18.typ` | 0/0 | 1/1 | igual | idêntico | +0.000 pt |
| `sec_22.typ` | 0/0 | 1/1 | igual | 1.168% pixels Δ>20 | -5.687 pt |
| `sec_27.typ` | 0/0 | 1/1 | difere | idêntico | -0.001 pt |

Os dez documentos compilaram em ambos os binários, uma página cada. Cinco rasters são pixel a pixel idênticos (`sec_09`, `sec_16`, `sec_17`, `sec_18`, `sec_27`). `sec_07` tem só diferença subvisual de um pixel. `sec_22` é a divergência visual prioritária: comprimentos/posições dos delimitadores diferem e o `page(height: auto)` fica 5,687 pt mais curto no Tekt. `sec_04`, `sec_10` e `sec_12` têm diferenças localizadas menores; a inspeção ampliada não encontrou conteúdo ausente ou clipping evidente nesses três.

A extração difere em `sec_09`, `sec_10`, `sec_12` e `sec_27`. Em `sec_09` e `sec_27`, o raster é idêntico: são diferenças do texto extraível/ordenação, não do desenho. Em `sec_10`, a extração separa alguns acentos e inclui caracteres das peças de bracket; `sec_12` difere na ordem do acento final. Esses resultados não provam que toda diferença da extração seja defeito do Tekt: o vanilla também descarta partes visuais de `cases` na extração. Paridade deve ser classificada por semântica, morfologia e sintaxe (ADR-0107), com sonda causal antes da correção.

As imagens `contact-1.jpg` e `contact-2.jpg` apresentam as dez páginas lado a lado, Tekt à esquerda, vanilla à direita. Os vinte PDFs da rodada estão neste diretório para inspeção e reprodução do texto extraído. Nenhum código ou L0 foi alterado por esta medição.
