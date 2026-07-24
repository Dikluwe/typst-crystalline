# Prompt L0 — infra/system-world
Hash do Código: 9e7769e8

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/world.rs`
**Atualizado em**: 2026-07-23 (P876 — cache de bytes de `read_bytes` por `FileId`)
**ADRs relevantes**: ADR-0001 (comemo/TrackedWorld), ADR-0005 (World trait), ADR-0017 (stubs)

## Contexto

`SystemWorld` é a implementação concreta de `World` (L1) para o
filesystem real. Vive em L3 porque lê ficheiros do disco (`std::fs`)
e terá I/O de fontes (Passo 8) e rede (Passo 9+).

O `World` trait original em `typst-cli` usa `LazyHash<Library>`,
`FontStore`, `FileStore`, `typst-kit` — todos evitados neste passo.
A implementação mínima usa `std::fs` directamente.

## Interface pública

```rust
pub struct SystemWorld { ... }

impl SystemWorld {
    pub fn new(root: PathBuf, main: PathBuf) -> Result<Self, SystemWorldError>
    /// **P450** — carrega e parseia um ficheiro `.bib` relativo a current_file.
    pub fn load_bibliography(&self, current_file: FileId, path: &str) -> Result<Vec<BibEntry>, String>
    /// **P694** — builder: associa os pares `--input` (raw strings) e converte
    /// para `SysInputs` (`EcoString`). Guarda no campo `inputs`.
    pub fn with_inputs(self, inputs: Vec<(String, String)>) -> Self
    /// **P699** — builder: instala o host de plugins WASM (L3) no
    /// `SystemWorld`. Consumido por `World::plugin_host()` (devolve
    /// `Some(...)`). Encadeado em `04_wiring/src/main.rs` com
    /// `Arc::new(typst_infra::plugin_host::WasmiPluginHost::new())`.
    pub fn with_plugin_host(self, host: Arc<dyn PluginHost>) -> Self
    /// **P772b** — devolve o path registado para `id`, se existir.
    /// Usado pelo formatter de diagnósticos de L4 para mostrar o
    /// ficheiro correcto em spans cross-file.
    pub fn path_of(&self, id: FileId) -> Option<PathBuf>
}

impl World for SystemWorld {
    fn library(&self) -> &Library;
    fn book(&self)    -> &FontBook;
    fn main(&self)    -> FileId;
    fn source(&self, id: FileId) -> FileResult<Source>;
    fn file(&self, id: FileId)   -> FileResult<Bytes>;
    fn font(&self, index: usize) -> Option<Font>;
    /// **P880** — cobertura Unicode lazy com cache por slot.
    fn candidates_for_char(&self, c: char) -> Vec<usize>;
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
    fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String>;
    /// **P694** — devolve os `--input` (clone barato; poucos pares).
    fn inputs(&self) -> SysInputs;
    /// **P699** — devolve `Some(Arc<WasmiPluginHost>)` (host WASM em L3).
    fn plugin_host(&self) -> Option<Arc<dyn PluginHost>>;
}
```

**Campo `inputs` (P694):** `SysInputs` inicializado vazio em `new` e populado
por `with_inputs`. É lido por `eval_with_full_error` via `World::inputs()` para
construir o módulo `sys`. A conversão `String → EcoString` acontece aqui (L3),
mantendo L2 livre de `ecow`/`indexmap`.

## Comportamento

- `library()` — retorna stub `Library(())`
- `book()` — retorna stub `FontBook(())`
- `main()` — `FileId` do ficheiro principal
- `source(id)` — lê do cache ou do disco via `std::fs::read_to_string`;
  retorna `FileError::NotFound` se não existir
- `file(id)` — lê bytes brutos via `std::fs::read`;
  retorna `FileError::NotFound` se não existir
- `font(_)` — `None` (stub — fontes reais no Passo 8)
- `today(_)` — `None` (stub — Datetime real após ADR-0017)

## Resolução de pacotes (P681, P-β de P678, P763)

`resolve_package(spec)` procura o pacote primeiro **offline**, sem rede, na cache local do
utilizador, seguindo a ordem de prioridade confirmada em P678:

1. **Data dir** — `$XDG_DATA_HOME/typst/packages` ou `~/.local/share/typst/packages`.
2. **Cache dir** — `$XDG_CACHE_HOME/typst/packages` ou `~/.cache/typst/packages`.

Quando o pacote não é encontrado em nenhuma das bases acima e `spec.namespace == "preview"`,
o `SystemWorld` delega o download ao mecanismo definido em
[`infra/package_downloader.md`](./package_downloader.md) (P763). Até à implementação desse
download, o erro mantém a mensagem offline.

Para cada base, o candidato é `{base}/{namespace}/{name}/{version}` (ex.:
`~/.cache/typst/packages/preview/cetz/0.2.2`). O primeiro que existir como
directório é usado:

1. Lê `{candidato}/typst.toml` e extrai `[package].entrypoint` (via `toml::Value`;
   `toml` é workspace dep adicionada a `03_infra/Cargo.toml`). Sem `entrypoint`
   → erro claro.
2. `entrypoint_path = candidato.join(entrypoint)`; `id = register_file(entrypoint_path)`
   (FileId estável, canonicalizado); `source(id)` devolve o `Source` do entrypoint.
   Imports internos do pacote resolvem relativamente ao `current_file` (entrypoint),
   cujo `directory_of` é o directório do entrypoint dentro da cache — logo os
   `#import "..."` internos ao pacote funcionam sem tratamento especial.

Erros (todos `Result::Err` com mensagem clara, distinta de "pacotes não suportados"):

- Pacote não presente em nenhuma base →
  `pacote '@preview/nome:versao' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)`.
- `typst.toml` ilegível/inválido ou sem `[package].entrypoint` → mensagem específica.
- Entrypoint não existe no disco → mensagem específica.

É **I/O em L3** (permitido): lê o filesystem fora do projecto (cache do utilizador).
L1 (`World` trait, `eval_module_import`) só declara/consome o contrato.

## Critérios de Verificação

```
Dado SystemWorld::new(root, main) com ficheiro existente
Quando main() for chamado
Então FileId válido

Dado SystemWorld com ficheiro existente
Quando source(main_id) for chamado
Então Ok(Source) com text() igual ao conteúdo do ficheiro

Dado SystemWorld
Quando source(id_inexistente) for chamado
Então Err(FileError::NotFound)

Dado SystemWorld
Quando font(0) for chamado
Então None

Dado MockWorld("Hello *world*")
Quando source(main()) for chamado
Então Ok(Source) com text() == "Hello *world*"
```

---

## Cache de bytes por `FileId` — P876

`SystemWorld` mantém um cache de bytes brutos lidos via `read_bytes(current_file, path)`,
indexado pelo `FileId` canónico do path resolvido. Chamadas repetidas ao mesmo ficheiro
(por exemplo, 50× `image("debian-logo.png")`) devolvem o **mesmo** `Arc<Vec<u8>>`, em vez
de reler o disco e alocar um novo `Arc` a cada chamada.

### Semântica

- A chave é o `FileId` obtido por `register_file(path_canonicalizado)`.
- O valor é `Arc<Vec<u8>>` — clone O(1) nas chamadas subsequentes.
- O cache é preenchido na primeira leitura bem-sucedida e nunca invalidado durante a
  vida do `SystemWorld` (modelo de read-only durante a compilação de um documento).
- `file(id)` também consulta este cache antes de ler do disco.

### Motivo

`01_core/src/entities/elements/image.rs:46,53` aloca um `Arc` novo a cada avaliação de
`image()`. Sem cache de bytes, 50 chamadas ao mesmo ficheiro produzem 50 `Arc`s distintos,
quebrando a deduplicação por `Arc::as_ptr` do exportador PDF (`export/images.rs`) e gerando
50 XObjects em vez de 1 (paridade vanilla).

---

## Cobertura Unicode lazy — P880

**Problema medido (P877/P879):** `font_info_from_bytes` chamava
`extract_coverage` eager para cada face durante o emparelhamento do
`FontBook`. Em sistemas com ~1086 fontes, o parse/iteração da tabela
`cmap` de todas as fontes no startup tornava documentos simples
visivelmente mais lentos.

**Solução:** `SystemWorld` sobrescreve `World::candidates_for_char` e
mantém um cache lazy `Mutex<HashMap<usize, Coverage>>` indexado pelo
índice do slot no `font_slots`. A cobertura é computada a partir de
`FontSlot::source_bytes()` + `ttf_parser::Face::parse` +
`extract_coverage` apenas quando o slot é primeiro consultado.

### Algoritmo de `candidates_for_char`

```rust
fn candidates_for_char(&self, c: char) -> Vec<usize> {
    let codepoint = c as u32;
    let mut cache = self.coverage_cache.lock().unwrap();
    let mut result = Vec::new();
    for (idx, slot) in self.font_slots.iter().enumerate() {
        let coverage = cache.entry(idx).or_insert_with(|| {
            slot.source_bytes()
                .and_then(|data| ttf_parser::Face::parse(&data, slot.index).ok())
                .map(|face| extract_coverage(&face))
                .unwrap_or_else(Coverage::new)
        });
        if coverage.contains(codepoint) {
            result.push(idx);
        }
    }
    result
}
```

### Propriedades

- **Lazy:** slots não consultados nunca têm a sua `cmap` percorrida.
- **Cache:** uma vez computada, a `Coverage` de um slot é reutilizada
  para todos os caracteres subsequentes.
- **Alinhamento com `FontBook`:** os índices devolvidos são os mesmos
  índices usados por `font()` e pelo `FontBook`, garantindo consistência
  no fallback.
- **Aproximação por bloco:** o chamador (shaper, `FallbackFontMetrics`)
  continua a confirmar `face.glyph_index(c)` antes de usar o candidato.

### Campo adicional em `SystemWorld`

```rust
coverage_cache: Mutex<HashMap<usize, Coverage>>,
```

Inicializado vazio em `new`.

## Fontes embutidas — P753

A partir de P753, `SystemWorld` suporta fontes embutidas via `typst-assets`:

```rust
impl SystemWorld {
    /// Carrega as fontes embutidas do vanilla (Libertinus Serif, New Computer
    /// Modern, New Computer Modern Math, DejaVu Sans Mono).
    pub fn with_embedded_fonts(mut self) -> Self;

    /// Combina embutidas + sistema + fontes de projecto.
    pub fn with_fonts_and_system(mut self, font_paths: &[PathBuf]) -> Self;
}
```

- `with_embedded_fonts` usa `crate::embedded_fonts::load_embedded_fonts()`.
- `with_fonts_and_system` carrega: (1) embutidas; (2) sistema via `fontdb`; (3)
  projecto via `discover_fonts`. A ordem garante que o conjunto vanilla-like é
  sempre disponível, e que `--font-path` pode adicionar/sobrepor fontes.
- `with_fonts(paths)` mantém o comportamento pré-P753: apenas as fontes dos
  paths fornecidos, sem embutidas nem sistema.

Ver Prompt L0 `00_nucleo/prompts/infra/embedded_fonts.md` para a especificação
completa do mecanismo de fontes embutidas.

## Resolução de caminhos absolutos (`/...`) — P686

`SystemWorld` resolve o `path` de `include_source` e `read_bytes` via o helper
privado `resolve_path(current_file, path)`:

- **Absoluto** (`path` começa por `/`): a base é a raiz do pacote de
  `current_file` (devolvida por `package_root_of`) quando existe, ou `self.root`
  (raiz do projecto) caso contrário. O resultado é `base.join(path sem '/' inicial)`.
- **Relativo**: `directory_of(current_file).join(path)` — comportamento inalterado.

Helpers de suporte (privados a `SystemWorld`):

- `path_of(id) -> Option<PathBuf>` — path registado em `slots` para o `FileId`.
- `package_search_roots() -> Vec<PathBuf>` — directórios `typst/packages` em
  `data_local_dir`, `data_dir` e `cache_dir` (local, data, cache).
- `package_root_of(&Path) -> Option<PathBuf>` — se o path vive sob um desses
  roots com a forma `{ns}/{name}/{version}/...`, devolve
  `{root}/{ns}/{name}/{version}`.

A detecção da raiz do pacote é por **prefixo de path** (sem I/O de rede),
coerente com `resolve_package`. Caminhos relativos com `../` que escapem a raiz
do pacote permanecem fora de escopo (débito de sandbox pré-existente).

**Língua vs mecânica (ADR-0107):** a regra absoluto/relativo e a noção de raiz de
pacote são semântica da linguagem; a forma `{ns}/{name}/{version}` e os roots de
procura são mecânica de L3.

