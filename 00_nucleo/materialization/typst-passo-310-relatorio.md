# Relatório — Passo 310 (formalizar Opção A da auditoria IEEE 754)

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-310.md`
**Tipo declarado spec**: passo documental S — formaliza decisão
política IEEE 754 (Opção A de P309 §7.1) via ADR nova + drift L0
duplo + anotação cumulativa ADR-0033. Sem código L1 tocado.
**Hipótese adoptada**: **HA Opção A** — escolha humana pós-P309
("Conformidade IEEE 754 total — status quo divergente"). Custo zero
de implementação; divergência vanilla **assumida explicitamente** em
ADR-0101.
**Baseline pós-P309**: 2 409 testes  →  **P310**: **2 409 testes**
(Δ = **0 net**: zero código L1 tocado; zero testes adicionados ou
removidos).
**Hash `entities/content.rs`**: `82d3c47d` preservado bit-exact
(**27º passo consecutivo**).
**Hash `export/*`**: todos preservados bit-exact (`mod.rs 54a226fa`,
`builder.rs 12d113a7`, `stream.rs 9acca994`, `images.rs ba5bcbb7`,
`fonts.rs c7d24b28`) — **3º passo consecutivo pós-P307**.
**Hash L0 `stdlib.md`**: `d4c214e1` → **`aa4ca50f`** (drift deliberado
P310; secção §"Política IEEE 754 — guard_float" adicionada).
**Hash L0 `eval.md`**: hash anterior → **`bf9002ad`** (drift
deliberado P310; secção §"Política IEEE 754 — propagação silenciosa"
adicionada). **Primeiro passo cristalino com drift L0 simultâneo em
dois prompts distintos**.
**21 ficheiros consumidores re-hashed via `crystalline-lint --fix-
hashes`**: 11× `stdlib/*.rs` → `aa4ca50f`; 10× `eval/*.rs` →
`bf9002ad`. Operação puramente mecânica, sem alteração de
comportamento.
**ADRs novas**: **1** — ADR-0101 (`Excepção IEEE 754 em stdlib —
rejeita NaN/Inf via guard_float`), categoria **Arquitectural /
Política numérica**. **NÃO conta como ADR meta** (ADR-0101 é decisão
política específica, não meta-metodológica).
**ADRs meta novas**: **0** (**17ª vez consecutiva** anti-padrão
P273.17 §0 honrado).
**ADRs pré-existentes anotadas**: 1 — ADR-0033 com secção
§"Anotação cumulativa P310 — Excepção IEEE 754 stdlib" anexada.
Status `EM VIGOR` preservado literal.

---

## §1 — Sumário executivo

P310 formaliza decisão política humana **Opção A** de P309 §7.1 —
"Conformidade IEEE 754 total (status quo divergente)". Três
artefactos documentais produzidos:

1. **ADR-0101** nova (~330 linhas, status `EM VIGOR`): formaliza
   política IEEE 754 categorial cristalina:
   - `eval`/layout/operators: IEEE 754 puro (paridade vanilla).
   - `stdlib` funções matemáticas escalares: rejeita NaN+Inf via
     `guard_float` (divergência consciente vanilla em 9 funções).
   - `calc.erf` (P308) mantém short-circuits dedicados.
   - Cat D (Float→Int saturating, color range checks) explicitamente
     fora do escopo Opção A; adiadas para reforços pontuais futuros.

2. **Drift L0 duplo deliberado**:
   - `stdlib.md` (`d4c214e1 → aa4ca50f`): secção §"Política IEEE
     754 — guard_float" adicionada após §"Helpers Internos" com
     cross-ref ADR-0101 + diagnóstico P309.
   - `eval.md` (anterior → `bf9002ad`): secção §"Política IEEE 754
     — propagação silenciosa" adicionada após §"Semântica Typst
     confirmada" com cross-ref ADR-0101 + diagnóstico P309.
   - **Primeira ocorrência cristalina** de drift L0 simultâneo em
     dois prompts distintos com cross-ref recíproco a uma ADR comum.

3. **Anotação cumulativa P310 em ADR-0033** (~50 linhas anexadas):
   secção §"Anotação cumulativa P310 — Excepção IEEE 754 stdlib"
   documenta excepção categorial sem revogar regra geral. Status
   `EM VIGOR` ADR-0033 preservado literal. Paralelo ADR-0054/0083/
   0091 pattern N=10+ cumulativo (Pattern 2 ADR-0093).

**Caracterização**:

- **Passo puramente documental**: zero código L1 tocado, zero testes
  alterados, zero variants/types/traits novos.
- **Custo zero implementação**: Opção A foi escolhida explicitamente
  por privilegiar conservadorismo sobre paridade. ADR-0101 codifica
  decisão tácita em decisão explícita.
- **Contradição declarativa resolvida**: pré-P310, L0 `eval.md`
  declarava IEEE 754 puro enquanto L0 `stdlib.md` declarava
  `guard_float` sem cross-ref. Pós-P310, ambos referenciam
  ADR-0101 que documenta a dualidade como decisão categorial.

**Resultado funcional**:

- ADR-0101 EM VIGOR (101ª ADR cristalina).
- 2 secções L0 novas com cross-ref recíproco.
- 1 anotação cumulativa em ADR-0033.
- 21 hashes propagados mecanicamente.
- 0 testes alterados; 2 409 verdes preservados.

**Resultado metodológico**:

- **Anti-padrão P273.17 §0 honrado 17.ª vez consecutiva**: zero ADRs
  meta promovidas (ADR-0101 é arquitectural/política, não
  meta-metodológica). Sub-padrões emergentes catalogados em §6 sem
  promoção formal a meta-ADR.
- **§8.7' A.0.0 template N=16 magnitude documental** — "decisão
  pós-diagnóstico transversal" (sub-padrão inaugural N=1).
- **Sub-padrão "Drift L0 simultâneo em prompts distintos" N=1
  inaugural** — primeira ocorrência cristalina (P306/P308 mexeram só
  `stdlib.md`; P310 mexe `stdlib.md` + `eval.md`).
- **Sub-padrão "Decisão política via ADR + anotação cumulativa
  paralela" N=1 inaugural** — ADR-0101 nova + anotação P310 em
  ADR-0033 (não revoga, anota excepção).

---

## §2 — Fase A (síntese)

P310 não tem Fase A clássica (não é descoberta de comportamento mas
materialização de decisão política tomada em P309).

| Secção | Veredicto |
|---|---|
| A.0.0 (N=16 reaplica §8.7') | Decisão já tomada P309 §7.1; P310 só materializa. Magnitude **documental**. |
| A.0.0' (subdivisão) | **HA Opção A** — escolha humana pós-P309 explícita |
| A.0 (ADR-0098) | ✅ `export/*` preservado bit-exact (**3º consecutivo pós-P307**); `content.rs 82d3c47d` preservado (**27º consecutivo**) |
| A.1 inventário | P309 diagnóstico produziu inventário completo (67 sítios catalogados); reusável literal |
| A.2 decisão | ADR-0101 nova (não anotação cumulativa) porque política é decisão arquitectural distinta de ADR-0033; anotação P310 em ADR-0033 documenta excepção |
| A.3 integração | Cross-ref recíproco entre `eval.md` ↔ `stdlib.md` ↔ ADR-0101 ↔ ADR-0033 |
| A.4 emit | n/a — sem código gerado |
| A.5 bugs latentes | Cat D pendências (Float→Int, color range, NaN backdoor) explicitamente fora do escopo; adiadas |
| A.5' anti-reflexão | **N=18 cumulativo** (P291-P310); P310 reconhece honestamente que recomendação P309 (Opção D) foi rejeitada por trade-off custo-implementação |

---

## §3 — Materialização

### §3.1 — `00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md` — ADR nova

**Estrutura** (paralelo ADR-0033, ADR-0054, ADR-0098):

| Secção | Conteúdo |
|---|---|
| Header | Status `EM VIGOR`, data 2026-05-20, passo promotor P309+P310, categoria Arquitectural / Política numérica, cross-ref ADR-0033 + ADR-0018 + ADR-0054 |
| Contexto | P308 divergência local → P309 auditoria transversal 67 sítios → decisão humana Opção A |
| Decisão | 4 sub-secções: (1) `eval`/operators IEEE 754 puro; (2) `stdlib/calc.rs` guard_float; (3) Cat D Float→Int saturating fora do escopo; (4) Cat D color sem range check fora do escopo |
| Divergências catalogadas vs vanilla | Tabela 15 funções `calc.*` reproduzida de P309 §4.3 para arquivo |
| Racional | 5 pontos numerados: custo zero, testes preservados, P308 erf mantém-se útil, consistência interna stdlib, erros explícitos > silent NaN/Inf |
| Consequências | Positivas (5), Negativas (4), Neutras (2) |
| Alternativas consideradas | Tabela A/B/C/D com status decisão; racional contra D (recomendação P309 rejeitada) |
| Plano de materialização | Tabela 7 artefactos pós-P310 |
| Não-objectivos | 5 pendências explicitamente fora do escopo |
| Referências | 10 cross-refs (P308, P309, P310, ADRs, ficheiros L1) |

**Total**: ~330 linhas.

### §3.2 — Drift L0 `stdlib.md` (`d4c214e1 → aa4ca50f`)

Secção nova inserida **após** §"Helpers Internos" e **antes** de
§"Sistema de Tipos — Regras de Promoção":

```markdown
## Política IEEE 754 — `guard_float` (ADR-0101 EM VIGOR)

`guard_float(f)` rejeita NaN e Inf no resultado de funções matemáticas
escalares (`calc.pow`, `calc.sqrt`, trig, hiperbólicas, log, exp, root,
norm, atan2). `calc.erf` (P308) tem política dedicada: NaN no input →
Err; ±∞ → short-circuit ±1.0; ±0 → short-circuit ±0.0.

[Política transversal cristalina: eval IEEE 754 puro vs stdlib rejeita
NaN+Inf — ADR-0101]
[Sítios afectados: 11 directos em calc.rs]
[Excepções não cobertas: Cat D Float→Int, color, backdoor]
[Catálogo completo: diagnostico-ieee754-passo-309.md]
```

Cross-ref recíproco com `eval.md` §"Política IEEE 754 — propagação
silenciosa".

### §3.3 — Drift L0 `eval.md` (→ `bf9002ad`)

Secção nova inserida **após** §"Semântica Typst confirmada" e
**antes** de §"Integração com comemo":

```markdown
## Política IEEE 754 — propagação silenciosa (ADR-0101 EM VIGOR)

Operações binárias (eval_binary_op) e unárias (eval_unary_op) sobre
Value::Float propagam IEEE 754 silenciosamente:
- 5.0 / 0.5e-200 → Float(Inf) sem erro.
- 0.0 / 0.0 → Err (caso especial divisor zero literal).
- 0.0 * f64::INFINITY → Float(NaN) sem erro.
- Float(NaN) == Float(NaN) → Bool(false) (IEEE 754).
- Float(NaN) < Float(5.0) → Bool(false) (idem).

[Não invoca guard_float — exclusivo de stdlib/calc.rs per ADR-0101]
[Política transversal cristalina: paridade vanilla aqui]
[Cross-refs: ADR-0101, stdlib.md §"Política IEEE 754", P309
 diagnóstico]
```

### §3.4 — Anotação cumulativa P310 em ADR-0033

Secção nova anexada **após** §"Referências" (final do ficheiro):

```markdown
## Anotação cumulativa P310 — Excepção IEEE 754 stdlib

**Data**: 2026-05-20.

P310 formaliza excepção categórica à paridade observable em funções
matemáticas escalares da stdlib (calc.*) via ADR-0101 EM VIGOR. [...]

[Status EM VIGOR ADR-0033 preservado literal]
[Decisão P310: Opção A de P309 §7.1]
[Paralelo ADR-0054 graded — precedente "divergência consciente
 documentada via ADR dedicada"]
[Sub-padrão N=1 inaugural — "Excepção categorial documentada via
 ADR dedicada"]
[Sub-padrão "Anotação cumulativa em vez de ADR nova" N+1 cumulativo]

Cross-references: ADR-0101, P308, P309, P310, stdlib.md, eval.md.
```

~50 linhas anexadas. Paralelo absoluto a anotações cumulativas P266/
P268.1/.../P273 em ADR-0054, ADR-0083, ADR-0091 (Pattern 2 ADR-0093).

### §3.5 — Propagação mecânica de hashes

`crystalline-lint --fix-hashes .` propagou **21 ficheiros**:

| Prompt | Hash novo | Consumers |
|---|---|---|
| `eval.md` | `bf9002ad` | 10× `01_core/src/rules/eval/*.rs` (bindings, closures, control_flow, markup, math, mod, modules, operators, rules, tests) |
| `stdlib.md` | `aa4ca50f` | 11× `01_core/src/rules/stdlib/*.rs` (assert, calc, figure_image, foundations, gradients, layout, mod, shapes, structural, text, transforms) |

Operação puramente mecânica — reescreveu apenas linha
`@prompt-hash` no header, sem tocar em comportamento.

### §3.6 — Zero alterações em código L1

| Componente | Pós-P310 |
|---|---|
| `Content` enum | **Inalterado** |
| `entities/value.rs` | **Inalterado** |
| `entities/layout_types.rs` | **Inalterado** (sítios Cat B identificados P309 §3.2 mantidos) |
| Eval pipeline (`eval/operators.rs`) | **Inalterado** (sítios Cat B mantidos) |
| Stdlib (`stdlib/calc.rs`) | **Inalterado** (11 sítios Cat A mantidos) |
| `entities/content.rs` | **Inalterado bit-exact** — hash `82d3c47d` (**27º passo**) |
| `export/*` | **Inalterado bit-exact** — 5 ficheiros preservados (**3º consecutivo pós-P307**) |
| Layouter / walks | **Inalterado** |

---

## §4 — Testes

### §4.1 — Sem testes adicionados

P310 é puramente documental — **zero código L1 tocado**. Suite
preserva 2 409 verdes.

### §4.2 — Validação `cargo test -p typst-core --lib`

```
test result: ok. 2409 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out
```

Idêntico a pós-P308. Confirma invariante "tests pré-existentes
inalterados" em ambos sentidos: 9 testes P308 `calc_erf` + 1 teste
P306 (`calc_modulo_expoe_41_funcoes`) + 2 399 outros pré-P308 todos
verdes.

### §4.3 — Sem corpus E2E adicionado

Idêntico a P308 e P309 — harness `lab/parity/eval_parity` bloqueado
por bug pré-existente.

---

## §5 — Validação

### §5.1 — `crystalline-lint .`

```
✓ No violations found
```

Executado **duas vezes** durante P310:
1. Após drift L0 duplo + ADR + anotação — propagou para 21 stdlib+eval
   files.
2. Validação final — zero violations.

### §5.2 — Hashes pós-P310

| Ficheiro | Pós P310 |
|---|---|
| `entities/content.rs` (`@prompt-hash`) | **`82d3c47d` preservado bit-exact** (**27º passo consecutivo**) |
| `infra/export/mod.rs` (`@prompt-hash`) | `54a226fa` preservado (**3º consecutivo pós-P307**) |
| `infra/export/builder.rs` | `12d113a7` preservado |
| `infra/export/stream.rs` | `9acca994` preservado |
| `infra/export/images.rs` | `ba5bcbb7` preservado |
| `infra/export/fonts.rs` | `c7d24b28` preservado |
| `rules/layout/mod.rs` | inalterado |
| `rules/layout/cursor.rs` | inalterado |
| L0 `rules/stdlib.md` | **`d4c214e1` → `aa4ca50f`** (drift deliberado) |
| L0 `rules/eval.md` | **anterior → `bf9002ad`** (drift deliberado) |
| 11× `rules/stdlib/*.rs` (`@prompt-hash`) | **`d4c214e1` → `aa4ca50f`** via `--fix-hashes` |
| 10× `rules/eval/*.rs` (`@prompt-hash`) | **anterior → `bf9002ad`** via `--fix-hashes` |
| ADR-0101 (novo) | publicada `EM VIGOR` |
| ADR-0033 (anotada) | status `EM VIGOR` preservado literal |

### §5.3 — Regressões verificadas

- **P308** (9 testes `calc_erf`): todos preservados — código `calc_erf`
  intocado.
- **P306** (17 testes aritmética/combinatória + 1 contagem
  actualizada): todos preservados.
- **P283** (17 testes trig/hyp/log): todos preservados.
- **Eval pipeline** (incluindo `eval_binary_op` IEEE 754 silent):
  inalterado.
- **Outros 41 stdlib tests não-calc**: inalterados.

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=16 magnitude documental

| Passo | A.0.0 N | Magnitude | Categoria |
|---|---:|---|---|
| P293-P309 | 1-15 | varia | mistura descoberta + confirmação + diagnóstico |
| **P310** | **16** | **documental** | **decisão pós-diagnóstico formalizada** |

P310 é **categoria nova** dentro do A.0.0 template — não é nem
"descoberta" nem "confirmação esperada" nem "diagnóstico". É
**"decisão"**. Spec foi factualmente correcta porque P309 já tinha
catalogado tudo; P310 só consolida em artefactos.

§8.7' N=16 **adiado** seguindo standard.

### §6.2 — Sub-padrão "Drift L0 simultâneo em prompts distintos" — N=1 inaugural

| Aplicação | Origem |
|---|---|
| **N=1 P310** | **`stdlib.md` + `eval.md` drift simultâneo** com cross-ref recíproco a ADR-0101 nova |

**Inaugural P310**. P306/P308 mexeram só `stdlib.md` (1 prompt). P307
mexeu prompts L3 (`export.md`/`pdf_emit.md`/etc.) mas não cruzados.
P310 é primeira ocorrência cristalina de **dois prompts L0 com cross-
ref recíproco** alterados num único passo.

Limiar tentativo N=3 longe; adiamento natural. Candidato a observação
cumulativa futura se padrão repetir.

### §6.3 — Sub-padrão "Decisão pós-diagnóstico transversal" — N=1 cumulativo (P309 →) P310

| Aplicação | Sequência |
|---|---|
| **N=1 P310** | P309 diagnóstico transversal 67 sítios → 4 opções → decisão humana Opção A → P310 materialização documental |

**Inaugural**. Paralelo histórico parcial: P156B (Layout diagnóstico)
→ P156G (Layout decisão), mas P156G foi material (não documental).
P309 → P310 é primeira ocorrência **documental-only** após
diagnóstico transversal.

Candidato observação cumulativa.

### §6.4 — Sub-padrão "Excepção categorial documentada via ADR dedicada" — N=1 inaugural

| Aplicação | Origem |
|---|---|
| **N=1 P310** | **ADR-0101** documenta excepção a ADR-0033 (paridade observable) sem revogar — anotação cumulativa P310 anexada |

**Inaugural P310**. Paralelo parcial: ADR-0054 (perfil graded) é
excepção a paridade bit-exact em ADR-0033. Mas ADR-0054 antecede a
formalização de ADR-0033 (foi criada P135); P310 é primeira excepção
**criada explicitamente como ADR distinta** após ADR-0033 formalizada
+ anotação cumulativa paralela na ADR original.

Candidato N=3 limiar tentativo se padrão repetir.

### §6.5 — Anti-padrão P273.17 §0 — **17 passos consecutivos**

**0 ADRs meta promovidas P293-P310**:

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P309 | 16 passos cumulativos | 0 |
| **P310** | **§6.2 + §6.3 + §6.4 (3 sub-padrões inaugurais N=1)** | **0** |

**17.ª vez consecutiva** anti-padrão honrado. ADR-0101 é
**arquitectural/política**, não meta-metodológica — não conta.

P310 confirma que **passos documentais** também respeitam o
anti-padrão — disciplina não depende de magnitude do passo nem do
tipo de artefacto produzido. Tentação de promover sub-padrões §6.2-
§6.4 a meta-ADR foi rejeitada porque N=1 está abaixo do limiar
tentativo N=3.

### §6.6 — ADR-0098 "single source of truth" — N=3 cumulativo

Hash `export/*` preservado bit-exact pelo **3º passo consecutivo**
P308→P309→P310. Invariante robusta sobre 3 features distintas:

- P308 — stdlib calc 1 função (não toca export).
- P309 — diagnóstico-primeiro (zero código tocado).
- **P310 — formalização documental** (zero código tocado).

Hash `content.rs` preservado em **27 passos consecutivos**.

### §6.7 — §8.6 "A.5' anti-reflexão" — N=18 cumulativo

P291-P310. P310 reconhece honestamente:

- **Recomendação P309 (Opção D) foi rejeitada** por trade-off custo-
  implementação. Decisão humana priorizou Opção A (conservadorismo)
  sobre cirurgia.
- **Cat D pendências adiadas explicitamente** — Float→Int saturating,
  color range checks, backdoor `native_float("NaN")` ficam fora do
  escopo. P310 não pretende ser "solução completa" para política
  IEEE 754 cristalina; é **decisão política específica** sobre
  manter status quo em stdlib funções escalares.
- **Sub-padrões §6.2-§6.4 N=1 ainda longe do limiar tentativo N=3**
  — adiamento de promoção natural.

### §6.8 — Drift L0 **duplo simultâneo** — variação P306/P308

P310 modifica **dois L0** simultaneamente:
- `stdlib.md` (`d4c214e1 → aa4ca50f`) — drift principal stdlib.
- `eval.md` (anterior → `bf9002ad`) — drift secundário eval.

`crystalline-lint --fix-hashes` propagou **uma única vez** mas
afectou **dois consumer-sets** distintos (11 + 10 = 21 ficheiros).

Paralelo histórico:
- P287 (SmartQuote): drift L0 único `content.md` + `stdlib.md`
  mas só `stdlib.md` propagou hashes (content.md tem outros
  consumers).
- P291 (text.leading): drift L0 único `entities/style.md`.
- P306 (15 calc): drift L0 único + 11 fix-hashes.
- P308 (calc.erf): drift L0 **duplo no mesmo prompt** + 22 fix-hashes.
- **P310 (formalizar Opção A)**: drift L0 **duplo em prompts
  distintos** + 21 fix-hashes (11+10).

**Distinção P308 vs P310**:
- P308 drift duplo = duas passagens **ao mesmo prompt** `stdlib.md`
  (refino Fase 5).
- P310 drift duplo = uma passagem em **cada um de dois prompts
  distintos** com cross-ref recíproco.

P310 inaugura modalidade nova; ambas são "drift L0 duplo" no nome
mas semanticamente distintas.

---

## §7 — Cobertura e marco

P310 **não altera cobertura funcional**:

- Stdlib calc: 41/41 (100%) — inalterado.
- Stdlib agregada: ~54,5% — inalterado.
- Cobertura global: inalterada.

**Marco P310**:

- **1ª decisão política IEEE 754 formalizada** em cristalino.
- **101ª ADR cristalina** (ADR-0101).
- **27º passo consecutivo** com `content.rs` hash inalterado.
- **3º passo consecutivo pós-P307** com `export/*` preservado.

---

## §8 — Frentes pendentes pós-P310

P310 fecha a decisão política IEEE 754. **Frentes pendentes
explicitamente adiadas em ADR-0101 §"Não-objectivos"**:

| Frente | Magnitude | Estado |
|---|---|---|
| Cat D Float→Int saturating reforço (`floor`/`ceil`/`round`/`trunc`/`quo`) | S-M | Candidato futuro |
| Cat D color range checks (`oklab`/`oklch`/`cmyk`/`hsl`/`hsv`) | S | Candidato futuro |
| Backdoor `native_float("NaN")` validação | XS | Candidato futuro |
| DEBT-libm migração agregada (ADR-0018) | M+ | Ortogonal; mantém `guard_float` |
| Uniformização linguística mensagens (PT vs EN) | XS | Ortogonal |

**Frentes não-IEEE 754** continuam idênticas a §8 P308/P309:
- Math style functions (12 funcs).
- Data parsing (json/csv/...).
- Footnote refinos.
- Etc.

---

## §9 — Decisão sobre P311

P310 fechado. P311 disponível para:

1. **Math style functions** (`bb`/`cal`/`frak`/...) — 12 funções;
   bloco coeso. **Candidato natural** após calc 41/41 + IEEE 754
   formalizada.
2. **Cat D reforços** pontuais (Float→Int Err, color range) — sub-
   passos S cada, requer ADR ou anotações.
3. **DEBT-libm migração agregada** — magnitude M+, primeira vez que
   `libm` entra em workspace.
4. **Outras categorias** stdlib.

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — Recomendação P309 rejeitada — assumido explicitamente

P309 §9 recomendou **Opção D** (híbrido por categoria) como
recomendação primária; Opção A como secundária. Decisão humana
escolheu **Opção A** — rejeição explícita da recomendação primária.

**Decisão honesta**: ADR-0101 §"Alternativas consideradas" documenta
trade-off custo-implementação vs paridade. P310 não esconde que
Opção D seria mais paritária; assume conservadorismo escolhido.
ADR-0101 não bloqueia revogação futura por ADR-0101-R1 se análise
posterior reverter o trade-off.

### §10.2 — Cat D pendências adiadas explicitamente

ADR-0101 §"Não-objectivos" lista 5 pendências fora do escopo:
1. Float→Int saturating reforço.
2. Color range checks.
3. DEBT-libm.
4. Backdoor `native_float("NaN")`.
5. Uniformização linguística.

**Decisão honesta**: P310 não pretende ser "solução completa". É
**decisão política específica** sobre 11 sítios `calc.*` cobertos
por `guard_float`. As outras 56 sítios catalogados em P309 §3.2-
§3.4 ficam intocados ou intocados-por-design.

### §10.3 — Sub-padrão "Excepção categorial documentada via ADR dedicada" N=1

P310 inaugura padrão: criar ADR distinta para excepção a regra geral
existente, anotando cumulativamente na ADR original sem revogar.

**Decisão honesta**: pattern é **defensável arquitecturalmente**
(separa decisão de "regra geral" de "excepção específica"). Mas N=1
está longe do limiar tentativo N=3. **Adiamento de promoção a
meta-ADR é natural**, não conservadorismo excessivo.

Paralelo parcial ADR-0054 (excepção a paridade bit-exact) reconhecido
mas distinto: ADR-0054 antecede formalização ADR-0033, P310 sucede
formalização. Pattern N=1 é genuinamente novo nessa modalidade.

### §10.4 — Drift L0 duplo modalidade nova

P306/P308 já fizeram "drift L0 duplo" mas em **modalidade diferente**
(duas passagens ao mesmo prompt em P308). P310 inaugura modalidade
"dois prompts distintos com cross-ref recíproco simultâneo".

**Decisão honesta**: nomeclatura "drift L0 duplo" é ambígua; refino
futuro poderia distinguir "drift L0 duplo intra-prompt" (P308) de
"drift L0 duplo inter-prompt" (P310). Adiado por agora; observação
em §6.8.

### §10.5 — Status `EM VIGOR` ADR-0033 preservado literal

Anotação cumulativa P310 em ADR-0033 documenta excepção sem revogar
nem alterar a regra geral. Status `EM VIGOR` preservado literal —
paralelo absoluto a anotações P266/P268.1/.../P273 em ADR-0054.

**Decisão honesta**: alternativa seria criar ADR-0033-R1 (revogação)
e ADR-0033-R2 (regra revista). Pattern ADR-0028 → ADR-0029 precedente.
Rejeitado porque **ADR-0033 regra geral não muda** — só ganha
excepção documentada. Pattern ADR-0093 Pattern 2 (anotação cumulativa)
aplicado.

### §10.6 — 17.ª anti-padrão consecutivo é "real"

P310 produziu **3 sub-padrões inaugurais N=1**. Anti-padrão P273.17
§0 honrado por adiamento de promoção formal a meta-ADR — não porque
não havia candidatos.

**Decisão honesta**: adiamento é **conservadorismo metodológico
disciplinado**, não preguiça. Sub-padrões N=1 são genuinamente
prematuros para promoção formal. Reavaliar quando N=3 atingido em
qualquer um dos 3 sub-padrões (§6.2, §6.3, §6.4).

### §10.7 — Custo zero implementação é facto, não ponto de venda

ADR-0101 §"Racional" lista "custo zero implementação" como primeiro
ponto. Pode soar como justificação ex post facto, mas é **facto
empírico** verificável: P310 modificou apenas docs; suite continua
2 409 verdes.

**Decisão honesta**: custo zero é **propriedade de Opção A**, não
mérito argumentativo. Opção B/D teriam custo M+ por design. Custo
zero entra na decisão como **legítimo trade-off**, não como
"trick".

---

## §11 — Fecho

P310 fechado com:

- **0 testes net** — zero código L1 tocado; suite 2 409 verdes
  preservada.
- **0 violations** no `crystalline-lint` (executado 2 vezes durante
  o passo).
- **0 drift remanescente** após `--fix-hashes` mecânico em 21
  ficheiros consumers (11× stdlib + 10× eval).
- **Hash `entities/content.rs` preservado** bit-exact (**27º passo
  consecutivo**).
- **Hash `export/*` preservado** bit-exact (**3º consecutivo pós-
  P307**).
- **L0 `stdlib.md` actualizado** (`d4c214e1 → aa4ca50f`) — secção
  §"Política IEEE 754 — guard_float" adicionada.
- **L0 `eval.md` actualizado** (→ `bf9002ad`) — secção §"Política
  IEEE 754 — propagação silenciosa" adicionada.
- **ADR-0101 publicada** EM VIGOR (~330 linhas).
- **ADR-0033 anotada cumulativamente** com ~50 linhas em §"Anotação
  cumulativa P310 — Excepção IEEE 754 stdlib"; status `EM VIGOR`
  preservado literal.
- **0 ADRs meta novas** — **17.ª vez consecutiva** anti-padrão
  P273.17 §0 honrado.

**MARCO P310**:

- **1ª decisão política IEEE 754 formalizada** em cristalino.
- **101ª ADR cristalina** (ADR-0101).
- **Contradição declarativa eval.md vs stdlib.md resolvida** por
  clarificação (não por mudança de código).
- **§8.7' N=16 categoria nova "decisão pós-diagnóstico"** —
  sétima categoria do template (anteriores: descoberta máxima,
  máxima-modesta, alta, alta-modesta, moderada, moderada-baixa,
  baixa-modesta).
- **Sub-padrão "Drift L0 simultâneo em prompts distintos" N=1
  inaugural** — primeira ocorrência cristalina (P310).
- **Sub-padrão "Decisão pós-diagnóstico transversal" N=1 inaugural**
  — sequência P309 → P310 documental-only.
- **Sub-padrão "Excepção categorial documentada via ADR dedicada"
  N=1 inaugural** — ADR-0101 + anotação P310 em ADR-0033.
- **Reutilização total** — pattern ADR-0093 Pattern 2 (anotação
  cumulativa) reusado N+1 cumulativo; ADR-0054 pattern "divergência
  consciente documentada via ADR dedicada" reusado.

**Lição final**: P310 prova que **decisões políticas explícitas
podem ser materializadas em passo documental único** sem código
tocado, **preservando completamente** invariantes de cobertura, hash
e testes. A tentação de "transformar P310 em passo material"
(implementar Cat D reforços oportunistamente) foi rejeitada — escopo
estrito ao que decisão Opção A exige é disciplina arquitectural
correcta. Pendências catalogadas explicitamente como adiadas ficam
para passos dedicados futuros, com magnitude e ADRs próprias.

A sequência **P308 (local) → P309 (transversal) → P310 (decisão)**
emerge como pattern cristalino reutilizável: divergência local
declarada → auditoria transversal factual → decisão política
documental. Sub-padrão observado §6.3 N=1; reavaliar em N=3 se
repetir em futuros casos.
