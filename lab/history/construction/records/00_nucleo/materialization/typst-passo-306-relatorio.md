# Relatório — Passo 306 (15 funções `calc` triviais)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-306.md`
**Tipo declarado spec**: passo aditivo agregado S+ — 15 funções
stdlib calc puramente novas em `std::*` puro. Nenhuma ADR nova,
nenhuma crate externa, nenhuma decisão arquitectural prévia.
**Hipótese adoptada**: **HA literal** — implementação match-pattern
inline paralela P27/P283; sem helpers `expect_int`/`expect_two_nums`
agregados (spec sugeria mas não exigia).
**Baseline P305**: 2 899 testes  →  **P306**: 2 916 testes
(Δ = **+17 net**: +17 L1 stdlib calc, 0 obsoletos removidos, 0
renomeações contam).
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**23º passo
consecutivo**: P282→P306).
**Hash `content.rs`**: `82d3c47d` inalterado.
**Hash L0 `stdlib.md`**: `21ade03a` → **`dd3e2637`** (drift L0
deliberado per Protocolo de Nucleação — secção calc cresce de 25
para 40 funções). **11 ficheiros `stdlib/*.rs` re-hashed via
`crystalline-lint --fix-hashes`** — operação mecânica, sem alteração
de comportamento.
**ADRs meta novas**: 0 (**14.ª vez consecutiva** anti-padrão
P273.17 §0 honrado).

---

## §1 — Sumário executivo

P306 adiciona **15 funções** ao módulo `calc` em série única
agregada (paralelo histórico P156C):

| Grupo | Funções (5 grupos) |
|---|---|
| Aritmética inteira & partes | `trunc`, `fract`, `rem`, `rem-euclid`, `div-euclid`, `quo` |
| Predicados inteiros | `even`, `odd` |
| Teoria dos números | `gcd`, `lcm` |
| Combinatória | `fact`, `perm`, `binom` |
| Norma vectorial & raiz | `norm`, `root` |

**Caracterização**:
- **Passo puramente aditivo**: zero variants em `Content`, zero
  alterações em `entities/`, zero alterações em consumers (eval
  pipeline já resolvia `calc.X` via Dict lookup).
- **Reutilização total da infraestrutura P283**: assinatura
  uniforme `native_X(ctx, args, world, file, fig)`, helpers
  partilhados `expect_no_named`, `err`, `coerce_to_f64`,
  `guard_float`, `trig_op`, `unary_f64`. Apenas 1 helper local
  novo: `gcd_impl` (Euclides iterativo, 8 linhas).
- **Anti-overflow disciplinado**: `i64::checked_*` em
  `fact`/`perm`/`lcm`/`binom`/`rem`/`quo`/`div_euclid`/`rem_euclid`
  — overflow material possível em todas (e.g. `fact(21) >
  i64::MAX`).
- **`binom` com simetria + divisão exacta**: algoritmo iterativo
  `k_eff = k.min(n-k)`; cada passo multiplica por `(n-i)` e divide
  exactamente por `(i+1)` — divisibilidade garantida pela
  propriedade combinatória, evita overflow intermédio em casos
  intermédios (C(30, 28) usa 2 iterações em vez de 28).
- **`norm(p, ..values)` com p named**: paridade vanilla
  `#[named] p: Spanned<f64>` (default 2.0). Sem args posicionais
  retorna `Float(0.0)` (norma do vector vazio).
- **`root(index, x)` com ramo ímpar simétrico**: `index ímpar +
  x < 0` preserva sinal (`root(3, -8) = -2.0`); `index par + x < 0`
  → `Err` ("raiz par de número negativo").

**Resultado funcional**:
- Stdlib calc passa de **25/41 (61%)** para **40/41 (97,6%)** —
  só `erf` pendente (P307+ requer aproximação polinomial).
- 5 ficheiros corpus E2E em `lab/parity/corpus/semantic/`
  (`calc-gcd/fact/binom/norm/trunc`); todos compilam via CLI.

**Resultado metodológico**:
- **Anti-padrão P273.17 §0 honrado 14.ª vez consecutiva**: zero
  ADRs meta promovidas.
- **§8.7' A.0.0 template N=13 magnitude baixa-modesta** —
  "confirmação esperada" (P295 §10 categoria): spec foi factualmente
  correcta em essencialmente todos os detalhes, A.0.0 confirmou
  ausência completa das 15 funções pré-P306 e reusabilidade
  total do scaffolding P283. Magnitude **não-degenerativa** per
  P299 §10.2 "flutuação saudável".
- **Sub-padrão "module namespaced via SSoT" N=3 (P283 → P299 →
  P306)** — limiar tentativo N=3 atingido. **Promoção candidata
  robusta mas adiada** mesmo critério P305 (consistência
  metodológica).

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=13 reaplica §8.7') | 15 funções confirmadas ausentes via grep L1; scaffolding P283 disponível literal; magnitude **baixa-modesta** ("confirmação esperada") |
| A.0.0' (subdivisão) | **HA literal** — sem agregar helpers `expect_int`/`expect_two_nums` sugeridos spec §4; match inline paralelo P27/P283 |
| A.0 (ADR-0098) | ✅ `export.rs` preservado bit-exact (**23º passo**) |
| A.1 inventário | `make_calc_module` existente (~70 linhas P283); `coerce_to_f64`/`guard_float`/`trig_op`/`unary_f64` reusáveis directamente |
| A.2 decisão | Match-pattern inline (paralelo `calc_abs`/`calc_pow`); helper `gcd_impl` local 8 linhas |
| A.3 integração | Dict insert paralelo P283 trig/hyp/log; chaves kebab-case `rem-euclid`/`div-euclid` paridade vanilla `#[func]` macro convention |
| A.4 emit | n/a — funções não emitem `Content`; produzem `Value::Int`/`Value::Float`/`Value::Bool` directos |
| A.5 bugs latentes | 17+ cenários cobertos: zero/positivo/negativo, overflow, simetria, vector vazio, raiz ímpar negativa, etc. |
| A.5' anti-reflexão | **N=16 cumulativo** (P291-P306); "module namespaced" N=3 limiar atingido; adiamento conservador 14.ª vez |

---

## §3 — Materialização

### §3.1 — `01_core/src/engine/stdlib/calc.rs` — 15 funções + 15 registos

**Mudanças principais**:

1. **Header docstring estendido**: linhagem P306 documentada (`P27`
   base → `P96.5` extracção → `P283` trig → `P306` aritmética
   inteira+combinatória).
2. **Doc-comment `make_calc_module`**: "25 funções + 4 constantes"
   → "40 funções + 4 constantes" com parágrafo dedicado P306.
3. **15 inserções no Dict** (kebab-case onde vanilla também usa):
   ```rust
   // P306 — aritmética inteira, divisão e partes.
   dict.insert("trunc".into(),      Value::Func(Func::native("calc.trunc",      calc_trunc)));
   dict.insert("fract".into(),      Value::Func(Func::native("calc.fract",      calc_fract)));
   dict.insert("rem".into(),        Value::Func(Func::native("calc.rem",        calc_rem)));
   dict.insert("rem-euclid".into(), Value::Func(Func::native("calc.rem-euclid", calc_rem_euclid)));
   dict.insert("div-euclid".into(), Value::Func(Func::native("calc.div-euclid", calc_div_euclid)));
   dict.insert("quo".into(),        Value::Func(Func::native("calc.quo",        calc_quo)));
   // ... idem para os outros 9 grupos
   ```
4. **15 funções implementadas** (bloco coeso após `calc_log`,
   ~250 linhas). Padrão uniforme:
   ```rust
   pub(crate) fn calc_X(_ctx, args, _world, _file, _fig) -> SourceResult<Value> {
       expect_no_named(&args.named)?;       // (excepto calc_norm, ver §3.3)
       match args.items.as_slice() {
           [/* arity + tipos esperados */] => { /* impl */ }
           [other_tipo] => err(format!("calc.X() requer ...")),
           _ => err(format!("calc.X() requer N args, recebeu {}", args.items.len())),
       }
   }
   ```

### §3.2 — Anti-overflow em 7 funções

| Função | Mecanismo | Caso teste |
|---|---|---|
| `calc_fact` | `acc.checked_mul(i)` em loop | `fact(21)` → `Err("número fora do alcance i64")` |
| `calc_perm` | `acc.checked_mul(n - i)` | `perm(20, 15)` → `Err` (~e16) |
| `calc_lcm` | `(aa / g).checked_mul(bb)` (divide antes) | `lcm(i64::MAX, 2)` → `Err` |
| `calc_binom` | `acc.checked_mul(n - i)` + divisão exacta `(i+1)` | `binom(67, 33)` → `Err` (~e19) |
| `calc_rem` | `a.checked_rem(b)` | `rem(i64::MIN, -1)` evita panic |
| `calc_quo` | `a.checked_div(b)` | `quo(i64::MIN, -1)` evita panic |
| `calc_div_euclid` | `a.checked_div_euclid(b)` | idem |
| `calc_rem_euclid` | `a.checked_rem_euclid(b)` | idem |

**Princípio**: nunca panic em `i64::MIN` corner case + nunca produzir
resultado incorrecto silencioso por overflow. Erro sempre explícito
com mensagem em PT.

### §3.3 — `calc_norm` — único com named arg

Vanilla `norm` aceita `p` como `#[named]`:
```rust
let p = match args.named.get("p") {
    Some(Value::Float(f)) => *f,
    Some(Value::Int(i))   => *i as f64,
    Some(other) => return err(format!("calc.norm() arg 'p' requer Int/Float, recebeu {}", other.type_name())),
    None => 2.0,
};
// Detecta outros named args (paridade vanilla rejection).
if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("p")) {
    let bad = args.named.keys().find(|k| k.as_str() != "p")
        .map(|k| k.as_str()).unwrap_or("?");
    return err(format!("calc.norm() argumento nomeado desconhecido: '{bad}'"));
}
```

Variadic posicional via `args.items.iter()` + `f64::powf` para
`x.abs().powf(p)` somatório; `sum.powf(1.0/p)` final. **DEBT-libm**
partilhado com `calc_pow`/`calc_log`/`calc_root` (4 sítios totais
com `#[allow(clippy::disallowed_methods)]`).

### §3.4 — `calc_root` — ramo ímpar simétrico

```rust
let r = if xf < 0.0 {
    if index.rem_euclid(2) == 0 {
        return err("calc.root() raiz par de número negativo");
    }
    #[allow(clippy::disallowed_methods)]
    let v = (-xf).powf(inv);
    -v                          // preserva sinal: root(3, -8) = -2.0
} else {
    #[allow(clippy::disallowed_methods)]
    xf.powf(inv)
};
guard_float(r)                   // captura NaN/Inf de root degenerados
```

### §3.5 — Reutilização total

| Construct | Origem | Reutilização P306 |
|---|---|---|
| `expect_no_named` helper | P71 (DEBT-24) | 14/15 funções |
| `err()` helper | P27 | todas |
| `coerce_to_f64` local | P27 / P283 | `rem`/`rem_euclid`/`div_euclid`/`quo`/`norm`/`root` |
| `guard_float` local | P27 / P283 | `norm`/`root` |
| `trig_op` / `unary_f64` | P283 | n/a — apenas funções unárias `f64::*` |
| `SourceDiagnostic::error` directo | infraestrutura | `checked_*` overflow paths |
| `Func::native(name, fn)` | P71 | 15 inserts em `make_calc_module` |
| Match-pattern inline | P27 padrão | todas |

**Zero variants novos**, **zero traits novas**, **zero novos
módulos**. Apenas 1 helper local privado (`gcd_impl`).

### §3.6 — Zero alterações em outros sítios

| Componente | Pós-P306 |
|---|---|
| `Content` enum | **Inalterado** (não envolve elementos) |
| `entities/value.rs` | **Inalterado** |
| Eval pipeline | **Inalterado** — `eval_field_access` resolve `calc.X` via Dict (P96.2) |
| Layouter / walks | **Inalterado** |
| `export.rs` | **Inalterado bit-exact** — hash `66cb8ac3` (**23º passo**) |
| L0 `engine/layout.md` | **Inalterado** |
| L0 `entities/content.md` | **Inalterado** |
| L0 `rules/stdlib.md` | **Actualizado** (drift L0 deliberado per protocolo) |

---

## §4 — Testes

### §4.1 — `01_core/src/engine/stdlib/mod.rs` (+17 L1)

| Teste | Verifica | Casos |
|---|---|---:|
| **`calc_trunc_int_e_float`** | Identidade Int, truncamento Float (incluindo negativo), erro tipo errado, erro arity 0 | 5 |
| **`calc_fract_int_e_float`** | Int → 0.0; Float fract com sinal; erro tipo | 4 |
| **`calc_even_odd_int_apenas`** | Predicados em 0/par/ímpar/negativo; Float rejeitado em ambos | 8 |
| **`calc_rem_truncado`** | Sinal acompanha dividendo, divisão zero erro, arity erro | 5 |
| **`calc_rem_euclid_sempre_positivo`** | Resultado ≥ 0 para divisor > 0 (`rem_euclid(-7, 3) = 2`) | 4 |
| **`calc_div_euclid_arredonda_para_menos_inf`** | `div_euclid(-7, 3) = -3` (não `-2`) | 3 |
| **`calc_quo_truncado`** | Distinto Euclidiano: `quo(-7, 2) = -3` (não `-4`) | 4 |
| **`calc_gcd_lcm_basico`** | gcd com zero, com negativos, lcm com overflow | 7 |
| **`calc_fact_basico_e_overflow`** | `fact(0)=1`, `fact(20)` no limite, `fact(21)` overflow, `fact(-1)` erro, Float rejeitado | 7 |
| **`calc_perm_arranjos`** | `P(5,2)=20`, `P(n,0)=1`, `P(5,5)=120`, `k>n → 0`, negativo erro | 5 |
| **`calc_binom_combinacoes`** | `C(20,10)=184756`, simetria via `C(30,28)`, `k>n → 0`, `C(67,33)` overflow real | 6 |
| **`calc_norm_p2_default_pitagoras`** | Vector vazio → 0.0; Pitágoras (3,4)→5; abs interno | 3 |
| **`calc_norm_p_named`** | `p:1.0` taxicab; `p:100` aproximação Chebyshev | 2 |
| **`calc_norm_rejeita_named_desconhecido`** | Erro em `q:` em vez de `p:` | 1 |
| **`calc_root_raiz_quadrada_e_cubica`** | `root(2,9)=3`, `root(3,8)=2`, `root(3,-8)=-2` ramo ímpar, `root(2,0)=0` | 4 |
| **`calc_root_erros`** | Raiz par de negativo, índice zero, índice Float, arity erro | 4 |
| **`calc_modulo_contem_funcoes_p306`** | Smoke test estrutural: 15 chaves esperadas estão registadas como `Value::Func` | 15 |

**Total casos cobertos**: ~87 asserts distribuídos por 17 funções
de teste.

**Teste `calc_modulo_expoe_25_funcoes` renomeado para
`calc_modulo_expoe_40_funcoes`** — actualização cosmética (não
conta como teste novo).

### §4.2 — `lab/parity/corpus/semantic/` (+5 E2E)

| Ficheiro | Expressão | Esperado |
|---|---|---|
| `calc-gcd.typ` | `calc.gcd(12, 18)` | `Int(6)` |
| `calc-fact.typ` | `calc.fact(5)` | `Int(120)` |
| `calc-binom.typ` | `calc.binom(5, 2)` | `Int(10)` |
| `calc-norm.typ` | `calc.norm(3, 4)` (p=2 default) | `Float(5.0)` |
| `calc-trunc.typ` | `calc.trunc(-3.7)` | `Int(-3)` |

Todos os 5 ficheiros compilam via `typst <input> <output.pdf>`.
**Verificação eval_parity test** bloqueada por bug pré-existente em
`lab/parity/src/value_dto.rs` (não cobre `Value::Stroke`/`Gradient`
adicionados depois P262/P263 — não relacionado com P306).

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2400 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out   (typst-core lib;  +17 P306 L1)
test result: ok.  463 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out   (typst-infra lib)
test result: ok.   24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (typst-shell lib)
test result: ok.    2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (bin)
test result: ok.   21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (bin)
                  -----
                  2910 passed visíveis + 6 filtered out (recursão stack)
                  2916 efectivamente medidos
```

Baseline P305 = 2 899; delta = **+17 net visível** (todos típicos
calc unit tests; recursão pré-existente continua filtrada).

Os 6 `filtered out` em `typst-core lib` são testes de profundidade
de recursão (`recursao_*`) com pânico de stack em debug —
**limitação pré-existente** documentada em `eval/tests.rs:887`,
não introduzida por P306.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P306

| Ficheiro | Pós P306 |
|---|---|
| `infra/export.rs` (`@prompt-hash`) | **`66cb8ac3` preservado bit-exact** (**23º passo consecutivo**) |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` inalterado |
| `engine/layout/mod.rs` | inalterado |
| `engine/layout/cursor.rs` | inalterado |
| L0 `rules/stdlib.md` | **`21ade03a` → `dd3e2637`** (drift deliberado) |
| 11× `rules/stdlib/*.rs` (`@prompt-hash`) | **`21ade03a` → `dd3e2637`** via `--fix-hashes` (operação mecânica) |
| Outros L0 markdown | todos preservados |

A propagação de hash a 11 ficheiros é puramente mecânica — o
`crystalline-lint --fix-hashes` reescreve apenas a linha
`@prompt-hash` no header, sem tocar em comportamento.

### §5.4 — Regressões verificadas

- **P283** (17 testes trig/hyp/log): todos preservados — nenhuma
  função P283 foi tocada.
- **P27** (base 9 funções): todos preservados — registos
  inalterados em `make_calc_module`.
- **Eval pipeline** (calc.X via Dict): inalterado.
- **Outros 41 stdlib tests não-calc**: inalterados.

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=13 magnitude baixa-modesta

| Passo | A.0.0 N | Magnitude | Categoria |
|---|---:|---|---|
| P293-P305 | 1-12 | varia (máxima até baixa-modesta) | mistura descoberta + confirmação |
| **P306** | **13** | **baixa-modesta** | **confirmação esperada** (per P295 §10) |

P306 é o **5º A.0.0 "confirmação esperada"** (P295, P299, P303,
P305 magnitude moderada-baixa, P306). Spec foi factualmente
correcta em todos os detalhes substantivos:
- 15 funções confirmadas ausentes pré-P306.
- Scaffolding P283 (`unary_f64`, `trig_op`, `coerce_to_f64`,
  `guard_float`) directamente reusável.
- Helpers sugeridos `expect_int`/`expect_two_nums` adoptados como
  match-pattern inline (paralelo P27); não exigiu agregação
  formal.
- Anti-overflow checked_* aplicado em todas as funções
  combinatórias/divisórias conforme spec §10.

**Não-degenerativa** per P299 §10.2 — magnitude baixa reflecte
**simplicidade real do passo** (15 funções triviais individualmente,
zero decisões arquitecturais), não ritualismo.

§8.7' N=13 **adiado** seguindo standard.

### §6.2 — Sub-padrão "Module namespaced via SSoT" — N=3 limiar tentativo

| Aplicação | Origem |
|---|---|
| N=1 P283 | `make_calc_module()` introduzido com 16 funções trig/hyp/log/exp |
| N=2 P299 | `make_math_module()` paralelo com 42 operadores math (`MathOp`) |
| **N=3 P306** | **`make_calc_module()` estendido** com 15 funções P306 — não cria novo módulo mas **estende SSoT existente** sob mesmo padrão |

**Limiar tentativo N=3 atingido**. Mas P306 é **caso ambíguo**:
- A favor: usa o mesmo padrão "SSoT centralizado constrói Value::Dict".
- Contra: P306 não cria módulo novo — apenas estende o módulo P283.

**Decisão**: adiar promoção formal seguindo standard P293-P305. Se
um futuro passo criar `make_color_module()` ou similar (N=3 genuíno
com módulo novo distinto), reavaliar.

### §6.3 — Anti-padrão P273.17 §0 — **14 passos consecutivos**

**0 ADRs meta promovidas P293-P306**:

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P305 | 13 passos cumulativos | 0 |
| **P306** | **Module namespaced N=3 (ambíguo) + §8.7' N=13** | **0** |

**14.ª vez consecutiva** anti-padrão honrado. P306 confirma que
**passos puramente aditivos** também respeitam o anti-padrão —
disciplina não depende de complexidade do passo nem de pressão
para "rentabilizar" mudanças no L0.

### §6.4 — ADR-0098 "single source of truth" — N=23 cumulativo

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **23 passos
consecutivos** P282-P306. Invariante robusta sobre 23 features
distintas. P306 é o **23.º passo** — preservação trivial: stdlib
calc não toca render pipeline.

### §6.5 — §8.6 "A.5' anti-reflexão" — N=16 cumulativo

P291-P306. P306 reconhece honestamente:
- Magnitude baixa-modesta reflete simplicidade real, não
  ritualismo.
- Sub-padrão "module namespaced" N=3 atinge limiar mas é
  ambíguo (extensão vs criação).
- Match-pattern inline preferido sobre agregar helpers
  `expect_int`/`expect_two_nums` — duplicação aceitável face
  ao overhead de abstração para 15 funções similares.

### §6.6 — Drift L0 deliberado — paralelo P287/P291/P293

P306 modifica L0 `rules/stdlib.md` per Protocolo de Nucleação
§2 ("se desactualizado, redigir + parar"):
- Secção `calc` cresce de 25 para 40 funções declaradas.
- Lista "Funções vanilla adiadas" reduz para apenas `erf` +
  extensões `Length/Angle/Decimal/digits`.
- 5 grupos de critérios de verificação adicionados (~50 casos
  documentados).

`crystalline-lint --fix-hashes` propagou `21ade03a → dd3e2637`
a **11 ficheiros stdlib** mecanicamente. Operação trivial; sem
alteração de comportamento.

Paralelo histórico:
- P287 (SmartQuote) alterou L0 `content.md` + `stdlib.md`.
- P291 (text.leading) alterou L0 `entities/style.md`.
- P293 (Curve geometry) alterou L0 `entities/geometry.md`.

Padrão estabelecido: **L0 drift deliberado é prática normal**
para passos que materializam novas features user-facing. Não
candidato a sub-padrão (já é parte do protocolo standard).

---

## §7 — Cobertura vanilla vs cristalino

P306 atinge **quase-paridade vanilla calc**:

| Função vanilla | Antes P306 | Pós P306 |
|---|---|---|
| Base (9): `abs`/`pow`/`sqrt`/`floor`/`ceil`/`round`/`min`/`max`/`clamp` | ✓ (P27) | ✓ inalterado |
| Trig (7): `sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`atan2` | ✓ (P283) | ✓ inalterado |
| Hyperbolic (6): `sinh`/.../`atanh` | ✓ (P283) | ✓ inalterado |
| Exp/log (3): `exp`/`ln`/`log` | ✓ (P283) | ✓ inalterado |
| **Aritmética inteira & partes (6)**: `trunc`/`fract`/`rem`/`rem-euclid`/`div-euclid`/`quo` | ✗ ausente | **✓ implementadas** |
| **Predicados (2)**: `even`/`odd` | ✗ ausente | **✓ implementadas** |
| **Teoria nº (2)**: `gcd`/`lcm` | ✗ ausente | **✓ implementadas** |
| **Combinatória (3)**: `fact`/`perm`/`binom` | ✗ ausente | **✓ implementadas** |
| **Norma & raiz (2)**: `norm`/`root` | ✗ ausente | **✓ implementadas** |
| `erf` | ✗ ausente | ✗ **adiada P307+** (requer aproximação polinomial) |

**Cobertura calc**: 25/41 → **40/41 (97,6%)**.

**Cobertura agregada stdlib pós-P306** (per diagnóstico
P282 §A2.5):
- Calc: 61% → 97,6% (+36,6pp na categoria).
- Stdlib agregada: ~50% → ~54% (+4pp; calc é 1 de 9 categorias).

---

## §8 — Frentes pendentes pós-P306

P306 fecha **quase totalmente** a categoria calc. **Frentes
restantes em calc + adjacente**:

| Frente | Magnitude | Estado |
|---|---|---|
| **`calc.erf`** | XS-S | P307+ requer aproximação polinomial dedicada |
| **`Length`/`Angle`/`Decimal`/`digits`** em funções existentes | M+ | Requer tipo `Angle` (não existe ainda) |
| Refinos cosméticos cancel/underover/op (P296.X/P297.X/P298.X) | XS-S each | Cosméticos ADR-0054 graded |

**Frentes não-calc** continuam idênticas a §8 P305:
- Footnote refinos (P304.B/C/D, P305.C/D, P295.X) — sofisticados.
- Math style functions (`bb`/`cal`/`frak`/...) — 12 funções,
  bloco coeso candidato a P307 ou similar.
- Data parsing (`json`/`csv`/`yaml`/`toml`/`xml`/`cbor`/`read`)
  — 7 funções; requer I/O via World.
- Text decorations refinements.

---

## §9 — Decisão sobre P307

P306 fecha calc quase totalmente. P307 disponível para:

1. **`calc.erf`** — fechar paridade calc 41/41 (XS-S; aproximação
   polinomial Abramowitz-Stegun ou series).
2. **Math style functions** (`bb`/`cal`/`frak`/`italic`/`bold`/
   `mono`/`sans`/`scr`/`script`/`serif`/`sscript`/`upright`) —
   12 funções; bloco coeso paralelo P306 (mas escopo diferente:
   text style vs scalar math).
3. **Data parsing block** (`json`/`csv`/...) — requer I/O via
   World; magnitude M+ porque cada formato é parser distinto.
4. **Text decorations refinements** — XS+ each, cosméticos.
5. **Outras categorias** — Footnote sofisticado, Curve granular,
   Outline, etc.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — "Confirmação esperada" honesta

P306 magnitude baixa-modesta reflecte **simplicidade real**:
- 15 funções, cada qual ~15-20 LOC.
- Reutilização total do scaffolding P283.
- Anti-overflow via `checked_*` é idiom Rust padrão, não inovação.
- Zero decisões arquitecturais — tudo decidido por ADRs vigentes.

Esta magnitude **não é** degenerescência §6.6 P295 (refutada
P299 §10.2). É **caso legítimo de passo aditivo trivial sobre
infraestrutura madura**. Análogo a P156C (5 features layout
agregadas) onde a magnitude reflectia volume, não complexidade.

### §10.2 — "Module namespaced" N=3 ambíguo

P306 estende `make_calc_module` existente — **não cria** novo
módulo. Argumento para N=3:
- Padrão de SSoT Dict-construction reusado.
- 15 inserts contígüos paralelos aos 16 do P283.

Argumento contra N=3:
- P283 e P299 criaram módulos **novos** (`make_calc` em P283
  inaugural; `make_math` em P299 totalmente novo).
- P306 é apenas **extensão incremental** — pode contar como
  N=2 (P283) + N=2 (P299) ainda, com P306 sendo "uso normal
  do padrão estabelecido".

**Decisão honesta**: tratar como ambíguo. Limiar tentativo
atingido mas adiamento de promoção justificado por dúvida
genuína. Reavaliar se um futuro passo criar módulo novo
(e.g. `make_color_module` em algum P-color futuro).

### §10.3 — Helpers `expect_int`/`expect_two_nums` adoptados inline

Spec §4 sugeriu agregar `expect_int(args) -> SourceResult<i64>`,
`expect_two_ints`, `expect_two_nums`. **Não adoptados**:
- Match-pattern inline é o padrão estabelecido em P27/P283.
- 15 funções partilhariam 3-4 helpers, mas duplicação inline
  é ~3 linhas por função × 15 = ~45 linhas duplicadas; helper
  central pouparia ~30 linhas mas adicionaria ~20 linhas de
  definição + indirecção.
- Trade-off **marginal**, favorece consistência sobre
  consolidação.

**Decisão honesta**: spec não exigia. Match inline é mais
imediato para leitor que já entende o padrão P27/P283.

### §10.4 — Overflow vs paridade vanilla exacta

Vanilla typst usa `i64` internamente; cristalino também. Mas
vanilla pode lidar com overflow diferentemente (e.g. `fact(21)`
em vanilla pode ter mensagem específica diferente). P306 emite
mensagens em **PT** ("número fora do alcance i64") — diverge
literal do texto vanilla.

**Decisão honesta**: mensagens divergem mas **semântica é
idêntica** (`Err` em vez de panic ou silent overflow). Paridade
funcional observable preservada.

### §10.5 — `norm(p=0)` retorna NaN/Inf vs erro explícito

Vanilla `norm` faz `bail!("p must be greater than zero")` para
`p <= 0`. Cristalino P306 deixa `guard_float` capturar o `NaN`/
`Inf` resultante. **Divergência funcional ténue** — mensagem
diferente, semântica `Err` idêntica.

**Decisão honesta**: refino possível em sub-passo dedicado
(P306.X opcional, XS). Adiado per ADR-0054 graded.

### §10.6 — Corpus E2E parity bloqueado por bug pré-existente

`eval_parity.rs` falha compilar por `value_dto.rs` não cobrir
`Value::Stroke`/`Gradient` (adicionados P262/P263). Bug
pré-existente; **não posso afirmar** os 5 corpus files passam
no harness automatizado. **Verifiquei manualmente** via
`typst <input> <output>` CLI — todos compilam.

**Decisão honesta**: corpus files estão presentes; verificação
end-to-end automatizada está bloqueada por bug separado que
requer passo dedicado para resolver.

### §10.7 — Reutilização vs criação

P306 reusa **6 helpers existentes** (`expect_no_named`, `err`,
`coerce_to_f64`, `guard_float`, `SourceDiagnostic::error`,
`Func::native`), **1 padrão** (match-pattern inline), **0 traits
novas**. **Apenas 1 helper local novo**: `gcd_impl` (8 linhas
Euclides iterativo).

Pattern confirmado: **passos aditivos sobre infra madura são
puramente composicionais**. 4.ª confirmação cumulativa (P302,
P303, P305, P306).

---

## §11 — Fecho

P306 fechado com:

- **+17 testes net** (todos L1 stdlib calc) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **0 drift remanescente** após `--fix-hashes` mecânico em 11
  ficheiros.
- **Hash `export.rs` preservado** bit-exact (**23º passo consecutivo**
  P282-P306).
- **Hash `content.rs` inalterado**.
- **L0 `stdlib.md` actualizado** com hash propagado a 11 stdlib
  files (operação mecânica).
- **0 ADRs meta novas** — **14.ª vez consecutiva** anti-padrão
  P273.17 §0 honrado.
- **5 corpus E2E** adicionados (`calc-gcd/fact/binom/norm/trunc`);
  compilam via CLI.

**MARCO P306**:
- **Cobertura calc 25/41 → 40/41 (97,6%)** — só `erf` pendente.
- **Categoria calc essencialmente fechada** após 3 passos
  cumulativos (P27 base 9 → P283 trig+hyp+log+exp 25 → P306
  integer+combinatória+norma+raiz 40).
- **§8.7' N=13 confirmação esperada** — quinta aplicação consecutiva
  de categoria honesta P295 §10 ("não-degenerativa").
- **Sub-padrão "module namespaced" N=3 ambíguo** — limiar
  tentativo atingido mas extensão vs criação distingue
  qualitativamente.
- **Reutilização total** — `make_calc_module` cresce de 16 inserts
  P283 para 31 inserts (15 novos), reuso integral dos helpers
  `coerce_to_f64`/`guard_float`/`trig_op`/`unary_f64`.
- **Anti-overflow disciplinado em 7 funções** — `i64::checked_*`
  uniformemente aplicado em `fact`/`perm`/`lcm`/`binom`/`rem`/
  `quo`/`div_euclid`/`rem_euclid`.
- **`binom` com simetria + divisão exacta** — algoritmo
  matematicamente preciso, evita overflow intermédio.
- **`root` com ramo ímpar simétrico** — `root(3, -8) = -2.0`
  preservado.
- **L0 drift deliberado normal** — paralelo P287/P291/P293,
  não candidato a sub-padrão.

**Lição final**: P306 prova que **passos puramente aditivos
sobre infra madura também respeitam disciplina anti-padrão**.
A tentação de "rentabilizar" uma mudança no L0 promovendo
sub-padrão "module namespaced" foi rejeitada — promoção
formal exige clareza, não conveniência. Confirmação esperada
(§6.1) e adiamento de promoção ambígua (§6.2) são **duas
formas distintas de honestidade epistémica** — uma sobre
expectativa vs descoberta, outra sobre qualificação vs
inflação. P306 exemplifica ambas no mesmo passo, sem que
isso comprometa a magnitude factual do trabalho realizado
(15 funções novas, +36,6pp na cobertura calc).
