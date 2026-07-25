# Prompt L0 — `rules/math/layout/spacing` — espaçamento automático por `MathClass`
Hash do Código: fa38cb8e

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/spacing.rs`
**Origem**: **P772y**. Cita `rules/math/layout/_comum.md` (struct/despacho
partilhados). Nono submódulo de `rules/math/layout/` (ver lista em
`_comum.md`). Mecanismo vanilla: `math/ir/process.rs::spacing()` +
`math/ir/item.rs::MathItem::lclass()/rclass()` + `math/mod.rs`
(`THIN`/`MEDIUM`/`THICK`).

---

## Contexto

Antes de P772y, `layout_sequence` concatenava os `MathBox` de uma
`MathSequence` sem qualquer espaço extra entre eles (`hconcat` — soma pura
de larguras). P772y confirmou por sonda (`mutool trace`, medição de deltas
de posição x em `a = b` / `a + b` / `(a)`) que o cristalino não tinha
nenhum mecanismo de espaçamento por classe — o vanilla usa uma tabela de
espaçamento inter-símbolo estilo TeX baseada em `MathClass` (Relação,
Binário, Abertura/Fecho, Pontuação, Operador grande), com três larguras
fixas: `THIN = 1/6 em`, `MEDIUM = 2/9 em`, `THICK = 5/18 em`.

## Interface pública (`pub(super)`)

```rust
/// `(lclass, rclass)` efectivos de um nó `Content` matemático.
pub(super) fn node_math_class(content: &Content) -> (MathClass, MathClass);

/// Promove Vary → Binary quando precedido por Normal|Alphabetic|Closing|Fence.
pub(super) fn promote_vary(class: MathClass, prev_rclass: Option<MathClass>) -> MathClass;

/// Espaço extra (pt) entre `l_rclass` (nó à esquerda) e `r_lclass` (nó à direita).
/// Assinatura/comportamento pré-P903 preservados (catch-all → 0.0) — thin
/// wrapper sobre `spacing_between_class` (privada, devolve `Option<f64>`,
/// `None` = nenhuma regra explícita, distinto de "regra explícita = 0.0";
/// ver P903 abaixo).
pub(super) fn spacing_between(l_rclass: MathClass, r_lclass: MathClass, size_pt: f64) -> f64;

/// Gaps (n-1) entre n nós adjacentes de uma sequência, com promoção Vary
/// aplicada sequencialmente. `in_script` (P891) — quando verdadeiro (toda
/// a sequência está dentro de um script de `MathAttach`), todos os gaps
/// são 0 (suprime `spacing_between`, paridade `process.rs::spacing()`
/// vanilla, condição "unless in script size"). `text_space_pt` (**P903**)
/// — largura de um espaço de texto normal no estilo/tamanho actual, medida
/// pelo caller via `FontMetrics::advance(" ", ...)`; usada como fallback
/// quando nenhuma regra explícita de classe se aplica e um dos nós
/// adjacentes é `Content::Text` (texto literal entre aspas).
pub(super) fn compute_gaps(
    nodes: &[Content],
    size_pt: f64,
    in_script: bool,
    text_space_pt: f64,
) -> Vec<f64>;
```

## Classificação por nó (`node_math_class`/`base_math_class`)

| `Content` | classe |
|---|---|
| `MathIdent(name)` | `default_math_class(primeiro char)`, fallback `Alphabetic` |
| `MathText(text)` | `default_math_class(primeiro char)`, fallback `Normal` |
| `Text(_)` | **`Alphabetic`** (P903 — texto literal entre aspas, `"..."` bare em modo math; paridade `TextItem::create`, vanilla, "spaced and has alphabetic math class"; antes caía em `_` → `Normal`) |
| `MathStyled(m)` | recurse em `m.body` (estilo não muda classe) |
| `MathClassOverride(e)` | `e.class` (override explícito, `math.class(...)`) |
| `MathDelimited(_)` | `(Opening, Closing)` — sempre assimétrico (paralelo `MathItem::lclass/rclass`, vanilla, caso Fenced) |
| outros (Frac/Attach/Root/Matrix/Cases/Accent/Cancel/Underover/Op) | `Normal` (paridade `unwrap_or(MathClass::Normal)`, vanilla — nenhum destes define `class` explícito) |

Simplificação registada: um nó `MathIdent`/`MathText` multi-carácter usa
**só o primeiro carácter** para classificar o nó inteiro (o layout do
cristalino trata cada nó da sequência como uma unidade já layoutada, ao
contrário do vanilla que resolve por-glifo). Correcto para o caso comum
(operadores/relações são nós de 1 carácter); identificadores multi-letra
(`sin`, `xyz`) classificam-se como `Alphabetic`/`Normal` de qualquer forma.

## Tabela de espaçamento (`spacing_between`) — paridade `process.rs::spacing()`

Ordem dos ramos do `match` é significativa (primeiro match ganha, tal como
o vanilla):

1. Antes de `Punctuation` → 0. Depois de `Punctuation` → `THIN`.
2. Depois de `Opening` / antes de `Closing` → 0.
3. `Relation`-`Relation` → 0. Ao redor de `Relation` (não ambos) → `THICK`.
4. Ao redor de `Binary` → `MEDIUM`.
5. Ao redor de `Large`, excepto antes de `Opening`/`Fence` → `THIN`.
6. Default → 0.

**P891 (achado 1 de P885/P889) — "unless in script size" implementado**:

Confirmado no código-fonte do vanilla (`typst-library/src/math/ir/process.rs:277-319`,
função `spacing`) que a condição suprime o espaço **por completo** (não reduz para um
valor menor) — cada ramo do `match` só chama `set_rspace`/`set_lspace` quando
`!script(l)`/`!script(r)` (`script(f) = f.size().is_some_and(|s| s <= MathSize::Script)`,
`process.rs:284`). A verificação é por lado independente (o item à esquerda e o item à
direita podem ter tamanhos diferentes no vanilla, já que `MathSize` é discreto e
propagado por item).

Cristalino não tem um `MathSize` discreto por item — `layout_sequence` chama
`compute_gaps(&filtered, style.size.val())` com **um só `TextStyle` para toda a
sequência** (todos os nós de uma chamada partilham o mesmo estilo/tamanho). Isto
simplifica a adaptação: em vez de verificar cada lado independentemente, um novo campo
`TextStyle::math_script: bool` (paralelo a `.math`, mesmo padrão P784) é lido **uma vez
por chamada** de `compute_gaps` — verdadeiro quando o body inteiro está dentro de um
script (sub/índice ou super-índice de `MathAttach`; `attach.rs` marca-o ao construir o
`script_style` que já reduz `size` por `script_percent_scale_down`). Quando
`math_script` é verdadeiro, **nenhuma regra de `spacing_between` é aplicada** (early
return antes do match) — suprime por completo, paridade com o comportamento confirmado
do vanilla para o caso comum (sequência inteira em script size, como `i=0` dentro de
`sum_(i=0)^n`). Casos com tamanhos MISTOS dentro do mesmo nível de sequência (que o
vanilla resolveria por item) não são um caso observado nos benchmarks actuais — scope-out
residual, registado aqui, não silencioso.

**P903 — mecanismo "spaced" (vanilla `is_spaced()`/`process.rs::spacing()`,
ramo `_ if l.is_spaced() || r.is_spaced() => return space`) parcialmente
implementado**: catalogado em P897 (fora de âmbito nesse passo), confirmado
e corrigido aqui **só para `Content::Text`** (texto literal entre aspas —
`TextItem::create`, vanilla, `.with_spaced(true)`). Cristalino não replica o
par genérico `(class, spaced: bool)` por item do vanilla; em vez disso,
`compute_gaps` recebe os `Content` nodes directamente e verifica
`matches!(node, Content::Text(_))` node a node — suficiente para este caso
sem precisar de um novo campo em `MathClass`/`Content`. Fallback só
dispara quando `spacing_between_class` (nova função privada, devolve
`Option<f64>`) devolve `None` — nenhuma regra explícita de classe se
aplicou — preservando a prioridade do vanilla (regras explícitas de
Punctuation/Opening/Closing/Relation/Binary/Large, mesmo quando o
resultado É 0.0, continuam a ganhar sobre o fallback de item espaçado,
tal como a ordem do `match` em `process.rs::spacing()`). `text_space_pt`
medido via `FontMetrics::advance(" ", ...)` pelo caller (`layout_sequence`),
não hardcoded — confirmado ≈3.65pt a 11pt via `mutool trace`, mesma ordem
de grandeza já registada em P825 (ver abaixo).

**Fora de escopo, ainda por implementar (registado, P772y/P825 — só o caso
`Content::Text` foi resolvido por P903)**:
- A regra "spaced frames" genérica (`#h()` explícito dentro de math) —
  sem equivalente no cristalino hoje.
- **P825 (sub-C de P810 §12)** — dois efeitos do mecanismo "spaced" **ainda
  não implementados**:
  - **`fence` "spaced"**: `$ a | b $` tem gap ≈ 3.65pt dos dois lados de
    `|` no vanilla (medido em P825 por `mutool trace`); o cristalino dá
    ≈ 0 (tabela de classes — `(Alphabetic, Fence)`/`(Fence, Alphabetic)`
    não têm regra, e `|` não é `Content::Text`, logo o fallback de P903
    não se aplica). Reconfirmado ainda presente após P903
    (`typst-passo-903-relatorio.md`, achado incidental).
  - **`class("normal", ...)` "spaced"**: `$ a #math.class("normal", "+") b $`
    — o vanilla marca o átomo wrapped como spaced (gaps ≈ 3.65pt); o
    cristalino aplica a tabela com a classe overridden (Normal-Normal →
    gap 0). Divergência de língua conhecida e registada, não bug a
    corrigir nesta linha de trabalho.
- **Achado incidental de P903**: `min_(x) f(x)` (função com limite +
  argumento adjacente) não tem espaço entre "min" e "𝑓(𝑥)" no cristalino
  (`min𝑓(𝑥)`), vanilla mostra `min 𝑓(𝑥)`. Confirmado reproduzível SEM
  qualquer texto literal envolvido (não é o mesmo mecanismo de P903) —
  candidato a passo dedicado futuro, não investigado aqui.

## Promoção Vary → Binary (`promote_vary`)

Paridade `process.rs` (vanilla): um item de classe `Vary` (ex.: `+`, `-`)
promove para `Binary` quando o `rclass` do item anterior é `Normal |
Alphabetic | Closing | Fence` — distingue uso como operador binário
(`a+b`) de uso como prefixo unário (`+b` no início da sequência, ou depois
de outra Relação/Abertura).

## Integração em `mod.rs`

- `layout_sequence`: computa `text_space_pt = self.metrics.advance(" ",
  style.size, style).val()` (P903), depois `gaps = compute_gaps(&filtered_nodes,
  style.size.val(), style.math_script, text_space_pt)` (P891 — terceiro
  argumento; P903 — quarto argumento) antes de layoutar os nós; chama
  `hconcat_spaced(boxes, &gaps)` em vez de `hconcat`.
- `hconcat` passa a ser um wrapper de `hconcat_spaced(boxes, &[])` (gaps
  vazios — usado por `delimited.rs` para abertura+corpo+fecho, onde a
  regra de classe já dá 0pt em ambos os lados, logo não há mudança de
  comportamento nesse call site).
- `hconcat_spaced(boxes, gaps)`: insere `x += gaps[i-1]` antes de posicionar
  a box `i` (`i > 0`); `gaps` mais curto que `boxes.len() - 1` trata gaps em
  falta como 0.
- `layout_node`: novo arm `Content::MathClassOverride(e) =>
  self.layout_node(&e.body, style)` — a classe só afecta espaçamento
  (calculado em `layout_sequence` a partir do `Content` bruto, antes da
  conversão para `MathBox`), o layout do body é normal.

## Critérios de verificação

Medido por sonda `mutool trace` (P772y), fonte de verdade `lab/typst-
original`, comparação directa de deltas de posição x entre glifos
adjacentes menos o `adv` do glifo:

- `a = b` (11pt): THICK ≈ 3.056pt de cada lado do `=` — bate exactamente
  com o vanilla (`5/18 × 11 = 3.0556`).
- `a + b` (11pt): `+` promovido a Binary (precedido por `a`, Alphabetic) →
  MEDIUM ≈ 2.444pt de cada lado — bate com vanilla (`2/9 × 11 = 2.4444`).
- `(a)` (11pt): 0pt extra em ambos os lados (Opening/Closing) — bate com
  vanilla.
- `math.class("relation", "z")` produz exactamente o mesmo delta que `=`
  no mesmo contexto (`x <relação> y`), e difere do delta de `z` sem
  override (Alphabetic-Alphabetic, 0pt extra) — confirma que o override
  força a classe e não o espaçamento por omissão do símbolo.
- 26 unit tests em `spacing.rs` (`node_math_class`, `promote_vary`,
  `spacing_between`, `compute_gaps`) + suite `rules::math::layout` (124
  tests) sem regressão.
