# Passo 121.A — Inventário vanilla + decisões de `--root`

**Data**: 2026-04-23

---

## Parte 1 — Vanilla

### Declaração (args.rs:392-394)

```rust
/// Configures the project root (for absolute paths).
#[clap(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
pub root: Option<PathBuf>,
```

- Long `--root`, sem short.
- Valor de `value_name = "DIR"`.
- **Env var `TYPST_ROOT`** — clap feature `env` activa fallback.
- Default: `None` → fallback em run-time.

### Uso (world.rs:243-256)

```rust
let root = {
    let path = world_args
        .root
        .as_deref()
        .or_else(|| input_path.as_deref().and_then(|i| i.parent()))
        .unwrap_or(Path::new("."));
    path.canonicalize().map_err(|err| match err.kind() {
        io::ErrorKind::NotFound => {
            WorldCreationError::RootNotFound(path.to_path_buf())
        }
        _ => WorldCreationError::Io(err),
    })?
};
```

Precedência:
1. `--root DIR` flag.
2. `input.parent()` (directório do input).
3. `Path::new(".")` (CWD relativo).

**Canonicalização em run-time** (L3 no cristalino). Se path
inválido, `RootNotFound` error.

### Semântica vanilla

```rust
let main = RootedPath::new(VirtualRoot::Project, VirtualPath::virtualize(&root, path)?).intern()
```

Vanilla **virtualiza** input relativo a root. Absolute paths em
`#import "/foo"` são resolvidos contra root. Semântica **forte**:
root é a "raiz do projecto".

---

## Parte 2 — `SystemWorld::new` cristalino

`03_infra/src/world.rs:101-107`:

```rust
pub fn new(root: impl Into<PathBuf>, main: impl AsRef<Path>) -> Result<Self, SystemWorldError> {
    let root = root.into();
    let main_path = if main.as_ref().is_absolute() {
        main.as_ref().to_path_buf()
    } else {
        root.join(main.as_ref())
    };
    // ... canonicalize + register
}
```

Cristalino:
- `root: PathBuf` + `main: impl AsRef<Path>`.
- Se `main` absoluto → usa directamente (ignora root para main).
- Se `main` relativo → `root.join(main)`.

### Imports cristalino (world.rs:220-230)

```rust
fn read_bytes(&self, current_file: FileId, path: &str) -> Result<Arc<Vec<u8>>, String> {
    let base_dir = self.directory_of(current_file);
    // ...
}
fn include_source(&self, current_file: FileId, path: &str) -> Result<Source, String> {
    let base_dir = self.directory_of(current_file);
    // ...
}
```

**Imports resolvem contra `current_file.parent()`**, não contra
`root`. Diferença importante face ao vanilla.

### Implicação

`--root` em cristalino tem **semântica mais fraca** que vanilla:

- Em cristalino, `root` só localiza o main file (via
  `SystemWorld::new(root, main)`).
- Imports usam `directory_of(current_file)` — `root` não
  participa.
- Absolute paths em imports não são "virtualizados" pelo root
  (comportamento vanilla não implementado).

**Trabalho futuro**: implementar virtualização via root em L3 se
necessário. Fora do escopo do Passo 121 (que só adiciona a flag).

---

## Parte 3 — Decisão de semântica

### Escolha: **(a) "Root para localizar main; fallback para input.parent()"**

Razões:

1. **Alinha com vanilla no default** — fallback chain
   `--root → input.parent() → "."`.
2. **Semântica mais fraca mas honesta** — cristalino ainda não
   implementa virtualização; `--root` funciona como "onde está
   o main" (+ um placeholder para quando virtualização existir).
3. **Usuário familiar com vanilla** não se surpreende.
4. **L2 só devolve PathBuf**, sem validação — ADR-0051 P5
   preservado.

### Divergência vs vanilla

| Aspecto | Vanilla | Cristalino (passo 121) |
|---------|---------|------------------------|
| Default fallback | `--root → input.parent() → "."` | **Idêntico** |
| Canonicalização | L3 (run-time) | L3 (`SystemWorld::new` já canonicaliza main_path; root é raw) |
| Virtualização de absolute imports | Sim via `VirtualPath::virtualize` | **Não implementado** |
| Env var `TYPST_ROOT` | Sim (clap feature `env`) | **Não adiada** — feature extra não declarada no Passo 115 |
| Error quando root inexistente | `RootNotFound` | Mensagem genérica via `SystemWorldError` |

**Virtualização** e **`TYPST_ROOT` env** são trabalho futuro.
Passo 121 adiciona a flag com semântica pragmática.

---

## Parte 4 — Decisão de default

### Escolha: **(a) `--root → input.parent() → "."`**

Mesmo fallback do vanilla. Tests 114 continuam a funcionar
(sem `--root`, default é `input.parent()` — idêntico a Passo 120).

---

## Parte 5 — `resolve_root_with` função pura

```rust
pub fn resolve_root_with(
    root: Option<&PathBuf>,
    input: &Path,
) -> PathBuf {
    root.cloned()
        .or_else(|| input.parent().map(Path::to_path_buf).filter(|p| !p.as_os_str().is_empty()))
        .unwrap_or_else(|| PathBuf::from("."))
}
```

### Filtro `filter(|p| !p.as_os_str().is_empty())`

`Path::parent()` devolve `Some("")` para input sem directório
(ex: `"file.typ"`). Empty PathBuf é confuso — filtrar para
cair no fallback `"."`.

### Testes previstos

1. `resolve_root_explicito_usa_valor` — `--root /custom` vence.
2. `resolve_root_omitido_usa_input_parent` — `/a/b/file.typ` → `/a/b`.
3. `resolve_root_input_sem_parent_usa_cwd` — `"file.typ"` → `"."`.

---

## Parte 6 — L4 integração

`04_wiring/src/main.rs` passa de:

```rust
let root = input
    .parent()
    .filter(|p| !p.as_os_str().is_empty())
    .map(Path::to_path_buf)
    .unwrap_or_else(|| PathBuf::from("."));
```

Para:

```rust
// `root` já resolvido por L2 — usa directamente.
let RunIntent { input, output, root, colored } = cli::parse();
```

**`main_path` continua `input.file_name()`** — consistente com
Passo 113. Limitações:
- Se `--root` explícito e input é deep (ex:
  `typst sub/main.typ --root /projeto`), SystemWorld procura
  `/projeto/main.typ` (não `/projeto/sub/main.typ`). User tem
  de passar path relativo correctamente ou root inclusivo.
- Documentar como limitação.

---

## Conclusões 121.A

| Decisão | Escolha |
|---------|---------|
| Semântica | Root localiza main + placeholder para virtualização futura |
| Default | `--root → input.parent() → "."` (alinha vanilla) |
| Canonicalização | L3 (`SystemWorld::new` canonicaliza main_path; root é raw) |
| `TYPST_ROOT` env | **Não** (feature `env` de clap não declarada) |
| Virtualização | **Não** neste passo (trabalho futuro) |
| `RunIntent.root` | `PathBuf` sempre resolvido (L4 consome) |

Gate 121.A **não dispara** — ADR-0051 P5 preservado (L2 não
valida).

ADR-0051 **preview acerta** — decisão final confirma. **Não é
preciso anotar** ADR-0051.
