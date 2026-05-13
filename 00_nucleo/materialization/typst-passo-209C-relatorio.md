# Relatório do passo P209C

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-209C.md`.
**Tipo**: implementação de composição N-ária (variants + query arms
intersect/union).
**Magnitude planeada**: M (~1-1.5h). **Magnitude real**: S-M (~50min).
**Marco**: M9c (Bloco VI — Selector extensions; variants 3-4 de 5).

---

## §1 O que foi feito

Materializados 2 variants compósitos do `Selector` enum:
`And(EcoVec<Selector>)` e `Or(EcoVec<Selector>)` per C4 P207A.
Query arms adicionados em `Introspector::query`: intersecção
(`And`) via filter+contains; união dedupliquada (`Or`) via
HashSet preservando ordem de primeira-aparição. **Opção A**
fixada em C3 para vazios: ambos retornam `vec![]`. Stdlib API
= Opção (c) Rust-only (sem dispatch via `Value`). C1-C5
cumpridas; sem `P209C.div-N`. Tests: 1924 verdes (1915 baseline
+ 9 novos); `crystalline-lint`: 0 violations.

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L0 | `00_nucleo/prompts/entities/selector.md` | +2 variants em Interface; +`use ecow::EcoVec;`; +Semântica composição N-ária + Query semantics secção dedicada; +Tests obrigatórios P209C; +Histórico 2026-05-12. Hash: `83989115 → db886542`. |
| L1 | `01_core/src/entities/selector.rs` | +`use ecow::EcoVec;`; +`And(EcoVec<Selector>)` + `Or(EcoVec<Selector>)` variants; +4 tests P209C (`and_estrutural`, `or_estrutural`, `and_or_vazio_estrutural`, `nested_recursivo`). `@prompt-hash f4d0f17d → 0cba412a`. |
| L1 | `01_core/src/entities/introspector.rs` | Query match exhaustive: 3 → 5 arms. `And(sels)` intersecção via `fold + filter + contains`; `Or(sels)` união via `HashSet::insert` check preservando ordem. Vazios devolvem `vec![]` (Opção A). |
| L1 | `01_core/src/rules/stdlib/mod.rs` | +5 tests P209C (and_vazio, or_vazio, and_interseccao, or_uniao_dedup, nested_or_dentro_de_and). |

Stdlib `foundations.rs` **não modificado** — Opção (c) Rust-only
significa que `native_query`/`native_locate` não recebem
dispatch novo para And/Or via `Value`. Construção é feita
directamente em Rust via `Selector::And(EcoVec::from(vec![...]))`.

Hash sincronizada via `crystalline-lint --fix-hashes .` (1
ficheiro); 0 drifts remanescentes.

---

## §3 Decisões substantivas

- **`EcoVec<Selector>` per spec P209A**: paridade vanilla
  literal (clone O(1) via Arc interno). Cristalino tem
  `ecow` em allowlist L1; uso natural para containers
  imutáveis-com-share.
- **Opção A para `And`/`Or` vazios** (Spec C3): ambos retornam
  `vec![]`. Cristalino single-pass não tem "universo"
  computável sem walk completo; consistência semântica entre
  os 2 vazios. Documentado em L0 Query semantics. Pattern
  "Caminho 1 anti-inflação" 5ª aplicação (P205D, P207E,
  P208B C1, P208D, P209C-vazios).
- **`And` intersecção via `filter+contains`** (vs HashSet):
  para N pequeno (típico ≤ 5 sub-selectors), `contains` em
  Vec linear é mais rápido que HashSet construction.
  Optimização HashSet diferida (per spec §5 risco 3).
- **`Or` união via HashSet `insert` check**: dedup correcto
  preservando ordem de primeira-aparição (`HashSet::insert`
  retorna `true` se novo). Output `Vec<Location>` ordenado
  por primeira-aparição (não por sub-selector iter order
  estritamente).
- **Test P209C estructural com Hash recursivo**: `EcoVec<Selector>`
  hashes via length + element hashes; `Selector` é recursivo
  mas Hash não causa stack overflow (valores concretos,
  ownership-bounded).
- **`And(v) != Or(v)`** mesmo com conteúdo idêntico (test
  `and_or_vazio_estrutural` + `or_estrutural` cobrem):
  variants distintos preservam desigualdade via discriminant.

---

## §4 Métricas

| Métrica | Antes (P209B) | Depois (P209C) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `Selector` variants | 3 | 5 | +2 (`And`, `Or`) |
| `Introspector::query` arms | 3 | 5 | +2 |
| Stdlib funcs registadas | ~52 | ~52 | 0 |
| Tests workspace | 1915 | 1924 | +9 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts modificados | — | 1 (`selector.md`) | +1 |
| L1 ficheiros modificados | — | 3 | +3 |
| Stdlib ficheiros modificados | — | 0 (Opção c) | 0 |

---

## §5 Divergências

Nenhuma `P209C.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4 → C5.

**Observação registada**: Hash recursivo em enum com
`EcoVec<Self>` funciona transparentemente — Rust deriva
`Hash` via discriminant + field hashes; `EcoVec<T>` hashes
via length + elements; recursão termina via ownership tree
finita. Sem precisar Hash manual.

---

## §6 Próximo sub-passo

**P209D** (per ADR-0076 §Plano de materialização):
`Selector::Regex` + ADR-0077 PROPOSTO + dep `regex` em
allowlist L1 + `Cargo.toml`. Magnitude M (~1-1.5h).

Trabalho concreto P209D (preview):
- Novo módulo `01_core/src/entities/regex.rs` — wrapper L1
  sobre `regex::Regex` crate.
- Hash + Eq + PartialEq manual (pattern string como key).
- ADR-0077 PROPOSTO em `00_nucleo/adr/typst-adr-0077-regex-l1.md`.
- Adicionar `regex` a `crystalline.toml:64` allowlist +
  `01_core/Cargo.toml` deps.
- `Selector::Regex(Regex)` variant.
- Query arm `Regex` — stub `vec![]` documentado (cristalino
  single-pass não tem Content text durante query phase).
- Stdlib func `native_regex(pattern)` constructor (Caminho A
  per P209A C2).

Pré-condição cumprida (P209C concluído). ADR-0076 mantém
`PROPOSTO`. Estado M9c: 2 séries fechadas + 3 sub-passos
P209 (A diagnóstico + B variants triviais + C compósitos).
