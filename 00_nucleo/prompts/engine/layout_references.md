# L0 — Layout: Referências e Labels
Hash do Código: 3eb508e5

## Módulo
`01_core/src/engine/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Resolve `@nome` (`Content::Ref`) usando
o `Introspector` fornecido ao Layouter. O caminho numérico P462 consulta
`Introspector::counter_key_for_label` + `query_by_label` e formata o número do
counter correspondente. O caminho legacy (`Content::Labelled`) continua a usar
`resolved_labels` / `figure_label_numbers` injectados pela introspecção
(Passagem 1). P463 envolve todo `Ref` num `FrameItem::Link` com destino
interno (`LinkTarget::Destination`), tornando a referência clicável no PDF.

## Regras de negócio
- `Labelled { target, label }` → layout transparente do target, depois registo
  da página em `counter.label_pages`. O registo ocorre **depois** do layout
  porque o target pode forçar uma quebra de página.
- `Ref { target }`:
  1. Caminho P462 (`Content::Label`): se `counter_key_for_label` retornar uma
     chave, formata o número na `Location` da label usando o counter
     correspondente e prefixa `supplement` (explícito ou default do tipo).
  2. Fallback legacy: `figure_number_for_label(&label)` → `"Fig. {n}"`, ou
     `resolved_label_for(&label)` → texto resolvido (ex: `"Secção 1"`).
  3. Label não encontrada → renderizar `"?"`.
- `current_page_number()` no Layouter: `self.pages.len() + 1` (Abordagem A).

## Critérios de verificação
- `Ref` a heading numerado via `Content::Label` → número (ex: `"1"`) no
  plain_text.
- `Ref` a figure numerada via `Content::Label` → `"Fig. 1"` no plain_text.
- `Ref` a equation numerada → `"(1)"`.
- `Ref` a table numerada → `"Table 1"`.
- Supplement explícito sobrepõe default.
- Label inexistente → `"?"`, sem panic.
- `Ref` produz `FrameItem::Link { target: LinkTarget::Destination(label), .. }`.
- `Content::Link` continua a produzir `LinkTarget::Url` (sem regressão).
- Label registada → `counter.label_pages` contém a chave após layout.
- Layout de label num elemento que força quebra de página → página registada
  é a do elemento, não a anterior.

---

## §P788 — Validação de refs (erros do vanilla) + precedência numérica + suplemento por língua

**Decisão (mensagens medidas no vanilla 0.15.0 por execução, 2026-07-20):**

1. **Label inexistente** → erro fatal de layout (via `layout_errors`):
   `` label `<{name}>` does not exist in the document `` — antes renderizava
   "?" em silêncio (P786 A10). Existência = qualquer caminho conhece o label
   (counter key / figure number / resolved legacy / query_by_label).
2. **Ref a heading sem `numbering`** → erro:
   `cannot reference heading without numbering` + hint
   `` you can enable heading numbering with `#set heading(numbering: "1.")` ``
   — antes renderizava vazio em silêncio (P786 A8). A flag por Location vem
   do introspector (`heading_numbering`, ver introspector.md/introspect.md §P788).
3. **Precedência numérica sobre o legacy `Labelled`:** os dois
   `label_to_counter_key.remove()` (walk `Content::Label` + populate arm
   `Labelled`) são **removidos** — refs a heading/equation/table/figure
   numerados usam o caminho do counter (formatado), ficando o
   `resolved_labels` legacy como fallback de última linha.
4. **Suplemento default de heading por língua** (vanilla: "Section 1" em
   docs `en` — medido; "Secção" hardcoded antes divergia): `en` →
   `Section `, `pt` → `Secção `, outras línguas → fallback `en`
   (limitação registada). Língua lida de `layouter.style.lang`.
5. **Spans:** `RefElem` não carrega span — os erros seguem com
   `Span::detached()` (limitação registada; vanilla aponta o `@ref`).
