# P674 — Custo fixo de arranque em documentos micro

**Data:** 2026-07-10  
**Commit base:** `0afc14763 — P673: torna FaceCache de shaped_width persistente ao longo do documento`

---

## Resumo

Após P657–P673 fecharem os gargalos de `shape_ms`, o `macro-10x` estava a 1,26× do vanilla, mas todos os documentos micro (34 de 34) continuavam entre 1,5× e 2× do vanilla — incluindo `test-array`, que mal faz trabalho real. O vanilla demorava ~95–100 ms nestes documentos; o cristalino demorava ~190 ms, sem variar muito com o conteúdo. Este passo isolou e removeu o custo fixo de arranque.

A causa era a **leitura duplicada de todas as fontes do sistema**: `fontdb` lia cada ficheiro de fonte para construir a base de dados, e depois `build_font_book` relia os mesmos ficheiros para extrair `FontInfo`. Com ~650 faces de sistema instaladas, isto produzia ~1343 chamadas `read` no arranque, consumindo ~182 ms de system time.

A correção reutiliza os bytes já carregados pelo `fontdb` via `db.with_face_data(face.id, |data, index| font_info_from_bytes(data, index))`, eliminando a segunda passagem de I/O.

---

## Sonda

### Documento mínimo

```bash
cat > /tmp/p674-vazio.typ <<'EOF'
a
EOF
```

### Hyperfine — antes da correção

```
Benchmark 1: ./target/release/typst /tmp/p674-vazio.typ /tmp/p674.pdf
  Time (mean ± σ):     189.7 ms ±   3.0 ms    [User: 7.7 ms, System: 181.8 ms]

Benchmark 2: lab/typst-original/target/release/typst compile /tmp/p674-vazio.typ /tmp/p674-vanilla.pdf
  Time (mean ± σ):      94.6 ms ±   0.6 ms    [User: 60.2 ms, System: 36.7 ms]

Summary: vanilla 2.00× mais rápido.
```

### `--timings-json` — antes

```json
{"parse_ms":0.0,"eval_ms":0.28,"introspect_ms":0.02,"expand_context_ms":0.0,
 "layout_ms":0.15,"shape_ms":0.12,"subset_ms":0.21,"render_ms":0.08,
 "total_ms":0.86}
```

A soma das fases instrumentadas é < 1 ms, mas o processo total é ~190 ms. O custo está **fora** das fases já instrumentadas — no arranque, antes do processamento do documento.

### `strace -c` — antes

```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ------------------
 80,45    0,166105         123      1343           read
  4,95    0,010221           3      2682      2436 readlink
  3,77    0,007786          13       588           munmap
  3,67    0,007569           5      1315         2 openat
  2,79    0,005768           3      1451         5 statx
  2,42    0,005002           3      1313           close
...
```

**1343 chamadas `read`** e 1315 `openat` confirmam escaneamento intensivo de ficheiros de fonte logo no arranque.

### Localização no código

- `04_wiring/src/main.rs:81-87` — `SystemWorld::new(...).with_fonts_and_system(&font_paths)` executa antes da compilação.
- `03_infra/src/world.rs:170-184` — `with_fonts_and_system` chama `crate::fontdb::load_system_fonts()`.
- `03_infra/src/fontdb.rs:23-46` — `load_system_fonts` usa `fontdb::Database::load_system_fonts()` e depois `crate::fonts::build_font_book(&slots)`.
- `03_infra/src/fonts.rs:203-212` — `build_font_book` faz `std::fs::read(&slot.path)` para cada face, relendo todos os ficheiros já lidos pelo `fontdb`.

---

## Implementação

Atualizado o Prompt L0 `00_nucleo/prompts/infra/fontdb.md` para mandar reutilizar os bytes do `fontdb` em vez de reler o disco.

Mudança em `03_infra/src/fontdb.rs`:

```rust
pub fn load_system_fonts() -> (Vec<FontSlot>, FontBook) {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();

    let mut slots = Vec::new();
    let mut book = FontBook::new();
    for face in db.faces() {
        let (path, index) = match &face.source { ... };

        let info = db
            .with_face_data(face.id, |data, idx| font_info_from_bytes(data, idx))
            .flatten();

        slots.push(FontSlot::new(path, index));
        if let Some(info) = info {
            book.push(info);
        }
    }

    (slots, book)
}
```

A mesma otimização foi aplicada a `load_fonts_from_dir`. Slots cujos `FontInfo` falhem a extrair são mantidos para preservar os índices; o `FontBook` simplesmente não os inclui.

---

## Resultados — depois da correção

### Hyperfine — depois

```
Benchmark 1: ./target/release/typst /tmp/p674-vazio.typ /tmp/p674-depois.pdf
  Time (mean ± σ):      40.3 ms ±   1.2 ms    [User: 8.8 ms, System: 31.2 ms]

Benchmark 2: lab/typst-original/target/release/typst compile /tmp/p674-vazio.typ /tmp/p674-vanilla.pdf
  Time (mean ± σ):      95.5 ms ±   1.8 ms    [User: 60.0 ms, System: 37.7 ms]

Summary: cristalino 2.37× mais rápido que vanilla.
```

### `strace -c` — depois

```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ------------------
 23,47    0,010544           3      2682      2436 readlink
 21,27    0,009555           7      1198           munmap
 15,66    0,007035           5      1315         2 openat
 13,01    0,005846           4      1213           mmap
 12,20    0,005481           3      1451         5 statx
 10,10    0,004539           3      1313           close
  2,47    0,001109          10       110           getdents64
  0,89    0,000402           3       123           read
...
```

Chamadas `read` caíram de **1343 para 123**.

### Benchmark completo (`tools/perf/benchmark-p507.py`)

Documentos micro passaram de ~1,9–2,0× do vanilla para ~0,39–0,43× do vanilla (ou seja, o cristalino ficou ~2,3–2,5× mais rápido que o vanilla nestes documentos). Exemplos:

| Documento | Vanilla (ms) | Cristalino (ms) | Razão |
|-----------|-------------:|----------------:|------:|
| test-array | 99.89 | 41.47 | 0.42 |
| test-calc | 94.26 | 39.35 | 0.42 |
| test-page | 97.55 | 39.76 | 0.41 |
| test-par | 96.40 | 40.05 | 0.42 |
| macro-10x | — | — | 1.18× (de 1,26×) |

O `macro-10x` melhorou ligeiramente (1,26× → 1,18×), mas o ganho principal é no custo fixo de arranque, que é irrelevante para documentos grandes.

---

## Validação

- `cargo test --workspace` — passou (todas as crates, todos os testes).
- `crystalline-lint .` — zero violations.
- `cargo build --release` — sem erros.
- Documento mínimo medido antes e depois; ganho confirmado.
- Falhas pré-existentes no benchmark (`test-str-methods`, `medium-combined`, `0.15.0-spec`) foram verificadas: também ocorrem no commit base `0afc14763`, portanto não são regressões desta alteração.

---

## Decisões e notas

- O `FontBook` continua a ser construído a partir dos mesmos dados; apenas a fonte dos bytes mudou (do disco para a memória já mapeada pelo `fontdb`). A paridade linguagem-vs-vanilla mantém-se.
- A otimização não altera a ordem dos slots nem os índices; slots inválidos são preservados.
- Não foi criado cache persistente entre execuções porque o ganho já atingiu o objetivo: o cristalino está abaixo do vanilla em documentos micro.

---

## Hash do commit

`686be5c48 — P674: elimina leitura duplicada de fontes do sistema no arranque`
