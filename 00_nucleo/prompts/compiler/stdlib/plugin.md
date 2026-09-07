# `rules/stdlib/plugin` — builtins de plugin WASM (linguagem Typst)
Hash do Código: 789b8631

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
- **P699 (fechado):** liga `PluginHost` à linguagem. `plugin()` devolve um
  `Module` real; por cada export do módulo WASM há uma função acessível por
  nome (`p.hello`), chamável como `p.hello(bytes) -> bytes`; o módulo é
  importável com `#import plugin("f.wasm"): hello`; as chamadas ao host
  são **memoizadas** (cache via `#[comemo::memoize]`). Isto exige injecção
  do host pelo `World` (ver `prompts/contracts/world.md`).
- **P819 (este):** `plugin.transition` (transition API do vanilla —
  `plugin.rs:158-202`), três mensagens L1 alinhadas verbatim com o vanilla
  (ficheiro inexistente, tipo do argumento de `plugin()`, argumento
  não-bytes na chamada), spans dos erros de plugin (`Args::span` em vez de
  `Span::detached()`) e alinhamento da feature `wat` do wasmi (texto do
  erro de parse WASM). Achado #6 de P810. Ver §"P819" abaixo — **a sonda
  foi medida antes de decidir** (ADR-0108) e a proveniência está registada.

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

Em `01_core/src/compiler/stdlib/plugin.rs` (estado P698, HEAD `b241f1525`),
há exactamente:

- `1 função pública` registada no escopo: `plugin` (nome exposto à
  linguagem), via `scope.define_func("plugin", native_plugin)` em
  `01_core/src/compiler/eval/mod.rs:1050`.
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

**P1141, condicionado ao gate:** a assinatura passa a aceitar também
`plugin(path("..."))`. Medição prévia: o vanilla usa `DataSource`, cuja forma
path é `PathOrStr` (`loading/mod.rs:46-63`), e a sonda P1141 demonstra que o
valor deve preservar sua base cross-file. `Value::Path` usa
`World::read_path`; `Value::Str` resolve uma vez no caller e depois lê pelo
mesmo caminho enraizado; `Bytes` permanece direto. O erro de cast passa a
enumerar `path, string, or bytes`. Nenhuma regra do host/cache/WASM muda.
Não implementar nem ressellar antes da confirmação ADR-0127.

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
existentes — **nenhuma alteração ao import** (ver `prompts/compiler/import.md` e
`01_core/src/compiler/eval/modules.rs:167-219`, eval.md §P683):

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

## P819 — `plugin.transition`, mensagens L1 e spans (achado #6 de P810)

### Sonda (medição ANTES — ADR-0108, proveniência registada)

**Proveniência** (regra de proveniência de medição, 2026-07-05):

- Commit: `2acc14eac` (HEAD) + **working tree não commitado** —
  `git diff HEAD --stat`: 69 ficheiros alterados, +5074/−430 (medido a
  2026-07-22 ~07:05 UTC).
- Binários: `./target/release/typst` (cristalino, build 2026-07-22 03:58) e
  `lab/typst-original/target/release/typst` (vanilla 0.15.0, build
  2026-06-29).
- Fixtures: `temp/p810/plugin/{t0..t13}*.typ`, `hello.wasm`, `no-memory.wasm`,
  `garbage.wasm` (de P810); `temp/p819/tm*.typ` e `temp/p819/mut.wasm`
  (345 B — plugin mutável com exports `add`/`get`, compilado de
  `temp/p819/mut.wat` via crate `wat` 1.246.2, scratch tool
  `temp/p819/wat2wasm`; o wasmi do vanilla **não** tem a feature `wat`,
  logo o fixture tem de ser binário).
- Comandos: vanilla `typst compile <t>.typ /tmp/v.pdf`; cristalino
  `typst <t>.typ /tmp/c.pdf` (**sem** subcomando `compile` — a CLI
  cristalina não o tem).
- Baseline de testes: `cargo test -p typst-core` → **4487 passed; 0 failed;
  2 ignored** (mesmo estado de working tree acima).

**Caso 1 — `plugin.transition` ausente.** `temp/p810/plugin/t10-transition.typ`
(`#let m = plugin.transition(p.hello)`):

```text
vanilla:     exit 0 (devolve módulo derivado; `type(plugin.transition)` = `function` — temp/p819/tm8)
cristalino:  t10-transition.typ:2:9: error: cannot access fields on type function   (exit 1)
```

Origem: `plugin` é registado como `Func::native` **sem namespace**
(`01_core/src/compiler/eval/mod.rs:1559`); field access em `Func` sem
namespace cai em `bindings.rs:1701-1704`. No vanilla, `transition` é
`#[func]` dentro de `#[scope] impl plugin`
(`lab/typst-original/crates/typst-library/src/foundations/plugin.rs:158-202`).

**Caso 2 — mensagens L1 divergentes** (saída literal, `<detached>` = span):

| Caso | vanilla | cristalino |
|------|---------|------------|
| t2 ficheiro inexistente | `error: file not found (searched at /home/…/temp/p810/plugin/nao-existe.wasm)` @ `1:8` | `plugin(): não foi possível ler 'nao-existe.wasm': erro ao ler 'nao-existe.wasm': No such file or directory (os error 2)` @ `<detached>` |
| t5 tipo do argumento | `error: expected path, string, or bytes, found integer` @ `1:8` | `plugin() requer caminho (str) ou bytes, recebeu int` @ `<detached>` |
| t6 arg não-bytes | `error: expected bytes, found integer` @ `2:9` | `plugin function arguments must be bytes, found int` @ `<detached>` |

**Caso 3 — spans `<detached>` em todo o caminho plugin.** Mesmo nos erros
com mensagem já verbatim (t4 `plugin does not export its memory`, t7/t7b
`plugin function takes 0 arguments, but 1 was given`, t8 `plugin errored
with: This is an `Err``, t9 `plugin panicked: wasm `unreachable`
instruction executed`), o vanilla aponta span (argumento fonte — t2/t5;
argumento ofensor — t6; chamada inteira — t7/t8/t9/t11) e o cristalino
emite `<detached>` em 100% dos casos. Origem: helper `err()` com
`Span::detached()` (`01_core/src/compiler/stdlib/plugin.rs:25-27`) e três
`Span::detached()` em `call_plugin`
(`01_core/src/compiler/eval/closures.rs:177,188,201`).

**Caso 4 — texto do erro de parse WASM diverge** (t3 `garbage.wasm`):

```text
vanilla:     failed to load WebAssembly module (magic header not detected: bad magic number - expected=[0x0,0x61,0x73,0x6d,] actual=[0x69,0x73,0x74,0x6f,] (at offset 0x0))
cristalino:  failed to load WebAssembly module (expected `(`  --> <anon>:1:1 | isto nao e um modulo wasm de certeza | ^)
```

Causa medida (não inferida): mesmo wasmi `1.0.9`, **features diferentes** —
o vanilla compila wasmi com `default-features = false, features = ["simd"]`
(`lab/typst-original/Cargo.toml.original:149`); o cristalino omite
`default-features = false` (`03_infra/Cargo.toml:39`), activando a feature
default `wat`, que tenta parsear o ficheiro como WAT. **Isto refuta a
inferência marcada no item 2 do catálogo de `prompts/infra/plugin_host.md`
("o texto tende a coincidir")** — medida em P810/P819, a refutação
proposta é alinhar as features.

**Medições vanilla de `plugin.transition`** (fixtures `temp/p819/tm*.typ`
com `mut.wasm` — comando `typst compile tmX.typ`, saída literal):

- **Semântica** (tm1): `#let mutated = plugin.transition(base.add, bytes("hello"))`
  ⇒ `base.get()` = `""` (base **inalterado**), `mutated.get()` = `"hello"`
  (PDF: `hello`). (tm9) transições encadeadas acumulam: `m1.get()`=`"a"`,
  `m2.get()`=`"ab"` (PDF: `a ab`). (tm12) chamadas após transition: exit 0.
- tm2 arg `str`: `error: expected bytes, found string` @ span do argumento.
- tm3 sem args: `error: missing argument: func` @ span da chamada.
- tm4 arg `int`: `error: expected function, found integer` @ span do arg.
- tm5 função nativa: `error: expected plugin function` @ span do arg
  (cast de `PluginFunc` — `plugin.rs:234-238`).
- tm6 arg não-bytes: `error: expected bytes, found integer` @ span do arg.
- tm7 named `func:`: `error: the argument `func` is positional` + hint
  `try removing `func:`` (routing genérico de args do vanilla — ver
  scope-out transversal).
- tm11 aridade na chamada da transição: `error: plugin function takes 1
  argument, but 2 were given` @ span da chamada inteira (a chamada de
  transição corre a função do plugin — mesmas mensagens do catálogo de
  `infra/plugin_host.md`).

### Especificação — `plugin.transition` (linguagem)

```typst
plugin.transition(func, ..args) -> module
```

- `func`: **obrigatório**, primeiro posicional. Tem de ser uma função de
  plugin (`FuncRepr::Plugin`). Erros (verbatim medidos acima):
  - ausente ⇒ `missing argument: func`;
  - não-função ⇒ `expected function, found {tipo}` (nome longo do tipo:
    `integer`, `string`, … — o cristalino já tem
    `bindings::long_type_name`, usado em P815);
  - função não-plugin ⇒ `expected plugin function`.
- `..args`: variádico, todos `bytes`; qualquer outro tipo ⇒
  `expected bytes, found {tipo}`.
- Named args: rejeitados (routing genérico cristalino — ver scope-out
  transversal; a mensagem vanilla medida é `the argument `func` is
  positional` + hint, mas o routing de named args é transversal a todas as
  nativas e fica fora de P819).
- **Semântica** (réplica de `Plugin::transition`, `plugin.rs:328-352`):
  executa a chamada (mutável) sobre uma instância, faz **snapshot da
  memória** (páginas + dados — `plugin.rs:420-426,522-545`) e devolve um
  `Module` **derivado** cujas funções observam a mutação; o módulo original
  fica inalterado. A limitação do vanilla mantém-se (é paridade, não bug):
  o snapshot cobre só a memória linear, **não** globals WASM
  (documentado no vanilla, `plugin.rs:177-182`). Erros da chamada
  (aridade, trap, `errored with`, etc.) são os do catálogo de
  `infra/plugin_host.md`, propagados verbatim.
- **Memoização**: `PluginFunc::transition` é `#[comemo::memoize]` como o
  vanilla (`plugin.rs:226-231`) — duas transições idênticas
  `(module, name, args)` chamam o host uma vez e devolvem o mesmo módulo
  derivado. Ver `prompts/entities/plugin_func.md`.
- **Registo na linguagem**: `plugin` passa a ser registado com namespace —
  `Func::native_with_namespace("plugin", native_plugin, ns)` (P493,
  precedente `table.header`, `entities/func.rs:159-173`) — com
  `transition` no namespace. `type(plugin.transition)` = `function`;
  field access desconhecido em `plugin` ⇒ `function does not contain
  field "{field}"` (mensagem existente de `bindings.rs:1698`).

### Especificação — mensagens L1 alinhadas (verbatim medido)

1. **Ficheiro inexistente** (t2): `file not found (searched at {path-absoluto-resolvido})`.
   - A resolução do path vive em L3 (`SystemWorld::resolve_path`,
     `03_infra/src/world.rs:471-479`): o formato `erro ao ler '{path}': {e}`
     passa a `file not found (searched at {full_path})` para
     `ErrorKind::NotFound` (demais kinds mapeados à la `FileError::from_io`
     do vanilla, `diag.rs:631-644`).
   - `native_plugin` **propaga verbatim** (deixa de prefixar
     `plugin(): não foi possível ler '{path}':`).
   - Único produtor da mensagem antiga: `03_infra/src/world.rs:478`; única
     asserção L1 afectada: o teste `plugin_caminho_inexistente_erro_de_leitura`
     (será actualizado). Os restantes consumidores de `read_bytes`
     (`loading.rs`, `figure_image.rs`, `visualize.rs`, `bibliography.rs`)
     **mantêm** os seus prefixos PT neste passo (scope-out transversal).
2. **Tipo do argumento de `plugin()`** (t5): `expected path, string, or bytes, found {tipo}`
   (nome longo). Substitui `plugin() requer caminho (str) ou bytes, recebeu {tipo}`.
   ("path" consta da mensagem do vanilla porque `DataSource` aceita path —
   é parte do texto verbatim, não uma feature nova.)
3. **Argumento não-bytes na chamada** (t6): `expected bytes, found {tipo}`
   (nome longo). Substitui `plugin function arguments must be bytes, found {tipo}`
   em `call_plugin`.

As mensagens já verbatim (t4, t7, t8, t9 e catálogo de
`infra/plugin_host.md`) **não mudam**.

### Especificação — spans dos erros de plugin

Todos os erros produzidos por `native_plugin`, `native_plugin_transition` e
`call_plugin` passam a usar **`args.span`** (P772s,
`entities/args.rs:23-28` — span da lista de argumentos da chamada real) em
vez de `Span::detached()`. `Args::span` já chega aos dois sítios
(`native_plugin(ctx, args, …)` e `call_plugin(p, args)`).

**`parse_anchored` (P814) NÃO se aplica aqui** — registo explícito porque o
passo o levantou: `parse_anchored` sintetiza spans ao **parsear strings**
avaliadas (`#eval`); os erros de plugin não nascem de um parse — o span
correcto já existe no callsite e chega via `Args::span`.

**Nuance registada** (mesma classe do nuance de P814): o vanilla aponta ao
**argumento ofensor** (t2/t5/t6) ou à **chamada inteira incluindo o callee**
(t7/t8/t9/t11); o cristalino aponta à **lista de argumentos** (`(…)`),
porque `Args::span` é por chamada, não por argumento (P772s). A linha e a
coluna inicial coincidem ou ficam adjacentes; o sublinhado exacto não é
paridade — aceite como desvio conhecido, a registar no relatório de P819.

### Especificação — feature `wat` do wasmi (L3)

`03_infra/Cargo.toml:39`: `wasmi = { version = "=1.0.9",
default-features = false, features = ["simd"] }` — alinha com o vanilla
(`Cargo.toml.original:149`) e faz o erro de parse WASM bater verbatim
(caso 4). Risco registado: se algum teste de L3 depende da feature `wat`
(parsear WAT texto), falha — verificar na implementação (o encoder de
testes de `03_infra/src/plugin_host.rs` gera WASM binário, não WAT).

### L0s relacionados (alterados neste passo, mesmo hash-fix)

- `prompts/contracts/plugin_host.md` — trait ganha `transition`.
- `prompts/entities/plugin_func.md` — `PluginFunc::transition` memoizada.
- `prompts/infra/plugin_host.md` — implementação `transition`
  (snapshot/restore) + alinhamento da feature `wat` + item 2 do catálogo
  (inferência **refutada** → agora especificado).

## Erros (observáveis — ADR-0108)

- Ausência de host: `error: plugins não suportados neste World`.
- Aridade errada / named args: mensagens do helper `reject_named` /
  validação de aridade existentes (P697).
- Tipo errado do arg (P819): `error: expected path, string, or bytes, found <tipo>`
  (nome longo do tipo) — substitui o texto PT de P697 (ver §P819; o texto
  anterior desta linha já não correspondia ao código).
- Ficheiro inexistente (P819): `file not found (searched at <path-absoluto>)`,
  propagado verbatim de L3 (ver §P819).
- Arg não-bytes na chamada (P819): `error: expected bytes, found <tipo>`.
- `plugin.transition` (P819): `missing argument: func`;
  `expected function, found <tipo>`; `expected plugin function`;
  `expected bytes, found <tipo>`.
- `host.load` / `host.exports` / `host.call` / `host.transition`:
  `PluginError.message` propagada verbatim via
  `SourceDiagnostic::error(span, message)`.

Desde P819, todos os erros são produzidos com `args.span` (P772s — span da
lista de argumentos do callsite; nuance registada em §P819). Antes de P819
eram `Span::detached()` — o texto anterior desta secção ("span do
callsite") estava em deriva face ao código (medido em P810/P819).

## Testes

Os testes vivem em `01_core/src/compiler/stdlib/plugin.rs` (unidade) e em
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

Cobertura mínima nova (P819):

- **`plugin.transition` caminho feliz:** mock host com `transition` ⇒
  módulo derivado com os mesmos exports; o módulo original inalterado.
- **Cache de transition:** duas `plugin.transition(f, b"x")` idênticas ⇒
  `host.transition` chamado uma vez (memoize).
- **Erros verbatim:** `plugin.transition()` ⇒ `missing argument: func`;
  `plugin.transition(42)` ⇒ `expected function, found integer`;
  `plugin.transition(<fn nativa>)` ⇒ `expected plugin function`;
  `plugin.transition(f, 1)` ⇒ `expected bytes, found integer`.
- **Mensagens P819:** `plugin()` com tipo errado ⇒
  `expected path, string, or bytes, found integer`; leitura falhada ⇒
  mensagem propagada verbatim sem prefixo PT; `p.f(1)` ⇒
  `expected bytes, found integer`.
- **Spans:** erro de `plugin()` e de `p.f(...)` ⇒ diagnóstico com span ==
  `args.span` (não detached).
- **e2e (04_wiring, host wasmi real):** `plugin.transition` com plugin
  mutável (encoder de teste) — base inalterado, derivado observa a mutação;
  erro de parse WASM bate verbatim com o vanilla (feature `wat` off).
- Baseline registada: `cargo test -p typst-core` = 4487 passed (proveniência
  em §P819); o passo seguinte reporta antes/depois com os testes novos.

## Fora de escopo (P699)

- Encoders WASM e limites de combustível/memória: mecânica L3
  (`prompts/infra/plugin_host.md`).
- Instanciação do host real e encadeamento no `SystemWorld`/CLI: L3/L4
  (`prompts/infra/system-world.md`, `prompts/infra/plugin_host.md`).
- Mudança ao import (`#import ... : ...`): já funciona (P683), sem toque.
- Remover `native_plugin` do registo `Native`: mantém-se (P697), apenas
  deixa de devolver valor provisório.

## Fora de escopo (P819) — medido e registado

- **Routing de named args** (t13/tm7): vanilla `the argument `source` is
  positional` + hint `try removing `source:`` vs cristalino `argumento
  nomeado inesperado em plugin(): 'source'`. O routing de named args é
  **transversal** a todas as nativas (helper PT por módulo); alinhá-lo é
  passo dedicado, não plugin-específico.
- **Mensagem de field inexistente em módulo** (t12): vanilla `module does
  not contain `nao_existe`` (módulo anónimo) vs cristalino `module 'plugin'
  does not contain field "nao_existe"` (módulo nomeado). Transversal ao
  acesso a campos de módulo (`bindings.rs:1733-1738`) e à decisão
  `Module::new("plugin", …)` vs `Module::anonymous` do vanilla.
- **Span por argumento:** o vanilla aponta ao argumento ofensor; o
  cristalino à lista de argumentos (P772s). Desvio aceite (§P819).
- **Prefixos PT dos outros consumidores de `read_bytes`** (`read()`,
  `image()`, `tiling()`, bibliografia): mantêm-se neste passo; o alinhamento
  verbatim dessas mensagens é transversal (cruza com os achados #10/#11 de
  P810).
- **`plugin(bytes)` não testável por documento:** construtor `bytes()` e
  `read(encoding:)` ausentes (achado transversal de P810) — a variante
  bytes só é exercitável em testes Rust.

## Critério de aceitação

No nível da **linguagem** (ADR-0107), não da mecânica:

- `plugin("f.wasm")` e `plugin(bytes)` devolvem `module`.
- `module.export(bytes)` chama a função WASM e devolve `bytes`.
- `#import plugin("f.wasm"): export` expõe a função ao escopo.
- Args não-bytes e ausência de host produzem os erros especificados.
- Duas chamadas idênticas batem no cache (host chamado uma vez).
- `cargo test --workspace` sem regressão; `crystalline-lint .` com
  zero violations.

Critérios novos (P819), todos com comando + saída literal no relatório:

- `plugin.transition(base.add, bytes("x"))` devolve módulo derivado que
  observa a mutação; o base não (`tm1`/`tm9` de `temp/p819/`).
- Os erros de `plugin.transition` batem verbatim com o vanilla
  (tm3–tm6); `type(plugin.transition)` = `function`.
- t2/t5/t6 batem verbatim com o vanilla (mensagens §P819).
- t3 (parse WASM) bate verbatim com o vanilla após
  `default-features = false`.
- Nenhum erro do caminho plugin emite `<detached>` (span = `args.span`;
  nuance do sublinhado registada).
- `cargo test --workspace` sem regressão; `crystalline-lint .` com zero
  violations.

---

**Nota de hash (P819):** este L0 foi alterado na fase de sonda/L0 do passo;
os `@prompt-hash` dos ficheiros de código afectados
(`01_core/src/compiler/stdlib/plugin.rs`, `01_core/src/contracts/plugin_host.rs`,
`01_core/src/entities/plugin_func.rs`, `03_infra/src/plugin_host.rs` e
demais tocados na implementação) serão recalculados **pelo humano** via
`crystalline-lint --fix-hashes .` após confirmação deste L0 — não correr
`--fix-hashes` antes disso.
