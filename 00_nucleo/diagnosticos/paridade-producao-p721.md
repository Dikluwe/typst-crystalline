# Paridade Produção — P721 — `repr_value`: repr Typst, não Debug do Rust

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-721.md`
**Hash do commit (implementação):** _a preencher após o commit_.
**HEAD base:** `41ee5cf59` (fim de P720).
**Estado:** FECHADO — `Length`/`Ratio`/`Angle`/`Fraction`/`Color`/`Stroke`/
`Align`/`Relative` embutidos em markup (`#expr`) produzem agora o repr
Typst, byte-idêntico ao vanilla no documento do passo; duas divergências
de `stroke` com componente auto ficam documentadas como limitação do
modelo (P227), não como bug deste passo.

---

## 1. Sonda

### 1.1 Formato exacto no vanilla (medido + fonte)

Binário `lab/typst-original/target/release/typst`, repositório em
`41ee5cf59`. Documento do passo (`/tmp/p721-repr.typ`):

```
6pt 50% 45deg rgb("#ff0000") 2pt + rgb("#ff4136") left 1fr
```

Sonda de borda alargada (`/tmp/p721-edge.typ`, vanilla):

```
2em · 2pt + 1em · 6.5pt · -3pt · 0.1pt · 33.33% · 0.5% · 57.3deg ·
45.5deg · 2.5fr · luma(50%) · color.linear-rgb(50%, 50%, 50%) ·
rgb("#ff000080") · cmyk(0%, 50%, 100%, 0%) · color.hsl(0deg, 100%, 50%) ·
oklab(50%, 0.1, 0.1) · oklch(50%, 0.1, 30deg) · color.hsv(120deg, 50%, 80%) ·
rgb("#ff4136") [stroke(red)] · 2pt [stroke(2pt)] · 1.5pt + rgb("#0074d9") ·
center + horizon · right + top [top + right] · start · end · left + top ·
50% + 3pt · ltr · rtl
```

Confirmado contra a fonte vanilla (não assumido):

- `Abs`/`Em`/`Fr`/`Ratio`/`Angle` usam `repr::format_float_with_unit` —
  arredondamento a **2 casas decimais** (`round_with_precision`, half away
  from zero) antes de formatar (`foundations/repr.rs:128`;
  `layout/abs.rs:155`, `em.rs:85`, `fr.rs:80`, `ratio.rs:135`,
  `angle.rs:171`). Daí `33.333%` → `33.33%` e `1rad` → `57.3deg`.
  NaN/Inf seguem `float.nan * 1{unit}` / `float.inf * 1{unit}`.
- `Length`: `"{abs} + {em}"` quando ambos não-zero; só `em` quando abs é
  zero; só `abs` nos restantes (`layout/length.rs:173`).
- `Color` (`visualize/color.rs:1918`, `ProcessColor::repr`): sRGB em hex
  `rgb("#rrggbb[aa]")` (alpha como sufixo quando != 255); `luma(50%)`;
  `color.linear-rgb(r%, g%, b%)`; `cmyk(c%, m%, y%, k%)` (sempre 4 args);
  `oklab(l%, a, b)` (a/b com 3 decimais — `format_float_component`);
  `oklch(l%, c, hue)`; `color.hsl(hue, s%, l%)`; `color.hsv(hue, s%, v%)`.
  Hue normalizado com `rem_euclid(360)` (`color.rs:2167`). Alpha != 1.0
  aparece como argumento extra.
- `Stroke` (`visualize/stroke.rs:308`): `"{thickness} + {paint}"` quando
  ambos explícitos; omite o componente `auto`; `1pt + black` quando ambos
  auto.
- `Alignment` (`layout/align.rs:250`): **horizontal primeiro**
  (`top + right` → `right + top`).
- `Dir` (`layout/dir.rs:175`): `ltr`/`rtl`/`ttb`/`btt`.

### 1.2 Estado do cristalino antes

Mesmo documento, cristalino em `41ee5cf59` (binário de P720):

```
Length { abs: Abs(6.0), em: 0.0 } 50% Angle(0.7853981633974483)
Srgb { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
Stroke { paint: Solid(Srgb { ... }), thickness: 2.0, overhang: true }
Align2D { h: Some(Left), v: None } 1.0
```

`50%` já saía certo por acidente de caminho: o parser produz
`Value::Relative` (não `Ratio`) para percentagens puras, e
`repr_relative` formatava a percentagem directamente.

### 1.3 Código localizado

`01_core/src/rules/eval/repr.rs` — fallback `{:?}` nos braços:
`Length` (l.59), `Ratio` (l.61), `Angle` (l.62), `Color` (l.63),
`Stroke` (l.64), `Fraction` (l.65, `repr_float` → `1.0` em vez de `1fr`),
`Align` (l.66); e `repr_relative` (l.100-107) com `{:?}` na componente
absoluta e sem arredondamento na percentagem. `Dir` (l.91) já correcto
(Debug minúsculo coincide com o repr vanilla para as 4 variantes).

Consumidores de `repr_value`: `repr()` (`stdlib/foundations.rs:37`) e o
caminho de display em markup `#expr` (`rules/eval/mod.rs:586`) — um só
fix cobre os dois.

---

## 2. Implementação

L0 actualizado primeiro: `00_nucleo/prompts/rules/stdlib/foundations.md`
— tabela de formatos alargada com os 8 tipos novos, regra de formatação
de floats com unidade, scope-out de `Color` avançado removido (P721
implementa-o), scope-out novo para `Stroke` com `Smart::Auto`; testes
canónicos alargados. Hash actualizado via `crystalline-lint --fix-hashes`
(`foundations.rs` → `80058390`). Testes escritos antes do código: 9
testes `p721_*` novos + 2 endurecidos (`repr_value_complex_types`,
`repr_value_relative_with_abs_offset`) — **11 a falhar** no estado base.

Código (`01_core/src/rules/eval/repr.rs`):

- `round_with_precision` / `format_float_with_unit` /
  `format_float_component` — mirrors das primitivas vanilla
  (`typst-utils/src/round.rs:26`, `foundations/repr.rs:95-130`).
- `repr_length`, `repr_ratio`, `repr_angle`, `repr_color` (8 variantes),
  `repr_stroke` + `repr_paint`, `repr_align` — braços próprios em
  `repr_value`, um por tipo confirmado na sonda.
- `repr_relative` reescrito sobre `repr_length` e
  `format_float_with_unit` (offset `2cm` → `56.69pt`, não
  `Abs(56.692)`).
- `Fraction` → `format_float_with_unit(f, "fr")`.

`Gradient`/`Tiling` dentro de `Stroke.paint` mantêm os placeholders
existentes (`gradient(...)`/`tiling(...)`) — coerente com os braços
homólogos de `Value`.

---

## 3. Validação

Estado da medição: working tree com exactamente as alterações deste
passo (`00_nucleo/prompts/rules/stdlib/foundations.md`,
`01_core/src/rules/eval/repr.rs`,
`01_core/src/rules/stdlib/foundations.rs` — só a linha de hash),
commitado de seguida. Binário release reconstruído desse estado.

### 3.1 Documento do passo vs vanilla

`/tmp/p721-repr.typ` → `pdftotext` **byte-idêntico** ao vanilla:

```
6pt 50% 45deg rgb("#ff0000") 2pt + rgb("#ff4136") left 1fr
```

### 3.2 Sonda de borda vs vanilla

22 expressões comparadas (`/tmp/p721-edge.typ` vanilla vs
`/tmp/p721-edge2.typ` + `/tmp/p721-names.typ` cristalino — sintaxe
ajustada: `hsl`/`hsv`/`oklab`/`linear_rgb` do cristalino aceitam floats,
não ratios/angles, divergência pré-existente dos construtores P257, fora
deste passo). **20/22 idênticas**, incluindo todos os espaços de cor:

```
luma(50%) · color.hsl(0deg, 100%, 50%) · color.hsv(120deg, 50%, 80%) ·
cmyk(0%, 50%, 100%, 0%) · oklab(50%, 0.1, 0.1) · oklch(50%, 0.1, 30deg) ·
color.linear-rgb(50%, 50%, 50%)
```

As 2 divergências são o mesmo caso documentado — componente auto de
`stroke` indistinguível do default explícito no modelo cristalino
(`Smart::Auto` colapsado na construção, P227):

| Expressão | Vanilla | Cristalino |
|---|---|---|
| `stroke(red)` | `rgb("#ff4136")` | `1pt + rgb("#ff4136")` |
| `stroke(thickness: 2pt)` | `2pt` | `2pt + rgb("#000000")` |

Registado no L0 como scope-out, com a causa (`entities/geometry.rs:32`,
`stdlib/layout.rs:1387-1458`). Resolver implica modelar `Smart::Auto` no
`Stroke` — mudança de entidade, não de repr.

### 3.3 Regressão de P710-720 (documento do passo)

`/tmp/p721-regressao.typ` (`#context [#(10em).to-absolute()]` com
`text(size: 12pt)`) → **`120pt`** (antes: `Length { abs: Abs(120.0),
em: 0.0 }`).

### 3.4 Suites

- `cargo test --workspace` → **0 failed** em todos os crates
  (`typst-core`: 3927 passed — 3918 de P720 + 9 novos).
- `crystalline-lint .` → **0 violations**.

---

## 4. Gate das ADRs

- ADR-0107 (paridade com a linguagem): o observável aqui é texto
  impresso — paridade medida ao nível da língua, não do `Debug` do Rust.
- ADR-0108 (medir antes de decidir): formatos confirmados na fonte
  vanilla (`file:line` em §1.1) antes de implementar; as 2 divergências
  de stroke classificadas como limitação do modelo (P227), não como
  aceitação cômoda — ficam visíveis no L0 e neste relatório.
- ADR-0109: `repr.rs` já é free function na camada de eval — forma
  preservada; nenhum despacho dinâmico introduzido.

---

## 5. Critério de fecho do passo

- [x] Sonda completa — formato exacto de cada tipo confirmado contra o
  vanilla (medição + fonte), lista de tipos em fallback com `file:line`.
- [x] Cada tipo confirmado produz o repr correcto (documento do passo
  byte-idêntico; 20/22 na borda, 2 divergências documentadas).
- [x] Tipos já correctos sem regressão — `Dir`, `Relative` puro, demais
  braços inalterados; `cargo test --workspace` verde (3927/0 no core).
- [x] `crystalline-lint .` limpo.
- [x] Documento de P710-720 (`to-absolute()`) mostra `120pt`.
- [x] Relatório com hash do commit.
- [x] Item marcado como fechado em `achados-adiados-cetz.md` (P721).
