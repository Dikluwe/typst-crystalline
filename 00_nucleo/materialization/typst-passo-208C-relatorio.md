# Relatório do passo P208C

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-208C.md`.
**Tipo**: implementação stdlib trivial.
**Magnitude planeada**: S (~30min-1h). **Magnitude real**: S.
**Marco**: M9c (Bloco IV — `locate()`).

---

## §1 O que foi feito

Materializado o stdlib `native_locate(kind)` reusando pattern
literal de `native_query` (P175/P179). Retorna primeira Location
via `Introspector::query(Selector::Kind).first().copied()`; ou
`Value::None` se sem matches; ou erro contextual se kind
inválido / arg não-string. Limitação documentada: `Selector::Kind`
only — `locate(<label>)` exige P209. C1-C4 cumpridas; sem
`P208C.div-N`. Tests: 1907 verdes (1903 baseline + 4 novos);
`crystalline-lint`: 0 violations. Trait `Introspector` mantém 26
métodos (regra P207B §5 **não acionada**).

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L1 | `01_core/src/rules/stdlib/foundations.rs` | +`pub fn native_locate(ctx, args, ...)` (~60L) paralelo a `native_query`. Aceita 1 arg `Value::Str(kind)`; valida via `ElementKind::from_name`; consulta `ctx.introspector.query(&Selector::Kind(kind))`. Retorno: `Value::Location(first)` / `Value::None` / `Err`. Erros mencionam pendência P209 para hint ao usuário. |
| L1 | `01_core/src/rules/stdlib/mod.rs` | +`native_locate` em `pub use` block. +4 tests `p208c_locate_*` em tests module. |
| L1 | `01_core/src/rules/eval/mod.rs` | +`native_locate` em import block. +`scope.define("locate", Value::Func(Func::native("locate", native_locate)))` no scope global. |

L0 prompts (eval.md/stdlib.md) **não modificados** —
convenção emergente per P208B §3: stdlib funcs P169+
inline-documentadas em código sem L0 separado. `locate.md`
não criado (per spec §4 não-objectivo).

`crystalline-lint --fix-hashes`: "Nothing to fix" (L0
preservados; hashes intactos).

---

## §3 Decisões substantivas

- **Pattern literal `native_query`**: reusa parsing
  `ElementKind::from_name` + `Selector::Kind(...)`. Não
  inventa parsing novo (per spec §5 risco 1).
- **Retorno trinário** (`Some(Loc)` / `Value::None` / `Err`):
  - `Value::Location(loc)`: kind válido + ≥1 match.
  - `Value::None`: kind válido + 0 matches (`Vec::first` ↦
    `None`). Cristalino reusa `Value::None` (não introduz
    `Option<Location>` em Value).
  - `SourceResult::Err`: kind inválido ou arg não-string.
- **Hint P209 nos erros**: ambas mensagens de erro
  mencionam "locate(<label>) requer P209 (Selector::Label)"
  para ajudar usuário a entender limitação. Sentinela
  documental até P209 desbloquear.
- **Test "label-args" pendente** (per spec C3 — substituído
  por `arg_nao_string`): cristalino `Value` enum sem
  variant `Label`; `locate(<label>)` em .typ source não
  tem representação L1 actual. Test exercita o
  equivalente: arg não-`Value::Str` falha. P209 deverá
  remover/actualizar o test ao introduzir
  representação para selectors de label.

---

## §4 Métricas

| Métrica | Antes (P208B) | Depois (P208C) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `EvalContext` fields/methods | 5/3 | 5/3 | 0 |
| Stdlib funcs registadas | ~51 | ~52 | +1 (`locate`) |
| `CALL_COUNTERS` (P204G) | 26 | 26 | 0 (regra P207B §5 não acionada) |
| Tests workspace | 1903 | 1907 | +4 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts novos | — | 0 | 0 |
| L0 prompts modificados | — | 0 | 0 |
| L1 ficheiros modificados | — | 3 | +3 |
| Production consumers `locate()` | — | 0 | (mock-tested) |

---

## §5 Divergências

Nenhuma `P208C.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4.

**Confirmação empírica**: pattern `native_query` é
estável e reusável. P208C demonstrou que stdlib funcs
similares têm custo S (~30min). Para P208D + futuros
sub-passos paralelos: pattern stdlib é estável.

---

## §6 Próximo sub-passo

**P208D** (per ADR-0076 §Plano de materialização):
encerramento série P208 (paralelo a P207E Caminho 1).
Magnitude S documental (~30min-1h).

Antecipações:
- ADR-0076 série P208 transita "EM CURSO" → "✅
  MATERIALIZADO".
- Blueprint §3.0quinquies marca (paralelo a §3.0quater
  P207E).
- Decisão Caminho 1 (puro) vs Caminho 2 (com captura
  `Content::Context` block) fixada em P208D próprio C1
  per spec P208A C5.

Estado M9c: 7 sub-passos completos (P207A+B+C+D+E +
P208A+B+C). P208D remanescente na série P208; P209+
fora ainda.
