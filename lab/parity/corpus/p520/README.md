# P520 — Corpus de paridade de produção (kerning + ligatures)

Este corpus contém documentos mínimos usados para validar a qualidade do
export PDF cristalino no que diz respeito a tipografia avançada.

## Ficheiros

- `test-ligatures.typ` — documento de linha única com as ligatures
  `fi`, `fl` e `ffi`. O PDF resultante deve mostrar os glifos de ligature
  (forma ligada) e não quadrados `.notdef`.

- `test-kerning.typ` — documento de linha única com "AVATAR". O operador
  `TJ` do stream de conteúdo deve conter ajustes de kerning (valores
  negativos ou positivos) para aproximar/afastar pares específicos.

## Validação esperada

```bash
cd typst-crystalline
cargo run --bin typst -- lab/parity/corpus/p520/test-ligatures.typ /tmp/p520-ligatures.pdf
cargo run --bin typst -- lab/parity/corpus/p520/test-kerning.typ /tmp/p520-kerning.pdf
```

1. Visualmente, `p520-ligatures.pdf` deve exibir `fi`, `fl`, `ffi` como
   ligatures.
2. `pdftotext /tmp/p520-ligatures.pdf -` deve extrair pelo menos três
   caracteres `'f'` (um por ligature; o ToUnicode é intencionalmente
   parcial — ver DEBT.md).
3. O stream de conteúdo de `p520-ligatures.pdf` não deve conter
   `<0000>` para os operadores TJ das ligatures.
4. O stream de conteúdo de `p520-kerning.pdf` deve conter ajustes no
   operador TJ (ex.: `<0001> -50 <0002> 0`).
