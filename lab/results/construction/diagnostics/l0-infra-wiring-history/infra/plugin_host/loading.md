# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host/loading.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `9254d9875b983a861b2d10e09be4199241f4af44e3f7f3f5c73124fefbf86c2f`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 9b8af1bd
prompt: infra/plugin_host/loading
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host/loading

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:32-35, 91-144`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host/loading.rs`. Possuir PluginBase (Module compilado e Linker<CallData>), load e exports. Compilar o WASM antes de verificar o export memory, construir o linker via typst_env::linker, atribuir id Relaxed somente após sucesso e inserir entry sem snapshot. Preservar mensagens de load e ordem estável dos exports de função, com lookup defensivo pelo host. Não instanciar módulos em load nem adicionar validação eager de assinaturas ou imports.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.

## Dependência

Adicionar a `03_infra/Cargo.toml` (L3 **apenas** — L1 nunca vê `wasmi`):

```toml
wasmi = "1.0.9"   # P698 — runtime WASM de plugins; versão = vanilla (Cargo.lock:3938-3943)
```

`1.0.9` é a versão do vanilla (confirmada em P696 §1 contra `Cargo.lock:3938-3943`).

**P819 — alinhamento de features (medição, ADR-0108):** o vanilla compila
wasmi com `default-features = false, features = ["simd"]`
(`lab/typst-original/Cargo.toml.original:149`); o cristalino omitiu
`default-features = false` (`03_infra/Cargo.toml:39`), activando a feature
default `wat`. Efeito observável medido (t3 de `temp/p810/plugin/`, wasm de
texto inválido): o cristalino devolve o erro do parser WAT
(`expected `(` --> <anon>:1:1 …`) e o vanilla devolve
`magic header not detected: bad magic number - expected=[0x0,0x61,0x73,0x6d,] actual=[…] (at offset 0x0)`.
**P819 fixa `default-features = false, features = ["simd"]`** — o item 2 do
catálogo passa a paridade byte-exacta (a inferência "o texto tende a
coincidir" foi **refutada** pela medição de P810/P819; a causa era a
feature, não o wasmi). Verificar no build/testes que nada em L3 depende da
feature `wat` (o encoder de testes gera WASM binário).

## `load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError>`

Replica `plugin.rs:275-299`:

1. `Module::new(&self.engine, bytes)` → em erro,
   `format!("failed to load WebAssembly module ({err})")` (`plugin.rs:275-276`).
2. **Memória obrigatória**:
   `if !matches!(module.get_export("memory"), Some(wasmi::ExternType::Memory(_)))`
   → `PluginError::new("plugin does not export its memory")`
   (`plugin.rs:279-281`).
3. Construir `Linker::new(&self.engine)` e registar os dois imports sob o módulo
   `"typst_env"` (`plugin.rs:283-297`):
   - `func_wrap("typst_env", "wasm_minimal_protocol_send_result_to_host",
     wasm_minimal_protocol_send_result_to_host)` (`plugin.rs:284-290`);
   - `func_wrap("typst_env", "wasm_minimal_protocol_write_args_to_buffer",
     wasm_minimal_protocol_write_args_to_buffer)` (`plugin.rs:291-297`).
   (O vanilla chama `.unwrap()` após `func_wrap`; aqui propagar via `expect` com
   mensagem interna — registo de imports conhecidos, não falha em runtime.)
4. Gerar `id = next_id.fetch_add(1, Ordering::Relaxed)`, inserir
   `Arc::new(PluginEntry { base: Arc::new(PluginBase { module, linker }),
   snapshot: None })` no mapa, devolver `Ok(id)`.

## `exports(&self, id) -> Result<Vec<EcoString>, PluginError>` (P699)

Novo método do contrato L1 (`00_nucleo/prompts/contracts/plugin_host.md`), necessário para que
`native_plugin` construa o `Module` com uma função por export (P699). Replica
`into_module` (`plugin.rs:366-380`), filtrando apenas `ExternType::Func`:

1. Lookup do `Arc<PluginEntry>` por `id`; se ausente →
   `PluginError::new("plugin module not found")` (mesma defesa de API directa
   do ponto 1 de `call`; **inferência marcada**, sem equivalente observável
   no vanilla).
2. Iterar `base.module.exports()`; para cada `export` cujo
   `export.ty()` é `wasmi::ExternType::Func(_)`, recolher
   `EcoString::from(export.name())`. A ordem é a do `wasmi` (estável para um
   dado módulo).
3. Devolver `Ok(names)`. Se não houver exports de função, devolve `Ok(vec![])`
   (módulo vazio — `native_plugin` produz um `Module` sem membros).

`EcoString` (ADR-0024) é o tipo do contrato; a conversão `&str → EcoString`
acontece aqui (L3), sem re-exportar `ecow` em L1.
