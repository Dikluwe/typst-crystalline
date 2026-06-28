# Relatório P487-D — Audit Final + Fix de Bloqueadores (XS)

**Data:** 2026-06-28
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P487-D (opção XS: audit final + resolução de conflitos + limpeza)
**Magnitude:** XS (bloqueadores operacionais, sem nova feature)

---

## 1. Resumo

P487-D resolveu três categorias de bloqueadores encontrados após `git stash pop`
do stash "temp P465":

| Categoria | Bloqueador | Resolução |
|-----------|------------|-----------|
| **Conflitos de merge** | `repr.rs`, `introspect.rs`, `value.md` com markers `<<<<<<<` | Upstream mantido (arquitetura correcta) |
| **Stack overflow em testes** | 3429 testes de L1 + testes de profundidade (MAX_CALL_DEPTH=80, MAX_SHOW_RULE_DEPTH=64) crashavam com SIGABRT | `RUST_MIN_STACK=33554432` em `.cargo/config.toml` |
| **Sentinelas stale** | `parse_color_nomes_conhecidos` (purple), `p299_math_module_total_42` (+1 op) | Actualizados |
| **Snapshot regredido** | `09-cidfont.pdf` 511 bytes maior após P486 x_offset | Regenerado com Noto Sans + krilla fonts |
| **Artefactos de stash** | `labelled.rs` deletado, prompt `labelled.md` órfão | Limpeza: ficheiro + prompt removidos |
| **Hash drift V5** | `value.rs` @prompt-hash desatualizado pós-conflict-fix | `crystalline-lint --fix-hashes .` |

**Resultado final:** `cargo test --workspace` 3429+526+... = 100% verde; `crystalline-lint .` 0 violations.

---

## 2. Conflitos de merge (ADR-0108)

### 2.1 Medições antes de decidir

| Ficheiro | Linha | Conflito | Decisão | Razão |
|---------|-------|---------|---------|-------|
| `01_core/src/rules/eval/repr.rs` | 444 | 2 testes `Value::Relative` (upstream) vs 0 (stash) | UPSTREAM | Testes P469 pertencem ao upstream |
| `01_core/src/rules/introspect.rs` | 3455 | `labelled_prod()` (upstream) vs `Content::label_auto()` (stash) | UPSTREAM | `labelled_prod` levanta Styled para `compute_labelled` |
| `01_core/src/rules/introspect.rs` | 3491 | P480 tests (upstream) vs deletados (stash) | UPSTREAM | P480 testes validam `Outline` kind_index |
| `00_nucleo/prompts/entities/value.md` | 7 | Texto actualizado P466/P469 (upstream) vs só P465 (stash) | UPSTREAM | Upstream tem histórico completo |

**Critério de decisão:** upstream = HEAD (commits P466–P486 já aplicados); stash = P465 incompleto.

### 2.2 Porquê `labelled_prod` e não `Content::label_auto`

`labelled_prod` (`introspect.rs:1286`) cria a estrutura:
```
Content::Label { label, auto: true, target: Content::heading_numbered(...) }
```
onde o `Heading` não está embrulhado em `Styled`. `compute_labelled` exige o
target puro sem `Styled` para resolver o texto do label auto. `Content::label_auto`
(stash) embrulharia no `Styled` do numbering, quebrando `compute_labelled`.

---

## 3. Stack overflow em testes de recursão

### 3.1 Diagnóstico

`Route::MAX_CALL_DEPTH = 80` e `MAX_SHOW_RULE_DEPTH = 64` são suficientes para parar
recursão infinita. Mas 80 frames do evaluator em modo debug ocupam >8MB de stack
(default Linux: 8MB). Resultado: SIGABRT antes do `Err` chegar ao assert.

Confirmação: com `RUST_MIN_STACK=67108864` (64MB), todos os testes passam.

### 3.2 Solução

`.cargo/config.toml` — secção `[env]`:

```toml
[env]
# P487: evaluador recursivo em debug mode usa ~100KB/frame × 80 frames = 8MB.
# 32MB evita crash nos testes de profundidade (Route::MAX_CALL_DEPTH = 80,
# MAX_SHOW_RULE_DEPTH = 64). Não afecta produção (só test threads).
RUST_MIN_STACK = "33554432"
```

32MB é suficiente (comprovado com 67MB); 32MB deixa margem sem desperdiçar.
Afecta apenas threads de teste — não há impacto em produção.

Testes afectados (não-exaustivo):
- `recursao_infinita_retorna_err_sem_crash`
- `recursao_profunda_retorna_err`
- `p350c_flag_on_nao_convergente_classifica`

---

## 4. Sentinelas stale

### 4.1 `parse_color_nomes_conhecidos`

P477 adicionou "purple" → `Some(Color::rgb(128, 0, 128))` em `shapes.rs:44`.
O teste `parse_color_nomes_conhecidos` ainda esperava `None` para "purple".

**Fix:** `assert_eq!(parse_color(&Value::Str("purple".into())), Some(Color::rgb(128, 0, 128)));`

### 4.2 `p299_math_module_total_42_operadores`

Um operador matemático foi adicionado pós-P299. Contagem actual: 43.

**Fix:** `assert_eq!(d.len(), 43, "P299+: 42 vanilla + 1 adicionado pós-P299");`

---

## 5. Snapshot `09-cidfont.pdf`

P486 adicionou `x_offset` ao TJ operator em `emit_shaped_pdf`. O fixture
"09-cidfont.typ" usa Noto Sans com caracteres acentuados (à é ç ñ ü) — rustybuzz
produz glifos com `x_offset != 0` (kern marks). PDF resultante: 559717B (vs 559206B
anterior = +511B).

**Fix:** regenerado via `cargo run -p typst-wiring -- 09-cidfont.typ --font-path lab/krilla-reference/assets/fonts`.

---

## 6. Limpeza de artefactos do stash P465

O stash "temp P465" (criado durante P464/P465) deletou `labelled.rs` e o prompt
`labelled.md` como parte da consolidação `Labelled → Content::Label` (P464).

| Artefacto | Estado pré-P487 | Acção P487 |
|-----------|-----------------|------------|
| `01_core/src/entities/elements/labelled.rs` | Staged como deleted pelo stash pop | Mantido (é correcto — P464 consolidou) |
| `00_nucleo/prompts/entities/elements/labelled.md` | V7 OrphanPrompt (ficheiro alvo deletado) | Removido |

O `elements/mod.rs` já tinha o comentário `// P464: labelled consolidado em label`.

---

## 7. Hash drift V5

`value.md` foi actualizado durante a resolução do conflito (upstream mais completo).
O `@prompt-hash` em `value.rs` ficou desatualizado.

**Fix:** `crystalline-lint --fix-hashes .` actualizou bidirecionalmente:
- `value.rs` → `@prompt-hash 0a2085a5`
- `value.md` → `Hash do Código: d0ab6632`

---

## 8. Ficheiros alterados em P487-D

| Ficheiro | Alteração |
|---------|-----------|
| `.cargo/config.toml` | `[env] RUST_MIN_STACK = "33554432"` |
| `00_nucleo/prompts/entities/elements/labelled.md` | DELETED (prompt órfão V7) |
| `00_nucleo/prompts/entities/value.md` | Conflito resolvido (upstream) + hash fix |
| `01_core/src/entities/elements/labelled.rs` | DELETED (stash P464 consolidação) |
| `01_core/src/entities/value.rs` | `@prompt-hash` actualizado (`0a2085a5`) |
| `01_core/src/rules/eval/repr.rs` | Conflito resolvido (upstream — P469 Relative tests) |
| `01_core/src/rules/introspect.rs` | Conflito resolvido (upstream — `labelled_prod` + P480 tests) |
| `01_core/src/rules/stdlib/mod.rs` | Sentinela `parse_color` ("purple") + `p299` count 42→43 |
| `03_infra/fixtures/p307b/reference/09-cidfont.pdf` | Snapshot regenerado (P486 x_offset) |

---

## 9. Resultados finais

```
cargo test --workspace
  typst-core: 3429 passed; 0 failed
  typst-infra: 526 passed; 0 failed; 6 ignored
  (outros): 47 passed
  Total: ✅ 0 falhas

crystalline-lint .
  ✅ No violations found (0 erros V1–V14)
```

---

## 10. Estado pós-P487

| Indicador | Estado |
|-----------|--------|
| DEBTs activos | 0 |
| Paridade (parity suite) | 73/73 matches; 0 diffs; 0 errors |
| `cargo test --workspace` | ✅ 100% verde |
| `crystalline-lint .` | ✅ 0 violations |
| **P487-D** | **FECHADO** |

| Trilha | Estado |
|--------|--------|
| Trilha 1 — Core types | ✅ |
| Trilha 2 — Eval | ✅ |
| Trilha 3 — Show/Set rules | ✅ |
| Trilha 4 — Layout | ✅ |
| Trilha 5 — TextShaped/rustybuzz | ✅ P482–P486 |
| Trilha 6 — LoF/LoT page numbers | 4/5 (2-pass convergent: DEBT longo prazo) |
| Trilha 7 — Outline/ToC | ✅ |
| Trilha 8 — Bibliography | ✅ |

---

## 11. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P488-A** | `y_offset` em emit se corpus RTL mostrar necessidade | S |
| **P488-B** | Expansão corpus lab/parity com ficheiros RTL (árabe/hebraico) | S |
| **P488-C** | Trilha 6 Fase 5 — LoF/LoT page numbers (2-pass convergente) | M–L |
