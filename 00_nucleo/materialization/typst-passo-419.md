# P419 — Carregamento de `.bib`/`.yaml`/`.json` de disco para Bibliography (M)

**Título**: Bibliography file loading — I/O de `.bib`/`.yaml`/`.json` via path no `#bibliography("refs.bib")`  
**Tipo**: Materialização (M) — consumer Bibliography + infraestrutura de file loading L1  
**Bloqueadores**: Nenhum externo; pré-condições internas verificáveis; hayagriva já parseia  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P418 (Bibliography CSL real), ADR-0062 (hayagriva IMPLEMENTADO)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 8 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: 6/8 mínimo; itens 7 e 8 são nice-to-have. Se qualquer um dos 6 obrigatórios falhar → **parar imediatamente** e reclassificar.

```bash
# 1. BibliographyElem já tem path: EcoString?
grep -rn "struct BibliographyElem" 01_core/src/entities/elements/bibliography.rs

# 2. hayagriva parseia .bib / .yaml / .json?
grep -rn "load\|parse\|from_yaml\|from_json\|from_bib" 01_core/src/ 2>/dev/null | grep -i hayagriva | head -10

# 3. Existe file loader genérico no World ou em L1?
grep -rn "fn resolve\|fn load_file\|FileLoader\|read_file\|read_to_string" 01_core/src/ | head -20

# 4. Path resolution existe (relative to source file)?
grep -rn "path\|Path\|PathBuf" 01_core/src/entities/world.rs 2>/dev/null | head -10

# 5. O eval de #bibliography("refs.bib") já existe?
grep -rn "native_bibliography\|bibliography" 01_core/src/engine/stdlib/ | head -20

# 6. O layout de Bibliography consome Vec<BibEntry> ou path?
grep -rn "BibliographyElem\|bib_entries\|entries" 01_core/src/engine/layout/bibliography.rs | head -20

# 7. [NICE-TO-HAVE] Existe tratamento de erro para file-not-found?
grep -rn "FileNotFound\|NotFound\|IOError\|io::" 01_core/src/ | head -10

# 8. [NICE-TO-HAVE] Existe test infrastructure para arquivos temporários?
grep -rn "tempfile\|temp_dir\|TempDir" Cargo.toml 01_core/Cargo.toml 2>/dev/null
```

**Output esperado**:
1. ≥1 hit com `path: EcoString` em `BibliographyElem`
2. ≥0 hits (hayagriva API pode ser usada diretamente; não precisa estar no código atual)
3. ≥0 hits (file loader genérico pode não existir)
4. ≥0 hits (path resolution pode ser básica)
5. ≥1 hit com `native_bibliography` ou similar
6. ≥1 hit confirmando que `BibliographyElem` tem `entries` ou `path` (P418 usou `Vec<BibEntry>` literal; verificar se `path` já existe)
7. ≥0 hits (tratamento de erro pode ser básico)
8. ≥0 hits (test infrastructure pode não existir)

**Se (1) falhar** → `BibliographyElem` não tem campo `path`; reclassificar para S (adicionar campo).  
**Se (5) falhar** → `native_bibliography` não existe; reclassificar para L (reabrir infraestrutura de eval).  
**Se (6) falhar** → layout de Bibliography não existe; reclassificar para L (reabrir P418).

**Se (3) falhar** (file loader genérico não existe):  
→ **Decisão a tomar**: criar file loader genérico em `World`, ou loader específico para bibliography?  
→ **Critério ADR-0107**: paridade é com a linguagem (`#bibliography("refs.bib")` carrega arquivo), não com a mecânica (como o `World` resolve paths).  
→ **Critério ADR-0108**: medir o custo. File loader genérico é L (reusa para imagens, dados, etc.). Loader específico é S (só para bibliography).  
→ **Decisão recomendada**: criar **loader específico para bibliography** neste passo (M), deixando a **generalização para file loader** como scope-out (P419.X ou passo dedicado). Isso mantém o escopo controlado.

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/elements/bibliography.md` + `rules/loading.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `#bibliography("refs.bib")` é uma **construção linguística** que:
- **Semântica**: resolve o path relativo ao arquivo `.typ` fonte, carrega o arquivo, parseia as entradas bibliográficas, e torna-as disponíveis para citações no documento.
- **Sintaxe**: path como string literal posicional; style e locale como named args.
- **Morfologia**: o `BibliographyElem` contém o path (não as entradas parseadas); as entradas são materializadas em eval time.

**O que NÃO é paridade (mecânica; diverge de propósito):**
- O mecanismo exato de resolução de path (vanilla usa VFS virtual; crystalline pode usar `std::fs` diretamente se L1 permitir).
- A estrutura interna do buffer de entradas (vanilla usa hayagriva `Entry` internamente; crystalline pode converter para `Vec<BibEntry>` ou manter `hayagriva::Entry`).
- A ordem de eval (vanilla pode lazy-load; crystalline carrega no primeiro `#bibliography`).

### A.1.2 — Decisão arquitetural: file loading (ADR-0108)

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — File loader genérico em `World` | `World` ganha `resolve_file(path) -> Result<String, IoError>` reutilizável para imagens, dados, etc. | L | Baixo | Reutilizável; mas expande escopo além de bibliography |
| **β** — Loader específico para bibliography | `BibliographyElem` carrega em eval via `std::fs::read_to_string` + hayagriva parse; isolado em `rules/eval/bibliography.rs` | **M** | **Baixo** | Escopo controlado; não afeta outras features |
| **γ** — I/O em L0 (no parser) | O parser lê o arquivo durante o parse do `.typ` | S | Alto | Viola separação parse→eval; path não é resolvido corretamente |

**Decisão recomendada**: **Opção β** — loader específico para bibliography.  
**Razão ADR-0108**: medir o custo. O file loader genérico (α) é desejável mas expande o escopo para além de M. A opção β fecha o gap #5 do P418 com o mínimo de LOC. A generalização (α) fica como **scope-out P419.X**.

### A.1.3 — Decisão arquitetural: atomização forma B (ADR-0109)

A lógica de file loading vive na **camada de eval**, não no struct:

- `entities/elements/bibliography.rs` — `BibliographyElem` ganha `path: EcoString` (se ainda não tiver); nenhuma lógica de I/O.
- `rules/eval/bibliography.rs` — free function `load_bibliography_from_path(ctx, path) -> Result<Vec<BibEntry>, EvalError>` (forma B).
- `rules/eval/bibliography.rs` — free function `eval_bibliography(ctx, &BibliographyElem) -> Value` chama o loader + hayagriva parse.
- `engine/layout/bibliography.rs` — consumer inalterado (recebe `Vec<BibEntry>` do cache, como no P418).

**Não usar Opção A** (método `impl BibliographyElem { fn load_from_disk(...) }` em `entities/`) — cria import de I/O no arquivo de dados (ADR-0109, rejeitado).

### A.1.4 — Estrutura de dados

```rust
// entities/elements/bibliography.rs
// BibliographyElem já existe (P418); verificar se precisa de ajuste:
pub struct BibliographyElem {
    pub path: EcoString,          // path do arquivo .bib/.yaml/.json
    pub style: Option<EcoString>, // CSL style (P418)
    pub locale: Option<EcoString>,// CSL locale (P418)
    pub title: Option<Box<Content>>, // título customizado (P418)
}
```

**Nota**: Se `BibliographyElem` atual tem `entries: Vec<BibEntry>` em vez de `path`, **substituir** `entries` por `path` + loader. O `entries` passa a ser materializado em eval time, não armazenado no struct. Isso é **paridade linguagem**: o vanilla não armazena entradas parseadas no elem; armazena o path.

### A.1.5 — Algoritmo de loading

1. **Eval de `#bibliography("refs.bib")`**:
   - `native_bibliography` recebe `path: EcoString` (positional arg).
   - Chama `load_bibliography_from_path(ctx, &path)`.

2. **Path resolution**:
   - Se `path` é absoluto: usar diretamente.
   - Se `path` é relativo: resolver relativamente ao diretório do arquivo `.typ` fonte (obtido via `ctx.world.source_path()` ou similar).
   - Se não houver source path (ex.: REPL, API programática): resolver relativamente a `std::env::current_dir()`.

3. **File loading**:
   - `std::fs::read_to_string(&resolved_path)` → `Result<String, IoError>`.
   - Em erro (`NotFound`, `PermissionDenied`): retornar `EvalError` com mensagem clara (paridade vanilla: "file not found: refs.bib").

4. **Parsing**:
   - Detectar formato pelo sufixo: `.bib` → BibTeX, `.yaml` → YAML, `.json` → JSON, `.yml` → YAML.
   - Usar hayagriva para parsear:
     - `hayagriva::io::from_biblatex_str(&content)` para `.bib`
     - `hayagriva::io::from_yaml_str(&content)` para `.yaml`/`.yml`
     - `hayagriva::io::from_json_str(&content)` para `.json`
   - Converter `hayagriva::Entry` → `Vec<BibEntry>` (ou manter `hayagriva::Entry` diretamente se o P418 já usa).
   - Em erro de parse: retornar `EvalError` com mensagem de parse (paridade vanilla: "failed to parse bibliography").

5. **Armazenamento**:
   - As entradas parseadas são armazenadas no `World`/`Introspector` (como no P418).
   - O `BibliographyElem` armazena apenas `path`, `style`, `locale`, `title`.

### A.1.6 — Paridade vanilla

- Vanilla: `#bibliography("refs.bib")` resolve path relativo ao `.typ`, carrega, parseia, e disponibiliza entradas. Erros de file-not-found e parse são reportados em eval time.
- Cristalino P419: paridade comportamental (path → entradas, erros reportados, relativo ao source). A mecânica (hayagriva, `std::fs`, ordem de eval) é livre.

### A.1.7 — Scope-out explícito

- File loader genérico para outras features (imagens, dados CSV, etc.) — scope-out P419.X.
- Path resolution via VFS virtual (vanilla usa VFS; crystalline usa `std::fs` diretamente) — scope-out; paridade é comportamental.
- Watch/reload de arquivo bibliography em modo interativo — scope-out.
- Encoding detection (UTF-8 assumido; vanilla detecta) — scope-out.
- `.bib` com macros BibTeX complexos — scope-out; hayagriva lida com o subset comum.
- Network URLs (`http://...`) — scope-out; apenas paths locais.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `bibliography.md` + `loading.md`
2. Sonda A.0 produziu 6/8 OK (mínimo)
3. Decisão β (loader específico) validada — `std::fs` disponível em L1
4. `BibliographyElem` tem `path` ou plano de adição/substituição está claro
5. hayagriva APIs de parse (`from_biblatex_str`, `from_yaml_str`, `from_json_str`) confirmadas via `cargo doc` ou source

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Entities

1. **`entities/elements/bibliography.rs`**:
   - Se `BibliographyElem` tem `entries: Vec<BibEntry>`: **remover** `entries` e substituir por `path: EcoString`.
   - Se já tem `path`: nenhuma alteração.
   - Atualizar `PartialEq`, `Debug`, `Hash` se necessário.
   - Atualizar construtores: `BibliographyElem::new(path, style, locale, title)`.

2. **`entities/content.rs`**:
   - Atualizar `Content::bibliography(...)` para receber `path` em vez de `entries`.
   - Se `Content::Bibliography` armazena `Vec<BibEntry>`: mudar para `BibliographyElem` com `path`.

### B.2 — Eval (`rules/eval/bibliography.rs` — forma B, ADR-0109)

```rust
use std::fs;
use std::path::Path;

pub(super) fn eval_bibliography(
    ctx: &mut EvalContext,
    elem: &BibliographyElem,
) -> Result<Value, EvalError> {
    // 1. Resolver path
    let resolved = resolve_bibliography_path(ctx, &elem.path)?;

    // 2. Carregar conteúdo
    let content = fs::read_to_string(&resolved)
        .map_err(|e| EvalError::io_error(format!(
            "failed to read bibliography file {}: {}", elem.path, e
        )))?;

    // 3. Parsear pelo formato
    let entries = parse_bibliography(&content, &resolved)?;

    // 4. Resolver CSL style (P418)
    let style = resolve_csl_style(ctx, elem.style.as_deref())?;

    // 5. Armazenar no World/Introspector (P418)
    ctx.introspector.set_bibliography(entries.clone(), style.clone());

    // 6. Retornar Content::Bibliography
    Ok(Value::Content(Content::Bibliography(elem.clone())))
}

fn resolve_bibliography_path(
    ctx: &EvalContext,
    path: &EcoString,
) -> Result<std::path::PathBuf, EvalError> {
    let p = Path::new(path.as_str());
    if p.is_absolute() {
        return Ok(p.to_path_buf());
    }
    // Relativo ao source file
    if let Some(source_dir) = ctx.world.source_path().parent() {
        Ok(source_dir.join(p))
    } else {
        // Fallback: relativo ao current_dir
        Ok(std::env::current_dir()
            .map_err(|e| EvalError::io_error(format!("current dir: {}", e)))?
            .join(p))
    }
}

fn parse_bibliography(
    content: &str,
    path: &Path,
) -> Result<Vec<BibEntry>, EvalError> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let hay_entries = match ext.as_str() {
        "bib" => hayagriva::io::from_biblatex_str(content)
            .map_err(|e| EvalError::io_error(format!("BibTeX parse error: {}", e)))?,
        "yaml" | "yml" => hayagriva::io::from_yaml_str(content)
            .map_err(|e| EvalError::io_error(format!("YAML parse error: {}", e)))?,
        "json" => hayagriva::io::from_json_str(content)
            .map_err(|e| EvalError::io_error(format!("JSON parse error: {}", e)))?,
        _ => return Err(EvalError::io_error(format!(
            "unsupported bibliography format: {}", ext
        ))),
    };

    // Converter hayagriva::Entry -> BibEntry (ou manter hayagriva::Entry se P418 já usa)
    Ok(convert_hay_entries(hay_entries))
}
```

### B.3 — Eval context / World extension

```rust
// Em world.rs ou eval_context.rs
impl World {
    pub fn source_path(&self) -> &Path { ... }  // path do arquivo .typ sendo compilado
}
```

**Se `source_path` não existir**: adicionar campo `source_path: Option<PathBuf>` ao `World` (setado no início da compilação). Scope-out se for complexo demais — usar `current_dir()` como fallback.

### B.4 — Stdlib (`rules/stdlib/structural.rs`)

```rust
fn native_bibliography(args: Args) -> Result<Value, EvalError> {
    let path: EcoString = args.expect("path")?;  // positional
    let style: Option<EcoString> = args.named("style")?;
    let locale: Option<EcoString> = args.named("locale")?;
    let title: Option<Content> = args.named("title")?;

    let elem = BibliographyElem {
        path,
        style,
        locale,
        title: title.map(Box::new),
    };

    eval_bibliography(ctx, &elem)
}
```

### B.5 — Layout (`engine/layout/bibliography.rs` — inalterado em relação a P418)

O layout continua consumindo `bib_render_cache` do `Layouter` (P418). Nenhuma alteração necessária — o loading aconteceu em eval time.

### B.6 — Tests

**Mínimo 14 tests**:
- 3 unit `resolve_bibliography_path`: absoluto, relativo ao source, relativo ao current_dir
- 4 unit `parse_bibliography`: `.bib` válido, `.yaml` válido, `.json` válido, formato desconhecido
- 3 unit `eval_bibliography`: file not found, parse error, sucesso
- 4 E2E: `#bibliography("refs.bib")` carrega e renderiza, path relativo funciona, erro file-not-found com mensagem clara, `.yaml` alternativo funciona

**Test infrastructure**: criar arquivos temporários `.bib`/`.yaml` nos tests via `std::fs::write` em diretório temporário (ou `include_str!` para fixtures).

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib -- p419
# → 14 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 14 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `#bibliography("refs.bib")` carrega arquivo do disco
- [ ] Path relativo resolvido corretamente (relativo ao `.typ` source)
- [ ] Erro `file not found` com mensagem clara em eval time
- [ ] Erro de parse com mensagem clara em eval time
- [ ] `.bib`, `.yaml`, `.json` suportados
- [ ] Layout/bibliography continua funcionando (P418 não regressado)
- [ ] Nenhum vtable/`dyn` introduzido (ADR-0109 / ADR-0026)
- [ ] `match` exaustivo preservado
- [ ] Lógica atomizada em free functions (forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica 8 pré-condições antes de qualquer decisão. Se `std::fs` não estiver disponível em L1 (ex.: target wasm), o passo para e reclassifica.
- **Paridade linguagem (ADR-0107)**: O contrato é `#bibliography("refs.bib")` → entradas carregadas do disco. A mecânica (hayagriva, `std::fs`, path resolution) é livre. O erro em eval time (não em layout) é comportamental — o vanilla reporta em compile time.
- **Atomização (ADR-0109)**: `BibliographyElem` é struct puro (path, style, locale, title). A lógica de loading vive em `rules/eval/bibliography.rs` como free functions. O layout não muda (P418). O `match` no consumer fica magro.
- **Honestidade epistêmica**: Este passo fecha o gap #5 do P418. Não é "mais bibliography"; é "bibliography com I/O real". O scope-out P419.X (file loader genérico) é honestamente documentado.
- **Próximo passo P420**: file loader genérico (L, scope-out P419.X) ou `.csl` customizado via path (gap #1, M) ou `repr()` completo (S).
