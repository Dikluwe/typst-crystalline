# Relatório P936 — Reversão de P935 e estudo holístico do pipeline de fontes do vanilla

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-936.md`  
**Commit base:** `acef3e88d0d5d7cc62c06dfbb741590aca2cc331` (P933-fixed)  
**Binário vanilla real:** `lab/typst-original/target/release/typst` — versão CLI `typst 0.15.0 (969087ec)`, strings `fontdb-0.23.0`.

---

## 1. Resumo executivo

A Fase B de P935 (coverage eager via `std::fs::read`, sem mmap) foi **revertida**.
O estudo holístico do pipeline do vanilla confirma a hipótese de P936 com uma
nuance importante: a **uniformidade** do tempo do vanilla (~270–310 ms para
latim, CJK e emoji) não vem só de "coverage exacta" nem só de "mmap" — vem da
**combinação** das duas:

1. O vanilla abre/mmap-todas as fontes do sistema no arranque (~83 ms medidos
   em P935 para ~1112 faces). Isso é barato porque o kernel mapeia sob demanda.
2. A coverage extraída nesse arranque é **exacta** (lista de runs de
   codepoints), não aproximada por bloco de 256.
3. Durante o shaping fallback, o vanilla consulta apenas o bitmap exacto
   (`info.coverage.contains(c)`) e nunca reabre uma face para confirmar um
   candidato.

O cristalino (P933-fixed) faz o oposto: coverage **aproximada** (bitmap de
bloco de 256, com falsos positivos) e carregamento **lazy** das fontes. Isso
obriga a abrir faces durante o shaping para confirmar candidatos, e o tempo
varia com o número de falsos positivos de cada script.

**Veredicto:** hipótese confirmada. A correção seguinte precisa de ambas as
peças — coverage exacta + I/O barato (mmap persistente ou equivalente). Portar
apenas uma não resolve.

---

## 2. Parte A — reversão para P933-fixed

### 2.1 Ficheiros revertidos

- `03_infra/src/fonts.rs`
- `03_infra/src/fontdb.rs`
- `03_infra/src/world.rs`
- `03_infra/src/shaper.rs` (não estava listado no passo, mas necessário para os testes passarem)
- `04_wiring/src/main.rs`
- `03_infra/Cargo.toml` (já coincidente com P933-fixed)
- L0s revertidos:
  - `00_nucleo/prompts/infra/fonts.md`
  - `00_nucleo/prompts/infra/fontdb.md`
  - `00_nucleo/prompts/infra/system-world.md`
  - `00_nucleo/prompts/wiring.md`

### 2.2 Hashes actualizados

```bash
crystalline-lint --fix-hashes .
```

Resultado: 0 drift warnings (apenas V7 pré-existente, `package_version_resolution.md`).

### 2.3 Validação

```bash
cargo test --workspace
```

Resultado: ok (743 passed em `typst-infra`, restantes crates ok).

```bash
cargo build --workspace --release
```

Resultado: ok.

### 2.4 Benchmark de confirmação (7 cenários canônicos + 4 UTF-8)

Comando:

```bash
hyperfine --warmup 1 --min-runs 3 \
  'target/release/typst <input>.typ /dev/null' \
  'lab/typst-original/target/release/typst compile <input>.typ /dev/null -f pdf'
```

| Cenário | Cristalino revertido (P933-fixed) | Vanilla real | Rácio (crist/van) |
|---|---:|---:|---:|
| `01-hello` | 88.2 ms | 267.7 ms | **0.33×** |
| `02-lorem` | 91.4 ms | 267.9 ms | **0.34×** |
| `03-math` | 116.9 ms | 272.2 ms | **0.43×** |
| `04-code` | 88.5 ms | 270.7 ms | **0.33×** |
| `05-utf8` | 6 357 ms | 310.0 ms | **20.51×** |
| `06-matrix` | 111.1 ms | 268.2 ms | **0.41×** |
| `07-cases` | 111.2 ms | 268.3 ms | **0.41×** |
| `utf8-latin` | 87.7 ms | 267.7 ms | **0.33×** |
| `utf8-greek` | 87.8 ms | 267.5 ms | **0.33×** |
| `utf8-cjk` | 1 934 ms | 303.6 ms | **6.37×** |
| `utf8-emoji` | 6 289 ms | 283.1 ms | **22.22×** |

*Nota sobre variação:* os três cenários de fallback pesado (`utf8-cjk`,
`utf8-emoji`, `05-utf8`) medidos aqui ficam ~10–20 % abaixo dos valores
registados em P934 (`2371 ms`, `7716 ms`, `7744 ms`). Latim/grego batem quase
exacto. Correndo com `--min-runs 5` os valores sobem para `2126 ms`,
`6878 ms` e `6841 ms`, aproximando-se mais de P934. A diferença residual é
atribuível a variação de cache/sistema entre sessões — o código é o mesmo
(revertido para P933-fixed) e a ordem de grandeza é idêntica.

**Conclusão da reversão:** os valores estão na mesma ordem de grandeza do
P933-fixed reportado em `typst-passo-933-relatorio.md`. O estado revertido é
funcionalmente equivalente ao P933-fixed.

---

## 3. Parte B — estudo holístico do pipeline do vanilla

### 3.1 Código-fonte do vanilla

#### `typst-library/src/text/font/info.rs:117-156`

O vanilla extrai coverage **exacta** no arranque:

```rust
let mut codepoints = vec![];
for subtable in ttf.tables().cmap.into_iter().flat_map(|table| table.subtables) {
    subtable.codepoints(|c| codepoints.push(c));
}
...
coverage: Coverage::from_vec(codepoints),
```

`Coverage` (`info.rs:269-318`) é um `Vec<u32>` codificado como runs alternadas
(de fora/dentro). `contains(c)` faz busca binária por runs — **O(log n)** e
sem falsos positivos.

#### `typst-library/src/text/font/book.rs:94-115`

`select_fallback` consulta apenas o coverage exacto, sem abrir face:

```rust
let c = text.chars().find(|&c| !c.is_whitespace() && !is_default_ignorable(c))?;
let ids = self.infos.iter().enumerate()
    .filter(|(_, info)| info.coverage.contains(c as u32))
    .map(|(index, _)| index);
self.find_best_variant(like, variant, ids)
```

**Observável:** nenhuma chamada a `Face::parse`, `font()` ou abertura de ficheiro
neste caminho.

### 3.2 Instrumentação com `strace`

Contagem de `openat` sobre ficheiros `.ttf`/`.otf`/`.ttc`/`.otc`:

| Cenário | Binário | Total aberturas fonte | Ficheiros únicos | Primeira abertura | Última abertura |
|---|---|---:|---:|---:|---:|
| `utf8-cjk` | Vanilla | 2 200 | 1 086 | 0,258 s | 0,796 s |
| `utf8-cjk` | Cristalino | 3 455 | 1 086 | 0,899 s | 18,352 s |
| `utf8-emoji` | Vanilla | 2 199 | 1 086 | 0,094 s | 0,633 s |
| `utf8-emoji` | Cristalino | 4 136 | 1 086 | 0,731 s | 6,244 s |

#### Distribuição temporal (CJK)

Vanilla: 100 % das aberturas de fonte concentram-se no arranque (primeiros
~0,5 s do processo).

Cristalino: aberturas espalhadas por ~17 s; a fase de shaping/layout continua a
abrir e reabrir ficheiros de fonte.

### 3.3 Interpretação

- O vanilla **paga o I/O de fontes no arranque**, mas de forma barata (mmap).
  Depois disso, o shaping é I/O-free porque a coverage exacta já responde à
  pergunta "esta fonte cobre este caractere?".
- O cristalino **adianta o I/O para o shaping**. Como a coverage é aproximada,
  cada candidato que sobrevive ao filtro grosseiro tem de ser confirmado
  abrindo a face (`face_covers_char` → `FaceCache::get` → `FontSlot::get` →
  `std::fs::read`). Isso explica a variação de tempo: CJK tem poucos falsos
  positivos (~32 candidatos reportados em P933), emoji tem muitos (~238), e o
  custo por falso positivo é uma leitura do disco.

### 3.4 Sobre a medição P933 (vanilla ~6,5 s para CJK/emoji)

O relatório P933 reportou tempos de ~6,5–8,2 s para `utf8-cjk.typ` e
`utf8-emoji.typ`, usando `target-original/release/typst`. As medições deste
passo, com o binário `lab/typst-original/target/release/typst` (hash
e73e4ac1...), dão ~300 ms.

**P934 já provou a causa desta discrepância:** o executável
`target-original/release/typst` usado em P933 não era o Typst original — era o
**próprio cristalino** num snapshot do estado P927 (SHA-256
`8446552f...`, strings `Typst compiler (crystalline)`, variáveis
`CRYSTALLINE_DOCUMENT_ID`, caminhos temporários
`/tmp/typst-crystalline-da18ea9f3/...`). Os ~6,5–8,2 s eram o cristalino P927, não
o vanilla. O vanilla real (`lab/typst-original/target/release/typst`) é
~300 ms para CJK/emoji, tal como P923 e P934 reportaram.

A "uniformidade" do vanilla real (~270–310 ms independentemente do script) é,
portanto, real. O objectivo deste passo foi entender a estrutura que a produz:
eager mmap + coverage exacta no arranque, shaping I/O-free durante o fallback.

---

## 4. Veredicto sobre a hipótese

> "A coverage exacta do vanilla elimina a verificação repetida de faces durante
> o shaping; o cristalino, com coverage aproximada, é forçado a abrir faces para
> confirmar candidatos."

**Confirmada**, com ressalva: a coverage exacta **sozinha** não explica a
uniformidade. Se o vanilla lesse as fontes com `std::fs::read` em vez de mmap,
o arranque seria tão lento quanto o cristalino (ver P935: ~4 s para 1112
faces). A uniformidade resulta da **combinação**:

```text
mmap barato  ─┐
              ├─>  eager coverage exacta no arranque  ──>  shaping I/O-free
coverage     ─┘
exacta
```

Portar só uma das peças para o cristalino não resolve:
- Só mmap + coverage aproximada: ainda há falsos positivos e aberturas durante
  o shaping.
- Só coverage exacta + `std::fs::read`: o arranque fica proibitivo.

---

## 5. Implicações para o próximo passo

A correção completa precisa de:

1. **Coverage exacta** no `FontBook` (estilo runs do vanilla), substituindo o
   bitmap por bloco de 256 codepoints. Isso elimina os falsos positivos que
   forçam `face_covers_char` durante o fallback.
2. **I/O barato e persistente** para as fontes — mmap via `fontdb` (como o
   vanilla) ou cache de `Font` carregadas. Sem isto, o arranque com coverage
   exacta é lento, e o shaping continua a pagar leituras de disco.
3. (Opcional) **Partilha de bytes entre faces do mesmo `.ttc`** — reduz
   redundância, mas é secundário face a (1) e (2).

A próxima implementação deve incluir ambas (1) e (2) no mesmo passo, porque são
interdependentes.

---

## 6. Proveniência

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Reversão / testes | `git checkout acef3e88 -- ...` + `cargo test --workspace` | working tree revertido para P933-fixed | `shaper.rs` também revertido para testes passarem |
| Benchmarks | `hyperfine --warmup 1 --min-runs 3` | idem | 11 cenários no corpus `tools/perf/corpus/p923/` |
| `strace` | `strace -f -tt -e trace=openat` | idem | Mediu aberturas de ficheiros de fonte; overhead do strace aumenta tempos absolutos, mas a distribuição relativa é válida |
| Código vanilla | leitura directa de `lab/typst-original/crates/typst-library/src/text/font/{info,book}.rs` | binário `lab/typst-original/target/release/typst` | Confirma coverage exacta e `select_fallback` I/O-free |

---

## 7. Validação final

- `cargo test --workspace`: ok.
- `cargo build --workspace --release`: ok.
- `crystalline-lint .`: 0 drift (V7 pré-existente).
- Código revertido para P933-fixed, confirmado por testes e benchmark.
- Nenhuma implementação de correção neste passo — só entendimento.
