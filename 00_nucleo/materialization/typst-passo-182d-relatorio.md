# Relatório — Passo 182D

**Data**: 2026-05-02
**Passo**: P182D — Layouter consumers via `Introspector` (substitution-with-fallback)
**Magnitude**: S (executada ~ 90 LOC: 2 edits inline + 3 tests E2E + 1 secção L0)
**Resultado**: 2 consumers Layouter migrados (heading-arm + equation-arm) com fallback legacy preservado; output observable inalterado em produção; tests verdes.

---

## 1. Resumo

`Layouter::layout_content` arm `Content::Heading` em `01_core/src/rules/layout/mod.rs:301` e `Layouter::layout_equation` em `01_core/src/rules/layout/equation.rs:24` foram migrados para o padrão substitution-with-fallback P168/P181G:

```rust
// heading prefix
let numbering_on = self.introspector
    .is_numbering_active("numbering_active:heading")
    || self.counter.is_numbering_active("heading");

// equation auto-numeração
let is_numbered = block
    && (self.introspector.is_numbering_active("numbering_active:equation")
        || self.counter.is_numbering_active("equation"));
```

Trait `Introspector` importado localmente em cada arm (replica padrão P181G `mod.rs:589`).

Para heading: ambos paths (Introspector populado via P182C + legacy populado via walk arm `introspect.rs:455–457` e write paralelo `layout/counters.rs:11–13`) devolvem o mesmo bool — fallback é redundante mas inofensivo. Output observable inalterado.

Para equation: `numbering_active:equation` não tem emitter em P182 (cristalino não tem variant `Content::SetEquationNumbering`). Introspector retorna sempre `false`; fallback legacy é o caminho activo. Output observable inalterado.

---

## 2. Confirmação `.F` (10 verificações)

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ (mesmas 2 warnings pré-existentes em `foundations.rs:355–359`, não tocadas) |
| 2 | `cargo test --workspace --lib` Δ vs P182C baseline 1.748 | ✅ (+3 → 1.751) |
| 3 | `crystalline-lint .` zero violations | ✅ (após `--fix-hashes`) |
| 4 | Layouter heading-arm consulta Introspector primeiro, fallback legacy | ✅ (`mod.rs:301–311`: `self.introspector.is_numbering_active("numbering_active:heading") \|\| self.counter.is_numbering_active("heading")`) |
| 5 | Layouter equation-arm consulta Introspector primeiro, fallback legacy | ✅ (`equation.rs:31–33`: `self.introspector.is_numbering_active("numbering_active:equation") \|\| self.counter.is_numbering_active("equation")`) |
| 6 | Walk arm `introspect.rs:455–457` **NÃO modificado** | ✅ (continua write canonical legacy) |
| 7 | Write paralelo `layout/counters.rs:11–13` **NÃO modificado** | ✅ |
| 8 | Copy-sites `layout/mod.rs:1414, 1442` **NÃO modificados** | ✅ |
| 9 | Snapshot tests ADR-0033 verdes (output observable inalterado) | ✅ (incluídos nos 1.751 passed; teste P182D dedicado `p182d_heading_numbering_paridade_legacy_vs_migrated` confirma plain_text idêntico em ambos paths) |
| 10 | Linter passa final | ✅ (`✓ No violations found`) |

---

## 3. Δ tests vs baseline P182C

| Crate | P182C | Após P182D | Δ |
|-------|-------|------------|---|
| `typst-core` (`01_core` lib) | 1.488 | 1.491 | +3 |
| `typst-infra` | 215 | 215 | 0 |
| `typst-shell` | 24 | 24 | 0 |
| `typst-wiring` integration | 21 | 21 | 0 |
| **Total** | **1.748** | **1.751** | **+3** |

Tests novos (em `01_core/src/rules/layout/tests.rs` após `layout_set_heading_numbering_activa_contador`):

1. `p182d_heading_numbering_via_introspector_path` — Introspector pré-populado com `Bool(true)` em `numbering_active:heading`; legacy state vazio (`CounterStateLegacy::default()`); documento sem `Content::SetHeadingNumbering` no AST. Path Introspector dispara prefixo `"1."`. Confirma que o Introspector é consultado primeiro.
2. `p182d_heading_numbering_via_fallback_legacy` — Introspector vazio (`TagIntrospector::empty()`); legacy state pré-populado com `numbering_active["heading"]=true`; documento sem `Content::SetHeadingNumbering` no AST. Fallback `||` dispara prefixo `"1."`. Confirma janela compat M6 funcional.
3. `p182d_heading_numbering_paridade_legacy_vs_migrated` — documento típico (`SetHeadingNumbering + 3 headings`); plain_text de `layout()` legacy igual a `layout_with_introspector` direct. Confirma paridade output observable.

Equation-arm não tem teste dedicado: cristalino não tem emitter para `numbering_active:equation`, logo o path Introspector retorna sempre `false` e o fallback legacy é exercitado pelos tests existentes em `layout/tests.rs` que cobrem equation numbering. Adicionar teste isolado seria circular (equivaleria a verificar que `false || X == X`).

---

## 4. Hashes finais de L0s modificados

| Ficheiro | `@prompt-hash` (header `.rs`) | "Hash do Código" (linha 2 do L0) |
|----------|-------------------------------|----------------------------------|
| `rules/layout` | `81cfe96c` → **`59811524`** | (anterior) → **`10004310`** |

Header `@prompt-hash` actualizado em **9 ficheiros** que partilham o L0 `rules/layout.md`:
- `mod.rs`
- `equation.rs`
- `cursor.rs`
- `grid.rs`
- `helpers.rs`
- `hyphenation.rs`
- `metrics.rs`
- `placement.rs`
- `tests.rs`

Apenas `mod.rs`, `equation.rs` e `tests.rs` foram modificados manualmente em P182D; os outros 6 receberam apenas update de hash pelo lint (`--fix-hashes`) — efeito colateral natural do partilharem o mesmo L0.

---

## 5. Decisões de execução notáveis

### 5.1 Trait `Introspector` importado localmente em cada arm

Replica padrão P181G (`mod.rs:589`): `use crate::entities::introspector::Introspector;` dentro do arm em vez de no top do file. Razão: scoping mínimo — método trait `is_numbering_active` é usado apenas em 2 sítios discretos; import local evita poluir o namespace do file inteiro. Revertível trivialmente para top-level se M6 simplificar a API.

### 5.2 `numbering_active:equation` sem emitter — gate trivial documentado

Auditoria `.A` confirmou: `Content::SetEquationNumbering` não existe em cristalino. P182C cobriu apenas `SetHeadingNumbering`. Em P182D, o Introspector retorna sempre `false` para `numbering_active:equation`; fallback legacy é o caminho activo. Documentado em comentário inline `equation.rs:24–29` e em L0 `layout.md` secção P182D.

Esta divergência face a heading não regride paridade: os tests existentes que cobrem equation numbering (`tests.rs:899` injecta directamente em `state.numbering_active["equation"]`) continuam a passar — o fallback `||` cobre. Quando algum dia equation set rule for materializada (fora P182), o emitter para `numbering_active:equation` reusa P182C literalmente — sem refactor de consumer.

### 5.3 Fallback redundante mas inofensivo para heading

Em produção, ambos `state.numbering_active["heading"]` (legacy) e `StateRegistry["numbering_active:heading"]` (P182C) são populados pelo mesmo `Content::SetHeadingNumbering` no documento. Quando o documento contém o variant, ambos retornam `true`; quando não contém, ambos retornam `false`. O `||` é logicamente equivalente a um único path — mas a redundância garante (a) preservação durante janela compat M6, (b) graceful degradation se um dos paths regredir num passo futuro, (c) robustez face a callers que passem `state` ou `introspector` parcialmente populados.

### 5.4 Sem decisão substancial. Sem ADR. Sem DEBT.

---

## 6. Estado actual

- **P182 série**: A ✅ | B ✅ | C ✅ | D ✅ | E–F pendentes.
- **M9**: 10/11 features (inalterado — feature `numbering_active` só conta como fechada após P182F encerrar lacuna #4).
- **Lacuna #4**: infra trait method (P182B) + arm extract_payload com locatable + auto-init from_tags (P182C) + 2 consumers Layouter migrados (P182D); falta tests E2E pipeline+Layouter dedicados (P182E) e fecho (P182F).
- **M5 progresso (consumers Introspector)**: figure-ref P168 + cite-arm P181G + heading-arm P182D + equation-arm P182D = 4 consumers migrados (heading e equation contam como 2 distintos por chave/arm). Consumers restantes em outras features: leituras intra-walk (`introspect.rs:360, 378`) **não migram** — consomem state local que o walk constrói; outros calls a `state.is_numbering_active(...)` são apenas em testes.
- Tests workspace: **1.751 verdes** (Δ +3 vs P182C; cumulative +13 vs baseline P181J 1.738).
- Lint: **zero violations**.

---

## 7. Pendências cumulativas

Inalteradas face a P182A §3 cláusula 6 (Opção 3):

- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Walk arm canonical `introspect.rs:455–457` continua write legacy.
- Write paralelo `layout/counters.rs:11–13` continua.
- Copy-sites `mod.rs:1414, 1442` continuam.
- Leituras intra-walk `introspect.rs:360, 378` continuam (não migram).
- Fallback `|| self.counter.is_numbering_active(...)` em ambos arms migrados — eliminado em M6.

Pendência adicional desde P182C (decisão 5.1): `from_tags::StateUpdate` auto-init divergência face a P171 strict — documentada; sem regressão observada.

---

## 8. Próximo passo

**P182E** — Tests E2E pipeline completo confirmando paridade Introspector vs legacy.

Escopo concreto:
1. Test pipeline completo `eval → walk → from_tags → layout_with_introspector` para documento típico com `#set heading(numbering: "1.1")`. Confirmar plain_text contém prefixos correctos.
2. Test de regressão: `#set heading(numbering: "1.1")` seguido de `#set heading(numbering: none)` (re-update; auto-init na primeira occurrence + update normal na segunda) — confirmar que o segundo heading sai sem prefixo.
3. Test de paridade snapshot: documento complexo com headings + equation block; comparar output `layout()` legacy vs `layout_with_introspector` directo.
4. (Opcional) Test que verifica que `walk` continua a popular `state.numbering_active` legacy paralelo (regressão evitada).

Escopo NÃO inclui: modificação de produção; remoção de fallback (M6); refactor de consumer.

Magnitude **S**. Sem dependência fora de P182D.
