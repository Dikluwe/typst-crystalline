# P1292 — B wrapper-width amendment-4

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest:** `537020ac9ab9c7c3864a0dd838a413ff1367e1a1b0ae03da09b6a7b09874588f`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Proveniência:** working tree não commitada; probes em
2026-09-01T01:16:05-03:00; L0 fechado em 2026-09-01T01:16:47-03:00;
`git diff HEAD --stat`: 35 arquivos, 1.025 inserções, 309 remoções. O delta
inclui P1292 de outros papéis. Este papel não leu/editou oráculo, código,
testes, ataques ou veredito.

**Regime:** protocolo completo segregado sem atestação de isolamento técnico.

## Bridge v4 e escopo

- contrato canônico v4:
  `ac95d6003126b4b3ebc8b0128e5cbca3ac95c05d88f2f0f6ceab930d6fa598b2`;
- seal v4:
  `876a870be1e98474f959c69db13eff926391fafc3fb19f17a3e4425d6200a86a`;
- receipt v4:
  `a71c4440d9b9c552db617c3e5081efc68db035de131ddb13d6dd42ce9f33320e`;
- lots v4:
  `14ae87c45cdc3c1a01813702039e849deecfbcdd2cd649d27b0d36da27f141a1`;
- comparison v4:
  `4ae63b34df2388100658ceec3dcfa3e3c2113e412f771b2496a6101902a1fb46`.

A normalização v4 está preservada: alturas e linhas são bilaterais. O
amendment-4 só corrige a atribuição causal da largura whole-wrapper em B. Não
muda surface, erros, repr, identidade, morfologia, valores ou resultados A-D.

## Medição bilateral com controles pareados

Vanilla ratificado `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
candidato `target/debug/typst` SHA-256
`bc9ec3e3bb9510e1fedbdb2ae71ede8c6ec566870cab54423e2612b82ab3c376`.
Todos os probes usaram `page(width:auto,height:auto,margin:0pt)` e SVG.

| Par base / underline | Vanilla width; height | Candidato width; height | Delta width B |
|---|---:|---:|---:|
| Text `$x$` / `$underline(x)$` | `6.292→6.292`; `7.513→7.513` | `6.292→6.292`; `7.513→7.513` | `0` bilateral |
| Display `$ x $` / `$ underline(x) $` | `6.292→6.292`; `4.983→7.359` | `6.292→6.292`; `4.983→7.359` | `0` bilateral |
| Script `$x_(y)$` / `$x_(underline(y))$` | `11.4356→11.4356`; `7.513→8.459` | `10.8196→10.8196`; `7.513→8.459` | `0` bilateral |
| Cramped `$1/y$` / `$1/underline(y)$` | `6.7276→6.7276`; `7.8738→9.537` | `5.70075→5.70075`; `7.8738→9.537` | `0` bilateral |

Os controles sem underline têm SHA-256
`eb988ee4c9a9a41fcbd02c39c9ee2757214554169d40d861e92a8500b2f85349`
(Script) e
`3ebafd572494d870b6dfd4dc515a5267b658301ef4ea686f83eacc6e3bbc6303`
(Cramped), iguais aos pins completos do manifest. Os probes com
underline têm SHA-256
`8e2d43a12806ea6ba07434d5410b778dee30ed8a135dc6982668286edd6f204b`
e `a536a428c4b92ff3b876e60364c8eec988b1b3b0aff3c9cb07fef49e70c0cd26`.

O gap whole-root é o mesmo antes e depois de aplicar underline: Script
`11.4356−10.8196=0.616pt`; Cramped
`6.7276−5.70075=1.02685pt`. Logo ele é resíduo de wrapper attach/fraction,
preexistente e não discriminatório para MathUnderline.

## Frame e regra isolados

- `x`: root/frame `6.292pt` antes/depois e regra `6.292pt` bilateral;
- `f`: root/frame `6.38pt` antes/depois e regra `5.39pt` bilateral, diferença
  `0.99pt` pelo termo `−italics_correction`;
- Script: regra do `y` `4.4583pt` bilateral, apesar do root wrapper distinto;
- Cramped: regra do `y` `4.4583pt` e fraction bar `4.5276pt` bilateral.

Hashes de fixtures isoladas: `italic-base.typ`
`8212c6e90ae027f3b37185c0a7cfbb6635d7ecec52ed8f1280579065c749f569`,
`italic-underline.typ`
`e90915819f91ed74e148ad653785fdb763ce3d6d2cc81638ad6d51bd386e7a70`,
`text-base.typ`
`ae3b2291ed1b4ee6a7d1f999177aa31e3307ed8ef0cb02d93a4aed0242f6da0c`
e `text-underline.typ`
`4d9b92c627f3bd2393dc765ca971a1707736e7d6327a3f026cfd118b7cd10de7`.

## Decisão e ownership

O oráculo B horizontal compara, dentro de cada renderer, que trocar `body` por
`underline(body)` preserva o width do body/wrapper (`delta=0`) e que a regra
mede `max(0, body.width−italics_correction)`. Mantém verticalmente os quatro
estilos exatos. Isso rejeita underline que alarga/encolhe a caixa, ignora
italic correction ou move a linha, sem confundir o owner com o wrapper.

O valor absoluto whole-wrapper Script/Cramped é scope-out preexistente. Não é
`Preserved`, não vira `Unknown` de B e não autoriza reparo: cursor, attach,
fraction, auto-page e wrapper-width ficam fora de P1292. Passo futuro deve
medir seus próprios owners.

Somente `compiler/layout/equation.md` precisava de emenda porque sua aceitação
v4 ainda listava widths whole-wrapper como se fossem discriminantes B. Novo
SHA-256 L0:
`8a557ebeb62cef433b30d7f1db66acb7ed27526e72770ceb2bd0a12514117387`.
`compiler/math/layout/underline.md` já fixava corretamente frame versus regra
e permaneceu byte a byte em
`659aabb6046579629095c4739f1f074c80a9d4b2bde0028bd05b5b2935f0ac52`.
Não surgiu owner novo; o set continua com 22.

Refutadores: delta de width não zero, regra diferente do body-minus-italic,
frame isolado alterado, ou regressão vertical/baseline. A persistência do
mesmo gap absoluto no par base/underline não refuta B; prova o scope-out.

Próximo papel é o autor independente do oráculo, para trocar somente a
comparação whole-root inválida pelos pares selados, sem adaptar a geometria do
underline nem tocar wrappers. **PARAGEM.**
