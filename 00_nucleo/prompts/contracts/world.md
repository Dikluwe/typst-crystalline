# Prompt L0 — `contracts/world` — O Contrato Supremo do Sistema
Hash do Código: 15212195

**Camada**: L1
**Ficheiro alvo**: `01_core/src/contracts/world.rs`
**Passo de origem**: Passo 1 (trait base), expandido em Passos 8, 15, 22
**ADRs relevantes**: ADR-0005 (padrão B3: World/TrackedWorld), ADR-0001 (comemo em L1),
                     ADR-0019 (ttf-parser → L3)

---

## Contexto e Objetivo

O `World` é o **trait supremo** do motor Cristalino — a fronteira arquitetural
entre o núcleo puro e determinístico (L1) e o sistema físico externo (L3).

O L1 **nunca realiza I/O directamente**. Precisa de fontes, ficheiros e a
data actual? Faz perguntas ao `World`. Quem implementa `World` (o `SystemWorld`
em L3) decide como obter esses dados — do disco, da rede, da memória ou de um
mock.

**Separação de `contracts/mod.rs`**: `mod.rs` é o agregador/facade (declara
`pub mod world`). Este ficheiro (`world.rs`) define o **contrato em si** — o
`trait World` e o `MockWorld` para testes.

---

## O Tipo `SysInputs` (P694)

Pares `chave → valor` passados à CLI via `--input chave=valor` e expostos à
linguagem em `sys.inputs`. Vive neste contrato porque atravessa a fronteira
ambiente→núcleo: é produzido em L2/L3 e consumido em L1.

```rust
pub type SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>;
```

`IndexMap` preserva a ordem de inserção (repr determinista); `FxBuildHasher` é
o hasher de `Value::Dict`; `EcoString` dá clone O(1). Os três crates estão em
`[l1_allowed_external]`, logo `SysInputs` na assinatura do trait não dispara
V14. Os valores são **sempre strings** (paridade vanilla: `--input n=42` →
`sys.inputs.n == "42"`).

## O Trait `World`

```rust
pub trait World: Send + Sync {
    /// A biblioteca de funções e valores padrão do Typst.
    /// Inclui as funções nativas registadas pela stdlib.
    fn library(&self) -> &Library;

    /// O catálogo de fontes disponíveis (nome, variante, índice).
    /// Usado pelo Layouter para resolver pedidos de fonte.
    fn book(&self) -> &FontBook;

    /// O ficheiro principal a compilar (ponto de entrada).
    fn main(&self) -> FileId;

    /// Obter o código-fonte de um ficheiro pelo seu id.
    /// Retorna FileError::NotFound se o ficheiro não existe.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Obter o conteúdo binário de um ficheiro (ex: imagens, fontes .ttf).
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Ler ficheiro binário por caminho relativo à raiz do projecto (Passo 71).
    /// Retorna Arc<Vec<u8>> partilhado — clone O(1) no AST.
    /// Implementação por omissão retorna Err — MockWorlds mínimos não precisam de I/O.
    fn read_bytes(&self, path: &str) -> Result<std::sync::Arc<Vec<u8>>, String> {
        Err(format!("leitura de ficheiro por caminho não suportada: {}", path))
    }

    /// Resolver um `PackageSpec` (`@preview/nome:versao`) para o `Source` do
    /// entrypoint do pacote, procurando na cache local (P681, P-β de P678).
    /// Implementação por omissão retorna Err — mocks e worlds sem filesystem
    /// não resolvem pacotes. A resolução real (data dir → cache dir, manifesto
    /// `typst.toml`, `entrypoint`) vive em L3 (`SystemWorld`); L1 só declara o
    /// contrato (nunca faz I/O). `spec` e `Source` são tipos L1 — nenhum tipo
    /// externo atravessa esta fronteira (V14).
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String> {
        let _ = spec;
        Err("resolução de pacotes não suportada nesta implementação de World".into())
    }

    /// Pares `--input chave=valor` da CLI para expor em `sys.inputs` (P694).
    /// Implementação por omissão retorna vazio — MockWorlds e worlds sem CLI
    /// não precisam de implementar este método (espelha `read_bytes`/
    /// `include_source`/`resolve_package`). Devolve owned (clone barato —
    /// poucos pares); L1 lê-o em `eval_with_full_error` para construir `sys`.
    fn inputs(&self) -> SysInputs {
        SysInputs::default()
    }

    /// **P699** — host de plugins WASM, se o ambiente o fornecer. `None` por
    /// omissão — MockWorlds e worlds sem runtime WASM não implementam este
    /// método; `native_plugin` devolve então erro claro ("plugins não suportados
    /// neste World"). `SystemWorld` (L3) devolve `Some(Arc<WasmiPluginHost>)`.
    /// `Arc` é L1-legal (ADR-0029); `PluginHost` é tipo L1 — sem externo na
    /// fronteira (V14). `PluginHost: Send + Sync` mantém `SystemWorld: Send + Sync`.
    fn plugin_host(&self) -> Option<std::sync::Arc<dyn crate::contracts::plugin_host::PluginHost>> {
        None
    }

    /// Obter uma fonte (bytes + metadados) pelo índice no FontBook.
    /// None se o índice está fora dos limites.
    fn font(&self, index: usize) -> Option<Font>;

    /// A data actual com offset UTC em horas (None se indisponível).
    /// Usa i64 em vez de Duration — o tipo Duration do Typst não existe em L1.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}
```

### Por que `Send + Sync`?

O compilador Typst suporta compilação paralela (Passos futuros). O `World`
deve ser passável entre threads sem `Arc<Mutex<>>` adicional.

---

## Padrão B3: `World` vs `TrackedWorld` (ADR-0005)

```
World (este trait) → o contrato puro: "o quê"
TrackedWorld       → adiciona rastreio incremental via comemo: "como"
```

- `World`: implementado por `SystemWorld` (L3) e `MockWorld` (testes)
- `TrackedWorld`: gerado pelo macro `#[comemo::track]` sobre `World`
- O `eval()` recebe `Tracked<dyn TrackedWorld>` — garante memoização
- O `World` puro compila **sem importar comemo** — confirmado pelo teste
  `world_pure_no_comemo_import_needed()`

---

## MockWorld nos Testes

```rust
struct MockWorld {
    library: Library,
    book:    FontBook,
    main_id: FileId,
}

impl World for MockWorld {
    fn library(&self) -> &Library  { &self.library }
    fn book(&self)    -> &FontBook { &self.book }
    fn main(&self)    -> FileId    { self.main_id }
    fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
    fn file(&self, _: FileId)   -> FileResult<Bytes>  { Err(FileError::NotFound) }
    fn font(&self, _: usize)    -> Option<Font>       { None }
    fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
}
```

O `MockWorld` mínimo — sem I/O, sem fontes reais — permite testar todo o
pipeline de eval sem dependências externas.

---

## Invariantes Críticos

| Regra | Consequência da violação |
|-------|--------------------------|
| L1 nunca importa `std::fs`, `std::net` etc | Compilação falha (import proibido) |
| L1 nunca chama I/O directamente | Teste sem mock quebra |
| `world.source(id)` pode falhar | Caller DEVE tratar `FileResult` |
| `world.resolve_package(spec)` pode falhar | Caller DEVE tratar `Err` (pacote ausente / não suportado); resolução real é I/O em L3 |
| `world.inputs()` tem default vazio | MockWorlds vêem `sys.inputs == (:)` sem implementar o método (P694) |
| `world.font(idx)` pode retornar `None` | Layouter DEVE ter fallback de fonte |
| `today(None)` significa "sem offset" | `today(Some(0))` = UTC exacto |

---

## Critérios de Verificação

```
// Compilabilidade sem comemo
// world.rs não contém: use comemo; / extern crate comemo;
grep "comemo" 01_core/src/contracts/world.rs = vazio

// MockWorld implementa World correctamente
let w = mock()
World::main(&w) = FileId::from_raw(NonZeroU16::new(1).unwrap())
World::source(&w, FileId::from_raw(2)) = Err(FileError::NotFound)
World::file(&w, FileId::from_raw(2))   = Err(FileError::NotFound)
World::font(&w, 0)                     = None
World::today(&w, None)                 = None
World::today(&w, Some(2))              = None

// Trait bounds
// World: Send + Sync — verificado por compilação com mock em thread
std::thread::spawn(|| {
    let w: &dyn World = &mock();
    let _ = w.main();
}).join().unwrap();

// library() retorna Library válida
World::library(&mock()) // não panic

// book() retorna FontBook válido
World::book(&mock()).len() = 0  // mock começa sem fontes
```

---

## Relação com Outros Módulos

| Módulo | Como usa `World` |
|--------|-----------------|
| `rules/eval.rs` (L1) | Recebe `Tracked<dyn TrackedWorld>` — acessa `source()` |
| `contracts/mod.rs` (L1) | Agrega via `pub mod world` |
| `03_infra/src/world.rs` (L3) | Implementa `World` com I/O real (`SystemWorld`) |
| `03_infra/src/integration_tests.rs` (L3) | Cria mocks — não usa `MockWorld` de L1 |

---

## Resolução de caminhos absolutos (`/...`) em `include_source` e `read_bytes` (P686)

`include_source(current_file, path)` e `read_bytes(current_file, path)` resolvem
`path` conforme a sua forma (implementação em L3 — `SystemWorld`):

- Se `path` começa por `/`, é **absoluto**: resolve-se relativamente à **raiz do
  pacote** quando `current_file` pertence a um pacote, ou à **raiz do projecto**
  (`World::root()`) caso contrário. A raiz do pacote é o directório
  `{namespace}/{name}/{version}` que contém `current_file`.
- Caso contrário, `path` é **relativo** e resolve-se relativamente ao directório
  de `current_file` (comportamento pré-existente, sem regressão).

Esta semântica iguala a do compilador vanilla e desbloqueia pacotes (ex.: `cetz`)
que importam módulos internos via `/src/...`.

**Língua vs mecânica (ADR-0107):** a distinção absoluto/relativo e a noção de
raiz de pacote fazem parte da **semântica** da linguagem (resolução de módulos);
a estrutura em disco (`{ns}/{name}/{version}`) é mecânica de L3 e diverge de
propósito.

