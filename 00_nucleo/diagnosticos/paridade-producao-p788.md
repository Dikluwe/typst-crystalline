# P788 — Referências e citações inválidas devem errar, não falhar em silêncio

> **Passo:** 788
> **Data:** 2026-07-20, medições entre ~18:55Z e ~20:15Z
> **Commit-base:** `0774275fe1b340d823e624958f66e5b6b344a51a` + working tree não commitado (P786a+P787+P788)
> **Ficheiros alterados neste passo (`git diff HEAD --stat`, subconjunto P788):** `03_infra/src/export/stream.rs` (+71), `03_infra/src/export/tests.rs`, `03_infra/src/integration_tests.rs` (+67), `03_infra/src/measurements.rs`, `01_core/src/engine/layout/references.rs`, `01_core/src/engine/layout/tests.rs`, `01_core/src/engine/eval/mod.rs`, `01_core/src/engine/introspect.rs`, `01_core/src/engine/introspect/{convergence,extract_payload}.rs`, `01_core/src/entities/{element_payload,elements/heading,introspector,element_info,tag}.rs`; L0s: `infra/export/stream.md`, `engine/layout_references.md`, `entities/{element_payload,introspector}.md`, `engine/{introspect,layout}.md`
> **Binários:** vanilla `lab/typst-original/target/release/typst` (0.15.0, rev `969087ec`); cristalino `./target/release/typst` (rebuild pós-correção)
> **ADRs:** ADR-0107 (mensagens de erro são observáveis), ADR-0108 (cada causa confirmada separadamente — foram 4 causas distintas, não uma).

---

## Resumo em uma linha

**As três divergências de `model::reference` corrigidas — e eram 4 causas independentes: (1) flip Y ausente no export PDF de filhos de `FrameItem::Link` ao nível da página (bug genérico de `#link`, não só de refs); (2) remoções deliberadas de `label_to_counter_key` que forçavam o caminho legacy "Secção" e bloqueavam o caminho numérico; (3) ausência de validações (label inexistente → "?" silencioso; heading sem numbering → vazio silencioso); (4) suplemento explícito `@x[sup]` descartado no eval. Validação final: erros idênticos ao vanilla, happy path "Ver Section 1 no texto." = vanilla, posicionamento pixel-idêntico (99.907, 68.271).**

---

## 1. Sonda — contrato do vanilla (execução)

| Caso | Vanilla 0.15.0 |
|---|---|
| `= Sem numeração <x>` + `@x` | exit 1: `cannot reference heading without numbering` + hint `` you can enable heading numbering with `#set heading(numbering: "1.")` `` |
| `Isto cita @naoexiste1984.` | exit 1: `` label `<naoexiste1984>` does not exist in the document `` |
| `#set heading(numbering: "1.")` + `Ver @sec1 no texto.` | `Ver Section 1 no texto.` (doc en) |
| `@sec1[Cap]` | `Cap 1` (join suplemento↔número com NBSP U+A0, `realize_reference`) |

**Nota de armadilha:** o documento de posicionamento do próprio passo (`= Título` **antes** do `#set heading(numbering:)`) é inválido para o caso feliz — o heading fica sem numbering (o vanilla erra). Corrigido para `#set` antes do heading nos testes.

## 2. Causas identificadas (leitura de código + probes instrumentados)

1. **Posicionamento (genérico de Link):** `build_page_stream` (03_infra/export/stream.rs) desenhava filhos de `FrameItem::Link` por `draw_item_local` — que **não aplica flip Y** (assume matriz invertida de `Group`). Sem Group, `pos.y` crua → fundo da página (medido: yMin 753.95 vs vanilla 68.27; x correcto). Layouter L1 sempre esteve correcto (probe: 78.57). Fix: extração de `draw_item_top` (com flip) + recursão no braço Link.
2. **"Secção" hardcoded:** `intr.label_to_counter_key.remove(label)` em 2 sítios (`introspect.rs` walk `Content::Label` + populate arm `Labelled`, decisão P462/P464) forçava o legacy `resolved_labels` ("Secção 1") mesmo para headings numerados. Probe mediu: `counter_key: None`, `resolved: Some("Secção 1")`, `formatted: Some("1")`. Removidos — precedência numérica restaurada.
3. **Validações ausentes:** `resolve_ref_text` caía em `"?"` (missing) / `""` (sem numbering). Adicionadas as duas validações em `layout_ref` → `layout_errors` (canal existente, drenado pelo pipeline como erro fatal).
4. **Suplemento explícito:** `Expr::Ref` lia só `target()`; `supplement()` (AST) e `reference_with_supplement` (Content) já existiam — ligados; join com NBSP (regra vanilla medida no código).

**Bónus:** `ElementPayload::Heading` ganhou `numbering_active` (baked da chain na emissão) — o counter de heading aplica-se incondicionalmente (P335), logo "tem counter" ≠ "tem numbering"; a flag alimenta o erro noheading. `TagIntrospector.heading_numbering` (loc → bool) + `Introspector::heading_has_numbering` (+ `CountingIntrospector` delega).

## 3. Validação final (release)

```text
noheading.typ  exit 1  error: cannot reference heading without numbering
                        hint: you can enable heading numbering with `#set heading(numbering: "1.")`  ✓ = vanilla
badlabel.typ   exit 1  error: label `<naoexiste1984>` does not exist in the document                ✓ = vanilla
reftest2.typ   exit 0  "Ver Section 1 no texto."                                                    ✓ = vanilla
rt3.typ        exit 0  "Ver Cap 1 no texto."                                                        ✓ = vanilla
linktest.typ   exit 0  "Clique" @ (99.907, 68.271)                                                  ✓ = vanilla (era 753.95)
```

- `cargo test --workspace`: **verde** (exit 0) — 4 testes P788 novos (integração) + 1 teste P788 de export (flip) + legacy actualizado.
- `crystalline-lint .`: **zero violações**.

## 4. Testes legacy — 11 investigados um a um

Todos na zona de refs; todos com fontes/expectativas que o vanilla rejeita ou que codificavam o legacy errado:

- **Headings sem numbering em testes de resolução** (5): docs actualizados com `Styled` + `heading.numbering` (validado: a chain alimenta o bake) → expectativa `Section 1` (en).
- **Fallback "?"** (3): label inexistente → asserção passa a ser o **erro** `does not exist in the document` (incl. `pdf_ref_unknown_label_emite_goto` — sem `/GoTo`).
- **`c4_paridade_pipelines`**: reescrito como teste de precedência — numérico ganha quando existe; legacy `resolved_labels` é fallback quando o mapa numérico falta.
- **NBSP** (2): asserções normalizam `\u{a0}`→espaço (join vanilla `realize_reference`).
- **`labelled_paridade` / `pipeline_duas`**: com wrapper `Styled`, `compute_labelled` não dispara (target não é `Heading` directo) — asserções de store ajustadas; população do store legacy continua coberta por `labelled_walk_emite_tag_e_popula_introspector`.

## 5. Registos para passos futuros

1. **Spans `<detached>`** nos dois erros novos: `RefElem` não carrega span (vanilla aponta o `@ref`). Adicionar span ao `RefElem` é pré-requisito — agrupa com T6/loading spans.
2. **Suplemento por língua além de en/pt**: fallback `en` registado (tabela completa de termos vanilla é maior).
3. **`labelled_path_not_resolved_numerically`** (teste pré-existente, não tocado): continua a documentar que Labelled não-numérico usa legacy — coerente com a precedência nova.
4. **Equation/table refs**: não tocados neste passo (scope: headings + labels + posicionamento); `equation` já formatava `(1)`, `table` `Table 1`.
