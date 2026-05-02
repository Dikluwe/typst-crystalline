# Passo 181H — Relatório (walk arm `Bibliography` puro + `layout()` legacy migra)

**Data**: 2026-05-01
**Natureza**: passo **arquitectural** — restaura invariante walk
puro (P163) violada por P159C/F para bib state. `layout()` legacy
migra de `TagIntrospector::empty()` para introspector populado via
`introspect_with_introspector` interno.
**Pré-condição**: P181G concluído. Cite-arm consome via Introspector
com fallback defensivo a state legacy.

---

## 1. Sumário

**Walk arm puro**: `Content::Bibliography { title, .. }` em
`introspect.rs:567-573` reduzido a apenas descida em `title`. As 5
linhas que mutavam `state.bib_entries.extend(...)` e
`state.bib_numbers.entry(key).or_insert(...)` removidas. P163
invariante restaurada — walk não modifica nada além de emitir Tags
via `extract_payload`.

**`layout()` legacy migra**: `layout(content, initial_state)` em
`mod.rs:1355` agora re-corre `introspect_with_introspector(content,
None, None)` internamente para obter `Introspector` populado com
`BibStore`. Descarta o `state` retornado e usa o `initial_state`
passado pelo caller (preserva backward compat de fields não-bib).
**Custo**: walk extra além do walk feito pelo caller via
`introspect()`. Aceitável — bib feature é raramente usada.

Após P181H:
- Walk puro para bib (P163 invariante restaurada).
- `state.bib_*` legacy continua a existir mas **vazios em
  produção** (M6 elimina os fields).
- Path legacy `layout()` funciona via Introspector populado
  internamente.
- Janela compat **encerrada** para bib state — fallback a state
  legacy preservado em cite-arm como segurança extra (M6 elimina).

**Outputs**:

- `00_nucleo/prompts/rules/introspect.md` (L0; +entrada Histórico
  Revisões 2026-05-01; hash final `941ad50a`).
- `00_nucleo/prompts/rules/layout.md` (L0; +secção "`layout()`
  legacy injecta Introspector populado (P181H)"; hash final `81cfe96c`).
- `01_core/src/rules/introspect.rs` (walk arm reescrito + 2 tests;
  linhagem `941ad50a`).
- `01_core/src/rules/layout/mod.rs` (`layout()` re-corre
  `introspect_with_introspector`; linhagem `81cfe96c`).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.H` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1471 → **1473** (+2; estimativa era +3) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `introspect.md` actualizado com hash `941ad50a` | ✅ |
| 5. L0 `layout.md` actualizado com hash `81cfe96c` | ✅ |
| 6. L1 `introspect.rs` linhagem `@prompt-hash 941ad50a` | ✅ |
| 7. L1 `layout/mod.rs` linhagem `@prompt-hash 81cfe96c` | ✅ |
| 8. Walk arm `Content::Bibliography` puro (não muta `state.bib_*`) | ✅ (test `walk_arm_bibliography_nao_muta_state_bib_legacy`) |
| 9. `state.bib_entries` e `bib_numbers` permanecem vazios após walk em produção | ✅ |
| 10. Tag `Tag::Start(loc, ElementInfo { payload: Bibliography {..}, .. })` emitida via `extract_payload` | ✅ (asserção dentro do test do item 8) |
| 11. `from_tags` continua a popular `BibStore` via arm (P181E) | ✅ (testes P181E preservados) |
| 12. `layout()` legacy migrado para `introspect_with_introspector` + `layout_with_introspector` | ✅ (linhas 1355-1383 actualizadas) |
| 13. Cite-arm em path `layout()` legacy renderiza correctamente via Introspector | ✅ (6 tests existentes que falhavam após walk puro voltam a passar) |
| 14. Paridade `layout()` vs `layout_with_introspector` confirmada | ✅ (test P181G `paridade_legacy_vs_introspector_para_cite` cobre 4 forms) |
| 15. Snapshot tests ADR-0033 verdes | ✅ (215 infra integration + 6 ignored) |
| 16. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **Discrepância signature `layout()`**: instrução assumiu `layout(content)` (1 arg) mas signature real é `layout(content, initial_state)`. Adaptação: `layout()` mantém signature, passa a re-correr `introspect_with_introspector(content)` internamente, descarta o state novo e usa `initial_state` recebido. Backward compat 100%.

- **Custo computacional confirmado**: caller pattern actual `let state = introspect(&c); layout(&c, state)` já fazia 1 walk em `introspect()`. Após P181H, `layout()` faz mais 1 walk para construir introspector. Total 2 walks no path legacy. Cost trade-off documentado em L0 `layout.md` — aceitável dado que bib feature é raramente usada.

- **Outros consumers de `state.bib_*`**: confirmado P180 inventário. Apenas cite-arm (`layout/mod.rs:593, 601`) consome. Não há outros consumers a migrar.

- **6 tests existentes falharam intermediariamente** (entre `.F` e `.G`): `cite_normal_renderiza_numero_quando_bib_populada`, `cite_form_prose_inalterada_com_bib_numerada`, `cite_normal_multiple_entries_numeradas_em_ordem`, `cite_normal_multi_bibliography_continua`, `cite_prose_renderiza_author_year_quando_key_existe`, e `paridade_legacy_vs_introspector_para_cite`. Todos passam após `.G` (`layout()` migration). Prova que o caminho `layout()` legacy realmente depende do Introspector populado.

---

## 4. Δ tests vs baseline P181G

| Suite | Baseline P181G | Pós-P181H | Δ |
|-------|----------------|-----------|---|
| core lib | 1471 | **1473** | +2 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1710 | **1712** | **+2** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1731 | **1733** | **+2** |

Estimativa P181H: **+3** (3 tests `.B`). **Actual +2**.

Tests adicionados (apenas em `introspect.rs`):

1. `walk_arm_bibliography_nao_muta_state_bib_legacy` — verifica
   walk puro (state.bib_* empty + Tag emitida).
2. `walk_arm_bibliography_desce_em_title` — verifica que descida
   em `title` continua a funcionar.

Tests sugeridos no instruction `.B` que NÃO foram adicionados (mas
cobertura equivalente já existia):

- `layout_legacy_renderiza_cite_via_introspector_apos_p181h` —
  cobertura idêntica em `tests_show_rule_integration::cite_normal_renderiza_numero_quando_bib_populada`
  (e 5 outros) que usam path `layout()` legacy. Estes tests
  falhariam se `layout()` migration estivesse incorrecta.
- `paridade_layout_vs_layout_with_introspector_apos_p181h` —
  cobertura em P181G `paridade_legacy_vs_introspector_para_cite`
  (4 forms × 2 paths).

Adicionar tests redundantes seria ruído — o sub-passo `.G`
implícitamente prova a migração: 6 tests existentes falharam após
walk puro e voltaram a passar após `layout()` migration. Esta é a
prova mais robusta possível.

---

## 5. Estado de M5 e P181

**Invariante walk puro P163 restaurada para Bibliography**.
Cristalino agora tem walk puro para todas as features locatable
(Heading/Figure/Cite/Metadata/State/StateUpdate/Outline/Bibliography).
Próximas adições futuras de `Content` variants podem replicar
padrão sub-store + tag + locatable kind sem reintroduzir mutação
em walk.

**Janela compat encerrada para bib state**: durante P181D-G havia
path duplo (state legacy populado por walk + BibStore populado por
from_tags). Pós-P181H, BibStore é a fonte única; state legacy
existe (M6 elimina) mas vazio em produção. Cite-arm fallback a
state preservado como segurança extra (sem custo prático já que
state está vazio).

**M5 progresso indirecto**: P181H não migra novo consumer
(P181G migrou cite-arm). Mas remove dependência walk-mutation que
bloqueava M6 cleanup.

**P181**:

- `.A`–`.H`: ✅ concluídos.
- `.I` (P181I): pendente — **tests E2E + lacuna #6 fechada**.
  Pré-condição satisfeita: infraestrutura completa (BibStore +
  ElementKind/Payload + locatable + from_tags + trait + Layouter
  consumer + walk puro). P181I valida E2E e marca lacuna como
  resolvida em `m1-lacunas-captura.md`.
- `.J` (P181J): pendente — relatório consolidado.

---

## 6. Pendências cumulativas

P181H restaura invariante mas mantém path legacy 100% funcional.
Sem novas pendências; pré-existentes inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

**M6 elimina**:
- `CounterStateLegacy.bib_entries` e `bib_numbers` (campos
  vazios em produção).
- Copy-sites em `pub fn layout`/`pub fn layout_with_introspector`
  (linhas 1397, 1399, 1425, 1427).
- Cite-arm fallback a state legacy.
- Re-walk em `layout()` legacy quando callers adoptarem
  `introspect_with_introspector + layout_with_introspector`
  directamente.

---

## 7. Estado pós-passo

- **P181H concluído**.
- **P181I desbloqueado**: validar E2E que tudo funciona em conjunto
  e fechar lacuna #6 em `m1-lacunas-captura.md`. Os 3 critérios
  fixados em P181A §2.6 (Opção 3) devem ser todos verdade:

  1. ✅ `01_core/src/entities/bib_store.rs` existe (P181B).
  2. ✅ `Introspector::bib_entry_for_key` + `bib_number_for_key`
     no trait + impl `TagIntrospector` (P181F); `from_tags` arm
     popula `bib_store` (P181E).
  3. ✅ Layouter cite-arm consulta via
     `self.introspector.bib_entry_for_key(...)` /
     `self.introspector.bib_number_for_key(...)` (P181G).

  P181I é validação E2E + actualização de `m1-lacunas-captura.md`.
  Magnitude S.

- **Output observable**: inalterado. PDF e diagnostics idênticos
  a P181G.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `rules/introspect.md ↔ introspect.rs` = `941ad50a ↔ 941ad50a`
    (mesmo hash — não significa erro; ambos hashs são iguais por
    coincidência hexadecimal).
  - `rules/layout.md ↔ layout/mod.rs` = `81cfe96c ↔ 81cfe96c`
    (idem).

P181H restaura invariante walk puro P163 e encerra janela compat
para bib state. Próximo: **P181I** valida E2E e fecha lacuna #6
formalmente.
