# Relatório P473 — `op. cit.` + wiring de `#show regex(...)`

**Data:** 2026-06-26
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P473 (Trilhas 6 + 3)
**Materialização:** Implementação + testes + specs L0

---

## 1. Resumo

**Sub-item A — `op. cit.`** (Trilha 6):
P472 materializou `ibid.` para citações *consecutivas* da mesma entry. P473 adiciona `op. cit.` para o caso complementar: a mesma key é citada de novo, mas não consecutivamente (outra entry foi citada entretanto).

Campo `previously_cited_keys: HashSet<String>` adicionado ao `Layouter`. Em `cite.rs`, antes de actualizar `last_cited_key`, a key anterior é inserida em `previously_cited_keys`. A detecção é: `Numeric + Normal + NOT ibid + key ∈ previously_cited_keys`. O render produz `[N] Author, op. cit.`.

**Sub-item B — `#show regex(...)`** (Trilha 3):
Sonda confirmou que **toda a infra já estava implementada desde P393**: `Value::Regex`, `Selector::Regex`, `native_regex`, wiring em `eval_show_rule`, aplicação em `apply_show_rules`, e testes L2. P473 confirma o funcionamento e actualiza o L0 `show-regex.md` como materializado.

---

## 2. Sondas pré-implementação (ADR-0108)

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `last_cited_key` existe no Layouter? | Sim — P472 | `layout/mod.rs:347` |
| `previously_cited_keys` existe? | Não | `layout/mod.rs:347` (ausente) |
| `is_op_cit` detectado? | Não | `layout/cite.rs:52` (ausente) |
| `Value::Regex` existe? | Sim — P393 | `entities/value.rs:100` |
| `Selector::Regex` em `show.rs`? | Sim — P393 | `entities/show.rs:65` |
| `native_regex` implementado? | Sim — P393 | `rules/stdlib/text.rs:559` |
| `native_regex` no scope? | Sim — P393 | `rules/eval/mod.rs:808` |
| `apply_show_rules` aplica `Selector::Regex`? | Sim — P393 | `rules/eval/rules.rs:498–555` |
| `eval_show_rule` mapeia `Value::Regex`? | Sim — P393 | `rules/eval/rules.rs:1043` |
| Testes L2 de show regex existem? | Sim — P393 | `eval/tests.rs:4682–4757` |

**Conclusão:** Sub-item B não requer código novo. Sub-item A requer `previously_cited_keys` + lógica `is_op_cit` em `cite.rs`.

---

## 3. Sub-item A — `op. cit.`

### Campo `previously_cited_keys` no Layouter

```rust
// 01_core/src/engine/layout/mod.rs — struct Layouter (após last_cited_key)
/// **P473** — conjunto de todas as keys citadas antes da posição actual,
/// excluindo a última (coberta por last_cited_key). Usada para op. cit.:
/// key já citada mas não consecutivamente → "[N] Author, op. cit.".
pub(super) previously_cited_keys: std::collections::HashSet<String>,
```

Inicializado em `Layouter::new()`:

```rust
previously_cited_keys: std::collections::HashSet::new(),
```

### Detecção e render em `cite.rs`

Antes das actualizações de estado:

```rust
let is_ibid = style == CitationStyle::Numeric
    && form == CitationForm::Normal
    && layouter.last_cited_key.as_deref() == Some(key.as_str());

let is_op_cit = style == CitationStyle::Numeric
    && form == CitationForm::Normal
    && !is_ibid
    && layouter.previously_cited_keys.contains(key.as_str());

// Actualizar estado (ordem obrigatória):
if let Some(prev) = layouter.last_cited_key.take() {
    layouter.previously_cited_keys.insert(prev);
}
layouter.last_cited_key = Some(key.clone());
```

Render quando `is_op_cit`:

```rust
if is_op_cit {
    let n_str = citation_number_for_key(intr, key)
        .or_else(|| bib_number_for_key(intr, key))
        .map(|n| n.to_string())
        .unwrap_or_else(|| key.clone());
    let author_abbrev = entry
        .map(|e| e.author.split(',').next().unwrap_or(&e.key).to_string())
        .unwrap_or_else(|| key.clone());
    let text = format!("[{}] {}, op. cit.", n_str, author_abbrev);
    layouter.layout_content(&Content::text(text));
    // supplement se presente
    return;
}
```

Resultado: `[1] Knuth, op. cit.` quando `BibEntry.author = "Knuth, D."`.

### Invariante de exclusão mútua

| Condição | Forma |
|----------|-------|
| `last_cited_key == current_key` | `ibid.` |
| `previously_cited_keys.contains(current_key)` AND NOT ibid. | `[N] Author, op. cit.` |
| Nenhuma das anteriores | render normal `[N]` |

---

## 4. Sub-item B — `#show regex(...)` (confirmação P393)

Toda a infra estava implementada:

| Componente | Ficheiro | Linha |
|-----------|---------|-------|
| `Value::Regex(Regex)` | `entities/value.rs` | 100 |
| `type_name()` → `"regex"` | `entities/value.rs` | 187 |
| `From<Regex> for Value` | `entities/value.rs` | 382 |
| `Selector::Regex(Regex)` | `entities/show.rs` | 65 |
| `native_regex` | `stdlib/text.rs` | 559 |
| `scope.define("regex", ...)` | `eval/mod.rs` | 808 |
| `Value::Regex(re) => Selector::Regex(re)` | `eval/rules.rs` | 1043 |
| `apply_show_rules` regex loop | `eval/rules.rs` | 498–555 |
| Testes L2 P393 | `eval/tests.rs` | 4682–4757 |

`show-regex.md` L0 actualizado com nota de materialização P393+P473.

---

## 5. Arquivos alterados

### Código de produção (modificados)

- `01_core/src/engine/layout/mod.rs` — campo `previously_cited_keys: HashSet<String>` + inicialização
- `01_core/src/engine/layout/cite.rs` — `is_op_cit` detection + render `[N] Author, op. cit.`

### Testes (adicionados)

- `01_core/src/engine/eval/tests.rs` — 3 testes P473 em secção `// ── P473`

### Specs L0 (actualizadas)

- `00_nucleo/prompts/engine/atomizacao_elementos.md` — §14 `op. cit. + previously_cited_keys`; scope-out de P472 corrigido
- `00_nucleo/prompts/engine/show-regex.md` — nota de materialização P393+P473

---

## 6. Resultados dos testes

### Testes específicos P473 (3 novos)

```
rules::eval::tests::tests::p473_op_cit_detectado_apos_citacao_intercalada   ok
rules::eval::tests::tests::p473_ibid_nao_confundido_com_op_cit              ok
rules::eval::tests::tests::p473_primeira_citacao_nunca_e_op_cit             ok
```

### Suite completa

```
test result: ok. 3371 passed; 0 failed; 11 filtered (stack-overflow pré-existentes)
```

### `cargo build --workspace`

```
Finished `dev` profile — 0 errors
```

### `crystalline-lint --fix-hashes .`

```
./01_core/src/engine/layout/cite.rs → e6442e3f
Re-running analysis... ✅ 0 drift warnings remaining
```

---

## 7. Scope-out explícito (documentado no L0)

| Área | Scope-out |
|------|-----------|
| **`op. cit.` em estilos não-numéricos** | Apenas `Numeric + Normal`. AuthorDate/Prose/Author/Year não usam op. cit. |
| **`ibid., p. N` / `op. cit., p. N`** | Page override via supplement — scope-out. |
| **`loc. cit.`** | Forma não implementada. |
| **Reset em nova secção** | `previously_cited_keys` não é resetado por pagebreak ou heading. |
| **Split interno de nó de texto por regex** | `apply_show_rules` transforma o nó inteiro que casa; split por match é scope-out de P393. |

---

## 8. Critério de fecho

- [x] Sondas com `file:line` para todos os pontos (Sub-item B confirmado já implementado).
- [x] `previously_cited_keys: HashSet<String>` adicionado ao Layouter.
- [x] Inicializado em `Layouter::new()`.
- [x] Arm de `Content::Cite` actualiza `previously_cited_keys` antes de `last_cited_key`.
- [x] `is_op_cit` detectado; render `[N] Author, op. cit.`.
- [x] `ibid.` e `op. cit.` mutuamente exclusivos por construção.
- [x] Sub-item B confirmado funcional (P393): `Value::Regex`, `Selector::Regex`, `native_regex`, wiring completo.
- [x] 3 testes P473 verdes.
- [x] 2 L0 prompts actualizados + `crystalline-lint --fix-hashes` zero drift.
- [x] `cargo build --workspace` verde; `crystalline-lint` zero violations.
- [x] **Trilha 6: 4/5 completo** (citação numérica P468 + back-refs/ibid/LoF/LoT P472 + op. cit. P473).
- [x] **Trilha 3: 2/3 completo** (sonda `Selector::Where` P467 + `#show regex(...)` P393/P473).

---

## 9. Próximo passo recomendado

**Trilha 6** — restante: `LoF/LoT com page numbers` (requer 2-pass convergente — DEBT longo prazo).

**Trilha 3** — restante: `Selector::Where` + `#show heading.where(level: N)`. A sonda P467 confirmou que `Selector::Where` está implementado em `entities/selector.rs` mas o wiring em show-rules (`eval_show_rule`) pode faltar. Candidato para P474.

**Trilha 8** — 5/8 completo. Restante: sonda real de `pad`/`corners`/`sides` antes de propor trabalho.

Opções para P474:
- **Trilha 3** — `Selector::Where` + `#show elem.where(field: value)` (fecha Trilha 3).
- **Trilha 8** — sonda `pad`/`corners`/`sides` (verifica se há itens pendentes reais).
