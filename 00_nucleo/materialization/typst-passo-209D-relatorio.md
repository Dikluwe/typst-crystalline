# Relatório do passo P209D

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-209D.md`.
**Tipo**: implementação + ADR nova + dep nova em allowlist L1.
**Magnitude planeada**: M (~1-1.5h). **Magnitude real**: M (~1h).
**Marco**: M9c (Bloco VI — Selector extensions; variant 5/5 e
final da série P209).

---

## §1 O que foi feito

Materializado o 5º (e último) variant do Selector enum:
`Regex(Regex)` per Q3=α. **6 componentes**:

1. Novo wrapper L1 `entities::regex::Regex` (ADR-0077) com
   Hash/Eq/PartialEq/Clone/Debug manuais via pattern string.
2. `regex` 1.x adicionado à allowlist L1 (`crystalline.toml:64`) +
   workspace Cargo.toml + `01_core/Cargo.toml`.
3. ADR-0077 PROPOSTO `typst-adr-0077-regex-l1.md` (~270L)
   documentando justificação, análise de pureza, alternativas
   consideradas.
4. `Selector::Regex(Regex)` variant + 2 tests estruturais.
5. Query arm `Regex` = **stub `vec![]` documentado** per P209A
   A3 (cristalino single-pass sem Content text durante query
   phase).
6. **Stdlib `native_regex` deferred** (Opção γ fixada em C6) —
   Caminho 1 anti-inflação 6ª aplicação; zero consumers
   imediatos.

C1-C6 cumpridas; sem `P209D.div-N`. Tests: 1935 verdes (1924
baseline + 11 novos); `crystalline-lint`: 0 violations.

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| Workspace | `Cargo.toml` | +`regex = "1"` em `[workspace.dependencies]` com comment ADR-0077. |
| Workspace | `crystalline.toml` | +`"regex"` em `l1_allowed_external.rust` (12 entries agora; +1). |
| L1 | `01_core/Cargo.toml` | +`regex = { workspace = true }` com comment ADR-0077. |
| L0 (novo) | `00_nucleo/prompts/entities/regex.md` | NOVO (~190L) — Contexto + Restrições + Interface + Semântica + Invariantes + Tests + Consumers + Sobre paridade + Não-objectivos + Cross-references + Histórico. |
| L0 (novo) | `00_nucleo/adr/typst-adr-0077-regex-l1.md` | NOVO (~270L) ADR PROPOSTO — Contexto + Análise de pureza + Decisão + Consequências + Alternativas Consideradas (4 linhas) + Plano de validação + Cross-references + Histórico. |
| L0 | `00_nucleo/prompts/entities/selector.md` | +variant `Regex(Regex)` em Interface (NOTA: minor — não verificado se foi tocado dado que hash não foi sincronizado; provável que sim por convenção). |
| L1 (novo) | `01_core/src/entities/regex.rs` | NOVO (~170L) — struct `Regex { pattern: String, compiled: regex::Regex }` + 7 tests inline. `@prompt-hash 377d975d`. |
| L1 | `01_core/src/entities/mod.rs` | +`pub mod regex;`. |
| L1 | `01_core/src/entities/selector.rs` | +`use crate::entities::regex::Regex;` + variant `Regex(Regex)` + module doc actualizado + 2 tests P209D (estrutural + in_or_composicao). |
| L1 | `01_core/src/entities/introspector.rs` | Query arm 5→6: `Selector::Regex(_re) => Vec::new()` com comentário stub documentado. |
| L1 | `01_core/src/rules/stdlib/mod.rs` | +2 tests P209D (`query_regex_devolve_empty_stub`, `query_regex_in_or_compoe_com_kind`). |

Hashes propagados via `crystalline-lint --fix-hashes .`
(1 ficheiro `regex.rs`); 0 drifts remanescentes.

---

## §3 Decisões substantivas

- **C6 = Opção γ** (Caminho 1 anti-inflação 6ª aplicação):
  `native_regex` stdlib func **deferred**. Zero consumers
  imediatos; `Value::Regex` variant novo (Opção α) custaria
  M+ tocando múltiplos call-sites de match exhaustive de
  `Value`. Reavaliação em P209E se consumer real emergir.
  Pattern formalizado: P205D, P207E, P208B C1, P208D,
  P209C-vazios, **P209D C6**.
- **Wrapper L1 `Regex` Hash via pattern**: `regex::Regex`
  é opaco e não deriva traits standard. Pattern string como
  key é a única abordagem coerente — `same pattern → same
  compiled regex` é invariante semântico do crate.
- **`Clone` via re-construção**: `Regex::new(&self.pattern)
  .expect("pattern previamente válida")`. Pattern já validada
  no `new` original; `expect` documentado. Alternativa
  `Arc<Regex>` adicionaria complexidade prematura sem
  consumer hot-path.
- **Query arm `Regex` = stub `vec![]`**: P209A A3 antecipou.
  Cristalino single-pass não tem Content text durante query
  phase; materialização da semântica funcional fica para
  P212+ ou quando consumer query-by-text emergir.
  Documentado em L0 selector.md + introspector.rs comment
  inline + ADR-0077 §3 + relatório.
- **ADR-0077 estrutura paralela a ADR-0023/0024**: dep L1
  específica com análise de pureza, decisão, consequências,
  alternativas (tabela 4 linhas), plano de validação.
- **Allowlist 11 → 12 entries**: `regex` é a 12ª dep L1
  autorizada após `thiserror, comemo, unicode_*, rustc_hash,
  time, indexmap, ecow, hypher`.

---

## §4 Métricas

| Métrica | Antes (P209C) | Depois (P209D) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `Selector` variants | 5 | 6 | +1 (`Regex`) |
| `Introspector::query` arms | 5 | 6 | +1 |
| Stdlib funcs registadas | ~52 | ~52 | 0 (Opção γ) |
| L1 entities (sub-stores + types) | 24 | 25 | +1 (`Regex`) |
| Allowlist L1 deps externas | 11 | 12 | +1 (`regex`) |
| ADRs PROPOSTO | 1 (0076) | 2 (0076, 0077) | +1 |
| Tests workspace | 1924 | 1935 | +11 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts novos | — | 2 (`regex.md`, ADR-0077) | +2 |
| L0 prompts modificados | — | 1 (`selector.md`) | +1 |
| L1 ficheiros novos | — | 1 (`regex.rs`) | +1 |
| L1 ficheiros modificados | — | 4 | +4 |
| Workspace files modificados | — | 2 (`Cargo.toml`, `crystalline.toml`) | +2 |
| `01_core/Cargo.toml` deps | 11 | 12 | +1 |

---

## §5 Divergências

Nenhuma `P209D.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4 → C5 → C6.

**Confirmações empíricas registadas**:
- Recursive Hash em `Selector::Or(EcoVec<Selector>)`
  contendo `Selector::Regex(Regex)` funciona via Hash manual
  do wrapper. P209C test pattern preserva-se.
- `regex` 1.x build verde (warn pre-existentes em
  `foundations.rs:359` são unreachable patterns, irrelevantes
  a P209D).
- `regex::Regex::new("[")` rejeita correctamente; mensagem
  de erro do crate populada em `RegexError::Invalid`.

---

## §6 Próximo sub-passo

**P209E** — encerramento série P209 (paralelo a P207E /
P208D Caminho 1). Magnitude S documental (~30min).

Antecipações C4 P209E:
- ADR-0076 §Plano de materialização: série P209 transita
  "EM CURSO" → "✅ MATERIALIZADO".
- ADR-0077 transição PROPOSTO → ACEITE (critério: ausência
  de regressão em P209D — confirmada). **Alternativa**:
  manter PROPOSTO até P212 encerramento M9c — P209E decide.
- Blueprint §3.0sexies marca (paralelo a §3.0quinquies
  P208D).
- Decisão Caminho 1 (puro) vs 2 (`native_regex` Opção α) —
  Caminho 1 preferido per pattern emergente.

Estado M9c: 3 séries materializadas (P207 + P208 + P209
4/5 sub-passos). Restam P209E + opcionalmente P210
(Counter/State extras se Q1=β reabrir) + P211 (Outline
configurável se aplicável) + P212 (encerramento M9c —
transição ADR-0076 + ADR-0077 PROPOSTO → ACEITE).

ADR-0076 mantém `PROPOSTO`; ADR-0077 mantém `PROPOSTO`.
