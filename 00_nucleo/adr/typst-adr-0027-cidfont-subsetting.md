# ADR-0027 — CIDFont com subsetting via ttf-parser

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-29

## Contexto

DEBT-5 exige suporte Unicode completo no PDF. O exportador actual usa Helvetica
Type1 com WinAnsiEncoding, que não suporta codepoints > U+00FF. Caracteres como
`•` (U+2022), acentos não-Latin-1, e qualquer texto não-europeu aparecem como `?`.

A solução padrão para PDF com Unicode arbitrário é CIDFont + ToUnicode CMap:

```
Type0 (composite font)
  └── DescendantFonts → CIDFont (Type2 = TrueType)
        ├── CIDSystemInfo (Adobe-Identity-0)
        ├── DW + W (widths por glyph)
        └── FontDescriptor → stream com dados TrueType
  └── Encoding → Identity-H
  └── ToUnicode → stream CMap (glyph ID → Unicode)
```

## Opções consideradas

**Opção A — CIDFont sem subsetting**: embeber a fonte TrueType completa no PDF.

**Opção B — CIDFont com subsetting**: embeber apenas os glyphs usados no documento,
com remapeamento de glyph IDs no ToUnicode CMap.

**Opção C — Manter Helvetica com placeholder `?`**: não implementar Unicode.

## Decisão

**Opção A** (CIDFont sem subsetting) no Passo 24.

## Razão

Subsetting requer remapear glyph IDs (os IDs no subconjunto não coincidem com
os IDs na fonte original). Este remapeamento invalida o ToUnicode CMap a menos
que seja gerado em coordenação com o subsetting. A implementação correcta
(subsetting + remapping) é complexidade adicional que não é necessária nesta fase.

A fonte completa garante que:
- O ToUnicode CMap é sempre correcto (glyph ID = índice real na fonte)
- Os testes passam sem fixtures de subsetting
- O comportamento é previsível

**Custo**: PDFs podem ser maiores (tipicamente +100KB–1MB por fonte embebida).
Aceitável para esta fase de desenvolvimento.

## Consequências

- PDFs com a fonte completa embebida — tamanho maior
- Sem subsetting até nova ADR quando volume de fontes exigir optimização
- Fallback para Helvetica Type1 mantido quando sem fonte TrueType disponível
  (caso dos testes unitários sem fixture de fonte)

## Próxima ADR

ADR-0028: subsetting com remapeamento de glyph IDs quando necessário.
