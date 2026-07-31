# Relatório P941 — glifos bitmap (CBDT/CBLC) como imagens XObject

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-941.md`  
**Commit base:** `d6880c048` (P940)  
**Binário cristalino P941:** `target/release/typst-p941` (strings `Typst compiler (crystalline)`)

---

## 1. Resumo executivo

Os glifos bitmap (CBDT/CBLC, ex.: Noto Color Emoji) passaram a ser **desenhados como imagens
XObject**, em vez de embutir a fonte inteira como `/CIDFontType2`. Isto resolve os três problemas
identificados em P940:

- **Tamanho:** `utf8-emoji.pdf` caiu de ~10.8 MB para **~52 KB**; `05-utf8.pdf` de ~10.8 MB para
  **~65 KB** — abaixo do vanilla real (~80 KB / ~69 KB). A distância de ~135× foi eliminada.
- **Tempo:** `render_ms` de emoji caiu de ~19 ms (P940) para **~10 ms** (vanilla ~4 ms).
- **Cor:** os emojis passaram a renderizar **a cores** (verificado com `mutool draw` e `pdftoppm`),
  corrigindo o defeito pré-existente de renderizarem monocromáticos.

**Ressalva (fora de escopo, pré-existente):** para 8 dos 19 codepoints emoji dos casos de teste
(🔥 ✨ 📊 🎭 🎬 🏆 🌍), o fallback do shaper escolhe outras fontes (FreeMono/FreeSans/Noto Sans
Symbols2) em vez de Noto Color Emoji — esses glifos continuam monocromáticos. É uma divergência
pré-existente do fallback (P838/P543), não do caminho de exportação. Ver secção 6.

---

## 2. Fase A — mecanismo real do vanilla e da fonte

- **Vanilla (krilla `text/glyph/bitmap.rs`):** extrai o PNG do glifo via bitmap strikes e desenha-o
  como imagem; a fonte CBDT nunca é embutida. O vanilla PDF tem só Libertinus embutida + 38 imagens
  (19 emojis × RGB+SMask).
- **`ttf_parser::Face::glyph_raster_image(gid, u16::MAX)`** já expõe tudo o necessário — sonda
  executada: `PNG 136×128, ppem 109, x=0, y=-27, ~1-4 KB por glifo`.
- **`NotoColorEmoji.ttf`:** um único strike (ppem 109), `unitsPerEm=2048`, advance 2550 — escala
  coerente `font_size/109` pt por pixel (largura da imagem ≈ advance: ~14.95 pt a 12 pt).
- **`subsetter::subset` falha com `UnknownKind`** para CBDT (confirmado em P940) — a fonte não pode
  ser subsetada nem embutida correctamente como outline.

## 3. Fase B — implementação

### 3.1 Novo módulo `03_infra/src/export/bitmap_glyphs.rs`

- `BitmapGlyph` — payload PDF (RGB + SMask opcional, via `process_png_for_pdf`), bearings, ppem.
- `BitmapGlyphRef` — nome do XObject, dimensões, bearings, ppem (para o stream de página).
- `collect_bitmap_glyphs_for_ids` — para cada glyph id, extrai o raster PNG se existir.
- `used_glyph_ids_for_face` — glyph ids usados no documento que pertencem à face.

### 3.2 `builder.rs`

- `per_font_used_glyphs` — glyph ids usados, agrupados por fonte resolvida (mesma selecção de
  índice que `emit_shaped_pdf`).
- Para cada fonte, `collect_bitmap_glyphs_for_ids` + `bitmap_only` (todos os glifos usados têm
  imagem raster).
- Cria XObjects de imagem para os glifos bitmap (**dedup por glyph id** — glifo repetido partilha
  o mesmo XObject).
- **Fonte 100% bitmap não é embutida:** o `/FontDescriptor` não referencia `/FontFile` e o stream
  da fonte é um objecto vazio (mantém a numeração do xref).
- `merge_bitmap_xobjects` — funde as entradas `/ImN id 0 R` dos glifos bitmap no bloco
  `/XObject << ... >>` das imagens do documento. **Bug encontrado e corrigido na Fase B:** sem
  esta fusão, as entradas ficavam fora do dicionário e os leitores não resolviam a referência
  (`XObject 'Im3' is unknown` no poppler; render monocromático no mutool).

### 3.3 `stream.rs`

- `FontScenario::Cidfont` e `Multifont` ganham o mapa de bitmap refs.
- `emit_bitmap_glyph_draws` — emite `q w 0 0 h x y cm /ImN Do Q` por glifo bitmap, com o cursor a
  avançar pelo `x_advance` do shaper (o espaço na linha é idêntico ao de um glifo de fonte).
- Posicionamento confirmado visualmente: `y_bottom = base_y + bearing_y * scale`
  (`bearing_y = -27` px em NotoColorEmoji).

### 3.4 Testes (Fase B, TDD)

3 testes novos em `03_infra/src/integration_tests.rs`:

- `p941_emoji_unico_gera_imagem_sem_embute_cbdt` — PDF contém `/Subtype /Image` e < 500 KB.
- `p941_emoji_repetido_dedup_um_xobject` — `🎉🚀🎉` → exactamente 2 entradas no dicionário
  `/XObject` (dedup por glifo).
- `p941_documento_misto_texto_e_emoji` — texto latino mantém fonte embutida (`/FontFile`), emoji
  sai como imagem.

**Suíte:** `cargo test -p typst-infra --lib` → **748 passed**; `cargo test --workspace` → verde.
**Linter:** `crystalline-lint .` → 0 drift (V7 pré-existente).

---

## 4. Fase C — medições

### 4.1 Tamanho do PDF (o eixo que motivou o passo)

| Cenário | P938 (comprimido) | P940 (sem compressão) | **P941** | Vanilla real |
|---|---:|---:|---:|---:|
| `utf8-emoji` | 10 006 257 | 10 812 350 | **52 420** | 80 560 |
| `05-utf8` | 10 020 516 | 10 826 609 | **65 242** | 69 627 |

A distância de ~135× foi eliminada — o PDF cristalino ficou **mais pequeno que o vanilla**
(compressão Flate das imagens RGB eficiente).

### 4.2 `render_ms` isolado

| Cenário | P938 (ms) | P940 (ms) | **P941 (ms)** | Vanilla (ms) |
|---|---:|---:|---:|---:|
| `05-utf8` | 319.3 | 19.6 | **12.3** | ~4 |
| `utf8-emoji` | 325.5 | 18.7 | **10.1** | ~4 |
| `utf8-cjk` | 0.3 | 0.3 | **0.3** | ~1 |
| `01-hello` | 0.3 | 0.3 | **0.2** | ~1 |

### 4.3 Benchmark completo (11 cenários, `hyperfine --warmup 1 --min-runs 10`)

| Cenário | P941 (ms) | P940 (ms) | P938 (ms) | Vanilla (ms) | P941/P940 | P941/P938 | P941/Vanilla |
|---|---:|---:|---:|---:|---:|---:|---:|
| `01-hello`   |   87.3 |   87.4 |   87.5 | 256.7 | 1.00 | 1.00 | 0.34 |
| `02-lorem`   |   90.9 |   89.7 |   90.3 | 257.0 | 1.01 | 1.01 | 0.35 |
| `03-math`    |   93.5 |   94.0 |   93.4 | 260.5 | 0.99 | 1.00 | 0.36 |
| `04-code`    |   89.7 |   88.9 |   88.6 | 256.0 | 1.01 | 1.01 | 0.35 |
| `05-utf8`    | 1274.4 | 1289.1 | 1574.6 | 289.2 | 0.99 | 0.81 | 4.41 |
| `06-matrix`  |   90.1 |   89.8 |   89.6 | 255.0 | 1.00 | 1.01 | 0.35 |
| `07-cases`   |   89.2 |   89.5 |   90.1 | 254.5 | 1.00 | 0.99 | 0.35 |
| `utf8-latin` |   88.0 |   87.6 |   87.6 | 254.1 | 1.00 | 1.00 | 0.35 |
| `utf8-greek` |   87.6 |   88.1 |   87.3 | 254.5 | 1.00 | 1.00 | 0.34 |
| `utf8-cjk`   | 1252.5 | 1250.1 | 1245.1 | 285.5 | 1.00 | 1.01 | 4.39 |
| `utf8-emoji` | 1257.9 | 1279.6 | 1538.3 | 268.2 | 0.98 | 0.82 | 4.69 |

**Interpretação:**
- **Caso comum:** zero regressão (P941 ≈ P940 ≈ P938).
- **Emoji/UTF-8:** P941 ≈ P940 em tempo (ambos rápidos), mas P941 ganha o eixo de tamanho e cor.
  Face a P938: ~18–19% mais rápido (`05-utf8` 0.81×, `utf8-emoji` 0.82×).
- **Distância ao vanilla:** ~4.4–4.7× nos cenários pesados (dominada pelo `layout_ms` — extração
  lazy de coverage, ver P939).

### 4.4 Confirmação visual lado a lado

`mutool draw` a 200 dpi, crop da linha de emojis:

- **Vanilla:** todos os emojis a cores (Noto Color Emoji).
- **P941:** 🎉🚀💯🌟💻📝✅❌⚠️⭐🎨🎵 **a cores** (imagens XObject); 🔥✨📊🎭🎬🏆🌍
  monocromáticos (texto noutras fontes — ver secção 6).
- **P938/P940:** todos monocromáticos.

`pdftoppm` sem erros de XObject (antes da correcção do dicionário de recursos: `XObject 'Im3'
is unknown`).

---

## 5. Quanto da distância total isto fecha

- O eixo de **tamanho** está fechado (~135× → paridade/menor que o vanilla).
- O eixo de **tempo** (`render_ms`): ~300 ms → ~10 ms nos casos emoji; o restante da distância
  total ao vanilla (~4.4–4.7×) está no `layout_ms` (extração lazy de coverage de todas as fontes),
  que é o próximo alvo (P938/P939).

---

## 6. Achado fora de escopo — divergência do fallback para alguns emojis

Para 🔥 ✨ 📊 🎭 🎬 🏆 🌍, o shaper escolhe FreeMono/FreeSans/Noto Sans Symbols2 em vez de
Noto Color Emoji. Medido em P941:

- A **cobertura** de NotoColorEmoji inclui esses codepoints (confirmado por sonda) — não é falha
  de coverage.
- O **scoring** de `select_fallback` penaliza NotoColorEmoji: `isFixedPitch=1` na fonte → flag
  `mono=true`, que perde o critério `mono_match` contra o `like` não-monospace (Libertinus Serif).
  FreeMono (`isFixedPitch=0` → `mono=false`) e companhia ganham o critério.
- O vanilla escolhe NotoColorEmoji para **todos** os emojis (render todo a cores). A divergência
  está no fallback (P838/P543), não no caminho de exportação.

**Candidato a passo próprio:** investigar porque o vanilla selecciona NotoColorEmoji nestes casos
(o scoring portado deveria penalizá-la da mesma forma) e alinhar a selecção de fallback.

---

## 7. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Sondas Fase A | `cargo test` (sondas temporárias), `fontTools` | working tree P941 | raster PNG, strikes, cmap, isFixedPitch |
| Tamanho PDF | `ls -la` de PDFs gerados | `typst-p941`/`typst-p940`/`typst-p938`/vanilla | secção 4.1 |
| `render_ms` | `--timings-json` | `typst-p941` | secção 4.2 |
| Benchmark 11 cenários | `hyperfine --warmup 1 --min-runs 10` | `typst-p941` | JSONs em `tools/perf/results/p941/` |
| Visual | `mutool draw` (200 dpi), `pdftoppm` | `typst-p941` vs vanilla | lado a lado |
| Suíte | `cargo test --workspace` | working tree P941 | 748 em typst-infra; verde |
| Linter | `crystalline-lint .` | working tree P941 | 0 drift (V7 pré-existente) |

---

## 8. Validação final

- [x] Glifos bitmap exportados como imagem XObject, com dedup por glifo.
- [x] Tamanho do PDF de emoji/UTF-8 na mesma ordem de grandeza do vanilla (menor, de facto).
- [x] Emojis a renderizar a cores (11/19; os restantes por divergência pré-existente do fallback).
- [x] `render_ms` residual reduzido (~19 ms → ~10 ms).
- [x] Benchmark 11 cenários: zero regressão no caso comum.
- [x] `cargo test --workspace` verde (748 em typst-infra).
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Confirmação visual lado a lado com o vanilla.
- [x] Nota sobre a distância restante (tempo: `layout_ms`/coverage; fallback emoji: passo próprio).
