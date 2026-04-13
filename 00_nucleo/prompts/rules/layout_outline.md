# L0 — Layout: Tabela de Conteúdos
Hash do Código: fd2dc66f

## Módulo
`01_core/src/rules/layout/outline.rs`

## Propósito
Encapsula o braço `Content::Outline`. Lê `headings_for_toc` do
`CounterState` injectado e gera a sequência visual da TOC.

## Regras de negócio
- Não faz introspecção — apenas consome `headings_for_toc` já populado.
- Gera um `Content::Sequence` com um heading de nível 1 ("Índice") e
  uma linha por título, indentada pelo nível.
- Cada linha usa `Content::Ref` apontando para a label automática gerada
  pela introspecção — o texto resolvido já estará em `resolved_labels`.
- Não calcula números de página (DEBT-12).

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o "Índice".
- Heading de nível 2 → linha indentada (contém espaços de indentação).
- Ausência de headings → TOC exibe apenas o título "Índice".
