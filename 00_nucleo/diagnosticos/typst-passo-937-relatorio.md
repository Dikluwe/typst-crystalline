# Relatório P937 — Implementação de coverage exacta + mmap

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-937.md`  
**Commit de implementação:** `931358677` (P937 Fase B: implementa coverage exacta + mmap)  
**Binário cristalino:** `target/release/typst` (strings `Typst compiler (crystalline)`, `typst-crystalline`)  
**Binário vanilla real:** `lab/typst-original/target/release/typst` (strings `Typst compiler`, `fontdb-0.23.0`)

---

## 1. Resumo executivo

Foi implementada a Fase B do P937: `Coverage` exacta (runs de codepoints, sem
bitmap por bloco) e carregamento lazy de fontes via `memmap2`. A cobertura
exacta eliminou os falsos positivos que obrigavam o cristalino a abrir faces
durante o shaping fallback.

**Resultado medido:**

- **Fallbacks pesados melhoraram drasticamente.** A distância ao vanilla real
caiu de ~6.4× (CJK) e ~22× (emoji) para ~4.0× e ~5.7×, respectivamente.
- **Caso comum regrediu.** O cristalino passou de ~0.33× do vanilla (P933-fixed)
para ~1.7× do vanilla. O overhead concentra-se no arranque: extrair coverage
exacta para todas as fontes do sistema é significativamente mais lento no
cristalino do que no vanilla, mesmo usando os mesmos bytes via `fontdb`.

**Veredicto:** coverage exacta + mmap funcionou para o problema do fallback,
mas a estratégia *eager* no arranque piorou o caso comum. A decisão "eager vs
lazy" (já marcada como em aberto no L0 do passo) precisa de ser revisitada com
estes números.

---

## 2. O que foi implementado

### 2.1 `Coverage` exacta (L1)

- Ficheiro: `01_core/src/entities/font_book.rs`
- Representação: `Vec<u32>` codificado como runs alternadas fora/dentro, port
directo do vanilla (`info.rs:269-318`).
- `contains(c)` percorre runs — O(n) no número de runs, sem falsos positivos.

### 2.2 `Font` com mmap (L1/L3)

- Ficheiro: `01_core/src/entities/world_types.rs`
- `Font` é agora enum `Vec(Arc<Vec<u8>>)` / `Mmap(Arc<memmap2::Mmap>)`.
- `Debug`/`PartialEq`/`Eq`/`Hash` manuais operam sobre `as_slice()`.

### 2.3 `FontSlot` lazy-mmap (L3)

- Ficheiro: `03_infra/src/fonts.rs`
- `mmap: OnceLock<Option<Arc<Mmap>>>` — criado na primeira chamada a
`source_bytes()` ou `get()`.
- `source_bytes()` devolve `Cow::Borrowed` sobre o mmap.
- Faces extraídas de `.ttc` continuam como `Font::Vec` (reconstrução de
ficheiro simples, P609/P838).

### 2.4 Extração de coverage no arranque (L3)

- Ficheiro: `03_infra/src/fonts.rs` (`extract_coverage`)
- Itera subtables unicode da `cmap` e constroi `Coverage::from_codepoints`.
- Chamado a partir de `font_info_from_bytes`, que é usado por
`load_system_fonts`/`load_fonts_from_dir` via `db.with_face_data`.

### 2.5 `SystemWorld` simplificado (L3)

- Ficheiro: `03_infra/src/world.rs`
- Removidos `coverage_cache`, `preload_coverage_if_needed`,
`embedded_coverage_union`, `source_text_nodes`.
- `candidates_for_char` delega directamente a `FontBook::candidates_for_char`.

### 2.6 Remoção da pré-carga explícita

- Ficheiro: `04_wiring/src/main.rs`
- Removida a chamada `world.preload_coverage_if_needed(&source)`.

---

## 3. Validação funcional

```bash
cargo test -p typst-core font_book      # 27 passed
cargo test -p typst-infra --lib p937    # 6 passed
cargo test -p typst-infra --lib         # 744 passed
cargo build --release -p typst-wiring   # ok
crystalline-lint .                      # 0 violations (só V7 pré-existente)
```

---

## 4. Benchmarks

### 4.1 Metodologia

Comando:

```bash
hyperfine --warmup 1 --min-runs 10 \
  'target/release/typst tools/perf/corpus/p923/<cenário>.typ /dev/null' \
  'lab/typst-original/target/release/typst compile tools/perf/corpus/p923/<cenário>.typ /dev/null -f pdf'
```

Corpus: `tools/perf/corpus/p923/` (7 cenários canônicos + 4 UTF-8).

### 4.2 Resultados — cristalino P937 vs vanilla real

| Cenário | Cristalino P937 (ms) | Vanilla real (ms) | Rácio (crist/van) |
|---|---:|---:|---:|
| `01-hello` | 484.9 | 286.6 | **1.69×** |
| `02-lorem` | 482.7 | 278.3 | **1.73×** |
| `03-math` | 476.8 | 280.3 | **1.70×** |
| `04-code` | 472.3 | 277.4 | **1.70×** |
| `05-utf8` | 1664.9 | 314.0 | **5.30×** |
| `06-matrix` | 497.9 | 304.3 | **1.64×** |
| `07-cases` | 484.7 | 296.0 | **1.64×** |
| `utf8-latin` | 502.7 | 289.8 | **1.73×** |
| `utf8-greek` | 494.3 | 290.7 | **1.70×** |
| `utf8-cjk` | 1318.3 | 327.8 | **4.02×** |
| `utf8-emoji` | 1741.8 | 304.7 | **5.72×** |

### 4.3 Comparação com o estado anterior (P933-fixed, revertido em P936)

| Cenário | P933-fixed (ms) | P937 (ms) | Vanilla real (ms) |
|---|---:|---:|---:|
| `01-hello` | 88.2 | 484.9 | 267.7 |
| `02-lorem` | 91.4 | 482.7 | 267.9 |
| `03-math` | 116.9 | 476.8 | 272.2 |
| `04-code` | 88.5 | 472.3 | 270.7 |
| `05-utf8` | 6357.0 | 1664.9 | 310.0 |
| `06-matrix` | 111.1 | 497.9 | 268.2 |
| `07-cases` | 111.2 | 484.7 | 268.3 |
| `utf8-latin` | 87.7 | 502.7 | 267.7 |
| `utf8-greek` | 87.8 | 494.3 | 267.5 |
| `utf8-cjk` | 1934.0 | 1318.3 | 303.6 |
| `utf8-emoji` | 6289.0 | 1741.8 | 283.1 |

**Observações:**

- Fallbacks pesados (`05-utf8`, `utf8-cjk`, `utf8-emoji`) melhoraram entre
2.6× e 4.8× em relação a P933-fixed.
- Caso comum piorou ~5.5× em relação a P933-fixed (de ~90 ms para ~480 ms).
- O vanilla mantém ~270–330 ms em todos os cenários; o cristalino ainda não
atingiu essa uniformidade.

---

## 5. Instrumentação de aberturas de fonte (`strace`)

Comando:

```bash
strace -f -tt -e trace=openat <binário> <cenário>.typ /dev/null
```

Contagem de `openat` sobre ficheiros `.ttf`/`.otf`/`.ttc`/`.otc`:

| Cenário | Binário | Total | Únicos | Primeira (s) | Última (s) | Duração (s) |
|---|---|---:|---:|---:|---:|---:|
| `01-hello` | Cristalino | 2198 | 1086 | 40.101 | 40.833 | 0.732 |
| `01-hello` | Vanilla | 2198 | 1086 | 40.915 | 41.486 | 0.571 |
| `05-utf8` | Cristalino | 2583 | 1086 | 41.568 | 43.113 | 1.544 |
| `05-utf8` | Vanilla | 2200 | 1086 | 43.601 | 44.195 | 0.594 |
| `utf8-cjk` | Cristalino | 2233 | 1086 | 44.303 | 45.827 | 1.524 |
| `utf8-cjk` | Vanilla | 2200 | 1086 | 45.986 | 46.625 | 0.639 |
| `utf8-emoji` | Cristalino | 2582 | 1086 | 46.736 | 48.311 | 1.575 |
| `utf8-emoji` | Vanilla | 2199 | 1086 | 48.785 | 49.399 | 0.614 |

### 5.1 Interpretação

- **Arranque:** ambos abrem ~2200 ficheiros de fonte no início (1086 únicos).
O cristalino demora ~0.73 s a fazer essas aberturas; o vanilla demora ~0.57 s.
A diferença de ~0.16 s no arranque explica parte, mas não a totalidade, da
regressão de ~0.2 s no caso comum.
- **Fallbacks:** nos cenários UTF-8, o cristalino continua a abrir mais ficheiros
durante o shaping (total 2583 vs 2200 do vanilla), embora muito menos do que
em P933-fixed (3455/4136 medidos em P936). Isso confirma que a coverage exacta
reduziu, mas não eliminou totalmente, as aberturas durante o fallback.
- **Distribuição temporal:** no vanilla, 100 % das aberturas concentram-se no
arranque. No cristalino, ainda há aberturas espalhadas pelo shaping nos casos
de fallback pesado.

---

## 6. Análise

### 6.1 Porque melhoraram os fallbacks?

A coverage exacta eliminou os falsos positivos do bitmap por bloco de 256.
Em P933-fixed, um caractere CJK/emoji passava pelo filtro grosseiro e obrigava
a abrir várias faces para confirmar (`face_covers_char`). Com coverage exacta,
o filtro já é definitivo: se `coverage.contains(c)` é falso, a face é
descartada sem I/O.

### 6.2 Porque piorou o caso comum?

O cristalino anterior (P933-fixed) usava coverage aproximada e extraía-a de
forma lazy/cheap no arranque. O P937 passou a extrair coverage exacta para
todas as ~1086 faces no arranque. O vanilla também faz isso, mas é mais rápido:

- O vanilla abre as mesmas fontes em ~0.57 s; o cristalino demora ~0.73 s.
- A diferença residual (~0.1–0.2 s) pode vir de:
  - Overhead na nossa construção de `Coverage::from_codepoints` (embora a
    lógica seja equivalente à do vanilla).
  - Diferenças no pipeline de renderização do cristalino que não foram
    alterados neste passo.
  - Outras inicializações em `SystemWorld` que ainda não estão alinhadas com o
    vanilla.

### 6.3 A estratégia eager é a culpada?

Sim, parcialmente. O material do P937 já levantava a questão: "extrair coverage
exata eager no arranque (como o vanilla) ou manter lazy?" Os números mostram
que eager no cristalino é caro o suficiente para regredir o caso comum.

A hipótese a testar no próximo passo é: **manter a coverage exacta, mas extraí-la
lazy** — só quando uma face é candidata a fallback. Isso preservaria o ganho nos
fallbacks (o filtro exacto evita falsos positivos) sem pagar o custo de scan de
todas as fontes no arranque.

---

## 7. Decisões e implicações para o próximo passo

1. **Não regressar o caso comum.** A melhoria nos fallbacks não compensa a
regressão de ~5× no caso comum. O próximo passo deve focar em recuperar o
tempo de arranque.
2. **Testar coverage exacta lazy.** Em vez de extrair coverage para todas as
fontes no arranque, extrair apenas quando a fonte é candidata. Isso requer
mudança estrutural em `FontSlot`/`SystemWorld` para cachear coverage extraída.
3. **Investigar a diferença residual de arranque.** Mesmo com eager, o vanilla é
~0.16 s mais rápido a abrir as mesmas fontes. Entender se é o fontdb, o
`ttf_parser`, ou outro factor.
4. **Manter o formato de runs.** A representação por runs está validada: sem
falsos positivos, paridade com o vanilla.

---

## 8. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Testes | `cargo test -p typst-infra --lib` | commit `931358677` | 744 passed |
| Build release | `cargo build --release -p typst-wiring` | commit `931358677` | binário `target/release/typst` atualizado |
| Benchmarks | `hyperfine --warmup 1 --min-runs 10` | commit `931358677` | 11 cenários, JSONs em `tools/perf/results/p937/` |
| `strace` | `strace -f -tt -e trace=openat` | commit `931358677` | 4 cenários × 2 binários, logs em `tools/perf/results/p937/strace/` |
| Vanilla real | strings + paths | `lab/typst-original/target/release/typst` | Confirmado distintivo do vanilla, não cristalino (lição de P934) |

---

## 9. Validação final

- [x] `cargo test -p typst-core font_book` — ok.
- [x] `cargo test -p typst-infra --lib p937` — ok.
- [x] `cargo test -p typst-infra --lib` — ok (744 passed).
- [x] `cargo build --release -p typst-wiring` — ok.
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Benchmark completo contra vanilla real.
- [x] Instrumentação `strace` de aberturas de fonte.
