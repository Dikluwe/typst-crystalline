# Prompt L0 — `infra/plugin_host` — `WasmiPluginHost` (nível 3 de P696)
Hash do Código: fbc7d62d

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/plugin_host.rs`
**Passo de origem**: P698 (segundo passo da divisão proposta em P696)
**Contrato L1**: `00_nucleo/prompts/contracts/plugin_host.md`
**ADRs relevantes**: ADR-0107 (paridade com a linguagem — mensagens são mecânica
observável), ADR-0108 (medir antes de decidir — cada mensagem citada com
`file:line` do vanilla), ADR-0109 (feature no seu ficheiro), ADR-0111 (I/O e
runtime em L3; fronteira L1 pura).

---

## Contexto e objectivo

Implementa `PluginHost` (trait L1, `contracts/plugin_host.md`) com o runtime
`wasmi`, replicando **exactamente** o comportamento confirmado em P696 contra o
código fonte do vanilla. Âmbito de P698: **só o host** — testado directamente em
Rust (`cargo test -p typst-infra plugin_host`), **sem** passar pela sintaxe
Typst (essa ligação é P699). Não há `Module`, `PluginFunc`, `transition`,
snapshot, fingerprint nem pool multi-thread neste passo.

Fonte da verdade (medida, não inferida): o ficheiro
`lab/typst-original/crates/typst-library/src/foundations/plugin.rs` (612 linhas),
lido directamente. Todas as mensagens e decisões abaixo citam `file:line`
desse ficheiro.

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

## Estrutura

```rust
pub struct WasmiPluginHost {
    engine: wasmi::Engine,
    // id atómico para handles; módulos por detrás de Mutex (trait é &self).
    next_id: AtomicU64,
    modules: Mutex<HashMap<PluginModuleId, Arc<PluginBase>>>,
}

struct PluginBase {
    module: wasmi::Module,
    linker: wasmi::Linker<CallData>,  // com os 2 imports typst_env registados
}

#[derive(Default)]
struct CallData {
    args: Vec<Vec<u8>>,                 // buffers da chamada corrente
    output: Vec<u8>,                    // resultado da chamada corrente
    memory_error: Option<MemoryError>,  // último erro de bounds (se houve)
}
struct MemoryError { offset: u32, length: u32, write: bool }
```

Notas de implementação (mecânica — divergem do vanilla de propósito, P329):

- **Instância fresca por `call`** (não pool). Para cada `call`, criar
  `Store::new(&engine, CallData::default())` + `linker.instantiate_and_start`.
  Isto dá a cada chamada a memória inicial do módulo (segmento de dados
  restaurado) e evita partilhar `Store` (que exige `&mut`). O vanilla usa um
  `Mutex<Vec<PluginInstance>>` pool para multi-threading (`plugin.rs:245,355-363`);
  o pool é optimização de concorrência — **scope-out** de P698 (host testado em
  single-thread). A **semântica** preserva-se: plugins devem ser puros, logo uma
  instância fresca por chamada é observacionalmente equivalente para plugins
  puros (o único caso suportado — ver P696 §4).
- `CallData.args` guarda `Vec<Vec<u8>>` (cópia dos buffers) em vez de
  `Vec<Bytes>` — `Bytes` é L1; dentro de L3 usamos o `Vec<u8>` cru para o host
  funcs. Equivalente (os bytes são os mesmos).

## `WasmiPluginHost::new() -> Self`

Replica `Plugin::new` parcial (`plugin.rs:268-308`):

1. `let mut config = wasmi::Config::default();`
2. `config.wasm_relaxed_simd(false);` — determinismo (`plugin.rs:271-272`).
3. `let engine = wasmi::Engine::new(&config);` (`plugin.rs:274`).
4. `modules: Mutex::new(HashMap::new())`, `next_id: AtomicU64::new(1)`.

O `Linker` é construído por módulo em `load` (o vanilla constrói um linker por
`PluginBase`, `plugin.rs:283-299`) — `func_wrap` regista as duas funções no
linker daquele módulo.

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
4. Gerar `id = next_id.fetch_add(1)`, inserir `Arc::new(PluginBase { module,
   linker })` no mapa, devolver `Ok(id)`.

## `call(&self, id, func_name, args) -> Result<Bytes, PluginError>`

Replica `PluginInstance::call` (`plugin.rs:447-520`) sobre uma instância fresca:

1. Lookup do `Arc<PluginBase>` por `id`; se ausente →
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

## `exports(&self, id) -> Result<Vec<EcoString>, PluginError>` (P699)

Novo método do contrato L1 (`contracts/plugin_host.md`), necessário para que
`native_plugin` construa o `Module` com uma função por export (P699). Replica
`into_module` (`plugin.rs:366-380`), filtrando apenas `ExternType::Func`:

1. Lookup do `Arc<PluginBase>` por `id`; se ausente →
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

## `transition` (P819) — snapshot/restore sobre instância fresca

Implementa `PluginHost::transition` (contrato em
`prompts/contracts/plugin_host.md`; semântica de linguagem em
`prompts/compiler/stdlib/plugin.md` §P819). Réplica de `Plugin::transition` +
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

## `Send + Sync` (P699)

`WasmiPluginHost` implementa `Send + Sync` (obrigatório: `World: Send + Sync`
— `contracts/world.rs:33` — e o host viaja em `SystemWorld` e dentro de
`Value::Func(FuncRepr::Plugin(...))`). O precedente é `ElementCtor`
(derivado `Send + Sync` em L1).

Os campos são todos `Send + Sync`:

- `wasmi::Engine` — `Send + Sync` (partilhado entre threads no vanilla).
- `AtomicU64` — `Send + Sync`.
- `Mutex<HashMap<PluginModuleId, Arc<PluginBase>>>` — `Send + Sync` se
  `PluginBase: Send`.
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

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| as 10 mensagens do catálogo | mensagem (mecânica observável) | sim (verbatim, com inferência marcada no `{err}` do item 2) |
| `wasm_relaxed_simd(false)` | semântica (determinismo) | sim |
| exigir `memory`, 2 imports `typst_env`, validação lazy | semântica/protocolo | sim |
| instância fresca por `call` (vs pool) | mecânica (estrutura/perf) | diverge de propósito — equivalente para plugins puros |
| `AtomicU64`+`Mutex<HashMap>` (vs `Mutex<Vec<Instance>>`) | mecânica (estrutura) | diverge de propósito |

## Critérios de verificação

Testes directos em Rust em `03_infra/src/plugin_host.rs` (`#[cfg(test)]`),
**sem** sintaxe Typst:

- `hello.wasm` (192 B, regenerado pelo `gen_wasm.py` de P696 §A):
  `let id = host.load(&bytes).unwrap(); host.call(id, "hello", &[])` →
  `Ok(Bytes::new(b"hello".to_vec()))`.
- Cada item do catálogo coberto por um teste que constrói (em scratch, gerado
  por helper de teste) um wasm mínimo que provoca a mensagem e afirma
  `err.message == "<texto exacto>"`: sem `memory` (1); param não-`i32` (3);
  resultado ≠ `[i32]` (4); nº de args errado (5); trap via `unreachable` (6);
  out-of-bounds em write e em read (7); retorno `1` com UTF-8 (8) e com
  não-UTF-8 (9); retorno `2` (10). Item 2 (wasm inválido) coberto com bytes
  truncados — afirma só o prefixo `failed to load WebAssembly module (` (porque
  o `{err}` é inferência marcada).
- Header `@prompt 00_nucleo/prompts/infra/plugin_host.md` e `@prompt-hash`
  correcto (via `crystalline-lint --fix-hashes .`).
- `cargo test -p typst-infra plugin_host` verde; `cargo test --workspace` sem
  regressão; `crystalline-lint .` zero violations (L3 pode importar `wasmi`;
  V3/V4/V14 não se aplicam a L3).

**Nota de hash (P819):** este L0 foi alterado na fase de sonda/L0 de P819
(transition + features wasmi + item 2 do catálogo); o `@prompt-hash` de
`03_infra/src/plugin_host.rs` será recalculado pelo humano via
`crystalline-lint --fix-hashes .` após confirmação.
