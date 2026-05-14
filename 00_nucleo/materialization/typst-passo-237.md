# Passo 237 — D.1 refino estendido `state_at(key, label)` (Fase 5 Layout candidata Categoria D 1/? refino aditivo; **lição metodológica P236.div-1 aplicada via C1 audit obrigatório bloqueante**)

**Série**: 237 (vigésimo-terceiro sub-passo Layout pós-M9c;
**décimo sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; **refino aditivo Categoria D** —
completa paralelo absoluto state↔counter via `state_at`
paralela `counter_at` P177; oitava aplicação automática
ADR-0080 EM VIGOR pós-P229).
**Marco**: nenhum status ADR; **primeira aplicação da
lição metodológica P236.div-1** — spec C1 audit obrigatório
bloqueante como primeira cláusula antes de fixar decisões
arquiteturais C2+; pattern emergente "spec C1 audit
obrigatório bloqueante pós-P236.div-1" N=1 inaugurado P237;
pattern "stdlib func runtime para label-based lookup" N=1
inaugurado P237 (paralelo a "stdlib func runtime para
final value lookup" P176+P236 N=2).
**Tipo**: refino aditivo trivial — **1 stdlib func nova**
(`native_state_at`); **zero fields novos**; **zero novos
variants**; **zero módulos novos**.
**Magnitude**: S (~45min-1h; paridade P236 refino aditivo
pós-divergência).
**Pré-condição**: P236 concluído (D.1 state_final refino
aditivo; `P236.div-1` registada; 2143 verdes; 0 violations;
saldo DEBTs 11; ADR-0066 SUPERSEDED-BY 0073 preservado
terminal; ADR-0079 Categoria A 5/5 + Categoria B 3/3 +
Categoria D 1/? refino retroactivo); humano fixou D.1
refino estendido state_at (decisão literal pós-P236 §8);
`Introspector::state_value(key, location)` baseline P171
(P236 audit §2 confirmou existência); `native_counter_at`
baseline P177 (audit C1 obrigatório confirma signature
exacta); 22 sub-passos cumulativos pós-M9c sem regressão.
**Output**: 1 ficheiro relatório curto + código alterado
em ~2-3 ficheiros L1 (`foundations.rs` + `eval/mod.rs` +
possível 1 import adjacente) + L0 NÃO tocado (oitava
aplicação automática ADR-0080 EM VIGOR) + inventário 148
anotação cumulativa (footnote ⁵⁶) + ADR-0079 anotação
**Categoria D 1/? refino estendido state_at**.

---

## §1 Trabalho

P236 materializou `state_final(key)` refino aditivo pós-
divergência factual `P236.div-1`. P237 estende paralelismo
state↔counter via `state_at(key, label)` aditivo —
paralelo absoluto a `counter_at(key, label)` P177.

Vanilla `state.at(location)` retorna valor de state em
location específica (snapshot durante walk). Cristalino
paridade: `state_at(key, label)` resolve label → Location
via Introspector; retorna `Introspector::state_value(key,
location)`.

**P237 materializa D.1 refino estendido**:
- **`native_state_at(key, label)`** em `foundations.rs` —
  stdlib func nova; reuso `Introspector::state_value`
  baseline P171; paralelo absoluto `counter_at` P177.
- **Registo scope** `state_at` em `eval/mod.rs` paridade
  P236.
- **L0 NÃO tocado** — oitava aplicação automática ADR-0080
  EM VIGOR.
- 6-7 unit tests cenários canónicos (paridade P236 subset
  minimal).

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 0 — C1 audit OBRIGATÓRIO BLOQUEANTE (lição P236.div-1)

**Pattern emergente "spec C1 audit obrigatório bloqueante
pós-P236.div-1" N=1 inaugurado P237**.

Spec P237 fixa decisões C2-C8 **condicionais ao output de
C1 audit**. Se audit revela contradição com hipótese spec
(e.g., `Introspector::state_value` não existe; signature
diverge; pattern `counter_at` distinto):
- Registar `P237.div-N` formal imediato.
- Pausar materialização C2+.
- Decisão humana via questionário paridade P236.div-1.

**Decisões C2-C8 abaixo são hipóteses sujeitas a audit C1.**

### Decisão 1 — Escopo Opção α (apenas state_at aditivo)

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Apenas `state_at(key, label)` — paralelo `counter_at` P177 | Refino aditivo minimal; paridade P236 (1 func por refino) |
| β | `state_at` + `state.display(...)` simultâneo | Inflacionário; viola paridade P236 |
| γ | Refactor mais amplo state runtime | Fora escopo; state runtime baseline P171/M9 |

**Decisão fixada — Opção α** (escopo minimal aditivo).

### Decisão 2 — Signature Opção α (paridade `counter_at` P177; sujeita audit C1)

3 opções (audit C1 confirma):

| Opção | Signature | Trade-off |
|-------|-----------|-----------|
| **α** | `state_at(key: Str, label: Str)` — paridade `counter_at` P177 literal | Coerente; reuso pattern |
| β | `state_at(key: Str, location: Location)` — Location directo | Inflexível; utilizador raramente tem Location object |
| γ | `state_at(state: State, label: Str)` — state é Value::State | Mas Value::State NÃO existe baseline (audit P236) |

**Decisão fixada (sujeita a audit C1) — Opção α**: paridade
literal `counter_at` P177 signature. Audit C1 confirma
signature exacta.

### Decisão 3 — Reuso `Introspector::state_value(key, location)`

Audit P236 §2 confirmou existência de `state_value(key,
location)` em `Introspector`. Audit C1 confirma signature
+ visibility.

3 opções (sujeitas audit):

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Wrapper trivial sobre `state_value` paridade `state_final` P236 | Reuso máximo; sem nova lógica |
| β | Implementação independente | Duplicação |
| γ | Helper privado novo `extract_state_at` | Inflacionário |

**Decisão fixada — Opção α**:

```rust
pub fn native_state_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label)] => {
            // Resolução label → Location paridade counter_at P177.
            let location = ctx.introspector.lookup_label(label.as_str())?;
            let value = ctx.introspector.state_value(key.as_str(), location)
                .cloned()
                .unwrap_or(Value::None);
            Ok(value)
        }
        // ... validações tipo ...
    }
}
```

### Decisão 4 — Semantic edge cases

3 cenários edge:

| Cenário | Comportamento | Justificação |
|---------|---------------|-------------|
| Key inexistente | `Value::None` | Paridade P236 `state_final` |
| Label inexistente | Erro hard | Paridade `counter_at` P177; label deve existir (responsabilidade utilizador) |
| Key existente; location sem updates anteriores | `init` value (semantic Introspector) | Audit C1 confirma comportamento `state_value` baseline |

**Decisão fixada (sujeita audit C1 cenário 3)**: paridade
P176+P177 patterns.

### Decisão 5 — Tests subset minimal Opção α (6-7 cenários canónicos)

Paridade P236 subset minimal:
- `state_at` resolução label OK retorna valor em location.
- `state_at` key inexistente retorna None.
- `state_at` label inexistente retorna erro.
- `state_at` updates múltiplos antes location.
- `state_at` updates múltiplos depois location.
- `state_at` arg tipo errado rejeita.
- `state_at` arity errada rejeita.

**Decisão fixada — Opção α** (6-7 unit tests; sem layout
E2E paridade P236).

### Decisão 6 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação automática N=8)

**Decisão fixada — oitava aplicação automática pós-P229**:

P237 é refino aditivo verdadeiro (1 stdlib func nova) —
qualifica Opção γ literal ADR-0080 §"Escopo" line 66
("stdlib func nova aditiva"). Paridade P236 Decisão 8
invertida pós-divergência (refino aditivo qualifica
aplicação automática).

Pattern "aplicação automática ADR-0080 EM VIGOR sem decisão
explícita por sub-passo" N=7 → **8 cumulativo** (P230+P231+
P232+P233+P234+P235+P236+**P237**). Pattern **extremamente
sólido**; oito aplicações consecutivas sem excepção.

L0 prompts NÃO tocados.

### Decisão 7 — ADR-0066 NÃO tocado (preservado SUPERSEDED-BY 0073)

P236 estabeleceu ADR-0066 SUPERSEDED-BY 0073 terminal.
P237 preserva literal — não-tocado.

### Decisão 8 — Sem promoção ADR-0079; sem marco cirúrgico blueprint

P237 é refino estendido D.1 (não fecha Categoria D nem
sub-categoria). ADR-0079 Categoria D mantém **1/? sub-passos
materializados** (refino interno; sem incremento contagem
sub-passos).

Sem marco cirúrgico blueprint — paridade pattern P236
(refinos aditivos não-marcam blueprint).

Reuso de dados (sem recolha nova):

- `Introspector::state_value(key, location)` baseline P171
  (P236 audit §2).
- `native_counter_at` baseline P177 (audit C1 obrigatório
  confirma signature).
- `Introspector::lookup_label(label)` baseline P139+P140
  (audit C1 confirma signature).
- Pattern "stdlib func runtime para final value lookup"
  N=2 baseline (counter_final P176; state_final P236).
- Pattern "aplicação automática ADR-0080 EM VIGOR" N=7
  baseline P230-P235+P236.
- Pattern P236.div-1 lição metodológica (C1 audit obrigatório
  bloqueante).
- ADR-0079 PROPOSTO Categoria D 1/? baseline P236.
- ADR-0066 SUPERSEDED-BY 0073 terminal baseline P204H.

---

## §2 Cláusulas (8 — atomização reduzida paridade P236 pós-divergência)

### C1 — AUDITORIA OBRIGATÓRIA BLOQUEANTE (lição P236.div-1)

**CRÍTICA absoluta** — primeira cláusula bloqueante; spec
C2+ depende output C1.

Audit empírico imediato:

```
grep -B 2 -A 20 "fn native_counter_at\|native_counter_at" 01_core/src/rules/stdlib/foundations.rs
grep -B 2 -A 20 "fn state_value\|state_value" 01_core/src/entities/introspector.rs
grep -n "fn lookup_label\|lookup_label" 01_core/src/entities/introspector.rs
grep -A 10 "Introspector::state_value\|introspector.state_value" 01_core/src/
ls 01_core/src/rules/stdlib/  # confirmar foundations.rs paridade P236
```

**Hipóteses sujeitas a confirmação empírica**:
- `native_counter_at` signature: `(key: Str, label: Str)`
  → Value (paralelo `counter_final` P176 mas +1 arg label).
- `Introspector::state_value(key: &str, location: Location)`
  baseline P171 visibility `pub`.
- `Introspector::lookup_label(label: &str)` baseline P139+
  P140; retorna `SourceResult<Location>`.

**Critérios de divergência crítica**:
1. **Se `state_value` não existe** → contradição com P236
   audit §2; registar `P237.div-1` formal; pausar C2+;
   questionário humano.
2. **Se `counter_at` signature diverge significativamente**
   → adaptar paridade; sem `P237.div-N` formal se ajuste
   trivial.
3. **Se `lookup_label` requer infraestrutura adicional**
   → audit decide; possível `P237.div-N`.
4. **Se `Introspector::state_value` retorna estructura
   diferente de hipótese** → adaptar wrapper inline.

**Decisão crítica C1**: se audit converge com hipótese
(provável; baseline confiável pós-P236 audit), prosseguir
C2+. Senão, **PAUSAR e registar `P237.div-N` formal**.

### C2 — Implementar `native_state_at(key, label)`

Editar `01_core/src/rules/stdlib/foundations.rs`:

```rust
pub fn native_state_at(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _figure_numbering: Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label)] => {
            // Paridade counter_at P177: resolução label → Location.
            let location = ctx.introspector.lookup_label(label.as_str())?;
            let value = ctx
                .introspector
                .state_value(key.as_str(), location)
                .cloned()
                .unwrap_or(Value::None);
            Ok(value)
        }
        [other_key, _] if !matches!(other_key, Value::Str(_)) => err(format!(
            "state_at() requer string como 1º argumento (key), recebeu {}",
            other_key.type_name()
        )),
        [_, other_label] if !matches!(other_label, Value::Str(_)) => err(format!(
            "state_at() requer string como 2º argumento (label), recebeu {}",
            other_label.type_name()
        )),
        _ => err(format!(
            "state_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}
```

**Reuso paridade absoluta `counter_at` P177** — signature
exacta + lookup label + state_value wrapper.

Magnitude C2: **XS (~15min)** — wrapper trivial.

### C3 — Registo scope `state_at`

Editar `01_core/src/rules/eval/mod.rs`:

```rust
// Após state_final P236 (~linha 601):
scope.define("state_at", Value::Func(Func::native("state_at", native_state_at)));
```

Imports actualizados se necessário (paridade P236 estructural).

Magnitude C3: **XS (~5min)**.

### C4 — Tests P237

`01_core/src/rules/stdlib/foundations.rs` test module:

```rust
#[test] fn p237_state_at_resolve_label_retorna_valor() { ... }
#[test] fn p237_state_at_key_inexistente_retorna_none() { ... }
#[test] fn p237_state_at_label_inexistente_retorna_erro() { ... }
#[test] fn p237_state_at_updates_antes_location_correto() { ... }
#[test] fn p237_state_at_updates_depois_location_nao_visivel() { ... }
#[test] fn p237_state_at_arg_nao_string_rejeita() { ... }
#[test] fn p237_state_at_arity_errada_rejeita() { ... }
```

Total tests P237: **6-7 unit tests** (paridade P236 subset
minimal; sem layout E2E pois state não-renderiza).

Esperado pós-P237: **2143 + 7 = ~2150 verdes**.

### C5 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação automática N=8)

**Decisão fixada — aplicação automática**: oitava aplicação
automática pós-promoção P229. Pattern N=7 → 8 cumulativo
extremamente sólido empíricamente.

L0 prompts NÃO tocados.

### C6 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2143 verdes pré-P237 + ~7 novos = **~2150 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~2-3 ficheiros L1 (`foundations.rs`
  + `eval/mod.rs` + possível 1 import adjacente).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: tests baseline P236 + counter_at P177.
Hipótese N=0 adaptações (refino aditivo trivial; sem
mudança signature funcs existentes).

### C7 — Inventário 148 footnote ⁵⁶ + ADR-0079 anotação Categoria D 1/? refino estendido

**Inventário 148**:
- §A.X Introspection: footnote ⁵⁵ → ⁵⁵ ⁵⁶.
- Footnote ⁵⁶ adicionada (~60 linhas) documentando state_at
  paralelo `counter_at` P177 + lição metodológica P236.div-1
  C1 audit obrigatório bloqueante + 8 decisões + paralelismo
  state↔counter completo (state/state_update/state_update_with/
  state_final/state_at; counter/counter_update/counter_final/
  counter_at) + L0 NÃO tocado N=8.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco `### P237
  anotação — Categoria D refino estendido state_at(key,
  label) paralelo counter_at P177; paralelismo state↔counter
  completo`.
- Status ADR-0079 mantido PROPOSTO (10/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓ +
  Categoria D 1/? refino estendido**).

### C8 — Critério aceitação P237

- ~6-7 tests novos verdes.
- 2143 tests pre-existentes preservados (N=0 adaptações
  hipotéticas).
- 0 violations.
- +1 stdlib func (`state_at`).
- Stdlib funcs: 61 → **62**.
- Sem novos variants; sem novos módulos; sem Layouter
  field; sem promoções ADR.
- ADR-0079 Categoria D 1/? refino estendido anotado.
- ADR-0080 EM VIGOR aplicação automática N=7 → 8 cumulativo.
- Cobertura Layout 89% preservada (refino qualitativo
  marginal).
- Cobertura Introspection refino marginal (+state_at
  paralelismo state↔counter completo).
- **Paralelismo state↔counter completo**: state runtime
  agora tem 5 ops (state/state_update/state_update_with/
  state_final/state_at) paridade counter 4 ops (counter/
  counter_update/counter_final/counter_at; counter sem
  paralelo state_update_with porque counter mutation é
  apenas Set, não Func).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-237-relatorio.md`.

Estrutura (~4-5 KB; magnitude S justifica brevidade)
com 6 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Auditoria pré-P237 OBRIGATÓRIA BLOQUEANTE (C1) —
  output audit empírico documentado.
- §3 Implementação `native_state_at` + registo scope
  (C2+C3).
- §4 Decisões substantivas (8 decisões fixadas incl.
  Decisão 0 lição P236.div-1) + oitava aplicação automática
  ADR-0080 EM VIGOR.
- §5 Resultados verificação + tests (C4+C6).
- §6 Próximo sub-passo (P238 candidatos).

Código alterado:
- **Editado**: `01_core/src/rules/stdlib/foundations.rs`
  (+ `native_state_at` ~30 linhas + ~6-7 unit tests).
- **Editado**: `01_core/src/rules/eval/mod.rs` (+ `scope.define`
  state_at).
- **Possivelmente editado**: imports adjacentes (paridade
  P236 estructural).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁶ P237).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria D 1/? refino estendido P237).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- `state.display(...)` — refino D.2+ candidato (não-bloqueador).
- Two-pass walk real para `state.final()` — sub-passo
  D.2 candidato (paridade P236 §4 não-objectivos).
- `query(target)` runtime — sub-passo D.3 candidato.
- `counter(key)` refinos adicionais — fora escopo (counter
  já completo via counter/counter_update/counter_final/
  counter_at).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas.
- Tocar ADR-0066 — SUPERSEDED-BY 0073 terminal preservado.
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática N=8.
- Reabrir decisões arquiteturais — P237 é refino estendido
  Categoria D.
- Marco cirúrgico blueprint pelo refino estendido —
  anti-inflação (refinos aditivos não-marcam blueprint).
- Promoção formal patterns emergentes consolidados (refino
  paralelo N=5, Smart→Option N=12, semantic adiada N=8,
  aplicação automática EM VIGOR N=8 — embora estes patterns
  estejam sólidos, promoção é passo administrativo separado).
- Helper `extract_label_to_location` novo — provavelmente
  já existe baseline (`Introspector::lookup_label` per
  P139+P140); audit C1 confirma.
- Refactor `state_value` signature — preservado baseline
  P171 literal.

---

## §5 Riscos a evitar

1. **Audit C1 revela `state_value` não existe** —
   contradição P236 audit §2. Mitigação: `P237.div-1` formal;
   pausar C2+; questionário humano paridade P236.div-1.
2. **`counter_at` signature diverge significativamente**:
   audit C1 confirma; possível ajuste signature `state_at`
   sem `P237.div-N` formal se trivial.
3. **`lookup_label` retorna `SourceResult<Location>` vs
   `Option<Location>`**: audit C1 confirma; ajuste
   error handling.
4. **Tests baseline P236 quebrados**: hipótese N=0
   adaptações (refino aditivo trivial não-conflita).
5. **L0 tocado por engano**: oitava aplicação automática
   sequencial. Mitigação: §5 risco 5 explícito + §C5
   fixa não tocar.
6. **Magnitude exceder S (~45min-1h)**: P236 chegou em
   ~45min. P237 mais trivial (sem divergência factual
   esperada). Hipótese real S- (~30min).
7. **Pattern "stdlib func runtime para label-based lookup"
   N=1 promoção prematura**: limiar formalização N=3-4
   não atingido. Mitigação: §"Não-objectivos" explícito.
8. **Mensagens de erro incoerentes com `counter_at` P177**:
   audit C1 confirma mensagens P177 exactas para paridade
   literal.
9. **Reabrir P236.div-1 retroactivamente via state_at**:
   tentação por "agora que adicionamos state_at, ADR-0066
   pode promover". Rejeitada — ADR-0066 SUPERSEDED-BY
   0073 terminal absoluto.
10. **Tests E2E layout incluídos por engano**: state não
    renderiza visualmente. Mitigação: apenas unit tests
    foundations paridade P236 subset minimal.
11. **`P237.div-N` formal por trivialidade**: registar
    div-N apenas para contradições significativas, não
    ajustes triviais (ex: signature `key: &str` vs `key:
    Str` é ajuste; não merece div-N).
12. **Documentar pattern "C1 audit bloqueante" em L0**:
    tentação por "lição metodológica importante". Rejeitada
    — ADR-0080 EM VIGOR aplicação automática; pattern
    documentado em footnote ⁵⁶ + ADR-0079 + spec C1
    suficiente.

---

## §6 Hipótese provável

C1 (audit obrigatório bloqueante) confirmará:
- `native_counter_at` signature `(key: Str, label: Str)
  → Value` baseline P177.
- `Introspector::state_value(key: &str, location:
  Location)` baseline P171 (P236 audit confirmou).
- `Introspector::lookup_label(label: &str) -> SourceResult<Location>`
  baseline P139+P140.

C2 implementará `native_state_at` paralelo absoluto
`counter_at` (signature + body trivialmente paralelo;
~30 linhas).

C3 registo scope trivial.

C4 criará 6-7 unit tests paridade P236 subset minimal.

C5 NÃO tocará L0 (oitava aplicação automática ADR-0080
EM VIGOR N=7 → 8).

C6 reportará ~2150 verdes; 0 violations; N=0 adaptações
hipotéticas.

C7 reclassificará footnote ⁵⁶ + ADR-0079 Categoria D 1/?
refino estendido.

C8 verifica critério aceitação.

Custo real: **S (~30-45min)** — refino aditivo trivial;
sem divergência factual esperada pós-P236 audit confiável.

Mas é hipótese, não decisão. C1 fixa empíricamente.
Possível `P237.div-N` se audit diverge.

---

## §7 Particularidade P237

P237 é estruturalmente distinto na trajectória pós-M9c:

- **Décimo sub-passo materialização Fase 5 Layout
  candidata** — segundo Categoria D refino aditivo
  cumulativo (P236 state_final; P237 state_at).
- **Primeira aplicação da lição metodológica P236.div-1**
  — spec C1 audit obrigatório bloqueante como primeira
  cláusula. Pattern emergente "spec C1 audit obrigatório
  bloqueante pós-P236.div-1" N=1 inaugurado P237 —
  metodológico crítico aplicável a sub-passos futuros
  D.2+/C.1+/runtime.
- **Pattern "stdlib func runtime para final value lookup"
  N=2 baseline preservado** (counter_final P176; state_final
  P236).
- **Pattern emergente "stdlib func runtime para label-based
  lookup" N=1 inaugurado P237** — distinto do final lookup
  (state_at requer Location resolução via label;
  state_final não requer).
- **Pattern "aplicação automática ADR-0080 EM VIGOR" N=7
  → 8 cumulativo** (P230-P236+**P237**). Pattern
  **extremamente sólido empíricamente** — oito aplicações
  consecutivas sem excepção.
- **Pattern "paralelismo state↔counter completo" N=1
  inaugurado P237** — state agora tem 5 ops paridade
  counter 4 ops (asymmetric porque state_update_with usa
  Func; counter mutation é apenas Set).
- **Cobertura Layout per metodologia preservada 89% real**
  — D.1 é Introspection refino, não Layout.
- **Anti-inflação 29ª aplicação cumulativa** pós-P205D —
  Opção α escopo minimal + Opção α signature paralela +
  Opção α reuso wrapper + Opção α subset tests + Opção γ
  L0 automático + Opção α sem promoção ADR + Opção α sem
  marco blueprint + Decisão 0 lição P236.div-1.

Por isso §5 risco 1 (audit C1 revela contradição) é o
mais provável simbolicamente embora improvável factualmente.
P236 audit §2 confirmou `state_value` existence; baseline
confiável. Mas pattern P236.div-1 ensinou que assumir
estado da codebase sem audit é risco metodológico.

**Critério de aceitação P237**:
- ~6-7 tests novos verdes.
- 2143 tests pre-existentes preservados (N=0 adaptações
  hipotéticas).
- 0 violations.
- +1 stdlib func (state_at).
- Stdlib funcs: 61 → 62.
- Sem novos variants; sem novos módulos; sem promoções
  ADR.
- ADR-0079 Categoria D 1/? refino estendido anotado.
- ADR-0080 EM VIGOR aplicação automática N=8.
- Paralelismo state↔counter completo.

**Estado pós-P237 esperado**:
- Tests workspace: 2143 → **~2150 verdes** (+6-7).
- **Stdlib funcs: 61 → 62** (+state_at).
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada (refino
  qualitativo marginal).
- **ADRs preservadas**: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  21; total 67. ADR-0066 SUPERSEDED-BY 0073 preservado.
- Saldo DEBTs: 11 preservado.
- **29 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=7 → 8 cumulativo** (P230-P237) — pattern
  extremamente sólido empíricamente.
- **Pattern "stdlib func runtime para final value lookup"
  N=2 preservado** (counter_final P176; state_final P236).
- **Pattern emergente "stdlib func runtime para label-based
  lookup" N=1 inaugurado P237** (state_at; counter_at
  baseline P177 não conta no N novo por ser anterior à
  série Categoria D).
- **Pattern emergente "spec C1 audit obrigatório
  bloqueante pós-P236.div-1" N=1 inaugurado P237** —
  metodológico crítico.
- **Pattern emergente "paralelismo state↔counter completo"
  N=1 inaugurado P237**.
- **Categoria D Fase 5 Layout: 1/? refino estendido**
  (state_final P236 + state_at P237; paralelismo state↔
  counter completo).
- **Fase 5 Layout candidata: 10/13-15 sub-passos
  materializados** (~67-77% cumulativo; **Categoria A
  100% + Categoria B 100% + Categoria D 1/? refino
  estendido completo**).
