# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `5ed5988de6f3d56b45002c2e42f3a14ebfd570d9e4e213c737b3b4de4de97a39`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: e5695a34
prompt: infra/plugin_host
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:24-45, 69-89, 215-224`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host.rs`. Fachada e estado do host: manter WasmiPluginHost público no caminho existente, engine, AtomicU64 iniciado em 1 e Mutex<HashMap<PluginModuleId, Arc<PluginEntry>>> privados. PluginEntry mantém a base compartilhada e snapshot opcional. Preservar new, Default, Send + Sync, lookup defensivo e os quatro métodos de PluginHost; seus corpos delegam estaticamente às unidades privadas. Descendentes acedem ao estado por descendência, sem pub(crate) novo. Não alterar o contrato L1 em 00_nucleo/prompts/contracts/plugin_host.md.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.

## Owners descendentes

- `03_infra/src/plugin_host/loading.rs`: `00_nucleo/prompts/infra/plugin_host/loading.md`.
- `03_infra/src/plugin_host/execution.rs`: `00_nucleo/prompts/infra/plugin_host/execution.md`.
- `03_infra/src/plugin_host/typst_env.rs`: `00_nucleo/prompts/infra/plugin_host/typst_env.md`.
- `03_infra/src/plugin_host/snapshot.rs`: `00_nucleo/prompts/infra/plugin_host/snapshot.md`.
- `03_infra/src/plugin_host/tests.rs`: `00_nucleo/prompts/infra/plugin_host/tests.md`.

## `WasmiPluginHost::new() -> Self`

Replica `Plugin::new` parcial (`plugin.rs:268-308`):

1. `let mut config = wasmi::Config::default();`
2. `config.wasm_relaxed_simd(false);` — determinismo (`plugin.rs:271-272`).
3. `let engine = wasmi::Engine::new(&config);` (`plugin.rs:274`).
4. `modules: Mutex::new(HashMap::new())`, `next_id: AtomicU64::new(1)`.

O `Linker` é construído por módulo em `load` (o vanilla constrói um linker por
`PluginBase`, `plugin.rs:283-299`) — `func_wrap` regista as duas funções no
linker daquele módulo.

## `Send + Sync` (P699)

`WasmiPluginHost` implementa `Send + Sync` (obrigatório: `World: Send + Sync`
— `contracts/world.rs:33` — e o host viaja em `SystemWorld` e dentro de
`Value::Func(FuncRepr::Plugin(...))`). O precedente é `ElementCtor`
(derivado `Send + Sync` em L1).

Os campos são todos `Send + Sync`:

- `wasmi::Engine` — `Send + Sync` (partilhado entre threads no vanilla).
- `AtomicU64` — `Send + Sync`.
- `Mutex<HashMap<PluginModuleId, Arc<PluginEntry>>>` — `Send + Sync` com
  base compartilhada e snapshot owned em cada entry.
- `PluginBase { module: wasmi::Module, linker: wasmi::Linker<CallData> }` —
  `wasmi::Module: Send + Sync`; `wasmi::Linker<CallData>: Send` (o vanilla
  guarda `Linker` em `Arc<PluginBase>` e partilha entre threads via
  `Mutex<Vec<PluginInstance>>`, `plugin.rs:245,355-363`). `CallData: Send`
  (só contém `Vec`/`Option`).

**Se o build falhar** por `Linker<CallData>` não ser `Send` (mudança de
versão do `wasmi`): refacto `PluginBase` para guardar só `module: wasmi::Module`
e reconstruir o `Linker` dentro de `call` (`load` não precisa de linker — só a
instanciação em `call`). O custo (reconstruir o linker por chamada) é aceitável
e não muda a semântica. Decisão a tomar **só** se o compilador o exigir
(ADR-0108: medir — compilar — antes de refactar).

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| as 10 mensagens do catálogo | mensagem (mecânica observável) | sim (verbatim, com inferência marcada no `{err}` do item 2) |
| `wasm_relaxed_simd(false)` | semântica (determinismo) | sim |
| exigir `memory`, 2 imports `typst_env`, validação lazy | semântica/protocolo | sim |
| instância fresca por `call` (vs pool) | mecânica (estrutura/perf) | diverge de propósito — equivalente para plugins puros |
| `AtomicU64`+`Mutex<HashMap>` (vs `Mutex<Vec<Instance>>`) | mecânica (estrutura) | diverge de propósito |


## Lookup e fachada

Lookup devolve Arc clonado da entry ou `plugin module not found` (defesa de API direta, não paridade nova). load/exports delegam a loading; call/transition a execution. new desativa relaxed SIMD; Default chama new. Sem novos campos ou métodos públicos.
