# Relatório P727 — corrigir o render de `curve` (página em branco)

**Data:** 2026-07-10 (execução: 2026-07-13)
**Commit:** A PREENCHER após o commit (ver secção final)
**Proveniência das medições:** commit base `4e37ac1b77af961e2838252b970e2a4953a36b02`
(P726 hash-fill) + working tree não commitado com exactamente 6 ficheiros alterados
(`git diff HEAD --stat` no momento da validação):

```
 00_nucleo/diagnosticos/achados-adiados-cetz.md |  6 +-
 00_nucleo/prompts/rules/stdlib/shapes.md       | 22 +++++--
 01_core/src/rules/layout/tests.rs              | 19 +++++-
 01_core/src/rules/stdlib/mod.rs                | 91 ++++++++++++++++++++++++++
 01_core/src/rules/stdlib/shapes.rs             | 19 ++++--
 03_infra/src/integration_tests.rs              | 18 +++++
```

Binários: cristalino `./target/release/typst` (reconstruído após a correcção),
vanilla `lab/typst-original/target/release/typst`. Rasterização `mutool draw -r 150`.
Diff de pixels com decoder PNG próprio (`/tmp/pngdiff.py`, zlib/struct puro — o venv
não tem PIL): não-brancos = qualquer canal `< 240`; diff = bytes de pixel com `|Δ| > 8`.

---

## Sonda — medições antes de decidir (ADR-0108)

### Caso mínimo reproduzido

`/tmp/p727-curve-minimo.typ` = `#curve(curve.move((0pt,0pt)), curve.line((50pt,50pt)))`
(só move+line, sem cubic/close) já falha: vanilla 10576 bytes de PNG com a forma
visível; cristalino 10122 bytes = tamanho conhecido de página em branco (mesmo valor
medido em P723/P726). O bug não está em segmentos complexos.

### O PDF tem o path, mas sem pintura

`mutool show /tmp/p727-cristalino.pdf 4` → content stream:

```
q
70.87 759.99 m
120.87 709.99 l
Q
```

O path existe (operadores `m`/`l`) mas **falta o operador de pintura** (`S`) e a cor de
stroke. O vanilla (mesmo documento) emite `/c0 CS 0 SCN 4 M 0 0 m 50 50 l S`.
Logo: conteúdo invisível, não ausente — a informação perde-se antes do export.

### Onde a informação se perde (file:line)

Medição por instrumentação (teste temporário com dump dos FrameItems da Page,
removido antes do commit):

1. A Page resultante do layout já traz `stroke: None`
   (`Shape { kind: Path([MoveTo, LineTo]), fill: None, stroke: None, ... }`).
2. O `eprintln!` em `curve::layout` (`01_core/src/rules/layout/curve.rs:118`) **não
   dispara** — o Shape na Page não vem desse caminho.
3. `native_curve` (`01_core/src/rules/stdlib/shapes.rs:452`) produz
   `Content::Shape { kind: Path }`, não `Content::Curve` — o dispatch
   `Content::Curve => curve::layout` (`rules/layout/mod.rs:866`) é código morto para
   este caso (fica para `#curve.move(...)` directo no markup).
4. **Causa raiz — `01_core/src/rules/stdlib/shapes.rs:592-595`**: `fill` e `stroke`
   vêm só dos argumentos nomeados; quando o utilizador não passa `stroke`, o Shape
   fica com `stroke: None` e o export não tem nada para pintar.
5. O vanilla aplica `Smart::Auto`: sem fill nem stroke → `FixedStroke::default()`
   (1pt preto); com fill sem stroke → sem stroke
   (`lab/typst-original/crates/typst-layout/src/shapes.rs:126-129`, documentado em
   `visualize/curve.rs:46-47`: "When setting a fill, the default stroke disappears").
6. O mesmo fallback já existia no cristalino em `native_rect`/`native_square`/
   `native_ellipse`/`native_circle` (`shapes.rs:96-101` e paralelos) — só faltava em
   `native_curve` (e em `native_polygon`, ver achados adiados).

Inocentados com leitura de código: export (`03_infra/src/export/stream.rs:518-523`
e `:779-784` emitem `S` quando `stroke.is_some()`), layout de curve, paginação/
slicing (`slicing.rs:108-113` preserva stroke), `cursor.rs:516-521,708-712`.

### ADRs em vigor — grep pelos termos centrais (curve, path, render)

Nenhuma ADR vigente decide defaults de stroke; as relevantes confirmam a forma da
correcção: **ADR-0107** (paridade é com a linguagem — o stroke default é semântica do
elemento `curve`, não detalhe mecânico), **ADR-0108** (medição acima precede a
classificação), **ADR-0109** (atomização — o `match` magro fica; a correcção é no
stdlib, não toca o dispatch), **ADR-0054** (scope-outs graded — este fallback não é
scope-out: `rect`/`circle` já o implementam no cristalino).

---

## Implementação

Fluxo L0-first (Regra de Ouro):

1. **L0 actualizado**: secção `native_curve` de `00_nucleo/prompts/rules/stdlib/shapes.md`
   passa a especificar o fallback determinístico (paridade vanilla `Smart::Auto`) e os
   novos casos canónicos; corrigida ainda a nota desactualizada que marcava os
   constructores `curve.move`/… como scope-out (existem desde P513 — prompt dedicado
   `stdlib/curve.md`). Hash recalculado (`crystalline-lint --fix-hashes .` →
   `9653507a` no header de `shapes.rs`).
2. **Testes primeiro** (fail-first confirmado antes da implementação):
   - `01_core/src/rules/stdlib/mod.rs` — 3 testes: `curve` sem cores → stroke preta
     1pt; `curve` com fill → sem stroke; `curve` com stroke explícito → preservado.
     Os dois primeiros falhavam, o terceiro passava (comportamento já correcto).
   - `01_core/src/rules/layout/tests.rs` — E2E markup → Page:
     `p727_layout_curve_fallback_stroke_chega_a_pagina` (falhava).
   - `03_infra/src/integration_tests.rs` — E2E pipeline completo → PDF:
     `p727_pdf_curve_contem_operador_stroke` exige `S\n` no content stream (falhava).
3. **Correcção** (`01_core/src/rules/stdlib/shapes.rs:592-607`): aplicação do padrão
   já existente em `native_rect` — `parsed_stroke` separado + fallback
   `if fill.is_none() && parsed_stroke.is_none()` → stroke preto 1pt. Sem tocar em
   `extract_stroke`, export ou dispatch.

Os 5 testes passam após a correcção.

---

## Validação

### Caso mínimo — diff de pixels (prova visual)

`/tmp/p727-depois.png` (cristalino corrigido) vs `/tmp/p727-vanilla.png`:

| Métrica | Antes (P726) | Depois |
|---|---|---|
| não-brancos cristalino | 0 | **523** |
| não-brancos vanilla | 523 | 523 |
| diff (`|Δ| > 8`) | — | 3138 / 6530142 bytes (**0.048%**) |

0.048% ≈ 523 px × 6 canais de borda — anti-aliasing da linha diagonal. Paridade
visual do caso mínimo: fechada.

### Regressões

- `cargo test --workspace`: 0 falhas (3981 + 631 + 33 + 27 + 2×2 testes).
- `crystalline-lint .`: 0 violations.
- `rect`/`circle` sem regressão (fallback partilhado inalterado).

### cetz — diff de pixels final (número exacto)

`/tmp/p727-cetz.typ` (`line((0,0),(2,1))` + `circle((0,0))`, cetz 0.5.2):

| Métrica | P726 (cristalino) | P727 (cristalino) | Vanilla |
|---|---|---|---|
| não-brancos | 0 | **1100** | 1451 |
| diff vs vanilla | — | 8043 / 6530142 (**0.123%**) | — |

O círculo do cetz agora renderiza (é um path de 4 cúbicas via `std.curve` → fallback
P727). **A linha continua ausente — por causa de um bug NOVO, descoberto e medido
nesta validação, fora do escopo do passo:**

`and`/`or` **sem short-circuit** em `01_core/src/rules/eval/mod.rs:687-692` — o
dispatch genérico de `Expr::Binary` avalia ambos os operandos antes de despachar.
Caso mínimo puro da linguagem (sem cetz): `#let a = (1, 2)` +
`type(a) == str and a.contains(".")` → erro "campo desconhecido em array: 'contains'"
(vanilla: `false`, sem erro). O cetz depende disto em
`~/.cache/typst/packages/preview/cetz/0.5.2/src/draw/shapes.typ:608,611`
(`type(first-elem) == str and not first-elem.contains(".")`): com a coordenada
`(0,0)` (array) o segundo operando não devia ser avaliado. Medido por instrumentação
temporária (removida): o field access dispara sobre o array `[Int(0), Int(0)]` — o
primeiro ponto da linha. Comportamentos medidos: `line` sozinha ou após `circle` →
compilação falha; `line` antes de `circle` → compila mas a linha não aparece
(anomalia registada para investigação — suspeita de memoização a esconder o erro).

Achado registado em `achados-adiados-cetz.md` como bloqueio actual, candidato a P728.
Registado ainda, por inspecção de código (sem caso medido — ADR-0108): `native_polygon`
tem o mesmo defeito de fallback que `curve` tinha (`shapes.rs:348-351` vs vanilla
`shapes.rs:336-339`).

---

## Campos fixos cetz (estado após P727)

| Elemento cetz | Caminho | Estado |
|---|---|---|
| `circle` | `std.curve` + 4 cúbicas + fallback stroke P727 | **Renderiza** (pixels próximos do vanilla) |
| `line` | `draw/shapes.typ:608` → bug `and` short-circuit | Bloqueado (P728) |

---

## Critério de fecho do passo

- [x] Sonda completa, causa exacta localizada (`shapes.rs:592-595`).
- [x] Corrigido, testado com diff de pixels no caso mínimo (523 vs 523, 0.048%).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] cetz re-testado — diff final registado com número exacto (1100 vs 1451; 0.123%).
- [x] Item `curve` marcado como fechado em `achados-adiados-cetz.md`.
- [ ] **A cadeia P678-727 NÃO fecha aqui** — o bug do render de `curve` (objecto deste
  passo) está corrigido com prova visual, mas a paridade de pixels do cetz tem um
  último bloqueio conhecido: `and`/`or` sem short-circuit (`eval/mod.rs:687-692`),
  encontrado e medido nesta validação. Fica como candidato natural a P728; quando
  esse fechar, o resumo da cadeia completa (50 passos, P678–P728) fica para o
  relatório desse passo.
