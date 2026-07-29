# Relatório P925 — diagnóstico: fallback lazy de fontes do sistema é lento para CJK/emoji

**Precede este passo:** `typst-passo-923-relatorio.md`, Fase E.2 — outlier `05-utf8` (25.91×
cristalino/vanilla), isolado até `SystemWorld::candidates_for_char` (`03_infra/src/world.rs:517`).

**Data:** 2026-07-28.

---

## Resumo executivo

O outlier `05-utf8` não é uma regressão de P922/P923; é um problema estrutural do fallback de
fontes no cristalino. Medições temporárias mostram que uma única mudança arquitectural —
pré-computar a coverage Unicode no `FontInfo` durante o arranque e usá-la para filtrar
candidatos sem carregar faces — reduz o tempo de `05-utf8` de **~8.6s para ~0.20s** (~43×),
ficando abaixo do vanilla (~0.31s). O custo é um aumento do tempo de arranque de ~0.2s para
~1.8s nesta máquina.

**Recomendação:** prosseguir com a pré-computação de coverage no arranque e com o uso da
coverage do `FontInfo` nos caminhos de fallback do shaper e do `FallbackFontMetrics`.

---

## Fase A — medição do mecanismo actual

### A.1 — `SystemWorld::candidates_for_char` (`03_infra/src/world.rs:517`)

A função itera sobre **todos** os `font_slots`. Para cada slot ainda não cacheado:

1. Chama `slot.source_bytes()` — lê o ficheiro de fonte do disco (ou bytes embutidos).
2. Faz `ttf_parser::Face::parse(&data, slot.index)` — parse completo da fonte.
3. Extrai a coverage via `extract_coverage(&face)`.
4. Guarda a coverage no `coverage_cache: Mutex<HashMap<usize, Coverage>>`.

Resultado: no **primeiro** caractere que exige fallback, todas as fontes do sistema são
parseadas de uma só vez. Medições:

| | Valor |
|---|---|
| Famílias de fontes únicas no sistema | 608 |
| Fontes com CJK (zh/ja/ko) | 31 |
| Fontes emoji | 1 |
| Slots carregados por `load_system_fonts()` | 1112 |
| Tempo `load_system_fonts()` (sem coverage) | ~199 ms |
| Tempo para extrair coverage de todos os slots | ~1593 ms |
| Tempo total se coverage for pré-computada | ~1792 ms |

A coverage é um bitmap de 4096 bits por blocos de 256 codepoints
(`01_core/src/entities/font_book.rs:155-196`). Uma vez preenchido o cache, caracteres do
mesmo bloco são rápidos; o custo domina no primeiro caractere de um bloco não coberto pela
fonte primária.

### A.2 — `FontInfo` já é construído no arranque, mas com coverage vazia

`03_infra/src/fontdb.rs:48-50` chama `font_info_from_bytes(data, idx)` para cada face
devolvida pelo `fontdb`. Esta função (`03_infra/src/fonts.rs:294-357`) JÁ faz
`ttf_parser::Face::parse` completo para extrair family, variant, flags, etc., mas deixa
`coverage: Coverage::new()` de propósito (comentário P880).

Isto significa que o parse completo das fontes já é pago no arranque; só a iteração da
`tabela cmap` é adiada para `candidates_for_char`.

### A.3 — `covering_all`/`best_covering_run` no shaper carregam candidatos em excesso

`03_infra/src/shaper.rs:667-697` (`covering_all`) devolve todos os candidatos cujo bloco
cobre o caractere. Para cada um, `best_covering_run` chama `face_covers_char`, que carrega a
fonte via `face_cache.get(world, slot_idx)` para verificar `glyph_index(c)`. Como há dezenas
de candidatos por caractere, isto carrega centenas de faces durante o shape de um documento
com fallback.

Medição temporária (input `05-utf8.typ`, commit `da18ea9f3`):

| | Original | Só coverage pré-computada |
|---|---:|---:|
| `FontSlot::get` misses | 1129 | 1129 |
| Tempo total | ~8.6 s | ~7.7 s |

A pré-computação isolada de coverage pouco ajuda porque o shaper ainda carrega todas as
faces candidatas.

### A.4 — `FallbackFontMetrics::covering` também carrega candidatos em excesso

`03_infra/src/font_metrics.rs:939-991` percorre `self.world.candidates_for_char(c)` e, para
cada slot, chama `self.cached_face(slot_idx)` para verificar `glyph_index(c)`. Este caminho é
usado por `advance`, `text_ink_bounds`, etc., que são chamados pelo layout — daí o
`layout_ms` alto mesmo quando o `shape_ms` é baixo.

### A.5 — medição temporária da solução completa

Foram aplicadas três alterações temporárias (não commitadas):

1. `font_info_from_bytes` preenche `coverage: extract_coverage(&face)`.
2. `CandidateSet::covering_all`/`best_covering_run` usam `FontInfo::coverage` para filtrar
candidatos; só carregam a face escolhida para shape.
3. `FallbackFontMetrics::covering` passa os índices filtrados por coverage diretamente para
`FontBook::select_fallback`, sem carregar faces intermediárias.

Resultado para `05-utf8.typ`:

| | Original | Solução temporária | Vanilla 0.15.0 |
|---|---:|---:|---:|
| Tempo total | ~8.6 s | **0.20 s** | 0.31 s |
| `layout_ms` | ~7012 ms | 52 ms | — |
| `shape_ms` | ~555 ms | 2 ms | — |
| `FontSlot::get` misses | 1129 | **5** | — |

A solução temporária é **mais rápida que o vanilla** neste input específico.

---

## Fase B — como o vanilla faz

### B.1 — `fontdb` no vanilla (`lab/typst-original/crates/typst-kit/src/fonts.rs:170-194`)

O vanilla cria uma `fontdb::Database`, chama `db.load_system_fonts()` e itera sobre
`db.faces()`. O `fontdb` usa `ttf_parser::RawFace::parse` (parse parcial, só tabelas
necessárias: `name`, `OS/2`, `post`) para indexar as fontes — rápido, ~20ms para 1906 faces
(documentação do `fontdb`).

### B.2 — `FontInfo::new` no vanilla (`lab/typst-original/crates/typst-library/src/text/font/info.rs:47-157`)

Para cada face, o vanilla chama `FontInfo::new(data, index)`, que faz `Face::parse` completo
e extrai a coverage iterando a `cmap`. Isto acontece no momento de construção do
`FontStore`, ou seja, **no arranque**.

### B.3 — fallback no vanilla (`lab/typst-original/crates/typst-library/src/text/font/book.rs:94-115`)

`FontBook::select_fallback` itera sobre os `FontInfo` e verifica
`info.coverage.contains(c as u32)`. Não carrega nenhuma fonte nesta fase. Só depois de
escolhido o índice é que a face é carregada para shaping.

---

## Fase C — opções de correcção

### Opção 1 — Pré-computar coverage no arranque (recomendada)

**Descrição:** em `font_info_from_bytes`, extrair `extract_coverage(&face)` e guardar no
`FontInfo`. Em `candidates_for_char`, devolver os índices cujo `FontInfo::coverage` contém o
bloco. No shaper e no `FallbackFontMetrics`, usar a coverage do `FontInfo` para filtrar
candidatos sem carregar faces; só carregar a face escolhida.

**Custo:** aumento do tempo de arranque de ~0.2s para ~1.8s nesta máquina (1112 slots).

**Benefício:** elimina o parse duplo; fallback torna-se O(n) sobre bitmaps em memória;
`05-utf8` passa de ~8.6s para ~0.2s.

**Risco:** muda o contrato de `FontInfo` (campo `coverage` passa a ser significativo no
arranque). Requer actualização do L0 `00_nucleo/prompts/infra/fontdb.md` e
`00_nucleo/prompts/infra/system-world.md`. Pode alongar o arranque para documentos que nunca
precisam de fallback; no entanto, o parse completo das fontes já é feito hoje, portanto o
custo incremental é só a iteração do `cmap`.

### Opção 2 — Cache de coverage mais amplo (por bloco Unicode)

**Descrição:** manter a estratégia lazy, mas preencher o `coverage_cache` por slot quando o
primeiro caractere de um bloco é consultado, em vez de preencher todos os slots de uma vez.

**Custo:** menos invasivo; não altera o arranque.

**Benefício:** reduz o impacto do primeiro caractere se o documento só tocar alguns blocos.

**Risco:** não resolve o problema do shaper/`FallbackFontMetrics` carregarem faces
intermediárias; o ganho real seria limitado.

### Opção 3 — Parsing parcial da tabela `cmap`

**Descrição:** manter a estratégia lazy, mas em vez de `Face::parse` completo, usar
`RawFace` ou parse parcial só para extrair a `cmap`.

**Custo:** mais complexo; `ttf_parser` não expõe API directa para isso.

**Benefício:** reduz o custo por fonte aberta.

**Risco:** pode introduzir bugs de parsing; ganho incerto face à Opção 1.

### Opção 4 — Usar `fontdb` com dados partilhados

**Descrição:** aproveitar a API `Database::make_shared_face_data` do `fontdb` para manter os
bytes das fontes mapeados em memória e reutilizáveis.

**Custo:** mudança estrutural no `FontSlot` e no `SystemWorld`.

**Benefício:** reduz I/O de disco e alocações.

**Risco:** usa memory-mapped files (unsafe); altera a arquitectura de carregamento de
fontes.

---

## Fase D — recomendação

Recomenda-se **Opção 1** — pré-computar coverage no arranque — por três razões:

1. **Alinha-se com o vanilla**, que paga o custo da coverage no arranque.
2. **O parse completo já é feito hoje** em `font_info_from_bytes`; a iteração extra do
   `cmap` é um custo incremental pequeno comparado com o ganho no fallback.
3. **Resolve ambos os gargalos** identificados (`candidates_for_char` e carregamento de
   faces intermediárias no shaper/`FallbackFontMetrics`).

Próximo passo: actualizar os Prompts L0 (`fontdb.md`, `system-world.md`, possivelmente
`shaper.md` e `font_metrics.md`), seguir o Protocolo de Nucleação, e implementar a Opção 1
com TDD e medições.

---

## Proveniência

- Commit base da análise: `da18ea9f3`.
- Medições de arranque: teste temporário em `03_infra/src/fontdb.rs` (removido após
  medição).
- Medições de performance: `tools/perf/benchmark-utf8-before-after.py`,
  `tools/perf/results/utf8-before-after/attestation.json`.
- Input de teste: `tools/perf/corpus/p923/05-utf8.typ`.
- Binário vanilla: `lab/typst-original/target/release/typst` (`typst 0.15.0`).
