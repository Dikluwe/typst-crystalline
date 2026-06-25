# L0 — Layout: Referências e Labels
Hash do Código: 3eb508e5

## Módulo
`01_core/src/rules/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Resolve `@nome` (`Content::Ref`) usando
o `Introspector` fornecido ao Layouter. O caminho numérico P462 consulta
`Introspector::counter_key_for_label` + `query_by_label` e formata o número do
counter correspondente. O caminho legacy (`Content::Labelled`) continua a usar
`resolved_labels` / `figure_label_numbers` injectados pela introspecção
(Passagem 1).

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
- Label registada → `counter.label_pages` contém a chave após layout.
- Layout de label num elemento que força quebra de página → página registada
  é a do elemento, não a anterior.
