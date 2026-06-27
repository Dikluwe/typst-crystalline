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

## §3 INCLUDE-com-diff (divergências documentadas)

Estes ficheiros são **INCLUDE** na matriz mas têm
divergências empíricas observadas — documentadas como
dados, não regressões.

### Estado P479 (2026-06-27)

| Ficheiro | Selector | Divergência | Causa identificada | Estado |
|----------|----------|-------------|---------------------|--------|
| `math/{block,simple}.typ` + visuais com equation | `equation` | Vanilla rejeita `equation` standalone ("unknown variable"); cristalino aceita | Vanilla usa `math.equation` namespace; cristalino aceita `equation` via `ElementKind::Equation` (P186B). Divergência arquitectónica de selector parsing. Fix exigiria parsing de namespace vanilla — fora-de-escopo P206. | INCLUDE-com-diff (error, não diff) |
| `visual/cite-bibliography.typ` | `heading` | ~~Cristalino eval falha (1 diagnostic)~~ → **✓ MATCH (P479)** | P479: `native_bibliography` agora define título padrão `Content::heading(1, "Bibliography")`; walk arm `Content::Bibliography` recursivo em `e.title` conta o heading → cristalino=1 = vanilla=1. | **RESOLVIDO P479** |
| `visual/outline-toc.typ` | `heading` | cristalino=5, vanilla=6 (sub-contagem por 1) | Vanilla conta o heading do título do `#outline()` (criado em layout). Cristalino usa introspector pré-layout que não vê headings criados durante o layout. Raiz: `walk` arm `Content::Outline` vazio (P189B — não recursivo em title); outline layout cria heading `Content::heading(1, title_content)` durante layout. Fix M-size: requer mudança em walk/layout/native_outline — scope-out P479. | INCLUDE-com-diff (1 diff) |

Resolução de cada um:

- `equation` namespace: fix exige expandir parsing de
  selector cristalino para suportar dotted syntax
  (`math.equation`). Sub-passo dedicado pós-P479.
- cite-bibliography heading: **RESOLVIDO P479** (ver tabela acima).
- Outline-toc heading count: raiz identificada P479 (layout-time
  heading não visível a pré-layout query). Fix M-size; scope-out.

---

## §4 Sumário de cobertura matriz P479 (2026-06-27)

| Etiqueta | Count | Percentagem |
|----------|------:|-------------|
| INCLUDE (testado em matriz) | 28 | 61% |
| SKIP-pre-existing | 1 | 2% |
| SKIP-feature | 17 | 37% |
| **Total corpus** | **46** | **100%** |

Dos 28 INCLUDE:
- 50 comparações com match; 1 diff documentado
  (`outline-toc heading`, M-size, raiz identificada).
- 22 errors (todos `equation` selector namespace — vanilla
  rejeita selector standalone; pré-existente).

### Histórico de cobertura

| Passo | Corpus | INCLUDE | Matches | Diffs |
|-------|-------:|--------:|--------:|------:|
| P150  | 25 | N/A | N/A | N/A |
| P206D | 36 | 23 | ~20 | 3 |
| P479  | 46 | 28 | 50 | 1 |

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
