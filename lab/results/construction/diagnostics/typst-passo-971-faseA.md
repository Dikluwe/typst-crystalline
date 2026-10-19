# Passo 971 — Fase A: limites de integral e a inclinação do símbolo (PARADO no gate, ADR-0127)

**Data:** 2026-08-05 · **Estado da árvore:** HEAD = `4cf0ebdec` (P970).
Nenhum código de produção alterado. L0 editado e resselado (ver abaixo).

## Achado confirmado e mecanismo real

Reproduzido no mínimo `$ integral_a^b f(x) dif x $` (temp/p971/min.typ,
`pdftotext -bbox`): a partir da aresta esquerda do ∫, o vanilla coloca o
subscrito "a" a **6.04pt** e o sobrescrito "b" a **10.99pt**; o cristalino
coloca ambos a **10.99pt**. Bate com a auditoria (6.1/11.0 vs 11.0/11.0).
Verticais já iguais nos dois lados — só o horizontal do subscrito diverge.

**Mecanismo** (não é o caminho de limites empilhados nem math kern):
`compute_post_script_widths` do vanilla
(`lab/typst-original/crates/typst-layout/src/math/scripts.rs:220-228`)
subtrai **`base.italics_correction()`** do kern do subscrito pós-fixado ("a
bounding box da base já conta com a italic correction"); o sobrescrito não
leva termo. O cristalino (`attach.rs:259-275`) aplica `kern_sub` sem o
termo — daí a mesma coluna para os dois scripts.

**Valores da fonte** (fontTools, NewCMMath-Book, upem 1000):
- `integral.v1` (variante de display): IC = **450du** = 4.95pt a 11pt —
  exactamente o delta medido (10.99 − 6.04 = 4.95).
- `integral` (base, inline): IC = 180du = 1.98pt — a IC tem de vir do
  **glifo final** (variante esticada em display), não do char.
- MathKernInfo não tem entradas para nenhum glifo `integral*` — kerns são
  0 neste caso; todo o efeito é o termo de IC. (O P892 tinha refutado
  itálico como causa de OUTRO problema; aqui é o mecanismo correcto —
  confirmado por medição directa, não por analogia.)
- **Proxy refutado**: `ink_xMax − advance` = −56du nos dois glifos — nem o
  sinal bate com a IC declarada. Não há caminho via `glyph_ink_bounds`;
  é preciso ler a tabela `MathItalicsCorrectionInfo`.

## Porque parou (ADR-0127 ponto 1 — contrato público)

A implementação requer método novo no trait `FontMetrics`
(`italics_correction(glyph_id, size, style) -> Pt`, por glyph id — como
`glyph_ink_bounds` de P952 — porque a IC é da variante esticada, não do
char) + extracção do glyph id real da `base_box` em `attach.rs`
(`FrameItem::Glyph` já o carrega, P906). Método novo em trait público =
paragem obrigatória.

A spec completa (fórmula, valores, desenho do acessor, guardas) está
registada em `00_nucleo/prompts/engine/math/layout/attach.md` §P971 —
L0 editado primeiro, `attach.rs` resselado (→ `e38f3e58`, mudança só de
hash, zero código), `crystalline-lint .` limpo (0 violations; só o V7
órfão pré-existente).

**À confirmação do dono**, a Fase B é: acessor na trait + implementação em
`03_infra/font_metrics.rs` (leitura de `MathItalicsCorrectionInfo` por
glyph id) + termo `−IC` no subscrito em `attach.rs` + testes (sub do ∫
recua 4.95pt em display; sup inalterado; base sem IC bit-a-bit igual;
verticais P959/P914 intactas) + revalidação secções 4/25 + benchmark.
