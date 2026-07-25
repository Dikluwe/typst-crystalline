# Prompt L0 — `math/layout/frac` — `MathFrac`
Hash do Código: 0200303d

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/frac.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathFrac` — fracções. Consome `fraction_rule_thickness` + `fraction_num_gap` +
`fraction_denom_gap` de `MathConstants`. Baseline x-height via
`MathLayouter::apply_axis_offset` (ver `_comum.md`); test regressão
`frac_com_axis_height_nao_regride` (`tests.rs:520+`).

**Critério**: `MathFrac { num: a, den: b }` → sem `[` nos items.

## P905 — offsets Y usavam convenção "topo do box" em vez de baseline-relativa

**Achado** (`typst-passo-905-relatorio.md`, materialização originalmente descrevia o sintoma
como restrito a `/` em argumentos de chamada de função — Fase A confirmou ser mais geral: **todo**
`/` em modo matemático, incluindo fracções soltas fora de qualquer chamada, ex. `$ a/b $`):
confirmado por PDF real (`pdftoppm`) que a linha de fracção atravessava o denominador "a meio da
altura" sempre que o denominador tinha glifo com ascendente alto (`b`, `y`, ...) — o numerador
parecia normal (só o denominador sobrepunha visivelmente), consistente com o sintoma catalogado em
P899 Parte B ("só o primeiro operando aparece, com um glifo estranho por baixo").

**Causa**: `layout_frac` construía `num_y=0.0` e `den_y`/`rule_local_y` a partir de
`num_box.height()` — assumindo (implicitamente, nunca declarado no código) a convenção "`y=0` =
topo da caixa". A convenção real, já confirmada e documentada em P901 (`root.md` acima) via leitura
de `hconcat_spaced`/`layout_equation` (`mod.rs`) e replicada correctamente em `attach.rs`, é
"`y=0` = **baseline própria** da `MathBox`, `y` cresce para baixo" — `frac.rs` nunca tinha sido
auditado contra essa convenção desde a sua introdução original (Passo 9.8/136-137, muito antes de
P800/P901 a terem confirmado). O único teste de posição pré-existente
(`math_frac_numerador_acima_denominador`) só verificava **ordem** (`num.y < den.y`), que continua
verdadeira mesmo com o bug (`0 < positivo`) — por isso a regressão nunca foi apanhada.

**Correcção**: `rule_local_y = 0.0` (a linha fica exactamente na baseline própria do `MathBox` da
fracção — por construção, `ascent`/`descent` já medem a partir daí); `num_y =
-(num_box.descent + gap + rule_thickness/2)` (a baseline do numerador sobe o suficiente para o seu
descent parar `gap` acima do topo da linha); `den_y = gap + rule_thickness/2 + den_box.ascent` (a
baseline do denominador desce o suficiente para o seu ascent parar `gap` abaixo do fundo da linha).
`ascent`/`descent` do `MathBox` resultante **não mudaram** (fórmula já estava correcta,
independente do bug de offset dos items) — por isso `root.rs`/`layout_stretchy_delimiter` já
recebiam a altura correcta para dimensionar `sqrt(frac(...))`, mas o símbolo `√` não escala
(bbox idêntico a `sqrt(x)` de um único carácter) — **achado novo, separado, fora de âmbito de
P905** (ver relatório, candidato a passo dedicado sobre `layout_stretchy_delimiter`).

Testes de regressão (magnitude do gap, não só ordem):
`p905_frac_numerador_tem_gap_acima_da_linha`,
`p905_frac_denominador_tem_gap_abaixo_da_linha_nao_sobrepoe`,
`p905_sqrt_de_fraccao_com_variaveis_nao_produz_saida_malformada`.
