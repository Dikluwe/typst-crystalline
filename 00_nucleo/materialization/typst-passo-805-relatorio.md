# Relatório — typst-passo-805 (achado P798 #10): `text::lorem_` — falta ponto final no output de `#lorem(n)`

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-805.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799–P804 (zonas não relacionadas). Validação "depois" com working tree não commitado: P799–P805 + P805a. Hora da validação: 2026-07-21 ~17:20 -0300.
**Binários:** `./target/release/typst` (rebuild 17:20), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Comandos: `./target/release/typst -o n{N}_c.pdf n{N}.typ` / `lab/typst-original/target/release/typst compile n{N}.typ n{N}_v.pdf`, com `n{N}.typ` = `#lorem(N)`; extracção `pdftotext - | tr '\n' ' '`.

| Fonte | Cristalino (antes) | Vanilla |
|---|---|---|
| `#lorem(1)` | `Lorem` | `Lorem.` |
| `#lorem(10)` | `Lorem ipsum dolor sit amet consectetur adipiscing elit sed do` | `Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.` |
| `#lorem(30)` | `Lorem ipsum ... magna aliqua Ut enim ad minim veniam ...` | `Lorem ipsum ... magnam aliquam quaerat voluptatem. Ut enim aeque doleamus animo, ...` |

**Medição que alarga o achado (ADR-0108):** não era só o ponto final — faltavam as vírgulas e o texto divergia totalmente a partir da palavra ~19. O cristalino usava um vocabulário cíclico próprio (Passo 391, scope-out explícito "o texto exacto não precisa de coincidir com o vanilla"); o vanilla gera com **Markov chain** determinística.

## Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-library/src/text/lorem.rs`: `lorem_impl(n)` usa a crate `lipsum` (0.9.1 — `Cargo.lock` do vanilla): `MarkovChain` de ordem 2 com `LOREM_IPSUM` + `LIBER_PRIMUS`, `iter_from(("Lorem", "ipsum"))`, RNG determinístico interno da crate (`ChaCha20Rng::seed_from_u64(97)`, verificado na fonte da crate em `~/.cargo/registry/src/.../lipsum-0.9.1/src/lib.rs:289-291`). A lógica de junção (saltar `--`→en-dash, capitalizar após `.!?`, garantir ponto final) é do próprio typst.

**Cristalino** — `01_core/src/engine/stdlib/text.rs::native_lorem` (antes): `const LOREM_WORDS` (59 palavras) em ciclo `words.join(" ")`, sem pontuação.

## Passo 2 — Implementação

Decisão arquitectural registada: **byte-parity** via a **mesma crate `lipsum` 0.9.1** (revoga o scope-out de Passo 391). Registos:
- `Cargo.toml` workspace + `01_core/Cargo.toml`: dependência `lipsum = "0.9.1"` (MIT; resolve offline — crate em cache).
- `crystalline.toml`: nova entrada `[l1_allowed_external.lipsum]` (`MarkovChain`, `LOREM_IPSUM`, `LIBER_PRIMUS`, `new`, `learn`, `iter_from`) — precedente: P756 (`icu_segmenter`), P785a (`syntect`/`two-face`).
- L0 `00_nucleo/prompts/engine/stdlib/text.md`: secção `lorem(n)` reescrita (P805), incluindo nota de performance: a cadeia é construída **por chamada** (L1 proíbe estado global; o vanilla usa `LazyLock`).

`native_lorem` passa a delegar num `lorem_impl` portado 1:1 do vanilla (mesma cadeia, mesma lógica de junção). Validação de argumentos (n<0, não-int, named args) inalterada.

## Passo 3 — Validação (depois)

### 3.2 — Saída literal (PDF, normalizada)

| Fonte | Resultado |
|---|---|
| `#lorem(1)` | IDÊNTICO ao vanilla ✓ |
| `#lorem(10)` | IDÊNTICO ✓ |
| `#lorem(30)` | IDÊNTICO ✓ (depois de P805a — ver abaixo) |
| `#lorem(100)` | IDÊNTICO ✓ (idem) |

**Nota de processo obrigatória:** a primeira validação de `#lorem(30)`/`#lorem(100)` **divergiu** — apenas em palavras com "fi" ("fieri"→"eri", "infinitum"→"in nitum"). A investigação mostrou que a string `lorem` já era byte-idêntica (teste unitário) e que a divergência era um **bug separado e pré-existente**: ligaduras fi/ffi sem entrada ToUnicode no caminho de embed integral CFF (renderizavam, mas a extracção perdia-os — afecta **qualquer** documento com "fi"). Corrigido no sub-passo **P805a** (relatório próprio: `typst-passo-805a-relatorio.md`). As linhas ✓ acima são posteriores a P805a.

### 3.3 — Testes

- Novo: `lorem_p805_byte_parity_vanilla` (`01_core/src/engine/stdlib/mod.rs`) — strings exactas do vanilla para n=1, 10, 30 (escrito primeiro; falhou antes em `stdlib/mod.rs:11502`).
- Existentes P391 (contagem de palavras para n=0/1/5/100, erros) — inalterados e a passar.
- Novo (P805a): `p805a_ligatura_fi_tem_entrada_to_unicode_no_embed_integral` (`03_infra/src/integration_tests.rs`).

### 3.4 — Suítes

- `cargo test -p typst-core --lib`: ANTES (fim de P804) **4328** passed + 1 ignored (total 4329); DEPOIS **4329** passed; 0 failed; 1 ignored (total 4330 = +1 teste novo ✓).
- `cargo test -p typst-infra`: **657** passed; 0 failed; 5 ignored (total 662 = +1 teste de P805a ✓; antes 656+5).

Lint: `crystalline-lint .` → exit 0, zero violações (hashes corrigidos: `stdlib/text.rs` → `aef1212c`, `export/builder.rs` → `5b1854d7`).
