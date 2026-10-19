# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host/execution.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `683ffc3ce05f59b3d7628e4f790afa31c8ef7446a76f2dd48d29dc7d496f7e36`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 8cd6ac64
prompt: infra/plugin_host/execution
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host/execution

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:146-212, 226-303`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host/execution.rs`. Possuir call, transition e run_call: instância/Store frescos por chamada, restauração do snapshot existente antes da execução, mesma validação lazy de assinatura/aridade e mesma precedência de trap, erro de bounds e retorno do protocolo. Transition descarta output, captura memória só após sucesso e regista id derivado compartilhando a base; erro não cria derivado e o original permanece intacto. Sem pool, memoização nova, alteração de mensagens ou reconstrução de linker.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.

## `call(&self, id, func_name, args) -> Result<Bytes, PluginError>`

Replica `PluginInstance::call` (`plugin.rs:447-520`) sobre uma instância fresca:

1. Lookup do `Arc<PluginEntry>` por `id`; se ausente →
   `PluginError::new("plugin module not found")`. **Inferência marcada**: este
   ramo **não** tem equivalente observável no vanilla — lá `into_module` só liga
   funções exportadas (`plugin.rs:366-380`), logo o utilizador nunca chama um
   `id` inexistente. É defesa de API directa (testes em Rust), **não** paridade
   de linguagem. O que a refutaria: um teste vanilla que produza esta mensagem
   (não existe).
2. Criar `store = Store::new(base.linker.engine(), CallData::default())` e
   `instance = base.linker.instantiate_and_start(&mut store, &base.module)`
   (`plugin.rs:433-437`); em erro de instantiate →
   `PluginError::new(format!("{err}"))` (`plugin.rs:436-437`, `eco_format!("{e}")`).
3. Obter a função: `instance.get_export(&store, func_name).and_then(|e| e.into_func())`.
   Se `func_name` não existe ou não é função →
   `PluginError::new(format!("plugin function `{func_name}` not found"))`.
   **Inferência marcada** (mesmo motivo do ponto 1): vanilla faz `.unwrap()`
   (`plugin.rs:448-453`) porque o nome vem de `into_module`; aqui é defesa de
   API directa. **Não** é paridade de linguagem.
4. **Validação de assinatura (lazy)** sobre `ty = func.ty(&store)`
   (`plugin.rs:454-477`):
   - se `ty.params().iter().any(|&v| v != wasmi::ValType::I32)` →
     `PluginError::new(format!("plugin function `{func_name}` has a parameter that is not a 32-bit integer"))`
     (`plugin.rs:459-463`);
   - se `ty.results() != [wasmi::ValType::I32]` →
     `PluginError::new(format!("plugin function `{func_name}` does not return exactly one 32-bit integer"))`
     (`plugin.rs:464-466`);
   - `expected = ty.params().len()`, `given = args.len()`; se diferem →
     `format!("plugin function takes {expected} argument{s}, but {given} {verb} given")`
     com `s = "" if expected==1 else "s"` e `verb = "was" if given==1 else "were"`
     (`plugin.rs:468-477`).
5. **Lengths como `i32`**: `lengths = args.iter().map(|a| wasmi::Val::I32(a.len() as i32))`
   (`plugin.rs:479-483`). Guardar `store.data_mut().args = args` (como
   `Vec<Vec<u8>>`) (`plugin.rs:485-486`).
6. **Chamada**: `code = Val::I32(-1)`;
   `func.call(&mut store, &lengths, slice::from_mut(&mut code))`. Em trap →
   `PluginError::new(format!("plugin panicked: {err}"))` (`plugin.rs:489-492`).
7. **Out-of-bounds**: se `store.data_mut().memory_error.take()` é
   `Some(MemoryError { offset, length, write })` →
   `format!("plugin tried to {kind} out of bounds: pointer {offset:#x} is out of bounds for {kind} of length {length}")`,
   `kind = "write" if write else "read"` (`plugin.rs:494-502`).
8. **Output**: `output = mem::take(&mut store.data_mut().output)` (`plugin.rs:504-505`).
9. **Código de retorno** (`plugin.rs:508-519`):
   - `Val::I32(0)` → `Ok(Bytes::new(output))`;
   - `Val::I32(1)` → se `str::from_utf8(&output)` é `Ok(message)`:
     `PluginError::new(format!("plugin errored with: {message}"))`; senão:
     `PluginError::new("plugin errored, but did not return a valid error message")`
     (`plugin.rs:510-515`);
   - qualquer outro valor → `PluginError::new("plugin did not respect the protocol")`
     (`plugin.rs:516`).

## `transition` (P819) — snapshot/restore sobre instância fresca

Implementa `PluginHost::transition` (contrato em
`00_nucleo/prompts/contracts/plugin_host.md`; semântica de linguagem em
`00_nucleo/prompts/compiler/stdlib/plugin.md` §P819). Réplica de `Plugin::transition` +
`snapshot`/`restore` do vanilla (`plugin.rs:328-352,420-426,522-545`),
adaptada ao desenho "instância fresca por `call`" de P698 (que se mantém):

```rust
struct Snapshot {           // plugin.rs:420-426
    mem_pages: u64,
    mem_data: Vec<u8>,
}

struct PluginEntry {
    base: Arc<PluginBase>,
    /// `None` para módulos do `load`; `Some` para derivados de `transition`.
    snapshot: Option<Snapshot>,
}

// WasmiPluginHost: modules: Mutex<HashMap<PluginModuleId, Arc<PluginEntry>>>
```

1. **`transition(id, func, args)`**: lookup da entry (id ausente ⇒
   `"plugin module not found"`, mesma defesa de API directa — inferência
   marcada); instancia uma instância fresca do `base` e, se
   `entry.snapshot` é `Some`, **restaura-a** antes da chamada (réplica de
   `restore`, `plugin.rs:532-545`: cresce a memória se
   `current_size < snapshot.mem_pages`, copia `mem_data`); executa a
   chamada **mutável** com exactamente a mesma validação e mensagens de
   `call` (pontos 3–9 de `call` — assinatura, aridade, trap, out-of-bounds,
   código de retorno; um erro aborta sem derivado); depois de `Ok`, faz
   **snapshot** (réplica de `snapshot`, `plugin.rs:522-530`:
   `memory.size` + `memory.data().to_vec()`); regista
   `PluginEntry { base: Arc::clone(&entry.base), snapshot: Some(snap) }`
   com id fresco de `next_id` e devolve-o.
2. **`call` passa a restaurar o snapshot**: no ponto 2 de `call`, depois de
   instanciar, se `entry.snapshot` é `Some`, restaurar antes de executar.
   Módulos do `load` têm `snapshot: None` — comportamento de P698/P699
   inalterado.
3. **`exports`** é cego ao snapshot (só lê `base.module.exports()`) — sem
   alteração para além do tipo do mapa (`PluginBase` → `PluginEntry`).
4. **Globals WASM não são capturados** — limitação documentada do vanilla
   (`plugin.rs:177-182`), mantida por paridade (é o observável: só a
   memória linear é garantida após transition).
5. **Sem pool/fingerprint** (mecânica, diverge de propósito): o pool
   `Mutex<Vec<PluginInstance>>` do vanilla é optimização de concorrência;
   o fingerprint u128 (`plugin.rs:330`) existe para hash/eq de `Plugin` —
   no cristalino o papel é cumprido pelo id fresco por derivação + a
   memoização de `PluginFunc::transition` em L1 (duas transições com a
   mesma chave servem o mesmo id da cache, sem segunda chamada ao host).

Cópia de memória por snapshot é O(páginas×64 KiB) — aceite (o vanilla faz o
mesmo, `plugin.rs:527-529`); performance de RAM é domínio de L1/L3
(ADR-0030), não há I/O novo.

## Catálogo de mensagens (observável — paridade exacta, ADR-0107)

Medido em `plugin.rs` (file:line). L3 produz **verbatim**; L1/L2 propagam.

| # | Situação | Mensagem exacta | `file:line` |
|---|----------|-----------------|-------------|
| 1 | módulo sem `memory` | `plugin does not export its memory` | `plugin.rs:280` |
| 2 | wasm inválido no `Module::new` | `failed to load WebAssembly module ({err})` | `plugin.rs:276` |
| 3 | param não-`i32` | `plugin function \`{f}\` has a parameter that is not a 32-bit integer` | `plugin.rs:460-462` |
| 4 | resultado ≠ `[i32]` | `plugin function \`{f}\` does not return exactly one 32-bit integer` | `plugin.rs:465` |
| 5 | nº de args errado | `plugin function takes {expected} argument{s}, but {given} {verb} given` | `plugin.rs:472-476` |
| 6 | trap WASM | `plugin panicked: {err}` | `plugin.rs:492` |
| 7 | leitura/escrita fora de limites | `plugin tried to {read\|write} out of bounds: pointer {offset:#x} is out of bounds for {read\|write} of length {length}` | `plugin.rs:497-500` |
| 8 | retorno `1`, UTF-8 | `plugin errored with: {message}` | `plugin.rs:511` |
| 9 | retorno `1`, não-UTF-8 | `plugin errored, but did not return a valid error message` | `plugin.rs:513` |
| 10 | retorno ≠ 0,1 | `plugin did not respect the protocol` | `plugin.rs:516` |

Itens 1, 3, 4, 5, 6, 7, 8, 9, 10 são **paridade de linguagem** (a mensagem é o
observável). Item 2: a inferência original ("o vanilla usa o mesmo wasmi
1.0.9, logo o texto tende a coincidir") foi **refutada pela medição de
P810/P819** — o texto divergia porque o cristalino compilava o wasmi com a
feature default `wat` activa (o vanilla usa `default-features = false`,
`Cargo.toml.original:149`). Com o alinhamento de features de P819 (ver
§Dependência), o item 2 passa a **paridade byte-exacta medida** (t3 de
`temp/p810/plugin/`: `magic header not detected: bad magic number - …`
idêntico nos dois binários após a correcção — a verificar no relatório de
P819).
