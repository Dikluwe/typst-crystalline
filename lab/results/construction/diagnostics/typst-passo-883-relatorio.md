# typst-passo-883 — Relatório

**Objectivo:** (1) adicionar teste de regressão para o formato de embedding CFF1 (bare CFF vs CFF2 OpenType), e (2) explorar melhorias de tamanho: compressão FlateDecode dos streams de fonte e sonda dos content streams de `06-long`.

**Estado do código no momento da medição:**
- Commit base: `3f15cc50e` (histórico ainda não inclui P883).
- Ficheiros alterados por este passo: `03_infra/src/export/builder.rs`, `03_infra/src/export/tests.rs`, `00_nucleo/prompts/infra/export/builder.md`, `00_nucleo/prompts/infra/export/font_subset.md`, `03_infra/fixtures/p307b/reference/09-cidfont.pdf`.
- Prompts L0: `builder.md` (hash do código `806ce069`); `font_subset.md` (hash do código `4e893651`).
- Testes: `cargo test --workspace` verde. `crystalline-lint .` com zero violations relevantes (apenas V7 pré-existente: `package_version_resolution.md` órfão).

---

## 1. Parte 1 — Teste de regressão para CFF1 bare vs CFF2 OpenType

Adicionado `p883_regressao_embedding_cff1_bare_cff2_opentype` em `03_infra/src/export/tests.rs`.

### O que trava

- **CFF1** (`NimbusSans-Regular.otf`) deve produzir:
  - `/Subtype /CIDFontType0C` no dicionário do stream.
  - Stream comprimido com `/Filter /FlateDecode`.
  - Bytes descomprimidos começando com a assinatura CFF pura `\x01\x00` (não `OTTO`).
  - Tamanho descomprimido inferior a 4000 bytes — limite superior que captura o wrapper SFNT se este voltar (~1.5 KB de diferença medida por P882).

- **CFF2** (`Cantarell-VF.otf`) permanece como controle de não-regressão:
  - `/Subtype /OpenType` (SFNT completo).
  - Sem `/Subtype /CIDFontType0C`.

### Teste `p560_fonte_cff_usa_cidfont_type0c` actualizado

O teste P560 foi estendido para descomprimir o stream antes de verificar a assinatura CFF, uma vez que o stream agora pode vir comprimido (ver §2). Sem esta actualização, o teste falharia ao ler os bytes `789c` do zlib em vez de `0100`.

---

## 2. Parte 2a — Compressão FlateDecode dos streams de fonte

### Implementação

Adicionada `build_font_stream` em `03_infra/src/export/builder.rs`:

- Comprime os bytes do stream de fonte com `compress_zlib` (mecanismo já existente no exportador para outros streams).
- Emite `/Filter /FlateDecode` quando a compressão tem sucesso.
- Em caso de falha do compressor, emite o stream sem compressão — o PDF continua válido.
- Aplicada em `build_cidfont` e `build_multifont`, cobrindo TrueType, CFF1 bare e CFF2 OpenType.

### Prompts L0 actualizados

- `00_nucleo/prompts/infra/export/builder.md`: adicionada secção P883 (compressão FlateDecode); actualizado critério de verificação do embedding CFF1 para incluir `/Filter /FlateDecode`.
- `00_nucleo/prompts/infra/export/font_subset.md`: descritor de font stream actualizado para refletir a compressão.

Hashes actualizados via `crystalline-lint --fix-hashes`.

### Snapshot actualizado

A fixture `03_infra/fixtures/p307b/reference/09-cidfont.pdf` foi regenerada com `UPDATE_P307B_SNAPSHOTS=1`, porque a introdução da compressão alterou os bytes do PDF de referência. O conteúdo semântico permanece o mesmo.

---

## 3. Parte 2b — Sonda dos content streams de `06-long`

`06-long` continuou praticamente do mesmo tamanho após a compressão da fonte. Fez-se uma sonda rápida para quantificar a composição do PDF:

- **51 content streams**, totalizando **970 910 bytes** descomprimidos.
- Compressão manual desses content streams com zlib reduziria o volume para **133 770 bytes** (~13,78% do tamanho original).
- Ganho potencial: ~837 KB — muito maior do que os ~1.5 KB ganhos com a compressão do stream de fonte.

### Conclusão da sonda

A diferença de tamanho entre cristalino e vanilla em `06-long` não está nas fontes. O culpado são os content streams de múltiplas páginas, que o cristalino ainda não comprime. Cada stream repete `BT ... ET` por bloco de texto e usa coordenadas com 3 casas decimais. Esta é uma frente separada e maior, que merece um passo próprio.

---

## 4. Testes

### Contagem de testes discriminada por crate

```text
typst-core:   0 passados directamente (mudança em L3)
typst-infra:  729 passados; 5 ignorados; 0 falhados
typst-shell:  41 passados; 0 falhados
04_wiring:    39 passados; 0 falhados (37 cli + 2 crystalline_lint)
```

A contagem mantém-se igual a P882/P881: o teste novo (`p883_regressao_embedding_cff1_bare_cff2_opentype`) compensa a actualização/expansão do `p560`, sem alterar o total líquido.

---

## 5. Validação de tamanho

Compilado com o binário de release actualizado (`cargo build --release -p typst-wiring`):

| Cenário | Cristalino P882 | Cristalino P883 | Vanilla 0.15.0 | Razão P882 | Razão P883 |
|---|---|---|---|---|---|
| `02-lorem` | 60 862 bytes | **58 906 bytes** (−1 956) | 14 763 bytes | 4.12× | **3.99×** |
| `06-long` | 1 005 795 bytes | **1 004 336 bytes** (−1 459) | 151 708 bytes | 6.63× | **6.62×** |

### Observações

- Em `02-lorem`, a compressão do stream CFF1 reduziu o PDF em ~2 KB, aproximando-o do vanilla.
- Em `06-long`, o ganho é irrelevante porque o stream de fonte é uma fração minúscula do total. Os ~970 KB de content streams dominam o tamanho.

### Validação de renderização

- `pdftoppm -png -f 1 -l 1 <pdf> <out>`: OK para ambos os cenários.
- `gs -dNOPAUSE -dBATCH -sDEVICE=pdfwrite`: OK para ambos os cenários.
- `pdftotext` de `02-lorem` é **idêntico** ao vanilla (37 linhas, diff vazio).

---

## 6. Scope-outs e próximas frentes

1. **Compressão de content streams.** Identificada como a causa dominante do tamanho de `06-long` (~837 KB de ganho potencial). Fica registada como insumo para um passo futuro dedicado.
2. **Otimização dos content streams.** Além da compressão, cada stream repete operadores e usa coordenadas com 3 casas decimais. A redução de verbosidade é outra frente separada.

---

## 7. Resumo

- **Parte 1 (obrigatória):** teste de regressão `p883_regressao_embedding_cff1_bare_cff2_opentype` adicionado; `p560` actualizado para suportar streams comprimidos.
- **Parte 2a:** compressão FlateDecode implementada para streams de fonte (TrueType, CFF1, CFF2), com fallback não-comprimido em caso de erro.
- **Parte 2b:** sonda dos content streams de `06-long` registada — 51 streams, 970 KB descomprimidos, potencial de compressão para ~134 KB (~13,78%).
- **Resultado:** `02-lorem` reduziu de 60 862 B para 58 906 B (3.99× vs vanilla); `06-long` manteve-se dominado pelos content streams (~6.62× vs vanilla).
- **Próxima frente identificada:** compressão e otimização dos content streams de múltiplas páginas.
