# Prompt L0 — `rules/math/layout/spacing` — espaçamento automático por `MathClass`
Hash do Código: 907ee000

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/spacing.rs`
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

> **Fonte de paridade**: documentação `https://typst.app/docs/reference/math/class/`
> descreve as classes e o espaçamento resultente; corpus
> `00_nucleo/corpus-docs/math/class.typ:8-50` cobre todos os exemplos de
> classes. O comportamento concreto de espaçamento foi medido contra o
> vanilla ratificado em P772y (ver `typst-passo-772y-relatorio.md`) e está
> travado em `01_core/src/compiler/math/layout/tests.rs:206-224`
> (critérios de verificação) e `:1799`
> (`p825d_mat_align_spacing_de_classe_no_limite`).

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
| `MathLimitsOverride(e)` | **recurse em `e.body`** (P992 — `limits()`/`scripts()` não afectam classe/espaçamento, paridade vanilla `resolve_limits`/`resolve_scripts`: resolvem o `body` normalmente, só sobrescrevem `set_limits`; **diferente** de `MathClassOverride`, que força a classe) |
| `MathDelimited(_)` | `(Opening, Closing)` — sempre assimétrico (paralelo `MathItem::lclass/rclass`, vanilla, caso Fenced) |
| `MathOp(_)` | **`Large`** (P907 Parte B — `min`/`max`/`lim`/`sin`/etc; paridade `resolve_op`, vanilla, `item.set_class(MathClass::Large)` incondicional, independente da flag `limits`) |
| `MathAttach(e)` | **recurse em `e.base`** (P907 Parte B — paridade `ScriptsItem::create`, vanilla, doc "inherits its math class from the base"; necessário para `min_(x)` continuar `Large`) |
| outros (Frac/Root/Matrix/Cases/Accent/Cancel/Underover) | `Normal` (paridade `unwrap_or(MathClass::Normal)`, vanilla — nenhum destes define `class` explícito; `Accent` também herda de `base` no vanilla — `AccentItem::create` tem a mesma doc de `ScriptsItem` — mas fora do achado confirmado por P907, que só cobriu o caso concreto de `min`/`max`; candidato a passo dedicado, ver secção P907 abaixo) |

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

> **Nota de verificação**: mecanismo interno de layout; o observável é o gap nulo em
> subscritos/sobrescritos. Guarda em
> `01_core/src/compiler/math/layout/tests.rs:2939-3145`
> (`p914/p915_tests`, sequências em `math_script` não produzem espaços de classe).

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

> **Nota de verificação**: mecanismo interno de layout. O observável é o espaço extra
> em torno de texto literal em math; guardas em
> `01_core/src/compiler/math/layout/tests.rs:1858-1867`
> (`p895_hspace_em_sequencia_math_contribui_largura`) e `:6720-6750`
> (`p903_*` / `align_boundary_spacing`) cobrem o comportamento com valores
> controlados.

**Fora de escopo, ainda por implementar (registado, P772y/P825 — casos
`Content::Text`/`Fence`/`MathOp` resolvidos por P903/P907)**:
- A regra "spaced frames" genérica (`#h()` explícito dentro de math) —
  sem equivalente no cristalino hoje.
- **`class("normal", ...)` "spaced"**: `$ a #math.class("normal", "+") b $`
  — o vanilla marca o átomo wrapped como spaced (gaps ≈ 3.65pt); o
  cristalino aplica a tabela com a classe overridden (Normal-Normal →
  gap 0). Divergência de língua conhecida e registada, não bug a
  corrigir nesta linha de trabalho.
- **`Accent`/`Cancel`/`Underover` não herdam classe do `base`** — achado
  incidental de P907 (Parte B), não corrigido: `AccentItem::create` do
  vanilla tem a MESMA doc de `ScriptsItem::create` ("inherits its math
  class from the base"), mas `Content::MathAccent`/`MathCancel`/
  `MathUnderover` continuam `Normal` em `base_math_class` — só
  `MathAttach` foi corrigido (era o caso confirmado, `min_(x)`).
  Candidato a passo dedicado.

## P907 — dois achados de espaçamento registados em P825/P903

**Parte A — `|` como fence** (`typst-passo-825-relatorio.md`, reconfirmado em P903): `$ a | b $` tem
gap ≈ 3.65pt dos dois lados de `|` no vanilla; o cristalino dava ≈ 0 (tabela de classes,
`(Alphabetic, Fence)`/`(Fence, Alphabetic)` não têm regra explícita, e `|` não é `Content::Text`,
logo o fallback "spaced" de P903 não se aplicava). Causa confirmada por leitura do vanilla real
(`math/ir/item.rs::MathItem::is_spaced`): `class() == Fence` é **sempre** "spaced",
incondicionalmente — mesmo mecanismo do fallback de P903 para `Content::Text`, só o gatilho muda
(classe, não tipo de nó). Corrigido generalizando o fallback de `compute_gaps` (antes `prev_is_text
|| is_text`, agora `prev_is_spaced || is_spaced`, onde `is_spaced = matches!(node, Content::Text(_))
|| raw_l == MathClass::Fence`). `abs(x)`/`Content::MathDelimited` não afectados (Opening/Closing,
regra explícita, não passa pelo fallback).

**Parte B — `min_(x) f(x)` sem espaço** (achado incidental de P903): confirmado em duas causas
encadeadas, ambas por leitura directa do vanilla real (`math/ir/resolve.rs`/`item.rs`, não inferidas):
1. `Content::MathOp` (`min`/`max`/`lim`/`sin`/etc.) não tinha braço em `base_math_class` — caía no
   catch-all `Normal`. Vanilla (`resolve_op`) marca SEMPRE `MathClass::Large`, mesmo para operadores
   sem `limits` (`sin`, `cos`) — a flag só afecta `Limits::Display`/`Never`, não a classe.
2. `Content::MathAttach` também não tinha braço — caía em `Normal`, mascarando a correcção #1 no
   caso COM subscrito (`min_(x)`). Vanilla (`ScriptsItem::create`) herda a classe do `base`
   explicitamente (doc do próprio código-fonte). Corrigido com `Content::MathAttach(e) =>
   base_math_class(&e.base)` — recursivo, cobre também `x^2`/`sum_(i=1)^n` (não só `min`/`max`).

Ambas as partes testadas com TDD directo (sem protocolo de dois agentes — mudança de regra de
espaçamento, mesma categoria de risco baixo de P903), confirmadas visualmente contra o vanilla real
(`pdftotext -bbox`, posições x coincidentes a <0.1pt).

## P1135 — matriz delimitada expõe classes Opening/Closing nas bordas

**Medição** (2026-08-23, working tree não commitado sobre HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`; `.typ/sec_05.typ`, vanilla
ratificado `a51e02804`): em `$ det mat(1, 2; 3, 4) $`, o cristalino insere
1,8333pt entre o fim de `det` e o delimitador `(`; o vanilla insere 0pt.
Como a fórmula é centrada, a largura extra desloca `det` 0,9167pt para a
esquerda e a matriz 0,9167pt para a direita. Todas as demais divergências
da secção 5 ficam dentro desse bloco.

**Causa medida antes da decisão**: `MathOp("det")` tem classe `Large`.
`spacing_between_class(Large, Opening)` já devolve 0, como o vanilla, mas
`node_math_class(MathMatrix)` caía no catch-all `Normal`, activando
`Large–Normal = THIN = 1/6em`. No vanilla, `resolve_mat` chama
`resolve_delimiters` (`math/ir/resolve.rs:1029-1077,1165-1186`) e produz um
`FencedItem`; `MathItem::lclass/rclass` (`math/ir/item.rs:122-148`) expõe
`Opening` quando existe delimitador esquerdo e `Closing` quando existe
delimitador direito, mantendo `Normal` somente na borda sem delimitador.

**Decisão**: `node_math_class(Content::MathMatrix(e))` devolve classes de
borda derivadas exclusivamente de `e.delim`: `Opening` à esquerda quando
`delim.0 != '\0'`, `Closing` à direita quando `delim.1 != '\0'`, e
`Normal` em cada borda ausente. Não há coordenada nem constante de fixture.
Matrizes sem delimitadores continuam `Normal–Normal`; a tabela geral de
espaçamento permanece inalterada.

**Critério de língua**: `det mat(...)` tem gap nulo e posição idêntica ao
vanilla; uma matriz delimitada seguida de operador respeita a classe
`Closing`; `mat(delim: none)` preserva classe `Normal` nas duas bordas.

## P1132h — classe do símbolo nomeado e ausência de regra Unary

`base_math_class` classifica `MathIdent` pelo primeiro carácter do glifo
resolvido por `ident_to_unicode`, quando o nome pertence à tabela de símbolos;
só identificadores não resolvidos usam o primeiro carácter textual do nome.
Classificar `nabla` pela letra `n` seria incorreto; U+2207 tem classe Unicode
`Unary`. Além disso, a tabela de `spacing()` do vanilla ratificado não possui
braços específicos para `Unary`. O cristalino remove os braços locais
`(_, Unary)`/`(Unary, _)`: assim, em `nabla times B`, a regra posterior do
operador `Binary` prevalece e os dois gaps valem `2/9 em`. `dif S` continua
sem gap depois de `dif`, pois o par `Unary–Alphabetic` cai naturalmente no
catch-all. Nenhum valor medido da secção 11 entra na produção.

> **Fonte de paridade**: documentação `https://typst.app/docs/reference/math/op/`
> lista `lim`, `max`, `min`, etc. como operadores predefinidos (corpus
> `00_nucleo/corpus-docs/math/op.typ:13-17`); guardas em
> `01_core/src/compiler/math/layout/tests.rs:7765`
> (`p992_scripts_nao_muda_classe_do_body`) e `:5114-5610`
> (`p952_tests`, cobre `MathClass::Large` em operadores).

## Promoção Vary → Binary (`promote_vary`)

Paridade `process.rs` (vanilla): um item de classe `Vary` (ex.: `+`, `-`)
promove para `Binary` quando o `rclass` do item anterior é `Normal |
Alphabetic | Closing | Fence` — distingue uso como operador binário
(`a+b`) de uso como prefixo unário (`+b` no início da sequência, ou depois
de outra Relação/Abertura).

## Integração em `mod.rs`

### P1132i — `|` pareado é módulo; `|` solitário é separador

Antes de calcular as fronteiras, `compute_gaps` identifica pipes matemáticos
bare no mesmo nível da sequência. Quando formam pares, alterna suas classes
efetivas entre `Opening` e `Closing`; assim `|x|` não recebe espaço interno.
Um pipe sem par conserva `Fence` e `is_spaced=true`, necessário ao separador
de conjunto `{x | condição}`. A decisão deriva da estrutura da sequência,
não da largura ou posição de uma fixture. Critérios: `|x|` produz gaps zero;
`x | y` usa a largura real do espaço textual dos dois lados; dois módulos
consecutivos reiniciam corretamente a alternância.

### P1132u — classes laterais do módulo pareado

**Medição antes da decisão** (secção 28, working tree não commitado,
2026-08-22): em `2 |E(G)|`, o vanilla conserva antes da primeira barra a
largura real do espaço textual; o cristalino fundia `2|E(G)|`. A barra de
abertura do módulo deve ter `lclass=Fence` (lado externo, item `spaced`) e
`rclass=Opening` (lado interno, zero). Simetricamente, a barra de fecho usa
`lclass=Closing` e `rclass=Fence`. Assim o interior de `|E(G)|` continua sem
espaços e as fronteiras externas recebem `text_space_pt`. A regra deriva da
assimetria lateral já prevista por `node_math_class`, sem coordenadas ou
constantes da fixture.

**Limite medido pela regressão da secção 25:** em tamanho Script, como no
limite `|z|=R` de `integral.cont_(...)`, a face `Fence` externa do módulo
pareado não aciona `is_spaced`; o vanilla mantém `|z|=R` compacto. A supressão
usa `in_script`, já derivado do `MathSize` ativo. Pipes solitários conservam a
semântica de separador; este limite aplica-se apenas às faces externas criadas
pelo reconhecimento do par.

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

## P1132o — supressão por `MathSize` em frações aninhadas

**Medição antes da decisão** (secção 18, working tree não commitado,
2026-08-22): em `1/(1+1/(1+1/(1+x)))`, o vanilla mantém o espaço de classe
ao redor do primeiro `+` em tamanho Text, mas suprime-o nos níveis Script
(7.7pt) e ScriptScript (5.5pt). O cristalino mantinha os espaços nos três
níveis. As baselines e os tamanhos das fontes já coincidem.

O L0 anterior dizia que o cristalino não possuía `MathSize` discreto; isso
deixou de ser verdade desde P945. `layout_sequence` deve considerar a
sequência em script quando `style.math_script` for verdadeiro **ou** quando
`style.math_size` for `Script`/`ScriptScript`. A decisão reproduz
`process.rs::spacing()` (`size <= MathSize::Script`) e deriva do estilo
tipográfico corrente, não da profundidade da expressão nem de coordenadas do
PDF. O nível Text continua aplicando normalmente a tabela de classes.

## P1132s — itens textuais espaçados dentro de scripts

**Medição antes da decisão** (secção 25, working tree não commitado,
2026-08-22): no limite inferior `p "prime"`, o vanilla conserva entre `p`
e o texto literal a largura real de um espaço da fonte no tamanho do script,
enquanto o cristalino funde os dois itens. A fonte ratificada confirma a
ordem: em `math/ir/process.rs::spacing`, as condições de tamanho Script
guardam os braços de Pontuação/Relação/Binário; o fallback posterior
`is_spaced()` não tem esse guard.

Portanto, `compute_gaps` não retorna zeros antecipadamente em scripts. Ele
continua a consultar a tabela na ordem vanilla: os espaços condicionais de
classe são suprimidos quando o lado relevante está em Script, enquanto o
fallback de item `Content::Text`/Fence conserva `text_space_pt`. Essa largura
vem de `FontMetrics::advance(" ", ...)` no tamanho ativo; nenhuma medida da
fixture entra no código.

Na mesma medição, `dif z =` expôs uma segunda falha estrutural: ao encontrar
um `HSpace`, `compute_gaps` reiniciava a classe anterior, mas deixava de emitir
a fronteira `HSpace→próximo`. O vetor ficava uma posição mais curto e o espaço
grosso pertencente a `z→=` era aplicado em `d→z`; o espaço posterior também
se deslocava. O vetor deve possuir exatamente `nodes.len() - 1` entradas.
Toda fronteira tocada por `HSpace` vale zero no vetor (a largura do próprio nó
já é dinâmica); o reinício de classes permanece. Isso reproduz a morfologia
do `HElem` ignorante do vanilla sem qualquer ajuste de coordenada.

Quando `Content::Text` possui um sucessor `MathDelimited`, a concatenação
aperta a sua caixa retirando o avanço de `" "` do mesmo estilo, saturado em
zero; nas outras fronteiras e quando terminal conserva a largura completa. O espaço
semântico é acrescentado uma única vez por `compute_gaps`; por isso
`"Res"(f)` encosta à abertura e `p "prime"` conserva um espaço e o centro sob
`∏`. A escolha é estrutural e usa métricas ativas, não o conteúdo `"Res"`.
