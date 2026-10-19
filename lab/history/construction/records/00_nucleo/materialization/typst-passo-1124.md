# L0 — Passo 1124: Implementar as 8 Correcções Já Diagnosticadas e Reconciliadas

**Gate**: `ADR-0127` — mudança de comportamento por defeito em 8 pontos,
todos já com causa citada em código real e reconciliados sem contradição
(P1122, P1123, e as duas rondas de reconciliação subsequentes).

---

## 1. Regressão da secção 35 (URGENTE — reverter/corrigir primeiro)

**Base**: commit `418c90181` aplicou `gap = prev_block_below_pending`
incondicionalmente para equações, ignorando `above:1.2em` (`13.20pt`)
explícito do `BlockElem` da equação. Correcto é
`gap = max(prev.below, curr.above)` — `max(8.25,13.20)=13.20pt` quando a
equação tem `above` maior que o `below` do heading, não sempre `8.25pt`.

**Mecanismo**: corrigir `equation.rs` para usar o `max()` genérico já
estabelecido em `block.rs` (P1061), não o atalho incondicional
introduzido em `418c90181`. Confirmar que isto **não** regride o caso já
validado em P1063/P1108 (onde `above` da equação era implicitamente
menor ou igual a `8.25pt` nalguns dos testes Iso — reconfirmar quais).

**Verificação**: secções 35 e 32 convergindo para os valores já
reconciliados (`59.53166pt` e `59.20036pt` respectivamente), Iso A-D do
P1063 revalidados.

## 2. Secção 27 — `+` antes de `dif` soma em vez de usar precedência

**Base**: `structural/math.rs` define `dif` com `HSpace` próprio
(`1/6em`); `spacing.rs` soma o espaçamento de classe `Binary` a isso, em
vez dos dois espaçamentos concorrentes serem resolvidos por precedência
(mesmo princípio "fraqueza"/`max()` já usado para espaçamento vertical,
aqui horizontal).

## 3. Secção 41 — sobrescrito empilhado usa `base_ascent` errado

**Base**: `attach.rs`, `compute_script_shifts` — quando a base já tem
scripts anteriores, `base_ascent` recebe a altura da caixa acumulada em
vez da altura do núcleo, inflando `drop_term` só do lado do sobrescrito.

## 4. Secção 31 — `sum` inline usa constantes de glifo simples

**Base**: `attach.rs`, scripts laterais de operador grande em modo
inline usam `superscript_shift_up`/`subscript_shift_down` de glifo comum,
ignorando a altura do glifo base grande e `sub_superscript_gap_min`.

## 5. Secções 26/28 — `|` sem entrada em `spacing_between_class`

**Base**: `spacing.rs`, falta braço `(MathClass::Fence, _)` — cai no
braço genérico `(_, Relation) => THICK`.

## 6. Secção 19 — `MathStyled` não actualiza `math_style`

**Base**: `math/layout/mod.rs`, braço `Content::MathStyled` aplica
`size_factor` a `style.size` mas não actualiza `math_style.math_size`
nem `cramped` — `layout_frac` interno continua a ler `MathSize::Display`.

## 7. Secção 23 — vírgula ASCII literal em fallback de chamada com string

**Base**: `eval/math.rs`, fallback de múltiplos argumentos posicionais
para callee-string (`#let Res = "Res"`) injecta `Content::MathText(", ")`
com espaço ASCII literal em vez de passar pela classe `Punctuation`
matemática correcta — mais o efeito de centragem somado (§ já
reconciliado).

## 8. Secção 23 (Eq2→Eq3) — descent de integral com círculo (`integral.cont`)

**Base**: `equation.rs:385`, `prev_block_equation_descent = ext.descent`
— `descent` de `integral.cont_gamma` medido maior no cristalino
(`4.30pt`) que no vanilla (`2.75pt`). Confirmar a métrica real do
glifo/variante antes de corrigir — não presumir que é o mesmo tipo de
bug já visto (`glyph_ink_bounds`) sem verificar para este glifo
específico (integral com círculo é variante menos comum, pode ter
métrica própria na fonte).

## Critérios de verificação

1. §1: secções 35, 32, Iso A-D todos a `0.0000pt` (±0.0005pt).
2. §2-8: cada secção/caso convergindo para os valores já reconciliados
   nas rondas anteriores (não remedir do zero — usar os números já
   validados como alvo).
3. Re-rodar P1086-1123 por inteiro — risco real de regressão dado o
   número de arquivos tocados (`equation.rs`, `attach.rs`, `spacing.rs`,
   `math/layout/mod.rs`, `eval/math.rs`).
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1 implementado e verificado antes de qualquer outra correcção ser
  considerada — é a de maior risco (regressão em trabalho já fechado).
- Cada uma das 8 correcções com o código real (já citado nas rondas
  anteriores) efectivamente alterado, não só descrito.
- Nenhuma correcção nova introduzida sem o mesmo nível de reconciliação
  já aplicado a estas 8.
