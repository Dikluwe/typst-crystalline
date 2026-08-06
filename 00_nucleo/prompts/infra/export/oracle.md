# export/oracle — oráculo de paridade de operador (P980)
Hash do Código: 4c630398

**Data:** 2026-08-05 · **Camada:** L3 · **Passo:** 980 (gate confirmado
pelo dono em 2026-08-05)

## Propósito

Caminho de diagnóstico **separado** do exportador de produção: recebe o
content stream já construído pelo modo verbose normal e aplica
transformações de paridade de operador que **não** pertencem à saída
principal (não mudam posições nem têm benefício visível fora de
comparação). Activo só com a flag CLI `--oracle-pdf`.

## Regras do módulo

1. `03_infra/src/export/oracle.rs`; funções puras sobre a string do
   content stream; zero impacto no caminho normal (a saída sem a flag é
   bit-a-bit a mesma — guardado por teste).
2. Activação: `RunIntent.oracle_pdf` (L2) → `compile_to_pdf_bytes_oracle`
   (L3, aditiva) → `export_pdf_oracle` (mesma dispatch de fontes da
   emissão normal) → `PdfBuilder::with_oracle(true)` → aplica
   `collapse_trivial_tj` ao content stream de cada página (antes da
   compressão).

## Transformação 1 — `collapse_trivial_tj`

Um array `[ … ] TJ` cujos ajustes são **todos inteiros zero** (entries
`<XXXX> 0` e ajustes de fronteira `0`) é semanticamente um `Tj` puro:
reescreve-se como `<XXXXYYYY…> Tj` (hex concatenado). Arrays com qualquer
ajuste ≠ 0 ficam intocados. Números são **parseados** como inteiros (um
`-0` conta como zero). Paridade: o vanilla usa `Tj` quando não há
ajustes (medido: 92.5% dos blocos de texto do documento canónico).
