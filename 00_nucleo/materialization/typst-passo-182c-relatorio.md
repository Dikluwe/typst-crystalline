# Relatório — Passo 182C

**Data**: 2026-05-02
**Passo**: P182C — `extract_payload` arm `Content::SetHeadingNumbering`
**Magnitude**: S (executada ~ 130 LOC: 1 arm extract_payload + 1 arm locatable promovido + 1 arm from_tags ajustado + 5 tests + edits L0)
**Resultado**: tag emitida durante walk; `StateRegistry` populado paralelamente a legacy; `Introspector::is_numbering_active("numbering_active:heading")` passa a refletir o estado real do documento.

---

## 1. Resumo

`Content::SetHeadingNumbering` foi promovido a locatable e o seu arm em `extract_payload` produz `ElementPayload::StateUpdate { key: "numbering_active:heading", update: StateUpdate::Set(Box::new(Value::Bool(active))) }`. Walk arm canonical em `introspect.rs:455–457` mantém-se inalterado e continua a popular `state.numbering_active` legacy.

Surpresa empírica revelada na execução: o defensive ignore em `from_tags::StateUpdate Set(value)` (P171 "update sem init é ignorado") bloqueava o caso interno — `Content::SetHeadingNumbering` não tem `Content::State` antecedente. Cláusula gate trivial aplicada (cf. §5): arm `from_tags::StateUpdate` ganha auto-init na primeira ocorrência. Comportamento P171 para userspace preservado (`Content::State` continua a inicializar via arm dedicado; ocorrências subsequentes seguem o caminho normal `update`).

Output observable em produção inalterado: Layouter heading-arm e equation-arm continuam a consultar `self.counter.is_numbering_active(…)` legacy (migração é P182D). Tags adicionais emitidas (Start/End para SetHeadingNumbering) afectam apenas o `TagIntrospector` interno — não há consumer Layouter ainda.

---

## 2. Confirmação `.G` (11 verificações)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ (mesmas 2 warnings pré-existentes em `foundations.rs:355–359`, não tocadas) |
| 2 | `cargo test --workspace` Δ vs P182B baseline 1.743 | ✅ (+5 → 1.748) |
| 3 | `crystalline-lint .` zero violations | ✅ (após `--fix-hashes`) |
| 4 | `extract_payload(&Content::SetHeadingNumbering { .. })` retorna `Some(...)` | ✅ (test `set_heading_numbering_active_true_produz_state_update_bool_true` + simétrico false) |
| 5 | Walk produz tag com payload `StateUpdate` para este variant | ✅ (test `walk_emite_tags_em_paralelo_com_state` ajustado de 4 para 6 tags) |
| 6 | `from_tags` popula `StateRegistry` com chave `numbering_active:heading` | ✅ (auto-init em primeira ocorrência) |
| 7 | `is_numbering_active("numbering_active:heading")` reflecte estado real | ✅ (tests E2E `introspector_set_heading_numbering_active_true_popula_state_registry` + simétrico false) |
| 8 | Walk arm `introspect.rs:455–457` **NÃO modificado** | ✅ (`introspect.rs:455–457` continua write canonical legacy `state.numbering_active.insert("heading", *active)`) |
| 9 | Layouter **NÃO modificado** (esperado em P182D) | ✅ (`layout/mod.rs:301`, `layout/equation.rs:24` continuam consulta legacy) |
| 10 | Snapshot tests ADR-0033 verdes | ✅ (incluídos nos 1.748 passed) |
| 11 | Linter passa final | ✅ (`✓ No violations found`) |

---

## 3. Δ tests vs baseline P182B

| Crate | P182B | Após P182C | Δ |
|-------|-------|------------|---|
| `typst-core` (`01_core` lib) | 1.483 | 1.488 | +5 |
| `typst-infra` | 215 | 215 | 0 |
| `typst-shell` | 24 | 24 | 0 |
| `typst-wiring` integration | 21 | 21 | 0 |
| **Total** | **1.743** | **1.748** | **+5** |

Tests novos:

1. `extract_payload::tests::set_heading_numbering_active_true_produz_state_update_bool_true` — input `active: true` → `Some(StateUpdate { key: "numbering_active:heading", update: Set(Box(Bool(true))) })`.
2. `extract_payload::tests::set_heading_numbering_active_false_produz_state_update_bool_false` — caso simétrico.
3. `locatable::tests::set_heading_numbering_e_locatable` — `is_locatable(SetHeadingNumbering)` retorna `true` para ambos `active: true` e `active: false`; invariante `is_locatable == extract_payload(...).is_some()` preservada.
4. `introspect::tests::introspector_set_heading_numbering_active_true_popula_state_registry` — pipeline E2E (walk + `from_tags`); `intr.is_numbering_active("numbering_active:heading")` retorna `true`.
5. `introspect::tests::introspector_set_heading_numbering_active_false_em_state_registry` — caso simétrico (Bool(false) é registado e propagado; helper retorna false porque o valor explícito é Bool(false), não por estar ausente).

Test pré-existente ajustado: `walk_emite_tags_em_paralelo_com_state` (assertion de `tags.len() == 4` → `== 6` com comentário a explicar que SetHeadingNumbering passou a ser locatable).

---

## 4. Hashes finais de L0s modificados

| Ficheiro | `@prompt-hash` (header `.rs`) | "Hash do Código" (linha 2 do L0) |
|----------|-------------------------------|----------------------------------|
| `rules/introspect/extract_payload` | `a30fd785` → **`e0e41040`** | `1da1c130` → **`8e7cb515`** |
| `rules/introspect/locatable` | `d26cf6ff` → **`186cea9d`** | `bdae0a1f` → **`304746d3`** |
| `rules/introspect/from_tags` | `75237ba7` → **`2010372a`** | `9acddbb4` (inalterado neste passo — apenas L0 mudou; código já hashed pelo lint anterior) |

`crystalline-lint --fix-hashes .` aplicou em single-pass; reportou 0 drift restantes.

---

## 5. Decisões de execução notáveis

### 5.1 Gate trivial inesperado: auto-init em `from_tags::StateUpdate`

P182C diagnóstico (P182A §3 cláusula 1) afirmava: "**`from_tags` arm `StateUpdate` (já existente) cobre**". Verificou-se ser **falso** quando aplicado a state interno (sem `Content::State` antecedente):

- `state_registry::update` (`state_registry.rs:55–61`) é defensivo: se key não foi inicializada, update é silenciosamente ignorado (P171 padrão).
- `Content::SetHeadingNumbering` emite directamente `StateUpdate` sem `Content::State` precedente (não há userspace `state(numbering_active:heading, …)`).

Solução cirúrgica: arm `from_tags::StateUpdate` arm `Set(value)` ganha verificação `value_at(key, loc).is_none()` — se `None`, chama `state.init`; senão, segue o caminho `state.update`. **Não toca** `state_registry` (semântica P171 preservada para userspace `Content::State` + `Content::StateUpdate` sequence). **Não toca** o arm `Content::State` userspace (init dedicado continua autoritativo).

Justificação documentada em comentário `from_tags.rs:160–168` e em L0 `from_tags.md` (entrada P182C no histórico). O comportamento auto-init é genérico — não filtrado por prefixo `numbering_active:*` — mas só dispara quando key ainda não foi vista. Userspace que escreva `#state(...).update(...)` sem `#state(...)` antes ganha auto-init em vez de defensive ignore: divergência consciente face a P171, ainda inferior à de vanilla (que gera erro). Para callers que dependiam do silent ignore como heurística de erro, a divergência é benigna — testam estados não-inicializados via `state_value(key, loc)` directamente, que continua a retornar `None`.

### 5.2 Cláusula gate trivial: `is_locatable(SetHeadingNumbering)` era `false`

Auditoria `.A` confirmou que `Content::SetHeadingNumbering` estava no bloco "Não-locatable (53 variants)" em `locatable.rs:66`. Mover para o bloco "Locatable" foi necessário (sem isso, walk não chama `extract_payload` para este variant — arm seria silencioso). Cobertura passou de 8/48 para 9/47.

### 5.3 Convenção de chave aplicada literalmente

Convenção `numbering_active:<feature>` estabelecida em P182B aplicada em P182C como `numbering_active:heading`. Equation usa a mesma convenção (`numbering_active:equation`) mas P182 actual não cobre o emitter — vanilla `EquationElem.numbering` não tem set rule equivalente em cristalino, e o legacy popula `state.numbering_active["equation"]` apenas em testes (cf. `tests.rs:899`). Layouter equation-arm em P182D consulta a chave dele separadamente; se valor ausente em `StateRegistry`, fallback legacy (P168 substitution-with-fallback) cobre.

### 5.4 Sem decisão substancial. Sem ADR. Sem DEBT.

L0s actualizados (`extract_payload.md`, `locatable.md`, `from_tags.md`) todos via histórico de revisões; nenhum exige ADR nova. Cobertura locatable +1 mas ainda dentro do rationale ADR-0026 (Content como enum fechado).

---

## 6. Estado actual

- **P182 série**: A ✅ | B ✅ | C ✅ | D–F pendentes.
- **M9**: 10/11 features (inalterado — feature `numbering_active` só conta como fechada após P182F encerrar lacuna #4).
- **Lacuna #4**: infra trait method (P182B) + arm extract_payload + auto-init from_tags (P182C) materializados; falta migração consumers Layouter (P182D), tests E2E pipeline+Layouter (P182E), fecho (P182F).
- Tests workspace: **1.748 verdes** (Δ +5 vs P182B; cumulative +10 vs baseline P181J 1.738).
- Lint: **zero violations**.

---

## 7. Pendências cumulativas

Inalteradas face a P182A §3 cláusula 6 (Opção 3):

- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Walk arm canonical `introspect.rs:455–457` continua a popular `state.numbering_active` legacy.
- Write paralelo `layout/counters.rs:11–13` continua.
- Copy-sites `mod.rs:1414, 1442` continuam.
- Leituras intra-walk `introspect.rs:360, 378` continuam a consultar `state.is_numbering_active(…)` directo (estas leituras consomem o `state` local que o walk constrói; não passam pelo Introspector — não migram para P182D).

Layouter heading-arm + equation-arm continuam a usar `self.counter.is_numbering_active(…)` directo — migração via substitution-with-fallback é P182D.

Pendência adicional não esperada (decisão 5.1): `from_tags::StateUpdate` auto-init é divergência face a P171 strict. Documentado em L0; comportamento é benigno para userspace e necessário para state interno P182C. Pode ser revisitado em M6+ se evidência de regressão.

---

## 8. Próximo passo

**P182D** — Layouter heading-arm + equation-arm via `Introspector` com substitution-with-fallback.

Escopo concreto:
1. **`01_core/src/rules/layout/mod.rs:301`** — heading prefix consumer:
   ```rust
   if self.introspector.is_numbering_active("numbering_active:heading")
       || self.counter.is_numbering_active("heading") {
       // ... gerar prefixo numérico
   }
   ```
2. **`01_core/src/rules/layout/equation.rs:24`** — equation auto-numeração:
   ```rust
   let is_numbered = block && (
       self.introspector.is_numbering_active("numbering_active:equation")
       || self.counter.is_numbering_active("equation")
   );
   ```
3. Ambos seguem padrão substitution-with-fallback P168/P181G.
4. **Não modificar** walk arm legacy nem write-sites (`introspect.rs:455–457`, `layout/counters.rs:11–13`, copy-sites em `mod.rs:1414, 1442`) — esses persistem até M6.
5. Tests E2E em P182E confirmam paridade pipeline+Layouter (output PDF idêntico via Introspector e via state legacy).

Magnitude **S**. Ainda zero impacto observable em produção (substitution-with-fallback `||` preserva output via fallback enquanto Introspector tem dados consistentes via P182C).
