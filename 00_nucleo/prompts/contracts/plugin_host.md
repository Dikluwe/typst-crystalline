# Prompt L0 — `contracts/plugin_host` — fronteira do runtime WASM (nível 3 de P696)
Hash do Código: 3f538240

**Camada**: L1
**Ficheiro alvo**: `01_core/src/contracts/plugin_host.rs`
**Passo de origem**: P698 (segundo passo da divisão proposta em P696)
**ADRs relevantes**: ADR-0107 (paridade com a linguagem — mensagens são mecânica
observável), ADR-0108 (medir antes de decidir), ADR-0109 (feature no seu ficheiro),
ADR-0111 (fronteira L1↔L3: tipos L1 puros, I/O em L3).

---

## Contexto e objectivo

P697 fixou o builtin `plugin()` e a **leitura** dos bytes (nível 2). P698
implementa **só o nível 3** do mapa de P696: o **runtime WASM** atrás de uma
trait de fronteira. Este ficheiro define a **fronteira L1** — o contrato puro que
o núcleo vê. A implementação concreta (`wasmi`) vive em L3 e é especificada em
`00_nucleo/prompts/infra/plugin_host.md`.

`wasmi` é um runtime grande e impuro-adjacente (interprete, memória mutável).
P696 §6 recomendou L3 atrás de trait (espelha `fontdb`/`hayagriva`): L1 vê só o
contrato; **`wasmi::*` nunca atravessa a fronteira** (evita V14 em L1 e mantém
"L1 mínimo").

## Por que um contrato novo (e não `World`)

`World` é o contrato *do ambiente de compilação* (ficheiros, fontes, data,
pacotes). O host de plugins é uma capacidade *ortogonal* (executar WASM) que
em P699 será injectada no pipeline de eval independentemente do `World`.
Separá-lo em trait própria (`PluginHost`) mantém `World` magro e permite testar
o host isoladamente em L3 sem um `World` completo — que é exactamente o âmbito
de P698 (testes directos em Rust, sem sintaxe Typst).

## Tipos de fronteira (L1 puros)

```rust
/// Handle opaco para um módulo WASM já carregado por um `PluginHost`.
/// L1 não vê o conteúdo — só o id. `Copy` para passar por valor em `call`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PluginModuleId(pub u64);
```

```rust
/// Erro do host de plugins. Carrega a **mensagem exacta** porque a mensagem
/// é o observável da linguagem (ADR-0107): `plugin("x.wasm")` que falha tem de
/// produzir, em P699, o mesmo texto do vanilla. Por isso a mensagem é
/// **contrato** (não detalhe de L3): L3 preenche, L1/L2 propagam verbatim.
///
/// NÃO contém `wasmi::Error` nem qualquer tipo externo — só `EcoString`
/// (ADR-0024, já em `[l1_allowed_external]`). Sem I/O, sem `std::error::Error`
/// via macro (Display/Error implementados à mão).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError {
    pub message: EcoString,
}

impl PluginError {
    pub fn new(msg: impl Into<EcoString>) -> Self { ... }
}
// impl fmt::Display  -> escreve self.message
// impl std::error::Error
```

`Bytes` é `crate::entities::bytes::Bytes` (tipo L1 puro, `Value::Bytes`) —
reusa o tipo já existente; **não** cria um novo. `EcoString` vem de `ecow`
(ADR-0024). Ambos estão em `[l1_allowed_external]` (ou são L1 nativos), logo a
assinatura do trait não dispara V14.

## O trait `PluginHost`

```rust
pub trait PluginHost: Send + Sync {
    /// Compila e valida os bytes WASM, devolvendo um handle. Falha (com a
    /// mensagem exacta do vanilla) se o módulo é inválido ou não exporta
    /// `memory`. Ver `infra/plugin_host.md` para o catálogo de mensagens.
    fn load(&self, bytes: &[u8]) -> Result<PluginModuleId, PluginError>;

    /// **P699** — lista os nomes dos exports do módulo que são funções
    /// (`ExternType::Func`). Replica `into_module` (`plugin.rs:366-380`); permite
    /// a `native_plugin` construir o `Module` com um `PluginFunc` por export.
    fn exports(&self, module: PluginModuleId) -> Result<Vec<EcoString>, PluginError>;

    /// Chama a função exportada `func_name` do módulo `module` com os buffers
    /// `args` (cada `Bytes` é um argumento; as **lengths** são passadas ao WASM
    /// como `i32`, o conteúdo é entregue via `typst_env`). Devolve os bytes de
    /// saída ou um `PluginError` com a mensagem exacta do vanilla.
    fn call(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<Bytes, PluginError>;

    /// **P819** — executa a chamada **mutável** `func_name(args)` sobre uma
    /// instância de `module`, faz snapshot da memória linear (páginas + dados
    /// — **não** globals WASM, limitação do vanilla mantida por paridade,
    /// `plugin.rs:177-182,420-426,522-545`) e regista um **módulo derivado**
    /// cujas chamadas seguintes (`call`/`exports` sobre o id devolvido)
    /// observam a mutação. O módulo original fica inalterado. Devolve o id
    /// do derivado. Erros da chamada: os mesmos do catálogo de `call`
    /// (verbatim). Réplica de `Plugin::transition`
    /// (`lab/typst-original/.../foundations/plugin.rs:328-352`); o
    /// fingerprint u128 do vanilla é mecânica — aqui o id fresco por
    /// derivação cumpre o mesmo papel (distingue "siblings").
    fn transition(
        &self,
        module: PluginModuleId,
        func_name: &str,
        args: &[Bytes],
    ) -> Result<PluginModuleId, PluginError>;
}
```

### Por que `&self` (e não `&mut self`)

Em P699 o host vive atrás de `Arc<dyn PluginHost>` partilhado pelo pipeline e
pelo `comemo` (memoização de `call`). `&self` força a mutabilidade interior a
ficar **em L3** (onde `Mutex` é permitido) e mantém o contrato L1 livre de
primitivas de sincronização. `Send + Sync` **não** é exigido neste passo (host
testado em single-thread); fica como débito para P699 se a injectão o exigir —
registado em "Scope-out".

## Contrato (o que L1 pode assumir)

1. **`load` valida à entrada**: módulo que compila e exporta `memory`. Depois de
   `Ok(id)`, `call(id, …)` nunca falha por "módulo inválido" nem por "sem
   memória" — esses casos foram excluídos no `load`.
2. **Mensagens verbatim**: qualquer `Err(PluginError { message })` produzido por
   L3 usa o texto **exacto** do vanilla (catálogo em `infra/plugin_host.md`).
   L1/L2 propagam `message` sem reescrever, prefixar ou traduzir — a paridade é
   ao nível da linguagem e a mensagem **é** o observável (ADR-0107).
3. **Pureza delegada ao autor**: tal como o vanilla, o host **não** enforce
   pureza; a memoização (P699) assume-a. P698 só garante que cada `call` corre
   sobre uma instância com a memória inicial do módulo (ver `infra/plugin_host.md`).

## Scope-out (próximos passos) — NÃO implementado aqui

- **P699** — `native_plugin` passa a devolver `Module` real: itera os exports
  `Func` do módulo carregado (`into_module`), cria `PluginFunc` por export,
  `p.f(*bytes) -> bytes`, `#[comemo::memoize]` em `call`/`load`, e injecta
  `Arc<dyn PluginHost>` no eval. É aí que o trait ganha `Send + Sync` se
  necessário.
- **P700** — validação contra `cetz_core.wasm`.
- ~~**P701+** — transition API (snapshot/restore, fingerprint)~~ —
  **fechado em P819**: o trait ganha `transition` (ver assinatura acima;
  especificação de linguagem em `prompts/compiler/stdlib/plugin.md` §P819,
  implementação em `prompts/infra/plugin_host.md`).

**Nota de hash (P819):** alterado na fase de sonda/L0 de P819; o
`@prompt-hash` de `01_core/src/contracts/plugin_host.rs` será recalculado
pelo humano via `crystalline-lint --fix-hashes .` após confirmação.

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| `load` recusa módulo sem `memory` | mensagem (mecânica observável) | sim (texto exacto) |
| `call` mapeia código 0/1/outro | mensagem (mecânica observável) | sim (texto exacto) |
| `call` reporta trap / out-of-bounds | mensagem (mecânica observável) | sim (texto exacto) |
| `PluginHost` ser trait em L1 (não `wasmi` em L1) | mecânica (estrutura) | diverge de propósito (P329) |
| id `u64` opaco em vez de `Arc<Plugin>` | mecânica (estrutura) | diverge de propósito |

## Critérios de verificação (deste ficheiro L1)

- `contracts/plugin_host.rs` compila em L1 **sem** importar `wasmi`, `std::fs`,
  `std::net`, `std::time` (V3/V4 limpos).
- Assinatura do trait usa só `Bytes`, `u64`/`PluginModuleId`, `EcoString`,
  `String`/`&str` — todos L1-nativos ou em `[l1_allowed_external]` (V14 limpo).
- `PluginError` implementa `Display`/`Error` e preserva `message` verbatim.
- Header `@prompt 00_nucleo/prompts/contracts/plugin_host.md` e `@prompt-hash`
  correcto (via `crystalline-lint --fix-hashes .`).
- `crystalline-lint .` zero violations.
