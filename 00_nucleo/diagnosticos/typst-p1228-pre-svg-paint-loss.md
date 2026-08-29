# P1228 — perda pré-SVG fechada na paginação exact-fit

A matriz independente A–H confirmou no vanilla ratificado que `transparentize`,
alpha e stops coincidentes são aceitos. A sonda diferencial refutou a hipótese
de paint: a fixture desaparecia igualmente com `fill: red`, enquanto a mesma
forma com altura menor aparecia. O último valor correto era `ShapeElem` e o
primeiro comportamento incorreto estava no teste de overflow de
`01_core/src/compiler/layout/shape.rs`: ele usava a baseline tipográfica
(`cursor_y + height`) em vez da caixa real da forma (`shape_base + height`).

O L0 proprietário foi atualizado primeiro. O teste focal falhou antes da
implementação: o documento exact-fit criava páginas vazias e não tinha Shape na
primeira página. Depois da correção, a forma sólida e as variantes gradient
ocupam `(5,5,130,40)`. A variante sRGB emite servidor resolvido com três stops
ordenados nos offsets `0,0,1` e alpha separado. A variante default volta a ter
geometria e fallback marcado, mas sua interpolação Oklab continua `Unknown`.

Proveniência inicial: HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada (paths em `git diff HEAD --stat`), instante
`2026-08-26T20:27:50-03:00`; passo SHA-256
`4b8b95e7e78e326989b72fb935763034e4787176db834bc66a7ded0ee3f04570`;
vanilla `/usr/local/bin/typst` SHA-256 começa por `7b4f40`; contrato independente
SHA-256 `009fb653d9258208a68fe4ca9c92f9964b6df17531a459c097197db2db80a5a8`;
oráculos `cfb00cf041cfa0d9365d244a464162227b79d42d6a7f001dcfffed5ef6cac06b`;
ataques `41b852fc6fc74212771c15f74df10921a2fa64685a4c0bef7eebf62b4bb396d6`.
O verificador independente adjudicou `ACCEPTED_WITH_CONTRACTUAL_UNKNOWN`,
matou 12/12 mutantes e selou resultados sob SHA-256
`433551e3d234495fbedf053ea8629fdfbdcf5f6a02878c896e7b21d1dc1d7338`.

```text
P1228 PRE-SVG PAINT LOSS CLOSED — COINCIDENT STOPS AND ALPHA PRESERVED
```

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
