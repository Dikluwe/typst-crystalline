# Relatório P929 — reduzir custo absoluto de fallback CJK/emoji

**Precede este passo:** `typst-passo-927-relatorio.md` — a Opção 6 (scan condicional) eliminou
a regressão no caso comum, mas deixou o custo absoluto de ~7 s para documentos que realmente
precisam de fallback de fontes do sistema. No final de P927 foram apontadas duas ideias para
atacar esse custo absoluto: (1) paralelizar o scan lazy; (2) cache em disco da coverage.
**Objetivo deste passo:** medir as duas ideias em protótipos temporários, sem implementar código
de produção, e decidir se alguma compensa.

**Data:** 2026-07-29.
**Commit base:** `da18ea9f3` (P922–P924 integrados).
**Commit working tree:** `da18ea9f3` com alterações não commitadas em
`03_infra/src/world.rs` (estado P927) e `04_wiring/src/main.rs`.

---

## Resumo executivo

Nenhuma das duas ideias é claramente superior ao estado P927:

- **Ideia 1 (scan paralelo):** inviável. No caso comum é ligeiramente mais lenta; quando o scan
  realmente dispara (CJK/emoji), o overhead das threads e a contenção no sistema tornam o tempo
  **2,9×–3,8× pior**.
- **Ideia 2 (cache em disco):** trade-off fraco. Melhora ~8% nos documentos CJK/misto, mas
  **regressa ~21% no documento emoji** e não traz ganho mensurável no caso comum. A 2ª execução
  não é mais rápida que a 1ª, porque o cache já é populado durante o warmup.
- **Decisão:** ficar com o P927 por enquanto. O custo absoluto de ~7 s para documentos reais com
  CJK/emoji fica registado como **limitação temporariamente aceite**, não como resolvido.

---

## Metodologia

- **Binário original:** `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072` (estado P927).
- **Binários protótipo:** `target/release/typst` recompilado para cada ideia; ambos os protótipos
  foram revertidos ao final do passo.
- **Cenários canônicos:** os 7 documentos da frente (P872–P921), comparados `depois/antes`.
- **Cenários UTF-8:** `05-utf8.typ` e 4 inputs de bloco Unicode isolado (`utf8-latin`,
  `utf8-greek`, `utf8-cjk`, `utf8-emoji`).
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 para canônicos; warmup 2 / min-runs 10
  para UTF-8. Para a Ideia 2, o cache em disco foi apagado antes da run 1 e mantido para a run 2.
- **Núcleos disponíveis:** 16 (`nproc`).
- **Atestações:**
  - Ideia 1: `tools/perf/results/p928-canonical/attestation.json` e
    `tools/perf/results/p928-utf8/` (os scripts de P928 foram reutilizados).
  - Ideia 2: `tools/perf/results/p929-ideia2/attestation.json`.

---

## Fase A — Ideia 1: scan paralelo

Implementação temporária: `candidates_for_char` dividia os slots em chunks e processava cada
chunk numa thread separada via `std::thread::scope`. Cada thread construía a sua própria vista dos
bytes do slot (embedded, shared_source ou leitura do disco), extraía a coverage e devolvia os
índices. Os resultados locais eram juntos no final e ordenados.

### Caso comum (7 cenários canônicos)

| Cenário | original (ms) | paralelo (ms) | rácio |
|---|---:|---:|---:|
| 01-hello | 89.64 | 91.81 | **1.02×** |
| 02-lorem | 111.81 | 115.78 | **1.04×** |
| 03-images | 95.37 | 99.91 | **1.05×** |
| 04-math | 151.86 | 152.92 | **1.01×** |
| 05-tables | 93.36 | 96.04 | **1.03×** |
| 06-long | 301.84 | 298.93 | 0.99× |
| 07-context | 131.13 | 135.85 | **1.04×** |

### Casos UTF-8

| Cenário | original (ms) | paralelo (ms) | rácio |
|---|---:|---:|---:|
| utf8-latin | 92.01 | 94.68 | **1.03×** |
| utf8-greek | 91.74 | 93.38 | **1.02×** |
| 05-utf8 | 8028.20 | 30357.41 | **3.78×** |
| utf8-cjk | 7228.64 | 20929.06 | **2.90×** |
| utf8-emoji | 7856.23 | 25599.06 | **3.26×** |

**Conclusão:** o scan paralelo piora em todos os cenários relevantes. O system time explode
(por exemplo, 144 s de system time no `05-utf8` contra ~5,9 s no original), o que indica
contenção massiva — provavelmente locks internos da biblioteca de fontes ou serialização do I/O.
Não vale a pena aprofundar.

---

## Fase B — Ideia 2: cache em disco da coverage

Implementação temporária: `candidates_for_char` procurava primeiro a coverage num ficheiro em
`target/tmp/coverage-cache/<chave>.cov`; em miss, calculava a coverage e guardava-a em disco.
A chave era estável (FNV-1a do path + índice para fontes do sistema; FNV-1a dos bytes + índice
para fontes embutidas). Cada `Coverage` era serializada como 64 × `u64` little-endian (512 bytes).

### Caso comum (7 cenários canônicos)

| Cenário | run 1 (cache vazio) | run 2 (cache cheio) |
|---|---:|---:|
| 01-hello | **1.04×** | **1.02×** |
| 02-lorem | **1.01×** | **1.03×** |
| 03-images | **1.03×** | **1.02×** |
| 04-math | **1.01×** | **1.02×** |
| 05-tables | **1.04×** | **1.03×** |
| 06-long | 0.99× | **1.01×** |
| 07-context | **1.02×** | **1.01×** |

### Casos UTF-8

| Cenário | run 1 (cache vazio) | run 2 (cache cheio) |
|---|---:|---:|
| 05-utf8 | **0.92×** | **0.92×** |
| utf8-latin | **1.03×** | **1.03×** |
| utf8-greek | **1.03×** | **1.03×** |
| utf8-cjk | **0.92×** | **0.92×** |
| utf8-emoji | **1.21×** | **1.21×** |

**Conclusão:**

- Caso comum: sem melhoria; ligeira regressão dentro da banda de ruído.
- CJK/misto: ganho real e consistente de ~8%.
- Emoji: regressão real e consistente de ~21%.
- A 2ª execução não é mais rápida que a 1ª: o warmup da 1ª execução já popula o cache, portanto
  o ganho entre invocações separadas do processo é pequeno.

A Ideia 2 é tecnicamente viável, mas introduz uma regressão num caso (emoji) em troca de um
ganho modesto noutro (CJK). Não é uma melhoria clara no conjunto completo.

---

## Fase C — decisão

Nenhuma das duas ideias supera o critério de aceitação de P929 (melhoria nos casos CJK/emoji
sem regressão no caso comum ou noutros casos UTF-8). Por isso:

- **Ficamos com o P927** como estado final desta ronda.
- O custo absoluto de ~7 s para documentos com CJK/emoji é registado como **limitação
  temporariamente aceite**.
- Futuras abordagens podem requerer olhar para o algoritmo de fallback em si (por exemplo,
  pré-computar a coverage no `FontInfo` durante a descoberta de fontes, como discutido em P926),
  em vez de otimizar quando ou como o scan lazy é feito.

---

## Fase D — estado da árvore

- `03_infra/src/world.rs`: apenas as alterações do P927 (`preload_coverage_if_needed`,
  `embedded_coverage_union`, `source_text_nodes`). Sem cache em disco, sem paralelização.
- `03_infra/src/fonts.rs`: sem alterações (métodos temporários removidos).
- `04_wiring/src/main.rs`: inalterado face ao P927 (chamada a `preload_coverage_if_needed`).
- `target/tmp/coverage-cache`: apagado.
- `cargo build --workspace --release`: ok.
- `crystalline-lint --fix-hashes .`: corrigido o hash do prompt `system-world.md` em
  `03_infra/src/world.rs` de `11e36c9b` para `37c9a8a2`.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`).
- `cargo test -p typst-infra --lib`: 743 passed, 0 failed.

---

## Proveniência

- Commit base: `da18ea9f3`.
- Binário original: `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`.
- Scripts:
  - Ideia 1: `tools/perf/benchmark-p928-canonical.py` e `tools/perf/benchmark-p928-utf8.py`.
  - Ideia 2: `tools/perf/benchmark-p929-ideia2.py`.
- Atestações:
  - Ideia 1: `tools/perf/results/p928-canonical/attestation.json` e
    `tools/perf/results/p928-utf8/`.
  - Ideia 2: `tools/perf/results/p929-ideia2/attestation.json`.
