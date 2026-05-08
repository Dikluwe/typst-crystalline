# Lab/parity SKIPS — manifest documental

**Data**: 2026-05-08.
**Sub-passo**: P206D.
**Spec**: per `typst-passo-206D.md` §C4.
**Convenção**: cada SKIP documentado com etiqueta +
razão literal. Categorias:

- **SKIP-pre-existing** — skip estabelecido antes de
  P206; preservado.
- **SKIP-feature** — skip por feature não suportada
  cristalino ou vanilla (escopo P206C estrutural).
- **INCLUDE-com-diff** — incluído na matriz mas com
  divergência empírica documentada (não regressão).

---

## §1 SKIP-pre-existing (3 ficheiros)

| Ficheiro | Categoria | Origem | Razão |
|----------|-----------|--------|-------|
| `markup/error.typ` | markup | Pre-P206 (linha 95 `tests/layout_parity.rs`) | Sintaxe inválida intencional: `#{{{broken`. Skip layout/PDF/structural; INCLUDE parse comparison via parity-runner. |
| `code/let.typ` | code | P206C C6 + P206D C4 | Categoria `code` é fora-de-escopo introspection — sem elementos query-able típicos. Smoke test de compilação cristalino preservado em `corpus_completo_p3`. |
| `code/set.typ` | code | P206C C6 + P206D C4 | Idem `let.typ`. Sem `#heading`/`#figure`/`#metadata` no source; queries devolvem 0 em ambos cristalino + vanilla → match trivial. Skip por relevância semântica. |

## §2 SKIP-feature (10 ficheiros)

Categoria `semantic/` é escopo P2 (eval), não P3
(layout) ou structural (introspection). P206C foca em
introspection — semantic SKIP coerente.

| Ficheiro | Categoria | Razão |
|----------|-----------|-------|
| `semantic/array-literal.typ` | semantic | `#let __resultado__ = (1,2,3)` — eval-only fixture. Sem elementos introspection. |
| `semantic/bool-true.typ` | semantic | `#let __resultado__ = true` — eval fixture. |
| `semantic/closure-aplicada.typ` | semantic | Eval fixture closure. |
| `semantic/condicional.typ` | semantic | Eval fixture if/else. |
| `semantic/dict-literal.typ` | semantic | Eval fixture dict. |
| `semantic/float-divisao.typ` | semantic | Eval fixture math op. |
| `semantic/funcao-builtin.typ` | semantic | Eval fixture stdlib call. |
| `semantic/int-aritmetica.typ` | semantic | Eval fixture int op. |
| `semantic/string-concat.typ` | semantic | Eval fixture string op. |
| `semantic/tipo-inspeccao.typ` | semantic | Eval fixture `type()`. |

Cobertura semantic → `eval_parity.rs` (P2 paridade).
P206D não duplica semantic em structural matrix.

---

## §3 INCLUDE-com-diff (3 divergências documentadas)

Estes ficheiros são **INCLUDE** na matriz P206D mas
têm divergências empíricas observadas em P206C —
documentadas como dados, não regressões.

| Ficheiro | Selector | Divergência | Causa identificada |
|----------|----------|-------------|---------------------|
| `math/{block,simple}.typ` + visuais com equation | `equation` | Vanilla rejeita `equation` standalone ("unknown variable"); cristalino aceita | Vanilla usa `math.equation` namespace; cristalino aceita `equation` via `ElementKind::Equation` (P186B). Divergência arquitectónica de selector parsing. Fix exigiria parsing de namespace vanilla — fora-de-escopo P206. |
| `visual/cite-bibliography.typ` | (todos selectors) | Cristalino eval falha (1 diagnostic) | Bibliography stdlib cristalino é parcial (P181 series não-completa). Vanilla compila ok. Gap conhecido. |
| `visual/outline-toc.typ` | `heading` | Count mismatch cristalino vs vanilla | TOC entries são contadas distintamente: cristalino emite headings auto-toc internos (P200B); vanilla query distingue. Decision design legítima. |

Resolução de cada um:

- `equation` namespace: fix exige expandir parsing de
  selector cristalino para suportar dotted syntax
  (`math.equation`). Sub-passo dedicado pós-P206.
- Bibliography stdlib: tracking via DEBT existente
  (P181 series); fora-de-escopo P206.
- Outline-toc heading count: design intencional per
  P200B (auto-toc emissions visíveis em query). Documentado.

---

## §4 Sumário de cobertura matriz P206D

| Etiqueta | Count | Percentagem |
|----------|------:|-------------|
| INCLUDE (testado em matriz) | 23 | 64% |
| SKIP-pre-existing | 3 | 8% |
| SKIP-feature | 10 | 28% |
| **Total corpus** | **36** | **100%** |

Dos 23 INCLUDE:
- ~20 produzem `Match` empírico em pelo menos 1
  selector (per matriz P206C runtime).
- 3 têm divergências documentadas (`equation` /
  `cite-bibliography` / `outline-toc`).

---

## §5 Convenção de manutenção

Quando ficheiro corpus novo é adicionado (futuras
séries P207+):

- Decidir etiqueta no momento da adição.
- Documentar entrada nesta tabela (§1, §2 ou §3).
- Sentinela `p206d_skips_documentados` (em
  `lab/parity/tests/structural_parity.rs`) verifica
  consistência runtime.

Quando ficheiro corpus muda categoria:

- Update entrada relevante.
- Re-correr matriz consolidada (`cargo test --manifest-path
  lab/parity --test consolidado_p206d`) para regerar
  `reports/latest.md`.

---

## §6 Cross-references

- ADR-0075 §"Plano de validação" cond 3 — manifest
  documentado per `lab/parity/SKIPS.md`.
- P206C diagnóstico §5 — tabela 36 ficheiros original.
- P206C inventário `typst-passo-206C-inventario.md` §5.
- ADR-0054 — divergência geométrica `FixedMetrics` vs
  vanilla (justifica `geometric` N/A).
- DEBT-53 (vanilla integration) — closed-by-P206 quando
  matriz P206D produzida.
- P204F.div-1 — vanilla integration deferred resolvida.
