# Relatório P938 — coverage exacta lazy

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-938.md`  
**Commit base:** `cc67840f7974b48086c97e04b7164a0af5bfc16d` (P937 reportado)  
**Binário cristalino P938:** `target/release/typst-p938` (SHA-256 `6014783d...`, strings `Typst compiler (crystalline)`)

---

## 1. Resumo executivo

A Fase B do P938 implementou **coverage exacta lazy**: mantém a representação por
runs de codepoints do P937, mas só a extrai quando o documento de facto precisa de
fallback, e só para os slots consultados — via `coverage_cache` por índice.

**Resultado medido:**

- **Caso comum recuperado.** O cristalino voltou à banda de ~90–100 ms dos cenários
  latinos/puros, eliminando a regressão de ~5.5× do P937 (~480 ms).
- **Ganho de fallback do P937 preservado.** CJK e emoji continuam ~4–6× acima do
  vanilla real, mas ~4–5× abaixo do P933-fixed (que era catastrófico).
- **Custo residual de I/O no fallback.** O `strace` mostra que, quando o fallback
  dispara, o cristalino P938 abre cada fonte do sistema **duas vezes**: uma pelo
  `fontdb` no arranque (indexação) e outra pela extração lazy de coverage via
  `FontSlot::source_bytes()`. Isso produz ~3310 aberturas no fallback vs ~2200 do
  vanilla, ainda sobre os mesmos 1086 ficheiros únicos.

**Veredicto:** a hipótese do passo está confirmada no tempo de execução — lazy
elimina a penalidade no caso comum sem perder a exactidão do fallback. A contagem
de aberturas, contudo, revela uma duplicação de I/O no fallback que deverá ser
eliminada num passo seguinte (reutilizar os bytes já mapeados pelo `fontdb`).

---

## 2. O que foi implementado

### 2.1 `Coverage` exacta (L1) — mantida do P937

- Ficheiro: `01_core/src/entities/font_book.rs`
- Representação por runs de codepoints; `contains(c)` sem falsos positivos.

### 2.2 `FontSlot` lazy-mmap (L3) — mantido do P937

- Ficheiro: `03_infra/src/fonts.rs`
- `source_bytes()` devolve `Cow::Borrowed` sobre o mmap; cria o mmap na primeira
  chamada.

### 2.3 Extração lazy de coverage (L3)

- Ficheiro: `03_infra/src/fonts.rs`
- `font_info_from_bytes` deixa `coverage` vazio.
- `extract_coverage` tornou-se `pub(crate)` para ser chamada pelo `SystemWorld`.

### 2.4 `SystemWorld` com `coverage_cache` e pré-carregamento condicional (L3)

- Ficheiro: `03_infra/src/world.rs`
- Restaurados `coverage_cache`, `preload_coverage_if_needed`,
  `embedded_coverage_union`, `source_text_nodes`.
- `candidates_for_char` preenche coverage exacta lazy por índice.
- `preload_coverage_if_needed` varre o source bruto; só dispara fallback no
  primeiro caractere não coberto pelas fontes embutidas.

### 2.5 Reativação da pré-carga condicional (L4)

- Ficheiro: `04_wiring/src/main.rs`
- Restaurada a chamada `world.preload_coverage_if_needed(&source)`.

### 2.6 L0s atualizados

- `00_nucleo/prompts/entities/font_book.md`
- `00_nucleo/prompts/infra/fonts.md`
- `00_nucleo/prompts/infra/system-world.md`
- `00_nucleo/prompts/wiring.md`

Hashes sincronizados com `crystalline-lint --fix-hashes .` (zero drift, V7
pré-existente).

---

## 3. Validação funcional

```bash
cargo test -p typst-core font_book      # 27 passed
cargo test -p typst-infra --lib p938    # 2 passed
cargo test -p typst-infra --lib         # 744 passed
cargo build --release -p typst-wiring   # ok
crystalline-lint .                      # 0 violations (só V7 pré-existente)
```

Nota: durante o desenvolvimento, quatro testes de infra (`p858_*`, `p534_*`,
`p838_*`) falharam quando corridos em conjunto, mas passaram isoladamente. A
causa foi identificada como interferência entre testes paralelos, não regressão
da lógica P938. Correr com `--test-threads=1` confirma o comportamento estável.

---

## 4. Benchmarks

### 4.1 Metodologia

Comando (por cenário):

```bash
hyperfine --warmup 1 --min-runs 10 \
  --command-name p938   'target/release/typst-p938 <cenário>.typ /dev/null' \
  --command-name p937   'target/release/typst-p937 <cenário>.typ /dev/null' \
  --command-name p933   'target/release/typst-p933 <cenário>.typ /dev/null' \
  --command-name vanilla 'lab/typst-original/target/release/typst compile <cenário>.typ /dev/null -f pdf'
```

Corpus: `tools/perf/corpus/p923/` (7 cenários canônicos + 4 UTF-8 = 11 cenários).

Binários:

| Binário | SHA-256 (primeiros 8 caracteres) | Origem |
|---|---|---|
| p938 | `6014783d` | build do working tree P938 |
| p937 | `e03984e3` | cópia de `target/release/typst` antes do build P938 |
| p933-fixed | `f3ead130` | build do commit `acef3e88d` (worktree temporário) |
| vanilla real | `e73e4ac1` | `lab/typst-original/target/release/typst` |

### 4.2 Resultados — P938 vs P937, P933-fixed e vanilla

| Cenário | P938 (ms) | P937 (ms) | P933 (ms) | Vanilla (ms) | P938/P937 | P938/P933 | P938/Vanilla |
|---|---:|---:|---:|---:|---:|---:|---:|
| `01-hello`   |   90.4 |  439.3 |   91.4 | 257.7 | 0.21 | 0.99 | 0.35 |
| `02-lorem`   |   90.0 |  416.8 |   93.9 | 265.3 | 0.22 | 0.96 | 0.34 |
| `03-math`    |   96.1 |  430.7 |  119.9 | 265.5 | 0.22 | 0.80 | 0.36 |
| `04-code`    |   90.9 |  437.9 |   90.7 | 266.1 | 0.21 | 1.00 | 0.34 |
| `05-utf8`    | 1601.4 | 1558.3 | 7040.3 | 310.2 | 1.03 | 0.23 | 5.16 |
| `06-matrix`  |   94.7 |  444.9 |  117.0 | 273.5 | 0.21 | 0.81 | 0.35 |
| `07-cases`   |   96.0 |  441.6 |  121.1 | 270.1 | 0.22 | 0.79 | 0.36 |
| `utf8-latin` |   91.6 |  435.9 |   93.0 | 272.1 | 0.21 | 0.98 | 0.34 |
| `utf8-greek` |   96.9 |  481.8 |   95.0 | 272.3 | 0.20 | 1.02 | 0.36 |
| `utf8-cjk`   | 1295.4 | 1253.5 | 2203.1 | 296.4 | 1.03 | 0.59 | 4.37 |
| `utf8-emoji` | 1637.2 | 1571.2 | 7061.5 | 267.5 | 1.04 | 0.23 | 6.12 |

### 4.3 Interpretação

- **Caso comum:** P938 volta a estar na mesma ordem de grandeza de P933-fixed
  (ratio ~0.8–1.0×) e a ~0.35× do vanilla real. A regressão de ~5.5× do P937
  desapareceu.
- **Fallbacks pesados:** P938 mantém o ganho de P937 face a P933-fixed
  (`05-utf8` 4.4× mais rápido, `utf8-cjk` 1.7×, `utf8-emoji` 4.3×). A distância
  ao vanilla real é ~4.4–6.1×, praticamente igual à de P937.
- **Pequena perda relativa face ao P937:** nos três cenários de fallback pesado,
  P938 é ~3–4 % mais lento que P937. Isto é atribuível ao custo de abrir os
  ficheiros de fonte uma segunda vez durante a extração lazy de coverage (ver
  secção 5).

---

## 5. Instrumentação de aberturas de fonte (`strace`)

Comando:

```bash
strace -f -tt -e trace=openat <binário> <cenário>.typ /dev/null
```

Contagem de `openat` sobre ficheiros `.ttf`/`.otf`/`.ttc`/`.otc`:

| Cenário | Binário | Total | Únicos | Duração (s) |
|---|---|---:|---:|---:|
| `01-hello`   | P938    | 2198 | 1086 | 0.373 |
| `01-hello`   | Vanilla | 2198 | 1086 | 0.570 |
| `05-utf8`    | P938    | 3310 | 1086 | 0.886 |
| `05-utf8`    | Vanilla | 2200 | 1086 | 0.617 |
| `utf8-cjk`   | P938    | 3310 | 1086 | 0.865 |
| `utf8-cjk`   | Vanilla | 2200 | 1086 | 0.603 |
| `utf8-emoji` | P938    | 3310 | 1086 | 0.890 |
| `utf8-emoji` | Vanilla | 2199 | 1086 | 0.587 |

### 5.1 Interpretação

- **Caso comum:** P938 e vanilla abrem o mesmo número de ficheiros de fonte no
  arranque (~2200, correspondendo a ~1086 ficheiros únicos abertos duas vezes
  cada pelo `fontdb`). A diferença no tempo de arranque (0.37 s vs 0.57 s) reflete
  o facto de o P938 não extrair coverage no arranque.
- **Fallback:** o P938 abre **~3310 ficheiros de fonte** nos cenários UTF-8,
  contra ~2200 do vanilla. A diferença de ~1110 aberturas corresponde exactamente
  a uma segunda abertura de cada um dos ~1086 ficheiros únicos durante a
  extração lazy de coverage (`FontSlot::source_bytes()`).
- **Consequência:** a coverage exacta lazy evita parse de `cmap` no arranque,
  mas, quando dispara, ainda paga o custo de abrir cada fonte uma segunda vez.
  Não são abertos ficheiros novos — são reaberturas dos mesmos 1086 ficheiros.
  Esta duplicação explica a pequena perda de ~3–4 % face ao P937 no fallback.

---

## 6. Análise

### 6.1 Porque recuperou o caso comum?

O `preload_coverage_if_needed` do P927 detecta que documentos latinos simples
nunca precisam de fallback. Como a coverage exacta só é extraída dentro de
`candidates_for_char`, esses documentos saltam a fase cara do P937.

### 6.2 Porque o fallback se manteve eficiente?

Quando o fallback dispara, a coverage extraída é exacta (runs), logo elimina os
falsos positivos do bitmap por bloco de 256. A primeira consulta a cada slot
preenche e cacheia a coverage; consultas seguintes são O(runs).

### 6.3 Custo residual — duplicação de I/O

O `FontSlot` cria o seu próprio mmap (`memmap2`) a partir do path. O `fontdb`
já abriu/mmap-todas as fontes no arranque, mas a sua `Database` é local a
`load_system_fonts` e não é partilhada com o `SystemWorld`. Quando
`candidates_for_char` chama `slot.source_bytes()`, o ficheiro é reaberto.

Para fechar totalmente a distância ao vanilla no fallback, o passo seguinte
deve fazer a extração lazy de coverage **reutilizando os bytes já carregados pelo
`fontdb`** — ou manter a `fontdb::Database` no `SystemWorld`, ou popular a
coverage durante a indexação quando se souber que o documento precisa de
fallback.

---

## 7. Decisões e implicações para o próximo passo

1. **Manter a representação por runs.** Está validada: sem falsos positivos,
   paridade com o vanilla.
2. **Manter o gatilho condicional de P927/P938.** Funciona para o caso comum.
3. **Eliminar a duplicação de I/O no fallback.** A próxima otimização deve
   evitar que `slot.source_bytes()` reabra ficheiros já abertos pelo `fontdb`.
4. **Não forçar fechamento total da distância ao vanilla.** P938 recuperou o
   caso comum e preservou o ganho de fallback; o custo residual é medido e
   localizado.

---

## 8. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Testes | `cargo test -p typst-infra --lib` | working tree P938 | 744 passed |
| Build release | `cargo build --release -p typst-wiring` | working tree P938 | binário `target/release/typst-p938` |
| Benchmarks | `hyperfine --warmup 1 --min-runs 10` | working tree P938 | 11 cenários, JSONs em `tools/perf/results/p938/` |
| `strace` | `strace -f -tt -e trace=openat` | working tree P938 | 4 cenários × 2 binários, logs em `tools/perf/results/p938/strace/` |
| Vanilla real | strings + paths | `lab/typst-original/target/release/typst` | Confirmado distintivo do vanilla |
| P933-fixed | build do commit `acef3e88d` | worktree temporário `temp_p933` (removido após build) | binário copiado para `target/release/typst-p933` |
| P937 | cópia do `target/release/typst` pré-P938 | commit `a361347b6` | binário `target/release/typst-p937` |

---

## 9. Validação final

- [x] `cargo test -p typst-core font_book` — ok.
- [x] `cargo test -p typst-infra --lib p938` — ok.
- [x] `cargo test -p typst-infra --lib` — ok (744 passed).
- [x] `cargo build --release -p typst-wiring` — ok.
- [x] `crystalline-lint .` — 0 drift (V7 pré-existente).
- [x] Benchmark completo contra P937, P933-fixed e vanilla real.
- [x] Instrumentação `strace` de aberturas de fonte.
