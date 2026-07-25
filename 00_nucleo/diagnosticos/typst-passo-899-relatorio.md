# Relatório — Passo 899: funções matemáticas nativas em falta

**Data:** 2026-07-24/25
**Commit de partida:** `dd11a52a6` (P898)
**Commits deste passo:** `d7e28d8c5` (Parte E), `01bdbb91a` (Parte B), `6c92ea3d7` (Parte D),
`2310b6412` (Parte A) — ordem de implementação real (E→B→D→A), diferente da ordem do catálogo
(A-E), por ter começado pela parte que a Fase A revelou ser mais simples de confirmar.

---

## Resumo por parte

| Parte | Estado | Mecanismo |
|---|---|---|
| A — acentos (`hat`/`tilde`/`dot`/`dot.double`) | ✅ Implementada | Novos braços hardcoded → `Content::math_accent` (reaproveita layout de Passo 296) |
| B — delimitadores (`abs`/`norm`/`floor`/`ceil`/`round`, + `bar`) | ✅ Implementada | Novos braços hardcoded → `Content::math_delimited` (reaproveita layout existente) |
| C — chaves grandes (`underbrace`/`overbrace`/`underbracket`/`overbracket`) | ❌ **Adiada** | Precisa de mecanismo de esticamento HORIZONTAL de glifo, inexistente em cristalino |
| D — `binom` | ✅ Implementada | Novo braço hardcoded → `Content::math_matrix` (reaproveita mecanismo de `vec`/`cases`/`mat`) |
| E — estilos de fonte (`bb`/`cal`/`frak`/`mono`/`sans`) | ✅ Implementada | 2 bugs em cadeia corrigidos (ver secção própria) |

---

## Fase A — metodologia e achados centrais

A investigação usou 3 fontes, por ordem de confiabilidade crescente para "o que é que a função X
realmente faz": (1) leitura do código cristalino, (2) leitura do código-fonte vanilla em
`lab/typst-original/`, (3) **compilação directa contra o binário vanilla real**
(`lab/typst-original/target/release/typst`, já compilado, ground truth incontestável) — usado
sempre que a leitura do código-fonte deixou dúvida sobre o comportamento observável.

### Achado 1 — `bar(x)` NÃO é uma função de acento (corrige o catálogo do próprio passo)

A materialização listava `bar(x)` em "Parte A — funções de acento". Compilação directa contra o
vanilla real:

```
$ bar(x) $   →  |x|      (delimitador)
$ hat(x) $   →  x̂        (acento, confirma catálogo)
$ tilde(x) $ →  x̃        (acento, confirma catálogo)
$ dot(x) $   →  ẋ        (acento, confirma catálogo)
```

Causa: no vanilla, `sym.bar` resolve para o carácter `|`; **chamar** um símbolo em modo math
invoca `Symbol::func()` (`foundations/symbol.rs`), que tenta primeiro `Accent::combining(valor)`
(tabela de acentos) e, falhando essa, `get_lr_wrapper_func(valor)` (tabela de delimitadores L/R,
`math/lr.rs`) — `|` está na segunda tabela, não na primeira. Implementado em Parte B, não Parte A.

### Achado 2 — mecanismo de despacho do vanilla vs cristalino

Vanilla usa despacho **genérico em runtime**: qualquer símbolo, quando chamado como função, tenta
resolver-se como acento OU como delimitador L/R através de tabelas fixas (`ACCENTS`, `DELIMS`).
Cristalino despacha `Expr::FuncCall` em modo math por um `match name.as_str()` **hardcoded**
(`eval/math.rs`, já usado por `frac`/`sqrt`/`root`/`vec`/`cases`/`mat` antes deste passo) — não há
mecanismo de "símbolo chamável" genérico. Replicar o genérico exigiria portar as duas tabelas
completas e um novo tipo de despacho — fora de âmbito. Todas as novas partes (A, B, D) seguem o
padrão hardcoded já estabelecido, mapeando cada nome directamente para o comportamento certo (char
de acento, par de delimitadores, ou construção de matriz), sem tentar replicar a generalidade do
vanilla.

### Achado 3 — categorização mapeamento-de-tabela vs layout-próprio (Fase A ponto 4)

- **Mapeamento de tabela** (reaproveita layout já existente, sem código novo de geometria): Partes
  A, B, D. Confirmado por leitura do layout já existente antes de assumir necessidade de trabalho
  novo — `layout_accent`/`layout_matrix` já existem e funcionam desde P296/anteriores.
- **Layout próprio necessário**: Parte C — ver secção dedicada abaixo.
- **2 bugs de despacho, não 1 função em falta**: Parte E — ver secção dedicada.

---

## Parte A — `hat`/`tilde`/`dot`/`dot.double`

**Mecanismo**: novo braço `"hat" | "tilde" | "dot" | "dot.double"` em `eval_math_expr`
(`Expr::FuncCall`), mapeando directamente para o combining-mark (mesmos valores de
`lab/typst-original/crates/typst-library/src/math/accent.rs::ACCENTS`: `hat`→U+0302, `tilde`→U+0303,
`dot`→U+0307, `dot.double`→U+0308), chamando `Content::math_accent(base, Content::MathText(char))`
— reaproveita `layout_accent` (Passo 296) sem alteração.

`dot.double(x)`: o callee é `FieldAccess(MathIdent("dot"), "double")`, não `MathIdent` bare — sem
tratamento especial, cairia no despacho namespaced genérico (`other_callee`), que tenta resolver
"dot" como símbolo e aplicar o modificador "double" (`Symbol::modified`), falhando com "unknown
symbol modifier 'double'" (mesma classe de confusão símbolo-vs-função já vista em P895 para
`sect`/`inter`). Adicionado um braço explícito, antes do genérico, que detecta este padrão exacto e
trata como `name = "dot.double"`.

**Achado incidental, pré-existente, não corrigido**: a base de um `MathAccent` renderiza em
"roman" upright, não itálico — confirmado já presente em `accent(x, "̂")` chamado directamente
(antes deste passo), não introduzido por P899.

**Testes**: `p899_hat_tilde_dot_dot_double_produzem_mathaccent` (4 casos, comparados contra os
combining-marks exactos da tabela vanilla). Vermelho→verde confirmado.

**Confirmação visual**: `hat(x)`→x̂, `tilde(x)`→x̃, `dot(x)`→ẋ, `dot.double(x)`→ẍ,
todos com o acento visivelmente correcto sobre a base — comparado directamente com o render do
binário vanilla real.

---

## Parte B — `abs`/`norm`/`floor`/`ceil`/`round`/`bar`

**Mecanismo**: novo braço `"abs" | "norm" | "floor" | "ceil" | "round" | "bar"`, escolhe o par
`(open, close)` por nome (`abs`/`bar`→`|`/`|`, `norm`→`‖`/`‖`, `floor`→`⌊`/`⌋`, `ceil`→`⌈`/`⌉`,
`round`→`⌊`/`⌉` — par assimétrico, confirmado contra o vanilla real) e chama
`Content::math_delimited(open, body, close)` — mesmo construtor já usado por `(x)`/`[x]` literais.

**Scope-out**: sem suporte ao named arg `size:` — `Content::math_delimited` não tem campo para
override manual do tamanho; o stretch automático (se existir) continua inalterado.

**Testes**: `p899_abs_norm_floor_ceil_round_produzem_mathdelimited` (6 casos, incluindo `bar`).
Vermelho→verde confirmado.

**Confirmação visual**: `|x|`, `‖x‖`, `⌊x⌋`, `⌈x⌉`, `⌊x⌉` todos correctos e com stretch vertical
funcional para conteúdo mais alto (confirmado com `floor(x/y)` — nota: a fracção em si não
renderizou devido ao achado incidental abaixo, mas o delimitador em torno do que renderizou
esticou correctamente).

**Achado incidental, registado, não investigado** (não introduzido por este passo): um argumento
contendo `/` dentro de QUALQUER chamada de função em modo math (`sqrt(x/y)`, `abs(x/y)`, …) produz
saída malformada (só o primeiro operando aparece, com um glifo estranho por baixo). Reproduzido com
`sqrt(x/y)` — função pré-existente, não tocada por este passo — confirma que não é regressão.
Candidato a passo dedicado futuro.

---

## Parte C — `underbrace`/`overbrace`/`underbracket`/`overbracket` (ADIADA)

**Veredicto da Fase A**: precisa de mecanismo de layout **novo**, não reaproveitamento — decisão
tomada seguindo a cláusula explícita da própria materialização ("se alguma parte revelar-se muito
maior que o esperado, registar e considerar destacar para um passo separado").

**Investigação**: cristalino já tem `Content::MathUnderover`/`layout_underover`
(`engine/math/layout/mod.rs:588-634`, Passo 297) — mas este handler **centra** `under`/`over` no
seu tamanho NATURAL, sem esticar horizontalmente para cobrir a largura da base. Confirmado por
leitura directa do código (`let dx = (w - ob.width) / 2.0` — centragem simples, sem `Stretch`).
Comparado com o vanilla real: `hat(a+b)` mostra um circunflexo LARGO cobrindo "a+b" inteiro — o
vanilla estica activamente o glifo do acento/chave (`math/ir/resolve.rs`, mecanismo `Stretch`/
`StretchInfo`, usado tanto para acentos largos como para `underbrace`/`overbrace`). Cristalino não
tem NENHUM mecanismo de esticamento **horizontal** de glifo (só tem `vertical_glyph_assembly` no
trait `FontMetrics`, usado para delimitadores altos como `(`/`[`/`{`).

Para acentos de base curta (1 carácter, o caso comum testado nas Partes A), a ausência de
esticamento é uma limitação cosmética menor (já aceite, documentada no próprio código como decisão
ADR-0054 graded). Para `underbrace`/`overbrace`, a ausência de esticamento seria muito mais grave:
a chave/colchete não teria relação visual nenhuma com a largura do conteúdo que supostamente
agrupa — um glifo pequeno e fixo flutuando acima/abaixo de uma expressão de largura arbitrária, sem
parecer relacionado com ela. Implementar sem o esticamento produziria algo pior do que o actual
fallback de texto literal (que pelo menos é honesto sobre não estar implementado) — decisão de não
o fazer, registada aqui em vez de silenciosa.

**Não implementado.** Candidato a passo dedicado futuro que construa o mecanismo de esticamento
horizontal de glifo (provavelmente um novo método no trait `FontMetrics`, paralelo a
`vertical_glyph_assembly`, mais lógica em `layout_underover` para o usar quando disponível).

---

## Parte D — `binom`

**Mecanismo**: no vanilla, `BinomElem` é resolvido (`ir/resolve.rs::resolve_binom`) pela MESMA
função que resolve `frac()` (`resolve_vertical_frac_like`), com um `bool` que suprime a barra — um
binomial é uma fracção sem barra, envolvida em parênteses esticados. Cristalino não tem modo "sem
barra" para `Content::math_frac`; em vez de adicionar esse campo, reaproveita-se
`Content::math_matrix` (2 linhas: `[upper]`, `[lower]` — já produz "pilha vertical sem barra entre
linhas, envolvida em delimitadores", mesmo mecanismo de `vec`/`cases`/`mat`). Args de `lower`
(variádico, mínimo 1) juntam-se numa única célula por `", "`, paridade vanilla (`SymbolElem::packed(',')`
entre elementos de `denom`).

**Testes**: `p899_binom_produz_mathmatrix_2_linhas_parenteses` +
`p899_binom_variadico_junta_lower_por_virgula_numa_celula`. Vermelho→verde confirmado.

**Confirmação visual**: `binom(n, k)` → n sobre k entre parênteses; `binom(n, k_1, k_2, k_3)` → n
sobre "k₁, k₂, k₃" entre parênteses. Qualidade do esticamento dos parênteses igual à já existente
para `vec`/`cases`/`mat` (não pixel-perfeito vs vanilla, mas sem regressão — mesma limitação
partilhada, fora de âmbito deste passo específico).

---

## Parte E — `bb`/`cal`/`frak`/`mono`/`sans` (e `serif`/`bold`/`italic`/`upright`/`scr`/etc., mesmo mecanismo)

**Achado inicial**: a alegação da materialização ("compilam sem erro mas devolvem o conteúdo sem
nenhum estilo aplicado") **não correspondia ao estado actual** para argumentos identificador
(`bb(R)` já funcionava, produzindo ℝ) — só reproduzível com **argumento string literal**
(`bb("R")`, forma usada na secção 16 do `.typ` de 30 secções) — devolvia `"R"` sem estilo. Medido
antes de aceitar a alegação da materialização (ADR-0108).

**2 bugs em cadeia**:

1. `wrap_math_style` (`stdlib/math_style.rs`) envolvia `Value::Str` em `Content::text(s)` (prosa) —
   fora do alcance de `apply_math_style` (só cobre `MathIdent`/`MathText`/`MathSequence`/
   `MathMatrix`). Corrigido: `Content::MathText(s)`.
2. **Mais fundo** (só descoberto ao verificar o fix #1 via CLI real, não só via teste unitário
   directo de `wrap_math_style`): o despacho P510 de chamadas bare a funções do scope global em modo
   math (`bb(x)`, `bold(x+y)`, mecanismo que serve TODAS as ~14 funções `math_style`, não só as 5
   catalogadas) forçava **todos** os argumentos por `eval_math_expr` + `Value::Content(...)` antes de
   chegarem à função nativa — um literal string nunca chegava como `Value::Str`, passava sempre por
   `Content::Text` (via `value_to_display_content`, partilhado e correcto para outros usos, não
   alterado). Isto **reverte uma restrição deliberada de P772y** ("`eval_math_arg_value` usado só no
   caminho namespaced, não no P510 bare-ident, para não arriscar regressão em `bb`/`bold`/etc.") — o
   próprio risco que a restrição queria evitar já era o bug real. Corrigido unificando os dois
   caminhos de avaliação de argumentos (P510 e P772y namespaced) via `eval_math_arg_value`.

**Regressão da suíte antes de aceitar como fechado**: 1 teste falhou após a correcção #2
(`p311b_accepts_string_body`, que consagrava deliberadamente o comportamento antigo/errado).
Marcado `#[ignore]` com razão explícita, mesmo padrão já usado no código adjacente para
`p311b_empty_args_produces_empty_body`/P811. Substituído por
`p899_bb_com_argumento_string_produz_mathtext_nao_text` (unitário) +
`p899_bb_de_string_via_pipeline_real_produz_mathtext` (pipeline completo, o teste que realmente
capturou o bug #2 — o unitário directo só cobria metade do caminho real).

**Testes**: 2 novos (unitário + pipeline real), 1 revogado com razão registada.

**Confirmação visual**: `bb("R")`→ℝ, `cal("L")`→𝓛, `frak("G")`→𝔊, `mono("code")`→𝚌𝚘𝚍𝚎,
`sans("sans")`→𝗌𝖺𝗇𝗌 — todos correctos, incluindo strings multi-carácter (antes só testado com 1
carácter).

---

## Suíte completa (após todas as 4 partes implementadas)

```
typst-core:    4732 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões em todo o percurso (verificado incrementalmente após cada parte, per o pedido da
materialização — cada parte com o próprio commit isolável).

## `crystalline-lint`

0 drift após cada parte (`--fix-hashes .`). Único warning restante:
`prompts/infra/package_version_resolution.md` [V7], pré-existente, não relacionado.

## Recompilação do `.typ` de 30 secções

Hash confirmado igual aos passos anteriores antes de recompilar:
`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`. Recompila sem crash
(`exit=0`), `MediaBox` finito, sequência `(1)`...`(44)` completa. Secção 10 (acentos) e secção 16
(estilos) recompiladas isoladamente e confirmadas visualmente sem regressão — `underbrace`/
`overbrace`/`underbracket`/`overbracket` continuam (correctamente, Parte C adiada) como fallback de
texto literal, comportamento inalterado. Secção 22 (delimitadores escaláveis, usa `lr(...)` com
fracções) não foi tocada por este passo — atinge o mesmo achado incidental já registado (bug de `/`
em argumentos de chamada), pré-existente.

## Benchmark (Fase C, uma vez no fim, per instrução da materialização)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Nenhum dos 7 `.typ` de benchmark usa qualquer função
tocada por este passo (confirmado por grep antes de interpretar os números). Leituras ligeiramente
elevadas (~2-4%) em TODOS os 7 cenários, incluindo `01-hello` (controlo, sem qualquer relação com
matemática) — remedição isolada (30 amostras) confirma o controlo na mesma elevação, mesmo padrão
de ruído ambiental já demonstrado em P896-898. Sem regressão real.

## Fora de âmbito (registado, não corrigido/investigado)

1. **Parte C** — mecanismo de esticamento horizontal de glifo, candidato a passo dedicado.
2. **Bug de `/` em argumentos de chamada de função em modo math** — pré-existente (`sqrt(x/y)`
   confirmado como reproduzindo o mesmo sintoma sem qualquer alteração deste passo).
3. **Base de `MathAccent` não itálica** — pré-existente desde Passo 296, achado incidental de Parte A.
4. **`vec(x)` com 1 argumento renderiza de forma estranha** (achado incidental durante a confirmação
   visual da secção 10, não investigado — `layout_matrix` não foi tocado por este passo).
5. P893 (`FontMetrics::math_constants`) continua parado em Fase A.
