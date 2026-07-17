# Paridade Produção — P699 — `plugin()` devolve `Module` real (host ligado à linguagem)

**Data:** 2026-07-11
**Passo:** `00_nucleo/materialization/typst-passo-699.md`
**Hash do commit (implementação):** `8baaadfa4` (detached HEAD; ver §1)
**Estado:** FECHADO — implementação completa, lint 0, workspace verde.

---

## 1. Proveniência das medições (regra P569 / ADR-0108)

Toda a contagem abaixo foi obtida a partir do estado exacto:

- **HEAD base:** `b241f15259bc82237fb345e6826cb62f01d0e7fa` (detached HEAD, fim de P698).
- **Hora da medição final:** `2026-07-11T03:04:12Z` (UTC, `date -u`).
- **Working tree no momento da medição:** apenas os 18 ficheiros de P699 (16 tracked
  modificados + 2 novos) mais este relatório. Lixo untracked (`temp_p*`, passos
  589–698 em `materialization/`, `perf.data`, ADRs auxiliares, `estado-geral-p695.md`,
  `registo-fecho-p583-p584-p585.md`) **não** entrou no commit.
- `git diff HEAD --stat` (ficheiros de código+L0 de P699):

```text
00_nucleo/prompts/contracts/plugin_host.md |   7 +-
00_nucleo/prompts/contracts/world.md       |  10 +
00_nucleo/prompts/entities/func.md         |  26 ++-
00_nucleo/prompts/infra/plugin_host.md     |  46 +++++
00_nucleo/prompts/infra/system-world.md    |   7 +
00_nucleo/prompts/engine/stdlib/plugin.md   | 320 +++++++++++++++++++++--------
01_core/src/contracts/plugin_host.rs       |  19 +-
01_core/src/contracts/world.rs             |  13 +-
01_core/src/entities/func.rs               |  15 +-
01_core/src/entities/mod.rs                |   1 +
01_core/src/engine/eval/closures.rs         | 134 ++++++++++++
01_core/src/engine/stdlib/plugin.rs         | 178 +++++++++++++---
03_infra/src/plugin_host.rs                |  56 ++++-
03_infra/src/world.rs                      |  22 +-
04_wiring/src/main.rs                      |   9 +-
crystalline.toml                           |   2 +-
```

Novos: `00_nucleo/prompts/entities/plugin_func.md`, `01_core/src/entities/plugin_func.rs`.

---

## 2. Objectivo e resultado

Ligar o `PluginHost` (P698) à linguagem Typst. Antes de P699, `plugin(...)`
lia os bytes (P697) e devolvia um erro provisório. Depois de P699:

- `plugin("f.wasm")` / `plugin(bytes)` ⇒ **`module`** (um `Module` real da
  linguagem).
- `module.export(bytes) ⇒ bytes` — uma função por export WASM, chamável.
- `#import plugin("f.wasm"): export` — expõe a função ao escopo (sem tocar no
  import, que já funcionava desde P683).
- Chamadas idênticas batem no **cache** (`#[comemo::memoize]`).

## 3. Decisões (medidas antes de decidir — ADR-0108)

### 3.1 Injeção do host — Opção A (método defaulted em `World`)

Medição: ~14 literais `Engine { ... }` em L1 + mudança de ABI (Opção B, campo
no `Engine`) vs 3 sítios (Opção A). Escolhida **Opção A**:

- `World::plugin_host() -> Option<Arc<dyn PluginHost>>` com **default `None`**
  (`01_core/src/contracts/world.rs`).
- `SystemWorld` (L3) sobrescreve e devolve `Some(...)` quando tem host.
- `04_wiring/src/main.rs` encadeia
  `.with_plugin_host(Arc::new(WasmiPluginHost::new()))`.

MockWorlds e testes de L1 não precisam de implementar (default `None` ⇒ erro
"plugins não suportados neste World").

### 3.2 Localização de `PluginFunc` — `entities/`, não `rules/`

ADR-0109 proíbe import `entities → rules`. `func.rs` referencia
`crate::entities::plugin_func::PluginFunc`; logo `PluginFunc` vive em
`01_core/src/entities/plugin_func.rs`. O braço de despacho fica em
`rules/eval/closures.rs` (`rules → entities`, permitido).

### 3.3 Cache — `#[comemo::memoize]` com `Hash`/`Eq` manuais

`comemo = "0.4"` já expõe `memoize`, mas **nenhum crate a usava antes de P699**
(confirmado por varredura: só `track`/`TrackedMut` em uso). Somos os primeiros.

- `PluginFunc::call(&self, args: Vec<Bytes>) -> Result<Bytes, PluginError>`
  anotado com `#[comemo::memoize]`.
- `PartialEq`/`Eq`/`Hash` **manuais** sobre `(module, name)`; `host`
  (`Arc<dyn PluginHost>`, não-`Hash`) excluído de ambos.
- Output `Result<Bytes, PluginError>` é `Clone + Send + Sync` (requisito do
  comemo); `Bytes: Hash` (`entities/bytes.rs:14`) torna `Vec<Bytes>` chave
  válida.

Confirmado em runtime: `call_cacheia_segunda_chamada_identica` — host chamado
**1** vez em 2 chamadas idênticas.

### 3.4 `PluginHost: Send + Sync` + método `exports`

- `Send + Sync` obrigatório: `World: Send + Sync` e o host viaja em
  `SystemWorld` e dentro de `Value::Func(FuncRepr::Plugin(...))` (precedente:
  `ElementCtor`).
- Novo método `exports(&self, module) -> Result<Vec<EcoString>, PluginError>`
  (réplica de `into_module`, `plugin.rs:366-380`, filtrando `ExternType::Func`).

`WasmiPluginHost: Send + Sync` verificado em runtime (`host_eh_send_e_sync`) —
`wasmi::Linker<CallData>` é `Send` (vanilla partilha `Linker` em
`Arc<PluginBase>` entre threads). O fallback documentado no L0 (guardar só
`module` e reconstruir o `Linker` em `call`) **não** foi necessário.

---

## 4. Ficheiros tocados

### L0 (especificação — Trava Arquitetural cumprida antes do código)

- `00_nucleo/prompts/entities/plugin_func.md` — **novo** (`PluginFunc`,
  memoize, Hash manual).
- `00_nucleo/prompts/contracts/plugin_host.md` — trait `: Send + Sync` +
  `exports`.
- `00_nucleo/prompts/contracts/world.md` — método defaulted `plugin_host()`.
- `00_nucleo/prompts/entities/func.md` — variante `FuncRepr::Plugin` +
  `Func::plugin`.
- `00_nucleo/prompts/engine/stdlib/plugin.md` — reescrita (sai erro provisório
  de P697; entra `Module` real + host via `World`).
- `00_nucleo/prompts/infra/system-world.md` — `fn plugin_host` + builder
  `with_plugin_host`.
- `00_nucleo/prompts/infra/plugin_host.md` — subsecção `exports` + nota
  `Send + Sync` (com plano de fallback).

### Código

- `01_core/src/contracts/plugin_host.rs` — `PluginHost: Send + Sync` + `exports`.
- `01_core/src/contracts/world.rs` — `plugin_host()` defaulted.
- `01_core/src/entities/plugin_func.rs` — **novo** (`PluginFunc` + memoize +
  Hash/Eq manuais + `CountingHost` de teste).
- `01_core/src/entities/mod.rs` — `pub mod plugin_func;`.
- `01_core/src/entities/func.rs` — `FuncRepr::Plugin`, `Func::plugin`,
  `name()`/`native_fn_addr()` ajustados.
- `01_core/src/engine/eval/closures.rs` — braço `FuncRepr::Plugin ⇒ call_plugin`
  + `call_plugin` + 3 testes do braço.
- `01_core/src/engine/stdlib/plugin.rs` — `native_plugin` devolve `Module`;
  testes reescritos.
- `03_infra/src/plugin_host.rs` — `WasmiPluginHost::exports` + 3 testes
  (`exports`, `exports` inexistente, `Send+Sync`).
- `03_infra/src/world.rs` — campo `plugin_host`, builder `with_plugin_host`,
  `World::plugin_host`.
- `04_wiring/src/main.rs` — `.with_plugin_host(...)`.
- `crystalline.toml` — `memoize` adicionado a `[l1_allowed_external.comemo]`
  (primeiro uso de `comemo::memoize` em L1).

---

## 5. Verificação (números com proveniência do §1)

- `crystalline-lint .` → **0 violations** (EXIT=0). (V7 de `plugin_func.md`
  órfão existiu durante a fase de L0 e desapareceu ao materializar
  `entities/plugin_func.rs`.)
- `cargo build` → Finished, EXIT=0; nenhum warning nos ficheiros tocados.
- `cargo test --workspace` → **sem regressão**:
  - L1 (`typst-core`): **3726 passed**, 0 failed (3719 em P698 + 7 de P699:
    3 em `plugin_func`, 2 líquidos em `stdlib::plugin`, 3 em `closures` —
    arredondamento de filtros; total 3726).
  - L3 (`typst-infra`): **626 passed**, 5 ignored (623 em P698 + 3 de P699:
    `exports_devolve_so_funcoes_exportadas`, `exports_modulo_inexistente_erro_defensivo`,
    `host_eh_send_e_sync`).
  - L2: 33 passed; L4: 2 passed; restantes: 27 / 2 / 3 ignored — inalterados.
- Testes P699 específicos (todos verdes):
  - Cache: `entities::plugin_func::tests::call_cacheia_segunda_chamada_identica`
    (host 1× em 2 chamadas).
  - Módulo real: `rules::stdlib::plugin::tests::plugin_devolve_module_com_funcao_por_export`.
  - Sem host: `...::plugin_sem_host_devolve_erro_suporte`.
  - Erro do host verbatim: `...::plugin_host_load_falha_propaga_verbatim`.
  - Braço `call_plugin`: `call_plugin_devolve_bytes`,
    `call_plugin_arg_nao_bytes_erro`, `call_plugin_named_arg_erro`.
  - L3: `exports_devolve_so_funcoes_exportadas`, `host_eh_send_e_sync`.

---

## 6. Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| `plugin(...)` devolve `module` | semântica (linguagem) | sim |
| `module.export(bytes) ⇒ bytes` | semântica (linguagem) | sim |
| `#import plugin("f.wasm"): export` | sintaxe (já existia, P683) | sim |
| Erros de validação de args (`arguments must be bytes`, named args) | mensagem (observável) | sim (verbatim) |
| Erros do host (`PluginError.message`) | mensagem (observável) | sim (verbatim, via catálogo P698) |
| `PluginFunc` em `entities/` (não `rules/`) | mecânica (topologia) | diverge de propósito (ADR-0109) |
| `World::plugin_host()` defaulted (vs campo no `Engine`) | mecânica (injeção) | diverge de propósito (medição §3.1) |
| Cache via `comemo::memoize` (vs pool/vtable) | mecânica (estrutura/perf) | diverge de propósito; equivalente para plugins puros |
| Instância fresca por `call` (P698) | mecânica | inalterada |

---

## 7. Disciplina anti-deriva (ADR-0108)

- **Medição antes da decisão:** contagem de literais `Engine{...}` (§3.1)
  precedeu a escolha da Opção A; varredura de `comemo::memoize` (§3.3)
  precedeu a adoção.
- **Intenção vs comportamento:** a ausência de host é mecânica (L3/L4), mas a
  **mensagem** "plugins não suportados neste World" é observável — tratada
  como paridade de linguagem.
- **Inferência marcada:** `host.exports(id)`/`host.call(...)` com `id`
  inexistente ⇒ "plugin module not found" é defesa de API directa (sem
  equivalente observável no vanilla) — marcada no L0 de `infra/plugin_host.md`.
- **Enquadramento cômodo recusado:** não se assumiu `Linker<CallData>: Send`;
  compilou-se e mediu-se (`host_eh_send_e_sync`) antes de decidir não aplicar
  o fallback.

## 8. Riscos materializados e mitigação

- **Cache `static` do comemo colide entre testes** (chave `(module, name,
  args)`; ids por host recomeçam em 1). Mitigado com **nomes de export únicos
  por teste** (`p699_*`, `cp_*`). Os 13 testes de P698 chamam `host.call`
  directo (não passam pelo `PluginFunc`) → não afectados.
- **`Linker<CallData>: Send`** — confirmado por compilação + teste; fallback
  documentado mas não aplicado.
- **Encoder WASM `#[cfg(test)]`-privado** — testes de L1 usam `CountingHost`/
  `StubHost` (mock); host real exercitado em L3 (`plugin_host.rs`, 16 testes).

## 9. Scope-out (não feito em P699)

- E2E de PDF em `04_wiring/tests/` com `hello.wasm` real: o caminho é coberto
  por (a) host real em L3 (16 testes, incl. `hello_devolve_bytes_hello`), (b)
  `native_plugin`→`Module` em L1, (c) braço `call_plugin` em L1. Um teste de
  PDF exigiria fixture `hello.wasm` partilhada e montagem do pipeline completo
  — valor marginal baixo face ao risco; fica para passo futuro se houver
  regressão observada.
- `Span` real do callsite nos erros do braço: `call_plugin` usa
  `Span::detached()` (consistente com `native_plugin` e demais nativos).
  Melhoria futura não bloqueante.

## 10. Critério de aceitação — cumprido

No nível da **linguagem** (ADR-0107):

- [x] `plugin("f.wasm")` e `plugin(bytes)` devolvem `module`.
- [x] `module.export(bytes)` chama a função e devolve `bytes`.
- [x] `#import plugin("f.wasm"): export` expõe a função (sem tocar no import).
- [x] Args não-bytes e ausência de host produzem os erros especificados.
- [x] Duas chamadas idênticas batem no cache (host chamado uma vez).
- [x] `cargo test --workspace` sem regressão; `crystalline-lint .` zero
      violations.
