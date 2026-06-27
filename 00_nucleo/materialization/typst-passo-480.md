---

# P480 — Outline-toc título heading + `math.equation` alias + audit final

> **Passo:** 480
> **Data:** 2026-06-27
> **Foco:** (A) Fechar diff outline-toc: heading de título "Índice"/"Contents" gerado por `native_outline` como `Content::Heading` pré-walk (em vez de gerado em layout); (B) `math.equation` alias no scope; (C) Audit final de estado do projecto.
> **Tipo:** Materialização S + XS + Documental.
> **Tamanho:** M total (~35 min).
> **ADR-0117 Cláusula 4:** `native_outline` em `rules/stdlib/structural.rs`; `layout/outline.rs:57` (heading gerado durante layout); `OutlineElem` em `entities/elements/outline.rs` (struct unit). Não propor campo novo sem verificar — a abordagem escolhida evita mudar `OutlineElem`.

---

## Contexto

P479 identificou dois diffs residuais cristalino vs vanilla:

1. **`outline-toc`** — vanilla conta 6 headings (5 do documento + 1 do título "Contents" do outline); cristalino conta 5. Causa raiz: `layout/outline.rs:57` cria o heading de título durante o layout, invisível ao walk de introspecção. Fix cirúrgico: `native_outline` emite `Content::Heading(1, "Índice")` + `Content::Outline` em vez de apenas `Content::Outline`; o layout de `Content::Outline` deixa de emitir o heading extra.

2. **`equation` selector namespace** — vanilla rejeita `equation` standalone e exige `math.equation`. Fix XS: registar módulo `math` no scope com `equation` como entrada.

Sub-item C é um audit de estado final para fechar formalmente o roteiro das trilhas e registar o estado de paridade.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Verificar em | Status |
|----------|-------------|--------|
| `layout/outline.rs:57` emite `Content::Heading`? | `rules/layout/outline.rs` | ✅ (confirmado P479) |
| `native_outline` produz apenas `Content::Outline`? | `rules/stdlib/structural.rs` | 🟡 sonda |
| `OutlineElem` é struct unit? | `entities/elements/outline.rs` | ✅ (confirmado knowledge) |
| Walk arm `Content::Outline` recursivo em children? | `rules/introspect.rs` | 🟡 sonda |
| `math` módulo existe no scope? | `rules/eval/mod.rs` | 🟡 sonda |
| `"equation"` função registada no scope? | idem | 🟡 sonda |

---

## Sub-item A — Outline título como `Content::Heading` pré-walk

### A.1 — Decisão de abordagem

O diff ocorre porque o heading de título do outline é criado em `layout/outline.rs:57` durante o layout — depois da introspecção já ter corrido. A solução mínima que não toca `OutlineElem` (struct unit) é deslocar a criação do heading para `native_outline`:

```rust
// native_outline (structural.rs) — antes:
Content::outline()

// native_outline — depois (P480):
Content::sequence(vec![
    Content::heading(1, Content::text("Índice")),   // heading visível ao walk
    Content::outline(),
])
```

O heading de título passa a existir no `Content` antes do walk, tornando-se visível ao introspector. O layout de `Content::Outline` (`outline.rs:57`) deixa de emitir o heading separado.

### A.2 — Impacto no layout

**Ficheiro:** `rules/layout/outline.rs:57`

Remover a linha que actualmente emite o heading de título:

```rust
// Antes (P479 baseline):
// outline.rs:57 — emite heading "Índice" durante layout
self.layout_content(&Content::heading(1, Content::text("Índice")));

// Depois (P480):
// Linha removida — heading já vem do Content gerado por native_outline
```

O resto do layout de `Content::Outline` permanece inalterado (itera `headings_for_toc`, emite entradas).

### A.3 — Retrocompatibilidade

`Content::outline()` (construtor directo Rust) ainda existe e produz um outline sem heading de título. Quem chama `Content::outline()` directamente nos testes de layout pode precisar de actualizar — a sonda deve verificar os call-sites.

**Alternativa mais segura:** em vez de remover o heading do layout, verificar se `headings_for_toc` já tem um heading antes de emitir. Mais defensivo mas mais complexo. Decisão: remover do layout, já que `native_outline` é o único entry point user-facing; call sites de teste actualizam-se mecanicamente.

### A.4 — Título localizado

Vanilla usa `"Contents"` em EN, `"Índice"` em PT, etc. via `text.lang`. Subset P480: string fixa `"Índice"` (paridade cristalino histórico). Divergência declarada: texto do título outline não é localizado.

**Alternativa:** usar `figure_supplement_for_lang` pattern (P158B/P470) mas para outline. Scope-out P480 — magnifica sem benefício directo para o diff de contagem (que é de count, não de texto).

---

## Sub-item B — `math.equation` alias

### B.1 — Estado actual

Vanilla expõe `equation` via módulo `math`: `typst query math.equation` funciona; `typst query equation` falha. Cristalino regista `equation` no scope de topo (`scope.define("equation", ...)`). O selector `math.equation` falha porque `math` não existe como módulo.

### B.2 — Fix XS

**Ficheiro:** `rules/eval/mod.rs` (ou onde `make_stdlib()` cria o scope)

Após registar `"equation"` no scope, criar módulo `math`:

```rust
// P480 — módulo math com equation alias (paridade vanilla namespace)
let equation_fn = scope.get("equation").cloned().unwrap_or(Value::None);
let mut math_dict = Dict::new();
math_dict.insert(EcoString::from("equation"), equation_fn);
scope.define("math", Value::Dict(math_dict));
```

**Nota:** verificar via sonda se `math` já existe como módulo (poderá ter sido criado para funções matemáticas de equação). Se existir, apenas adicionar `equation` à entrada existente.

### B.3 — Impacto no corpus

O selector `math.equation` nos ficheiros `semantic/` do corpus passará de SKIP-feature para potencialmente INCLUDE. Verificar quais ficheiros usam `math.equation` e se agora compilam.

---

## Sub-item C — Audit final de estado

### C.1 — Estado das ADRs

Documentar (sem código) o estado final das ADRs mais relevantes:

| ADR | Estado | Observação |
|-----|--------|------------|
| ADR-0083 (Color paridade) | IMPLEMENTADO | Operadores cor fechados P476/P477 |
| ADR-0075 (Vanilla integration) | ACEITE | Paridade 50/73 matches pós-P479 |
| ADR-0054 (Graded scope-outs) | EM VIGOR | Divergências declaradas |
| ADR-0017 (Value enum fechado) | IMPLEMENTADO | Aberto P395 |
| ADR-0061 (Layout roadmap) | IMPLEMENTADO | Trilhas 1, 2, 7 completas |

### C.2 — Inventário de divergências de paridade declaradas

Actualizar `lab/parity/SKIPS.md` com estado final pós-P480:

| Divergência | Tipo | Estado |
|-------------|------|--------|
| `equation` vs `math.equation` selector | Arquitectónica | Resolvida P480 (alias) |
| `outline-toc` heading count | Layout | Resolvida P480 |
| Titulo outline não localizado | Divergência declarada | Preservado |
| Footnote numeração por página | Divergência declarada | Preservado |
| Operador `%` em regex vs modulo | Divergência arquitectónica | Preservado |
| Shaping rustybuzz | Épico XL | Trilha 5 declarada |

### C.3 — Roteiro pós-P480

Após P480, o projecto entra em estado de **consolidação e manutenção**:

- Zero DEBTs activos.
- Trilhas 1–4, 7, 8 completas.
- Trilha 5 (shaping) = épico XL dedicado, magnitude estimada 8–12h.
- Trilha 6: 4/5 (LoF/LoT page numbers = scope-out longo prazo).
- Paridade: ≥52/73 matches (estimado pós-P480 com 2 diffs fechados).

---

## Tests

### Sub-item A

- **L2:** `native_outline()` produz `Content::Sequence([Heading(1, "Índice"), Outline])`.
- **L2:** `walk` sobre `Content::sequence([Heading(1, ...), Outline])` conta 1 heading a mais que outline sozinho.
- **L2:** `layout` de `Content::Outline` não emite heading duplicado.
- **L2:** Documento com `#outline()` → `query("heading")` conta N headings do documento + 1 do outline.
- **Regressão:** testes de layout existentes de `Content::Outline` actualizar para reflectir heading emitido por `native_outline`, não pelo layout.
- **Parity:** `p480_outline_toc_heading_count_match` — após fix, `outline-toc.typ` corpus = match.

### Sub-item B

- **L1:** `scope.get("math")` devolve `Value::Dict` com entrada `"equation"`.
- **L2:** `eval("math.equation")` devolve `Value::Func(...)` (mesma que `eval("equation")`).
- **Parity:** `p480_math_equation_alias_corpus_match` — `semantic/` com `math.equation` agora INCLUDE.

### Sub-item C

- Nenhum teste de código. Documentação e actualização de `SKIPS.md`.

---

## Spec L0

### Actualizados

- `rules/stdlib/structural.md` — `native_outline` produz `Sequence([Heading, Outline])` (P480).
- `rules/layout_outline.md` — remove emissão de heading de título (P480).
- `rules/eval/mod.rs.md` (ou stdlib) — módulo `math` com `equation` alias.
- `lab/parity/SKIPS.md` — estado final pós-P480.
- ADR-0075 — anotação cumulativa P480 (paridade final, diffs residuais).

---

## Scope-out explícito

- **Título do outline localizado** (`"Contents"` EN / `"Índice"` PT) — scope-out; string fixa "Índice".
- **`OutlineElem` com campo `title`** — struct unit preservado; título gerado em `native_outline` como `Content::Heading` separado.
- **`math.frac`, `math.vec`, etc.** — outros membros do módulo math vanilla não são adicionados neste passo; apenas `equation`.
- **SKIP-feature `semantic/` corpus totalmente resolvidos** — apenas `math.equation` pode ser recuperado; outros semânticos mantêm SKIP.

---

## Critério de fecho

- [ ] Sondas: `layout/outline.rs:57` localizado; `native_outline` produz `Content::outline()` confirmado; `math` módulo ausente confirmado — todos com `file:line`.
- [ ] `native_outline` produz `Content::sequence([Heading(1, "Índice"), Outline])`.
- [ ] `layout/outline.rs:57` heading removido.
- [ ] Testes de layout existentes de `Outline` actualizados (call-sites que chamavam `Content::outline()` directamente).
- [ ] Módulo `math` com `equation` registado no scope.
- [ ] 6+ testes verdes (sub-item A) + 2 testes verdes (sub-item B).
- [ ] `p480_outline_toc_heading_count_match` verde.
- [ ] `p480_math_equation_alias_corpus_match` verde.
- [ ] `SKIPS.md` actualizado com estado final.
- [ ] ADR-0075 anotação cumulativa P480.
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] Paridade estimada ≥ 52/73 matches pós-P480.

---

## Estado pós-P479 (para referência)

| Indicador | Estado |
|-----------|--------|
| DEBTs activos | 0 |
| Trilhas 1–4, 7, 8 | COMPLETAS |
| Trilha 5 | Épico XL |
| Trilha 6 | 4/5 |
| Paridade | 50/73 matches; 1 diff (outline-toc + equation) |
| **P480** | Fecha 2 diffs residuais + audit final | 🔄 EM PREPARAÇÃO |
