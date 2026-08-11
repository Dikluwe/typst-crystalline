# Relatório — Passo 992: `limits()`/`scripts()` reconhecidas no eval math

**Estado do código das medições**: HEAD `b59db7f16` (P991) + alterações
deste passo. Commit final no fim.
**Gate ADR-0127**: **parou** para confirmação do dono — mudança de
contrato público confirmada na Fase A (novo elemento `Content::
MathLimitsOverride`, campo novo em `Content`, enum fechado ADR-0026).
Dono escolheu (AskUserQuestion): dividir o escopo — `limits()`/`scripts()`
neste passo, `attach()` de 6 cantos (`t`/`b` novos em `MathAttachElem`)
para passo próprio, separado.

## Fase A — investigação refuta a hipótese optimista do próprio passo

O passo especulava que o mecanismo de 6 cantos ou a flag `is_limits`
pudessem já existir prontos em `Content::MathAttach`, faltando só
dispatch de eval (classe de bug P958/962/981). Confirmado por leitura
directa do código E do vanilla real (`lab/typst-original/crates/
typst-library/src/math/attach.rs`):

- **`attach()` (6 cantos)**: vanilla `AttachElem` tem `base, t, b, tl, bl,
  tr, br` — 6 campos independentes. `MathAttachElem` cristalino só tem
  `base, tl, bl, sub, sup` (sub≈br, sup≈tr) — **`t`/`b` não existem**.
  Confirma a hipótese pessimista do passo: escopo maior, precisa de novo
  campo em `MathAttachElem` — mudança de contrato.
- **`limits()`/`scripts()`**: a flag `limits` já existe, mas em
  `MathOpElem`, não em `MathAttachElem` — e reaproveitar `MathOp`
  directamente é **inseguro** para um base arbitrário: `apply_math_default`
  ignora deliberadamente `Content::MathOp` (não recursa — "operadores como
  sin/lim mantêm aparência normal"), o que apagaria o itálico por defeito
  de uma base como `A`; e `base_math_class` força `MathOp` sempre a
  `Large` (espaçamento errado para uma base não-operador). Vanilla
  confirma: `ScriptsElem`/`LimitsElem` são elementos **separados** de
  `OpElem`, resolvidos por `resolve_scripts`/`resolve_limits` (`ir/
  resolve.rs:612-634`) que **preservam** a resolução normal do `body` e só
  sobrescrevem `set_limits(...)` no item já resolvido. Precisa de um
  elemento novo — também mudança de contrato.

Ambos os caminhos exigem crescer o `Content` (enum fechado, ADR-0026) ou
uma struct existente — o gate ADR-0127 aplicou-se correctamente.

## Fase B — desenho e implementação

**Desenho**: um só elemento novo consolida os dois separados do vanilla
(ADR-0107 — paridade é com a língua, não a mecânica, mesmo padrão de
`binom()` reaproveitando `math_matrix`): `Content::MathLimitsOverride(Arc<
MathLimitsOverrideElem { body, limits: bool, inline: bool }>)`. `scripts
(body)` → `limits: false`; `limits(body, inline:)` → `limits: true`.
L0s redigidos primeiro (novo `entities/elements/math_limits_override.md`
+ secções §P992 em `content.md`, `_comum.md`, `attach.md`, `spacing.md`,
`eval.md`, `introspect.md`, `introspect/locatable.md`,
`stdlib/foundations.md`, `layout.md`, `infra/query-helpers.md`).

**Fórmula crítica em `layout_attach`** (`attach.md` §P992): o override tem
de ser verificado **antes** do `self.block &&` exterior — `limits(body,
inline: true)` (default vanilla) empilha mesmo em modo inline, ao
contrário de `Content::MathOp{limits,..}` (continua gated por
`self.block`). `is_limits = e.limits && (e.inline || self.block)` cobre
os 3 casos (`scripts`=nunca; `limits` default=sempre; `limits(inline:
false)`=só em bloco).

**Touch points mecânicos** (mesmo padrão de `MathClassOverride`, "V trata
todas as camadas" — 9 sítios em `content.rs` + `layout_node`/
`base_math_class`/`materialize_time`/`walk`×2/`repr_content`/
`layout_math_fallback`/`query_helpers.rs`×2): todos os matches exaustivos
tocados, confirmados pelo próprio compilador (`E0004` por cada site em
falta) — sem `_ =>` a esconder omissões. **Diferença face a
`MathClassOverride`**: `apply_math_default` ganha braço recursivo (gap
que `MathClassOverride` tem e fica fora de escopo aqui) — necessário
porque o caso de uso do achado é uma base de 1 letra (`limits(A)`).

**RED encontrado durante TDD** (bug real, não hipotético): `true`/`false`
**sem `#`** em modo math não lexam como `SyntaxKind::Bool` — só
`engine/lexer/code.rs::keyword` reconhece o token; a lexagem de math nunca
o chama. `limits(A, inline: false)` (sintaxe vanilla real, sem `#`) fazia
`inline` ficar preso em `true` silenciosamente (`Expr::MathIdent("false")`
cai no catch-all de `eval_math_arg_value`, falha a resolver como símbolo,
`Err` descartado pelo `if let Ok(Value::Bool(b))`). Corrigido com caso
especial (mesmo padrão de `parse_delim_val` para `delim: "["`). Documentado
em `eval.md` §P992 como gotcha para futuros named args booleanos em
chamadas bare de math.

**GREEN**: 15 testes novos — 4 unitários em `math_limits_override.rs`
(`plain_text_delega_no_body`, `igualdade_estrutural`,
`map_content_recurse_preserva_limits_e_inline`,
`map_text_terminal_preserva_campos`, mesmo padrão de
`math_class_override.rs`); 7 `p992_tests` em `math/layout/tests.rs` —
incluindo `p992_limits_com_inline_true_empilha_mesmo_em_modo_inline`, o
diferenciador crucial vs `MathOp`; 4 `tests_p992` em `eval/tests.rs`, via
pipeline completo `MockWorld`/`extract_math_content`, incluindo o achado
do bug de `inline: false`. Suíte completa, discriminada por crate:
**5812 testes verdes** (4945 `typst-core` + 787 `typst-infra` + 41
`typst-shell` + 37+2 `typst-wiring`), 0 falhas — reconciliação exacta
com a base de P991 (`5797 + 15 = 5812`; `4930 + 15 = 4945`).
**Nota de correcção (2026-08-11)**: a primeira versão deste relatório e
a mensagem do commit de P992 diziam "11 testes novos" — contagem que só
incluía os dois módulos `#[cfg(test)]` desenhados deliberadamente
(`p992_tests`/`tests_p992`), esquecendo os 4 testes unitários do
ficheiro de entidade novo. O total de testes verdes (5812) sempre esteve
correcto; só a etiqueta "11" estava errada, por exactamente os mesmos 4
que fecham a reconciliação acima. `crystalline-lint .`: 0 violations (só
V7 órfão pré-existente, não relacionado).

## Fase C — Revalidação

**Recompilação dos 4 casos do achado** (secção 32 de
`typst-math-extended-test.typ`, binário release actual):
- `attach(A, t:, b:, tl:, tr:, bl:, br:)` e `attach(sum, t:, b:)`:
  continuam texto literal (`attach(𝐴)`, `attach(∑)`) — **scope-out
  esperado e confirmado**, não regride nem finge estar implementado.
- `limits(A)^alpha_beta`: medido via `pdftotext -bbox` — α (y=330.7-338.4),
  𝐴 (y=338.2-349.2), β (y=348.1-355.8), todos em x≈491-499 (~8pt) —
  empilhados verticalmente, confirmando `is_limits=true`.
- `scripts(A)^alpha_beta`: 𝐴 em x=488.2-496.4; α/β ambos deslocados para
  x=496.4-502.1 (à direita) — scripts laterais, confirmando
  `is_limits=false` mesmo sem a base ser naturalmente empilhável (guarda
  de não-regressão visual).
- Confirmação visual a 1200dpi (`pdftoppm`+`convert`): crop da região
  mostra claramente α/β empilhados sobre "A" no caso `limits()`, e
  α/β laterais no caso `scripts()` — inequívoco.

Benchmark canónico (`benchmark-p992-canonical.py`, antes = release
`b59db7f16`): 7 cenários, ratio médio **0.999** (01-hello 1.013, 02-lorem
1.011, 03-images 1.017, 04-math 0.950, 05-tables 0.986, 06-long 1.008,
07-context 1.010). Sem regressão sistemática (dispersão consistente com
ruído de medição).

## Resultado

- `limits()`/`scripts()` implementadas, sem vazamento de texto nem perda
  de argumentos — fecham 2 dos 4 casos do achado original.
- `attach()` de 6 cantos **separado explicitamente**, registado para
  passo próprio (decisão do dono) — não forçado neste passo, continua com
  o fallback textual documentado (não finge estar corrigido).
- Benchmark sem regressão.
