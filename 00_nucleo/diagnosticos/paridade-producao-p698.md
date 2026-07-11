# Paridade produção — P698 — host `wasmi` em L3 atrás de `PluginHost` (nível 3 de P696)

**Commit deste passo:** `(preenchido no commit seguinte)` (ver §Proveniência).
**Passo:** `00_nucleo/materialization/typst-passo-698.md`. **Tamanho:** M.
**ADRs:** ADR-0107 (paridade com a linguagem — mensagens são mecânica
observável), ADR-0108 (medir antes de decidir), ADR-0109 (feature no seu
ficheiro), ADR-0111 (fronteira L1↔L3; I/O/runtime em L3).
**Dependências:** P696 (protocolo mapeado com `file:line`, `hello.wasm`
reproduzível), P697 (leitura de bytes já funcional).

---

## Proveniência das medições (regra de proveniência)

- **Estado medido:** working tree **não commitado** sobre
  `HEAD = a1df19bae0c1658ab779f1e6dc191a3d80dd150b` (P697 "preenche hash").
- **Hora:** `2026-07-10T22:50:51-03:00` (ambiente), na mesma janela do
  `cargo test --workspace` e do `crystalline-lint .` (sem edições entre eles).
- **Ficheiros alterados no momento da medição** (`git diff HEAD --stat`):

```
 01_core/src/contracts/mod.rs |   1 +
 03_infra/Cargo.toml          |   1 +
 03_infra/src/lib.rs          |   1 +
 Cargo.lock                   | 122 +++++++++++++++++++++++++++++++++++++++++++
 4 files changed, 125 insertions(+)
```

Novos ficheiros (não no diff acima — untracked no momento da medição):
`00_nucleo/prompts/contracts/plugin_host.md`,
`00_nucleo/prompts/infra/plugin_host.md`,
`01_core/src/contracts/plugin_host.rs`,
`03_infra/src/plugin_host.rs`, este relatório. O passo
`00_nucleo/materialization/typst-passo-698.md` segue a convenção dos passos
589–697 e permanece untracked (não entra no commit).

## Objectivo

Segundo passo da divisão de P696: implementar o **runtime WASM** (nível 3) em
L3, atrás de uma trait `PluginHost` em L1, com o protocolo `typst_env` completo
confirmado por P696 (`wasm_minimal_protocol_write_args_to_buffer`,
`wasm_minimal_protocol_send_result_to_host`), validação de assinatura, códigos
de retorno 0/1/erro e as mensagens exactas do vanilla. **Âmbito deliberado:**
só o host, testado directamente em Rust; `plugin()` devolver `Module` real
(`PluginFunc`, `p.f(bytes)`) fica para **P699**.

## Decisões (com medição — ADR-0108)

1. **Trait em L1, impl em L3** (P696 §6, ADR-0111): `PluginHost`
   (`load`/`call`) + `PluginModuleId(u64)` + `PluginError { message: EcoString }`
   em `01_core/src/contracts/plugin_host.rs`; `WasmiPluginHost` em
   `03_infra/src/plugin_host.rs`. `wasmi::*` **nunca** atravessa a fronteira
   (L0 `contracts/plugin_host.md`); a `message` é contrato (observável,
   ADR-0107) — L3 preenche verbatim, L1/L2 propagam.
2. **`wasmi = "=1.0.9"` com `features = ["simd"]`** — medido, não assumido:
   com `wasmi = "1.0.9"` (caret) o cargo resolveu `1.1.0` e a build falhou com
   `E0599: no method named wasm_relaxed_simd`. Verificação directa nos fontes
   em cache (`~/.cargo/registry/src/.../wasmi-1.0.9/src/engine/config.rs:303-304`)
   confirmou que o método existe mas é `#[cfg(feature = "simd")]`; o `default`
   de `wasmi` é só `["std","wat"]`. Fixar `=1.0.9` (versão exacta do vanilla,
   `Cargo.lock:3938-3943`) + `simd` reproduz `config.wasm_relaxed_simd(false)`
   (`plugin.rs:271-272`) — determinismo idêntico ao vanilla.
3. **Instância fresca por `call`** (sem pool): cada `call` cria
   `Store<CallData>` + `linker.instantiate_and_start` (`plugin.rs:433-437`),
   devolvendo a memória inicial do módulo. Diverge do pool
   `Mutex<Vec<PluginInstance>>` do vanilla (`plugin.rs:245,355-363`) — que é
   optimização de concorrência. Para plugins **puros** (único caso suportado,
   P696 §4) é observacionalmente equivalente; pool/Send+Sync ficam para P699
   (marcado como mecânica, não paridade de linguagem).
4. **Ramos defensivos marcados como inferência** (ADR-0108): `call` com `id`
   inexistente → `"plugin module not found"`; `func_name` não exportada →
   `"plugin function \`{f}\` not found"`. O vanilla faz `.unwrap()`
   (`plugin.rs:448-453`) porque `into_module` só liga exports — estes ramos
   **não** são atingíveis pela linguagem; existem só para a API Rust directa de
   P698. O que os refutaria: um teste vanilla que produza essas mensagens (não
   existe).

## Protocolo replicado (file:line do vanilla)

Fonte: `lab/typst-original/crates/typst-library/src/foundations/plugin.rs`
(lido directamente). `WasmiPluginHost::load` replica `plugin.rs:275-299`
(`Module::new`, exigência de `memory`, os 2 `func_wrap` sob `"typst_env"`);
`call` replica `plugin.rs:447-520` (validação lazy, lengths como `i32`, trap,
bounds, códigos 0/1/outro); os host funcs replicam `plugin.rs:577-612`.

## Catálogo de mensagens — testado uma a uma (observável, ADR-0107)

| # | Mensagem exacta | Teste | `file:line` |
|---|-----------------|-------|-------------|
| 1 | `plugin does not export its memory` | `msg_01_modulo_sem_memory` | `plugin.rs:280` |
| 2 | `failed to load WebAssembly module ({err})` | `msg_02_wasm_invalido` (prefixo) | `plugin.rs:276` |
| 3 | `plugin function \`f\` has a parameter that is not a 32-bit integer` | `msg_03_param_nao_i32` | `plugin.rs:460-462` |
| 4 | `plugin function \`f\` does not return exactly one 32-bit integer` | `msg_04_resultado_nao_um_i32` | `plugin.rs:465` |
| 5 | `plugin function takes {n} argument{s}, but {m} {was/were} given` | `msg_05_arg_count_singular`/`_plural` | `plugin.rs:472-476` |
| 6 | `plugin panicked: {err}` | `msg_06_trap` (prefixo) | `plugin.rs:492` |
| 7 | `plugin tried to {write\|read} out of bounds: pointer 0x100000 is out of bounds for {write\|read} of length 5` | `msg_07_out_of_bounds_write`/`_read` | `plugin.rs:497-500` |
| 8 | `plugin errored with: boom` | `msg_08_erro_utf8` | `plugin.rs:511` |
| 9 | `plugin errored, but did not return a valid error message` | `msg_09_erro_nao_utf8` | `plugin.rs:513` |
| 10 | `plugin did not respect the protocol` | `msg_10_codigo_retorno_invalido` | `plugin.rs:516` |

Item 2 afirma só o **prefixo** `failed to load WebAssembly module (` — o `{err}`
interno depende do `wasmi` (igual ao vanilla, que usa o mesmo 1.0.9), mas não se
afirma igualdade byte-a-byte do `{err}` (inferência marcada no L0).

## `hello.wasm` end-to-end

Módulo `hello` (memória 1 página, segmento `"hello"` no offset 0,
`hello() -> i32` que chama `send_result_to_host(0,5)` e devolve 0) gerado por um
**encoder WASM próprio em Rust** nos testes (port de `gen_wasm.py` de P696 §A) —
auto-contido e determinista, sem toolchain WASM/Python. Resultado:
`host.load(&bytes)` → `Ok(id)`; `host.call(id, "hello", &[])` →
`Ok(Bytes::new(b"hello".to_vec()))`. ✅ (mesma evidência executável de P696 §2.)

## Honestidade — bug de teste apanhado na primeira corrida

A primeira corrida de `cargo test -p typst-infra plugin_host` deu
**3 passed / 10 failed**, todos os 10 com `"plugin module not found"`. Causa:
**bug de teste** (não de implementação) — cada teste chamava o helper `host()`
**duas** vezes (`let id = host().load(...)` e depois `host().call(id, ...)`),
logo o `id` ficava num host e a chamada noutro (mapa vazio). Os 3 que passavam
(`hello`, `msg_01`, `msg_02`) só instanciavam o host uma vez. Correcção: cada
teste passou a bindar `let h = host();` e reutilizar `h` em `load`+`call`.
Resultado seguinte: **13 passed / 0 failed**. Registado por transparência
(ADR-0108: desconfiar do enquadramento cômodo — o erro inicial parecia de
protocolo mas era de arnês de teste).

## Testes e lint

- 13 testes novos em `03_infra/src/plugin_host.rs` (hello + 10 mensagens + 2
  auxiliares de contagem de args) e 3 em `01_core/src/contracts/plugin_host.rs`
  (preservação de mensagem, `PluginModuleId` Copy/Hash, object-safety de
  `dyn PluginHost`).
- `cargo test -p typst-infra plugin_host`: **13 passed / 0 failed**.
- `cargo test --workspace`: **sem regressão** — `3719 + 623 + 33 + 2 + 27 + 2`
  (core 3716→3719, +3; infra 610→623, +13, com 5 ignored inalterados; restantes
  iguais). Delta total +16 = os 16 testes novos.
- `crystalline-lint .`: **0 violations** (após `--fix-hashes`; hashes
  `contracts/plugin_host.md = fd56183e`, `infra/plugin_host.md = 386e15d7`; L3
  importa `wasmi` livremente — V3/V4/V14 não se aplicam a L3; L1 não importa
  `wasmi` nem faz I/O).

## Língua vs mecânica (ADR-0107)

Paridade (linguagem): as 10 mensagens do catálogo (verbatim, com inferência
marcada no `{err}` do item 2); `wasm_relaxed_simd(false)`; exigência de
`memory`; os 2 imports `typst_env`; validação lazy de assinatura; códigos
0/1/outro. Diverge por agora (mecânica/estrutura): trait em L1 + impl em L3 em
vez de `wasmi` no núcleo; `PluginModuleId(u64)` opaco em vez de `Arc<Plugin>`;
instância fresca por `call` em vez de pool multi-thread — tudo mecânica que
diverge de propósito (P329), sem efeito no observável da linguagem para plugins
puros.

## Critério de fecho do passo

- [x] `wasmi` adicionado como dependência de L3 (`=1.0.9` + `simd`).
- [x] `PluginHost` (trait em L1, impl em L3) construído e testado isoladamente.
- [x] `hello.wasm` chamado com sucesso através de `PluginHost` (→ `b"hello"`).
- [x] Todas as mensagens de erro confirmadas por P696 replicadas e testadas.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p698.md`, com hash
      do commit (preenchido no commit seguinte, per a convenção de P696/P697).

## Próximo passo

P699 (níveis 4–5 de P696): `native_plugin` passa a devolver um `Module` real —
itera os exports `Func` (`into_module`), cria `PluginFunc` por export,
`p.funcao(*bytes) -> bytes`, com `#[comemo::memoize]` em `load`/`call` e
injectão de `Arc<dyn PluginHost>` no eval (é aí que o trait ganha `Send + Sync`,
se necessário). E2E: `#import plugin("hello.wasm"): hello`.
