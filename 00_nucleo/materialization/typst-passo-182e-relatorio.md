# Relatório — Passo 182E

**Data**: 2026-05-02
**Passo**: P182E — Tests E2E pipeline confirmando paridade
**Magnitude**: S (executada ~ 130 LOC em tests; zero código de produção tocado)
**Resultado**: 5 tests E2E novos em `mod p182e_e2e_heading_numbering`; auto-init validado em pipeline real; paridade snapshot Introspector vs legacy confirmada; sentinela contra regressão de janela compat M6.

---

## 1. Resumo

Submódulo `p182e_e2e_heading_numbering` adicionado ao fim de `01_core/src/rules/layout/tests.rs`. Replica padrão P181I (`p181i_e2e_bib`): submódulo dedicado, Content construído manualmente (sem `eval` real — em linha com prática estabelecida P181/P182D), tests assertam invariantes observáveis (`plain_text` + `Introspector::is_numbering_active`).

5 tests cobrem 4 cenários de `.B`–`.E` do plano (1 cenário tem teste duplo: pipeline via `layout()` legacy E pipeline via `layout_with_introspector` directo).

Zero código de produção tocado (P182E é puramente tests). Zero L0s modificados. Output observable em produção inalterado.

---

## 2. Confirmação `.F` (8 verificações)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace --lib` Δ vs P182D baseline 1.751 | ✅ (+5 → 1.756) |
| 3 | `crystalline-lint .` zero violations | ✅ (sem `--fix-hashes`; nenhum L0 modificado) |
| 4 | Tests `p182e_e2e_heading_numbering::*` passam isoladamente | ✅ (5/5 — `cargo test --workspace p182e`) |
| 5 | Tests existentes não regridem | ✅ |
| 6 | Output observable em produção inalterado | ✅ (P182E não toca produção) |
| 7 | Snapshot tests ADR-0033 verdes | ✅ |
| 8 | Linter passa final | ✅ |

---

## 3. Δ tests vs baseline P182D

| Crate | P182D | Após P182E | Δ |
|-------|-------|------------|---|
| `typst-core` (`01_core` lib) | 1.491 | 1.496 | +5 |
| `typst-infra` | 215 | 215 | 0 |
| `typst-shell` | 24 | 24 | 0 |
| `typst-wiring` integration | 21 | 21 | 0 |
| **Total** | **1.751** | **1.756** | **+5** |

5 tests novos no submódulo `p182e_e2e_heading_numbering`:

1. **`pipeline_completo_heading_numbering_via_layout_legacy`** (.B caminho legacy) — documento típico (`SetHeadingNumbering(true)` + 3 headings em nesting [1, 2, 1]) processado via `layout()` legacy. Confirma `plain_text` contém `"1."`, `"1.1"`, `"2."`. Após P181H, este path re-corre `introspect_with_introspector` internamente — ambos paths Introspector e fallback legacy estão activos via redundância.
2. **`pipeline_completo_heading_numbering_via_layout_with_introspector`** (.B caminho novo, irmão) — mesmo documento processado via `layout_with_introspector` directo. Confirma adicionalmente que `intr.is_numbering_active("numbering_active:heading") == true` antes de chamar layout (validação intermédia de P182C).
3. **`re_update_active_true_then_false`** (.C re-update) — sequência `SetHeadingNumbering(true) → H1 → SetHeadingNumbering(false) → H1`. Confirma:
   - H1 (Intro) com numbering ON tem prefixo `"1."`.
   - H2 (Apêndice) com numbering OFF não tem prefixo `"2."` mas o corpo `"Apêndice"` está presente.
   - `intr.is_numbering_active("numbering_active:heading") == false` no estado final (auto-init na primeira occurrence + update na segunda; `final_value` reflecte o último valor).
4. **`paridade_documento_complexo_legacy_vs_migrated`** (.D paridade) — documento complexo (3 headings em níveis variados + texto + equation block) processado em ambos paths. `plain_text` idêntico. Confirma migração P182B–D não introduziu divergência observável.
5. **`walk_continua_a_popular_legacy_apos_p182cd`** (.E sentinela) — documento `Content::SetHeadingNumbering { active: true }` produz `state.numbering_active["heading"] == true` após `walk` (path legacy ainda activo). Caso simétrico para `active: false`. Sentinela contra regressão de janela compat M6.

Padrão dos tests P181I (`p181i_e2e_bib`) replicado: submódulo dedicado, `use super::*` + imports localizados, helpers `Content::Sequence(Arc::from(vec![…]))`, asserções em `plain_text` ou trait method.

---

## 4. Hashes finais de L0s modificados

**Nenhum L0 modificado** em P182E. Tests novos não exigem actualização de prompts L0 (cobertura é instrumental, não arquitectural). `crystalline-lint .` reportou zero violations sem `--fix-hashes`.

---

## 5. Decisões de execução notáveis

### 5.1 5 tests em vez de 4

O plano `.B`–`.E` previa 3-4 tests (com `.E` opcional). Optei por incluir 5 (todos `.B`–`.E` mais 1 irmão para `.B`):
- **`.B` em duas variantes** (`via_layout_legacy` + `via_layout_with_introspector`) — cobre os dois entry points públicos. Custo trivial (~ 12 LOC); benefício: detectar divergência se `layout()` re-walk interno (P181H) for alterado em passo futuro.
- **`.C` re-update** — caso edge mais relevante porque exercita auto-init P182C 5.1 em pipeline real. Asserção dupla: output Layouter + estado final do Introspector.
- **`.D` paridade snapshot** — a verificação mais forte de paridade observable (1 documento, 2 paths, mesma string).
- **`.E` sentinela** — protege a janela compat M6 contra regressão silenciosa.

### 5.2 Re-update test exercita ambas as garantias arquitecturais

O test `re_update_active_true_then_false` confirma simultaneamente:

(a) **`from_tags::StateUpdate` auto-init** (P182C decisão 5.1): a primeira `SetHeadingNumbering(true)` é capturada via `state.init`; a segunda `SetHeadingNumbering(false)` segue o caminho normal `state.update` porque a key já está inicializada. Isto é validado pelo facto de `intr.is_numbering_active("numbering_active:heading")` retornar `false` (último update) em vez de `true` (init que ganharia se `update` fosse silenciosamente ignorado).

(b) **Path activo do bool é o fallback legacy mutável** durante o layout walk: o consumer `Introspector::is_numbering_active` usa `state_final_value` (que retorna sempre `false` neste cenário); seria insuficiente para distinguir H1 (numbering ON) de H2 (numbering OFF). Mas o fallback `||` `self.counter.is_numbering_active("heading")` consulta o estado mutável durante o layout walk — que reflecte ON na altura de H1 e OFF na altura de H2. Resultado correcto.

A divergência face a vanilla — que usa StyleChain hierárquica location-aware — é uma escolha consciente registada em P182A §3 cláusula 5: "manter divergência é honesto: cristalino é 'última escrita ganha' e não vai mudar com #4". P182E confirma que, para documentos com escritas múltiplas, **o fallback legacy é o caminho funcional**, não o Introspector — que só serve como redundância de janela compat.

Implicação operacional para M6: quando a pendência for retomada e o fallback `||` removido, o Introspector terá de ganhar semântica location-aware (`is_numbering_active_at(key, location)` que delega a `state_value(key, location)` em vez de `state_final_value`). Caso contrário, regressões em re-update emergem. Isto é trabalho substancial de P+ — fora P182. Documentado em §7.

### 5.3 `eval` de markup não usado

Plano `.B` mencionava "eval ou markup string equivalente" como opção. Optei por construção manual de `Content` (padrão P181I/P182D) — mais robusto contra mudanças no eval e mais fácil de auditar empiricamente. Acresce-se que P182 não toca eval, logo `eval(#set heading(numbering: "1.1"))` produz `Content::SetHeadingNumbering { active: true }` (cf. `eval/rules.rs:227`), que é exactamente o que o test constrói directamente.

### 5.4 Sem ADR. Sem DEBT.

---

## 6. Estado actual

- **P182 série**: A ✅ | B ✅ | C ✅ | D ✅ | E ✅ | F pendente.
- **M9**: 10/11 features (inalterado — fechamento formal em P182F).
- **Lacuna #4**: pipeline completo materializado e validado (P182B trait method + P182C arm extract_payload com locatable + auto-init from_tags + P182D 2 consumers Layouter migrados + P182E tests E2E paridade). Falta apenas **P182F** — fecho formal: actualizar `m1-lacunas-captura.md` com estado "✅ resolvida"; relatório consolidado série P182.
- Tests workspace: **1.756 verdes** (Δ +5 vs P182D; cumulative +18 vs baseline P181J 1.738).
- Lint: **zero violations**.

---

## 7. Pendências cumulativas

Inalteradas face a P182A §3 cláusula 6 (Opção 3):

- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Walk arm canonical `introspect.rs:455–457` continua write legacy (validado em `walk_continua_a_popular_legacy_apos_p182cd`).
- Write paralelo `layout/counters.rs:11–13` continua.
- Copy-sites `mod.rs:1414, 1442` continuam.
- Leituras intra-walk `introspect.rs:360, 378` continuam.
- Fallback `|| self.counter.is_numbering_active(...)` em ambos arms migrados.

Pendência adicional desde P182C (decisão 5.1): `from_tags::StateUpdate` auto-init divergência face a P171 strict.

**Pendência identificada em P182E** (decisão 5.2): se M6 remover o fallback `||`, o Introspector precisa de semântica location-aware para preservar correctness em re-update. Fora P182. Documentado para futuro.

---

## 8. Próximo passo

**P182F** — Fecho da lacuna #4 + relatório consolidado série P182.

Escopo concreto:
1. Actualizar `00_nucleo/diagnosticos/m1-lacunas-captura.md`:
   - Linha 62 (entrada lacuna #4): mudar de "P182A decisões fixadas; plano P182B–P182F (S-M) em curso" para "✅ **Resolvida em P182** (link diagnóstico + relatórios B/C/D/E/F)".
   - Tabela §Resumo (linha 127): mudar de "P182A decisões fixadas (M1+A2+Opção 3); plano P182B–P182F (S-M) em curso" para "✅ **Resolvida em P182** (cascade `Introspector::is_numbering_active` + `extract_payload` arm + `from_tags` auto-init + 2 consumers Layouter migrados; Opção 3 paridade preservada via fallback)".
   - Linha 106 (M9 features): mudar de "10/11" para "11/11" (lacuna #4 fechada implica M9 completa).
2. Escrever `00_nucleo/materialization/typst-passo-182f-relatorio.md` consolidando série P182 (A–F).
3. Verificações finais: cargo test (deve manter 1.756); crystalline-lint zero violations; nenhum ficheiro L1–L4 tocado.

Magnitude **S**. Ainda zero impacto observable em produção (P182F é apenas documentação + meta).
