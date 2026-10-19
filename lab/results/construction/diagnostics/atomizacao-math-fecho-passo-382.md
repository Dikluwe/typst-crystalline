# Passo 382 — atomização fatia math (final): o layout fecha

> **Estado: COMPLETO. A atomização do LAYOUT fecha aqui.** Caveat de stack:
> `RUST_MIN_STACK=33554432`. HEAD pós-P381. **Suíte verde** (2747+472+24+21+2, 0 falhas; rede `+11`
> + testes de math sem asserção virada); **lint 0/0**; build limpo.

## Fase A — a convenção do subsistema math (medida, não assumida)
O destino é `rules/math/layout/`, um subsistema com **organização própria**:
- `MathLayouter` (`math/layout/mod.rs:223`) + `layout_node` (`:258`, despacha as 16 variantes
  internamente) + `layout_equation` (`:246`) + arquivos por-feature (`frac`/`attach`/`matrix`/`cases`/
  `delimited`/`root`/`stretchy`/`assembly`.rs). **O layout dos nós math já está atomizado lá.**
- A ponte do lado-layout é `engine/layout/equation.rs::Layouter::layout_equation` (`:21`) —
  **convenção `impl Layouter` método** (ADR-0037/P96.7), **não** a forma B free-function.

**Os 3 arms math no `layout_content` eram cola fina** (não lógica de math):
- `Equation` (`@770`): decode do gate `equation.numbering` da chain + `layout_equation`.
- 16-variante agrupado (`@780`): **fallback defensivo** (`plain_text`→`layout_word`) para nós math
  fora de equação — o despacho real é `layout_node`, não isto.
- `MathAlignPoint`/`Linebreak` (`@809`): **no-op**.

## Decisão do dono + movimento (forma medida, não forma B)
**Consolidar a cola em `engine/layout/equation.rs`** (convenção `impl Layouter` do subsistema):
- `Layouter::layout_equation_arm(&mut self, e: &EquationElem)` — decode + bridge.
- `Layouter::layout_math_fallback(&mut self, content: &Content)` — o fallback defensivo.

Arms magros no núcleo:
```
Content::Equation(e) => self.layout_equation_arm(e),
Content::MathSequence(_) | … (16) => self.layout_math_fallback(content),
Content::MathAlignPoint(_) | Content::Linebreak(_) => {}   // no-op fica
```
`equation.rs` mantém a sua linhagem (`layout.md`). **Sem** `dyn`, **sem** wildcard, `entities/`
intacto. (A forma B free-function **não** foi imposta — seguiu-se a convenção `impl Layouter` do
subsistema, como a Fase A apontou.)

## Métrica de leitura — a atomização do LAYOUT está FECHADA
- `layout_content` **564 → 545 linhas** (−19; o body da cola saiu — a lista das 16 variantes
  permanece no `match`, agora a delegar). **Acumulado desde 1857: −1312 (~71%).**
- **44 unidades de domínio atomizadas (P376→P382).**
- **O `layout_content` agora é só: máquina** (`Sequence`/`Styled`/`Dynamic`/`SetPage`) **+ adaptadores
  de introspeção** (`CounterDisplay`/`StateDisplay`/… — [a-decidir]) **+ primitivas/no-ops**
  (`Empty`/`Space`/`Metadata`/`State`/…) **+ pontes finas** (`Equation`/math-fallback/`Ref`/`Labelled`/
  `Outline`/`Align`). **Nenhum elemento de domínio tem layout inline no monólito.**

## Não-metas confirmadas (medido)
`match` exaustivo (0 wildcards) · despacho estático (0 `dyn`) · `entities/` **não tocado** →
`content→elements` inalterado · content-preserving (rede `+11` + testes de math) · INTACTOS α/caso 2,
`morph_canon`/`==`, caso 4, flag P350c, F-5b, numbering, `#set`. **Máquina NÃO tocada.**

## Commits
| Estágio | Commit |
|---|---|
| L0 (pré-código) | `bd81370b5` |
| fatia math (fecho) | (este) |

## A frente seguinte
A atomização do **layout fechou** (elementos de domínio + math no subsistema). Resta:
- a atomização do **`introspect.rs`** (43 arms) — próxima frente;
- os **displays counter/state** ([a-decidir], fronteira de introspeção);
- a **máquina do layouter** fica (Sequence/Styled/Dynamic/SetPage — não são elementos).
Tudo **fora de escopo (Trava 5)** — decisão do dono, passos separados.
