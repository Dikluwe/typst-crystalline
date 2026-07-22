# Relatório — typst-passo-818: `foundations::ops` — ordenação e operadores ausentes (achado #5 de P810, prioridade alta)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-818.md`; único ficheiro acedido em `materialization/`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** nas medições "antes" e "depois" (`git diff HEAD --stat` na sonda ANTES: 39 ficheiros, +2665/-324 — passos P823/P824/P827/P814/P815/P821/P816/P820/P817; `git diff HEAD --stat` final, 2026-07-22 01:29 -03: 41 ficheiros, +3262/-367). Binário cristalino rebuildado (`cargo build --release`) após a implementação.
**Binários:** `./target/release/typst` (cristalino — `typst <input> -o <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Texto extraído com `pdftotext`. Fixtures em `temp/p818/`.

---

## Nota sobre o ficheiro e o L0

O prompt cita `ops.rs`; o ficheiro real é `01_core/src/engine/eval/operators.rs` (`@prompt 00_nucleo/prompts/engine/eval/ops.md`). O L0 foi lido antes de implementar. Scope-outs declarados **respeitados**: divisões mistas `Length↔Relative`/`Ratio↔Relative` (§P713), `Length * Ratio` (§P725), `Dict * Int` (§P722), textos de mensagens de erro (sub-achado (i), §P706/§P728). **Uma fronteira do L0 estava errada face à medição** e foi corrigida no L0 com a medição anexada (ver §g): "Comparações de `Relative` permanecem sem suporte — requerem contexto de layout" é falso no vanilla (`layout/rel.rs:193-202` compara puramente).

## (a) Ordenação de `str` — ANTES → DEPOIS

`ord.typ`: `#("b" < "a") #("a" < "b") #((1,2) < (1,3)) #((1,2) < (1,2,0)) #(false < true) #(true > false)`

```text
vanilla:            false true true true true true
cristalino (ANTES): error: cannot apply Lt to str and str
cristalino (DEPOIS): false true true true true true   ✓ byte-idêntico
```

Código: vanilla `ops::compare` (`foundations/ops.rs:483` — `Str(a), Str(b) => a.cmp(b)`). Cristalino: não havia braço; implementado via helper `value_cmp` + braço combinado `(op @ (Lt|Leq|Gt|Geq), a, b)` em `operators.rs`.

## (b) Ordenação lexicográfica de `array`

Mesmo documento `ord.typ` (linhas 3-4): `(1,2) < (1,3)` → `true`, `(1,2) < (1,2,0)` → `true` — ANTES erro, DEPOIS ✓. Código: vanilla `try_cmp_arrays` (`ops.rs:514-530` — recursiva com a comparação completa; prefixo igual → mais curto é menor). Cristalino: `cmp_arrays` (port) com recursão em `value_cmp`; elementos incomparáveis → erro (paridade do observável, medido no vanilla: `cannot compare ...`).

## (c) Ordenação de `bool`

`ord.typ` (linhas 5-6): `false < true` → `true`, `true > false` → `true` — ANTES erro, DEPOIS ✓. Código: vanilla `ops.rs:473` (`Bool(a), Bool(b) => a.cmp(b)`).

## (d) Divisões `Relative/Relative` e `Ratio/Ratio`

`div.typ`: `#(50% / 25%)` `#((10pt + 0%) / (5pt + 0%))`; `divzero.typ`: `#(50% / 0%)`

```text
vanilla:            22   (duas linhas, ambas 2)
cristalino (ANTES): error: cannot apply Div to relative length and relative length
cristalino (DEPOIS): 22   ✓ byte-idêntico
divzero vanilla:    error: cannot divide by zero
divzero DEPOIS:     error: cannot divide by zero   ✓ verbatim (antes: "cannot apply Div...")
```

Código: vanilla `ops.rs:330` (`Relative/Relative → Rel::try_div`, `layout/rel.rs:128-137` — rel ambos zero → rácio de abs; abs ambos zero → rel/rel; misto → erro `"cannot divide these two relative lengths"`), `ops.rs:323` (`Ratio/Ratio → Float`), gate `is_zero` (`ops.rs:344-359`). Cristalino: braço `Relative/Relative` (port de `try_div`), braço `Ratio/Ratio`, gate alargado a `Relative`/`Ratio`/`Angle`. Confirmado no L0: estas duas divisões **não** estavam no scope-out (que cobre só as mistas). Misto incomensurável (`(10pt + 10%) / (5pt + 5%)`) → erro nos dois ✓ (teste unitário).

## (e) Repetição `Str * Int`

`strmul.typ`: `#("ab" * 2) #(2 * "ab") #("ab" * 0)`; `strmulneg.typ`: `#("ab" * -1)`

```text
vanilla:            abab abab   (+ linha vazia para *0)
cristalino (ANTES): error: cannot apply Mul to str and int
cristalino (DEPOIS): abab abab   ✓ byte-idêntico
negativo vanilla:   error: number must be at least zero
negativo DEPOIS:    error: number must be at least zero   ✓ verbatim
```

Código: vanilla `ops.rs:272-273` + `Str::repeat` (`foundations/str.rs:92-99`). Cristalino: braço com guarda partilhada (espelho do P722 `Array * Int`).

## (f) Igualdade `Length == Relative` com rel zero

`eqrel.typ`: `#(10pt == (10pt + 0%))` `#(10pt == (10pt + 1%))`

```text
vanilla:            true false
cristalino (ANTES): false false
cristalino (DEPOIS): true false   ✓ byte-idêntico
```

Código: vanilla `ops::equal` (`ops.rs:458-460` — `len == rel.abs && rel.rel.is_zero()`). Cristalino: coberto em `values_eq` (helper novo, usado pelo braço genérico de `Eq`/`Neq`).

## (g) Ordenação `Length < Relative` (+ `Length < Length` e `Relative < Relative`)

`ordlen.typ`: `#(10pt < (20pt + 0%))` `#((20pt + 0%) > 10pt)` `#(1cm < 2cm)` `#(2em > 1em)`; `ordguard.typ`: `#(10pt < (10pt + 1%))`; `relord*.typ`: `50% < 60%`, `(10pt + 50%) < (20pt + 50%)`

```text
ordlen   vanilla: true true true true   /   cristalino ANTES: error   /   DEPOIS: true true true true ✓
ordguard vanilla: error: cannot compare length and relative length   /   DEPOIS: error (fronteira) ✓ (observável)
relord1  vanilla: true true   /   DEPOIS: true true ✓  (`50% < 60%`)
relord misto:     vanilla: error: cannot compare 50% + 10pt with 50% + 20pt   /   DEPOIS: error ✓ (observável)
```

Código: vanilla `ops.rs:476` (Length/Length), `:481` (Relative/Relative), `:491-494` (guards Length↔Relative com rel zero), `length.rs:195-204` e `rel.rs:193-202` (ordens parciais — comparável só quando medido numa única componente). Cristalino não tinha **nenhuma** ordenação de Length. Implementado `length_partial_cmp`/`rel_partial_cmp` (ports) + guards em `value_cmp`. **Nota:** o L0 declarava "comparações de `Relative` requerem contexto de layout" — medição refutou (o vanilla compara puramente); a fronteira foi **revogada no L0 com a medição anexada** (ADR-0108: medir antes de decidir; desconfiar do enquadramento cômodo). P810 já notava "L0 cobre parcialmente".

## (h) Coerção `Int ↔ Float` aninhada (igualdade e contenção)

`nested.typ`: `#((1,2) == (1.0,2.0))` `#((1,(2,)) == (1.0,(2.0,)))` `#((a: 1) == (a: 1.0))` `#(1 in (1.0, 2.0))` `#((1,) in ((1.0,), (2,)))`

```text
vanilla:            true true true true true
cristalino (ANTES): false false false true false
cristalino (DEPOIS): true true true true true   ✓ byte-idêntico
```

Código: vanilla `Value::eq` **é** `ops::equal` (`value.rs:295-299`) — a coerção propaga-se naturalmente a elementos aninhados; o cristalino delegava no `PartialEq` derivado (ADR-0025), sem coerção. Implementado `values_eq` recursivo (coerção em qualquer profundidade; `Length↔Relative` e `Ratio↔Relative` aninhados; `Content` morfológico P345 aninhado; resto delega no derivado), usado pelo braço genérico de `Eq`/`Neq` e por `value_eq` do operador `in`.

## Extra medido (não listado em P810) — ops de `Angle`

Consequência de P817 (`calc.asin` e cia. passaram a devolver `angle`): as combinações de `Angle` do vanilla tornaram-se alcançáveis pela sintaxe. `angle.typ`: `#(30deg < 45deg)` `#(30deg == 30deg)` `#(90deg / 2)` `#(2 * 30deg)` `#(30deg / 30deg)`

```text
vanilla:            true true 45deg 60deg 1
cristalino (ANTES): error: cannot apply Lt to angle and angle
cristalino (DEPOIS): true true 45deg 60deg 1   ✓ byte-idêntico
```

Implementado: `Angle * Int|Float` (e inverso), `Angle / Int|Float`, `Angle / Angle` → Float (vanilla `ops.rs:241-246,317-319`), ordenação (`ops.rs:477`). `Angle == Angle` já funcionava (derivado).

## (i) Mensagens de erro de tipo — scope-out declarado, não reaberto

Declarado aceite no L0 (`ops.md` §P706 :107-113 e §P728 :446-450): o texto das mensagens de fronteira diverge do vanilla (`cannot apply Lt to str and str` vs `cannot compare string and string`), preservando o observável "é erro de tipo". Não tocado. Erros que **são** o observável ficaram verbatim: `cannot divide by zero`, `number must be at least zero`, `cannot divide these two relative lengths` (este último no caminho misto, mesmo texto do vanilla).

## Controlo de regressão

`vals.typ` de P810 (21 expressões, `temp/p810/ops/vals.typ`): **byte-idêntico** aos dois binários após as alterações ✓ (`diff vals-control-va.txt vals-control-cr2.txt` vazio). Sondas de P817 não afectadas (módulo distinto).

## Testes

- **10 testes novos** em `01_core/src/engine/eval/tests.rs` (`p818a`…`p818h`, `p818d` ×2, `p818_angle_ops`), escritos **antes** da implementação: 10/10 falharam, confirmando a lacuna.
- Nenhum teste antigo precisou de actualização (as combinações novas eram erro antes; o braço genérico de `Eq` só alarga a coerção).
- **Contagem (`cargo test -p typst-core`):** ANTES `4439 passed; 0 failed; 2 ignored` → DEPOIS `4449 passed; 0 failed; 2 ignored` (4439 + 10 novos ✓).
- **`cargo test -p typst-infra`:** `667 passed; 0 failed; 5 ignored` (inalterado — não toquei infra).

## Lint e L0

- L0 `00_nucleo/prompts/engine/eval/ops.md`: revogada a fronteira errada sobre comparações de `Relative` (com medição); nova secção **§P818** (tabelas de ordenação, divisões, `Str * Int`, igualdade recursiva, ops de `Angle`, scope-outs mantidos). `crystalline-lint --fix-hashes .` executado → header de `operators.rs` actualizado para `a16b6afa`.
- `crystalline-lint .` → **0 errors** (restam apenas warnings V7 de prompts órfãos pré-existentes, não relacionados).

## Ficheiros alterados

- `01_core/src/engine/eval/operators.rs` — braços novos (ordenação combinada, div Relative/Ratio/Angle, mul Str/Angle), gate `is_zero` alargado, helpers `values_eq`/`value_cmp`/`cmp_arrays`/`length_partial_cmp`/`rel_partial_cmp`, header P818.
- `01_core/src/engine/eval/tests.rs` — 10 testes p818.
- `00_nucleo/prompts/engine/eval/ops.md` — revogação da fronteira errada sobre `Relative` + §P818.

## Scope-outs consolidados

1. Divisões mistas `Length↔Relative` / `Ratio↔Relative` (L0 §P713 — `Value::Ratio` não produzível por sintaxe de utilizador; sem consumidor medido).
2. `Length * Ratio` / `Ratio * Length` (L0 §P725); `Dict * Int` (L0 §P722 — inexistente no vanilla).
3. Ordenação `Ratio ↔ Relative` (vanilla `ops.rs:492,494`) — `Ratio` não produzível por sintaxe; sem consumidor medido. (`Ratio/Ratio` eq/ord/div e `Ratio↔Relative` eq já cobertos.)
4. Textos das mensagens de erro de tipo — sub-achado (i), declarado aceite no L0; só os erros que são o observável ficaram verbatim.
5. `Content * Int` (repetição de content, vanilla `ops.rs:276-277`) — não medido neste passo; registado para triagem futura.
