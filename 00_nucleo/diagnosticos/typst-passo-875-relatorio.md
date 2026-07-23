# Relatório — P875: Filtro de fallback por cobertura Unicode + partilha de bytes entre faces `.ttc`

**Data:** 2026-07-23T18:27:56-03:00  
**Commit:** `30122788891b32a423b0354b97eebdbf28ccf068`  
**Estado:** fechado  
**L0s afetados:**
- `00_nucleo/prompts/entities/font-book.md` → `f3f3080f`
- `00_nucleo/prompts/infra/fonts.md` → `0bfce99e`
- `00_nucleo/prompts/infra/shaper.md` → `e39ccdac`

---

## 1. Problema

P873 mediu que, quando um caractere não é coberto pelas fontes primárias, o shaper percorria **todo** o `FontBook` (~1086 fontes de sistema) chamando `load_fallback` para cada slot. Para colecções TrueType (`.ttc`) grandes como `NotoSansCJK` (~19 MB, 10 faces), cada `FontSlot::get()` lia o ficheiro inteiro da coleção para extrair uma face. Em documentos math, símbolos matemáticos/gregos ausentes em CJK faziam com que se lessem dezenas de MB de ficheiros CJK só para confirmar que nenhum deles tinha o glifo.

Dois problemas independentes:
1. **Busca não filtrada:** o fallback global não usava nenhum metadado de cobertura Unicode; carregava faces para depois rejeitar.
2. **Leitura duplicada:** faces irmãs do mesmo `.ttc` não partilhavam os bytes do ficheiro físico.

---

## 2. Solução

### 2.1 Bitmap de cobertura Unicode em `FontInfo` (L1)

`01_core/src/entities/font_book.rs`:
- Novo tipo `Coverage` com 64×64 bits (4096 bits), cada bit representa um bloco de 256 codepoints (U+0000..U+0FFFFF).
- `Coverage::insert(codepoint)`, `contains(codepoint)`, `is_empty()`.
- Campo `coverage: Coverage` adicionado a `FontInfo`.
- Novo método `FontBook::candidates_for_char(c: char)` devolve os índices cujo bitmap cobre o bloco de `c`.

A aproximação por bloco de 256 é deliberada: equilíbrio entre precisão e tamanho do `FontInfo` (~512 bytes por face). O shaper continua a verificar `face.glyph_index(c)` depois do filtro.

### 2.2 Extração de cobertura e partilha de bytes (L3)

`03_infra/src/fonts.rs`:
- `font_info_from_bytes` preenche `coverage` percorrendo a tabela `cmap` via `subtable.codepoints(...)` (ttf_parser 0.25).
- `FontSlot` ganha campo `shared_source: Option<Arc<OnceLock<Option<Arc<Vec<u8>>>>>>`.
- `discover_fonts` cria slots de `.ttc`/`.otc` com a mesma `Arc<OnceLock<...>>`.
- `FontSlot::get()` e `FontSlot::source_bytes()` usam o cache partilhado: a primeira face lê o ficheiro, as restantes reutilizam o `Arc<Vec<u8>>`.
- `pair_slots_with_book` usa `source_bytes()` para também beneficiar da partilha.

### 2.3 Filtro no fallback global (L3)

`03_infra/src/shaper.rs`:
- `CandidateSet::covering_all` itera `world.book().candidates_for_char(c)` em vez de todo o `FontBook`.
- Candidatos cujo bitmap não cobre o bloco de `c` são ignorados antes de carregar a face.
- A semântica de escolha final (`select_fallback` P838) e a ordem de preferência são preservadas.

---

## 3. Ficheiros alterados

- `01_core/src/entities/font_book.rs`
- `03_infra/src/fonts.rs`
- `03_infra/src/shaper.rs`
- `03_infra/src/pipeline.rs` (ajuste de `FontInfo { coverage: ... }` em testes)
- `01_core/src/engine/eval/tests.rs` (ajuste de `FontInfo { coverage: ... }`)

---

## 4. Validação

### 4.1 Testes

```text
$ cargo test --workspace
...
test result: ok. 4686 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
 test result: ok. 727 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
 test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
...
Total: 5495 passed (contando doc-tests ignorados)
```

**+5 testes** em relação ao estado de P874 (722 → 727 no `typst-infra`), todos explicados na secção 5.

### 4.2 Linter

```text
$ crystalline-lint .; echo "exit: $?"
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' ... [V7]
exit: 0
```

Zero violations. O único warning (V7) é pré-existente e não relacionado com P875.

### 4.3 Compilação manual

Documento math com símbolos greques e CJK compilou com sucesso:

```text
$ ./target/release/typst /tmp/math_fallback.typ /tmp/math_fallback.pdf
```

### 4.4 Medições de I/O

Tentativa de medição comparativa com `strace` e `/usr/bin/time`:
- Documento `/tmp/math_fallback.typ` (latim + CJK + símbolo math U+2A3F).
- Com P875: ~6.64–6.65 s (3 runs).
- Sem filtro P875 (revertido temporariamente): ~6.65–6.70 s (3 runs).

A diferença é marginal no ambiente atual. A razão principal: a **descoberta inicial de fontes** (`discover_fonts` / `face_count`) já gera a maioria do I/O de fontes, lendo cada ficheiro uma vez para determinar o número de faces. O P875 optimiza o caminho de **fallback durante o shaping** e a **partilha entre faces irmãs**, mas neste documento específico o shaping não domina o tempo total.

A otimização permanece correta e é coberta pelos testes unitários:
- `p875_covering_all_vazio_quando_nenhum_bloco_cobre` confirma que, quando nenhuma fonte cobre o bloco do caractere, **nenhuma face é carregada** (`face_cache.map.is_empty()`).
- `p875_ttc_slots_partilham_source` confirma que slots do mesmo `.ttc` partilham o mesmo `OnceLock`.

Uma medição de alto impacto exigiria um cenário controlado onde o shaping/fallback seja o gargalo dominante (por exemplo, muitos caracteres raros distribuídos por múltiplos blocos, ou documento que carregue dezenas de faces de um `.ttc`). Esse cenário fica para validação futura com benchmark dedicado.

---

## 5. Testes adicionados

### `01_core/src/entities/font_book.rs` (+4 testes)

- `p875_coverage_insert_contains_por_bloco`
- `p875_coverage_ignora_codepoints_acima_do_bitmap`
- `p875_candidates_for_char_inclui_bloco_coberto`
- `p875_candidates_for_char_exclui_bloco_nao_coberto`

### `03_infra/src/fonts.rs` (+3 testes)

- `p875_font_info_coverage_nao_vazia_para_fonte_real`
- `p875_font_info_bytes_invalidos_coverage_default`
- `p875_ttc_slots_partilham_source`

### `03_infra/src/shaper.rs` (+2 testes)

- `p875_covering_all_filtra_por_coverage_bitmap`
- `p875_covering_all_vazio_quando_nenhum_bloco_cobre`

---

## 6. Decisões e limitações

- **Bitmap por bloco de 256:** aproximação aceite. O shaper ainda verifica `glyph_index(c)` para evitar falsos positivos.
- **`Coverage` em `FontInfo`:** aumento de ~512 bytes por face. Para ~1086 fontes de sistema, ~540 KB adicionais no `FontBook` — considerado aceitável face à redução de I/O.
- **Partilha de bytes via `OnceLock`:** não é estado global; o `Arc` é criado no momento da descoberta e mantido enquanto houver slots vivos.
- **Codepoints ≥ U+100000:** ignorados no bitmap. Caso raro para fallback de texto; documentado.

---

## 7. Próximos passos

- P876: cache de imagem por `FileId`/conteúdo (duplicação de imagens medida em P873).
