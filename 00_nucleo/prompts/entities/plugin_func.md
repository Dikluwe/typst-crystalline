# Prompt L0 — `entities/plugin_func` — `PluginFunc` chamável + cache (níveis 4–5 de P696)
Hash do Código: fc42ece4

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/plugin_func.rs`
**Passo de origem**: P699 (terceiro passo da divisão proposta em P696)
**Contratos**: `00_nucleo/prompts/contracts/plugin_host.md` (`PluginHost`),
`00_nucleo/prompts/entities/func.md` (`FuncRepr::Plugin`),
`00_nucleo/prompts/engine/stdlib/plugin.md` (constrói o `Module`).
**ADRs relevantes**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de
decidir), ADR-0109 (feature no seu ficheiro — `PluginFunc` vive em `entities`
porque `func.rs` não pode importar `rules`, ADR-0109 proíbe `entities→engine`).

---

## Contexto e objectivo

`plugin("x.wasm")` (P697/P698) passa, em P699, a devolver um `Module` cujo
`Scope` liga cada **export** WASM do tipo `Func` a uma função chamável. Esse
valor chamável é `PluginFunc` — o equivalente ao `PluginFunc` do vanilla
(`lab/typst-original/.../foundations/plugin.rs:205-231`). Vive em `entities`
(não em `rules`) porque `FuncRepr` (`entities/func.rs`) precisa de o referenciar
e a ADR-0109 proíbe import reverso `entities→engine`.

`PluginFunc` captura o host (`Arc<dyn PluginHost>`), o handle do módulo e o
nome do export. A chamada valida os argumentos (todos `bytes`) e delega a
`PluginHost::call`; a cache é `#[comemo::memoize]` (comemo 0.4, já disponível
no workspace — ver §Cache).

## Tipo

```rust
#[derive(Debug, Clone)]
pub struct PluginFunc {
    /// Host que executa o WASM (L3 por detrás do trait). Capturado no momento
    /// em que `native_plugin` constrói o `Module` — a chamada não precisa de o
    /// ir buscar ao `Engine`/`World`.
    host: Arc<dyn PluginHost>,
    /// Handle do módulo WASM carregado (emitido pelo host).
    module: PluginModuleId,
    /// Nome do export WASM (ex.: "hello").
    name: EcoString,
}
```

`host: Arc<dyn PluginHost>` é `Send + Sync` porque o trait `PluginHost` ganha
o supertrait `Send + Sync` em P699 (ver `contracts/plugin_host.md`). Isto é
**necessário**, não opcional: o host viaja dentro de `Value::Func` (via
`SystemWorld` → `World`, que é `Send + Sync`) e o precedente do cristalino é
exactamente este — `ElementCtor = Arc<dyn Fn … + Send + Sync>`
(`entities/element_registry.rs:37`) porque "Value/Content vivem em contextos
`Send + Sync`".

## `FuncRepr::Plugin` e `Func::plugin` (em `entities/func.rs`)

`FuncRepr` ganha uma variante (ver `entities/func.md`):

```rust
pub(crate) enum FuncRepr {
    // ...existentes...
    /// P699 — função de plugin WASM (nome de export dinâmico; chamada delegada
    /// ao `PluginHost` capturado).
    Plugin(PluginFunc),
}
```

Construtor público:

```rust
impl Func {
    pub fn plugin(p: PluginFunc) -> Self {
        Self(Arc::new(FuncRepr::Plugin(p)))
    }
}
```

Comportamento dos acessores existentes para a nova variante:
- `name()` → `Some(p.name.as_str())` (apresentação/erros).
- `native_fn_addr()` → `None` (não é fn-ptr nativo; como `Closure`/`Element`).
- `namespace()` → `None`.

## Despacho — `apply_func` (`rules/eval/closures.rs`)

O dispatcher (`apply_func`, `rules/eval/closures.rs:63-94`) ganha um braço:

```rust
FuncRepr::Plugin(p) => call_plugin(p, args),
```

`call_plugin` (helper livre em `plugin_func.rs` ou local ao braço):
1. Valida que **todos** os `args.items` são `Value::Bytes`; qualquer outro tipo
   → erro `"plugin function \`{name}\` expects bytes arguments, found {tipo}"`
   (paridade: vanilla só aceita `bytes`; a mensagem exacta é mecânica — ver
   `infra/plugin_host.md`).
2. Reúne `Vec<Bytes>` e chama `p.call(bytes)` (método memoizado — §Cache).
3. `Ok(bytes)` → `Value::Bytes(bytes)`; `Err(PluginError { message })` →
   `SourceDiagnostic::error(Span::detached(), message)` (propaga verbatim — a
   mensagem é o observável, ADR-0107).

## Cache — `#[comemo::memoize]` (medido contra comemo 0.4.0)

Não há uso prévio de `#[comemo::memoize]` nos crates cristalinos (confirmado
por varredura: só `#[comemo::track]`); comemo 0.4 exporta `memoize`
(`comemo-0.4.0/src/lib.rs:95`) e compila aqui. Restrições lidas no fonte do
macro (`comemo-macros-0.4.0/src/memoize.rs`, `comemo-0.4.0/src/{input,cache}.rs`):

- Receiver `&self` é **hashed** → `Self: Hash` obrigatório (não `Track`).
- Cada arg tem de ser `Input` (blanket-impl para `T: Hash`) → `Vec<Bytes>`
  serve porque `Bytes: Hash` (`entities/bytes.rs:14`).
- Retorno `Clone + Send + Sync + 'static` (não precisa de `Hash`) →
  `Result<Bytes, PluginError>` serve (`PluginError` é `Clone`, não `Hash`).

Como `Arc<dyn PluginHost>` **não** é `Hash` (trait object), espelhamos o
vanilla (`plugin.rs:383-400`, que alimenta só a identidade ao hasher) e
**escrevemos `PartialEq`/`Eq`/`Hash` à mão sobre a identidade `(module, name)`**,
excluindo `host` de **ambos** (mantém o contrato `a==b ⇒ hash(a)==hash(b)`):

```rust
impl PartialEq for PluginFunc {
    fn eq(&self, o: &Self) -> bool { self.module == o.module && self.name == o.name }
}
impl Eq for PluginFunc {}
impl std::hash::Hash for PluginFunc {
    fn hash<H: std::hash::Hasher>(&self, s: &mut H) {
        self.module.hash(s);
        self.name.hash(s);
        // host excluído de propósito: é determinado por `module` (um
        // `PluginModuleId` só é válido no host que o emitiu).
    }
}

impl PluginFunc {
    #[comemo::memoize]
    pub fn call(&self, args: Vec<Bytes>) -> Result<Bytes, PluginError> {
        self.host.call(self.module, self.name.as_str(), &args)
    }
}
```

**Colisão entre hosts (inferência marcada, ADR-0108):** a chave é
`(module, name, args)`. Dois hosts distintos que emitam o mesmo `module` id
colidem na cache `static` do comemo (partilhada pelo processo/testes). Em
produção há um host por `World`, logo `module` identifica univocamente. Em
testes, cada teste usa **nomes de export distintos** (ou args distintos) para
que as chaves não colidam — isto é arnês de teste, não paridade de linguagem.

## `PluginFunc::transition` (P819)

Réplica de `PluginFunc::transition` do vanilla
(`lab/typst-original/.../foundations/plugin.rs:226-231`), também memoizada:

```rust
impl PluginFunc {
    /// **P819** — transition API: executa a chamada mutável e devolve o
    /// `Module` derivado (mesmos exports; as chamadas sobre o derivado
    /// observam a mutação; o módulo original fica inalterado). Memoizada
    /// como o vanilla — duas transições idênticas `(module, name, args)`
    /// chamam `host.transition` uma vez e devolvem o mesmo derivado.
    #[comemo::memoize]
    pub fn transition(&self, args: Vec<Bytes>) -> Result<Module, PluginError> {
        let derived = self.host.transition(self.module, self.name.as_str(), &args)?;
        let names = self.host.exports(derived)?;
        // scope com um PluginFunc { host, module: derived, name } por export,
        // `Module::new("plugin", scope)` — mesma construção de `native_plugin`.
        ...
    }
}
```

- O `host` é excluído da chave de cache (mesma fórmula `PartialEq`/`Hash`
  manual de §Cache); o id do derivado é emitido pelo host, uma vez por
  derivação efectiva — duas derivações com a mesma chave são servidas pela
  cache, logo observam o **mesmo** id derivado (como o fingerprint do
  vanilla distingue siblings e identifica gémeos).
- Restrição do comemo (medida em P699): o retorno tem de ser
  `Clone + Send + Sync + 'static` — `Module`/`PluginError` cumprem
  (verificar no build; `Module` já viaja em `Value::Module` clonável).
- `Module` vem de `crate::entities::module::Module` (já usado em
  `native_plugin`); não há novo import de camada.

**Nota de hash (P819):** alterado na fase de sonda/L0 de P819; o
`@prompt-hash` de `01_core/src/entities/plugin_func.rs` será recalculado
pelo humano via `crystalline-lint --fix-hashes .` após confirmação.

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| `p.export(bytes…)` é chamável e devolve `bytes` | semântica/sintaxe | sim |
| um `PluginFunc` por export `Func` | semântica | sim (`plugin.rs:366-380`) |
| só `bytes` são aceites como args | semântica (protocolo) | sim |
| cache de chamadas repetidas | semântica (pureza assumida) | sim (comemo, como vanilla) |
| `Arc<dyn PluginHost>` capturado (vs `Arc<Plugin>`) | mecânica (estrutura) | diverge de propósito (P329) |
| `Hash` manual sobre `(module,name)` | mecânica (cache key) | diverge de propósito |

## Critérios de verificação

- `PluginFunc` compila em L1 (só `Arc`, `EcoString`, `PluginHost`/`PluginModuleId`/
  `PluginError`/`Bytes` de `contracts`/`entities` — sem `wasmi`, sem I/O;
  V3/V4/V14 limpos).
- `#[comemo::memoize]` em `call` compila (Self: Hash manual; `Vec<Bytes>`: Hash;
  retorno `Clone + Send + Sync + 'static`).
- Teste de cache: `CountingHost` (host de teste com `AtomicUsize`) — chamar o
  mesmo `PluginFunc` com os mesmos `bytes` **duas** vezes incrementa o
  contador do host **uma** vez (segunda chamada servida pela cache).
- Teste de validação: arg não-`bytes` → erro; `bytes` → `Value::Bytes`.
- Header `@prompt 00_nucleo/prompts/entities/plugin_func.md` e `@prompt-hash`
  correcto (`crystalline-lint --fix-hashes .`); `crystalline-lint .` zero
  violations.
