# `rules/stdlib/plugin` — builtins de plugin WASM (linguagem Typst)
Hash do Código: 09c4ad8e

Módulo de **linguagem** (P329, ADR-0107): regista a função standard
`plugin` e expõe a sua semântica. A mecânica WASM (wasmi, encoders, linker,
limites de combustível/memória) vive em L3 (`03_infra/src/plugin_host.rs`,
coberto por `prompts/infra/plugin_host.md`). Aqui só se define **o que a
linguagem observa**: o valor devolvido, a chamada `p.funcao(bytes)`,
`#import plugin("f.wasm"): export`, os erros e o cache.

## Regra de divisão entre passos

- **P697** (fechado): `plugin("f.wasm")` / `plugin(bytes)` lê o payload
  (path relativo via `World::read_bytes`, ou bytes directos), valida aridade
  e tipos, rejeita named args. O valor devolvido era provisório.
- **P698** (fechado): introduziu `PluginHost` (trait L1 + impl L3) com
  `load(bytes) -> PluginId` e `call(id, func, args) -> Bytes`; a infra real
  fica pronta, mas ainda não ligada à linguagem.
- **P699 (este):** liga `PluginHost` à linguagem. `plugin()` devolve um
  `Module` real; por cada export do módulo WASM há uma função acessível por
  nome (`p.hello`), chamável como `p.hello(bytes) -> bytes`; o módulo é
  importável com `#import plugin("f.wasm"): hello`; as chamadas ao host
  são **memoizadas** (cache via `#[comemo::memoize]`). Isto exige injecção
  do host pelo `World` (ver `prompts/contracts/world.md`).

## Dependência (cadeia de chamada a partir da linguagem)

```text
rules/stdlib/plugin.rs (este módulo, L1)
  ↓
contracts/world::World::plugin_host() -> Option<Arc<dyn PluginHost>>
  ↓
contracts/plugin_host::PluginHost { load, exports, call }
  ↓
03_infra/src/plugin_host.rs (wasmi, L3 — mecânica, fora do L0 de linguagem)
```

O mecanismo de injecção (quem instancia `WasmiPluginHost` e onde o
`SystemWorld` o recebe) é mecânica de L3/L4, não matéria deste L0 de
linguagem; fica especificado em `prompts/infra/system-world.md` e
`prompts/infra/plugin_host.md`.

## Inventário de items públicos (medição prévia, ADR-0108)

Em `01_core/src/engine/stdlib/plugin.rs` (estado P698, HEAD `b241f1525`),
há exactamente:

- `1 função pública` registada no escopo: `plugin` (nome exposto à
  linguagem), via `scope.define_func("plugin", native_plugin)` em
  `01_core/src/engine/eval/mod.rs:1050`.
- `1 função nativa`: `native_plugin` (4 args, `Native`, `#[comemo::memoize]`,
  assinatura `native_plugin(engine, args, span) -> Value`).
- `0` items `pub` adicionais exportáveis pelo módulo (não há struct/enum
  público definido aqui; o tipo chamável `PluginFunc` vive em
  `entities/plugin_func.rs`, ver `prompts/entities/plugin_func.md`).

## Semântica de `plugin` (linguagem)

### Assinatura observável

```typst
plugin("caminho/relativo.wasm")  // lê via World::read_bytes (path relativo ao ficheiro atual)
plugin(bytes)                    // payload directo (já lido/carregado noutro lugar)
```

Devolve `module` (um módulo da linguagem, `Value::Module(Module)`). Não
devolve bytes, não devolve função, não devolve string.

### Regras de aridade e tipo (inalteradas de P697)

- **Exactamente 1 argumento posicional.** `reject_named(args, span, "plugin")`
  rejeita named args com `error: unexpected argument`.
- O argumento posicional é **`str`** (path relativo, lido via
  `World::read_bytes`) **ou `bytes`** (payload directo).
  - `str` ⇒ `World::read_bytes(path_str)`, erro `PluginError` mapeado
    verbatim para `SourceDiagnostic`.
  - `bytes` ⇒ usado tal qual.
- Qualquer outro tipo ⇒ `error: expected string or bytes, found <tipo>`.

### Resolução do host (novo em P699)

Depois de obter `bytes: Bytes`:

```text
let host = world.plugin_host()
    .ok_or_else(|| erro "plugins não suportados neste World")?;
```

- `World::plugin_host()` é um método defaulted que devolve
  `Option<Arc<dyn PluginHost>>` (default `None`); ver
  `prompts/contracts/world.md`. `SystemWorld` sobrescreve e devolve
  `Some(...)` quando há host instalado.
- A mensagem de ausência de host é **parte do observável** (a ausência é
  mecânica de L3/L4, mas a **mensagem** é observável — ADR-0108). Verbatim
  recomendada: `plugins não suportados neste World`.

### Carregar e enumerar exports

```text
let id    = host.load(&bytes).map_err(|e| diag(&e.message))?;
let names = host.exports(id).map_err(|e| diag(&e.message))?;
```

- `PluginHost::load` valida o módulo (formato WASM, limites) e devolve um
  `PluginId` opaco (só significativo para este host).
- `PluginHost::exports(id)` devolve a **lista dos nomes exportados que são
  funções** (`ExternType::Func`), na ordem estável que o host definir.
  Ver `prompts/contracts/plugin_host.md` (contrato) e
  `prompts/infra/plugin_host.md` (implementação: iterar `module.exports()`
  e filtrar `ExternType::Func`).
- `PluginError { message }` ⇒ `SourceDiagnostic::error(span, message)`
  (propagação verbatim — a mensagem de erro é observável, ADR-0108).

### Construção do `Module` devolvido

Para cada `name` em `names`:

```text
scope.define(name.clone(), Value::Func(Func::plugin(PluginFunc {
    host:   Arc::clone(&host),
    module: id,
    name,
})))
```

- O `PluginFunc` é um `Value` chamável (`Func`) cujo `FuncRepr::Plugin`
  carrega `host`, `module`, `name`. Ver `prompts/entities/plugin_func.md`
  e `prompts/entities/func.md`.
- `Arc::clone(&host)` é partilhado por todos os exports do mesmo módulo
  (barato, O(1) por export).
- O `Module` é construído com `Module::new("plugin", scope)` (nome do
  módulo = `"plugin"`, idêntico ao da infra real / consistência histórica;
  o conteúdo é o `scope` com as funções).

Devolve `Ok(Value::Module(module))`.

### Chamada `p.funcao(bytes) -> bytes`

A chamada é feita em `apply_func` (`rules/eval/closures.rs`), via
`FuncRepr::Plugin(p) => call_plugin(p, args, span)`. Contrato:

- **Validação de args:** todos os argumentos posicionais têm de ser
  `Value::Bytes`. Se qualquer arg não for bytes, erro:
  `error: plugin function arguments must be bytes, found <tipo>`.
- **Sem named args:** `reject_named(args, span, p.name)` — named args são
  rejeitadas (a ABI WASM é posicional).
- **Reunião:** os `Bytes` dos args são reunidos num `Vec<Bytes>` (cópia de
  handle, não de conteúdo).
- **Chamada ao host:** `p.call(args_vec)` (memoizada — ver abaixo).
- **Resultado:** `Ok(bytes) ⇒ Value::Bytes(bytes)`; `Err(e) ⇒
  SourceDiagnostic::error(span, e.message)` (verbatim).

`PluginFunc::name()` devolve `Some(&self.name)` (para o erro de named
args / mensagens); `native_fn_addr()` e `namespace()` devolvem `None`
(um plugin não tem endereço de função nativa nem namespace).

### `#import plugin("f.wasm"): export`

`#import plugin("f.wasm"): hello` é uma composição de duas operações já
existentes — **nenhuma alteração ao import** (ver `prompts/engine/import.md` e
`01_core/src/engine/eval/modules.rs:167-219`, eval.md §P683):

1. `plugin("f.wasm")` ⇒ `Module` (esta especificação).
2. `: hello` ⇒ seleciona `module.scope().get("hello")` ⇒ `Value::Func`.

Resultado: `hello` é uma função chamável no escopo actual. Importar um
export que não existe ⇒ erro do import (`module ... has no member ...`),
sem alteração (P699 não toca no import).

## Cache (comemo)

`PluginFunc::call(&self, args: Vec<Bytes>) -> Result<Bytes, PluginError>`
é anotado com `#[comemo::memoize]`:

- **Chave de cache:** `(self, args)` = `(module, name, args)`; `host` é
  **excluído** da chave (é handle partilhado; dois `PluginFunc` do mesmo
  `(module, name)` são o mesmo export).
- **`PartialEq`/`Eq`/`Hash` manuais** em `PluginFunc` sobre
  `(module, name)` — **não** usar `#[derive]` (incluiria `host`, que não
  é `Hash`). Ver `prompts/entities/plugin_func.md` para a fórmula exacta.
- **Efeito observável:** duas chamadas `p.hello(b"x")` com os mesmos args
  chamam o host **uma** vez; a segunda é servida do cache. O contador de
  chamadas do host (num host de teste) prova o hit.
- **`Bytes` é `Hash`/`Eq`/`Clone`** (`01_core/src/entities/bytes.rs:14`),
  por isso `Vec<Bytes>` é chave válida sem adaptação.
- **Consequência operacional (testes):** como a chave inclui `module`
  (um id por host) e hosts distintos recomeçam ids em `1`, o cache
  `static` do comemo pode colidir entre testes que partilhem
  `(module=1, name="x", args=...)`. Mitigação: cada teste P699 usa
  **nomes de export únicos** (p.ex. o teste de cache usa um nome só dele).
  Os testes de P698 (`plugin_host.rs`) chamam `host.call` directo (não
  passam pelo `PluginFunc`/memoize) e não são afectados.

Nota de descoberta (P699): `comemo = "0.4"` já expõe `memoize` no
workspace, mas **nenhum crate a usa antes de P699**; somos os primeiros.
`comemo::track` está em uso (`entities/sink.rs`) e `comemo::Track` é
derivado por várias structs — não confundir.

## Erros (observáveis — ADR-0108)

- Ausência de host: `error: plugins não suportados neste World`.
- Aridade errada / named args: mensagens do helper `reject_named` /
  validação de aridade existentes (P697).
- Tipo errado do arg: `error: expected string or bytes, found <tipo>`.
- `host.load` / `host.exports` / `host.call`: `PluginError.message`
  propagada verbatim via `SourceDiagnostic::error(span, message)`.

Todos os erros são produzidos com `span` do callsite (o `span` passado a
`native_plugin` e o `span` do `apply_func`).

## Testes

Os testes vivem em `01_core/src/engine/stdlib/plugin.rs` (unidade) e em
`04_wiring/tests/` (e2e, com host real). O encoder WASM é
`#[cfg(test)]`-privado de `03_infra/src/plugin_host.rs`; testes L1 usam
um `CountingHost`/mock host (não precisam de WASM real).

Cobertura mínima (P699):

- **Caminho feliz:** `plugin(bytes)` ⇒ `Module`; `p.hello(b"")` ⇒ bytes;
  `#import plugin("f.wasm"): hello` ⇒ função chamável.
- **Cache:** duas chamadas `p.hello(b"x")` ⇒ contador do host = 1.
- **Arg não-bytes:** `p.hello(1)` ⇒ erro "arguments must be bytes".
- **Sem host:** `World` sem host ⇒ `plugin(...)` ⇒ erro de ausência.
- **Erro do host:** `host.load`/`exports`/`call` a falhar ⇒ mensagem
  verbatim no diagnóstico.

## Fora de escopo (P699)

- Encoders WASM e limites de combustível/memória: mecânica L3
  (`prompts/infra/plugin_host.md`).
- Instanciação do host real e encadeamento no `SystemWorld`/CLI: L3/L4
  (`prompts/infra/system-world.md`, `prompts/infra/plugin_host.md`).
- Mudança ao import (`#import ... : ...`): já funciona (P683), sem toque.
- Remover `native_plugin` do registo `Native`: mantém-se (P697), apenas
  deixa de devolver valor provisório.

## Critério de aceitação

No nível da **linguagem** (ADR-0107), não da mecânica:

- `plugin("f.wasm")` e `plugin(bytes)` devolvem `module`.
- `module.export(bytes)` chama a função WASM e devolve `bytes`.
- `#import plugin("f.wasm"): export` expõe a função ao escopo.
- Args não-bytes e ausência de host produzem os erros especificados.
- Duas chamadas idênticas batem no cache (host chamado uma vez).
- `cargo test --workspace` sem regressão; `crystalline-lint .` com
  zero violations.
