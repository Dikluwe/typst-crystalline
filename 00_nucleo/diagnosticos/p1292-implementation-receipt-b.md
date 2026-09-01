# P1292 — recibo final de implementação do lote B (`math.underline`)

Data final dos checks próprios: 2026-09-01T01:31:46-03:00
Estado: working tree não commitado sobre
`0eb39f8ecb48930515f2cadb6a378450855b5a72`.

## Papel e fronteira

Este é o recibo final do implementador do lote B. Não é ataque, autoria do
oráculo nem veredito independente. O lote C não foi iniciado.

O oráculo protegido `04_wiring/tests/p1292_contract.rs` não foi lido,
executado nem editado por este papel. Seu resultado final é registrado abaixo
somente como evidência coordenada recebida pelo manifesto. Nenhum
`--fix-hashes` foi rodado.

Este recibo sucede o recibo v3/contrato v4, SHA-256
`f6e41ab8ddaead4e0deb0b88653a29cac3f3c47c31708d813f877e1f0b45911f`.

## Entradas finais v5

- manifesto:
  `0d083b86d7c6ec1f60f4cb4a7494c4baef9f93667ee11f569e843c8b94f8c0bf`;
- contrato canónico v5:
  `5f5b4ab885529a213497d274705c66e0d62e3195e2bcfdf02fb98209a5edb7fc`;
- selo v5:
  `42029c04946e45a36c39825720e71f32fde192aa1e93e8f2a408470c56c8fd7d`;
- oráculo final declarado, não aberto:
  `73fad840f9ac04e0cda0404dc7412c9f5d04a6acde1ec1fc789c605ec530f429`;
- L0 final `compiler/layout/equation.md`, SHA-256 bruto:
  `8a557ebeb62cef433b30d7f1db66acb7ed27526e72770ceb2bd0a12514117387`;
- hash canónico do mesmo L0 para o header `@prompt-hash`:
  `04a90b1b`.

O manifesto registra a reprodução do coordenador como `11 tests: 7 GREEN,
4 RED; A+B fully GREEN; C two semantic RED because math.vec is absent; D two
semantic RED because place.flush is absent; Unknown=0`. Este papel não
reexecutou esse comando protegido.

## Implementação B final

- `MathUnderlineElem` é entidade matemática própria, distinta da decoração
  textual, com transporte por `Content::MathUnderline`, travessia, repr,
  registro público e dispatcher estático para a free function dona.
- Bare `underline(...)` em modo math resolve por `math.underline`; o binding
  textual global fora de math permanece inalterado.
- A geometria usa as constantes MATH `underbar_vertical_gap`,
  `underbar_rule_thickness` e `underbar_extra_descender`.
- O frame conserva `body.width`; somente a regra usa
  `max(0, body.width - italic_correction)`, sem redimensionar por overhang.
- O ramo inline de `equation.rs` normaliza o extent por leading efetivo,
  fator upstream `0.7` e text edges tipados antes da única chamada a
  `note_inline_extent`. O ramo block conserva o extent cru.
- Spacing herda classes do body; introspecção trata o elemento como terminal
  math e os query helpers devolvem os valores selados `false`/`0`.

O refinamento v5 não exigiu mudança de lógica. A única alteração após o
recibo v4 foi a linhagem de `equation.rs`: o header passou a
`@prompt-hash 04a90b1b`, com data `2026-09-01`.

## Evidência bilateral própria preservada

As sete fixtures próprias em `/tmp`, compiladas antes do fecho de linhagem
por candidato e vanilla, produziram:

| Caso | Candidato | Vanilla | Classificação v5 |
|---|---:|---:|---|
| plain `$x$` | `6.292 × 7.513` | `6.292 × 7.513` | GREEN |
| text `$underline(x)$` | `6.292 × 7.513` | `6.292 × 7.513` | GREEN |
| display `$ underline(x) $` | `6.292 × 7.359` | `6.292 × 7.359` | GREEN |
| script `$x_(underline(y))$` | `10.8196 × 8.459` | `11.4356 × 8.459` | altura/regra e delta base→underline GREEN |
| cramped `$1 / underline(y)$` | `5.70075 × 9.537` | `6.7276 × 9.537` | altura/regra e delta base→underline GREEN |
| mixed `A $underline(x)$ B` | `25.905 × 7.513` | `25.905 × 7.513` | GREEN |
| italic `$underline(f)$` | `6.38 × 7.513`; regra `5.39` | `6.38 × 7.513`; regra `5.39` | GREEN |

O contrato v5 mede horizontalmente o delta base→underline dentro de cada
renderer. Esse delta é zero em Text/Display/Script/Cramped. As diferenças
absolutas de `0.616pt` em attach e `1.02685pt` em fraction já existem nos
controles sem underline e são scope-out explícito, não RED de B nem
autorização para alterar cursor, attach, fraction, auto-page ou wrapper.

## Ficheiros e hashes finais

| Ficheiro | SHA-256 final |
|---|---|
| `01_core/src/compiler/layout/equation.rs` | `5a33b6dc9e7981fbeaf3fd1645460cc58f2f85ed89644d67e589fae301c41af0` |
| `01_core/src/compiler/math/layout/underline.rs` | `203370f9bd940a3f61cf79fd3f12149ae75ee26d383e7eafb836bc73ac3700d9` |
| `01_core/src/entities/elements/math_underline.rs` | `6a08c552d47f7a90263b6f3e447f15ee9e42f17885201a866db910edddfa4948` |

Antes do fecho v5, `equation.rs` tinha SHA-256
`4c9cf3a286e2ee684a620954298bc55075cf6b4a384b16872329f5855c9e0cfd`;
a diferença final é somente o header de linhagem. O bloco independente
`#[cfg(test)]` de `math_underline.rs` permaneceu integralmente preservado.

## Checks finais próprios

1. `cargo build --bin typst` — GREEN; warnings preexistentes.
2. `cargo test -q -p typst-core p1292_math_underline_owner_preserva_body_estrutural`
   — GREEN: `1 passed`, `0 failed`, `5350 filtered out`.
3. `rustfmt --edition 2021 --check 01_core/src/compiler/layout/equation.rs
   01_core/src/compiler/math/layout/underline.rs` — GREEN.
4. `crystalline-lint --checks v5 .` — código `0`; nenhum V5 nos owners B.
5. `git diff --check` — GREEN.

O V5 ainda reporta quatro avisos fora do escopo B, preservados para os owners
de integração P1292:

- `01_core/src/compiler/eval/mod.rs`: L0 `6058051a`, código `82030adc`;
- `01_core/src/compiler/math/layout/cancel.rs`: L0 `95a40caa`, código `84b2c852`;
- `01_core/src/compiler/stdlib/layout.rs`: L0 `7b184988`, código `d185de1d`;
- `01_core/src/entities/elements/math_cancel.rs`: L0 `112fd354`, código `cc164630`.

## Parada serial

Lote B fechado como GREEN pelo contrato v5 reproduzido pelo coordenador,
Unknown `0`, com checks próprios de build/focal/linhagem/formatação/diff
GREEN. Os quatro RED restantes pertencem semanticamente aos lotes C-D. Este
implementador para antes de C.
