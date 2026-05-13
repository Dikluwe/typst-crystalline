# Relatório do passo P207C

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-207C.md`.
**Tipo**: refactor estrutural (sub-store) + trait extension.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M.
**Marco**: M9c (Bloco III sub-store refactor + Bloco II item 7
per ADR-0076).

---

## §1 O que foi feito

Refactor `LabelRegistry` para semântica multi-label (`HashMap<Label,
Location>` → `HashMap<Label, Vec<Location>>`) + materialização do
trait method `Introspector::label_count(&Label) -> usize`. Trait
passa de 21 para 22 métodos. `lookup` mantém comportamento
single-Location (compatibilidade preservada); novos métodos
públicos `lookup_all` + `count` expõem semântica multi-label.
C1-C5 cumpridas; sem `P207C.div-N`. Tests: 1885 verdes
(1878 baseline + 7 novos); `crystalline-lint`: 0 violations.

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L0 | `00_nucleo/prompts/entities/label_registry.md` | Refactor literal multi-label: Contexto + Restrições Estruturais + Interface (api 5→7 métodos) + Semântica + Invariantes + Tests obrigatórios + Sobre paridade + Resultado Esperado + linha Histórico 2026-05-12. Hash do Código: `de91e049 → 7bd1f25f`. |
| L0 | `00_nucleo/prompts/entities/introspector.md` | +entrada `fn label_count` no trait + linha Histórico 2026-05-12. Hash do Código sincronizado. |
| L0 | `00_nucleo/prompts/infra/measurements.md` | "21 métodos" → "22 métodos" em 3 pontos (descrição wrapper, restrições, cross-references). |
| L1 | `01_core/src/entities/label_registry.rs` | `inner: HashMap<Label, Location>` → `HashMap<Label, Vec<Location>>`; `lookup` ajustado para devolver primeira Location; +`pub fn lookup_all(&Label) -> &[Location]`; +`pub fn count(&Label) -> usize`; `iter` reescrito (flat_map sobre Vecs preservando ordem inserção); `add` agora faz `push` em vez de `or_insert`. +6 tests P207C. `@prompt-hash 358133ac → 06720061`. |
| L1 | `01_core/src/entities/introspector.rs` | +`fn label_count(&Label) -> usize` no trait + impl em `TagIntrospector` (delega a `labels.count`). +1 test `p207c_introspector_label_count_via_trait`. `@prompt-hash 22bcb907 → 12aab176`. |
| L3 | `03_infra/src/measurements.rs` | `INTROSPECTOR_METHODS: [&str; 21] → [&str; 22]` (entry `label_count`); `CALL_COUNTERS: [...; 21] → [...; 22]`; impl `Introspector for CountingIntrospector` ganha `fn label_count` com `record_call(21)`; sentinel actualizado (21 → 22). `@prompt-hash 11bb9509 → cbccd899`. |

Hashes L0+L1 propagados via `crystalline-lint --fix-hashes .`;
0 drifts remanescentes.

---

## §3 Decisões substantivas

- **`HashMap<Label, Vec<Location>>` simples** (C2 = Opção a):
  sem dep nova (`multimap` crate rejeitada). Cristalino tem
  precedente `kind_index: HashMap<ElementKind, Vec<Location>>`
  per P207A A4 #3.
- **`lookup` mantém compat single-Location**: devolve primeira
  inserção (`v.first().copied()`); `lookup_all` é novo
  método para semântica completa. Esta decisão preserva o
  único production call-site (`introspector.rs:299`
  `query_by_label` impl) sem alteração.
- **`len()` = labels únicas** (chaves do mapa), **não**
  total de pares. Documentado explicitamente em L0 e em
  doc comment. `iter().count()` dá o total de pares.
- **`iter` agrupa multi-label**: ordenação alfabética por
  `Label.0` para keys; dentro de cada grupo, ordem de
  inserção via `Vec`. Tests confirmam `len=2`, `iter.count=5`
  para 2 labels com 3+2 inserções.
- **`figure_label_numbers` não tocado**: sub-store separado
  em `TagIntrospector` que usa `HashMap<Label, usize>` e
  assume label única por figura (per spec §5 risco 2 +
  P207A A4 #4). Documentado em L0 Restrições Estruturais.
- **Test pre-existente `duplicada_preserva_primeira_location`
  preservado**: continua válido (semântica `lookup` retorna
  primeira; `len()=1`); novos tests P207C cobrem o
  comportamento multi-label adicional.

---

## §4 Métricas

| Métrica | Antes (P207B) | Depois (P207C) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 21 | 22 | +1 |
| `LabelRegistry` API pública | 5 | 7 | +2 |
| `CALL_COUNTERS` slots | 21 | 22 | +1 |
| Tests workspace | 1878 | 1885 | +7 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts modificados | — | 3 | +3 |
| L1 ficheiros modificados | — | 3 | +3 |
| Production call-sites quebrados | — | 0 | 0 |

---

## §5 Divergências

Nenhuma `P207C.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4 → C5.

**Confirmações empíricas registadas**:
- C1 grep `labels.lookup`: 1 production call-site
  (`introspector.rs:299`); `labels.add`: 2 production
  (`introspect.rs:513` + 1 test-only em `stdlib/mod.rs:400`).
  Refactor compatível — `lookup` continua a retornar
  primeira; `add` agora acumula em Vec sem requerer
  mudança nos callers.
- Regra empírica P207B §5 confirmada: cada trait method
  novo requer propagação em `CountingIntrospector` L3
  (4 pontos: `INTROSPECTOR_METHODS`, `CALL_COUNTERS`,
  impl, sentinel). Acionada novamente em P207D-E.

---

## §6 Próximo sub-passo

**P207D** (per ADR-0076 §Plano de materialização):
page-aware trait methods (`pages`, `page`, `page_numbering`,
`page_supplement`). Magnitude M-L (~5-6h).

**Bloqueio activo**: P207D-E exigem decisão arquitectural
separada (estender `LayouterRuntimeState` vs criar `PageStore`).
Fora do escopo reduzido aprovado em `P207A.div-1`. Aguardar
decisão humana antes de prosseguir.

ADR-0076 §Plano de materialização anotado: P207C marcado
`✅ MATERIALIZADO 2026-05-12`; bloqueio remanescente
reformulado (P207D-E aguardam decisão arquitectural; P207B+C
materializados dentro do escopo reduzido).
