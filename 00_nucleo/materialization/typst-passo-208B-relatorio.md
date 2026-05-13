# Relatório do passo P208B

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-208B.md`.
**Tipo**: implementação cross-modular (infra `EvalContext` +
stdlib func).
**Magnitude planeada**: S-M (~2-3h). **Magnitude real**: S.
**Marco**: M9c (Bloco IV — `here()`).

---

## §1 O que foi feito

Materializado o stdlib `native_here()` + infraestrutura minimal
em `EvalContext` (`current_location: Option<Location>` field +
setter `with_current_location`). Sub-mecanismo (i) minimal
fixado em C1 — sem walk advance automático; populated via
setter (mock em tests; consumers futuros via show-rule
integration quando `Content::Context` block for materializado).
C1-C4 cumpridas; sem `P208B.div-N`. Tests: 1903 verdes (1899
baseline + 4 novos); `crystalline-lint`: 0 violations. Trait
`Introspector` mantém 26 métodos (regra P207B §5 **não
acionada** — `here()` é func stdlib, não trait method).

---

## §2 Sub-mecanismo fixado em C1 — evidência empírica

**C2 = Opção (i) minimal** — `current_location` field em
`EvalContext` sem walk advance automático.

Justificação literal (C1):

- **C1.1 — Vanilla `#context` block** (CONFIRMADO):
  `lab/typst-original/.../foundations/context.rs:65` define
  `ContextElem` (Locatable element). Show-rule
  `CONTEXT_RULE` (linha 78) constrói `Context::new(Some(loc),
  Some(styles))` e invoca o func do elemento com
  `context.track()`. Vanilla usa **sub-mecanismo ii análogo**
  (`ContextElem` + show-rule deferred eval). Custo de
  espelhar: M-L (novo variant `Content::Context` + show-rule
  + eval re-entry).
- **C1.2 — Cristalino eval walk** (CONFIRMADO):
  `EvalContext::new()` é chamado em 7+ pontos (eval mod,
  introspect, fixpoint, from_tags, stdlib tests). Nenhum
  precedente de avanço automático de Location durante eval.
  `Layouter::current_location` (P185C, `pub(super)`) vive
  no Layouter, não no EvalContext — fases distintas.
- **C1.3 — Consumer real** (NÃO APLICÁVEL — zero consumers):
  grep `Selector::Before`/`After` retorna zero matches.
  `Selector` cristalino é P175 minimal (`Kind` only).
  P209 Q-decisões (β/γ/α/β) confirmam scope reduzido:
  `Selector::Label` + `And` + `Or`; `Before/After` não estão
  no roadmap. **Sem caller imediato para `here()`** —
  materialização exige tests sintéticos (mock pattern
  autorizado per spec §5 risco 3).

**Critério satisfeito**:
- (ii) ContextElem-style: rejeitado por custo M-L sem
  consumer.
- (iii) Snapshot iter anterior: rejeitado por semântica
  degenerada (`here()` retorna "current"; snapshot não é
  current).
- **(i) minimal**: aceito. Field infra + setter + stdlib
  func; walk advance deferred a passo futuro quando
  consumer real emergir (provavelmente quando `Content::Context`
  block for materializado, paralelo a vanilla ContextElem).

P207E pattern formalizado (Caminho 1 anti-inflação) replicado
no espírito: materialização honesta proporcional ao consumo
imediato.

---

## §3 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L1 | `01_core/src/rules/eval/mod.rs` | +field `pub current_location: Option<Location>` em `EvalContext`; default `None` em `new()`; +método `pub fn with_current_location(self, loc) -> Self` setter conveniente. +import implícito `Location` via path full-qualified (sem novo `use`). Scope register: `scope.define("here", Value::Func(Func::native("here", native_here)))`. |
| L1 | `01_core/src/rules/stdlib/foundations.rs` | +`pub fn native_here(ctx, args, ...) -> SourceResult<Value>` (paralelo a `native_query` P175/P179). Sem args; lê `ctx.current_location`; devolve `Value::Location(loc)` ou erro contextual `"here() chamado fora de contexto locatable — current_location não populado"`. |
| L1 | `01_core/src/rules/stdlib/mod.rs` | +`native_here` em `pub use` block. +4 tests `p208b_here_*` em tests module. |

L0 prompts `eval.md` (`Passo 15`) e `stdlib.md` (`Passo 17`)
são **antigos** — convenção observável: stdlib funcs P169+
(`metadata`, `state`, `query`, etc.) são inline-documentados
em código sem extension dos L0 originais. `here()` segue o
mesmo pattern. Não há L0 novo (`stdlib/here.md` referido
na spec C2 dispensado por C1=Opção i minimal).

`crystalline-lint --fix-hashes`: "Nothing to fix" (L0 não
modificados; hashes preservados).

---

## §4 Decisões substantivas

- **Field directo + setter conveniente** (vs `Tracked<Context>`):
  reusa pattern de `EvalContext.introspector` (P174). Stdlib
  read directamente; sem novo tipo wrapper. Spec C3 confirmou
  assinatura idêntica a `native_query`.
- **Setter `with_current_location`** retorna `Self` (não
  `&mut Self`) para pattern builder: `EvalContext::new()
  .with_current_location(loc)`. Field também é
  directamente writable (`ctx.current_location = Some(loc)`)
  — ambos pattern são suportados (tests cobrem os 2).
- **Erro contextual** (não panic) para `None` — caller que
  invoca `here()` fora de contexto locatable recebe
  `SourceResult::Err`. Mensagem inclui hint sobre infra
  minimal P208B.
- **Sem args** (paridade vanilla literal): `here()` sem
  argumentos. Test `p208b_here_com_args_retorna_err`
  confirma rejeição.
- **L0 prompts não actualizados**: eval.md/stdlib.md são
  P15/P17. Convenção emergente: stdlib funcs P169+
  inline-documentadas. Extension futura pode consolidar
  L0 com Histórico explícito.

---

## §5 Métricas

| Métrica | Antes (P208A) | Depois (P208B) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `EvalContext` fields | 4 | 5 | +1 (`current_location`) |
| `EvalContext` methods | 2 | 3 | +1 (`with_current_location`) |
| Stdlib funcs registadas | ~50 | ~51 | +1 (`here`) |
| `CALL_COUNTERS` (P204G) | 26 | 26 | 0 (regra P207B §5 não acionada) |
| Tests workspace | 1899 | 1903 | +4 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts novos | — | 0 | 0 |
| L0 prompts modificados | — | 0 | 0 |
| L1 ficheiros modificados | — | 3 | +3 |
| Production consumers `here()` | — | 0 | (mock-tested) |

---

## §6 Divergências

Nenhuma `P208B.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4. Decisão sub-mecanismo registada como
parte integrante de C1 (não como `div`).

**Observação registada**: cristalino diverge intencionalmente
de vanilla. Vanilla usa `ContextElem`+show-rule (deferred
eval com Location). Cristalino expõe field em `EvalContext`
populated externamente. Documentar em ADR-0076 se P208C/D
revelarem necessidade adicional. Per `P205A.div-1`:
divergências arquitectónicas legítimas.

---

## §7 Próximo sub-passo

**P208C** (per ADR-0076 §Plano de materialização):
`native_locate(selector)` stdlib materialização. Magnitude
S (~30min-1h) — trivial após P208B; reusa
`Introspector::query` + `Value::Location` per P175/P179.

**Limitação herdada** (documentada em P208A C2): `Selector`
cristalino só tem `Kind` variant. `locate(selector(label))`
exige P209 (Selector::Label). P208C entrega `locate("kind")`
com kind-as-string args (paralelo a `native_query`).

ADR-0076 §Plano de materialização anotado: P208B marcado
`✅ MATERIALIZADO 2026-05-12` + sub-mecanismo (i) minimal
fixado em C1. Estado M9c: 6 sub-passos completos (P207A+B+C+D+E
+ P208A+P208B). P208C-D remanescentes na série P208.
