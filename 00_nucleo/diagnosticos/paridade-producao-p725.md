# Paridade Produção — P725 — `Length * Int|Float` (as quatro combinações)

**Data:** 2026-07-13
**Passo:** `00_nucleo/materialization/typst-passo-725.md`
**Hash do commit (implementação):** `e99344271`.
**HEAD base:** `5fe90f9d3` (fim de P724, branch `Tekt`).
**Estado:** FECHADO — as quatro combinações `Length*Int`, `Int*Length`,
`Length*Float`, `Float*Length` implementadas em `eval_binary_op`, paridade
medida com o vanilla (incl. NaN → 0 e inf silencioso). 10 testes novos
verdes, workspace sem regressão, `crystalline-lint .` limpo. `cetz`
re-testado — avançou um bloqueio; o bloqueio actual (5º da cadeia) ficou
isolado com `file:line` para P726.

**Proveniência das medições (regra de proveniência, ADR-0108/regra
2026-07-05):** todas as medições deste relatório foram corridas em
**working tree não commitado** sobre `5fe90f9d3`, com exactamente estes
ficheiros alterados (`git diff HEAD --stat`):

```
 00_nucleo/diagnosticos/achados-adiados-cetz.md |   8 +-
 00_nucleo/prompts/rules/eval/ops.md            |  88 ++++++++++++++-
 01_core/src/rules/eval/operators.rs            |  31 +++++-
 01_core/src/rules/eval/tests.rs                | 143 +++++++++++++++++++++++++
```

---

## 1. Sonda (ADR-0108 — medir antes de decidir, cumprida)

### 1.1 Comportamento exacto no vanilla (medido)

Documento da sonda do passo (`/tmp/p725-mul.typ`), binário
`lab/typst-original/target/release/typst`, exit 0:

```
#(2.0 * 1pt)  #(1pt * 2.0)  #(2 * 1pt)  #(1pt * 2)  #(0 * 1pt)  #(-1 * 1pt)
→ 2pt 2pt 2pt 2pt 0pt -1pt
```

Sondas adicionais com `repr` (mesmo binário, `/tmp/p725-repr.typ`,
`/tmp/p725-nan.typ`, `/tmp/p725-nan2.typ`):

| Expressão | `repr` vanilla |
|---|---|
| `1pt * 2.5`, `2.5 * 1pt` | `2.5pt` |
| `3 * 2em` | `6em` |
| `0.5 * (1pt + 1em)` | `0.5pt + 0.5em` |
| `1pt * 2 + 3pt` | `5pt` |
| `1pt * -0.0` | `-0pt` |
| `1e308 * 1pt`, `1pt * 1e308`, `1pt * float.inf` | `float.inf * 1pt` — **inf propaga-se, sem erro** |
| `1em * float.inf` | `float.inf * 1em` |
| `1pt * float.nan`, `float.nan * 1pt` | `0pt` — **NaN → 0** |
| `1em * float.nan`, `(1pt + 1em) * float.nan` | `0pt` |

Nota sobre `1e308 * 1pt`: o `inf` vem da escala interna do vanilla
(127 raw/pt — `1e308 * 127` transborda `f64`), não do literal; o
cristalino usa escala 1.0 = 1pt (simplificação documentada ADR-0029,
`entities/layout_types.rs:705`), logo o limiar exacto de overflow difere
— divergência de representação interna aceite, só observável para
comprimentos > ~1.4e306 pt (absurdos; nenhum documento real).

### 1.2 Mecanismo exacto do vanilla (fonte, não assumido)

`lab/typst-original/crates/typst-library/src/foundations/ops.rs:238-243`:

```rust
(Length(a), Int(b))   => Length(a * b as f64),
(Length(a), Float(b)) => Length(a * b),
(Length(a), Ratio(b)) => Length(a * b.get()),   // scope-out — ver §2.2
(Int(a), Length(b))   => Length(b * a as f64),
(Float(a), Length(b)) => Length(b * a),
(Ratio(a), Length(b)) => Length(b * a.get()),   // scope-out — ver §2.2
```

O NaN → 0 **não** está em `ops.rs`: `Abs` e `Em` embrulham `Scalar`
(`layout/abs.rs:13`, `layout/em.rs:16`), e `Scalar::new` é
`Self(if x.is_nan() { 0.0 } else { x })`
(`typst-utils/src/scalar.rs:30-32`); toda a multiplicação de componentes
passa por ele (`scalar.rs:203-209`). Inf não é tocado. O `repr`
(`float.inf * 1pt` etc.) vem de `repr::format_float`
(`foundations/repr.rs:95-115`) — forma já replicada no cristalino
(`rules/eval/repr.rs:141-152`, P721).

### 1.3 Estado do cristalino antes

Mesmo documento da sonda, cristalino em `5fe90f9d3` (binário de P724):

```
/tmp/p725-mul.typ:1:3: error: cannot apply Mul to float and length [DEBUG-P724 lhs=2.0 rhs=1pt]  (exit 1)
```

Confirmado o diagnóstico de P724: zero braços `Mul` com `Length` em
`operators.rs` (só `Length / Int|Float`, P713). Consumidor real:
`canvas.typ:146-147,182-186` de `cetz` (`(x - offset) * length`, escala
de coordenadas; também `canvas.typ:124`: `length * (-y - offset-y)`).

## 2. Implementação (L0: `prompts/rules/eval/ops.md`, secção P725)

- **`01_core/src/rules/eval/operators.rs`** — dois braços com ordens em
  guarda partilhada, sobre `Length: Mul<f64>` já existente
  (`entities/layout_types.rs:801-806`), espelhando o agrupamento do
  vanilla (`ops.rs:238-243`):

  ```rust
  (BinOp::Mul, Value::Length(a), Value::Int(b)) | (BinOp::Mul, Value::Int(b), Value::Length(a)) =>
      Ok(Value::Length(sanitize_length_nan(a * b as f64))),
  (BinOp::Mul, Value::Length(a), Value::Float(b)) | (BinOp::Mul, Value::Float(b), Value::Length(a)) =>
      Ok(Value::Length(sanitize_length_nan(a * b))),
  ```

- **`sanitize_length_nan`** (helper local, mesmo ficheiro) — aplica
  `if x.is_nan() { 0.0 } else { x }` a `abs` e `em` do resultado:
  paridade do efeito observável de `Scalar::new` no vanilla (medido em
  §1.1). Fica **no braço do eval**, não em `Length::mul`, por disciplina
  um-bug-por-passo: `Length / Float` com NaN (P713) mantém o
  comportamento actual — divergência latente registada em
  `achados-adiados-cetz.md`.

- **L0 actualizado antes do código** (Regra de Ouro): secção P725 em
  `00_nucleo/prompts/rules/eval/ops.md` com mecanismo vanilla, tabela de
  medições, scope-outs, semântica e critérios de verificação; hash
  recalculado (`operators.rs` → `0c1a5927`).

### 2.1 Decisão medida — saneamento NaN incluído neste passo

O passo sugeria "multiplicação directa sobre Abs/Em". A sonda mediu que
o vanilla saneia NaN → 0 em **ambos** os componentes (`1em * NaN → 0pt`,
não `float.nan * 1em`), e o `repr` torna isso observável — paridade de
linguagem (ADR-0107), não mecânica. Custo: um helper de 3 linhas.
Incluído; registado no L0.

### 2.2 Scope-outs medidos, não assumidos

- **`Length * Ratio` / `Ratio * Length`** (`ops.rs:240,243` do vanilla):
  `Value::Ratio` não é produzível por sintaxe de utilizador no
  cristalino (`50%` produz `Value::Relative`, P469) — sem consumidor
  possível. Mesmo raciocínio de P713.
- **NaN por sintaxe de utilizador**: `float.nan`/`float.inf`/`calc.nan`
  não existem no eval cristalino (`float` é só `Type::Float`,
  `eval/mod.rs:1086`; `calc` expõe `inf` mas não `nan`,
  `stdlib/calc.rs:108`). O caminho NaN é hoje inalcançável a partir de
  documentos — coberto pelos testes unitários de `eval_binary_op`; o
  caminho inf é alcançável via `calc.inf` e tem cobertura E2E.

### 2.3 Testes (10 novos, fail-first confirmado: 10/10 FAILED antes)

Em `eval/tests.rs` (junto aos P713/P720/P722): as quatro combinações,
zero e negativo, `em` puro, misto `pt+em`, NaN → 0 (três formas), inf
propagado (abs e em), o padrão exacto do cetz (`(3 - 1) * 2.5pt`), e
E2E de `repr` com `calc.inf` (que também exercita o saneamento:
`1em * inf` → abs = `0 * inf` = NaN → 0 → ramo `em` do repr).
Regressão P713 coberta pelos testes existentes (nenhum teste existente
foi alterado).

## 3. Validação

- `cargo test -p typst-core --lib p725` → **10 passed, 0 failed**.
- `cargo test --workspace` → **exit 0, 0 failed** (core 3965 = 3955 de
  P724 + 10 novos; resto do workspace verde).
- `crystalline-lint --fix-hashes .` + `crystalline-lint .` → **0
  violations**.
- Documento da sonda: cristalino agora produz `2pt 2pt 2pt 2pt 0pt
  -1pt` — **conteúdo idêntico ao vanilla** (pdftotext).
- Sondas `repr` (`/tmp/p725-repr2.typ`): cristalino produz `2.5pt 2.5pt
  6em 5pt 0.5pt + 0.5em float.inf * 1pt float.inf * 1em -0pt` —
  idêntico às medições do vanilla em §1.1.

## 4. `cetz` re-testado — campos fixos (bloqueio 5 da cadeia)

```
$ time ./target/release/typst /tmp/p725-cetz.typ /tmp/p725-cetz.pdf
/tmp/p725-cetz.typ:<detached>: error: block(fill): espera Color, recebeu none
real    0m53,625s
Exit code: 1
```

O bloqueio 4 (`Mul` com `Length`) está resolvido — o documento avançou
para o bloqueio seguinte. O tempo de eval (~54s) é pré-existente (P724
mediu 54.5s no mesmo documento); não introduzido por este passo. Vanilla
compila o mesmo documento em 0.044s (referência).

Bloqueio actual isolado (fonte cristalina + pacote cetz 0.5.2):

- **Erro:** `block(fill): espera Color, recebeu none` —
  `stdlib/layout.rs:858-865` (`block(fill)` só aceita `Value::Color`).
- **Consumidor (file:line):** `canvas.typ:111,129` —
  `block.with(breakable: false)` invocado com `fill: background,
  stroke: stroke`, defaults `none` em `canvas.typ:25`.
- **Vanilla:** `block(fill: none)` e `block(stroke: none)` são válidos
  (none = sem fill/stroke).
- **Próximo erro previsível:** `extract_stroke` (`layout.rs:431-449`)
  também rejeita `none` — `block(stroke): espera Length / Color /
  Stroke, recebeu none` aparecerá assim que o fill for corrigido.
  Mesma classe de bug; candidato natural a P726 (verificar também
  `box`/`grid`/`table`, mesmos padrões em `layout.rs:321,1098` e
  `structural.rs:685`).
- Registado em `achados-adiados-cetz.md`.

## 5. ADRs

Grep a `00_nucleo/adr` em vigor pelos termos centrais (`Mul`, `Length`):
só ADR-0029 menciona `Length` (estrutura `{ abs: Abs, em: f64 }` —
seguida; nada sobre semântica de `Mul`). ADR-0107 (paridade com a
linguagem): o NaN → 0 e o `float.inf * 1pt` são observáveis via `repr`
— tratados como paridade, não mecânica (§2.1). ADR-0108: toda a
classificação foi precedida de medição (§1.1-1.3, §4); proveniência
registada no cabeçalho. ADR-0109 não se aplica (nenhuma atomização de
render tocada). Nada a atualizar nas ADRs.

## 6. Critério de fecho do passo

- [x] Sonda mínima completa — quatro combinações + zero + negativo +
  overflow/NaN confirmados no vanilla (§1.1).
- [x] Implementado e testado — 10 testes novos, fail-first confirmado.
- [x] Sem regressão em `cargo test --workspace` (P713 intacto, testes
  existentes inalterados).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — campos fixos registados (§4), bloqueio actual
  isolado para P726.
- [x] Grep às ADRs em vigor pelos termos centrais (`Mul`, `Length`) (§5).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p725.md`
  (hash do commit `e99344271`).
