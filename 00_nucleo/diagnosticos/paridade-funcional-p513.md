# Relatório de Paridade Funcional — Passo 513

**Data:** 2026-06-30  
**Tema:** `curve.move` / `curve.line` / `curve.cubic` / `curve.quad` / `curve.close`

## Resumo

Materialização dos cinco elementos de segmento de curva do Typst no compilador cristalino. O trabalho cobre:

- Entidade `CurveElem` com `CurveSegment` (`Move`, `Line`, `Cubic`, `Quad`, `Close`) e `CurvePoint` (coordenadas em `Length`).
- Variante `Content::Curve(Arc<CurveElem>)`.
- Construtores ergonómicos `Content::curve_move/line/cubic/quad/close`.
- Funções nativas `native_curve_move/line/cubic/quad/close`.
- Namespace `curve` anexado à função `curve` existente.
- Consumo de `Content::Curve` como argumento posicional da função `curve(...)` (concatenação de segmentos).
- Layout atomizado em `rules/layout/curve.rs`, que converte segmentos para `PathItem` absolutos e emite `FrameItem::Shape`.
- Registo do variant em `repr.rs`, `introspect.rs`, `introspect/locatable.rs` e `query_helpers.rs`.

## Decisões tomadas

### 1. `Length` em vez de `f64` nos pontos

`CurvePoint` armazena `x: Length` e `y: Length`. Isto permite aceitar `10pt`, `1em` e literais numéricos (`10` tratado como pt) na superfície Typst. No entanto:

- Em `native_curve(...)` (eval-time), a componente `em` não é resolvível (não há font-size disponível). Usa-se apenas `Length::abs` para converter para `PathItem` absoluto. `em` em argumentos de `curve(...)` fica como scope-out documentado.
- Em `layout/curve.rs`, o font-size actual do `Layouter` é usado para resolver `Length` completo (`resolve_pt`).

### 2. `curve.quad` converte para cúbica

Seguindo a paridade vanilla e a implementação prévia de `native_curve` (P294), `CurveSegment::Quad` é convertido para `PathItem::CubicTo` com a fórmula `control_q2c(p, c) = (p + 2c) / 3`. A conversão é idêntica tanto em `native_curve` (quando recebe `Content::Curve`) como no layout de `Content::Curve`.

### 3. Layout standalone de `Content::Curve`

Um `curve.move(...)` usado fora de `#curve(...)` produz um `Content::Curve` que é renderizado directamente: stroke preto 1pt, sem preenchimento. Esta é uma aproximação razoável para o subset P513; a semântica vanilla de herdar `fill`/`stroke` de um `#curve(...)` pai não se aplica quando o segmento é usado isoladamente.

### 4. Syntaxe legada preservada

A sintaxe descritiva de tuplos `curve(("move", (0,0)))` continua a funcionar sem alterações. A nova funcionalidade é aditiva.

## Ficheiros alterados

- L0:
  - `00_nucleo/prompts/entities/elements/curve.md` (novo)
  - `00_nucleo/prompts/rules/stdlib/curve.md` (novo)
- L1:
  - `01_core/src/entities/content.rs` (variante + construtores)
  - `01_core/src/entities/elements/curve.rs` (novo)
  - `01_core/src/rules/stdlib/shapes.rs` (`native_curve_*` + consumo de `Content::Curve`)
  - `01_core/src/rules/stdlib/mod.rs` (re-exports + testes)
  - `01_core/src/rules/eval/mod.rs` (namespace `curve`)
  - `01_core/src/rules/eval/repr.rs` (representação)
  - `01_core/src/rules/introspect.rs` (terminal em walk/materialize_time)
  - `01_core/src/rules/introspect/locatable.rs` (não-locatable)
  - `01_core/src/rules/layout/mod.rs` (braço de layout + medição)
  - `01_core/src/rules/layout/curve.rs` (novo — layout atomizado)
- L3:
  - `03_infra/src/query_helpers.rs` (não-texto / count terminal)

## Validação

### Testes unitários

```text
cargo test -p typst-core
=> 3544 passed; 0 failed

cargo test -p typst-wiring
=> 21 passed; 0 failed (mais 2 do crystalline_lint.rs)
```

Foram adicionados testes para:
- `native_curve_move/line/cubic/quad/close` (devolvem `Content::Curve` com segmento correcto).
- `native_curve` a aceitar `Content::Curve` como argumentos posicionais e concatenar segmentos.

### Compilação de snippets

```text
cargo build  => OK
```

Snippets validados (todos produziram PDF sem erros):

```typst
#curve(
  curve.move((0pt, 0pt)),
  curve.line((100pt, 0pt)),
  curve.line((50pt, 80pt)),
  curve.close(),
  fill: blue,
  stroke: black + 2pt,
)
```

```typst
#curve(
  curve.move((0pt, 100pt)),
  curve.cubic((25pt, 0pt), (75pt, 0pt), (100pt, 100pt)),
  stroke: red + 2pt,
)
```

```typst
#curve(
  curve.move((0pt, 100pt)),
  curve.quad((50pt, 0pt), (100pt, 100pt)),
  stroke: green + 2pt,
)
```

Sintaxe legada ainda funciona:

```typst
#curve(("move", (0, 0)), ("line", (100, 0)), ("line", (50, 80)), ("close",))
```

### Corpus P490 + P500

Todos os ficheiros dos corpus P490 e P500 foram compilados com o binário cristalino sem erros:

```text
lab/parity/corpus/p490/*.typ => OK (19 ficheiros)
lab/parity/corpus/p500/*.typ => OK (17 ficheiros)
```

### Linter

```text
crystalline-lint .
=> ✓ No violations found
```

## Scope-outs / divergências conhecidas

1. **Resolução de `em` em `native_curve(...)`:** argumentos `Content::Curve` com componente `em` são convertidos usando apenas a parte absoluta em `native_curve`. O layout standalone resolve `em` correctamente.
2. **`fill`/`stroke` em segmentos isolados:** `curve.move(...)` usado fora de `#curve(...)` renderiza com stroke preto 1pt e sem fill, independentemente de `#set` rules.
3. **Relativos (`Relative`, `Ratio`) como coordenadas:** scope-out do Passo 513; aceitam-se `Length`, `Float` e `Int`.
4. **A ferramenta `typst-parity` (`cargo test --manifest-path lab/parity/Cargo.toml`) não corre** devido a um erro de compilação pré-existente em `lab/parity/src/value_dto.rs` (variantes `Value::State`/`Counter`/`Label` não cobertas), não relacionado com o Passo 513.

## Conclusão

Os cinco elementos de curva estão materializados e validados. O critério de saída `crystalline-lint .` com zero violations é satisfeito; a suíte de testes mantém-se verde.
