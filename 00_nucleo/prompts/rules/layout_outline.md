# L0 — Layout: Tabela de Conteúdos
Hash do Código: ef0abff8

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
- **Número da entrada (P359, DEBT-60 b, paridade vanilla).** A entrada usa
  `Content::reference(auto-toc-label)`, cujo `resolved_text` é o **número** do
  heading (`{n}.`, ex. "1.1."), **sem** o supplement "Secção" — definido em
  `compute_heading_auto_toc` (`rules/introspect.rs`, P359). Headings não-numerados →
  `resolved_text` vazio → entrada sem número (só o título). Espelha o vanilla, cujo
  `prefix` de outline formata o numbering e não acrescenta supplement para heading
  (`model/outline.rs:123-124`). Coberto por `layout_outline_mostra_numero_sem_supplement_seccao`.

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o "Índice".
- Heading de nível 2 → linha indentada (contém espaços de indentação).
- Ausência de headings → TOC exibe apenas o título "Índice".
- `is_readonly = true` durante layout de cada linha → CounterUpdate no clone
  não avança contadores.
