# Relatório — Passo 182B

**Data**: 2026-05-02
**Passo**: P182B — `Introspector::is_numbering_active` trait method
**Magnitude**: S (executada ~ 80 LOC: 1 método trait + 1 impl + 5 tests + entrada L0)
**Resultado**: trait method materializado; impl delega a `StateRegistry`; 5 tests verdes; zero código fora de `01_core/src/entities/introspector.rs` + L0 `entities/introspector.md`.

---

## 1. Resumo

Adicionado método `fn is_numbering_active(&self, key: &str) -> bool` ao trait `Introspector` em `01_core/src/entities/introspector.rs`. Impl em `TagIntrospector` delega a `self.state.final_value(key)` com matching `Some(Value::Bool(true))`. Default `false` para state ausente, `Bool(false)`, ou variant não-Bool. Output observable em produção inalterado — `StateRegistry` ainda não recebe inits para chave `numbering_active:*` (esse arm é P182C `extract_payload`).

---

## 2. Confirmação `.F` (10 verificações)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ (1.0s, 2 warnings pré-existentes em `foundations.rs:355–359`, não tocados) |
| 2 | `cargo test --workspace` passa; Δ vs P182A baseline 1.738 = +5 | ✅ (1.743 passed; 7 ignored; 0 failed) |
| 3 | `crystalline-lint .` zero violations | ✅ (após `--fix-hashes`) |
| 4 | `is_numbering_active` accessível via trait `Introspector` | ✅ (`introspector.rs:112`) |
| 5 | `TagIntrospector` impl delega correctamente | ✅ (`introspector.rs:218–220`: `matches!(self.state.final_value(key), Some(Value::Bool(true)))`) |
| 6 | Documento sem `set state("numbering_active:*", _)` produz `is_numbering_active(*)` = false | ✅ (test `is_numbering_active_em_introspector_vazio_devolve_false`) |
| 7 | Walk **NÃO modificado** | ✅ (`introspect.rs` não tocado neste passo; arm `Content::SetHeadingNumbering:455–457` continua write canonical legacy) |
| 8 | Layouter **NÃO modificado** | ✅ (`layout/mod.rs:301`, `layout/equation.rs:24` ainda consultam `self.counter.is_numbering_active(…)` legacy — migração é P182D) |
| 9 | Snapshot tests ADR-0033 verdes | ✅ (incluídos nos 1.743 passed) |
| 10 | Linter passa final | ✅ (`✓ No violations found`) |

---

## 3. Δ tests vs baseline P182A

| Crate | Antes (P181J) | Após P182B | Δ |
|-------|---------------|------------|---|
| `typst-core` (`01_core` lib) | 1.478 | 1.483 | +5 |
| `typst-infra` (`03_infra` lib) | 215 | 215 | 0 |
| `typst-shell` (`02_shell` lib) | 24 | 24 | 0 |
| `typst-wiring` (integration tests) | 21 | 21 | 0 |
| **Total workspace** | **1.738** | **1.743** | **+5** |

5 tests novos co-localizados em `mod tests` dentro de `introspector.rs` (após bloco P181F, antes do `}` de fecho):

1. `is_numbering_active_em_introspector_vazio_devolve_false` — `TagIntrospector::empty()` retorna false para qualquer key.
2. `is_numbering_active_apos_init_bool_true_devolve_true` — `state.init("numbering_active:heading", Bool(true), loc(10))` → true.
3. `is_numbering_active_keys_distintas_isoladas` — heading activado não activa equation; leitura por chave isolada.
4. `is_numbering_active_bool_false_devolve_false` — `Bool(false)` retorna false explicitamente.
5. `is_numbering_active_value_nao_bool_devolve_false` — `Value::Int(1)` retorna false (graceful degradation).

Padrão dos tests P181F (`bib_methods_resolvem_apos_populacao_directa_do_sub_store`) replicado: popular `state` directamente via `pub(crate) fn init` em vez de passar por `from_tags` (esse caminho será coberto em P182C).

---

## 4. Hashes finais de L0s modificados

| L0 | Antes | Após `--fix-hashes` |
|----|-------|---------------------|
| `entities/introspector.md` (`@prompt-hash` no header de `introspector.rs`) | `c91f6d5b` | **`30bd91d8`** |
| `entities/introspector.md` (linha 2 "Hash do Código") | `9c591aff` | **`3f5b73cc`** |

`@prompt` apontador inalterado (`00_nucleo/prompts/entities/introspector.md`).
`@layer L1`, `@updated 2026-04-30` inalterados (data não mexida pelo lint; pode ser actualizada manualmente em fecho de série se desejado).

---

## 5. Decisões de execução notáveis

- **Convenção de chave** `numbering_active:<feature>` documentada no doc-comment do trait method e no L0. Justificação: prefixo namespace evita colisão com states user-space (`#state(...)`) e alinha com semântica do P171 StateRegistry. Equivalência cristalino actual: `state.numbering_active.get("heading")` ↔ `state.final_value("numbering_active:heading")`.
- **`init` vs `update`** nos tests: usei `init` em vez de `update`. Razão: `update` é ignorado se key não foi inicializada (defensivo, cf. `state_registry.rs:55–61`). Nos tests basta a primeira escrita — `init` cobre. Em P182C (`extract_payload` arm `Content::SetHeadingNumbering`), o caminho real será emitir `ElementPayload::StateUpdate { key, update: StateUpdate::Set(Bool(active)) }`; o `from_tags` arm `StateUpdate` (`from_tags.rs:154–166`) já chama `state.init` na primeira ocorrência (cf. P171/P173) e `state.update` nas seguintes — comportamento idêntico ao caminho de teste.
- **Fix-hashes manual** após edit do L0: `crystalline-lint --fix-hashes .` corrigiu 1 ficheiro (`introspector.rs`), reportou 0 drift restante. Single-pass, sem ciclos.
- **Sem mudança em `_updated_`** no header `@updated 2026-04-30`: P182B não altera (consistente com prática observada em commits recentes — `@updated` é tocado em refactor maior, não em adição local).

Sem decisões substanciais. Sem ADR. Sem DEBT.

---

## 6. Estado actual

- **P182 série**: A ✅ | B ✅ | C–F pendentes.
- **M9**: 10/11 features (inalterado — feature `numbering_active` só conta como fechada após P182F encerrar a lacuna #4).
- **Lacuna #4**: infra trait method materializada; falta `extract_payload` arm (P182C), Layouter migration (P182D), tests E2E (P182E), fecho (P182F).
- Tests workspace: **1.743 verdes** (Δ +5 vs P181J).
- Lint: **zero violations**.

---

## 7. Pendências cumulativas

Inalteradas face a P181J. Pendências M6 já documentadas em `auditoria-fresh-projecto.md` F1 + critério "Opção 3" P182A §3 cláusula 6. Em particular:

- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Walk arm canonical `introspect.rs:455–457` continua a popular `state.numbering_active` legacy.
- Write paralelo `layout/counters.rs:11–13` continua.
- Copy-sites `mod.rs:1414, 1442` continuam.
- Leituras intra-walk `introspect.rs:360, 378` continuam a consultar `state.is_numbering_active(…)` directo.

Layouter heading-arm + equation-arm continuam a usar `self.counter.is_numbering_active(…)` directo — migração via substitution-with-fallback é P182D.

---

## 8. Próximo passo

**P182C** — `extract_payload` arm `Content::SetHeadingNumbering` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Value::Bool(active)) }`.

Escopo concreto:
1. Em `01_core/src/rules/introspect/extract_payload.rs`, arm `Content::SetHeadingNumbering { active }` (junto a `Content::State` / `Content::StateUpdate`):
   ```rust
   Content::SetHeadingNumbering { active } => Some(ElementPayload::StateUpdate {
       key:    "numbering_active:heading".to_string(),
       update: StateUpdate::Set(Value::Bool(*active)),
   }),
   ```
2. **Não modificar** walk arm `introspect.rs:455–457` — continua write canonical legacy paralelo (M6 elimina).
3. **Não modificar** `from_tags` — arm `StateUpdate` já existente cobre (cf. `from_tags.rs:154–166`).
4. Teste E2E mínimo: documento com `Content::SetHeadingNumbering { active: true }` produz `TagIntrospector` onde `is_numbering_active("numbering_active:heading") == true`.
5. Verificações: cargo test +N, crystalline-lint zero, hashes recalculados após edit do L0 `extract_payload.md` (se aplicável).

Magnitude **S**. Sem dependência fora de P182B. Ainda zero impacto em Layouter (P182D).
