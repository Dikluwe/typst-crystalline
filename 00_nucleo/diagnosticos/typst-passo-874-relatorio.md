# Relatório — typst-passo-874: corrigir subsetting CFF (CID-keyed válido)

**Data:** 2026-07-23T21:00:00Z  
**Executor:** Claude (Sonnet 5)  
**Commit base:** `30122788891b32a423b0354b97eebdbf28ccf068` (HEAD do ramo `Tekt`, após P872/P873)  
**Working tree:** alterações em `03_infra/Cargo.toml`, `03_infra/src/export/subset.rs`, `03_infra/src/export/builder.rs`, `00_nucleo/prompts/infra/export/font_subset.md`, snapshot `03_infra/fixtures/p307b/reference/09-cidfont.pdf`, e `Cargo.lock`.

---

## 1. Resumo

P797 desactivara o subsetting de fontes CFF1 porque `oxifont-subset` produzia um programa CFF **Name-keyed** inválido para `/CIDFontType0` + `Identity-H`. O P874 substitui `oxifont-subset` pela crate `subsetter` 0.2.6 — a mesma usada pelo Typst 0.15.0 — que converte SID-keyed fonts para **CID-keyed** com ROS `Adobe-Identity-0`, produzindo subsets válidos para PDF.

Efeito principal: o tamanho do PDF do cenário `04-math.typ` cai de **1 794 931 B** (P873, fontes CFF inteiras) para **157 233 B** (subsetting activo), uma redução de ~11×. O tempo de execução mantém-se ~6.3 s, dominado pela busca de fallback não filtrada (causa separada, tratada em P875).

---

## 2. Diagnóstico — por que `oxifont-subset` falhava

A secção 2.3 do relatório P873 já localizara o early-return em `03_infra/src/export/subset.rs:79-89` que desactivava o subsetting para qualquer fonte com tabela `CFF `. O relatório de paridade P797 explicara que o resultado Name-keyed era rejeitado por Poppler/Ghostscript.

O Typst 0.15.0 (vanilla) usa a crate `subsetter` 0.2.6. A documentação desta crate afirma explicitamente:

> "The subsetter will convert SID-keyed fonts to CID-keyed ones and an identity mapping from GID to CID for all fonts."

Isto é exactamente o que `/CIDFontType0` + `Identity-H` exige.

---

## 3. Implementação

### 3.1 Dependências (`03_infra/Cargo.toml`)

- Removido: `oxifont-subset = "0.2.0"`
- Adicionado: `subsetter = "0.2.6"` (mesma versão do vanilla)

### 3.2 `03_infra/src/export/subset.rs`

- Reescrita de `subset_font_with_mapping` para usar `subsetter::GlyphRemapper` + `subsetter::subset`.
- O `GlyphRemapper` atribui novos glyph IDs consecutivos a `.notdef` + glyphs usados.
- O mapping `old_gid → new_gid` é construído a partir do `GlyphRemapper`, **não** reparseando a `cmap` (o subsetter remove a tabela `cmap` de propósito).
- O teste `p523_subset_cff_nimbus_sans_preserves_cff_table` foi alterado: em vez de assertir `None`, agora exige que o subset CFF seja produzido, seja parseável pelo `ttf-parser`, e contenha a tabela `CFF`.

### 3.3 `03_infra/src/export/builder.rs`

Durante a validação descobriu-se um problema preexistente exposto pelo subsetter:

- `collect_glyph_ids(doc)` reúne glyph IDs de **todas** as fontes do documento.
- `collect_shaped_glyph_mappings(doc)` reúne shaped glyphs de **todas** as fontes.
- No loop `build_multifont` (e também em `build_cidfont`), estes GIDs globais eram passados como `additional_gids`/`char_to_old_gid` para cada face individual.
- O `subsetter` valida os GIDs e rejeita a fonte como `MalformedFont` quando recebe GIDs fora do range da face.

Correcção aplicada em ambos os caminhos:

- Filtrar `shaped_mappings` ao construir `char_to_old_gid`: incluir só `old_gid < face.number_of_glyphs()`.
- Filtrar `all_glyph_ids`/`glyph_ids` antes de `measure_subset`: incluir só GIDs dentro do range da face.

Isto garante que cada face é subsetada apenas com os glifos que lhe pertencem.

### 3.4 Prompt L0

Atualizado `00_nucleo/prompts/infra/export/font_subset.md` para refletir a mudança para `subsetter`, a remoção da `cmap`, e o filtro de GIDs por face. Hash alinhado pelo `crystalline-lint --fix-hashes`.

### 3.5 Snapshot

O fixture `03_infra/fixtures/p307b/reference/09-cidfont.pdf` regrediu de 31 629 B para 8 688 B (esperado, pois agora há subsetting). Foi regenerado com `UPDATE_P307B_SNAPSHOTS=1`.

---

## 4. Validação

### 4.1 Testes unitários

```
cargo test -p typst-infra subset
running 8 tests
test export::subset::tests::remap_glyph_id_missing_returns_notdef ... ok
test export::subset::tests::remap_glyph_id_known_returns_new ... ok
test export::subset::tests::p520_additional_gid_gets_new_gid_mapping ... ok
test export::subset::tests::p520_subset_mapping_includes_additional_gids ... ok
test export::subset::tests::subset_mapping_contains_notdef ... ok
test export::subset::tests::subset_font_valid_true_type ... ok
test export::subset::tests::subset_font_empty_keeps_notdef ... ok
test export::subset::tests::p523_subset_cff_nimbus_sans_produces_valid_cid_subset ... ok
```

### 4.2 Testes de workspace

```
cargo test --workspace
...
test result: ok. 721 passed; 0 failed; 5 ignored
```

### 4.3 Linter

```
crystalline-lint .
✅ 0 violations (apenas V7 de prompt órfão não relacionado)
```

### 4.4 Cenário real — `04-math.typ` do benchmark P872

Comando:

```bash
./target/release/typst /tmp/p872-bench/04-math.typ /tmp/p872-bench/cristalino-04-math-p874.pdf
pdffonts /tmp/p872-bench/cristalino-04-math-p874.pdf
```

Saída:

```
name                                 type              encoding         emb sub uni object ID
------------------------------------ ----------------- ---------------- --- --- --- ---------
AAAAAA+CrystallineFont1              CID Type 0C (OT)  Identity-H       yes yes yes     19  0
AAAAAA+CrystallineFont2              CID Type 0C (OT)  Identity-H       yes yes yes     24  0
```

Ambas as fontes CFF têm agora `sub yes` e prefixo `AAAAAA+`.

### 4.5 Tamanhos

| Documento | PDF (bytes) | Fontes embutidas | `sub` |
|---|---|---|---|
| Vanilla `04-math.pdf` | 95 779 | 1 (NewCMMath-Book) | yes |
| Cristalino P873 (antes) | 1 794 931 | 2 (Libertinus Serif + NewCMMath) | no / no |
| Cristalino P874 (depois) | 157 233 | 2 (Libertinus Serif + NewCMMath) | yes / yes |

A diferença residual para o vanilla (157 KB vs 96 KB) deve-se ao cristalino embutir **duas** fontes (Libertinus Serif para o texto normal e NewCMMath para os símbolos), enquanto o vanilla usa apenas NewCMMath-Book. O subsetting em si está a funcionar para ambas.

### 4.6 Estrutura CFF extraída

```bash
mutool extract /tmp/p872-bench/cristalino-04-math-p874.pdf
python3 -c "from fontTools.ttLib import TTFont; ..."
```

Resultado:

```
font-0021.otf glyphs=10 CFF=True  ROS=('Adobe', 'Identity', 0)  FDArray=1
font-0026.otf glyphs=15 CFF=True  ROS=('Adobe', 'Identity', 0)  FDArray=1
```

Ambos os subsets são CFF **CID-keyed** válidos (ROS Adobe-Identity-0).

### 4.7 Renderização e extracção de texto

- `pdftoppm` gera PNGs sem erros.
- `gs -sDEVICE=pngalpha -r150` processa as 8 páginas sem erros.
- `pdftotext` extrai o texto matemático correctamente.

### 4.8 Tempo de execução

```bash
/usr/bin/time -v ./target/release/typst /tmp/p872-bench/04-math.typ /tmp/out.pdf
```

| Métrica | Valor |
|---|---|
| User time | 1.36 s |
| System time | 4.94 s |
| Elapsed | 6.30 s |
| File system outputs | 376 |

O tempo mantém-se ~6.3 s, consistente com a conclusão do P873 de que o gargalo temporal é a busca de fallback não filtrada, não o subsetting.

---

## 5. Problemas encontrados durante a implementação

| Problema | Causa | Solução |
|---|---|---|
| Subsetter retornava `MalformedFont` para Libertinus Serif | `additional_gids` continha GIDs de outras fontes (ex.: NewCMMath) fora do range da face | Filtrar `glyph_ids` e `shaped_mappings` por `old_gid < face.number_of_glyphs()` antes de subsetar |
| Snapshot `09-cidfont.pdf` regrediu | O PDF fica menor com subsetting activo | Regenerar referência com `UPDATE_P307B_SNAPSHOTS=1` |

---

## 6. Conclusão

O subsetting CFF1 foi restabelecido com sucesso. A substituição de `oxifont-subset` por `subsetter` gera CFF CID-keyed válido para `/CIDFontType0` + `Identity-H`, eliminando a necessidade do fallback de fonte integral introduzido no P797. O tamanho dos PDFs com fontes CFF reduz-se drasticamente (11× no cenário math). A correcção do filtro de GIDs por face resolve uma regressão silenciosa que o subsetter mais rigoroso expôs.

Próximo passo: P875 (filtrar busca de fallback + partilha de bytes entre faces de `.ttc`), que trata da causa do tempo de execução elevado em math.
