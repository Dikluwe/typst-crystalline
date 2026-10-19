# Relatório P939 — perfil da distância residual ao vanilla e investigação do flake de testes

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-939.md`  
**Commit base:** `c59424132` (P938)  
**Binário cristalino P938:** `target/release/typst-p938` (SHA-256 `6014783d...`)

---

## 1. Resumo executivo

### Parte A — perfil da distância residual

- **Implementação de `Coverage` e extração de `cmap` são equivalentes ao vanilla.**
  Lidos linha a linha: `Coverage::from_codepoints`/`contains` (`01_core/src/entities/font_book.rs:161-227`)
  correspondem a `Coverage::from_vec`/`contains` (`typst-library/src/text/font/info.rs:289-338`);
  `extract_coverage` (`03_infra/src/fonts.rs:336-347`) corresponde ao bloco de coverage em
  `FontInfo::new` (`info.rs:117-123`). Não há diferença de algoritmo ou alocação que explique a distância.
- **A descoberta inicial usa parse completo, não `RawFace`.** Tanto o cristalino (`db.with_face_data(..., font_info_from_bytes)`)
  como o vanilla (`db.with_face_data(..., FontInfo::new)`) fazem `ttf_parser::Face::parse` de cada fonte durante a indexação.
  Não há parse parcial a explorar.
- **A distância residual é dominada pelo trabalho lazy por fonte e pelo render PDF, não pela duplicação de I/O.**
  - Timings P938 (`utf8-cjk`): `layout_ms` ~739 ms, `render_ms` ~0.3 ms, total ~746 ms.
  - Timings vanilla (`utf8-cjk`): `scan system fonts` ~256 ms, `layout document` ~292 ms (inclui o scan), `pdf` ~1.1 ms.
  - O `layout_ms` do cristalino inclui a extração lazy de coverage de todas as ~1086 fontes, enquanto o vanilla
    paga esse custo no `scan system fonts` do arranque. O cristalino é ~3× mais lento nesse trabalho.
  - O `render_ms` do cristalino (~300 ms nos casos emoji/UTF-8) é ~75× mais lento que o `pdf` do vanilla (~4 ms),
    contribuindo significativamente para a distância nos cenários pesados.
- **A correção da duplicação de I/O foi implementada, medida e revertida.**
  Usando `fontdb::Database::make_shared_face_data` + `FontSlot::new_shared`, a extração lazy de coverage deixou de
  reabrir ficheiros (`strace`: 3310 → 2557 aberturas em `05-utf8`), mas o caso comum regrediu ~1.5× (~90 ms → ~140 ms)
  porque o `make_shared_face_data` é executado para todas as fontes no arranque. Como não melhorou o fallback
  (1.62 s → 1.62 s) e regrediu o caso comum, a alteração foi revertida.

### Parte B — flake de testes paralelos

- **Não reproduzido.** Correndo `cargo test -p typst-infra --lib` 20 vezes consecutivas com paralelismo por defeito
  (após as alterações do P939), a suíte passou sempre (20/20).
- **As falhas da primeira tentativa de reprodução foram invalidadas.** Os logs de `tools/perf/results/p939/flake/`
  mostram erros de compilação (E0282, E0308) introduzidos por edições a meio da execução, não falhas de teste.
- O candidato registado por P924 (`XDG_DATA_HOME` mutável em `world.rs:982-993`) continua plausível, mas não confirmado.

**Veredicto:** a duplicação de I/O identificada em P938 não explica a distância residual de 4.4×–6.1× ao vanilla.
O custo está no processamento lazy de todas as fontes (extração de coverage) e no render PDF do cristalino.
O comportamento do P938 é mantido; nenhuma alteração de código foi mantida.

---

## 2. Parte A — perfil da distância residual

### 2.1 Comparação de código: coverage

| Aspecto | Cristalino | Vanilla | Equivalente? |
|---|---|---|---|
| Representação | `Vec<u32>` runs alternadas | `Vec<u32>` runs alternadas | Sim |
| Construção | `Coverage::from_codepoints` (sort + dedup + runs) | `Coverage::from_vec` (sort + dedup + runs) | Sim |
| Consulta | `contains` O(runs), mesmo algoritmo | `contains` O(runs), mesmo algoritmo | Sim |
| Extração `cmap` | `subtable.codepoints` para subtables unicode | `subtable.codepoints` para subtables unicode | Sim |

Referências: `01_core/src/entities/font_book.rs:161-227`, `03_infra/src/fonts.rs:336-347`,
`lab/typst-original/crates/typst-library/src/text/font/info.rs:117-123,289-338`.

### 2.2 Comparação de código: descoberta inicial

- Cristalino (`03_infra/src/fontdb.rs`): `db.with_face_data(face.id, |data, idx| font_info_from_bytes(data, idx))`.
- Vanilla (`lab/typst-original/crates/typst-kit/src/fonts.rs:184-186`): `db.with_face_data(face.id, FontInfo::new)`.

Ambos fazem `ttf_parser::Face::parse` completo de cada fonte durante a indexação. Não há parse parcial (`RawFace`)
em nenhum dos lados.

### 2.3 Timings comparados

Cristalino P938 (`--timings-json`) vs vanilla (`--timings`, parseado do formato Chrome trace):

| Cenário | Cristalino layout_ms | Cristalino render_ms | Vanilla layout document (ms) | Vanilla scan system fonts (ms) | Vanilla pdf (ms) |
|---|---:|---:|---:|---:|---:|
| `05-utf8` | 784.5 | 306.9 | 289.3 | 253.8 | 3.9 |
| `utf8-cjk` | 738.9 | 0.3 | 291.9 | 255.9 | 1.1 |
| `utf8-emoji` | 768.3 | 299.7 | 268.5 | 254.2 | 3.1 |

**Interpretação:**

- O vanilla paga o custo de fontes no `scan system fonts` (~256 ms) e depois o layout é rápido (~35 ms).
- O cristalino paga a extração lazy de coverage dentro de `layout_ms` (~700 ms), e o render PDF custa ~300 ms
  nos casos emoji/UTF-8 (vs ~4 ms no vanilla).
- A soma destes dois factores explica a distância de 4.4×–6.1×, não a duplicação de I/O.

### 2.4 Tentativa de correção da duplicação de I/O (implementada, medida, revertida)

Implementação: `fontdb::Database::make_shared_face_data` + `FontSlot::new_shared` (campo `shared` no `FontSlot`),
de forma que a extração lazy de coverage reutilize o mmap da descoberta.

**Resultado do `strace` (`05-utf8`):**

| Binário | Total aberturas fonte | Únicos | Nota |
|---|---:|---:|---|
| P938 (independente) | 3310 | 1086 | Reabre cada fonte para coverage |
| P939 (partilhado) | 2557 | 1086 | Não reabre para coverage; ~385 aberturas são candidatos de shaping |

**Resultado do benchmark (4 cenários-chave):**

| Cenário | P939 (partilhado) | P938 (independente) | Vanilla |
|---|---:|---:|---:|
| `01-hello` | 136.6 ms | 106.2 ms | 287.1 ms |
| `05-utf8` | 1769.7 ms | 1753.8 ms | 310.4 ms |
| `utf8-cjk` | 1306.2 ms | 1305.0 ms | 296.0 ms |
| `utf8-emoji` | 1620.5 ms | 1587.7 ms | 275.5 ms |

**Decisão:** a partilha de mmap reduz a reabertura de ficheiros mas **regrediu o caso comum ~1.5×**
(~90 ms → ~140 ms) e **não melhorou o fallback**. A duplicação de I/O não explica a distância residual.
A alteração foi revertida; o comportamento do P938 é mantido.

---

## 3. Parte B — investigação do flake de testes paralelos

### 3.1 Primeira tentativa (invalidada)

Foi corrida `cargo test -p typst-infra --lib` 25 vezes em background. As runs 1–14 passaram; as runs 15–25 falharam.
Os logs (`tools/perf/results/p939/flake/`) mostram erros de compilação (E0282, E0308) introduzidos por edições ao
código a meio da execução — não são falhas de teste. Esta tentativa não serve para avaliar o flake.

### 3.2 Segunda tentativa (válida)

Após concluir as alterações do P939, foram corridas 20 vezes consecutivas:

```bash
for i in $(seq 1 20); do cargo test -p typst-infra --lib; done
```

**Resultado:** 20/20 passaram, 0 falhas (`tools/perf/results/p939/flake2/summary.txt`).

### 3.3 Conclusão

- O flake relatado em P938 (`p858_*`, `p534_*`, `p838_*` falhando em conjunto) **não foi reproduzido** em 20 execuções.
- O candidato de P924 (`XDG_DATA_HOME` mutável em `system_world_include_source_resolve_de_package`, `world.rs:982-993`)
  continua plausível mas não confirmado.
- Como não há causa confirmada, não se força correção — regista-se o resultado com mais detalhe que P924
  (condições: 20 execuções consecutivas, paralelismo por defeito, `nice -n 10`, suíte `typst-infra --lib`).

---

## 4. Benchmarks completos (11 cenários)

Comando: `hyperfine --warmup 1 --min-runs 10`. Binários:

| Binário | SHA-256 (8 chars) | Nota |
|---|---|---|
| P939 (partilhado) | — | Binário medido antes da reversão |
| P938 (independente) | `6014783d` | Estado actual |
| P937 (eager) | `e03984e3` | Referência |
| Vanilla real | `e73e4ac1` | `lab/typst-original` |

| Cenário | P939 (ms) | P938 (ms) | P937 (ms) | Vanilla (ms) | P939/P938 | P939/Vanilla |
|---|---:|---:|---:|---:|---:|---:|
| `01-hello`   | 136.6 | 106.2 | 481.1 | 287.1 | 1.29 | 0.48 |
| `02-lorem`   | 147.8 |  91.5 | 459.5 | 314.6 | 1.62 | 0.47 |
| `03-math`    | 148.9 |  96.3 | 470.1 | 277.8 | 1.55 | 0.54 |
| `04-code`    | 138.3 |  87.6 | 484.7 | 285.5 | 1.58 | 0.48 |
| `05-utf8`    | 1769.7 | 1753.8 | 1731.3 | 310.4 | 1.01 | 5.70 |
| `06-matrix`  | 144.9 |  91.6 | 441.7 | 269.3 | 1.58 | 0.54 |
| `07-cases`   | 147.7 |  96.5 | 441.9 | 264.2 | 1.53 | 0.56 |
| `utf8-latin` | 139.4 |  93.2 | 427.7 | 277.5 | 1.50 | 0.50 |
| `utf8-greek` | 136.2 |  88.4 | 434.0 | 261.2 | 1.54 | 0.52 |
| `utf8-cjk`   | 1306.2 | 1305.0 | 1256.9 | 296.0 | 1.00 | 4.41 |
| `utf8-emoji` | 1620.5 | 1587.7 | 1560.0 | 275.5 | 1.02 | 5.88 |

---

## 5. Instrumentação `strace`

| Cenário | Binário | Total | Únicos | Duração (s) |
|---|---|---:|---:|---:|
| `01-hello` | P939 | 2172 | 1086 | 0.411 |
| `05-utf8`  | P939 | 2557 | 1086 | 1.594 |

Comparação com P938 (relatório anterior): `01-hello` 2198 / `05-utf8` 3310.
A partilha de mmap eliminou a reabertura para coverage (3310 → 2557), mas o custo do `make_shared_face_data`
no arranque regrediu o caso comum.

---

## 6. Decisões e implicações

1. **Manter o comportamento do P938.** A partilha de mmap regrediu o caso comum sem melhorar o fallback.
2. **A duplicação de I/O não é a causa da distância residual.** O custo está no processamento lazy de todas as
   fontes e no render PDF.
3. **Próximos alvos de otimização:** (a) tornar a extração lazy de coverage mais barata (evitar re-parse da face,
   ou pré-filtro por bloco para reduzir o número de extrações exactas); (b) perfilar e otimizar o `render_ms` do
   cristalino (~300 ms vs ~4 ms do vanilla nos casos pesados).
4. **Flake de testes:** não reproduzido em 20 execuções; registado com condições detalhadas. Não forçar correção
   sem causa confirmada.

---

## 7. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Testes flake | `cargo test -p typst-infra --lib` ×20 | commit `c59424132` + alterações P939 (revertidas depois) | 20/20 ok |
| Benchmark 11 cenários | `hyperfine --warmup 1 --min-runs 10` | idem | JSONs em `tools/perf/results/p939/` |
| `strace` | `strace -f -tt -e trace=openat` | idem | `tools/perf/results/p939/strace-p939-*.log` |
| Timings | `--timings-json` (cristalino), `--timings` (vanilla) | idem | `tools/perf/results/p939/timings/` |
| Comparação de código | leitura directa | `lab/typst-original` | `info.rs:117-123,289-338`; `typst-kit/src/fonts.rs:184-186` |

---

## 8. Validação final

- [x] `cargo test -p typst-infra --lib` ×20 — 0 falhas.
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Comparação linha a linha de `Coverage` e extração de `cmap` — equivalentes.
- [x] Benchmark 11 cenários contra P938, P937 e vanilla real.
- [x] `strace` de aberturas de fonte.
- [x] Correção da duplicação de I/O implementada, medida e revertida (registada a razão).
