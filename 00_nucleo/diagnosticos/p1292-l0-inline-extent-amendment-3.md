# P1292 — L0 inline-extent amendment-3

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:** `eeae48cdd86b4ea1025f4023936745233e1d6ecc52076234489b84414eb95b32`

**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`

**Estado:** working tree não commitada; medição 2026-09-01T00:57:33-03:00;
`git diff HEAD --stat` ao fechar a autoria: 35 arquivos, 943 inserções e 308
remoções. O delta inclui trabalho P1292 de outros papéis; esta autoria escreveu
somente os dois L0s autorizados, este diagnóstico e o seal/receipt.

**Regime:** protocolo completo segregado, sem alegação de isolamento técnico.
O código candidato foi inspecionado porque o manifest exige auditar a mecânica
refutada; o oráculo protegido `04_wiring/tests/p1292_contract.rs` não foi lido.

## Bridge e refutação

- contrato canônico v3:
  `f72392739e4f9ebf73dcaf769710e27f767c945a5970eb2f58a1c3b3822d612e`;
- seal v3:
  `0cfcd2e18719efd280bc73b51315f0d8da69e9bed1fc18cbb7a0fb7ff2dc9fb3`;
- receipt v3:
  `ffde65ffd16eeaa2d2d4b1b24f5aefd62b8a4ac394bc41c833e2c6f093d694ba`;
- lots v3:
  `21a7135267f81e8727681f4eefdf3fcc28cda3485c0e6f75ffe61304d79b2567`;
- comparison v3:
  `dc7f7885cd426aca64d5423a55061e2dff0452c21e19edab9180964e9ade5925`;
- implementação literal refutada `equation.rs`:
  `5683f8469819678652041847f464cf1151d525cb68a0313b1b87722b7cf4e66a`.

O v3 ordenava `note_inline_extent(extent.ascent, extent.descent)`. A chamada
foi implementada literalmente, sem rescan/relayout, e mudou Text de
`6.292×7.238pt` para `6.292×9.735pt`, contra vanilla `6.292×7.513pt`.
Portanto o selo v3 fica superado apenas no mecanismo inline; os vetores,
valores e resultados públicos A-D permanecem congelados.

## Probes bilaterais

Referência: vanilla ratificado `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Candidato: `target/debug/typst` SHA-256
`f047eee722fcad1c7c3afe9fe4d7b144a92b6c263eb4442bec5797b4c30b52f8`.
Todos os `.typ` estavam em `/tmp/p1292-amendment3`, página auto e margem zero.

| Probe | Vanilla | Candidato raw-v3 |
|---|---:|---:|
| plain `$x$` | `6.292×7.513` | `6.292×7.359` |
| Text `$underline(x)$` | `6.292×7.513` | `6.292×9.735` |
| mista `A $underline(x)$ B` | `25.905×7.513` | `25.905×9.735` |
| Display `$ underline(x) $` | `6.292×7.359` | `6.292×7.359` |
| Script `$x_(underline(y))$` | `11.4356×8.459` | `10.8196×13.189` |
| Cramped `$1 / underline(y)$` | `6.7276×9.537` | `5.70075×16.4758` |

Na mista, todos os glyphs da linha compartilham baseline (`7.513` vanilla;
`7.238` candidato): o raw descent infla o fundo, não desalinha os irmãos.
Em duas linhas, trocar `$x$` por `$underline(x)$` desloca a segunda baseline
candidata de `21.747` para `24.123`; vanilla mantém `21.901`. Em Cramped, o
raw ascent também desloca a baseline candidata para `9.4468`, versus `7.513`.

Para largura, `$underline(f)$` mediu caixa/root `6.38pt` e regra `5.39pt`
bilateralmente: o termo `−italics_correction` mede `0.99pt` e só encurta a
regra. `$underline(x)$` mede caixa e regra `6.292pt`. A fonte pinada
`03_infra/fixtures/fonts/NewCMMath-Book.otf`, SHA-256
`60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`,
tem cap-height 683/1000; a 11pt são exatamente `7.513pt`.

Fixtures centrais e hashes: `plain-x.typ`
`ae3b2291ed1b4ee6a7d1f999177aa31e3307ed8ef0cb02d93a4aed0242f6da0c`,
`underline-x.typ`
`4d9b92c627f3bd2393dc765ca971a1707736e7d6327a3f026cfd118b7cd10de7`,
`mixed-underline.typ`
`da63023730d61b3f86a26c65b5f1a81dd26f174107f4e9aaf83b7e9b8ef7ae3`,
`script.typ`
`8e2d43a12806ea6ba07434d5410b778dee30ed8a135dc6982668286edd6f204b`,
`cramped.typ`
`a536a428c4b92ff3b876e60364c8eec988b1b3b0aff3c9cb07fef49e70c0cd26`
e `italic-width.typ`
`e90915819f91ed74e148ad653785fdb763ce3d6d2cc81638ad6d51bd386e7a70`.

## Fonte e decisão medida

`lab/typst-original/crates/typst-layout/src/math/mod.rs:49-101`, SHA-256
`88b33ef6eb23deae0e6956c7c59bdbc57604ae1d39a0c6171233f426d87b146b`,
normaliza cada frame inline antes do parágrafo: `slack=leading×0.7`,
`ascent=max(top_edge, raw_ascent-slack)` e
`descent=max(bottom_edge, raw_descent-slack)`. A fonte math local fixa
`bounds` internamente em `typst-library/src/math/ir/resolve.rs:24-35`, SHA-256
`115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112`;
isso não substitui os edges externos da equação.

No cristalino, `EquationExtent` já é a caixa lógica `MathBox` produzida no
mesmo run. `equation` deve normalizar esse par com leading/edges efetivos e só
então chamar uma vez `note_inline_extent`; `cursor.rs` continua intacto como
owner da agregação max/shift/flush. Com leading default `0.65em`, slack é
`5.005pt`: Text engole o descent `2.497`; Script preserva `0.946`; Cramped
produz ascent `7.513` + descent `2.024` = `9.537`. Nenhum valor de fixture é
assado.

`typst-layout/src/math/line.rs:20-67`, SHA-256
`d71623f47d8d3d9d9b5435820e5ca05399563642a4a48627f76ce0c8ed997b34`,
mantém `frame.width=body.width` e aplica `−italics_correction` somente à
regra. A decisão equivalente foi explicitada no L0 underline: correção
negativa real pode fazer a regra extrapolar sem alargar a caixa.

## Ownership, inferência e paragem

- `compiler/layout/equation.md` → `01_core/src/compiler/layout/equation.rs`;
  L0 SHA-256
  `c73dc8b19afb451272e3e758378793bf81ca6a9f5e2ce77b5b07e3ef817279c7`.
- `compiler/math/layout/underline.md` →
  `01_core/src/compiler/math/layout/underline.rs`; L0 SHA-256
  `659aabb6046579629095c4739f1f074c80a9d4b2bde0028bd05b5b2935f0ac52`.

Não foi descoberta dependência normativa nova; os 22 ownerships permanecem.
Inferimos que a normalização medida é suficiente porque seus termos reproduzem
exatamente Text/Script/Cramped e mantém Display fora do ramo. Refutam: qualquer
valor não atingir vanilla, baseline mista divergir, leading lexical não mudar
slack, largura afetar altura ou ser necessário reinspecionar items. Refutação
reabre o owner; não ajusta fator à fixture.

`crystalline-lint . --checks v15,v26 --fail-on warning` terminou com
`✓ No violations found`. Próxima ação causal pertence ao autor independente
dos oráculos para reconhecer o v4 e então ao implementador. **PARAGEM.**
