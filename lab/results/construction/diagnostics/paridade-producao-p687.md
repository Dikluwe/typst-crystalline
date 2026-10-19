# Paridade Produção — P687 — Cores nomeadas globais (paridade vanilla)

**Estado:** fechado (as 18 cores oficiais do vanilla estão ligadas no scope global
com bytes sRGB exactos; `gray` de P686 desapareceu em cetz; próximo bloqueio medido é
`field access não suportado em color`, fora de escopo).

**Commit do trabalho:** `19456bc382594b6bc65fe19f9b4e023ab5150ca5`
**Commit base (HEAD antes deste passo):** `99c8a2a9b550077543ab46bb3a54fa8f1ba3bd28`
**Hora da medição:** `2026-07-10T16:49:16-03:00` (saída de `date -Is`)
**Árvore:** detached HEAD; working tree com apenas 2 ficheiros tracked alterados
(untracked pré-existentes em `materialization/`, `adr/`, `diagnosticos/`, `temp_p*`
não foram tocados).

---

## Proveniência da medição (regra de P569/P574)

- **Código medido:** este commit `19456bc382594b6bc65fe19f9b4e023ab5150ca5`, sobre a base `99c8a2a9`.
- `git diff HEAD --stat` no momento da medição:

```
 00_nucleo/prompts/engine/stdlib/color.md | 58 ++++++++++++++-----
 01_core/src/engine/stdlib/color.rs       | 99 ++++++++++++++++++++++++++++++---
 2 files changed, 135 insertions(+), 22 deletions(-)
```

- `cargo test --workspace`: **4359 passed, 0 failed** (3690 + 610 + 28 + 2 + 27 + 2;
  3 doc-tests ignorados). P686 tinha 4356; **+3** = os 3 testes novos deste passo.
- `crystalline-lint .`: **0 violations**.
- Binários: cristalino `target/debug/typst` (CLI posicional); vanilla
  `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)` (`compile`).

---

## O problema (medido antes de decidir — ADR-0108)

P686 encontrou `unknown variable: gray` ao avaliar `cetz`. Em vez de adicionar só
`gray`, este passo confirma a **lista oficial completa** de cores nomeadas do vanilla
e fecha todos os gaps de uma vez.

**Fonte autoritativa (não inferida):**
`lab/typst-original/crates/typst-library/src/lib.rs:359-376` regista **exactamente 18**
cores globais; os valores estão em `crates/typst-library/src/visualize/color.rs:291-322`.
Confirmado por `#repr(<cor>)` no vanilla (sonda):

| cor | vanilla `repr` | sRGB |
|-----|----------------|------|
| black | `luma(0%)` | `#000000` |
| gray | `luma(66.67%)` | `#AAAAAA` |
| silver | `luma(86.67%)` | `#DDDDDD` |
| white | `luma(100%)` | `#FFFFFF` |
| navy | `rgb("#001f3f")` | `#001F3F` |
| blue | `rgb("#0074d9")` | `#0074D9` |
| aqua | `rgb("#7fdbff")` | `#7FDBFF` |
| teal | `rgb("#39cccc")` | `#39CCCC` |
| eastern | `rgb("#239dad")` | `#239DAD` |
| purple | `rgb("#b10dc9")` | `#B10DC9` |
| fuchsia | `rgb("#f012be")` | `#F012BE` |
| maroon | `rgb("#85144b")` | `#85144B` |
| red | `rgb("#ff4136")` | `#FF4136` |
| orange | `rgb("#ff851b")` | `#FF851B` |
| yellow | `rgb("#ffdc00")` | `#FFDC00` |
| olive | `rgb("#3d9970")` | `#3D9970` |
| green | `rgb("#2ecc40")` | `#2ECC40` |
| lime | `rgb("#01ff70")` | `#01FF70` |

**Falsos positivos excluídos (medidos):** `ostrich`, `pink`, `cyan`, `magenta` →
`unknown variable` no vanilla. Não fazem parte do conjunto oficial. (`cyan`/`magenta`
existiam no cristalino pré-P687 e foram mantidos — ver Débitos.)

**Estado anterior do cristalino** (`predefined_color_bindings` em
`01_core/src/engine/stdlib/color.rs`): tinha 9 entradas com valores **aproximados** e
nomes CSS (`red=rgb(0xEF,0x23,0x11)` ≠ vanilla `#FF4136`; `green=rgb(0,0xB3,0)` ≠
vanilla `#2ECC40`; faltavam `gray/silver/navy/aqua/teal/eastern/purple/fuchsia/maroon/orange/olive/lime`).

**Classificação (ADR-0107):** o *conjunto* de cores globais e os seus **valores
observáveis** (sRGB → PDF) são **semântica da linguagem** → paridade exigida. O
espaço de cor interno (`Luma` vs `Srgb`) e a formatação de `repr` (`luma(66.67%)` /
`rgb("#..")` vs `Srgb { .. }` do `Debug`) são **mecânica** → divergem de propósito
(P329). Aceitação medida pelos bytes sRGB, nunca pela string de `repr`.

---

## O que foi feito

**L0 (Trava Arquitetural — antes de código):**
`00_nucleo/prompts/engine/stdlib/color.md` — secção "Cores predefinidas" reescrita com
a tabela das 18 cores + bytes sRGB, a nota língua/mecânica, e o registo dos extras.
`crystalline-lint --fix-hashes .` → `01_core/src/engine/stdlib/color.rs` `@prompt-hash`
actualizado para `87325eda`.

**Código (`01_core/src/engine/stdlib/color.rs`):** `predefined_color_bindings()`
reescrito — as 18 cores oficiais com `Color::rgb(..)` exacto, seguidas dos extras
`cyan`/`magenta`/`none` (mantidos sem regressão). Consumidores inalterados
(`eval/modules.rs:58`, `eval/mod.rs:364`) injectam tudo no scope global.

**Testes (`#[cfg(test)]` em `color.rs`, +3):**
- `p687_cores_vanilla_18_srgb_exacto` — as 18 cores com `to_srgb()` byte-perfect.
- `p687_cores_inexistentes_no_vanilla_ausentes` — `ostrich`/`pink` ausentes.
- `p687_extras_pre_p687_sem_regressao` — `cyan`/`magenta`/`none` presentes.

---

## Validação cristalino vs vanilla

**As 18 cores compilam nos dois compiladores** (`cores18.typ`: `#black #gray … #lime`
como conteúdo): vanilla exit 0, cristalino exit 0.

**Bytes sRGB (paridade byte-perfect) — `repr` lado a lado:**

| cor | vanilla | cristalino (`Debug` Srgb) | bytes sRGB |
|-----|---------|---------------------------|------------|
| gray | `luma(66.67%)` | `Srgb { r:0.6666667, g:0.6666667, b:0.6666667 }` | `(170,170,170)` ✓ |
| silver | `luma(86.67%)` | `Srgb { 0.8666667×3 }` | `(221,221,221)` ✓ |
| white | `luma(100%)` | `Srgb { 1.0×3 }` | `(255,255,255)` ✓ |
| navy | `rgb("#001f3f")` | `Srgb { r:0.0, g:0.12156863, b:0.24705882 }` | `(0,31,63)` ✓ |
| eastern | `rgb("#239dad")` | `Srgb { r:0.13725491, g:0.6156863, b:0.6784314 }` | `(35,157,173)` ✓ |
| red | `rgb("#ff4136")` | `Srgb { r:1.0, g:0.25490198, b:0.21176471 }` | `(255,65,54)` ✓ |
| green | `rgb("#2ecc40")` | `Srgb { r:0.18039216, g:0.8, b:0.2509804 }` | `(46,204,64)` ✓ |
| lime | `rgb("#01ff70")` | `Srgb { r:0.003921569, g:1.0, b:0.4392157 }` | `(1,255,112)` ✓ |

Os f32 do cristalino são exactamente os do vanilla (bytes sRGB idênticos); a diferença
é só a **formatação** do `repr` (`Debug` vs pretty) e o espaço (`Srgb` vs `Luma`) —
mecânica (ADR-0107).

**cetz re-testado (critério do passo):**
- Erro de `gray` **desapareceu** (`grep -i gray` no output → vazio).
- Próximo erro cristalino: `field access não suportado em color` em `<detached>` —
  cetz acede a um campo/método de uma `Color` (ex.: componentes), feature que o
  cristalino ainda não suporta. É um bloqueio **novo e distinto**, a montante da
  resolução de nomes (que já passa). O vanilla, no mesmo doc mínimo, também não
  produz PDF — falha mais à frente, em `panicked with: Failed to resolve coordinate`
  (`shapes.typ`/`canvas.typ:24`), porque *tem* field-access em cor e avança até às
  coordenadas. Logo o cristalino agora bloqueia **depois** da resolução de nomes/cores
  (P686+P687 fechados) mas **antes** do ponto onde o vanilla tropeça — registado como
  débito, fora de escopo de P687.

---

## Débitos (fora de escopo)

- `field access não suportado em color` — paridade de field/method access em valores
  `Color` (cetz usa-o); passo próprio.
- `cyan`/`magenta` (globais extra não-vanilla) — falso-aceite pré-existente: cristalino
  aceita `#cyan`/`#magenta` que o vanilla rejeita. Mantidos por "sem regressão"; remover
  é uma decisão separada (paridade estrita de conjunto vs compatibilidade).
- Espaço de cor / formatação de `repr` (`Srgb`+`Debug` vs `Luma`/`rgb("#..")`) —
  mecânica; não afecta os bytes sRGB observáveis.
- Parser de cores por string (`fill: "gray"` → `rgb(128,128,128)` em `shapes.rs`) — via
  separada (nomes CSS); inalterado e fora do âmbito (cores *globais*) deste passo.

## Conclusão

O conjunto oficial de 18 cores nomeadas do vanilla está ligado no scope global do
cristalino com **bytes sRGB exactos** (paridade ao nível da língua, ADR-0107), medido
com proveniência (ADR-0108). O bloqueio de `gray` (P686) está removido em cetz; o
próximo bloqueio medido (`field access` em `Color`) é independente e fica registado.
