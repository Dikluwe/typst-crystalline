# P444 — Relatório: text decorações `underline`, `overline`, `strike`

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** `Tekt`  
> **Foco:** Materializar/fechar as 3 funções nativas de decoração de texto e garantir que `#show underline` / `#show strike` / `#show overline` funcionam.

---

## Resumo executivo

A **infraestrutura central** das decorações de texto já tinha sido implementada nos passos anteriores **P284–P286**:

- `01_core/src/rules/stdlib/text.rs` — `native_underline`, `native_strike`, `native_overline`.
- `01_core/src/entities/content.rs` — variantes `Content::Underline`/`Strike`/`Overline` e construtores.
- `01_core/src/rules/layout/decorations.rs` — layout wrap-aware que emite `FrameItem::Line` por segmento.
- `01_core/src/rules/layout/mod.rs` — braço `Content::Underline/Strike/Overline` delega a `decorations::layout`.
- `01_core/src/rules/eval/mod.rs` — registo das funções no scope stdlib.
- `00_nucleo/prompts/rules/stdlib/text.md` — spec L0 das três funções.
- Testes de unidade (stdlib) e de integração (layout) já existiam.

O **trabalho restante do P444** era habilitar os **selectors de show rule** para as três decorações (`#show underline: ...`, `#show strike: ...`, `#show overline: ...`). Isso foi feito adicionando `NodeKind::Underline/Strike/Overline` e respectivos matches no `selector_matches` e na resolução de funções nativas em `eval_show_rule`.

**Resultado:** P444 fechado; `cargo test --workspace` verde; `crystalline-lint .` sem novas violações.

---

## 1. Mudanças de código do P444

### 1.1 `01_core/src/entities/show.rs`

- Adicionados 3 variants a `NodeKind`:
  - `NodeKind::Underline`
  - `NodeKind::Strike`
  - `NodeKind::Overline`
- Actualizado o comentário do enum para reflectir o conjunto completo.

### 1.2 `01_core/src/rules/eval/rules.rs`

- `selector_matches`: o braço `Selector::NodeKind` agora casa `Content::Underline(_)` com `NodeKind::Underline`, `Content::Strike(_)` com `NodeKind::Strike` e `Content::Overline(_)` com `NodeKind::Overline`.
- `eval_show_rule`: ao resolver um selector que é uma função nativa, adicionado mapeamento por function-pointer para `native_underline`, `native_strike` e `native_overline`, produzindo os respectivos `Selector::NodeKind(...)`.
- Mensagem de erro actualizada para listar os tipos suportados incluindo as novas decorações.
- Adicionados 4 testes unitários no módulo `tests`:
  - `p444_selector_underline_casa_content_underline`
  - `p444_selector_strike_casa_content_strike`
  - `p444_selector_overline_casa_content_overline`
  - `p444_selector_decoration_nao_casa_texto_plano`

---

## 2. Estado pré-existente (P284–P286)

| Componente | Ficheiro | Estado |
|------------|----------|--------|
| Funções nativas | `01_core/src/rules/stdlib/text.rs` | `native_underline`/`strike`/`overline` com `body`, `stroke`, `offset`, `extent` |
| Variantes de `Content` | `01_core/src/entities/content.rs` | `Underline`/`Strike`/`Overline` com `UnderlineElem`/`StrikeElem`/`OverlineElem` |
| Layout | `01_core/src/rules/layout/decorations.rs` | Emite `FrameItem::Line` por segmento, wrap-aware |
| Dispatch layout | `01_core/src/rules/layout/mod.rs` | Braços delegam a `decorations::layout` |
| Registo stdlib | `01_core/src/rules/eval/mod.rs` | `scope.define("underline", ...)` etc. |
| Spec L0 | `00_nucleo/prompts/rules/stdlib/text.md` | Secção `underline / strike / overline` |
| Tests L1 | `01_core/src/rules/stdlib/mod.rs` | P284 tests de unidade |
| Tests L3 | `01_core/src/rules/layout/tests.rs` | P284–P286 tests de integração |

Não houve necessidade de duplicar esta infraestrutura nem de introduzir `Style::Underline/Overline/Strike` (o modelo adoptado pelo projecto usa variantes próprias de `Content`, analogamente a `strong`/`emph`).

---

## 3. Verificação

### 3.1 `cargo test --workspace`

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam**, incluindo:
- 4 novos tests de selector em `rules/eval/rules.rs`.
- Tests pré-existentes P284–P286 de stdlib e layout.
- Pipeline completo (`typst-core`, `typst_shell`, `typst_infra`, `typst_wiring`, CLI, `crystalline_lint`).

### 3.2 `crystalline-lint .`

Resultado: **zero novas violações**. Apenas os 2 warnings órfãos de prompts pre-existentes.

---

## 4. Scope-out preservado

- `offset` avançado, `extent`, `evade`, `background` e `stroke` como objecto `Stroke` rico continuam scope-out (já documentados em P284).
- `DEBT.md` não foi alterado (não abre nem fecha dívida).

---

## 5. Commits

- Branch: `Tekt`
- Commit: `P444: selectors de show rule para underline/strike/overline`

Alterações incluídas no commit:
- `01_core/src/entities/show.rs`
- `01_core/src/rules/eval/rules.rs`
- `00_nucleo/materialization/typst-passo-444-relatorio.md`
- `00_nucleo/materialization/typst-passo-444.md`
