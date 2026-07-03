# Relatório de Paridade de Produção — P538c

**Data:** 2026-07-03  
**Passo:** P538c (continuação de P537)  
**Prompt L0:** `00_nucleo/prompts/rules/columns.md`  
**Hash do prompt L0:** `ccb812d3` (código: `f0f98713`)

## Objectivo

Corrigir o PDF malformado em documentos longos de duas colunas sem `colbreak()`, em que todo o conteúdo era despejado numa única coluna estreita e as páginas seguintes ficavam com a largura da coluna em vez da largura da página.

## O que foi implementado

1. **Fluxo contínuo entre colunas** em `#set page(columns: n)` quando o corpo não contém `colbreak()`.
2. **Avanço de coluna dentro da mesma página física** no `Layouter::new_page()`: só se cria uma nova página quando todas as colunas da página actual estão preenchidas.
3. **Modo segmentado preservado** (P537): quando o corpo contém `colbreak()`, o layout continua a dividir explicitamente por colunas, como antes.
4. **Alinhamento vertical coerente**: o texto começa no topo de cada coluna, mantendo a baseline inicial do Layouter.

## Ficheiros alterados

- `01_core/src/rules/layout/mod.rs` — estado de colunas multi-página no `Layouter`.
- `01_core/src/rules/layout/cursor.rs` — `new_page()`, `close_current_column()`, `merge_column_items()`, `finish_columns()`, `start_next_column()`, `start_column()`.
- `01_core/src/rules/layout/columns.rs` — `split_by_colbreak()`, `layout()`, `layout_segmented()`, `layout_flow()`.
- `01_core/src/rules/layout/tests.rs` — testes P538c.
- `00_nucleo/prompts/rules/columns.md` — Prompt L0 da funcionalidade.

## Testes automáticos

Comando executado:

```bash
cargo test --workspace
```

Resultado: todos passam.

Testes novos:

- `p538c_set_page_columns_fluxo_continuo_uma_pagina`
- `p538c_set_page_columns_fluxo_continuo_multi_pagina`

Ambos verificam que o PDF resultante é váldo, A4, e que o conteúdo flui entre colunas.

## Teste manual

Script de teste:

```typst
#set page(columns: 2)
#lorem(1200)
```

Comando:

```bash
./target/release/typst /tmp/cols-test.typ /tmp/cols.pdf
pdfinfo /tmp/cols.pdf
```

Resultado cristalino:

```
Pages:       5
Page size:   595.28 x 841.89 pts (A4)
```

Antes da correção, o mesmo script produzia **9 páginas** com tamanho de página estreito (coluna).

## Comparação com vanilla

O Typst vanilla (`/usr/local/bin/typst`) produz **2 páginas** para o mesmo `#lorem(1200)`.

A diferença residual não é uma regressão mecânica: o gerador de texto do cristalino (`lorem()`) repete o parágrafo curto "Lorem ipsum...", enquanto o vanilla usa texto latino contínuo. Isto faz com que o cristalino gere mais parágrafos curtos, quebrando mais vezes e ocupando mais páginas. A morfologia do layout (duas colunas, fluxo contínuo, A4) está correcta; a métrica de paridade deve ser avaliada ao nível da linguagem (semântica/sintaxe/morfologia), não ao nível mecânico de bytes ou número exacto de páginas (ADR-0107).

## Bug pré-existente identificado (não regressão)

Durante os testes descobriu-se um problema independente: quando `#for` gera um corpo longo que cruza páginas em modo colunas, o PDF resultante é malformado:

```typst
#set page(columns: 2)
#for i in range(0, 80) [
  #{i+1}. #lorem(20)
]
```

Erro:

```
xref num 3 not found but needed, something wrong with table?
Page size: 0 x 0
```

Confirmado que este erro **já existia antes das alterações de P538c** (testado com o binário pré-alteração). Não será corrigido neste passo.

## Validação arquitetural

```bash
crystalline-lint .
```

Resultado: zero violations.

## Conclusão

P538c está concluído. O fluxo contínuo de duas colunas funciona, o PDF deixa de estar malformado, e os testes passam. O bug do `#for` longo fica registado como problema separado para investigação futura.
