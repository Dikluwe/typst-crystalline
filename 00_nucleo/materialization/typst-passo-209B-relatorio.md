# Relatório do passo P209B

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-209B.md`.
**Tipo**: implementação trivial (variants + query arms + stdlib
dispatch refactor).
**Magnitude planeada**: S (~45min). **Magnitude real**: S.
**Marco**: M9c (Bloco VI — Selector extensions; 2 dos 5 variants).

---

## §1 O que foi feito

Materializados 2 variants triviais do `Selector` enum:
`Label(Label)` e `Location(Location)` per Q3=α humano. Trait
arms adicionados em `Introspector::query` (Label via
`query_by_label`; Location singleton). Stdlib `native_query` +
`native_locate` refactored com helper privado
`parse_selector_arg` que dispatcha por `Value` variant
(`<name>` syntax → Label; `Value::Location` → Location; kind
string → Kind). C1-C4 cumpridas; sem `P209B.div-N`. Tests:
1915 verdes (1907 baseline + 8 novos); `crystalline-lint`: 0
violations.

---

## §2 Alterações em código

| Camada | Ficheiro | Edição |
|--------|----------|--------|
| L0 | `00_nucleo/prompts/entities/selector.md` | +2 variants em Interface; +Semântica multi-variant; +Tests obrigatórios P209B; +Histórico 2026-05-12. Hash do Código: `3490d19c → 83989115`. |
| L1 | `01_core/src/entities/selector.rs` | +`use Label, Location`; +`Label(Label)` + `Location(Location)` variants em enum; +3 tests P209B. `@prompt-hash 92ddd3cd → f4d0f17d`. |
| L1 | `01_core/src/entities/introspector.rs` | Query match exhaustive: 1 → 3 arms. `Label(l)` delega a `query_by_label(l).map(\|loc\| vec![loc])`; `Location(loc)` retorna `vec![*loc]`. |
| L1 | `01_core/src/rules/stdlib/foundations.rs` | Refactor: `native_query` e `native_locate` extraem dispatch para helper privado `parse_selector_arg(items, func_name)`. Helper aceita 3 casos: `Str("<name>")` → Label; `Str(kind)` → Kind via `ElementKind::from_name`; `Location(loc)` → Location. Erros contextual com hint sobre P209D (Regex) + And/Or Rust-only. |
| L1 | `01_core/src/rules/stdlib/mod.rs` | +5 tests P209B (locate_label_syntax, locate_label_inexistente, query_location_arg, locate_location_arg, query_label_via_introspector_directo); test P208C `locate_arg_nao_string_retorna_err_com_hint_p209` actualizado com comentário sobre o que mudou (Value::Location agora dispatched). |

Hashes L0+L1 propagados via `crystalline-lint --fix-hashes .`;
0 drifts remanescentes.

---

## §3 Decisões substantivas

- **Parsing `<name>` syntax simplista**: `s.len() >= 2 &&
  s.starts_with('<') && s.ends_with('>')` → extrai nome via
  slice `[1..len-1]`. Edge cases:
  - `"<>"` (len 2): aceita "" como label nome (string vazia).
    Aceitável — semântica idêntica a `Label("")`.
  - `"<a b>"`: aceita; label com espaço. Cristalino `Label`
    não impõe restrição.
  - `"<incompleto"`: não termina com `>` → cai no arm
    `Str(kind)` → falha `ElementKind::from_name`.
- **Helper `parse_selector_arg`**: extraído como função
  privada em `foundations.rs` (não pub). Reuso entre
  `native_query` e `native_locate`; ambos partilham a mesma
  semântica de parsing. Mensagens diferenciadas por
  `func_name` arg.
- **`err()` helper limitado a `SourceResult<Value>`**:
  `parse_selector_arg` retorna `SourceResult<Selector>` —
  precisei de closure local `msg` que constrói
  `SourceResult<Selector>` directamente via
  `SourceDiagnostic::error`. Decisão minimal: não generalizar
  `err()` para outros T sem necessidade clara.
- **Query arm `Label`**: usa `query_by_label(l).map(|loc|
  vec![loc]).unwrap_or_default()` em vez de `into_iter()`.
  `query_by_label` retorna `Option<Location>` (P175 single
  Location compat); P207C `LabelRegistry` multi-label expõe
  primeira via `lookup`. Pattern preserva compat.
- **Test P208C actualizado**: nome mantido por estabilidade
  histórica; comentário interno explica o que mudou em P209B
  (Value::Location agora dispatched, mas Value::Int
  continua erro).

---

## §4 Métricas

| Métrica | Antes (P209A) | Depois (P209B) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| `Selector` variants | 1 | 3 | +2 (`Label`, `Location`) |
| `Introspector::query` arms | 1 | 3 | +2 |
| Stdlib funcs registadas | ~52 | ~52 | 0 |
| Tests workspace | 1907 | 1915 | +8 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| Hash drifts pós `--fix-hashes` | — | 0 | — |
| L0 prompts modificados | — | 1 (`selector.md`) | +1 |
| L1 ficheiros modificados | — | 4 | +4 |
| Helpers privados novos | — | 1 (`parse_selector_arg`) | +1 |

---

## §5 Divergências

Nenhuma `P209B.div-N`. Workflow executado linearmente
C1 → C2 → C3 → C4.

**Observação registada**: `Label::new()` inexistente
(antecipado pela spec §5 risco 3). Cristalino usa
`Label(name.to_string())` constructor directo (tuple struct
`pub Label(pub String)`). Pattern uniforme em todos os
call-sites; sem necessidade de novo método construtor.

---

## §6 Próximo sub-passo

**P209C** (per ADR-0076 §Plano de materialização):
`Selector::And` + `Selector::Or` materialização. Magnitude M
(~1-1.5h). Estrutura `EcoVec<Selector>` per C1 fixado em
P209A. Stdlib API: Opção (c) Rust API only (sem stdlib
expression `.typ` source).

Query arms (preview):
- `And(sels)` → intersecção: `sels.iter().fold(query(sels[0]),
  |acc, s| acc.intersect(query(s)))`.
- `Or(sels)` → união: `sels.iter().flat_map(|s| query(s))
  .collect::<HashSet<_>>().into_iter().collect()`.

Pré-condição cumprida (P209B concluído).

ADR-0076 mantém `PROPOSTO` até P212. Estado M9c: 2 séries
fechadas (P207+P208) + 2 sub-passos P209 (A diagnóstico + B
variants triviais).
