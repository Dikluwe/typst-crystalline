# Prompt L0 — P792 — `layout()`, `text.lang`, `here().position()`
Hash do Código: 51a5c754

**Camada**: L1
**Ficheiros alvo**:
- `01_core/src/engine/stdlib/layout.rs` — nova função `native_layout`
- `01_core/src/engine/eval/mod.rs` — registo de `layout` no scope global
- `01_core/src/engine/eval/closures.rs` — intercepção `here().<método>()`
- `01_core/src/engine/eval/bindings.rs` — intercepção `text.<campo>` em field access
- `01_core/src/entities/content.rs` — nova variante `Content::Layout`
- `01_core/src/engine/layout/mod.rs` — arm `Content::Layout` no walk do Layouter

**Origem**: Passo 792 (achados de P786 confirmados em sonda).
**ADRs relevantes**: ADR-0033 (paridade observable), ADR-0107 (paridade linguagem), ADR-0108 (medir-antes-de-decidir), ADR-0109 (atomização forma B).

---

## 0. Medição de base (ADR-0108 — proveniência)

**Commit da medição**: `0774275fe` (HEAD → Tekt)
**Estado**: working tree limpa (0 ficheiros alterados).

As três divergências são **independentes** — causas raiz distintas, mecanismos de correção distintos.

---

## 1. Divergência A — `layout()` é `unknown variable`

A função nativa global `layout` não está registada no scope. Omissão simples.

**Semântica**: `layout(size => content)` — materializa `Content::Layout { func }`.

---

## 2. Divergência B — `text.lang` → `cannot access fields on type function`

`text` é `Value::Func(native)` sem namespace. Intercepção em `eval_field_access` para `Func("text")`.

---

## 3. Divergência C — `here().position()` → `cannot access fields on type location`

`Value::Location` não tem arm em `eval_func_call`. Intercepção para métodos `.page()`, `.position()`, `.page-numbering()`.

---

## 4. Critérios

```bash
./target/release/typst /tmp/p792-layout2.typ    # exit 0
./target/release/typst /tmp/p792-textlang.typ   # exit 0
./target/release/typst /tmp/p792-position.typ   # exit 0
cargo test --workspace
crystalline-lint .
```
