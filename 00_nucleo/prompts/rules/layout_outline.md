# L0 — Layout: Tabela de Conteúdos
Hash do Código: P457

## Módulo
`01_core/src/rules/layout/outline.rs`

## Propósito
Encapsula o braço `Content::Outline`. Lê `headings_for_toc` do
`Introspector` injectado e gera a sequência visual da TOC respeitando os
parâmetros do `OutlineElem` (`title`, `depth`, `indent`).

## Regras de negócio
- Não faz introspecção — apenas consome `headings_for_toc` já populado.
- Título da TOC:
  - `e.title.clone()` se presente;
  - caso contrário, heading de nível 1 com corpo `"Índice"`.
- Profundidade (P457): ignora entradas com `level > e.depth`.
- Indentação (P457): se `e.indent` for `true`, prefixa a linha com
  `"  ".repeat(level.saturating_sub(1))`; se `false`, não prefixa.
- Para cada entrada, lê `runtime.known_page_numbers.get(&label)` antes de activar
  `is_readonly` para evitar borrow duplo.
- Activa `runtime.is_readonly = true` antes de `layout_content` e restaura
  `false` depois — bloqueia CounterUpdate/step durante o clone (DEBT-13).
- Número de página: `"  N"` se disponível em `known_page_numbers`; string vazia
  na Passagem 2 (draft). Acrescentado ao fim da linha.
- Não calcula números de página por si — lê-os do estado injectado.
- **Número da entrada (P359, DEBT-60 b, paridade vanilla).** A entrada usa o
  campo `number` devolvido por `headings_for_toc` (ex. `"1."`, `"1.1."`), **sem**
  o supplement `"Secção"`. Headings não-numerados → `number` é `None` e a linha
  começa directamente com o título.

## Critérios de verificação
- Documento com 3 headings → TOC tem 3 linhas após o título.
- `outline(depth: 1)` lista apenas headings de nível 1.
- `outline(indent: false)` não indenta entradas de nível 2 (corpo ainda
  presente; plain_text não preserva posição).
- `outline(title: [Sumário])` renderiza `"Sumário"` em vez de `"Índice"`.
- Heading de nível 2 com `indent: true` → linha indentada.
- Ausência de headings → TOC exibe apenas o título.
- `is_readonly = true` durante layout de cada linha → CounterUpdate no clone
  não avança contadores.
