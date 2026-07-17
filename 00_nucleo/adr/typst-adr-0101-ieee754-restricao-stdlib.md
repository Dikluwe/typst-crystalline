# ⚖️ ADR-0101: Excepção IEEE 754 em stdlib — rejeita NaN/Inf via `guard_float`

**Status**: `EM VIGOR`
**Data**: 2026-05-20
**Passo promotor**: P309 (diagnóstico transversal) + P310 (formalização)
**Categoria**: Arquitectural / Política numérica
**Cross-ref**: ADR-0033 (paridade observable; excepção documentada),
              ADR-0018 (DEBT-libm; ortogonal mas informa decisão),
              ADR-0054 (perfil graded; precedente "divergência consciente")

---

## Contexto

O Passo 308 implementou `calc.erf` com três short-circuits, um dos
quais foi documentado como "divergência consciente":

> `erf(NaN) → Err` (cristalino) vs `libm::erf(NaN) = NaN` (vanilla).

O Passo 309 produziu auditoria transversal que revelou que a
divergência **não é local** a `erf`: é política transversal cristalina
expressa via helper central `guard_float` em `01_core/src/engine/stdlib/
calc.rs:803-806`. Simultaneamente, o L0 `00_nucleo/prompts/engine/
eval.md` documenta política oposta:

> Float → IEEE 754: NaN e Inf propagados silenciosamente (sem guarda)

Cristalino tem portanto **duas políticas IEEE 754 contraditórias em
sítios diferentes do código**, ambas pré-existentes e nunca
formalizadas:

| Categoria L1 | Política IEEE 754 | Sítios contados (P309) |
|---|---|---|
| **A** — Rejeita NaN/Inf | `guard_float` | 11 sítios em `calc.rs` |
| **B** — Propaga IEEE 754 | sem guarda | ~16 estruturais (operators + layout) |
| **C** — Validação por intervalo | `x ≤ 0 → Err` etc. | 24 sítios |
| **D** — Conversões | preserva ou satura | 16 sítios |

A política cristalina depende **do caminho sintáctico** que o
utilizador escolheu: `calc.exp(1000)` retorna `Err` (overflow → Inf →
`guard_float`), mas `5e200 * 5e200` no markup Typst retorna `Float(Inf)`
silenciosamente. Esta inconsistência foi a descoberta crítica de P309.

P309 enunciou 4 opções de política (A/B/C/D) com tabela
comparativa em 12 critérios. **Decisão humana pós-P309**: **Opção A —
Conformidade IEEE 754 total (status quo divergente)**.

Esta ADR formaliza essa decisão.

---

## Decisão

Política IEEE 754 cristalina é **categorial e consciente**:

### 1. Em `01_core/src/engine/eval/operators.rs` (camada B)

Operações binárias (`eval_binary_op`) e unárias (`eval_unary_op`)
sobre `Value::Float` **propagam IEEE 754 silenciosamente**:

```rust
(BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b))
```

- `5.0 / 0.5e-200` → `Float(Inf)` sem erro.
- `0.0 * f64::INFINITY` → `Float(NaN)` sem erro.
- `Float(NaN) == Float(NaN)` → `Bool(false)` (semântica IEEE 754).

**Caso especial preservado**: divisão por zero literal (`Int(0)` ou
`Float(0.0)`) retorna `Err("cannot divide by zero")` — paridade
vanilla `foundations/ops.rs::div`.

**Paridade vanilla total** neste sítio.

### 2. Em `01_core/src/engine/stdlib/calc.rs` (camada A)

Helper `guard_float` rejeita NaN e Inf no **resultado** de funções
matemáticas escalares:

```rust
fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan()           { err("resultado não é um número (NaN)") }
    else if f.is_infinite() { err("resultado é infinito") }
    else                    { Ok(Value::Float(f)) }
}
```

Invocado em 9 funções (~24 pontos de invocação cobertos por wrapper
`trig_op` partilhado):
- `calc.pow` (resultado Float)
- `calc.sqrt`
- `calc.sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`atan2` (via `trig_op`)
- `calc.sinh`/`cosh`/`tanh`/`asinh`/`acosh`/`atanh` (via `trig_op`)
- `calc.exp` (via `trig_op`)
- `calc.ln` / `calc.log`
- `calc.norm`
- `calc.root`

`calc.erf` (P308) tem política dedicada:
- `NaN` no input → `Err`
- `±∞` no input → short-circuit `±1.0`
- `±0` no input → short-circuit `±0.0`
- finite restante → A&S 7.1.26 polinomial

**Divergência consciente vanilla** neste sítio. Cobertura completa
da divergência em §"Divergências catalogadas vs vanilla" abaixo.

### 3. Em conversões Float→Int (camada D)

`calc.floor`/`calc.ceil`/`calc.round`/`calc.trunc`/`calc.quo` usam
`f.X() as i64` que tem semântica **saturating** Rust 1.45+ (RFC
2484):

- `NaN as i64 = 0`
- `f64::INFINITY as i64 = i64::MAX`
- `f64::NEG_INFINITY as i64 = i64::MIN`

Cristalino **não emite erro** nestes casos — comportamento delegado à
saturação implícita. Paridade vanilla parcial (vanilla usa
`libm::floor`/`ceil`/`round` + cast similar; mesma semântica de
saturação).

**Esta excepção não é objectivo de ADR-0101**. Reforço (`NaN as i64
→ Err`) fica adiado per P309 §10.6 como candidato futuro a sub-passo
de reforço pontual.

### 4. Em color constructors (camada D)

`native_oklab`/`oklch`/`linear_rgb`/`cmyk`/`hsl`/`hsv` aceitam
`Float`/`Int` via helper `as_f32` sem range check `[0.0, 1.0]`:

```rust
fn as_f32(v: &Value, name: &str) -> SourceResult<f32> {
    match v {
        Value::Float(f) => Ok(*f as f32),  // preserva NaN/Inf
        Value::Int(i)   => Ok(*i as f32),
        // ...
    }
}
```

Vanilla usa tipos validados (`RatioComponent` / `ChromaComponent`)
que validam range na construção. Cristalino diverge — aceita NaN/Inf
silenciosamente em componentes f32.

**Esta excepção também não é objectivo de ADR-0101**. Reforço (range
check em `oklab/...`) fica adiado per P309 §10.5.

---

## Divergências catalogadas vs vanilla

Tabela completa P309 §4.3 reproduzida para arquivo:

| Função | Cristalino | Vanilla | Divergência |
|---|---|---|---|
| `pow` | `guard_float` resultado (NaN+Inf→Err) | NaN→Err, Inf passa | **Cristalino +restritivo** em Inf |
| `sqrt` | `guard_float` resultado | resultado transparente | **+restritivo** em Inf+NaN |
| `sin`/`cos`/`tan` | `guard_float` resultado | transparente IEEE 754 | **divergência total** |
| `asin`/`acos` | range + `guard_float` | range apenas | **+restritivo** |
| `atan` | `guard_float` resultado | transparente | **divergência total** |
| `atan2` | `guard_float` resultado | transparente | **divergência total** |
| `sinh`/`cosh`/`tanh` | `guard_float` resultado | transparente | **divergência total** |
| `asinh` | `guard_float` resultado | transparente | **divergência total** |
| `acosh`/`atanh` | range + `guard_float` | range apenas | **+restritivo** |
| `exp` | `guard_float` resultado | NaN→Err, Inf passa | **+restritivo** em Inf |
| `ln` | x>0 + `guard_float` | x>0 + Inf→Err (NaN não) | **+restritivo** em NaN |
| `log` | x>0 + base + `guard_float` | x>0 + base + Inf+NaN→Err | **paridade quase total** |
| `root` | índice + ramo + `guard_float` | índice + ramo apenas | **+restritivo** |
| `norm` | `guard_float` | (não existe vanilla) | n/a (específico cristalino P306) |
| `erf` | NaN→Err + ±∞→±1 + ±0→±0 + A&S poly | `libm::erf` directo | **divergência total** input+algoritmo |

**Total**: 9 funções com divergência total (cristalino bloqueia, vanilla
passa); 4 funções com divergência parcial (cristalino +restritivo);
1 função com paridade quase total (`log`); 1 função
cristalino-exclusiva (`norm`).

**Operações binárias** (`eval_binary_op` vs `ops.rs`): **paridade
total** — ambos propagam IEEE 754 com divisão por zero como caso
especial.

---

## Racional

### Por que Opção A e não B/C/D?

P309 §8 tabelou implicações por opção. Decisão humana pondera:

1. **Custo zero implementação**: Opção A não toca código L1; outras
   opções exigem refactor M+ ou granular cirúrgico.
2. **Testes pré-existentes verdes preservados**: 12+ testes em
   `01_core/src/engine/stdlib/mod.rs` verificam `Err` para inputs
   degenerados (overflow, NaN result em pow/exp, etc.). Opção B
   exigiria refactor de 5-8 destes.
3. **Refino P308 mantém-se útil**: `erf` com short-circuit A&S +
   `±0` é trabalho substantivo que Opção B/D descartariam.
4. **Consistência interna stdlib**: as 12 funções `calc.*` que
   actualmente rejeitam NaN/Inf formam categoria coerente — usuário
   experimenta política uniforme.
5. **Erros explícitos > silent NaN/Inf**: filosofia preferida —
   reporta condição matemática indefinida em vez de propagar para
   output PDF.

### Por que documentar agora (não antes)?

A política existia de facto desde P27/P283 mas nunca foi formalizada.
Sem ADR explícita, futuros passos de paridade poderiam reverter
inadvertidamente. ADR-0101 codifica decisão tácita em decisão
explícita.

### Por que não revogar ADR-0033?

ADR-0033 (paridade funcional) permanece **regra geral** com escopo
literal:
- Layout/eval/output PDF mantêm paridade total.
- Stdlib funções matemáticas escalares são **excepção categorial
  documentada** via ADR-0101 + anotação cumulativa P310 em ADR-0033.

Paralelo: ADR-0054 (perfil graded) é "excepção" a paridade bit-exact
documentada em ADR-0033 §"Como verificar paridade". P310 segue
pattern.

---

## Consequências

### Positivas

- **Política IEEE 754 explícita**: documentação alinha-se com código;
  contradição declarativa eval.md vs stdlib.md resolvida por
  clarificação.
- **Erros explícitos no contexto matemático**: utilizadores de `calc.*`
  recebem erro útil em vez de NaN/Inf propagado.
- **Refino P308 preserved**: `erf` cristalino mantém-se distinto e
  defensável.
- **Consistência interna stdlib**: 12 funções `calc.*` com política
  uniforme.
- **Decisão tácita codificada**: passos futuros têm critério claro;
  reverter exigirá ADR explícita de revogação.
- **Custo de implementação zero**: P310 é puramente documental.

### Negativas

- **Divergência vanilla assumida explicitamente**: paridade observable
  não é total em stdlib. Usuário que confia no comportamento vanilla
  pode encontrar erro onde esperaria NaN/Inf.
- **`calc.erf` mantém divergência total vs vanilla**: P308 short-
  circuits + A&S 7.1.26 vs `libm::erf` transparente. Migração futura
  DEBT-libm não simplifica `calc.erf` (helper `erf_approx_as` +
  guards mantidos).
- **DEBT-libm não é simplificado**: migração agregada para `libm::*`
  futura mantém wrapper `guard_float`. 7 sítios + 1 `erf` continuam
  com `#[allow(clippy::disallowed_methods)]` se Opção A vigente
  quando migração ocorrer.
- **Float→Int saturating e color range checks adiados**: P309 §10.5/
  §10.6 ficam pendentes — possíveis refinos futuros pontuais fora do
  escopo Opção A.

### Neutras

- **Linguagem mensagens de erro**: `"resultado não é um número (NaN)"`
  é genérica — não indica qual função stdlib produziu. Refino possível
  (`guard_float_at(f, op_name)`) ortogonal a esta ADR.
- **Opção D recomendada por P309 §9 rejeitada**: utilizador escolheu
  conservadorismo (A) sobre cirurgia (D). Decisão registada como
  trade-off custo-implementação vs paridade.

---

## Alternativas consideradas

P309 §7 enunciou 4 opções; resumo da decisão:

| Opção | Política | Status decisão |
|---|---|---|
| **A** — Conformidade IEEE 754 total (status quo + ADR) | Manter `guard_float`; documentar em ADR | **ESCOLHIDA** |
| B — Conformidade IEEE 754 total (alinhar vanilla) | Remover `guard_float` em 9 funções trig/hyp; M+ refactor | Rejeitada (custo alto) |
| C — Manter status quo sem ADR | Não codificar decisão | Rejeitada (contradição declarativa persiste) |
| D — Híbrido por categoria (recomendada P309 §9) | Cirúrgico: paridade vanilla onde existe + reforço Cat D | Rejeitada (mais ambicioso que conservadorismo escolhido) |

**Racional contra D** (recomendação P309 rejeitada): paridade
parcial em 9 funções + reforço Cat D em 5+5 sítios introduz **3-5
sub-passos S** de implementação. Custo total M+ contra **custo
zero** Opção A. Trade-off priorizou conservadorismo no momento da
decisão.

**Opção A não bloqueia D futuro**: ADR-0101 pode ser revogada por
ADR-0101-R1 (futuro passo) se análise posterior reverter o trade-off.
Pattern ADR-0028 → ADR-0029 (pureza física) precedente.

---

## Plano de materialização (P310)

| Artefacto | Estado pós-P310 |
|---|---|
| ADR-0101 (este ficheiro) | **PUBLICADA** com status `EM VIGOR` |
| `00_nucleo/prompts/engine/stdlib.md` §"Política IEEE 754" | **ADICIONADA** com cross-ref ADR-0101 |
| `00_nucleo/prompts/engine/eval.md` §"Política IEEE 754" | **ADICIONADA** com cross-ref ADR-0101 |
| ADR-0033 §"Anotação cumulativa P310" | **ADICIONADA** com cross-ref ADR-0101 |
| Hashes propagados via `crystalline-lint --fix-hashes` | **EXECUTADO** |
| Código L1 | **INALTERADO** |
| Testes | **INALTERADOS** |

Magnitude P310: **S documental**.

---

## Não-objectivos desta ADR

**ADR-0101 não cobre**:

1. **Cat D Float→Int saturating** (`floor`/`ceil`/`round`/`trunc`/
   `quo` produzem `0` ou `±i64::MAX`). Reforço (`NaN as i64 → Err`)
   adiado per P309 §10.6.
2. **Cat D color range checks** (`oklab/...` sem `[0.0, 1.0]`
   validation). Reforço adiado per P309 §10.5.
3. **DEBT-libm** (ADR-0018). Ortogonal — migração futura mantém
   `guard_float`.
4. **Uniformização linguística mensagens** (PT vs EN em `eval/
   operators.rs:27` "cannot divide by zero" vs `calc.rs` PT).
   Ortogonal.
5. **Backdoor `native_float("NaN")`**: aceita string literal
   "NaN"/"inf" e produz `Value::Float(NaN/Inf)` — P309 §3.4 #15.
   Sítio único, refino futuro possível.

Estas 5 pendências ficam **fora do escopo Opção A**. Candidatos a
sub-passos pontuais de reforço futuros se desejado.

---

## Referências

- **P308** — `calc.erf` (divergência local que motivou auditoria).
  Relatório `00_nucleo/materialization/typst-passo-308-relatorio.md`.
- **P309** — diagnóstico transversal IEEE 754 (catálogo 67 sítios,
  comparação vanilla obrigatória todas categorias).
  `00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md`.
- **P310** — formalização Opção A (este ADR + drift L0 + anotação
  ADR-0033). `00_nucleo/materialization/typst-passo-310.md`.
- **ADR-0033** — Paridade funcional com vanilla; anotação cumulativa
  P310 §"Excepção IEEE 754 stdlib" referencia esta ADR.
- **ADR-0018** — `rustc_hash` + DEBT-libm; ortogonal mas relacionado
  (migração agregada futura mantém `guard_float`).
- **ADR-0054** — Perfil observacional graded; precedente "divergência
  consciente" documentada via ADR dedicada.
- **ADR-0093** — Meta-metodologia evolução ADRs (Pattern 2: anotação
  cumulativa em ADR pré-existente).
- `00_nucleo/prompts/engine/stdlib.md` §"Política IEEE 754" — secção
  L0 referenciando esta ADR.
- `00_nucleo/prompts/engine/eval.md` §"Política IEEE 754 — propagação
  silenciosa" — secção L0 referenciando esta ADR.
- `01_core/src/engine/stdlib/calc.rs:803-806` — implementação
  `guard_float`.
- `01_core/src/engine/eval/operators.rs:14-20` — comentário declarando
  propagação IEEE 754.
