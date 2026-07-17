# Relatório — Passo 309 (auditoria conformidade IEEE 754)

**Data**: 2026-05-20
**Spec**: `00_nucleo/materialization/typst-passo-309.md`
**Tipo declarado spec**: passo diagnóstico-primeiro per ADR-0065 — sem
código tocado, sem ADR nova, sem L0 alterado, sem testes tocados.
Magnitude M documental (escopo α + comparação β escalou para M+ na
execução).
**Hipótese adoptada**: **escopo α (L1 completo) + comparação β
(obrigatória todas as categorias)** — escolhas humanas explícitas
pré-arranque. Magnitude declarada M passou a **M+** com comparação β.
**Baseline pós-P308**: 2 409 testes  →  **P309**: **2 409 testes**
(Δ = **0 net**: zero código L1 tocado; zero testes adicionados ou
removidos).
**Hash `entities/content.rs`**: `82d3c47d` preservado bit-exact
(**26º passo consecutivo** pós-P308 25º).
**Hash `export/*`**: todos preservados bit-exact (`mod.rs 54a226fa`,
`builder.rs 12d113a7`, `stream.rs 9acca994`, `images.rs ba5bcbb7`,
`fonts.rs c7d24b28`) — **2º passo consecutivo pós-P307**.
**Hash L0**: nenhum alterado — `stdlib.md d4c214e1`, `eval.md`
anterior preservados literal (drift L0 ficou para P310).
**ADRs novas**: **0** — diagnóstico é factual, não decisão.
**ADRs meta novas**: 0 (**16ª vez consecutiva** anti-padrão
P273.17 §0 honrado).
**Output**: 1 ficheiro publicado em
`00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md` (729
linhas; 67 sítios catalogados posicionalmente + comparação vanilla
sítio-a-sítio + 4 opções enunciadas + recomendação operacional
explícita).

---

## §1 — Sumário executivo

P309 produz auditoria transversal factual da política IEEE 754
cristalina em L1, motivada pela divergência local `erf(NaN) → Err`
documentada em P308 §10.3 e expandida em pergunta humana subsequente
("o cristalino segue IEEE 754?").

**Achado crítico**: o cristalino **já tem duas políticas IEEE 754
contraditórias** em sítios diferentes do código:
- `00_nucleo/prompts/engine/eval.md` declara IEEE 754 puro (propaga
  NaN/Inf silenciosamente).
- `00_nucleo/prompts/engine/stdlib.md` declara `guard_float` (rejeita
  NaN/Inf via Err).

A divergência **não é local a `erf`**: é política transversal expressa
via helper central `guard_float` em `01_core/src/engine/stdlib/calc.rs:
803-806`. **Política depende do caminho sintáctico** (markup
arithmetic vs `calc.*` function) — não da identidade do valor.

**Caracterização**:

- **Passo puramente diagnóstico**: zero código L1 tocado, zero testes
  alterados, zero variants/types/traits novos, zero ADRs criadas.
- **Catálogo factual posicional**: 67 sítios L1 catalogados por
  `ficheiro:linha:função` em 4 categorias (A/B/C/D).
- **Comparação vanilla obrigatória todas as categorias** (escolha β
  do utilizador): 15 funções `calc.*` cristalino vs vanilla
  sítio-a-sítio + operações binárias `ops.rs` vs `operators.rs` +
  color constructors `RatioComponent` vs `as_f32`.
- **4 opções de política enunciadas** (A/B/C/D) sem decisão; tabela
  comparativa em 12 critérios em §8.
- **Recomendação operacional explícita** (per ADR-0065): Opção D
  primária (híbrido por categoria); Opção C secundária. **Decisão
  fica para humano**.

**Resultado funcional**:

- 1 ficheiro factual publicado (`diagnostico-ieee754-passo-309.md`,
  729 linhas).
- 0 alterações de código.
- 0 alterações de testes.
- 0 hashes propagados (sem drift L0).

**Resultado metodológico**:

- **Anti-padrão P273.17 §0 honrado 16.ª vez consecutiva**: zero ADRs
  meta promovidas. Sub-padrão emergente "Auditoria de política
  transversal pós-divergência local isolada" N=1 inaugural catalogado
  em §6 sem promoção formal a meta-ADR.
- **§8.7' A.0.0 template N=14 categoria nova "diagnóstico
  transversal"** — primeira ocorrência cristalina de auditoria
  ampla L1 + comparação vanilla obrigatória.
- **Sub-padrão "Diagnóstico-primeiro factual após divergência
  declarativa detectada" N=4 cumulativo** — paralelo P156B
  (Layout), P154A (Model), P307a (export); P309 é o 4º caso. Limiar
  tentativo N=3 ultrapassado; adiamento de promoção justificado por
  critério estrito "divergência declarativa".

---

## §2 — Fase A (síntese — diagnóstico não tem Fase A clássica)

P309 é diagnóstico-primeiro per ADR-0065 — não há "implementação" que
exigisse Fase A. Mas o passo seguiu sequência análoga:

| Secção | Veredicto |
|---|---|
| A.0.0 (N=14 reaplica §8.7') | Auditoria L1 confirma 67 sítios distribuídos por 4 categorias; magnitude **M+** (comparação β escalou) |
| A.0.0' (subdivisão) | **HA factual** — sem inferência politica; só catalogação posicional + comparação vanilla obrigatória todas as categorias |
| A.0 (ADR-0098) | ✅ `export/*` preservado bit-exact (**2º consecutivo pós-P307**); `content.rs 82d3c47d` preservado (**26º consecutivo**) |
| A.1 inventário | Grep `guard_float`/`is_nan`/`is_infinite`/`is_finite` em `01_core/src/`; mapeamento de impls `std::ops::*` em `layout_types.rs`; inspecção sítio-a-sítio de `foundations.rs` color constructors |
| A.2 decisão | n/a — sem implementação a decidir |
| A.3 integração | n/a — sem código gerado |
| A.4 emit | n/a |
| A.5 bugs latentes | **Múltiplos** identificados: backdoor `native_float("NaN")`, Float→Int saturating silent, color sem range check — todos adiados como pendências em §10 do diagnóstico |
| A.5' anti-reflexão | **N=17 cumulativo** (P291-P309); P309 reconhece honestamente que recomendação Opção D pode ser rejeitada por trade-off custo-implementação |

---

## §3 — Materialização

### §3.1 — `00_nucleo/diagnosticos/diagnostico-ieee754-passo-309.md` (729 linhas)

**Estrutura final** (13 secções):

| Secção | Linhas | Conteúdo |
|---|---:|---|
| §1 Contexto e escopo | ~30 | Motivação P308 → P309; objectivo factual |
| §2 Política IEEE 754 declarada | ~50 | eval.md (propaga) + stdlib.md (rejeita) + contradição declarativa |
| §3 Inventário factual L1 | ~280 | 4 sub-secções com tabelas posicionais (Cat A/B/C/D) |
| §4 Comparação vanilla | ~120 | Política vanilla deduzida + sítios equivalentes + divergências sítio-a-sítio |
| §5 Política DEBT-libm | ~40 | 7+ sítios `#[allow(clippy::disallowed_methods)]` + intersecção com decisão NaN |
| §6 Inventário mensagens de erro | ~30 | 5 mensagens guard_float + inconsistência linguística PT/EN |
| §7 Opções de política | ~120 | 4 opções A/B/C/D enunciadas sem decisão |
| §8 Implicações por opção | ~30 | Tabela comparativa 12 critérios × 4 opções |
| §9 Recomendação operacional | ~50 | Opção D primária + Opção C secundária; A/B não recomendadas |
| §10 Pendências adiadas | ~20 | 7 decisões adiadas explicitamente para passos subsequentes |
| §11 Invariantes preservados | ~15 | Tabela com 6 invariantes |
| §12 Sub-padrão observado | ~20 | "Auditoria de política transversal pós-divergência local isolada" N=1 |
| §13 Fecho | ~25 | Recapitulação + conclusão substantiva |

### §3.2 — Categoria A — Rejeita NaN/Inf (11 sítios)

Catálogo completo posicional em §3.1 do diagnóstico. Resumo:

| Família | Funções | Sítios |
|---|---|---:|
| Power/root direct | `calc_pow`/`calc_sqrt`/`calc_root` | 3 |
| Trig + hyperbolic (via `trig_op`) | `calc_sin/cos/tan/asin/acos/atan/sinh/cosh/tanh/asinh/acosh/atanh/exp` | 13 callsites cobertos por 1 helper |
| Atan2 direct | `calc_atan2` | 1 |
| Log/Exp result | `calc_ln`/`calc_log` (resultado) | 2 |
| Norm result | `calc_norm` | 1 |
| Erf input + base | `calc_erf` (input NaN/Inf+0; P308) + `calc_log` base (`!is_finite()`) | 3 |

**Total**: 11 sítios físicos cobrindo ~24 pontos de invocação via
wrapper `trig_op` partilhado.

**Achado**: 0 sítios fora de `calc.rs` rejeitam NaN/Inf em L1
produção. `layout/tests.rs:11125` (`is_finite()` em teste) é único
sítio fora — filtrado por não ser produção.

### §3.3 — Categoria B — Propaga IEEE 754 (~16 estruturais + ~30 distributivos)

`eval/operators.rs`: 11 sítios estruturais — Add/Sub/Mul/Div Float
(3 ramos cada) + comparisons (Lt/Leq/Gt/Geq, 12 ramos coerce) +
Eq/Neq Float↔Int + Neg/Pos Float.

`entities/layout_types.rs`: 6 sítios estruturais — `Pt`/`Abs`/`Length`
impls `std::ops::Add/Sub/Mul<f64>/AddAssign`.

`engine/layout/**` e `rules/math/**`: distributivo (~30 ficheiros)
sem guarda IEEE 754 explícita.

`entities/color.rs`: storage f32 sem guarda.

**Confirmação empírica**: `5.0 * f64::INFINITY` em markup Typst
compila e produz `Float(Inf)` sem erro. `0.0 * f64::INFINITY`
produz `Float(NaN)`.

### §3.4 — Categoria C — Validação por intervalo (24 sítios)

20 sítios em `calc.rs` + 3 em `foundations.rs` + 1 em
`eval/operators.rs`.

**Sub-distinção**: 8/24 sítios delegam implicitamente NaN ao
`guard_float` posterior (compostos Cat A+C). 1 sítio único
(`calc_log` base linha 413) trata NaN/Inf **explicitamente** no input
via `is_finite()`.

### §3.5 — Categoria D — Conversões/coerções (16 sítios)

**Achado crítico**: `Float → Int as i64` em `calc_floor/ceil/round/
trunc/quo` (5 sítios) usa saturação implícita Rust 1.45+ (RFC 2484):
- `NaN as i64 = 0`
- `f64::INFINITY as i64 = i64::MAX`
- `f64::NEG_INFINITY as i64 = i64::MIN`

**Cristalino actual não emite erro** nestes casos. Vanilla é igualmente
unsafe (delegação `libm`). Reforço pontual seria correcção legítima
(Opção D §7.4 do diagnóstico).

**Achado adicional**: `native_float("NaN")` aceita string literal e
retorna `Value::Float(NaN)`. **Backdoor** para introduzir NaN/Inf no
eval pipeline.

`oklab`/`oklch`/`cmyk`/`hsl`/`hsv` aceitam f32 via `as_f32` **sem
range check** `[0.0, 1.0]`. Vanilla usa `RatioComponent` /
`ChromaComponent` validados.

### §3.6 — Comparação vanilla obrigatória todas as categorias

Tabela em §4.3 do diagnóstico documenta 15 funções `calc.*` sítio-
a-sítio:

| Tipo de divergência | Funções | N |
|---|---|---:|
| Divergência total (cristalino bloqueia, vanilla passa) | `sin/cos/tan/atan/atan2/sinh/cosh/tanh/asinh/erf` | 10 |
| Divergência parcial (cristalino +restritivo) | `pow/sqrt/asin/acos/acosh/atanh/exp/ln/root` | 9 |
| Paridade quase total | `log` | 1 |
| Cristalino-exclusiva | `norm` (não existe vanilla — P306) | 1 |

**Cat B operações binárias**: **paridade total** com vanilla
`foundations/ops.rs` (ambos propagam IEEE 754; divisão por zero é
caso especial).

**Cat D color**: divergência **estrutural** — cristalino usa f32
directo, vanilla usa tipos validados.

### §3.7 — Zero alterações em código L1

| Componente | Pós-P309 |
|---|---|
| `Content` enum | **Inalterado** |
| `entities/value.rs` | **Inalterado** |
| `entities/layout_types.rs` | **Inalterado** (Cat B mantidos) |
| Eval pipeline (`eval/operators.rs`) | **Inalterado** (Cat B mantidos) |
| Stdlib (`stdlib/calc.rs`) | **Inalterado** (11 sítios Cat A mantidos) |
| `entities/content.rs` | **Inalterado bit-exact** — hash `82d3c47d` (**26º passo**) |
| `export/*` | **Inalterado bit-exact** — 5 ficheiros preservados (**2º consecutivo pós-P307**) |
| L0 markdown | **Todos inalterados** (drift L0 ficou para P310) |

---

## §4 — Testes

### §4.1 — Sem testes adicionados

P309 é puramente diagnóstico — **zero código L1 tocado**. Suite
preserva 2 409 verdes (idêntico a pós-P308).

### §4.2 — Validação `cargo test -p typst-core --lib`

```
test result: ok. 2409 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out
```

Confirmação invariante "tests pré-existentes inalterados" — incluindo
9 testes P308 `calc_erf` + 1 P306 (`calc_modulo_expoe_41_funcoes`).

### §4.3 — Sem corpus E2E adicionado

Idêntico a P308 — harness `lab/parity/eval_parity` bloqueado por
bug pré-existente em `value_dto.rs` desde P262/P263.

---

## §5 — Validação

### §5.1 — `crystalline-lint .`

```
✓ No violations found
```

Executado **uma vez** durante P309 — confirmação final que diagnóstico
não introduziu drift (zero código tocado, zero L0 tocado).

### §5.2 — Hashes pós-P309

| Ficheiro | Pós P309 |
|---|---|
| `entities/content.rs` (`@prompt-hash`) | **`82d3c47d` preservado bit-exact** (**26º passo consecutivo**) |
| `infra/export/mod.rs` (`@prompt-hash`) | `54a226fa` preservado (**2º consecutivo pós-P307**) |
| `infra/export/builder.rs` | `12d113a7` preservado |
| `infra/export/stream.rs` | `9acca994` preservado |
| `infra/export/images.rs` | `ba5bcbb7` preservado |
| `infra/export/fonts.rs` | `c7d24b28` preservado |
| `engine/layout/mod.rs` | inalterado |
| `engine/layout/cursor.rs` | inalterado |
| L0 `rules/stdlib.md` | `d4c214e1` preservado (drift ficou para P310) |
| L0 `rules/eval.md` | anterior preservado (drift ficou para P310) |
| 11× `rules/stdlib/*.rs` | `d4c214e1` preservados |
| 10× `rules/eval/*.rs` | anteriores preservados |

### §5.3 — Regressões verificadas

- **P308** (9 testes `calc_erf`): todos preservados.
- **P306** (17 testes aritmética/combinatória + 1 contagem):
  todos preservados.
- **P283** (17 testes trig/hyp/log): todos preservados.
- **Eval pipeline**: inalterado.
- **Outros stdlib tests**: inalterados.

---

## §6 — Padrões metodológicos

### §6.1 — §8.7' "A.0.0 template" — N=14 categoria nova "diagnóstico transversal"

| Passo | A.0.0 N | Magnitude | Categoria |
|---|---:|---|---|
| P293-P308 | 1-13 | varia | mistura descoberta + confirmação + diagnóstico local |
| **P309** | **14** | **M+** | **diagnóstico transversal** (categoria nova) |

P309 é **categoria nova** dentro do A.0.0 template — não é
"descoberta máxima" nem "confirmação esperada" nem "diagnóstico
local". É **"diagnóstico transversal pós-divergência local"** —
auditoria ampla L1 motivada por pergunta humana após divergência
isolada (P308 `erf(NaN) → Err`).

§8.7' N=14 **adiado** seguindo standard. Categoria nova
"diagnóstico transversal" candidata a observação cumulativa futura
se padrão repetir.

### §6.2 — Sub-padrão "Auditoria de política transversal pós-divergência local isolada" — N=1 inaugural

| Aplicação | Sequência |
|---|---|
| **N=1 P309** | **P308 divergência local (`erf(NaN) → Err`)** → pergunta humana → P309 auditoria transversal 67 sítios + 4 opções + recomendação |

**Inaugural P309**. Sub-padrão observado pela primeira vez. Pattern:

1. Passo material implementa divergência local declarando-a
   "consciente".
2. Pergunta humana posterior expõe que divergência é sistémica,
   não local.
3. Auditoria transversal emerge como diagnóstico factual.
4. Decisão fica para humano em opções enunciadas.

Limiar tentativo N=3 longe; adiamento natural.

### §6.3 — Sub-padrão "Diagnóstico-primeiro factual" — N=4 cumulativo

| Aplicação | Origem |
|---|---|
| N=1 P156B | Layout diagnóstico antes de materialização |
| N=2 P154A | Model diagnóstico antes de materialização |
| N=3 P307a | Export diagnóstico antes de decomposição |
| **N=4 P309** | **IEEE 754 diagnóstico antes de decisão política** |

**Limiar tentativo N=3 ultrapassado**. Mas critério estrito é
"divergência declarativa **detectada**" — P156B/P154A/P307a foram
diagnósticos **pré-materialização** sem "divergência declarativa"
prévia. P309 é distinto pelo gatilho.

**Adiamento de promoção justificado** por critério estrito; pattern
genérico "diagnóstico-primeiro" pode justificar meta-ADR futura mas
P309 não força a decisão.

### §6.4 — Anti-padrão P273.17 §0 — **16 passos consecutivos**

**0 ADRs meta promovidas P293-P309**:

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P308 | 15 passos cumulativos | 0 |
| **P309** | **§6.2 + §6.3 (2 sub-padrões; um N=1 inaugural, outro N=4 cumulativo)** | **0** |

**16.ª vez consecutiva** anti-padrão honrado. P309 confirma que
**passos diagnósticos-primeiro** também respeitam o anti-padrão —
disciplina não depende de magnitude nem de tipo de artefacto
produzido.

Tentação de promover §6.2 inaugural a meta-ADR foi rejeitada porque
N=1 está abaixo do limiar tentativo N=3. Tentação de promover §6.3
N=4 cumulativo a meta-ADR foi rejeitada porque critério estrito não
está atingido.

### §6.5 — ADR-0098 "single source of truth" — N=2 cumulativo pós-P307

Hash `export/*` preservado bit-exact pelo **2º passo consecutivo**
P308→P309. Invariante robusta sobre 2 features distintas:

- P308 — stdlib calc 1 função (não toca export).
- P309 — diagnóstico-primeiro (zero código tocado).

Hash `content.rs` preservado em **26 passos consecutivos**.

### §6.6 — §8.6 "A.5' anti-reflexão" — N=17 cumulativo

P291-P309. P309 reconhece honestamente:

- **Magnitude M+** efectiva (não M declarada) — comparação β
  obrigatória todas as categorias escalou o esforço. Reportado
  transparentemente no header e em §1.
- **Recomendação Opção D pode ser rejeitada** — diagnóstico assume
  trade-off custo-implementação como factor humano legítimo. §9 do
  diagnóstico apresenta D primária mas C secundária explícita como
  "alternativa mais barata".
- **Pendências catalogadas explicitamente** — Cat D pendências
  (Float→Int, color, backdoor) ficam em §10 do diagnóstico para
  reforços pontuais futuros, fora do escopo decisão IEEE 754.

### §6.7 — Drift L0 — explicitamente adiado para P310

P309 **não toca L0** apesar de ser diagnóstico-primeiro. Decisão:
drift L0 documentando política IEEE 754 fica para **passo de decisão**
(que executou Opção A em P310).

**Justificação**: documentar política em L0 antes de decisão seria
prematuro — opções A/B/C/D ainda em aberto. Pattern "diagnóstico
factual sem mudança declarativa" preservado.

---

## §7 — Cobertura e marco

P309 **não altera cobertura funcional**:

- Stdlib calc: 41/41 (100%) — inalterado.
- Stdlib agregada: ~54,5% — inalterado.
- Cobertura global: inalterada.

**Marco P309**:

- **1ª auditoria transversal IEEE 754** em cristalino.
- **67 sítios L1 catalogados posicionalmente** (11 Cat A + ~16 Cat B
  estruturais + 24 Cat C + 16 Cat D).
- **15 funções `calc.*` comparadas sítio-a-sítio com vanilla**.
- **26º passo consecutivo** com `content.rs` hash inalterado.
- **2º passo consecutivo pós-P307** com `export/*` preservado.

---

## §8 — Frentes pendentes pós-P309

P309 cataloga; **decisões ficam adiadas** explicitamente em §10 do
diagnóstico:

| Frente | Magnitude | Estado pós-P309 |
|---|---|---|
| Adoptar Opção A/B/C/D política IEEE 754 | S documental ↔ M+ código | **decidido P310** (Opção A escolhida) |
| Reverter ou manter `calc_erf` NaN→Err | XS-S | depende da política — **mantido P310** (Opção A) |
| Migração agregada libm (ADR-0018) | M+ | ortogonal; **mantém `guard_float`** sob Opção A |
| ADR-IEEE754-conformidade | XS | **materializada P310 como ADR-0101** |
| Range check em `oklab/...` | S | **adiada P310** (fora escopo Opção A) |
| Float→Int guard | S | **adiada P310** (fora escopo Opção A) |
| Backdoor `native_float("NaN")` | XS | **adiada P310** (fora escopo Opção A) |
| Uniformização linguística mensagens (PT vs EN) | XS | ortogonal |

**Frentes não-IEEE 754** continuam idênticas a §8 P306/P308:
- Math style functions (12 funcs).
- Data parsing (json/csv/...).
- Footnote refinos.
- Etc.

---

## §9 — Decisão sobre P310 (já tomada pós-P309)

P309 emitiu **recomendação operacional** em §9 do diagnóstico:
- Opção D primária (híbrido por categoria).
- Opção C secundária (status quo documentado).
- Opção A/B não recomendadas.

**Decisão humana real**: **Opção A** — "Conformidade IEEE 754 total
(status quo divergente)". Rejeição da recomendação primária.

P310 (`typst-passo-310.md` + `typst-adr-0101-ieee754-restricao-stdlib.
md` + drift L0 duplo + anotação cumulativa ADR-0033) materializou
Opção A em passo documental único, sem código tocado.

P309 **respeitou estrutura ADR-0065**: produziu recomendação sem
forçar; humano teve agência total para escolher diferentemente.
Pattern confirmado.

---

## §10 — Honestidade epistémica

### §10.1 — Magnitude declarada M, efectiva M+

Spec P309 §"Magnitude" declarou **M documental**. Execução com
comparação β (obrigatória todas as categorias) escalou para **M+**:
- 729 linhas no diagnóstico (vs ~400 estimado para escopo α + comp
  α).
- Comparação vanilla 15 funções `calc.*` + ops.rs + color
  constructors = ~120 linhas adicionais.

**Decisão honesta**: header do relatório reporta M+ explicitamente.
Magnitude declarada vs efectiva é **diferença legítima** quando
escopo escolhido pelo humano (β) é mais ambicioso que default (α).

### §10.2 — Recomendação Opção D rejeitada pelo humano

P309 §9 recomendou Opção D primária. Humano escolheu Opção A
(rejeição da recomendação). P310 materializou.

**Decisão honesta**: P309 não pressuponha que humano siga a
recomendação. ADR-0065 explicitamente permite divergência. P309
**fez o seu trabalho** (recomendação fundamentada); decisão posterior
ficou intocada por viés de P309.

Refinos possíveis no formato de recomendação: hierarquia mais clara
de critérios de decisão (custo vs paridade vs cobertura). Adiado.

### §10.3 — Sub-padrão "Auditoria de política transversal pós-divergência local isolada" N=1

P309 inaugura padrão. Pattern emerge de sequência cristalina
específica:

1. P308 implementou divergência local (`erf(NaN) → Err`) declarando-a
   "consciente".
2. Relatório P308 §10.3 documentou divergência.
3. Pergunta humana posterior ("o cristalino segue IEEE 754?") expôs
   que divergência é sistémica.
4. P309 emerge como auditoria transversal.

**Decisão honesta**: pattern é **defensável metodologicamente** —
divergências locais "conscientes" merecem auditoria transversal
quando o humano expressa dúvida sistémica. Mas N=1 está longe do
limiar tentativo N=3. **Adiamento de promoção a meta-ADR natural**.

### §10.4 — Sub-padrão "Diagnóstico-primeiro factual" N=4 cumulativo

P156B + P154A + P307a + P309 = N=4. Acima do limiar tentativo N=3.

**Decisão honesta**: critério estrito ("divergência declarativa
detectada") não está atingido em P156B/P154A/P307a — esses foram
diagnósticos pré-materialização **sem** divergência declarativa
prévia.

Pattern genérico "diagnóstico-primeiro" pode justificar meta-ADR
futura quando categoria refinada (P309 é caso novo). Adiado.

### §10.5 — Comparação vanilla obrigatória todas as categorias foi trabalho substantivo

Escolha β do humano (vs α default) **escalou esforço** mas produziu
catálogo materialmente mais completo. Tabela §4.3 do diagnóstico
com 15 funções `calc.*` é o output mais valioso de P309 — sem ela, o
diagnóstico seria abstracto.

**Decisão honesta**: β foi a escolha correcta. Recomendação spec
era β para Cat A apenas; humano expandiu. Recompensa: catálogo
completo.

### §10.6 — Pendências adiadas explicitamente catalogadas

§10 do diagnóstico lista 7 decisões adiadas. P310 confirmou
materialização de 4 destas (decisão política + ADR + reverter erf
+ anotação ADR-0033) e adiou 3 (range check, Float→Int guard,
backdoor).

**Decisão honesta**: catálogo §10 é **roadmap futuro**, não
"insuficiência" de P309. Diagnóstico-primeiro per ADR-0065 explicita
que decisões são humanas.

### §10.7 — 16.ª anti-padrão consecutivo é "real"

P309 produziu **2 sub-padrões observados** (§6.2 inaugural N=1; §6.3
N=4 cumulativo). Anti-padrão P273.17 §0 honrado por adiamento de
promoção formal a meta-ADR — não porque não havia candidatos.

**Decisão honesta**: adiamento é **conservadorismo metodológico
disciplinado**. §6.3 N=4 é o caso mais próximo da promoção
(>limiar) mas critério estrito não está atingido. Reavaliar quando
N=3 atingido na categoria refinada de §6.2 OU quando critério estrito
de §6.3 reformular para incluir P309.

---

## §11 — Fecho

P309 fechado com:

- **0 testes net** — zero código L1 tocado; suite 2 409 verdes
  preservada.
- **0 violations** no `crystalline-lint` (executado 1 vez durante
  o passo).
- **0 drift remanescente** — sem alterações para o linter ver.
- **Hash `entities/content.rs` preservado** bit-exact (**26º passo
  consecutivo**).
- **Hash `export/*` preservado** bit-exact (**2º consecutivo pós-
  P307**).
- **L0 markdown intocado** — drift adiado para P310.
- **0 ADRs criadas** — diagnóstico é factual, não decisão.
- **0 ADRs meta novas** — **16.ª vez consecutiva** anti-padrão
  P273.17 §0 honrado.
- **1 ficheiro publicado** — `diagnostico-ieee754-passo-309.md`
  (729 linhas).

**MARCO P309**:

- **1ª auditoria transversal IEEE 754** em cristalino.
- **67 sítios L1 catalogados posicionalmente** em 4 categorias
  (11 Cat A + ~16 Cat B + 24 Cat C + 16 Cat D).
- **15 funções `calc.*` comparadas sítio-a-sítio com vanilla**.
- **Contradição declarativa eval.md vs stdlib.md exposta
  explicitamente** (resolvida posteriormente em P310 por
  clarificação documental).
- **§8.7' N=14 categoria nova "diagnóstico transversal"** —
  primeira ocorrência cristalina.
- **Sub-padrão "Auditoria de política transversal pós-divergência
  local isolada" N=1 inaugural** — P308 local → P309 transversal.
- **Sub-padrão "Diagnóstico-primeiro factual" N=4 cumulativo**
  (P156B+P154A+P307a+P309); limiar tentativo N=3 ultrapassado mas
  critério estrito adia promoção formal.
- **4 opções de política enunciadas** sem decisão; tabela
  comparativa 12 critérios.
- **Recomendação operacional explícita** (per ADR-0065) — Opção D
  primária + Opção C secundária. **Decisão posterior do humano foi
  Opção A**; recomendação respeitada como conselho, não como
  imposição.

**Lição final**: P309 prova que **diagnósticos-primeiro per ADR-0065
podem ser amplos** (escopo L1 completo + comparação vanilla
obrigatória todas as categorias) **sem código tocado** e **preservando
completamente** invariantes de cobertura, hash e testes. A tentação
de "transformar P309 em passo material" (e.g. adoptar Opção A
directamente no diagnóstico) foi rejeitada — diagnóstico é factual,
decisão é humana.

A sequência **P308 (local) → P309 (transversal) → P310 (decisão)**
emerge como pattern cristalino reutilizável: divergência local
declarada → auditoria transversal factual → decisão política
documental. Pattern observado em §6.2 (sub-padrão inaugural N=1);
reavaliar promoção formal a meta-ADR em N=3 cumulativo se padrão
repetir.

Adicionalmente, P309 demonstra que **comparação vanilla obrigatória
todas as categorias é trabalho substantivo legítimo**, não inflação
de escopo — produziu o output materialmente mais valioso do passo
(tabela §4.3 com 15 funções). Escolha humana β sobre α default foi
correcta para magnitude desejada (M+ vs M).

Por fim, P309 confirma que **recomendação explícita per ADR-0065
respeita agência humana**: P309 §9 recomendou Opção D mas humano
escolheu Opção A. Recomendação é **trabalho de diagnóstico
completo** (catálogo + opções + recomendação fundamentada),
**não imposição de decisão**. Pattern preservado.
