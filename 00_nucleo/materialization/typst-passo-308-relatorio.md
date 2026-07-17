# Relatório — Passo 308 (`calc.erf` — paridade calc 41/41 = 100%)

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-308.md`
**Tipo declarado spec**: passo aditivo singular XS-S — 1 função stdlib
calc puramente nova com aproximação polinomial pura em `f64`. Nenhuma
ADR nova, nenhuma crate externa, nenhuma decisão arquitectural prévia.
**Hipótese adoptada**: **HA Caminho A** — Abramowitz & Stegun 7.1.26
(polinomial 5 coeficientes Horner) per recomendação spec §3; Caminho C
(`libm::erf`) seria mais preciso mas viola política agregada DEBT-libm
(ADR-0018). Diagnóstico P308a integrado **inline no L0** (opção α + γ
do spec §13).
**Baseline pós-P307**: 2 400 testes  →  **P308**: **2 409 testes**
(Δ = **+9 net**: +9 L1 stdlib `calc_erf`; 0 testes pré-existentes
removidos; 1 renomeação `calc_modulo_expoe_40_funcoes` →
`calc_modulo_expoe_41_funcoes` com assert 40→41 — actualização
numérica obrigatória, não conta como teste novo).
**Hash `entities/content.rs`**: `82d3c47d` preservado bit-exact
(**25º passo consecutivo** após P307 — 24º contado em §"Invariantes" do
spec porque P307 ainda preservou também).
**Hash L0 `stdlib.md`**: `dd3e2637` → `5fae4b55` → **`d4c214e1`**
(duplo drift L0: primeira passagem para registar `erf` + diagnóstico
inline; segunda para refinar caso de borda `±0` descoberto durante
implementação). **11 ficheiros `stdlib/*.rs` re-hashed duas vezes via
`crystalline-lint --fix-hashes`** — operação mecânica.
**Hash `export/*`**: todos preservados bit-exact (`mod.rs 54a226fa`,
`builder.rs 12d113a7`, `stream.rs 9acca994`, `images.rs ba5bcbb7`,
`fonts.rs c7d24b28`) — **1º passo consecutivo pós-P307** (P307 mexeu
export; P308 retoma a invariante).
**ADRs meta novas**: 0 (**15.ª vez consecutiva** anti-padrão
P273.17 §0 honrado).

---

## §1 — Sumário executivo

P308 adiciona **1 função** ao módulo `calc` — `erf` (função erro de
Gauss) — encerrando a paridade vanilla `calc` em **41/41 = 100%**.
Primeira categoria stdlib cristalina a atingir paridade categórica
absoluta.

**Caracterização**:

- **Passo singular**: 1 função apenas; aditivo puro. Zero variants em
  `Content`, zero alterações em `entities/`, zero alterações em
  consumers (eval pipeline já resolvia `calc.X` via Dict lookup desde
  P96.2).
- **Aproximação polinomial pura**: 6 constantes (`A1`..`A5` + `P`) +
  forma fechada em 4 linhas de Horner. Erro máximo absoluto
  **1,5 × 10⁻⁷** dentro do perfil ADR-0054 graded.
- **Reutilização total da infraestrutura P283/P306**: assinatura
  uniforme `native_X(ctx, args, world, file, fig)`, helpers partilhados
  `expect_no_named`, `err`, `coerce_to_f64`. Apenas 1 helper local
  novo: `erf_approx_as` (16 linhas, polinomial Horner).
- **DEBT-libm partilhado** (ADR-0018): `f64::exp` directo no helper
  com `#[allow(clippy::disallowed_methods)]` — futuro 7º sítio que
  migra agregadamente para `libm::*`.
- **3 short-circuits explícitos** preservam paridade vanilla nos
  limites:
  1. `NaN` → `Err` (divergência consciente vs vanilla `libm::erf(NaN)
     = NaN`; paridade convenção cristalina `guard_float`).
  2. `±∞` → `±1.0` (limite analítico; evita `0·∞ = NaN` na fórmula).
  3. **`±0` → `±0.0` (refino descoberto durante testes)**: a soma
     analítica `A1+A2+A3+A4+A5 = 1.0` recebe ruído de arredondamento
     `f64` (≈ 10⁻⁹) que se materializaria como `erf(0) ≈ 1e-9` em vez
     de `0` exacto. Short-circuit preserva paridade bit-exact com
     vanilla `libm::erf(0) = 0`.

**Resultado funcional**:

- **Stdlib calc passa de 40/41 (97,6%) para 41/41 (100%)** — primeira
  categoria stdlib cristalina a fechar.
- 9 testes unitários novos (identidade, valores conhecidos, simetria
  ímpar, limites finitos, infinito short-circuit, NaN guard, arity/
  tipos inválidos, named rejeitado, registo no módulo).
- 1 assert numérico actualizado (`calc_modulo_expoe_*_funcoes` 40→41).

**Resultado metodológico**:

- **Anti-padrão P273.17 §0 honrado 15.ª vez consecutiva**: zero ADRs
  meta promovidas. Sem candidato sequer plausível neste passo XS-S.
- **§8.7' A.0.0 template N=15 magnitude baixa-modesta** — "confirmação
  esperada" (P295 §10 categoria): spec foi factualmente correcta em
  todos os detalhes substantivos (Caminho A recomendação, vanilla usa
  `libm::erf`, ADR-0054 graded suficiente). Magnitude **não-degenerativa**
  per P299 §10.2 — passo singular trivial sobre infra madura.
- **Sub-padrão "diagnóstico inline L0" N=2 cumulativo** (P306
  expandiu §calc inline; **P308 integra diagnóstico P308a inline**
  evitando ficheiro `diagnosticos/` separado). Limiar tentativo
  ainda longe (N≥3 stable); adiamento de promoção formal natural.
- **Sub-padrão "refino descoberto durante implementação"** —
  short-circuit `±0` não estava na spec nem no diagnóstico inicial;
  emergiu do teste falhado `calc_erf_identidade_e_int_coerce`. L0
  actualizado para reflectir descoberta (drift L0 secundário). Caso
  legítimo de **L0 evoluir junto com L1** durante a Fase 5,
  re-propagando hash imediatamente.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=15 reaplica §8.7') | 1 função confirmada ausente via grep L1 (`erf` não estava em `calc.rs`); scaffolding P283/P306 disponível literal (`expect_no_named`, `err`, `coerce_to_f64`); magnitude **baixa-modesta** ("confirmação esperada") |
| A.0.0' (subdivisão) | **HA Caminho A** — Abramowitz & Stegun 7.1.26 per recomendação spec §3; rejeitado Caminho C (`libm`) por violação política agregada DEBT-libm |
| A.0 (ADR-0098) | ✅ `export/*` preservado bit-exact (**1º passo consecutivo pós-P307**); `content.rs 82d3c47d` preservado (**25º passo consecutivo**) |
| A.1 inventário | `make_calc_module` existente (~70 linhas P283 + 15 inserts P306); `coerce_to_f64`/`expect_no_named` reusáveis directamente; padrão `#[allow(clippy::disallowed_methods)]` em helper isolado per P283 §A.2 |
| A.2 decisão | Caminho A polinomial Horner inline + helper local `erf_approx_as` (16 linhas); short-circuits `NaN`/`±∞`/`±0` em `calc_erf` per ramo |
| A.3 integração | Dict insert paralelo P283/P306 (`dict.insert("erf", Value::Func(Func::native("calc.erf", calc_erf)))`); nome trivial sem kebab-case |
| A.4 emit | n/a — função não emite `Content`; produz `Value::Float` directo |
| A.5 bugs latentes | 9 cenários cobertos: identidade, valores conhecidos (3 pontos tabulares), simetria ímpar (5 pontos), limites finitos `±5`, infinito `±∞` short-circuit, NaN guard, arity 0/2/Str/Bool inválidos, named rejeitado, registo no módulo + contagem 45 entradas |
| A.5' anti-reflexão | **N=17 cumulativo** (P291-P308); refino `±0` descoberto Fase 5 — anti-padrão "ignorar descoberta tardia" rejeitado; L0 actualizado em vez de aceitar `erf(0) ≈ 1e-9` divergente |

---

## §3 — Materialização

### §3.1 — `01_core/src/engine/stdlib/calc.rs` — 1 função + 1 helper + 1 registo

**Mudanças principais**:

1. **Header docstring estendido**: linhagem P308 documentada (`P27`
   base → `P96.5` extracção → `P283` trig → `P306` aritmética
   inteira+combinatória → **`P308` função erro de Gauss — paridade
   calc 41/41 = 100%**).
2. **Doc-comment `make_calc_module`**: "40 funções + 4 constantes" →
   "41 funções + 4 constantes" com parágrafo dedicado P308 referenciando
   ADR-0054 graded e Caminho A Abramowitz & Stegun 7.1.26.
3. **1 inserção no Dict** (após `calc_root`, antes das constantes):
   ```rust
   // P308 — função erro de Gauss (paridade calc 41/41).
   dict.insert("erf".into(), Value::Func(Func::native("calc.erf", calc_erf)));
   ```
4. **1 função implementada** (`calc_erf`, ~24 linhas) com 3 short-circuits
   ordenados:
   ```rust
   pub(crate) fn calc_erf(_ctx, args, _world, _file, _fig) -> SourceResult<Value> {
       expect_no_named(&args.named)?;
       match args.items.as_slice() {
           [v] => {
               let x = coerce_to_f64(v, "calc.erf()")?;
               if x.is_nan() {
                   return err("calc.erf() valor é NaN");
               }
               if x.is_infinite() {
                   return Ok(Value::Float(if x > 0.0 { 1.0 } else { -1.0 }));
               }
               if x == 0.0 {
                   return Ok(Value::Float(x));   // preserva ±0
               }
               Ok(Value::Float(erf_approx_as(x)))
           }
           _ => err(format!("calc.erf() requer 1 argumento, recebeu {}", args.items.len())),
       }
   }
   ```
5. **1 helper local novo** `erf_approx_as` (16 linhas, polinomial Horner):
   ```rust
   #[allow(clippy::disallowed_methods)]
   fn erf_approx_as(x: f64) -> f64 {
       const A1: f64 =  0.254_829_592;
       const A2: f64 = -0.284_496_736;
       const A3: f64 =  1.421_413_741;
       const A4: f64 = -1.453_152_027;
       const A5: f64 =  1.061_405_429;
       const P:  f64 =  0.327_591_1;
       let sign = if x < 0.0 { -1.0 } else { 1.0 };
       let x_abs = x.abs();
       let t = 1.0 / (1.0 + P * x_abs);
       let poly = t * (A1 + t * (A2 + t * (A3 + t * (A4 + t * A5))));
       let y = 1.0 - poly * f64::exp(-x_abs * x_abs);
       sign * y
   }
   ```

### §3.2 — Os 3 short-circuits — racional explícito

| Short-circuit | Razão | Paridade vanilla |
|---|---|---|
| `NaN → Err` | Convenção cristalina `guard_float` / `coerce_to_f64` em todas as unárias (P283 §A.1). Divergência consciente vs `libm::erf(NaN) = NaN` | **divergência consciente documentada** |
| `±∞ → ±1.0` | Sem short-circuit a fórmula produziria `poly · exp(-∞) = 0 · 0 = 0` mas o produto intermédio `(-x_abs * x_abs)` overflowa para `-∞` antes do `exp`, e em alguns paths intermédios `0 · ∞ = NaN`. Limite analítico bem-definido | **bit-exact** |
| `±0 → ±0.0` | A soma analítica `A1+A2+A3+A4+A5 = 1.0` recebe ruído `f64` (~10⁻⁹). Sem short-circuit `erf(0) = 1 - 1.0... · 1 = ~ -10⁻⁹` em vez de zero | **bit-exact (paridade `libm::erf(0) = 0`)** |

### §3.3 — Reutilização total

| Construct | Origem | Reutilização P308 |
|---|---|---|
| `expect_no_named` helper | P71 (DEBT-24) | 1/1 — `calc_erf` |
| `err()` helper | P27 | 3 sítios (NaN, arity, named) |
| `coerce_to_f64` local | P27 / P283 | 1 sítio (entrada) |
| `Func::native(name, fn)` | P71 | 1 insert em `make_calc_module` |
| `#[allow(clippy::disallowed_methods)]` em helper isolado | P283 §A.2 padrão | 1 sítio (`erf_approx_as`) |
| Match-pattern inline | P27 padrão | 1 função |

**Zero variants novos**, **zero traits novas**, **zero novos módulos**.
Apenas 1 helper local privado (`erf_approx_as`, 16 linhas).

### §3.4 — `00_nucleo/prompts/engine/stdlib.md` — drift L0 deliberado duplo

**Primeira passagem** (`dd3e2637 → 5fae4b55`):
- Cabeçalho actualizado: 40→41 funções, Passos de origem +`P308`, ADRs
  relevantes +`ADR-0054`.
- Secção `Módulo calc — make_calc_module()`: contagem actualizada;
  marco "primeira categoria stdlib a fechar".
- Nova subsecção **§"Função erro (P308)"** com:
  - Diagnóstico inline P308a (vanilla = Caminho C `libm::erf`;
    cristalino = Caminho A A&S 7.1.26 com fórmula explícita).
  - Domínio e casos de borda (`±∞`, `NaN`, descrição inicial `±0`).
  - Tipos aceites + `expect_no_named`.
  - DEBT-libm partilhado (ADR-0018).
- Lista "Funções vanilla adiadas": `erf` riscado/fechado P308.
- Nota DEBT-libm actualizada: 6 → 7 sítios.
- Critérios de verificação: 13 casos canónicos `calc_erf`.

**Segunda passagem** (`5fae4b55 → d4c214e1`) — refino:
- Secção §"Domínio e casos de borda" reescrita para o caso `±0`:
  ```
  - x = ±0.0 → short-circuit retorna Float(x) (preserva sinal de
    zero). Sem este desvio, A&S 7.1.26 produziria ~1e-9 em vez
    de 0 exacto: a soma analítica a₁+a₂+a₃+a₄+a₅ = 1.0 recebe
    ruído de arredondamento f64 (≈ 1e-9) que se torna o resultado
    quando x = 0. Paridade vanilla libm::erf(0) = 0 mantida bit-exact.
  ```

### §3.5 — Zero alterações em outros sítios

| Componente | Pós-P308 |
|---|---|
| `Content` enum | **Inalterado** (não envolve elementos) |
| `entities/value.rs` | **Inalterado** |
| Eval pipeline | **Inalterado** — `eval_field_access` resolve `calc.erf` via Dict (P96.2) |
| Layouter / walks | **Inalterado** |
| `export/*` | **Inalterado bit-exact** — 5 ficheiros preservados (`mod.rs`, `builder.rs`, `stream.rs`, `images.rs`, `fonts.rs`); **1º passo consecutivo pós-P307** |
| L0 `engine/layout.md` | **Inalterado** |
| L0 `entities/content.md` | **Inalterado** |
| L0 `rules/stdlib.md` | **Actualizado duplamente** (drift L0 deliberado per protocolo + refino `±0`) |

---

## §4 — Testes

### §4.1 — `01_core/src/engine/stdlib/mod.rs` (+9 L1, +1 helper local)

**Helper de teste novo** (escopo limitado a P308 — não polui scaffolding
global):

```rust
fn approx_float_erf(v: Value, expected: f64) {
    match v {
        Value::Float(f) => assert!(
            (f - expected).abs() < 2e-7,
            "esperado ≈{expected} (tol 2e-7), obtido {f}",
        ),
        other => panic!("esperado Value::Float, obtido {other:?}"),
    }
}
```

Tolerância **2e-7** > envelope A&S 1.5e-7 (margem operacional).
Distinto do `approx_float` global (tol 1e-10) porque o limite real da
aproximação A&S é grosseiramente maior.

**Testes P308**:

| Teste | Verifica | Casos |
|---|---|---:|
| **`calc_erf_identidade_e_int_coerce`** | `erf(0) = 0` exacto, Int 0 coerce, Int 1 vs Float 1.0 bit-exact | 3 |
| **`calc_erf_valores_conhecidos`** | Valores tabulares A&S 7.1: `erf(0.5)≈0.5205`, `erf(1)≈0.8427`, `erf(2)≈0.9953` | 3 |
| **`calc_erf_simetria_impar`** | `erf(-x) = -erf(x)` em 5 pontos `[0.1, 0.5, 1.0, 1.7, 3.3]` | 5 |
| **`calc_erf_limites_finitos`** | `erf(5) ≈ 1.0` e `erf(-5) ≈ -1.0` (limite assintótico dentro da tolerância) | 2 |
| **`calc_erf_infinito_short_circuit`** | `erf(+∞) = 1.0` exacto, `erf(-∞) = -1.0` exacto | 2 |
| **`calc_erf_nan_err`** | NaN → Err (divergência consciente documentada) | 1 |
| **`calc_erf_arity_e_tipos_invalidos`** | Arity 0/2 → Err; tipos `Str`/`Bool` → Err | 4 |
| **`calc_erf_named_arg_rejeitado`** | `expect_no_named` em named arg `p:` | 1 |
| **`calc_erf_registado_no_modulo`** | `dict.get("erf")` é Func + `dict.len() == 45` (41 funcs + 4 consts) | 2 |

**Total casos cobertos**: **23 asserts** distribuídos por 9 funções de
teste.

**Teste `calc_modulo_expoe_40_funcoes` renomeado para
`calc_modulo_expoe_41_funcoes`** — actualização de assert numérico
inerente ao passo (40→41), não conta como teste novo. Comentário interno
actualizado para incluir "+1 P308 (erf)".

### §4.2 — Sem corpus E2E

P308 **não adiciona corpus E2E**:
- P306 adicionou 5 corpus (`calc-gcd/fact/binom/norm/trunc`) mas o
  harness `lab/parity/eval_parity` está bloqueado por bug pré-existente
  em `value_dto.rs` (não cobre `Value::Stroke`/`Gradient` pós-P262/P263).
- Adicionar `calc-erf.typ` seria simbólico sem ganho funcional. Adiado
  até desbloqueio do harness.

---

## §5 — Validação

### §5.1 — `cargo test -p typst-core --lib`

```
test result: ok. 2409 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out
```

Baseline pós-P307 = 2 400; delta = **+9 net visível** (todos P308 calc
erf unit tests; renomeação 40→41 não conta).

Os 6 `filtered out` em `typst-core lib` são testes de profundidade de
recursão (`recursao_*`) com pânico de stack em debug — **limitação
pré-existente** documentada desde P306 (eval/tests.rs:887), não
introduzida por P308.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

Executado **três vezes** durante P308:
1. Após primeira passagem L0 (`5fae4b55`) — propagou para 11 stdlib files.
2. Após implementação L1 + refino L0 (`d4c214e1`) — re-propagou para 11
   stdlib files.
3. Validação final — zero violations.

### §5.3 — Hashes pós-P308

| Ficheiro | Pós P308 |
|---|---|
| `entities/content.rs` (`@prompt-hash`) | **`82d3c47d` preservado bit-exact** (**25º passo consecutivo**) |
| `infra/export/mod.rs` (`@prompt-hash`) | `54a226fa` preservado (**1º consecutivo pós-P307**) |
| `infra/export/builder.rs` | `12d113a7` preservado |
| `infra/export/stream.rs` | `9acca994` preservado |
| `infra/export/images.rs` | `ba5bcbb7` preservado |
| `infra/export/fonts.rs` | `c7d24b28` preservado |
| `engine/layout/mod.rs` | inalterado |
| `engine/layout/cursor.rs` | inalterado |
| L0 `rules/stdlib.md` | **`dd3e2637` → `5fae4b55` → `d4c214e1`** (duplo drift deliberado) |
| 11× `rules/stdlib/*.rs` (`@prompt-hash`) | **`dd3e2637` → `5fae4b55` → `d4c214e1`** via `--fix-hashes` duplo (operação mecânica) |
| Outros L0 markdown | todos preservados |

A propagação dupla de hash a 11 ficheiros é puramente mecânica — cada
`crystalline-lint --fix-hashes` reescreve apenas a linha `@prompt-hash`
no header, sem tocar em comportamento.

### §5.4 — Regressões verificadas

- **P283** (17 testes trig/hyp/log): todos preservados — nenhuma função
  P283 foi tocada.
- **P306** (17 testes aritmética/combinatória): todos preservados —
  nenhuma função P306 foi tocada.
- **P27** (base 9 funções): todos preservados — registos inalterados em
  `make_calc_module`.
- **Eval pipeline** (`calc.X` via Dict): inalterado.
- **Outros 41 stdlib tests não-calc**: inalterados.
- **`calc_modulo_expoe_*_funcoes`**: assert numérico actualizado 40→41
  (mudança esperada do passo, não regressão).

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=15 magnitude baixa-modesta

| Passo | A.0.0 N | Magnitude | Categoria |
|---|---:|---|---|
| P293-P307 | 1-14 | varia (máxima até baixa-modesta) | mistura descoberta + confirmação |
| **P308** | **15** | **baixa-modesta** | **confirmação esperada** (per P295 §10) |

P308 é o **6º A.0.0 "confirmação esperada"** (P295, P299, P303, P305,
P306, **P308**). Spec foi factualmente correcta em todos os detalhes
substantivos:
- 1 função confirmada ausente pré-P308 (grep L1 zero matches).
- Scaffolding P283/P306 directamente reusável (`expect_no_named`,
  `err`, `coerce_to_f64`, padrão `#[allow]` em helper isolado).
- Caminho A recomendado pela spec §3 confirmou-se óptimo (rejeição
  fundamentada de B/C).
- Diagnóstico P308a inline confirmou vanilla = `libm::erf` per
  inspecção directa (1 linha em `lab/typst-original/.../calc.rs:537`).
- 3 short-circuits explícitos honram paridade vanilla nos limites.

**Não-degenerativa** per P299 §10.2 — magnitude baixa reflecte
**simplicidade real do passo** (1 função singular, zero decisões
arquitecturais, infra madura). Análogo a P305 magnitude moderada-baixa.

§8.7' N=15 **adiado** seguindo standard.

### §6.2 — Sub-padrão "Diagnóstico inline L0" — N=2 cumulativo

| Aplicação | Origem |
|---|---|
| N=1 P306 | §"calc P306" expandida com sub-secções inline em vez de criar `diagnosticos/diagnostico-calc-passo-306.md` separado |
| **N=2 P308** | **Diagnóstico P308a integrado inline no L0 §"Função erro"** (opção α + γ do spec §13) — evita ficheiro `diagnosticos/diagnostico-erf-passo-308a.md` separado |

**Limiar tentativo N=3 ainda não atingido**. Adiamento de promoção
formal natural. Reavaliar quando N=3 surgir.

**Justificação P308 para inline**: magnitude XS-S do passo não merece
ficheiro dedicado de diagnóstico. Inline preserva contexto leitor
(L0 + diagnóstico no mesmo lugar) e reduz fragmentação de documentação.

### §6.3 — Sub-padrão "Refino descoberto Fase 5" — N=1 inaugural

**Inaugural P308**: descoberta tardia do caso `±0` não estava nem na
spec, nem no diagnóstico inicial. Apareceu como falha do teste
`calc_erf_identidade_e_int_coerce` durante Fase 5.

**Resposta correcta** (aplicada P308):
1. Não relaxar o teste (manteria divergência `erf(0) ≈ 10⁻⁹` vs
   vanilla `0`).
2. Adicionar short-circuit no L1 (`if x == 0.0 { return ... }`).
3. **Actualizar o L0 imediatamente** para reflectir descoberta
   (segunda passagem `5fae4b55 → d4c214e1`).
4. Re-propagar hash via `crystalline-lint --fix-hashes`.

**Anti-padrão evitado**: aceitar divergência funcional silenciosa
"porque era erro de implementação, não da spec". A spec era correcta
em concepção (Caminho A A&S); o limite numérico do método é facto
matemático que o L0 deve documentar.

**Sub-padrão "L0 evolui durante Fase 5"** ainda não nomeado
formalmente — candidato a observação cumulativa em futuros passos
com refino tardio similar.

### §6.4 — Anti-padrão P273.17 §0 — **15 passos consecutivos**

**0 ADRs meta promovidas P293-P308**:

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P307 | 14 passos cumulativos | 0 |
| **P308** | **Diagnóstico inline N=2 + §8.7' N=15 + Refino Fase 5 N=1** | **0** |

**15.ª vez consecutiva** anti-padrão honrado. P308 confirma que
**passos singulares XS-S** (1 função apenas) também respeitam o
anti-padrão — disciplina não depende de magnitude do passo.

### §6.5 — ADR-0098 "single source of truth" — N=1 cumulativo pós-P307

Hash `export/*` preservado bit-exact pelo **1º passo consecutivo**
P308. P307 quebrou a contagem 23-consecutiva P282-P306 (ADR-0098
mexeu export para snapshot tests P307); P308 retoma a invariante
trivialmente (stdlib calc não toca render pipeline).

`entities/content.rs` hash `82d3c47d` mantém-se em **25 passos
consecutivos** (P282-P308, sem quebra em P307 que não tocou content
mas sim export).

### §6.6 — §8.6 "A.5' anti-reflexão" — N=17 cumulativo

P291-P308. P308 reconhece honestamente:
- Magnitude baixa-modesta reflecte simplicidade real, não ritualismo.
- Refino `±0` foi descoberta tardia legítima (não "erro de
  preparação" da spec — a soma `≈ 1.0` é um facto do método A&S
  conhecido na literatura mas que só se manifesta quando se mede).
- Sub-padrão "diagnóstico inline" N=2 ainda longe do limiar.
- Caminho A escolhido por política agregada (DEBT-libm ADR-0018), não
  por preguiça — Caminho C `libm` daria bit-exact mas adicionaria
  primeira crate de runtime numérico apenas para `erf`.

### §6.7 — Drift L0 deliberado **duplo** — variação P287/P291/P293/P306

P308 modifica L0 `rules/stdlib.md` **duas vezes** per Protocolo de
Nucleação §2:
1. **Drift principal** (Fase 2): nova secção `erf` + diagnóstico
   inline + actualização contagem 40→41 + remoção da lista
   "adiadas" + critérios de verificação.
2. **Drift secundário** (Fase 5 refino): caso `±0` adicionado à
   secção §"Domínio e casos de borda" após descoberta no teste.

`crystalline-lint --fix-hashes` propagou duas vezes:
- `dd3e2637 → 5fae4b55` (drift principal).
- `5fae4b55 → d4c214e1` (drift secundário).

A 22 ficheiros stdlib mecanicamente (11 × 2). Operação trivial; sem
alteração de comportamento.

Paralelo histórico:
- P287 (SmartQuote): drift L0 único.
- P291 (text.leading): drift L0 único.
- P293 (Curve geometry): drift L0 único.
- P306 (15 calc): drift L0 único + 11 fix-hashes.
- **P308 (calc.erf)**: drift L0 **duplo** + 22 fix-hashes (11 × 2).

**Novidade P308**: primeira ocorrência de drift L0 duplo no mesmo
passo. Padrão potencial "L0 evolui em duas fases" para passos com
refino Fase 5. Candidato a observação cumulativa.

---

## §7 — Cobertura vanilla vs cristalino

P308 atinge **paridade categórica absoluta calc**:

| Função vanilla | Antes P308 | Pós P308 |
|---|---|---|
| Base (9): `abs`/`pow`/`sqrt`/`floor`/`ceil`/`round`/`min`/`max`/`clamp` | ✓ (P27) | ✓ inalterado |
| Trig (7): `sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`atan2` | ✓ (P283) | ✓ inalterado |
| Hyperbolic (6): `sinh`/.../`atanh` | ✓ (P283) | ✓ inalterado |
| Exp/log (3): `exp`/`ln`/`log` | ✓ (P283) | ✓ inalterado |
| Aritmética inteira & partes (6) | ✓ (P306) | ✓ inalterado |
| Predicados (2): `even`/`odd` | ✓ (P306) | ✓ inalterado |
| Teoria nº (2): `gcd`/`lcm` | ✓ (P306) | ✓ inalterado |
| Combinatória (3): `fact`/`perm`/`binom` | ✓ (P306) | ✓ inalterado |
| Norma & raiz (2): `norm`/`root` | ✓ (P306) | ✓ inalterado |
| **`erf`** | ✗ ausente | **✓ implementada Caminho A** |

**Cobertura calc**: 40/41 → **41/41 (100%)**.

**Cobertura agregada stdlib pós-P308** (per diagnóstico P282 §A2.5):
- Calc: 97,6% → **100% (+2,4pp na categoria)** — **primeira categoria
  stdlib a fechar**.
- Stdlib agregada: ~54% → **~54,5%** (+0,5pp; calc é 1 de 9 categorias).

**Marco categórico** vs incremento numérico: ganho global é marginal
(+0,5pp) mas o ganho **simbólico/metodológico** é máximo — primeira
categoria stdlib cristalina a atingir paridade vanilla literal completa.
Estabelece precedente "100% é atingível" para futuras categorias.

---

## §8 — Frentes pendentes pós-P308

P308 fecha **integralmente** a categoria calc. **Frentes restantes em
calc**:

| Frente | Magnitude | Estado |
|---|---|---|
| **`Length`/`Angle`/`Decimal`/`digits`** em funções existentes | M+ | Requer tipo `Angle` (não existe ainda); bloqueia paridade *extensional* calc (não funcional) |

**Frentes não-calc** continuam idênticas a §8 P306/P307:
- **Math style functions** (`bb`/`cal`/`frak`/`italic`/`bold`/`mono`/
  `sans`/`scr`/`script`/`serif`/`sscript`/`upright`) — 12 funções;
  bloco coeso candidato a P309 ou similar.
- **Data parsing block** (`json`/`csv`/`yaml`/`toml`/`xml`/`cbor`/
  `read`) — 7 funções; requer I/O via World.
- **Footnote refinos** (P304.B/C/D, P305.C/D, P295.X) — sofisticados.
- **Text decorations refinements** — XS+ each, cosméticos.
- **Outras categorias** — Curve granular, Outline, etc.

---

## §9 — Decisão sobre P309

P308 fecha calc integralmente. P309 disponível para:

1. **Math style functions** (`bb`/`cal`/`frak`/`italic`/`bold`/`mono`/
   `sans`/`scr`/`script`/`serif`/`sscript`/`upright`) — 12 funções;
   bloco coeso paralelo P306 (escopo diferente: text style sobre
   conteúdo math). **Candidato natural** se for desejável fechar
   segunda categoria stdlib após calc.
2. **Data parsing block** (`json`/`csv`/...) — requer I/O via World;
   magnitude M+ porque cada formato é parser distinto.
3. **Outras categorias** — Footnote sofisticado, Curve granular,
   Outline, etc.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — "Confirmação esperada" honesta

P308 magnitude baixa-modesta reflecte **simplicidade real**:
- 1 função apenas, ~24 LOC + 16 LOC helper.
- Reutilização total do scaffolding P283/P306.
- Caminho A é receita textbook (Numerical Recipes; Abramowitz & Stegun
  1964 §7.1.26) — zero inovação.
- Política agregada DEBT-libm (ADR-0018) decidida muito antes.

Esta magnitude **não é** degenerescência §6.6 P295. É **caso legítimo
de passo singular trivial sobre infraestrutura madura**. Análogo a um
passo P291 (text.leading) onde a magnitude reflectia decisão pontual,
não complexidade arquitectural.

### §10.2 — Refino `±0` foi descoberta legítima Fase 5

O caso `erf(0) ≈ 10⁻⁹` é **conhecido na literatura** sobre A&S 7.1.26
(soma `1 - 1·exp(0) = 1 - 1.0...` onde `1.0...` é a soma analítica
ruidosa). Mas a spec inicial e o diagnóstico P308a não o destacaram —
foram redigidos focados na precisão *média* (1,5×10⁻⁷ erro máximo
absoluto), não nos casos de fronteira específicos.

**Decisão honesta**: o refino é **descoberta legítima do harness de
testes**, não erro da spec. O teste `calc_erf_identidade_e_int_coerce`
foi escrito antes da implementação com expectativa `Float(0.0)` exacto
(per paridade vanilla `libm::erf(0) = 0`); falhou; investigação rápida
revelou o limite do método; short-circuit é correcção mínima.

**Lição metodológica**: testes-primeiro **funcionam** mesmo em passos
XS-S. O teste do caso degenerado capturou um limite numérico que o
diagnóstico textual não capturou. Sub-padrão "TDD apanha limites
numéricos" candidato a observação futura.

### §10.3 — Divergência consciente `erf(NaN) → Err`

Vanilla `libm::erf(NaN) = NaN` (propagação IEEE 754 natural). Cristalino
P308 emite `Err("calc.erf() valor é NaN")`. Paralelo a outras funções
unárias cristalinas (`calc_sin`/`calc_cos` etc. usam `guard_float` que
mapeia NaN→Err).

**Decisão honesta**: divergência **funcional ténue** preservada por
consistência interna. Migração futura agregada para `libm::*`
(ADR-0018) pode reabrir a decisão — naquele ponto, a política
cristalina "NaN no domínio é erro do utilizador" poderia ser revogada
para paridade bit-exact.

### §10.4 — Caminho A vs Caminho C — política agregada acima de bit-exact

Caminho C (`libm::erf`) daria paridade bit-exact com vanilla. Caminho A
(A&S 7.1.26) tem erro absoluto até 1,5×10⁻⁷ em magnitudes pequenas e
relativo maior próximo de `|x| = ∞`. Para o consumidor típico
(impressão tipográfica de valores numéricos com 4-6 algarismos
significativos), nenhum dos dois é distinguível.

**Decisão honesta**: Caminho A não é "second-best por preguiça". É
**escolha de política**: adicionar `libm` como crate runtime exclusivamente
para `erf` violaria a granularidade agregada de ADR-0018 (`libm`
substituiria 6 sítios em P306 + 1 novo P308 simultaneamente, num passo
dedicado futuro). Para um consumidor que precise bit-exact `erf`, esse
passo agregado é o caminho.

### §10.5 — `dict.len() == 45` assert estrutural

O teste `calc_erf_registado_no_modulo` inclui `assert_eq!(dict.len(),
45, ...)`. Isto é **assert estrutural duplicado** com
`calc_modulo_expoe_41_funcoes` que conta 41 funcs + 4 floats = 45.

**Decisão honesta**: redundância intencional. O teste P308 verifica
**simultaneamente** que `erf` foi registado **e** que nenhuma outra
função foi acidentalmente perdida durante a edição. Em passo futuro
que adicione outra função, ambos os asserts precisam ser actualizados,
o que é benéfico (força revisão).

### §10.6 — Sem corpus E2E adicionado

P306 adicionou 5 corpus em `lab/parity/corpus/semantic/`. P308 **não**
adiciona `calc-erf.typ` porque:
- O harness `lab/parity/eval_parity` está bloqueado por bug
  pré-existente desde P262/P263 (`value_dto.rs` não cobre Stroke/
  Gradient).
- Adicionar corpus simbólico (sem harness verificável) seria
  documentação muda.

**Decisão honesta**: adiar corpus E2E até desbloqueio do harness.
Não é redução de cobertura — é reconhecimento que cobertura E2E
está bloqueada por dívida técnica separada.

### §10.7 — Reutilização vs criação

P308 reusa **5 helpers existentes** (`expect_no_named`, `err`,
`coerce_to_f64`, `SourceDiagnostic::error` via `err`, `Func::native`),
**1 padrão** (`#[allow(clippy::disallowed_methods)]` em helper isolado),
**0 traits novas**. **Apenas 1 helper local novo**: `erf_approx_as`
(16 linhas polinomial Horner).

Pattern confirmado: **passos singulares sobre infra madura são
puramente composicionais**. 5.ª confirmação cumulativa (P302, P303,
P305, P306, **P308**).

---

## §11 — Fecho

P308 fechado com:

- **+9 testes net** (todos L1 stdlib `calc_erf`) — todos verdes.
- **+1 helper de teste local** (`approx_float_erf` tol 2e-7).
- **+1 assert numérico actualizado** (`calc_modulo_expoe_*_funcoes`
  40→41; renomeação cosmética).
- **0 violations** no `crystalline-lint` (executado 3 vezes durante
  o passo).
- **0 drift remanescente** após `--fix-hashes` mecânico duplo em 11
  ficheiros stdlib (22 propagações totais).
- **Hash `entities/content.rs` preservado** bit-exact (**25º passo
  consecutivo**).
- **Hash `export/*` preservado** bit-exact (**1º consecutivo pós-P307**).
- **L0 `stdlib.md` actualizado duplamente** (`dd3e2637 → 5fae4b55 →
  d4c214e1`) — primeira passagem para registo de `erf` + diagnóstico
  inline; segunda passagem para refino `±0` descoberto na Fase 5.
- **0 ADRs meta novas** — **15.ª vez consecutiva** anti-padrão
  P273.17 §0 honrado.

**MARCO P308**:

- **Cobertura calc 40/41 → 41/41 (100%)** — **primeira categoria
  stdlib cristalina a fechar paridade vanilla literal completa**.
- **Stdlib agregada ~54% → ~54,5%** (ganho marginal numérico; ganho
  simbólico máximo — estabelece precedente "100% atingível").
- **§8.7' N=15 confirmação esperada** — sexta aplicação consecutiva
  de categoria honesta P295 §10 ("não-degenerativa") em passo
  singular XS-S.
- **Sub-padrão "diagnóstico inline L0" N=2 cumulativo** — limiar
  tentativo N=3 ainda não atingido; adiamento natural.
- **Sub-padrão "refino descoberto Fase 5" N=1 inaugural** —
  short-circuit `±0` emergiu de teste falhado, não da spec; L0
  actualizado em vez de relaxar teste.
- **Sub-padrão "drift L0 duplo no mesmo passo" N=1 inaugural** —
  primeira ocorrência cristalina; candidato observação cumulativa.
- **Reutilização total** — `make_calc_module` cresce de 40 inserts
  P306 para 41 inserts (1 novo); reuso integral dos helpers
  `coerce_to_f64`/`expect_no_named`/`err`.
- **3 short-circuits explícitos** (`NaN`/`±∞`/`±0`) — duas paridades
  bit-exact com vanilla + uma divergência consciente documentada.
- **DEBT-libm partilhado 7º sítio** — futura migração agregada
  ADR-0018 resolve `erf` juntamente com `pow`/trig/log existentes.

**Lição final**: P308 prova que **mesmo passos singulares XS-S
respeitam disciplina arquitectural completa** — Protocolo de Nucleação
em 6 fases foi seguido literal (incluindo Fase 1.5 diagnóstico inline +
Fase 3 trava humana fix-hashes + Fase 4 testes-falham-antes). A
tentação de "atalhar" um passo trivial (uma função, 24 LOC) foi
rejeitada — disciplina é independente de magnitude.

Adicionalmente, P308 inaugura dois sub-padrões metodológicos
candidatos (refino Fase 5 + drift L0 duplo) que **só emergem em passos
trabalhados com rigor**: passos atalhados nunca os teriam descoberto.
O marco categórico calc 41/41 = 100% é simbolicamente máximo mas
metodologicamente é apenas a consequência natural de 3 passos
consecutivos disciplinados (P27 → P283 → P306 → P308) sem nunca
quebrar a invariante de paridade vanilla observable.
