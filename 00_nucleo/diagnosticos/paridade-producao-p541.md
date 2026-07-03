# Relatório de Paridade de Produção — P541

**Data:** 2026-07-03  
**Passo:** 541  
**Prompt L0:** `00_nucleo/prompts/rules/layout.md` (hash `9c9b7122`)  
**Dependências:** P532 (numeração simples de página), P538d (`style.font` no texto de numeração)

## Objectivo

Suportar padrões compostos de numeração de página (ex.: `"1 / 1"`, `"I-1"`),
onde o total de páginas só é conhecido depois do layout completo.

## Sonda

1. **`format_counter`** (`01_core/src/entities/counter_format.rs`): já trata
   texto literal misturado com tokens; recebe um slice de inteiros e consome
   um valor por token de numeração.
2. **Total de páginas**: só está disponível no final de `Layouter::finish()`,
   após todas as páginas estarem no `Vec<Page>`.
3. **Mecânica de aplicação**: P532 aplicava a numeração página a página à
   medida que cada página era fechada. Para padrões compostos é necessária
   uma segunda passagem após o total ser conhecido.

Conclusão da sonda: reaproveitar `format_counter` com `[page_number,
total_pages]` e adiar a renderização dos padrões com ≥2 tokens de numeração
para o final de `finish()`.

## Implementação

`01_core/src/entities/counter_format.rs`:

- `is_numbering_token(ch: char) -> bool` — reconhece `'1' | 'I' | 'i' | 'a' | 'A'`.
- `count_numbering_tokens(pattern: &str) -> usize` — conta tokens num pattern.

`01_core/src/rules/layout/cursor.rs` (`new_page`):

- Padrões simples (1 token): comportamento anterior — desenhar imediatamente.
- Padrões compostos (≥2 tokens): guardar entrada em
  `self.pending_page_numbering.push((page_index, page_number, pattern))`
  em vez de renderizar.

`01_core/src/rules/layout/mod.rs` (`finish`):

- Aplica a mesma regra de adiamento na última página.
- Após todas as páginas estarem no `Vec`, itera `pending_page_numbering` e
  chama `format_counter(&[page_number, total_pages], pattern)`, inserindo o
  `FrameItem::Text` no rodapé de cada página.
- O estilo do texto usa `TextStyle::from(&self.chain)` com `size = font_size_pt`,
  herdando a correcção de P538d.

## Testes

### Teste unitário

`01_core/src/rules/layout/tests.rs` —
`p541_page_numbering_composto_usa_total_de_paginas`:

```typst
#set page(numbering: "1 / 1")
Página um.
#pagebreak()
Página dois.
#pagebreak()
Página três.
```

Verifica que cada página contém `"1 / 3"`, `"2 / 3"`, `"3 / 3"`.

### Teste manual

```bash
cat > /tmp/num-composto.typ <<'EOF'
#set page(numbering: "1 / 1")
Página um.
#pagebreak()
Página dois.
#pagebreak()
Página três.
EOF
./target/release/typst /tmp/num-composto.typ /tmp/nc.pdf
pdftotext /tmp/nc.pdf -
```

Cristalino:

```text
Página um.
1/3
Página dois.
2/3
Página três.
3/3
```

Vanilla (`typst compile`):

```text
Página um.
1/3
Página dois.
2/3
Página três.
3/3
```

O `pdftotext` colapsa os espaços em redor de `/`, mas o output é idêntico
entre cristalino e vanilla.

### Padrão `"I-1"`

Cristalino (`pdftotext`):

```text
I-3
II - 3
III - 3
```

Os numerais romanos usam o número da página; o arábico final usa o total.

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Resultado:

- `cargo test --workspace`: 570 passaram; 1 falha intermitente
  (`p307b_snapshot_tests::p307b_snapshot::p307b_07_multi_feature`). A falha
  reproduz-se sem as alterações de P541 (verificado via `git stash`), pelo que
  é uma flakiness preexistente relacionada com cache de fontes/shaper, não
  causada por este passo.
- `crystalline-lint .`: zero violations.

## Ficheiros alterados

- `01_core/src/entities/counter_format.rs` — `is_numbering_token` e
  `count_numbering_tokens`.
- `01_core/src/rules/layout/cursor.rs` — adiamento de numeração composta em
  `new_page`.
- `01_core/src/rules/layout/mod.rs` — adiamento na última página e aplicação
  final em `finish`.
- `01_core/src/rules/layout/tests.rs` — teste
  `p541_page_numbering_composto_usa_total_de_paginas`.
- `00_nucleo/diagnosticos/paridade-producao-p541.md` — este relatório.

## Conclusão

P541 está concluído. Padrões compostos de numeração de página funcionam no
cristalino, usando o total de páginas correcto. Padrões simples de P532
permanecem sem regressão.
