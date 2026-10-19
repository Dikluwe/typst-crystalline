# Relatório — Passo 457: `outline()` parametrizável

## Resumo

Implementou-se a parametrização da função `outline()` (Table of Contents) de
acordo com os campos settable do vanilla Typst cobertos neste passo:
`title`, `depth` e `indent`. A função deixou de ser um interceptador especial
em `eval/closures.rs` e passou a ser uma função nativa da stdlib
(`native_outline`), registada em `make_stdlib()`.

## Alterações

### 1. `01_core/src/entities/elements/outline.rs`
- `OutlineElem` deixou de ser unit struct e passou a ter três campos públicos:
  `title: Option<Content>`, `depth: usize`, `indent: bool`.
- Implementações de `map_content` e `map_text` preservam `depth`/`indent` e
  aplicam a transformação recursivamente sobre `title`.
- A struct deriva `PartialEq` e `Hash` (não `Eq`, porque `Content` não é `Eq`).

### 2. `01_core/src/entities/content.rs`
- `Content::outline()` mantém o comportamento default
  (`outline_with(None, 3, true)`).
- Adicionado `Content::outline_with(title, depth, indent)` para construção
  programática do elemento parametrizado.
- Atualizado o braço `PartialEq` de `Content::Outline` para delegar à comparação
  estrutural do `Arc<OutlineElem>`.

### 3. `01_core/src/engine/stdlib/structural.rs`
- Implementado `native_outline(ctx, args, world, current_file)`.
- Aceita título como primeiro argumento posicional **ou** named `title`.
- Aceita `depth: int` (default `3`; erro se `< 1`) e `indent: bool` (default
  `true`).
- Rejeita tipos inválidos e o uso simultâneo de título posicional e named.

### 4. `01_core/src/engine/eval/mod.rs`
- Registada `native_outline` no `make_stdlib()` com o nome `"outline"`.

### 5. `01_core/src/engine/eval/closures.rs`
- Removida a interceptação especial de `outline()` (Passo 61). Agora o call
  segue o caminho normal de função nativa da stdlib.

### 6. `01_core/src/engine/layout/outline.rs`
- `layout_outline` recebe `&OutlineElem` e respeita:
  - `title` customizado (fallback para `"Índice"`);
  - `depth` (ignora entradas com `level > depth`);
  - `indent` (prefixa com `"  ".repeat(level - 1)` quando ativo).

### 7. `01_core/src/engine/layout/tests.rs`
- Adicionados 5 novos testes de layout:
  - `layout_outline_title_custom`
  - `layout_outline_depth_limita_niveis`
  - `layout_outline_indent_false_nao_indenta`
  - `layout_outline_parametros_default_igual_a_vanilla`

### 8. `01_core/src/engine/eval/tests.rs`
- Adicionados 2 testes end-to-end de source Typst:
  - `p457_outline_source_parametros_named`
  - `p457_outline_source_positional_title`

### 9. Prompts L0
- `00_nucleo/prompts/entities/elements/outline.md`: reflete a struct com
  campos e o fato de `map_content`/`map_text` serem recursivos sobre `title`.
- `00_nucleo/prompts/engine/layout_outline.md`: documenta `title`, `depth` e
  `indent` e os critérios de verificação correspondentes.
- `00_nucleo/prompts/engine/stdlib/structural.md`: adicionada secção
  `native_outline(title?, depth:?, indent:?)` e exemplos canónicos.
- `00_nucleo/prompts/entities/content.md`: `Outline` passou da lista de
  terminais para a lista de containers (devido ao `title` recursivo).
- Atualizados os `@prompt-hash` nos ficheiros de código para eliminar drift
  reportado pelo `crystalline-lint`.

## Verificação

```bash
RUST_MIN_STACK=16777216 cargo test --workspace
# Resultado: todos os crates passaram (3228 + 491 + 24 + 2 + 21 + 2 + doc-tests).

crystalline-lint .
# Resultado: sem erros de drift relacionados a P457; restam apenas warnings
# pré-existentes de prompts órfãos não referenciados.
```

## Notas

- A contagem de ocorrências nos testes de `depth` é necessária porque
  `PagedDocument::plain_text()` concatena o texto de toda a página, incluindo
  os headings reais do documento. Assim, um heading excluído da TOC ainda
  aparece uma vez como conteúdo.
- O layout do título da TOC continua a usar `Content::heading(1, ...)` fora do
  modo read-only; esse heading é criado em tempo de layout e não é indexado por
  `headings_for_toc`.
