# Passo 433 — relatório: fecho do subset `rules/stdlib/calc.rs` (DEBT-57)

**Tipo:** documental / especificação L0 (zero código funcional modificado).  
**Data:** 2026-06-24. **HEAD base:** `a04572b31`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=8388608`.

## O que se fez

Fechou-se o subset `rules/stdlib/calc.rs` do débito DEBT-57, reestruturando
o prompt L0 dedicado `calc.md` para o formato de secções individuais exigido
pelo plano P433.

- **L0 actualizado:**
  - `00_nucleo/prompts/rules/stdlib/calc.md` — reescrito com 21 secções
    individuais (7 trig + 6 hiperbólicas + `exp` + `ln` + `log` + `log` com
    base explícita + 4 constantes). O conteúdo anterior das funções base
    (P27), aritmética inteira/combinatória/norma/raiz (P306) e `erf` (P308)
    foi preservado num apêndice de tabelas.
- **Cabeçalho `@prompt`:**
  - `01_core/src/rules/stdlib/calc.rs` já apontava para `calc.md`; o hash
    foi sincronizado pela ferramenta de lint.
- **Prompt comum actualizado:**
  - `00_nucleo/prompts/rules/stdlib/_comum.md` — adicionada nota sobre o
    `calc.md` (P433).
- **Débito actualizado:**
  - `00_nucleo/diagnosticos/debt/DEBT.md` — adicionada entrada para
    `rules/stdlib/calc.rs` marcada como fechada em P433; critério de fecho
    actualizado.

`cargo test --workspace` verde; `crystalline-lint` sem novas violações
(mantêm-se apenas os dois warnings pré-existentes: `adr-stub-vs-fallback.md`
e `show-regex.md`).

## Nota sobre a contagem de entradas

O plano P433 listava 21 funções nativas (`sin`, `cos`, `tan`, `asin`, `acos`,
`atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `exp`,
`ln`, `log`, `pi`, `tau`, `e`, `inf`). Essa lista contém 20 itens; as 4
constantes não são funções. Para atingir as 21 secções solicitadas, o prompt
separa `calc.log` em duas secções:

1. `calc.log(x)` — base default 10.
2. `calc.log(x, base)` — base explícita (divergência vanilla: vanilla usa
   named arg `base:`, cristalino usa segundo posicional).

Assim, o prompt tem 21 secções, cobrindo o subset de 20 itens do plano mais
a variante de interface do `log`.

## Decisão de engenharia

- **Atomização por função (ADR-0109):** o plano P433 exigia 1 secção por
  entrada do subset. O prompt anterior `calc.md` usava tabelas por categoria;
  o P433 converteu as entradas do subset para o formato de secção individual,
  mantendo as restantes funções do módulo em apêndice para não perder
  cobertura documental.
- **Scope-outs preservados:**
  - Bucket 2 adiado P283 (`root`, `erf`, `fact`, `perm`, `binom`, `gcd`,
    `lcm`, `trunc`, `fract`, `even`, `odd`, `rem`, `div_euclid`,
    `rem_euclid`, `quo`, `norm`) continua fora do subset, embora documentado
    no apêndice.
  - Extensões `Length`/`Angle`/`Decimal` continuam scope-out.
- **Linhagem `@prompt`:** `calc.rs` já tinha prompt dedicado; o P433 limitou-se
  a reestruturar o seu conteúdo e sincronizar hashes.

## Verificação

| Critério | Resultado |
|----------|-----------|
| `00_nucleo/prompts/rules/stdlib/calc.md` com 21 secções | ✓ |
| Cada secção cobre assinatura, args, domínio, semântica, paridade vanilla, limitações e testes canónicos | ✓ |
| `_comum.md` actualizado | ✓ |
| `DEBT.md` actualizado com nota de fecho P433 | ✓ |
| `crystalline-lint` sem novas violações | ✓ |
| `cargo test --workspace` verde | ✓ |
| Zero código funcional modificado | ✓ |

## Artefactos

- L0:
  - `00_nucleo/prompts/rules/stdlib/calc.md` (reescrito)
  - `00_nucleo/prompts/rules/stdlib/_comum.md` (actualizado)
- Código (apenas cabeçalho de linhagem / hashes):
  - `01_core/src/rules/stdlib/calc.rs`
  - `01_core/src/rules/stdlib/assert.rs` (hash sincronizado)
  - `01_core/src/rules/stdlib/gradients.rs` (hash sincronizado)
  - `01_core/src/rules/stdlib/transforms.rs` (hash sincronizado)
- Débito:
  - `00_nucleo/diagnosticos/debt/DEBT.md`
- Plano:
  - `00_nucleo/materialization/typst-passo-433.md`
- este relatório.

## Próximo passo

Com P433 fechado, continuam em aberto em DEBT-57 os subsets `assert.rs`
(trivial, XS), `shapes.rs`, `transforms.rs`, `gradients.rs` e
`foundations.rs`. Alternativas fora de DEBT-57: **DEBT-43** (linter
type-level) ou **DEBT-55** (probe hayagriva).
