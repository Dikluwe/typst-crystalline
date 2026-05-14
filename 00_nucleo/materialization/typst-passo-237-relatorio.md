# Relatório do passo P237 — `state_at(key, label)` stdlib refino estendido (Fase 5 Categoria D 1/? refino aditivo; primeira aplicação lição metodológica P236.div-1 via spec C1 audit obrigatório bloqueante)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-237.md`.
**Tipo**: refino aditivo trivial — 1 stdlib func nova; zero
fields; zero variants; zero módulos novos.
**Magnitude planeada**: S (~45min-1h). **Magnitude real**: S
(~30min — audit C1 trivial; implementação wrapper; tests;
docs).
**Marco**: **primeira aplicação da lição metodológica
P236.div-1** — spec C1 audit obrigatório bloqueante como
primeira cláusula; 3 patterns inaugurados; oitava aplicação
automática ADR-0080 EM VIGOR; **paralelismo state↔counter
completo**.

---

## §1 O que foi feito

P237 estende paralelismo state↔counter via refino aditivo
trivial:
- **`native_state_at(key, label)` em `foundations.rs`** —
  paralelo absoluto `counter_at` P177; reuso
  `Introspector::query_by_label` P139+P140 + `state_value`
  P171; chain `.and_then().unwrap_or(Value::None)`.
- **Registo scope** `state_at` em `eval/mod.rs:606`.
- **L0 NÃO tocado** — 8ª aplicação automática ADR-0080
  EM VIGOR.
- **ADR-0066 NÃO tocado** — SUPERSEDED-BY 0073 preservado.
- 7 unit tests cenários canónicos; **2143 → 2150 verdes**
  (+7); 0 adaptações intencionais; 0 regressões reais;
  0 violations.
- **Stdlib funcs**: 61 → **62** (+state_at).

---

## §2 Auditoria pré-P237 OBRIGATÓRIA BLOQUEANTE (C1) — lição P236.div-1 aplicada

**Audit empírico** (paralelo lição P236.div-1 que ensinou
a auditar antes de assumir):

- `Introspector::query_by_label(label: &Label) -> Option<Location>`
  baseline P139+P140 — confirmado. Spec hipotetizou
  `lookup_label(&str) -> SourceResult` — **divergência
  trivial signature**: nome real é `query_by_label`, toma
  `&Label` (não `&str`), retorna `Option` (não
  `SourceResult`). Ajuste signature trivial sem `P237.div-N`
  formal (per spec §5 risco 11).
- `Introspector::state_value(key: &str, location: Location)
  -> Option<&Value>` baseline P171 — confirmado paridade
  P236 audit §2.
- `native_counter_at` P177 pattern: build `Label(label_str)`
  → `query_by_label` → `.and_then(|loc| state_value(key,
  loc).cloned())` → `.unwrap_or(Value::None)`.
- **Sem `P237.div-N` formal** — audit converge com
  hipóteses revistas.

---

## §3 Implementação `native_state_at` + registo scope (C2+C3)

`01_core/src/rules/stdlib/foundations.rs`:

```rust
pub fn native_state_at(
    ctx:                &mut EvalContext,
    args:               &Args,
    _world:             &dyn crate::contracts::world::World,
    _current_file:      FileId,
    _figure_numbering:  Option<&str>,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(key), Value::Str(label_str)] => {
            let label = Label(label_str.to_string());
            let value = ctx
                .introspector
                .query_by_label(&label)
                .and_then(|loc| ctx.introspector.state_value(key.as_str(), loc).cloned())
                .unwrap_or(Value::None);
            Ok(value)
        }
        [_, other] => err(format!(
            "state_at() requer string como segundo argumento (label), recebeu {}",
            other.type_name()
        )),
        [other, _] => err(format!(
            "state_at() requer string como primeiro argumento (key), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "state_at() requer 2 argumentos (key, label), recebeu {}",
            args.items.len()
        )),
    }
}
```

`01_core/src/rules/eval/mod.rs:606`:

```rust
scope.define("state_at", Value::Func(Func::native("state_at", native_state_at)));
```

---

## §4 8 decisões fixadas + oitava aplicação automática ADR-0080 EM VIGOR

**8 decisões fixadas** (Decisão 0 = lição P236.div-1
aplicada):
- **Decisão 0** — C1 audit obrigatório bloqueante; sem
  `P237.div-N` (audit converge).
- **Decisão 1** — Opção α escopo minimal aditivo (apenas
  `state_at`).
- **Decisão 2** — Signature `(key: Str, label: Str) →
  Value` paridade `counter_at` P177 literal (ajuste trivial
  pós-audit: `query_by_label` em vez de `lookup_label`).
- **Decisão 3** — Reuso wrapper trivial `state_value`
  paralelo P236 `state_final`.
- **Decisão 4** — Semantic edge case revisitado: label
  inexistente retorna `Value::None` (paridade counter_at
  empty default; **revisão de spec** que sugeria erro
  hard).
- **Decisão 5** — 7 unit tests subset minimal cenários
  canónicos.
- **Decisão 6** — **Opção γ L0 NÃO tocado** (8ª aplicação
  automática ADR-0080 EM VIGOR).
- **Decisão 7** — ADR-0066 NÃO tocado (SUPERSEDED-BY 0073
  terminal preservado).
- **Decisão 8** — Sem promoção ADR-0079; sem marco
  cirúrgico blueprint (refino estendido não-fecha
  Categoria nem sub-categoria).

**ADR-0080 EM VIGOR aplicação automática N=7 → 8**:
- L0 prompts NÃO tocados em P237.
- `crystalline-lint --fix-hashes`: "Nothing to fix".
- **Oitava aplicação automática pós-promoção P229**
  (P230+P231+P232+P233+P234+P235+P236+**P237**).
- Pattern **extremamente sólido empíricamente** —
  8 aplicações consecutivas sem excepção.

**Anti-inflação 29ª aplicação cumulativa** pós-P205D —
Opção α escopo minimal + Opção α signature paralela +
Opção α reuso wrapper + Opção α subset tests + Opção γ
L0 automático + Opção α sem promoção ADR + Opção α sem
marco blueprint + Decisão 0 lição P236.div-1.

**Patterns emergentes inaugurados/consolidados P237** (4):
- "L0 minimal para refactors" aplicação automática N=7
  → **8 cumulativo**.
- **"stdlib func runtime para label-based lookup" N=1
  inaugurado P237** — distinto do "final value lookup"
  (state_at requer Location resolução via label).
- **"spec C1 audit obrigatório bloqueante pós-P236.div-1"
  N=1 inaugurado P237** — metodológico crítico aplicável
  a sub-passos futuros D.2+/C.1+/runtime.
- **"paralelismo state↔counter completo" N=1 inaugurado
  P237** — state agora 5 ops; counter 4 ops (counter sem
  paralelo state_update_with porque counter mutation é
  apenas Set).

---

## §5 Resultados verificação + tests (C4+C6)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2150 verdes | **2150 verdes** (1861+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ |
| Adaptações pre-existentes | N=0 | **N=0** ✓ |
| Stdlib funcs | 61 → 62 | ✓ |
| Sem novos variants/módulos/Layouter fields/promoções ADR | sim | ✓ |
| Regressões reais | 0 | **0** |

**Tests P237** (7 unit; sem layout E2E):
- `p237_state_at_label_inexistente_retorna_none`.
- `p237_state_at_key_inexistente_retorna_none`.
- `p237_state_at_resolve_label_retorna_init`.
- `p237_state_at_updates_antes_location_visivel` —
  populates 2 updates antes location consultada → último
  visível.
- `p237_state_at_updates_depois_location_nao_visiveis` —
  update em loc raw=5 invisível em consulta loc raw=2.
- `p237_state_at_arg_nao_string_rejeita` (2 sub-cenários:
  key Int + label Int).
- `p237_state_at_arity_errada_rejeita` (3 sub-cenários:
  0 args + 1 arg + 3 args).

1 ajuste trivial durante implementação: `LabelRegistry::insert`
não existe — método real é `add` (sed bulk replace; não
merece `P237.div-N`).

---

## §6 Próximo sub-passo

P237 completa paralelismo state↔counter D.1 refino estendido.
Próxima sessão candidatos:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **D.2 state.display(...)** | render-mediated state via callback (paridade vanilla `state.display(fn)`) | M (~2-3h) | média (completa state runtime user-facing) |
| **D.3 query refinos** | `query` refinos pós-baseline P175/P179 | S-M | baixa (já materializado) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b | L+ a XL (~10-20h) | baixa |
| **ADR meta admin XS** | Promoção formal pattern `.or()` resolution N=3 ou outros patterns sólidos | XS (~30min) | média (consolidação meta paridade P229) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **ADR meta admin XS** —
patterns sólidos acumulados (`.or()` N=3; refino paralelo
N=5; Smart→Option N=12; semantic adiada N=8; aplicação
automática EM VIGOR N=8; etc) atingiram limiar formalização.
Promoção formal num ou mais ADR meta consolidaria meta
sem custo significativo. Alternativa: **D.2 state.display**
se humano priorizar continuar Categoria D.

**Decisão humana fica em aberto literal** pós-P237.

**Estado pós-P237**:
- Tests workspace: 2143 → **2150 verdes** (+7 P237).
- Content variants: 60 preservado.
- Value variants: 55 preservado.
- **Stdlib funcs: 61 → 62** (+state_at).
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada (refino
  qualitativo marginal).
- **ADRs preservadas**: PROPOSTO 12; EM VIGOR 29 (ADR-0080);
  IMPLEMENTADO 21; total 67. ADR-0066 SUPERSEDED-BY 0073
  preservado.
- **Saldo DEBTs: 11 preservado**.
- **29 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=7 → 8 cumulativo** (P230-P237) — pattern
  extremamente sólido empíricamente.
- **Pattern "stdlib func runtime para final value lookup"
  N=2 preservado** (counter_final P176; state_final P236).
- **Pattern "stdlib func runtime para label-based lookup"
  N=1 inaugurado P237**.
- **Pattern "spec C1 audit obrigatório bloqueante
  pós-P236.div-1" N=1 inaugurado P237** — metodológico
  crítico.
- **Pattern "paralelismo state↔counter completo" N=1
  inaugurado P237**.
- **Categoria D Fase 5 Layout: 1/? refino estendido
  completo** — paralelismo state↔counter literal.
- **Fase 5 Layout candidata: 10/13-15 sub-passos
  materializados** (~67-77% cumulativo; **Categoria A
  100% + Categoria B 100% + Categoria D 1/? refino
  estendido completo**).
