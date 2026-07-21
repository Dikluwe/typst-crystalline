# Relatório — typst-passo-802 (achado P798 #5): `utils::listset` — falta warning de label não-anexada

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-802.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799–P801 (zonas não relacionadas). Validação "depois" com working tree não commitado: P799–P802. Hora da validação: 2026-07-21 ~16:20 -0300.
**Binários:** `./target/release/typst` (rebuild 16:20), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Fonte original de P798 (`temp/p798/5_listset.typ`): `<abc> Hello #context query(<abc>)`

Comandos: `./target/release/typst -o <out>.pdf temp/p798/5_listset.typ 2>&1` / `lab/typst-original/target/release/typst compile temp/p798/5_listset.typ <out>.pdf 2>&1`.

- cristalino (antes): **sem nenhum warning** (stderr vazio).
- vanilla:
  ```
  warning: label `<abc>` is not attached to anything
    ┌─ temp/p798/5_listset.typ:1:0
    │
  1 │ <abc> Hello #context query(<abc>)
    │ ^^^^^
  ```

Confirmado: vanilla avisa, cristalino cala.

Medições extra de enquadramento (controlo):
- `Hello <abc>` (label depois de texto): vanilla **não avisa** e `query(<abc>).len()` = 1 (label anexa ao nó de texto); cristalino não avisa mas query = 0 — divergência separada já registada em P791 §6 ("label em nó de texto não indexado pelo introspector"), **fora do âmbito** deste passo.
- `= Title <abc>`: ambos sem warning, query = 1.

## Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-eval/src/markup.rs:52-72`: ao avaliar `Value::Label` em markup, procura na sequência acumulada o último nó anexável (`!node.can::<dyn Unlabellable>()`); se não existir:

```rust
vm.engine.sink.warn(warning!(
    expr.span(),
    "label `{}` is not attached to anything",
    label.repr(),
));
```

**Cristalino** — `01_core/src/engine/eval/mod.rs`, braço `SyntaxKind::Label` de `eval_markup` (antes): recolhe espaços finais, faz `parts.pop()`; se vazio, "Se parts estiver vazio após remover espaços, ignorar." — descarte silencioso (e os espaços recolhidos perdiam-se).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/eval.md`, nova bullet "Warning de label órfã (P802)". Hashes corrigidos com `crystalline-lint --fix-hashes .` (`eval/mod.rs`, `eval/tests.rs` → `3f09960d`).

Diff de `01_core/src/engine/eval/mod.rs` (braço `SyntaxKind::Label`):

```diff
                     } else {
-                    // Se parts estiver vazio após remover espaços, ignorar.
+                    // P802 — label órfã (sem elemento anterior anexável):
+                    // warning, paridade vanilla `typst-eval/markup.rs`
+                    // ("label `<x>` is not attached to anything"). A label
+                    // é descartada (como no vanilla); os espaços recolhidos
+                    // são re-inseridos para não se perderem.
+                    engine.sink.warn_note(
+                        child.span(),
+                        &format!("label `<{name}>` is not attached to anything"),
+                        "",
+                    );
+                    trailing.reverse();
+                    parts.extend(trailing);
                 }
```

Formato do diagnóstico: o do projecto (uma linha `path:line:col: warning: msg` vs caixa do vanilla) — conforme o passo ("adaptado ao formato de diagnóstico já usado no projecto"). O warning vanilla "content labelled multiple times" (2ª label no mesmo elemento) não foi implementado — registado no L0 como fora de âmbito.

## Passo 3 — Validação (depois)

Mesmos comandos:

- cristalino: `/home/.../temp/p798/5_listset.typ:1:0: warning: label `<abc>` is not attached to anything` ✓ (mesmo texto do vanilla)
- controlo `Hello <abc>`: stderr vazio (sem warning) ✓ — paridade com o vanilla medido

Testes novos (escritos primeiro; `label_orfa_emite_warning` falhou antes em `tests.rs:5195`):
- `label_orfa_emite_warning` — `<abc> Hello #context query(<abc>)` emite o warning exacto; eval não é fatal.
- `label_anexada_nao_emite_warning_orfa` — `Hello <abc>` e `= Título <abc>` sem warning de órfã.
- Helper novo `eval_for_test_keep_sink` (devolve o `Sink` para asserções).

Suíte `typst-core` (`cargo test -p typst-core --lib`):
- ANTES (fim de P801): **4321** passed + 1 ignored (total 4322).
- DEPOIS: **4323** passed; 0 failed; 1 ignored (total 4324 = +2 testes novos ✓).

Lint: `crystalline-lint .` → exit 0, zero violações.
