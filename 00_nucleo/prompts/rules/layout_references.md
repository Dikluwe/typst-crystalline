# L0 — Layout: Referências e Labels
Hash do Código: 3eb508e5

## Módulo
`01_core/src/rules/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Consulta `resolved_labels`
injectado pela introspecção (Passagem 1). Não escreve em `resolved_labels`
— essa escrita foi removida no Passo 60 e pertence apenas a `introspect.rs`.

## Regras de negócio
- `Labelled { target, label }` → layout transparente do target, depois registo
  da página em `counter.label_pages`. O registo ocorre **depois** do layout
  porque o target pode forçar uma quebra de página.
- `Ref { target }` → consulta `self.counter.resolved_labels`; se encontrar,
  desenha o texto resolvido; se não, desenha `@nome` (nunca panic).
- `current_page_number()` no Layouter: `self.pages.len() + 1` (Abordagem A).

## Critérios de verificação
- `Ref` com label existente → texto resolvido no plain_text.
- `Ref` com label inexistente → `@nome` no plain_text, sem panic.
- Label registada → `counter.label_pages` contém a chave após layout.
- Layout de label num elemento que força quebra de página → página registada
  é a do elemento, não a anterior.
