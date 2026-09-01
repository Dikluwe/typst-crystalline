# P1292 — recibo de medição vanilla, lotes A–D

Papel: **MEDIDOR**, sequência 1. Este documento registra observações; não
redige L0, não autoriza implementação e não emite veredito.

## Estado e entradas

- instante final da medição: `2026-08-31T22:19:26-03:00`;
- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- branch: `Tekt`;
- estado anterior à escrita deste recibo:

  ```text
  ?? 00_nucleo/diagnosticos/p1292-baseline-status.txt
  ?? 00_nucleo/diagnosticos/p1292-manifest.json
  ```

- SHA-256 de `git status --short`:
  `404949c51b35f13286c0794b6c47d3b9ba80c5695f5141df3f38512b3b8b1920`;
- `git diff HEAD --stat`: vazio, SHA-256
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- manifesto: `00_nucleo/diagnosticos/p1292-manifest.json`, SHA-256
  `044e8eac99ed78edeecf0e557c56a33908c9e3666e8706aa8e8728a2c339e4c0`;
- status do baseline: `00_nucleo/diagnosticos/p1292-baseline-status.txt`,
  SHA-256 `fbddc7594893df1dfae68451893bce3136792b3cd2470bb981d6f57d1ccb0eb2`;
- vanilla ratificado: `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  revisão pinada `a51e02804`;
- cristalino medido: `target/release/typst`, SHA-256
  `06314ba817d3ad85bf015b326f78b1018742c6cb1fcb732f7a041c44db5c0aa9`.

## Baseline fresco da superfície default

Comando:

```text
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin target/release/typst --output /tmp/p1292-surface-baseline.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
```

Entradas:

| Artefato | SHA-256 |
|---|---|
| `lab/surface-inventory/run_probes.py` | `3bd082751fbd05c89f24a312353d81f74f2882902db5a203180e5ee5fb2c10cf` |
| `lab/surface-inventory/probes.json` | `1e7534a0cc8ebcfa1f5ed6652918711b738d7471d94a883a500becf297357b78` |
| `00_nucleo/diagnosticos/p1284-inventory-default.json` | `41a16a2856b335275656af64649f9bbd85465ff9ebf91c979e3398ae3650db4e` |

O comando terminou com código `0` e produziu:

```json
{"different_or_disabled": 16, "same": 95, "total": 111}
```

`/tmp/p1292-surface-baseline.json` tem SHA-256
`61df7e72f0fed09e7ec3b47eab1cf1b6f2a30a3dd2705f020b60a41473303dc6`.
Os 16 resultados não-MATCH classificam-se em `2 DISABLED`,
`10 EXTRA_BINDING` e `4 MISSING_MEMBER`. Os quatro `MISSING_MEMBER` são
exatamente:

| Caminho | Vanilla | Cristalino |
|---|---|---|
| `math.cancel` | `(function, "cancel")` | `module 'math' does not contain field "cancel"` |
| `math.underline` | `(function, "underline")` | `module 'math' does not contain field "underline"` |
| `math.vec` | `(function, "vec")` | `module 'math' does not contain field "vec"` |
| `place.flush` | `(function, "flush")` | `cannot access fields on type function` |

A condição de paragem por divergência do conjunto residual não ocorreu.

## Superfície pública vanilla

Todos os casos abaixo usaram:

```text
/usr/local/bin/typst eval '<expressão>' --format json
```

| Lote/caso | Expressão | Código | Observação pública |
|---|---|---:|---|
| A default | `repr(math.cancel([x]))` | 0 | `cancel(body: [x])` |
| A callback | `repr(math.cancel([x], angle: a => 0deg))` | 0 | `cancel(body: [x], angle: (..) => ..)` |
| A body ausente | `repr(math.cancel())` | 1 | `missing argument: body` |
| A named desconhecido | `repr(math.cancel([x], nope: 1))` | 1 | `unexpected argument: nope` |
| A length inteiro | `repr(math.cancel([x], length: 1))` | 1 | `expected relative length, found integer` |
| A angle inteiro | `repr(math.cancel([x], angle: 1))` | 1 | `expected angle, function, or auto, found integer` |
| B default | `repr(math.underline([x]))` | 0 | `underline(body: [x])` |
| B body ausente | `repr(math.underline())` | 1 | `missing argument: body` |
| B body inteiro | `repr(math.underline(1))` | 1 | `expected content, found integer` |
| B identidade | `repr(math.underline == underline)` | 0 | `false` |
| C zero filhos | `repr(math.vec())` | 0 | `vec(children: ())` |
| C dois filhos | `repr(math.vec([a], [b]))` | 0 | `vec(children: ([a], [b]))` |
| C named desconhecido | `repr(math.vec([a], nope: 1))` | 1 | `unexpected argument: nope` |
| C gap inteiro | `repr(math.vec([a], gap: 1))` | 1 | `expected relative length, found integer` |
| C align inteiro | `repr(math.vec([a], align: 1))` | 1 | `expected alignment, found integer` |
| C delim inteiro | `repr(math.vec([a], delim: 1))` | 1 | `expected array, none, symbol, or string, found integer` |
| D default | `repr(place.flush())` | 0 | `flush()` |
| D namespace por `with` | `repr(place.with(dx: 1pt).flush)` | 0 | `flush` |
| D chamada por `with` | `repr(place.with(dx: 1pt).flush())` | 0 | `flush()` |
| D posicional | `repr(place.flush([x]))` | 1 | `unexpected argument` |
| D named desconhecido | `repr(place.flush(nope: 1))` | 1 | `unexpected argument: nope` |

O caso explícito completo de A:

```text
repr(math.cancel([x], length: 80% + 1em, inverted: true, cross: true, angle: 45deg, stroke: red + 1pt, background: true))
```

devolveu, com código `0`:

```text
cancel(
  body: [x],
  length: 80% + 1em,
  inverted: true,
  cross: true,
  angle: 45deg,
  stroke: 1pt + rgb("#ff4136"),
  background: true,
)
```

O caso explícito de C:

```text
repr(math.vec([a], [b], delim: "[", align: left, gap: 1em))
```

devolveu, com código `0`:

```text
vec(
  delim: ("[", "]"),
  align: left,
  gap: 0% + 1em,
  children: ([a], [b]),
)
```

## C — `vec.gap` em região finita e infinita

A medição usou `measure` com `width: 100pt`. A região finita recebeu
`height: 100pt`; a região infinita recebeu `height: auto`. Cada valor foi
transportado por `metadata` e lido com:

```text
printf '<fonte abaixo>' | /usr/local/bin/typst query - 'metadata' --format json
```

Fonte principal:

```typst
#set page(width: 200pt, height: 200pt, margin: 0pt)
#context [#metadata(measure(math.equation(math.vec([a], [b], gap: 0%)), width: 100pt, height: 100pt)) <finite-zero>]
#context [#metadata(measure(math.equation(math.vec([a], [b], gap: 10%)), width: 100pt, height: 100pt)) <finite-ten>]
#context [#metadata(measure(math.equation(math.vec([a], [b], gap: 0%)), width: 100pt, height: auto)) <infinite-zero>]
#context [#metadata(measure(math.equation(math.vec([a], [b], gap: 10%)), width: 100pt, height: auto)) <infinite-ten>]
#context [#metadata(measure(math.equation(math.vec([a], [b], gap: 1em)), width: 100pt, height: auto)) <infinite-em>]
```

Resultado, código `0`:

| Caso | width | height |
|---|---:|---:|
| `finite-zero` | `15.79pt` | `7.7pt` |
| `finite-ten` | `23.53pt` | `22.88pt` |
| `infinite-zero` | `15.79pt` | `7.7pt` |
| `infinite-ten` | `15.79pt` | `7.7pt` |
| `infinite-em` | `23.53pt` | `22.88pt` |

Medição mista adicional:

| Caso | gap | width | height |
|---|---|---:|---:|
| `finite-mixed` | `10% + 1em`, height `100pt` | `23.53pt` | `29.96pt` |
| `infinite-mixed` | `10% + 1em`, height `auto` | `23.53pt` | `22.88pt` |

Observação estritamente medida: na região infinita criada por `height: auto`,
o componente percentual de `gap` produziu o mesmo tamanho que `0%`; o
componente absoluto `1em` permaneceu. Na região finita, o componente
percentual alterou o tamanho. Não se infere deste recibo uma implementação.

Proveniência da fonte vanilla:

| Fonte | Linhas medidas | SHA-256 |
|---|---:|---|
| `lab/typst-original/crates/typst-library/src/math/matrix.rs` | 33–67 | `fb6ed72f7fac4bb5489170059b9aa0f919beb0b1a23bf4e862d4cff52905120f` |
| `lab/typst-original/crates/typst-library/src/math/ir/resolve.rs` | 1002–1025 | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| `lab/typst-original/crates/typst-layout/src/math/table.rs` | 25–34 | `77e1f383e1b252af5331803c9e88ba64d079f1528183083896b9d9f7501e7e04` |
| `lab/typst-original/crates/typst-library/src/layout/measure.rs` | 70–85 | `4b0ab977b50853c04a8122793dd6f715b6a4fc215e37d7590aea96838ee3690d` |

As fontes mostram `gap: Rel<Length>`, transporte para `TableItem`, resolução
contra `ctx.region.size` e `height: auto` construindo região com
`Abs::inf()`; as dimensões acima são o observável público correspondente.

## D — efeito de `place.flush`

Controle discriminatório com página `100pt × 100pt`, um bloco inicial de
`30pt` e um bottom float de `80pt`:

```typst
#set page(width: 100pt, height: 100pt, margin: 0pt)
#block(height: 30pt)[before]
#place(bottom, float: true, block(width: 100pt, height: 80pt)[#context [#metadata(here().position()) <float>]])
#place.flush()
#block(height: 10pt)[after #context [#metadata(here().position()) <after>]]
```

Consulta `metadata`, código `0`:

```text
float: page 2, x 0pt, y 20pt
after: page 3, x 0pt, y 7.24pt
```

O mesmo documento sem `#place.flush()` devolveu:

```text
float: page 2, x 0pt, y 20pt
after: page 1, x 0pt, y 50.44pt
```

Um segundo float criado depois do marcador foi medido separadamente:

```text
float-before: page 2, x 0pt, y 20pt
after:        page 3, x 0pt, y 7.24pt
float-after:  page 3, x 0pt, y 90pt
```

Assim, neste caso limítrofe, o marcador força o float já pendente antes de o
conteúdo posterior prosseguir, enquanto o float posterior permanece posterior
ao marcador. Esta é uma observação do caso medido, não um veredito geral sobre
todas as combinações de float.

Proveniência da fonte vanilla:

| Fonte | Linhas medidas | SHA-256 |
|---|---:|---|
| `lab/typst-original/crates/typst-library/src/layout/place.rs` | 195–213 | `ec64092c3f09428aa5d5c2e43a35849cfa2e062947d8d662066a4cdfc3541579` |
| `lab/typst-original/crates/typst-layout/src/flow/distribute.rs` | 514–521 | `36f0732a6004d189024da6dfade8767f9f53ad0cacc346211d0b2d3cf3c9b09b` |

## Limites

- Nenhum L0, consumer produtivo, teste ou oráculo foi editado.
- Nenhuma decisão de contrato ou implementação é emitida por este papel.
- As medições de geometria cobrem os casos finito/infinito e o caso de float
  pendente descritos, não equivalência funcional geral.
- `Unknown` nos casos executados: `0`.
