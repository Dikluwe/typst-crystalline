# Relatório — typst-passo-817: `foundations::calc` — trig inversa `angle`, `quo` floored, `pow` int-neg, `decimal`, precisão (achado #4 de P810, prioridade alta)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-817.md`; único ficheiro acedido em `materialization/`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** nas medições "antes" e "depois" (`git diff HEAD --stat` na sonda ANTES, 2026-07-22 ~00:20 -03: 34 ficheiros, +1996/-204 — alterações de P823/P824/P827/P814/P815/P821/P816/P820; `git diff HEAD --stat` final: 39 ficheiros, +2665/-324). Binário cristalino rebuildado (`cargo build --release`) antes da sonda ANTES (baseline) e depois da implementação.
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Texto extraído com `pdftotext`. Fixtures em `temp/p817/`.

---

## Sub-achado A — `asin`/`acos`/`atan`/`atan2` devolviam `float` em vez de `angle`

**Sonda ANTES** — `probe-a.typ`: `#repr(calc.asin(0.5)) #repr(calc.acos(0.5)) #repr(calc.atan(1)) #repr(calc.atan2(1, 2))`

```text
vanilla:            30deg 60deg 45deg 63.43deg
cristalino (ANTES): 0.5235987755982989 1.0471975511965979 0.7853981633974483 1.1071487177940904
```

**Código identificado:** cristalino `01_core/src/engine/stdlib/calc.rs:422-495` (`calc_asin`/`calc_acos`/`calc_atan`/`calc_atan2` via `trig_op`/`guard_float` → `Value::Float`) vs vanilla `crates/typst-library/src/foundations/calc.rs:302-359` (`-> SourceResult<Angle>`, `Angle::asin/acos/atan/atan2`). O cristalino já tinha `Value::Angle` (`entities/layout_types.rs:906`) e repr de ângulo com paridade (`engine/eval/repr.rs:203` → `63.43deg`).

**Implementação:** as quatro funções passam a devolver `Value::Angle(Angle::rad(r))` via novo helper `angle_op`; checagem de domínio de `asin`/`acos` mantida (mensagem PT — ver §"Mensagens" no fim). **Dependência de composabilidade medida:** com o retorno em `angle`, `calc.sin(calc.asin(0.5))` passaria a falhar sem suporte a `Angle` em `sin`/`cos`/`tan` — o vanilla aceita `AngleLike` (`calc.rs:233-296`; medido `#repr(calc.sin(90deg))` → vanilla `1.0` / cristalino ANTES erro `esperava Int ou Float, recebeu angle`). Implementado `unary_angle` (aceita `Int|Float|Angle`, converte para radianos) nas três funções.

**Validação DEPOIS:**

```text
probe-a  cristalino: 30deg 60deg 45deg 63.43deg   ✓ byte-idêntico ao vanilla
m5       `#repr(calc.tan(45deg))`: vanilla 0.9999999999999999 / cristalino 0.9999999999999999 ✓
m6       `#calc.asin(0.5)` (display): vanilla 30deg / cristalino 30deg ✓
```

## Sub-achado B — `quo` truncava em vez de arredondar para baixo (floor)

**Sonda ANTES** — `probe-b.typ`: `#calc.quo(-7, 2) #calc.quo(7, 2) #calc.quo(-7.0, 2)`

```text
vanilla:            −4 3 −4     (− = U+2212, ver §"Apresentação")
cristalino (ANTES): -3 3 -3
```

**Código identificado:** cristalino `calc.rs:892-921` (`checked_div` + `trunc() as i64`; o comentário "paridade vanilla `calc.quo(-7, 2) = -3`" estava **errado**, como P810 notou) vs vanilla `calc.rs:1140-1177` (floored: `q - 1` quando sinais diferem e há resto; caminho float faz `floor`; retorno sempre `i64`).

**Implementação:** `calc_quo` reescrito — Int/Int com ajuste floored verbatim do vanilla; caminho float `(af/bf).floor()` com verificação de alcance i64 (o ANTES saturava silenciosamente via `as i64`).

**Validação DEPOIS:** `probe-b` cristalino → `-4 3 -4` ✓ (valores iguais ao vanilla).

## Sub-achado C — `pow` com expoente inteiro negativo

**Sonda ANTES** — `probe-c.typ`: `#calc.pow(2, -1) #repr(calc.pow(2, -1)) #calc.pow(2, 3) #repr(calc.pow(2.0, -1))`

```text
vanilla:            0.5 0.5 8 0.5        (exit 0)
cristalino (ANTES): error: calc.pow() expoente negativo requer Float   (exit 1)
```

**Código identificado:** cristalino `calc.rs:141-164` vs vanilla `calc.rs:108-156`.

**Implementação** (`calc_pow` reescrito, paridade vanilla): `0^0` → Err (medido vanilla: `error: zero to the power of zero is undefined`); expoente Int que não cabe em i32 → Err; expoente Float não-normal (inf/subnormal/NaN) → Err; `(Int, Int≥0)` → `Int` com `checked_pow` (o ANTES usava `saturating_pow` — overflow silencioso; vanilla dá `the result is too large`); `(Int, Int<0)` → `Float` via `powi`; resto → `powf` + `guard_float`. **Divergência consciente mantida (ADR-0101):** `calc.pow(0, -1)` → vanilla `float.inf` / cristalino Err `resultado é infinito` (política `guard_float` documentada no L0 §"Política IEEE 754").

**Validação DEPOIS:** `probe-c` cristalino → `0.5 0.5 8 0.5` ✓ byte-idêntico ao vanilla.

## Sub-achado D — `decimal` em `calc`

**Decisão de âmbito (CONDICIONAL do prompt):** procurada decisão anterior nos handoffs `handoff-novo-chat-p762.md`, `p798.md`, `p807.md` — **nenhuma decisão de scope-out encontrada** (`decimal` só aparece em p807 como a linha do achado de P810). O tipo `Value::Decimal` existe em L1 desde P399 (`entities/decimal.rs`, ADR-0112) com constructor `decimal(...)` funcional no eval, mas nenhuma função `calc` o aceitava. **Decisão: implementar conforme a medição vanilla** (registado aqui).

**Sonda ANTES:**

```text
probe-d   `#repr(calc.abs(decimal("-342.440"))) #repr(calc.round(decimal("3.14159"), digits: 2))`
  vanilla:            decimal("342.440") decimal("3.14")
  cristalino (ANTES): error: calc.abs() requer Int ou Float, recebeu decimal
probe-d2  `#calc.pow(decimal("2"), 2)`
  vanilla: 4
  cristalino (ANTES): error: calc.pow() base: esperava Int ou Float, recebeu decimal
probe-d3  `#calc.pow(decimal("2"), 2.0)`
  vanilla: error: cannot apply this operation to a decimal and a float
           = hint: if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`
  cristalino (ANTES): error: calc.pow() base: esperava Int ou Float, recebeu decimal   (sem erro dedicado, sem hint)
```

**Código identificado:** vanilla suporta `Decimal` em `abs` (:71-93), `pow` c/ expoente int (:131-136), `floor`/`ceil`/`trunc` (:734-794), `fract` (:810-815), `round` (:856-874) e tem o erro dedicado `cant_apply_to_decimal_and_float` + hint (:1371-1377). Cristalino: nenhum braço `Decimal` em `calc.rs`.

**Implementação:**
- `entities/decimal.rs`: novos métodos `abs`/`floor`/`ceil`/`trunc`/`fract`/`to_i64`/`checked_powi` (feature `maths` do `rust_decimal` activada no workspace `Cargo.toml` — mesma dependência ADR-0112, feature que o vanilla também usa) e `round_with_digits` (port verbatim de `foundations/decimal.rs:159`, `MidpointAwayFromZero` + dígitos negativos).
- `calc.rs`: braços `Decimal` em `calc_abs` (→ Decimal), `calc_floor`/`calc_ceil`/`calc_trunc` (→ Int, overflow → Err), `calc_fract` (→ Decimal), `calc_round` (→ Decimal), `calc_pow` (`(Decimal, Int)` → Decimal via `checked_powi`; `(Decimal, Float)` → **erro dedicado verbatim vanilla + hint**, com span real `1:9` — primeira mensagem calc sem `<detached>`).
- `engine/eval/repr.rs`: repr de `Decimal` corrigido para `decimal("...")` (paridade vanilla; ANTES: número puro).
- `engine/eval/mod.rs`: display de `Decimal` em markup usa `to_string()` (medido: `#decimal("1.50")` → vanilla `1.50` / cristalino ANTES teria usado o repr). 
- `crystalline.toml`: `[l1_allowed_external.rust_decimal]` + `RoundingStrategy`, `MathematicalOps`.

**Achados extra medidos neste sub-achado (tipos de retorno — morfologia da língua):**
- `calc.fract(3)` → vanilla `0` (**Int**) / cristalino ANTES `0.0` → corrigido para `Int(0)`.
- `calc.round(2.5)` → vanilla `3.0` (**Float**, half away from zero) / cristalino ANTES `3` (Int) → corrigido: `round(Float)` → Float sempre.
- `calc.round(123, digits: -1)` → vanilla `120` (**Int**) / cristalino ANTES `120.0` → corrigido com `round_int_com_precisao` (port de `typst-utils/round.rs:82`, away-from-zero); `digits > 0` em Int é no-op (vanilla devolve o Int inalterado; ANTES devolvia Float).
  Nota: estas divergências estavam **mascaradas no display** (vanilla mostra `3.0` como `3` em markup) — só visíveis via `repr`.

**Validação DEPOIS:**

```text
probe-d   cristalino: decimal("342.440") decimal("3.14")   ✓ byte-idêntico
probe-d2  cristalino: 4   ✓ (display; repr → decimal("4") como o vanilla)
probe-d3  cristalino: probe-d3.typ:1:9: error: cannot apply this operation to a decimal and a float
                      hint: if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`   ✓ verbatim + hint
m6        `#decimal("1.50")` → 1.50 ✓; testes unitários: round(decimal("-6.5")) = -7, round(decimal("3333.45"), digits: -2) = 3300 (asserts do doc vanilla reproduzidos)
```

**Scope-out medido em D:** `min`/`max`/`clamp`/`quo` com `Decimal` (o vanilla aceita via `DecNum::apply2`, com o mesmo erro dedicado em misturas decimal×float — `calc.rs:1035/1077/1122/1166`). Não implementado: exige coerção mista Int/Float/Decimal em 4 funções variádicas/multi-arg; não estava na medição de P810 (que mediu `abs` + erro dedicado). Fica registado como trabalho futuro com a medição vanilla já localizada.

## Sub-achado E — `log10`/`deg`/`rad` extra

**Sonda:** `probe-e2`/`probe-e3`:

```text
vanilla:            error: module `calc` does not contain `log10`   /   error: module `calc` does not contain `deg`
cristalino:         3   /   180   (funcionam)
```

Confirmado: são **extensões cristalinas** (introduzidas em P501), o vanilla não as expõe. **Decisão (conforme o prompt: "registar a decisão… não implementar sem essa decisão"): MANTER como extensão documentada** — remover quebraria o L0 de P501 e código existente sem ganho de paridade da língua (o vanilla simplesmente não tem as funções; a presença delas não altera nenhum programa vanilla válido). Trancado pelo teste `p817e_log10_deg_rad_extensao_cristalina` e registado no header de `calc.rs`.

## Sub-achado F — precisão `erf`/`log`/`exp`

**Sonda ANTES** — `probe-e.typ`: `#calc.erf(1) #calc.erf(0.5) #calc.log(1000) #calc.log(100, base: 10) #calc.exp(1) #calc.cosh(1)`

```text
vanilla:            0.8427007929497149 0.5204998778130465 3 2 2.7182818284590455 1.543080634815244
cristalino (ANTES): 0.8427006897475899 0.5205000163047472 2.9999999999999996 2 2.718281828459045 1.5430806348152437
```

**Implementado:** `calc_log` com despacho por base exacta (paridade vanilla `calc.rs:506-515`): base `e` → `ln`, base `2` → `log2`, base `10` → `log10`, outras → `ln(x)/ln(base)`. **DEPOIS: `calc.log(1000)` → `3` ✓** (e `log(8, base: 2)` → `3` exacto).

**Scope-out com justificação medida:**
- `erf` — divergência **documentada e consciente** (A&S 7.1.26, erro máx. 1.5e-7; ADR-0054 graded, L0 §"Função erro", diagnóstico P283/P308). Fechar exigiria `libm::erf`. 
- `exp(1)`/`cosh(1)` — último dígito (1 ulp): o vanilla usa `libm::exp`/`libm::cosh`; o cristalino usa `f64::exp`/`f64::cosh` (std). **`libm` não é dependência do workspace** — adicioná-la a L1 exige ADR nova + whitelist `[l1_allowed_external]` (o débito já está registado como DEBT-libm em `calc.rs`, ADR-0018). Fora do alcance deste passo; fica a medição exacta acima para o passo que introduzir `libm`.

## Achado extra (não listado em P810) — `calc.root` com ordem de argumentos invertida

Descoberto pelo **controlo de regressão** (`t1.typ` de P810): `calc.root(3, -8)` → vanilla `0.8716855428717357` / cristalino ANTES `-2`. Medição dedicada (`probe-root`):

```text
vanilla:            #calc.root(3, -8) #calc.root(-8, 3) #calc.root(27.0, 3)  →  0.8716855428717357 −2 3
cristalino (ANTES): error: calc.root() índice deve ser Int, recebeu float
```

O vanilla é `calc.root(radicand, index)` (`calc.rs:212-236`, doc `#calc.root(16.0, 4)`); o cristalino tinha `(index, x)` — e o **L0 `calc.md` documentava a ordem errada** (tal como documentava o `quo` truncado errado). Corrigido para a ordem vanilla (índice negativo → `x^(1/index)`, radicando negativo c/ índice ímpar → raiz real negativa) e L0 actualizado. **DEPOIS: `0.8716855428717357 -2 3` ✓ byte-idêntico.**

## Apresentação (registado, fora do módulo calc)

Diferenças que restam no diff do documento completo, todas fora da morfologia de `calc`:
- **Sinal de menos:** o vanilla renderiza números negativos com U+2212 (`−4`); o cristalino usa hífen ASCII (`-4`). Camada de display/render — achado novo, a triar num passo próprio.
- **`calc.inf` em markup:** vanilla mostra `∞`; cristalino mostra `inf` (display de Float infinito — camada de display).
- **Linhas em branco:** paginação/espaçamento de parágrafos (camada de layout).
- **Mensagens de erro em português + `<detached>`** (nota transversal de P810): mantidas em PT por consistência com o resto do cristalino — **excepto** o erro dedicado decimal×float, implementado verbatim em inglês + hint (a mensagem é o observável, ADR-0108). O span real (`1:9`) aparece nesse erro novo; os restantes mantêm `<detached>` (limitação transversal já registada em P810).

## Testes

- **20 testes novos** em `01_core/src/engine/eval/tests.rs` (`p817a_*`×5, `p817b_*`×2, `p817c_*`×3, `p817d_*`×7, `p817e_*`×1, `p817f_*`×1, `p817_root_*`×1), escritos **antes** da implementação: 18 falharam (2 já passavam — a extensão E e `pow` int positivo), confirmando a falha antes da correcção.
- **Testes antigos actualizados** (codificavam o comportamento divergente medido como errado): `calc_pow_negativo_retorna_err` → `calc_pow_expoente_negativo_devolve_float`; `calc_quo_truncado` → `calc_quo_floored`; `calc_floor_ceil_round`; `p495_calc_round_digits_named`/`_int_input`; `calc_fract_int_e_float`; `calc_asin/acos/atan/atan2_*` (agora `approx_angle`); `calc_root_raiz_quadrada_e_cubica`/`calc_root_erros` (ordem vanilla); `p709_std_submodulo_apesar_de_sombreamento` (`round(3.7)` → Float(4.0)); teste de repr em `repr.rs` (decimal → `decimal("123")`); novo helper `approx_angle` em `stdlib/mod.rs`.
- **Contagem (`cargo test -p typst-core`):** ANTES `4419 passed; 0 failed; 2 ignored` → DEPOIS `4439 passed; 0 failed; 2 ignored` (4419 + 20 novos ✓).
- **`cargo test -p typst-infra`:** `667 passed; 0 failed; 5 ignored` (inalterado — não toquei infra).
- **Controlo de regressão:** as chamadas já em paridade (`temp/p817/control.typ`, derivado de `t1.typ` de P810 sem as linhas em correção) mantêm os mesmos valores; o documento completo `t1-full.typ` (51 chamadas) difere do vanilla apenas nos itens de apresentação/precisão listados em §"Apresentação" e §F scope-out.

## Lint e L0

- L0 `00_nucleo/prompts/engine/stdlib/calc.md` actualizado (trig inversa → `Angle`; `sin/cos/tan` aceitam `Angle`; `pow`; `quo` floored; `root` ordem vanilla; `round`/`fract`/`floor`/`ceil`/`trunc`/`abs` com `Decimal` e tipos correctos; `log` com despacho; decisão E). `crystalline-lint --fix-hashes .` executado → header de `calc.rs` actualizado para `8c403615`.
- `crystalline-lint .` → **0 errors** (restam apenas warnings V7 de prompts órfãos **pré-existentes**, não relacionados com calc).

## Ficheiros alterados

- `01_core/src/engine/stdlib/calc.rs` — A/B/C/D/F + root + header.
- `01_core/src/entities/decimal.rs` — métodos de apoio a decimal (abs/floor/ceil/trunc/fract/to_i64/checked_powi/round_with_digits).
- `01_core/src/engine/eval/repr.rs` — repr de decimal + teste.
- `01_core/src/engine/eval/mod.rs` — display de decimal em markup.
- `01_core/src/engine/eval/tests.rs` — 20 testes p817 + actualização p709.
- `01_core/src/engine/stdlib/mod.rs` — testes antigos actualizados + helper `approx_angle`.
- `Cargo.toml` — feature `maths` do `rust_decimal`.
- `crystalline.toml` — whitelist `RoundingStrategy`/`MathematicalOps`.
- `00_nucleo/prompts/engine/stdlib/calc.md` — L0 actualizado (+ hash `8c403615`).

## Scope-outs consolidados

1. **E** — `calc.log10/deg/rad` mantidos como extensão cristalina (decisão; vanilla não as tem).
2. **F** — `erf` (A&S documentado, ADR-0054) e último ulp de `exp/cosh` (requer `libm` em L1 → ADR nova; DEBT-libm já registado).
3. **D** — `Decimal` em `min`/`max`/`clamp`/`quo` (medição vanilla localizada em `calc.rs:1035/1077/1122/1166`; fora da medição de P810).
4. **Apresentação** — minus U+2212, `∞` vs `inf`, linhas em branco de paginação (camada display/layout; achados novos a triar).
5. **Consciente (ADR-0101)** — `calc.pow(0, -1)` → Err no cristalino vs `float.inf` no vanilla (política `guard_float`).
