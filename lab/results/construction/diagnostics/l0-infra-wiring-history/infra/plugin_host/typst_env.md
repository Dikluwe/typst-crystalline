# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host/typst_env.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `ce0468277d7325546633c85a097c640cca67ea0940524c79e02b4a5305020093`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: e2dae107
prompt: infra/plugin_host/typst_env
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host/typst_env

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:54-67, 103-124, 346-382`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host/typst_env.rs`. Possuir CallData, MemoryError, construção do linker e as duas host functions registradas sob typst_env, com nomes, assinaturas, ordem e mensagens internas expect originais. CallData é por Store e contém cópias dos buffers, output e último erro de memória. write_args consome args, escreve consecutivamente e registra a primeira falha; send_result consome output, redimensiona, lê e registra falha sem devolver buffer. Não adicionar imports, validação ou limites novos.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.

## Host functions (imports `typst_env`)

Réplica literal de `plugin.rs:577-612`. São `fn` livres em L3 (não métodos),
assinaturas exactas para `func_wrap`:

```rust
fn wasm_minimal_protocol_write_args_to_buffer(
    mut caller: wasmi::Caller<CallData>, ptr: u32,
) { /* plugin.rs:577-595 */ }

fn wasm_minimal_protocol_send_result_to_host(
    mut caller: wasmi::Caller<CallData>, ptr: u32, len: u32,
) { /* plugin.rs:598-612 */ }
```

- `write_args_to_buffer` (`plugin.rs:577-595`): obtém `memory`, faz
  `mem::take(&mut caller.data_mut().args)`, e para cada `arg` escreve em
  `offset` (começa em `ptr`); ao primeiro `memory.write` que falhar regista
  `MemoryError { offset, length: arg.len() as u32, write: true }` e retorna;
  senão `offset += arg.len()`.
- `send_result_to_host` (`plugin.rs:598-612`): obtém `memory`,
  `buffer = mem::take(&mut caller.data_mut().output)`, `buffer.resize(len, 0)`,
  lê `memory.read(ptr, &mut buffer)`; em falha regista
  `MemoryError { offset: ptr, length: len, write: false }` e retorna; senão
  `caller.data_mut().output = buffer`.
