# Relatório P940 — `render_ms` de emoji ~75× mais lento que o vanilla

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-940.md`  
**Commit base:** `657bb6676` (materializações P939/P940)  
**Binário cristalino P940:** `target/release/typst-p940` (strings `Typst compiler (crystalline)`)

---

## 1. Resumo executivo

**Causa exacta confirmada:** o `subsetter` (mesma biblioteca do vanilla, versão 0.2.6) falha em fontes bitmap por
CBDT/CBLC (`UnknownKind`). Nesse caso, o export embute a fonte inteira (~10.8 MB para Noto Color Emoji) e a
compressão FlateDecode do stream de fonte domina o `render_ms` (~300 ms medidos). O vanilla evita o problema
porque **não embute a fonte**: desenha os glifos bitmap como imagens (krilla `text/glyph/bitmap.rs`).

**Correção:** emitir streams de fonte acima de 256 KB sem compressão FlateDecode. O caminho normal
(subset bem-sucedido) continua comprimido. Isto reduz o `render_ms` de emoji/UTF-8 de ~300 ms para ~19 ms
(~16× melhoria) e o tempo total dos cenários pesados em ~17%.

**Veredicto:** a distância de 75× em `render_ms` foi reduzida para ~5× (19 ms vs ~4 ms do vanilla). Os emojis
continuam a renderizar exactamente como antes (monocromáticos — limitação pré-existente da embed CBDT como
CIDFontType2). A renderização a cores de glifos bitmap (como imagens, estilo krilla) fica como trabalho futuro.

---

## 2. Fase A — isolamento da causa

### 2.1 Fonte usada nos testes

Os casos `utf8-emoji.typ` e `05-utf8.typ` contêm emojis que só são cobertos por
`/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf` — fonte **CBDT/CBLC** (bitmap embutido, não `COLR`/`SVG`).

### 2.2 Comportamento do cristalino

- `03_infra/src/export/subset.rs::subset_font_with_mapping` usa `subsetter::subset`.
- Para NotoColorEmoji, `subsetter::subset` devolve `Err(UnknownKind)` — a biblioteca não reconhece o formato CBDT.
- O export cai no fallback `embed_font_data = font_data.to_vec()` (fonte inteira, ~10.8 MB).
- `build_font_stream` comprime esse stream com FlateDecode — ~300 ms de CPU.

Medição directa (`03_infra/src/export/subset.rs::p940_measure_noto_color_emoji_subset`):

```
NotoColorEmoji: fonte 10788068 bytes, 3976 glifos, 19 chars mapeados
subsetter::subset falhou: UnknownKind
subset FALHOU (None) — export embute fonte inteira
```

### 2.3 Comportamento do vanilla

O vanilla usa `krilla` para exportar PDF. Em `krilla/src/text/glyph/bitmap.rs`, glifos com dados bitmap
(`BitmapData::Png`) são **desenhados como imagens** na superfície, não embutidos como fonte. A fonte CBDT nunca
é embutida; logo, não há stream de 10 MB para comprimir.

### 2.4 Custo por glifo vs custo fixo

O custo é **fixo e único** (compressão do stream de fonte), não proporcional ao número de emojis. Isto confirma
que não é "emoji ser mais complexo" — é a compressão do fallback de fonte inteira.

---

## 3. Fase B — implementação

### 3.1 Alteração

`03_infra/src/export/builder.rs::build_font_stream`:

- Se `font_stream_data.len() <= MAX_COMPRESS_FONT_STREAM` (256 KB), comportamento inalterado (comprime).
- Se maior, emite o stream **sem compressão** — o custo de CPU cai para uma cópia de memória.

O caminho normal (subset bem-sucedido, streams pequenos) continua comprimido. Não há alteração de conteúdo.

### 3.2 Testes

- `cargo test -p typst-infra --lib` — **745 passed** (inclui o novo teste `p940_measure_noto_color_emoji_subset`,
  que documenta a falha de subsetting CBDT).
- `crystalline-lint .` — 0 drift (V7 pré-existente).

### 3.3 Confirmação visual

PDF gerado com `target/release/typst-p940` e renderizado com `mutool draw`: os emojis aparecem **exactamente
como antes** (monocromáticos, mesmos glifos). A alteração não muda o conteúdo renderizado — só o tempo de exportação.

Nota: os emojis renderizam monocromáticos porque a fonte CBDT é embutida como CIDFontType2 (sem outlines),
limitação pré-existente. A renderização a cores requer o caminho de glifos-bitmap-como-imagens (futuro).

---

## 4. Fase C — medições

### 4.1 `render_ms` isolado

| Cenário | P938 (ms) | P940 (ms) | Melhoria |
|---|---:|---:|---:|
| `05-utf8` | 319.3 | 19.6 | ~16× |
| `utf8-emoji` | 325.5 | 18.7 | ~17× |
| `utf8-cjk` | 0.3 | 0.3 | 1× (inalterado) |
| `01-hello` | 0.3 | 0.3 | 1× (inalterado) |

### 4.2 Benchmark completo (11 cenários)

Comando: `hyperfine --warmup 1 --min-runs 10`.

| Cenário | P940 (ms) | P938 (ms) | P937 (ms) | Vanilla (ms) | P940/P938 | P940/Vanilla |
|---|---:|---:|---:|---:|---:|---:|
| `01-hello`   |  93.7 |  94.7 | 455.3 | 274.9 | 0.99 | 0.34 |
| `02-lorem`   |  95.5 |  94.7 | 448.5 | 277.9 | 1.01 | 0.34 |
| `03-math`    | 103.7 |  94.6 | 452.8 | 278.9 | 1.10 | 0.37 |
| `04-code`    |  95.4 |  95.8 | 437.2 | 293.2 | 1.00 | 0.33 |
| `05-utf8`    | 1359.1 | 1642.5 | 1615.0 | 315.4 | 0.83 | 4.31 |
| `06-matrix`  | 101.8 |  89.8 | 424.8 | 257.8 | 1.13 | 0.39 |
| `07-cases`   |  89.7 |  90.1 | 417.7 | 254.4 | 1.00 | 0.35 |
| `utf8-latin` |  88.1 |  87.6 | 413.3 | 253.4 | 1.01 | 0.35 |
| `utf8-greek` |  87.7 |  87.2 | 415.4 | 258.1 | 1.01 | 0.34 |
| `utf8-cjk`   | 1309.8 | 1268.6 | 1291.7 | 313.9 | 1.03 | 4.17 |
| `utf8-emoji` | 1370.8 | 1644.2 | 1616.0 | 295.2 | 0.83 | 4.64 |

### 4.3 Interpretação

- **Caso comum:** inalterado (P940/P938 ≈ 1.0; pequenas variações são ruído).
- **Fallback pesado com emoji:** melhoria de ~17% no tempo total (`05-utf8` 1.66 s → 1.36 s; `utf8-emoji` 1.64 s → 1.37 s).
- **CJK:** inalterado (P940/P938 ≈ 1.03, dentro do ruído) — confirma que o sinal era específico de emoji (CBDT).
- **Distância ao vanilla:** reduzida de ~5.3×/5.7× para ~4.3×/4.6× nos cenários emoji.

### 4.4 Quanto da distância geral isto fecha

O `render_ms` explicava ~300 ms da distância de ~1.3 s ao vanilla nos cenários emoji. Removendo-o, o cristalino
fica ~280 ms mais rápido, fechando ~20% da distância total. O restante (~1.0 s) está no `layout_ms`
(extração lazy de coverage de todas as fontes) — ver P939.

---

## 5. Decisões e implicações

1. **Manter o limiar de 256 KB para compressão de streams de fonte.** Elimina o custo de compressão do fallback
   integral sem afectar o caminho normal.
2. **Registar a limitação do subsetter com CBDT.** O teste `p940_measure_noto_color_emoji_subset` documenta que
   `subsetter::subset` devolve `UnknownKind` para Noto Color Emoji.
3. **Renderização a cores de glifos bitmap é trabalho futuro.** Requer um novo caminho de exportação (glifos
   bitmap como imagens XObject, estilo krilla `text/glyph/bitmap.rs`). Esse caminho também eliminaria a necessidade
   de embutir a fonte CBDT, reduzindo o tamanho do PDF de ~10 MB para ~50 KB.

---

## 6. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Causa subsetting CBDT | `cargo test -p typst-infra --lib p940_measure_noto_color_emoji_subset` | working tree P940 | `UnknownKind` confirmado |
| `render_ms` | `--timings-json` | `target/release/typst-p940` | 4 cenários |
| Benchmark 11 cenários | `hyperfine --warmup 1 --min-runs 10` | `target/release/typst-p940` | JSONs em `tools/perf/results/p940/` |
| Confirmação visual | `mutool draw` | `target/release/typst-p940` | Emojis inalterados (monocromáticos) |
| Vanilla real | strings + paths | `lab/typst-original/target/release/typst` | Confirmado distintivo |
| Suíte completa | `cargo test -p typst-infra --lib` | working tree P940 | 745 passed |
| Linter | `crystalline-lint .` | working tree P940 | 0 drift (V7 pré-existente) |

---

## 7. Validação final

- [x] Causa exacta confirmada (subsetter CBDT `UnknownKind` + compressão Flate do fallback integral).
- [x] `render_ms` isolado reduzido de ~300 ms para ~19 ms (~16×).
- [x] Benchmark 11 cenários: melhoria ~17% nos cenários emoji, caso comum inalterado.
- [x] `cargo test -p typst-infra --lib` — 745 passed.
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Confirmação visual — emojis inalterados.
