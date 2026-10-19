# Relatório — typst-passo-819: `foundations::plugin_` — `plugin.transition`, mensagens L1, spans (achado #6 de P810)

**Data**: 2026-07-22
**Estado**: fechado (implementação + validação completas)

---

## Proveniência (regra de 2026-07-05)

- **Commit**: `2acc14eac` (HEAD) + **working tree não commitado** em todas as
  medições. `git diff HEAD --stat` no momento da validação final:
  **80 ficheiros alterados, +6390/−658** (inclui o estado dirty pré-existente
  — 69 ficheiros/+5074/−430 antes de P819 — mais as alterações de P819).
- **Binários**: cristalino `./target/release/typst` (rebuild `cargo build
  --release` de 2026-07-22, pós-implementação); vanilla
  `lab/typst-original/target/release/typst` (0.15.0, build 2026-06-29).
- **Comandos**: vanilla `typst compile <t>.typ out.pdf`; cristalino
  `typst <t>.typ out.pdf` (a CLI cristalina **não tem** subcomando `compile`).
- **Fixtures**: `temp/p810/plugin/*` (P810); `temp/p819/` — `tm*.typ`,
  `mut.wat`/`mut.wasm` (345 B, contador em memória, exports `add`/`get`),
  `mut-ascii.wat`/`mut-ascii.wasm` (274 B, contador inicial `'a'` para
  observável ASCII), `tm13-semantics-noctor.typ`. A scratch tool
  `wat2wasm` usada para compilar os `.wat` foi **removida** (provocava V8/V1
  no lint — ficheiro `.rs` fora da topologia).

## Medição ANTES (sonda — registada também no L0 `prompts/engine/stdlib/plugin.md` §P819)

| Caso | vanilla | cristalino (ANTES) |
|------|---------|--------------------|
| t10 `plugin.transition(p.hello)` | exit 0 | `error: cannot access fields on type function` @ 2:9 |
| t2 ficheiro inexistente | `error: file not found (searched at /…/nao-existe.wasm)` @ 1:8 | `plugin(): não foi possível ler 'nao-existe.wasm': erro ao ler '…': No such file or directory (os error 2)` @ `<detached>` |
| t3 parse WASM | `failed to load WebAssembly module (magic header not detected: bad magic number - …)` @ 1:8 | `failed to load WebAssembly module (expected `(` --> <anon>:1:1 …)` (parser WAT) @ `<detached>` |
| t5 tipo do arg | `error: expected path, string, or bytes, found integer` @ 1:8 | `plugin() requer caminho (str) ou bytes, recebeu int` @ `<detached>` |
| t6 arg não-bytes | `error: expected bytes, found integer` @ 2:9 | `plugin function arguments must be bytes, found int` @ `<detached>` |
| t4/t7/t8/t9 (msgs já verbatim) | span real | mesmas mensagens, `<detached>` |

**Causa raiz do caso t3 (medida, não inferida)**: mesmo wasmi 1.0.9, features
diferentes — vanilla `default-features = false, features = ["simd"]`
(`lab/typst-original/Cargo.toml.original:149`) vs cristalino sem
`default-features = false` (`03_infra/Cargo.toml:39`), que activava a
feature default `wat`. Isto **refutou** a inferência do item 2 do catálogo
de `prompts/infra/plugin_host.md` ("o texto tende a coincidir").

## Código identificado

**Vanilla** (`lab/typst-original/crates/typst-library/src/`):

- `plugin.transition`: `foundations/plugin.rs:158-202` (`#[scope] impl plugin`),
  `:226-231` (`PluginFunc::transition` memoizada), `:328-352`
  (`Plugin::transition` — fingerprint hash128, snapshot, move de instância),
  `:420-426` + `:522-545` (`Snapshot`/`snapshot`/`restore` — só memória
  linear; globals fora, `:177-182`).
- `file not found (searched at …)`: `diag.rs:647-660` (`Display` de
  `FileError`), via `loading/mod.rs:86-105` (`DataSource::load`).

**Cristalino** (pontos da divergência, ANTES):

- `plugin` registado sem namespace: `01_core/src/engine/eval/mod.rs:1559`;
  erro de field access: `01_core/src/engine/eval/bindings.rs:1701-1704`.
- Spans detached: `01_core/src/engine/stdlib/plugin.rs:25-27` (helper
  `err()`), `01_core/src/engine/eval/closures.rs:177,188,201`.
- Mensagens PT: `plugin.rs:54-56,60-64`; `closures.rs:186-195`.
- Formato de leitura L3: `03_infra/src/world.rs:478`.
- Feature `wat`: `03_infra/Cargo.toml:39`.

## Diff / resumo da implementação

1. **`03_infra/Cargo.toml`** — wasmi `=1.0.9` com
   `default-features = false, features = ["simd"]` (alinha com o vanilla;
   verificado antes: nada em L3 usa a feature `wat`).
2. **`03_infra/src/world.rs`** (`read_bytes`) — formato do `Display` de
   `FileError` do vanilla: `file not found (searched at {path-resolvido})`
   para `ErrorKind::NotFound`, `failed to load file (access denied)` para
   `PermissionDenied`, `failed to load file ({e})` nos restantes (à la
   `FileError::from_io`, `diag.rs:631-644`).
3. **`03_infra/src/plugin_host.rs`** — `PluginEntry { base, snapshot }` +
   `Snapshot { mem_pages, mem_data }`; corpo de `call` extraído para a fn
   livre `run_call` (partilhado por `call` e `transition`); `call` restaura
   o snapshot da entry (se houver) na instância fresca; `transition`
   executa a chamada mutável, faz snapshot e regista o derivado com id
   fresco (sem pool/fingerprint — mecânica divergente de propósito, ver L0).
4. **`01_core/src/contracts/plugin_host.rs`** — trait ganha
   `transition(module, func_name, args) -> Result<PluginModuleId, PluginError>`.
5. **`01_core/src/entities/plugin_func.rs`** —
   `PluginFunc::transition(args) -> Result<Module, PluginError>` com
   `#[comemo::memoize]` (chave `(module, name, args)`, `host` excluído via
   `Hash` manual — como `call`); constrói o `Module` derivado com um
   `PluginFunc` por export sobre o id derivado.
6. **`01_core/src/engine/stdlib/plugin.rs`** — `native_plugin_transition`
   (mensagens verbatim tm2–tm6); `native_plugin`: leitura propaga verbatim
   de L3 (sem prefixo PT), tipo do arg ⇒ `expected path, string, or bytes,
   found <tipo-longo>`; helper `err(span, msg)` — todos os erros com
   `args.span` (P772s).
7. **`01_core/src/engine/eval/closures.rs`** — `call_plugin`: arg não-bytes
   ⇒ `expected bytes, found <tipo-longo>`; spans `args.span`.
8. **`01_core/src/engine/eval/mod.rs`** — `plugin` registado via
   `Func::native_with_namespace` com `transition` no namespace (padrão
   P493/`table.header`, como `cbor.encode`); `stdlib/mod.rs` re-exporta
   `native_plugin_transition`.

`parse_anchored` (P814) **não** foi usado — não se aplica (sintetiza spans
de strings parseadas; os erros de plugin têm o span no callsite via
`Args::span`). Registado no L0.

## Medição DEPOIS (saída literal, ambos os binários, 2026-07-22)

**t2** — vanilla:
`error: file not found (searched at /home/…/temp/p810/plugin/nao-existe.wasm)` @ `1:8`
cristalino:
`t2-notfound.typ:1:7: error: file not found (searched at /home/…/temp/p810/plugin/nao-existe.wasm)`
→ **mensagem byte-idêntica** (path absoluto resolvido incluído); span 1:7
(lista de args) vs 1:8 (literal string) — nuance registada.

**t3** — cristalino:
`t3-garbage.typ:1:7: error: failed to load WebAssembly module (magic header not detected: bad magic number - expected=[0x0,0x61,0x73,0x6d,] actual=[0x69,0x73,0x74,0x6f,] (at offset 0x0))`
→ **byte-idêntico** ao vanilla (inclui o `{err}` interno do wasmi) após
`default-features = false`. Item 2 do catálogo: paridade confirmada.

**t5** — cristalino: `…:1:7: error: expected path, string, or bytes, found integer`
→ byte-idêntico ao vanilla (`1:8`; nuance do span).

**t6** — cristalino: `…:2:8: error: expected bytes, found integer`
→ byte-idêntico ao vanilla (`2:9`; nuance do span).

**t4/t7/t8/t9** — mensagens (já verbatim) agora com span real, sem
`<detached>`: `1:7`, `2:8`, `2:14`, `2:13` respectivamente.

**t10** (`plugin.transition(p.hello)`) — vanilla exit 0; cristalino **exit 0**
(antes: `cannot access fields on type function`).

**tm3–tm6** (erros de `plugin.transition`) — cristalino, byte-idêntico ao
vanilla:
`error: missing argument: func` / `error: expected function, found integer`
/ `error: expected plugin function` / `error: expected bytes, found integer`
(e tm2: `expected bytes, found string`). Spans reais (lista de args; nuance
registada face ao span do argumento/chamada do vanilla).

**Semântica de transition a nível de documento** — `tm13-semantics-noctor.typ`
(com `mut-ascii.wasm`; sem construtor `bytes()` — o arg de `add` é
`base.get()`):

```typst
#let base = plugin("mut-ascii.wasm")
#let m1 = plugin.transition(base.add, base.get())
#let m2 = plugin.transition(m1.add, base.get())
#str(base.get())#str(m1.get())#str(m2.get())
```

- vanilla: exit 0, `pdftotext` → **`abc`**
- cristalino: exit 0, `pdftotext` → **`abc`**

(base inalterado `"a"`; derivados observam a mutação acumulada `"b"`, `"c"`
— incl. transições encadeadas.) Os docs tm1/tm9/tm11/tm12 da sonda usam
`bytes("x")` e não compilam no cristalino (construtor `bytes()` ausente —
scope-out transversal de P810, inalterado); tm13 é o equivalente sem
construtor, corrido **nos dois** binários.

**`type(plugin.transition)`** = `function` (registo via namespace P493);
field desconhecido em `plugin` ⇒ `function does not contain field "…"`.

## Testes novos (13) e contagens

Novos testes (todos escritos/actualizados neste passo):

- **L1 `entities/plugin_func.rs`** (+3): caminho feliz do derivado, cache
  (host chamado 1 vez), erro do host verbatim.
- **L1 `engine/stdlib/plugin.rs`** (+4): caminho feliz
  `plugin.transition`, cache, 4 erros verbatim (tm3–tm6), spans =
  `args.span` (não detached) em `plugin()` e `plugin.transition`.
  Actualizados: `plugin_caminho_inexistente_erro_de_leitura` (verbatim de
  L3), `plugin_tipo_errado_…` (nova mensagem).
- **L1 `engine/eval/closures.rs`** (actualizado): `call_plugin_arg_nao_bytes_erro`
  ⇒ `expected bytes, found integer`.
- **L3 `03_infra/src/plugin_host.rs`** (+4): derivado observa / base
  inalterado; encadeamento acumula; erro da chamada aborta sem derivado;
  id inexistente defensivo.
- **L4 `04_wiring/tests/cli.rs`** (+2): e2e com host wasmi real —
  `plugin.transition` caminho feliz (módulo mutável embebido de 274 B); 8
  casos de erro verbatim + ausência de `<detached>` (incl. `magic header
  not detected` end-to-end, provando o alinhamento da feature `wat`).

Contagens (comando `cargo test -p <crate>`, mesmo estado de working tree):

| Suíte | ANTES | DEPOIS |
|-------|-------|--------|
| `typst-core` | 4487 passed; 0 failed; 2 ignored | **4494 passed; 0 failed; 2 ignored** (+7) |
| `typst-infra` | 668 passed; 0 failed; 5 ignored | **672 passed; 0 failed; 5 ignored** (+4) |
| `typst-wiring` (tests/cli.rs) | 29 passed | **31 passed; 0 failed** (+2) |
| `typst-shell` | 33 | 33 (inalterado) |

`cargo test --workspace` verde; `cargo build --release` verde;
`crystalline-lint .` → **0 violações** (6 warnings V7 de prompts órfãos
**pré-existentes**, sem relação com P819; a scratch tool `wat2wasm` foi
removida por provocar V8/V1).

## Nuance registada (desvio aceite — L0 §P819)

O vanilla aponta o span ao **argumento ofensor** (t2/t5/t6, tm2/tm4–tm6) ou
à **chamada inteira incluindo o callee** (t7/t8/t9, tm3/tm11); o cristalino
aponta à **lista de argumentos** (`Args::span`, P772s — span por chamada,
não por argumento). Coluna inicial adjacente (ex.: 1:7 vs 1:8). Mesma
classe do nuance registado em P814.

## Scope-outs (registados no L0, inalterados por P819)

- **Routing de named args** (t13/tm7): vanilla `the argument `source` is
  positional` + hint vs PT cristalino — transversal a todas as nativas.
- **Field inexistente em módulo** (t12): `module 'plugin' does not contain
  field "nao_existe"` vs `module does not contain `nao_existe`` —
  transversal (`Module` nomeado vs anónimo).
- **Prefixos PT dos outros consumidores de `read_bytes`** (`read()`,
  `image()`, `tiling()`, bibliografia): o formato L3 mudou
  (`file not found (searched at …)`), mas esses consumidores mantêm o
  prefixo `fname(): não foi possível ler '…':` — alinhamento é passo
  dedicado (cruza achados #10/#11 de P810).
- **Span por argumento** (ver nuance acima).
- **Construtor `bytes()` / `read(encoding:)`** ausentes — `plugin(bytes)` e
  os docs tm1/tm9/tm11/tm12 só são exercitáveis em testes Rust ou com o
  truque `base.get()` (tm13).
- **Pool multi-instância e fingerprint u128** do vanilla: mecânica;
  substituídos por instância fresca + id por derivação + memoização L1
  (registado no L0 de infra, ADR-0107).

## Ficheiros alterados (P819)

- `03_infra/Cargo.toml` — features do wasmi.
- `03_infra/src/world.rs` — formato do erro de leitura.
- `03_infra/src/plugin_host.rs` — `PluginEntry`/`Snapshot`/`run_call`/
  `transition`/restore + 4 testes.
- `01_core/src/contracts/plugin_host.rs` — `transition` no trait.
- `01_core/src/entities/plugin_func.rs` — `PluginFunc::transition` + 3
  testes.
- `01_core/src/engine/stdlib/plugin.rs` — `native_plugin_transition`,
  mensagens, spans, testes.
- `01_core/src/engine/stdlib/mod.rs` — re-export.
- `01_core/src/engine/eval/closures.rs` — `call_plugin` (mensagem/span) +
  StubHost.
- `01_core/src/engine/eval/mod.rs` — registo com namespace.
- `04_wiring/tests/cli.rs` — 2 testes e2e.
- L0s (fase anterior, hashes fixados pelo humano):
  `00_nucleo/prompts/engine/stdlib/plugin.md`,
  `00_nucleo/prompts/contracts/plugin_host.md`,
  `00_nucleo/prompts/entities/plugin_func.md`,
  `00_nucleo/prompts/infra/plugin_host.md`.
- Fixtures: `temp/p819/` (`.wat`, `.wasm`, `tm*.typ`).
