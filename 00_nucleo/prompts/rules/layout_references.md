# L0 — Layout: Referências e Labels
Hash do Código: bb60ffb8

## Módulo
`01_core/src/rules/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Consulta `resolved_labels`
injectado pela introspecção (Passagem 1). Não escreve em `resolved_labels`
— essa escrita foi removida no Passo 60 e pertence apenas a `introspect.rs`.

## Regras de negócio
- `Labelled { target, label: _ }` → layout transparente do target apenas.
- `Ref { target }` → consulta `self.counter.resolved_labels`; se encontrar,
  desenha o texto resolvido; se não, desenha `@nome` (nunca panic).

## Critérios de verificação
- `Ref` com label existente → texto resolvido no plain_text.
- `Ref` com label inexistente → `@nome` no plain_text, sem panic.
