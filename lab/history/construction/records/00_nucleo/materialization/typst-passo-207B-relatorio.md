# Relatório do passo P207B

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-207B.md`.
**Tipo**: implementação focada (Bloco I — trait baixo custo).
**Magnitude planeada**: S (~1-2h). **Magnitude real**: S.
**Marco**: M9c (primeiro item Bloco I per ADR-0076).

---

## §1 O que foi feito

Materializado o método `query_labelled() -> Vec<(Label, Location)>`
no trait `Introspector`, primeiro item Bloco I do roadmap M9c (per
ADR-0076 C1). Trait passa de 20 para 21 métodos. Suporte
infraestrutural via novo `LabelRegistry::iter()` ordenado
alfabéticamente por `Label`. C1-C4 cumpridas; sem
`P207B.div-N`. Tests workspace: 1878 verdes (1873 baseline +
5 novos); `crystalline-lint`: 0 violations.

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L0 | `00_nucleo/prompts/entities/label_registry.md` | +entrada `pub fn iter(...)` em Interface; +semântica; +tests obrigatórios entry; +linha Histórico 2026-05-12. Hash do Código: `630fada0 → de91e049`. |
| L0 | `00_nucleo/prompts/entities/introspector.md` | +entrada `fn query_labelled(...)` em trait; +linha Histórico 2026-05-12. Hash do Código: `3544334d → cb327d65`. |
| L0 | `00_nucleo/prompts/infra/measurements.md` | "20 métodos" → "21 métodos" em 3 pontos (descrição wrapper, restrições, cross-references). Hash do Código sincronizado. |
| L1 | `01_core/src/entities/label_registry.rs` | +`pub fn iter(&self) -> impl Iterator<Item = (&Label, &Location)> + '_` (sort por `Label.0`, O(n log n)) + 2 tests (`p207b_iter_em_registry_vazio_*`, `p207b_iter_ordena_*`). `@prompt-hash 8bfee760 → 358133ac`. |
| L1 | `01_core/src/entities/introspector.rs` | +`fn query_labelled` no trait `Introspector` + impl em `TagIntrospector` (delega a `labels.iter()` com clone+copy) + 3 tests (`p207b_query_labelled_*`). `@prompt-hash 918d279b → 22bcb907`. |
| L3 | `03_infra/src/measurements.rs` | `INTROSPECTOR_METHODS: [&str; 20] → [&str; 21]` (entry `query_labelled`); `CALL_COUNTERS: [AtomicUsize; 20] → [AtomicUsize; 21]`; impl `Introspector for CountingIntrospector` ganha `fn query_labelled` com `record_call(20)`; sentinel `p204g_introspector_call_counts_existe` actualizado (20 → 21). `@prompt-hash 84928cb2 → 11bb9509`. |

Hashes L0+L1 propagados via `crystalline-lint --fix-hashes .`
(per spec C4); 0 drifts remanescentes.

---

## §3 Decisões substantivas

- **Ordenação determinística por `Label.0`** (per spec §5
  preferência): `LabelRegistry::iter()` faz `sort_by(|a, b|
  a.0.0.cmp(&b.0.0))` em vez de derivar `Ord` em `Label` —
  decisão minimamente invasiva (não toca `label.rs`). Tests
  validam ordem alfabética independente da ordem de `add`.
- **Handle-based return type** (`Vec<(Label, Location)>` vs
  vanilla `EcoVec<Content>`): preservado per ADR-0073 §C6 +
  ADR-0074 — cristalino devolve handles; consumers fazem
  lookup quando precisam de `Content` materializado.
  Documentado no doc comment do trait.
- **Não criar `MultiMap` ainda**: P207B fica em escopo S
  (`HashMap → MultiMap` é P207C — bloqueado pelas Q1-Q4 +
  `P207A.div-1`).
- **Tests `iter` em `label_registry.rs::tests`** (2
  adicionais ao C3 spec): redundantes com tests de
  `query_labelled` mas necessários para validar o
  invariante L0 que a iteração é ordenada. Total +5 tests
  vs spec +3.

---

## §4 Métricas

| Métrica | Antes | Depois | Δ |
|---------|-------|--------|---|
| Trait `Introspector` métodos | 20 | 21 | +1 |
| `LabelRegistry` API pública | 4 | 5 | +1 |
| `CALL_COUNTERS` slots | 20 | 21 | +1 |
| Tests workspace | 1873 | 1878 | +5 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts modificados | — | 3 | +3 |
| L1 ficheiros modificados | — | 3 | +3 |

---

## §5 Divergências

Nenhuma `P207B.div-N` registada. Workflow executado
linearmente C1 → C2 → C3 → C4.

**Surpresa não-bloqueante** (registada para futura referência):
o trait extension propagou-se a `03_infra/src/measurements.rs`
(L3 `CountingIntrospector` wrapper P204G). Fix mecânico:
+1 entry em `INTROSPECTOR_METHODS`, +1 slot em
`CALL_COUNTERS`, +1 método delegando com `record_call(20)`,
+1 ajuste sentinel `p204g_introspector_call_counts_existe`
(assertion `20 → 21`). L0 `measurements.md` também
actualizado (descrição "21 métodos — 20 originais +
query_labelled"). Não acionou ADR — wrapper L3 segue
contracto trait L1. Para sub-passos futuros (P207C-E,
P207D): cada novo trait method propaga obrigatóriamente a
este wrapper.

---

## §6 Próximo sub-passo

**P207C** (per ADR-0076 §Plano de materialização):
refactor `LabelRegistry → MultiMap` + trait method
`label_count`. Magnitude M (~2-3h).

**Bloqueio activo**: P207C-E aguardam decisão humana sobre
`P207A.div-1` (escopo reduzido fundamentado) + respostas
às 4 questões pendentes (Q1-Q4 de P207A C10). P207B foi
materializado por já estar dentro do escopo reduzido
proposto pela div-1 (per cláusula C1 do diagnóstico P207A).

ADR-0076 §Plano de materialização anotado: P207 série
transita de "PENDENTE" → "EM CURSO"; P207B marcado
`✅ MATERIALIZADO 2026-05-12`.
