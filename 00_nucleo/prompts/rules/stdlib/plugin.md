# Prompt L0 — `rules/stdlib/plugin` — builtin `plugin()` (nível 2 de P696)
Hash do Código: 9b34a5db

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/plugin.rs`
**Passo de origem**: P697 (primeiro passo da divisão proposta em P696)
**ADRs relevantes**: ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes
de decidir), ADR-0109 (cada feature no seu ficheiro), ADR-0111 (leitura reusa
`World::read_bytes`, decode/lógica em L1)

---

## Contexto e objectivo

`plugin(source) -> Module` é o builtin que carrega um plugin WebAssembly
(sonda completa em P696). Este passo implementa **só o nível 2** do mapa de
P696: reconhecer `plugin(...)` como builtin e **ler os bytes** do `source`,
reaproveitando a infra de leitura já existente (`World::read_bytes`, a mesma de
`#include`/`#import`/`read`, P679/P686/ADR-0111). O **runtime WASM** (nível 3)
fica para **P698** — por isso, depois de ler os bytes com sucesso, o builtin
devolve um **erro provisório claro** (em vez de um `Module` funcional), provando
que a leitura funciona sem fingir que o resto está pronto.

## Assinatura

```rust
pub fn native_plugin(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn World,
    current_file: FileId,
) -> SourceResult<Value>
```

`plugin(source)` aceita **um** argumento posicional, sem nomeados:

- `Value::Str(path)` — caminho do ficheiro `.wasm`; lido via
  `world.read_bytes(current_file, path)` (resolve relativo ao ficheiro actual e
  absoluto `/...` à raiz do pacote/projecto, P686 — o mesmo caminho de `read()`).
- `Value::Bytes(b)` — conteúdo WASM directo (já disponível; sem leitura).

Qualquer outro tipo/aridade → erro de argumento (`plugin() requer caminho (str)
ou bytes, recebeu <tipo>` / `plugin() requer 1 argumento, recebeu N`).

## Decisão: erro provisório em vez de `Module`

Depois de obter os bytes (por leitura ou directos), `native_plugin` devolve:

```
Err("plugin: runtime WASM ainda não implementado (P697); {n} bytes lidos com sucesso")
```

Isto mantém a **sintaxe** e a **leitura** correctas e testáveis, e separa
claramente três situações observáveis (paridade de mensagem = mecânica
observável, ADR-0107):

| Situação | Mensagem (cristalino) |
|----------|------------------------|
| `plugin` não existia (antes de P697) | `unknown variable: plugin` |
| Ficheiro inexistente | `plugin(): não foi possível ler 'X': <motivo>` (mesma forma de `read()`/`#import`) |
| Ficheiro lido com sucesso | `plugin: runtime WASM ainda não implementado (P697); {n} bytes lidos com sucesso` |

O erro de ficheiro inexistente reusa a **forma** do helper `read_bytes` de
`loading.rs` (`"{fname}(): não foi possível ler '{path}': {msg}"`), para que o
diagnóstico seja do mesmo tipo já usado por `#include`/`#import`/`read`.

## Scope-out (próximos passos)

- **P698** — host `wasmi` em L3 atrás de trait `PluginHost`; protocolo `typst_env`
  completo; `native_plugin` passa a devolver `Module` real em vez do erro
  provisório.
- **P699** — `Module` de `PluginFunc` + `#[comemo::memoize]` + E2E.
- **P700+** — validação `cetz`; transition API (só se necessário).

Nada disto é implementado aqui. O `Value` de retorno em P697 é sempre erro.

## Língua vs mecânica (ADR-0107)

| Aspecto | Classificação | Paridade? |
|---------|---------------|-----------|
| `plugin(path\|bytes)` é builtin chamável | semântica/sintaxe | sim (a partir deste passo) |
| leitura relativa/`/...` igual a `read()`/`#import` | semântica (resolução) | sim |
| erro de ficheiro inexistente na forma de `read()` | mensagem (mecânica observável) | sim |
| devolver `Module` funcional | semântica | **não** (P698) — erro provisório |

## Critérios de verificação

- `native_plugin` com `Value::Bytes` devolve o erro provisório com a contagem
  correcta de bytes (sem I/O).
- `native_plugin` com `Value::Str` e `MockWorld` que serve o ficheiro devolve o
  erro provisório (leitura OK); com ficheiro ausente devolve
  `plugin(): não foi possível ler …`.
- Header `@prompt 00_nucleo/prompts/rules/stdlib/plugin.md` e `@prompt-hash`
  correcto (via `crystalline-lint --fix-hashes .`).
- `crystalline-lint .` zero violations (sem V3/V4/V14; L1 não faz I/O — reusa
  `World::read_bytes`).
