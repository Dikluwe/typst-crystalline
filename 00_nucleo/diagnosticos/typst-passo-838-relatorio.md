# Relatório — typst-passo-838: fallback CJK sem similarity scoring (achado #24 de P831)

**Origem**: achado #24 de P831 (lote 5). Prompt: `00_nucleo/materialization/typst-passo-838.md`.

## Proveniência

- **HEAD no arranque**: `ee02d0b2b` ("chore: P837 — #22/#23 ..."), árvore limpa (`git status --porcelain` vazio), 2026-07-22 ~16:20 -03.
- **Medições "antes"**: HEAD acima, árvore limpa, binários release já existentes (`lab/typst-original/target/release/typst` = vanilla 0.15.0 (969087ec), jun 29; `target/release/typst` = cristalino pré-P838, jul 22 16:20).
- **Medições "depois"**: working tree **não commitado** com as alterações deste passo (lista exacta no fim), binário rebuildado com `cargo build --release --bin typst` às 17:01 -03. Nota: a primeira tentativa de rebuild usou `cargo build --release -p typst`, que **falhou** (pacote chama-se `typst-wiring`; o binário é `--bin typst`) — uma medição intermédia com o binário velho ainda mostrava Droid; a medição "depois" abaixo é com o binário correcto.
- Fonte escolhida verificada por extracção das fontes embutidas no PDF (`mutool extract` + `fc-scan`) — o exportador cristalino anonimiza os nomes (`CrystallineFont*`), logo `pdffonts` não discrimina; geometria via `pdftotext -bbox`. Fixtures em `temp/p838/`.
- Decisões de implementação abaixo são **do executor** (não do dono), assinaladas como tal.

## Baseline de testes (antes)

```
cargo test -p typst-core   → 4558 passed, 0 failed
cargo test -p typst-infra  →  683 passed, 0 failed
```

## Inventário de fontes CJK do ambiente

`fc-list`: `NotoSansCJK-{Regular,Bold}.ttc` (JP/KR/SC/TC/HK + Mono, faces ttc),
`NotoSerifCJK-{Regular,Bold}.ttc` (JP/KR/SC/TC/HK), `DroidSansFallbackFull.ttf`,
mais dezenas de Noto por script. A fonte de referência do vanilla
(`NotoSansCJKjp-Regular`) **existe** no ambiente — comparação directa possível.

---

## Sonda (medição antes)

Fixtures: `Latin text 日本語のfallback test.` / `中文回退测试文本。` / `한국어 폴백 테스트.` (sem `font:` explícito).

Fontes embutidas (extracção + fc-scan):

| Caso | Vanilla | Cristalino (antes) |
|---|---|---|
| jp | LibertinusSerif-Regular + **NotoSansCJKjp-Regular** | Libertinus Serif + **Droid Sans Fallback** |
| zh | LibertinusSerif-Regular + **NotoSansCJKjp-Regular** | Libertinus Serif + **Droid Sans Fallback** |
| kr | LibertinusSerif-Regular + **NotoSansCJKjp-Regular** | **Droid Sans Fallback** (Libertinus nem embutida — todo o texto saiu na face de fallback) |

Geometria jp (`pdftotext -bbox`, 11pt):

| | Vanilla | Cristalino (antes) |
|---|---|---|
| glifo CJK | 11.0pt/glifo (160.802−116.802)/4 | **7.7pt/glifo** (147.603−116.803)/4 |
| overlap | nenhum | **overlap**: `日本語の` xMax=147.60 > `fallback` xMin=143.20 (medição e shaping usavam faces diferentes) |

### Causa raiz — duas, não uma

1. **Sem scoring** (o achado #24 declarado): o fallback global cristalino
   escolhia a primeira fonte por ordem de índice que cobre o carácter
   (`03_infra/src/shaper.rs:627-629`, `03_infra/src/font_metrics.rs:726-749`),
   enquanto o vanilla aplica `find_best_variant`/`similarity`/`distance`
   (`lab/typst-original/crates/typst-library/src/text/font/book.rs:139-234`).
2. **Faces `.ttc` incarregáveis** (descoberto durante a validação, medição
   abaixo): `extract_collection_face` (P609, `03_infra/src/fonts.rs`) fazia
   slice `data[start..end]` da face, mas os offsets do directório de tabelas
   em TTC são **absolutos ao início da colecção** (medido:
   `NotoSansCJK-Bold.ttc` face 0 no offset 52, tabela `BASE` no offset
   absoluto 2732 — fora do slice de 320 bytes). `Face::parse` falhava em
   todas as faces de colecção → **todas as Noto CJK estavam invisíveis ao
   fallback**; `Droid Sans Fallback` (um `.ttf` simples) era a única fonte
   CJK carregável. Sem esta correcção, o scoring sozinho não mudava o PDF.

### Decomposição do scoring do vanilla (medição das fontes reais)

- `like` = Libertinus Serif: panose OS/2 = `[0,0,…]` → **SERIF=false**
  (critério vanilla `matches!(panose, [2, 2..=10, ..])`, `info.rs:131-138`).
- Noto Sans CJK JP: panose `[2,11]` → serif=false → mono/serif match ✓;
  família 15 chars. Noto Serif CJK JP: panose `[2,2]` → serif=true → match ✗.
  Noto Sans Mono CJK JP: mono → match ✗. Droid: match ✓ mas 19 chars → perde
  no `Reverse(len)`. Empate JP/KR/SC/TC/HK (todas 15 chars, mesma variante) →
  primeiro na ordem do book → **JP** (coerente com o observado: vanilla
  embute `NotoSansCJKjp` até para zh/kr).

---

## Implementação (testes primeiro — RED confirmado)

RED: `cargo test -p typst-core --lib p838` → E0599 (`select_fallback`,
`FontStretch::distance` não existiam); `cargo test -p typst-infra --lib p838`
→ E0061 (aridades novas). Só depois se implementou.

### Diff (substância)

- **`01_core/src/entities/font_book.rs`** (L1): `FontStretch::distance`
  (abs_diff); `FontBook::select_fallback(like, variant, candidates)` —
  score `(like.map(similarity), Reverse(distance))` com comparação
  estritamente maior (empate total → primeiro candidato, como o vanilla);
  `fallback_similarity` = (mono match, serif match,
  `shared_prefix_words` via `unicode_words` — ADR-0013, a mesma função do
  vanilla, `Reverse(family.len())`); `fallback_distance` = (style, stretch,
  weight). **Divergência declarada (mecânica)**: sem 3.º elemento
  (flag VARIABLE) nem eixos na distance — `FontInfo` cristalino não os tem
  (VF via `axis_variations`, P525/P836). Documentado no L0.
- **`03_infra/src/fonts.rs`** (L3): `font_info_from_bytes` passa a detectar
  `flags.serif` via panose OS/2 bytes 32..45 (`face.raw_face().table(b"OS/2")`,
  critério verbatim do vanilla — antes era `false` fixo).
  `extract_collection_face` reescrita: reconstrói o ficheiro da face
  (cabeçalho sfnt + directório com offsets recalculados + bytes das tabelas,
  padding a 4, `checkSumAdjustment` do `head` recalculado) em vez de slice.
- **`03_infra/src/shaper.rs`** (L3): `CandidateSet` ganha `like` (FontInfo da
  primeira primária = `ctx.first()` do vanilla) e `variant`; no passo de
  fallback global de `covering_run`, o vencedor de `select_fallback` é movido
  para a frente da lista — em empate de comprimento de run (caso CJK típico)
  vence o scoring; run-mais-longo (P543) continua primário.
- **`03_infra/src/font_metrics.rs`** (L3): `covering(c, primary, variant)` —
  primárias primeiro; depois recolhe **todas** as fontes que cobrem e escolhe
  via `select_fallback` (antes: primeira por índice). Uma face inválida já não
  aborta o scan (`else { continue }` em vez de `?`).
- **L0**: `entities/font-book.md` (secção `select_fallback`), `infra/fonts.md`
  (panose + extracção ttc), `infra/shaper.md` (secção P838; revoga o scope-out
  "ordem do FontBook" de P534), `infra/font_metrics.md` (`covering`).

### Testes novos (14)

- L1 (9): `p838_fontstretch_distance`, `p838_select_fallback_caso_cjk_medido_p831`
  (réplica exacta do ambiente: like Libertinus + Droid/NotoSerifCJK/NotoSansMonoCJK/
  NotoSansCJK-JP/KR → JP), `p838_select_fallback_serif_match`, `..._mono_match`,
  `..._shared_prefix_words` ("Noto Sans" prefere "Noto Sans Arabic" a
  "IBM Plex Arabic"), `..._familia_mais_curta_em_empate`, `..._distance_sem_like`,
  `..._empate_total_primeiro_candidato`, `..._vazio_e_none`.
- L3 fonts (3): `p838_serif_detectado_via_panose`, `p838_extract_collection_face_produz_fonte_valida`
  (TTC sintético da fixture NimbusSans; checksum ≡ 0xB1B0AFBA),
  `p838_extract_collection_face_noto_cjk_real`.
- L3 shaper (1): `p838_fallback_global_prefere_scoring_vanilla` (book
  [DejaVu, Droid, NotoSansCJK.ttc] → run CJK vai para Noto, não Droid).
- L3 font_metrics (1): `p838_covering_fallback_scoring_vanilla` (SystemWorld
  real → Noto Sans CJK JP **Regular w400**, não a Bold w700 — a distance de
  variante decide, como no vanilla que embute `-Regular`).

---

## Medição depois (binário rebuildado)

Fontes embutidas (extracção + fc-scan):

| Caso | Cristalino (depois) | Vanilla |
|---|---|---|
| jp | Libertinus Serif + **Noto Sans CJK JP Regular** | LibertinusSerif + NotoSansCJKjp-Regular ✓ |
| zh | Libertinus Serif + **Noto Sans CJK JP Regular** | LibertinusSerif + NotoSansCJKjp-Regular ✓ |
| kr | Libertinus Serif + **Noto Sans CJK JP Regular** | LibertinusSerif + NotoSansCJKjp-Regular ✓ |

Geometria (`pdftotext -bbox`; vanilla → cristalino-depois):

- **glifo CJK jp**: 11.0pt/glifo = 11.0pt/glifo ✓ (antes: 7.7pt); overlap
  eliminado (palavras contíguas, sem interseção de bboxes).
- **zh**: linha CJK xMax 215.802 → 215.803 (Δ0.001pt = arredondamento) ✓.
- **kr**: palavras batem a 0.001–0.002pt (`한국어` 147.162→147.163,
  `폴백` 170.152→170.153); `테스트.` Δ0.64pt no fim (resíduo abaixo).
- **regressão latino** (`latin.typ`, 17 palavras + acentos): todas as bboxes
  idênticas ao vanilla a 0.001pt ✓.

### Resíduos conhecidos (documentados, não forçados)

1. **jp: `日本語の fallback` (vanilla) vs `日本語のfallback` (cristalino)** —
   o vanilla insere 2.75pt (= 11pt/4) na fronteira CJK→latino mesmo sem espaço
   na fonte. Isto é o parâmetro **`text.cjk-latin-spacing`** do vanilla
   (espaço CJK–latino de ¼ em), uma feature de texto **independente** do
   fallback de fonte — P831 atribuiu-a a este achado, mas a medição mostra
   que é outra funcionalidade (o vanilla renderiza o espaço; o cristalino
   não tem o parâmetro). Candidata a passo próprio.
2. **kr: Δ0.64pt na última palavra** (`테스트.`): origem não isolada (ponto
   final após hangul pode ser medido em face diferente ou kerning residual).
   Ordem de grandeza: 0.6% da linha. Registado para investigação futura.

## Contagens de testes (antes → depois)

```
cargo test -p typst-core   → 4558 → 4567 passed, 0 failed   (+9: os 9 testes L1 p838)
cargo test -p typst-infra  →  683 →  688 passed, 0 failed   (+5: 3 fonts + 1 shaper + 1 font_metrics)
```

## Lint / linhagem

- `crystalline-lint --fix-hashes .` actualizou os headers de `font_book.rs`,
  `fonts.rs`, `font_metrics.rs`, `shaper.rs` — **mas** atribuiu o hash errado
  em `fallback_fonts.rs` (ficheiro multi-`@prompt`, bug conhecido): escreveu o
  hash de `font_metrics.md` (`bc49e09d`) sob o `@prompt` de `shaper.md` e deixou
  o de `font_metrics.md` obsoleto (`19cb5086`) → V5 drift. Corrigido manualmente:
  `shaper.md → ac467fb3`, `font_metrics.md → bc49e09d`.
- `crystalline-lint .` final: **exit 0**, sem drift V5 (só warnings V7 de prompts
  órfãos pré-existentes, sem relação com este passo).

## Ficheiros alterados (`git diff HEAD --stat`)

```
 00_nucleo/prompts/entities/font-book.md |  42 +++++-
 00_nucleo/prompts/infra/font_metrics.md |  13 +-
 00_nucleo/prompts/infra/fonts.md        |  28 +++-
 00_nucleo/prompts/infra/shaper.md       |  35 ++++-
 01_core/src/entities/font_book.rs       | 226 +++++++++++++++++++++++++-
 03_infra/src/fallback_fonts.rs          |   4 +-
 03_infra/src/font_metrics.rs            | 104 ++++++++++---
 03_infra/src/fonts.rs                   | 227 +++++++++++++++++++++++++-----
 03_infra/src/shaper.rs                  |  92 ++++++++++--
 9 files changed, 704 insertions(+), 67 deletions(-)
```

Sem commit (instrução do passo). Fixtures de medição em `temp/p838/`
(não commitadas).

## Decisões do executor a rever antes do commit

1. **Escopo alargado à extracção ttc** (`extract_collection_face`): sem esta
   correcção o achado #24 não era validável end-to-end (todas as CJK do
   sistema estavam incarregáveis). Bug pré-existente de P609, com causa
   medida (offsets absolutos) incluída acima. Se o dono preferir passo
   separado, separar `fonts.rs` (extracção) do resto — mas o teste
   `p838_extract_collection_face_*` e a medição depois dependem dele.
2. **Run-mais-longo (P543) mantido como critério primário** no shaper, com o
   scoring a desempatar — o vanilla não considera comprimento de run (usa só
   scoring e re-faz fallback no restante). Divergência mecânica deliberada
   para não regredir P543; para o caso CJK observado o resultado é idêntico.
3. **`covering` deixou de curto-circuitar** na primeira fonte que cobre:
   agora carrega todas as faces do book que cobrem o carácter (uma vez, com
   cache). Custo único equivalente ao que o vanilla paga ao construir a
   cobertura do book no arranque; a lazy-loading do cristalino apenas o
   adia para o primeiro carácter sem cobertura primária.
4. **Alinhamento book↔slots em `fontdb.rs`**: notado que `slots.push` corre
   para todas as faces mas `book.push` só para as que parseiam — se alguma
   face do sistema falhar `font_info_from_bytes`, os índices divergem a
   partir desse ponto (latente, pré-existente, não observado neste ambiente).
   Não tratado neste passo.
