# L0 — Layout: Tabela de Conteúdos
Hash do Código: 48bf5672

## Módulo
`01_core/src/rules/layout/outline.rs`

## Propósito
Encapsula o braço `Content::Outline`. Lê `headings_for_toc` do
`CounterState` injectado e gera a sequência visual da TOC.

## Regras de negócio
- Não faz introspecção — apenas consome `headings_for_toc` já populado.
- Desenha um heading de nível 1 ("Índice") fora do modo read-only.
- Para cada entrada, lê `counter.label_pages.get(&label)` antes de activar
  `is_readonly` para evitar borrow duplo.
- Activa `counter.is_readonly = true` antes de `layout_content` e restaura
  a `false` depois — bloqueia CounterUpdate/step durante o clone (DEBT-13).
- Número de página: `"  N"` se disponível em `label_pages`; string vazia na
  Passagem 2 (draft). Acrescentado ao fim da linha.
- Não calcula números de página por si — lê-os do `label_pages` injectado.
  DEBT-12 resolvido via orquestração em 3 passagens em L3.

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o "Índice".
- Heading de nível 2 → linha indentada (contém espaços de indentação).
- Ausência de headings → TOC exibe apenas o título "Índice".
- `is_readonly = true` durante layout de cada linha → CounterUpdate no clone
  não avança contadores.
