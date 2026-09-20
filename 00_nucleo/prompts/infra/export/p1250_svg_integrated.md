# Prompt L0 — integração funcional SVG P1250
Hash do Código: 561153f6

**Camada:** L3 — teste de integração
**Ficheiro alvo:** `03_infra/tests/p1250_svg_integrated.rs`

## Contrato

O teste constrói um corpus pequeno que combina tiling, clips, destinos,
imagens, glifos, gradientes e strokes. Deve verificar:

- referências locais sem IDs duplicados nem alvos ausentes;
- geometria, transformações e clips observáveis;
- alpha, payloads visíveis e fallback de destino desconhecido;
- coexistência de glyph, paint, pattern, clip e destino sem captura de IDs.

O teste não valida catálogos de mutantes, percentuais, confirmações, registros ou bytes
congelados de um benchmark.
