# P420 — `.csl` Customizado via Path: CSL style de arquivo no disco (M)

**Título**: Bibliography CSL custom — carregamento de `.csl` XML via path local no `style:`  
**Tipo**: Materialização (M) — consumer Bibliography + CSL loading + hayagriva CSL XML  
**Bloqueadores**: Nenhum externo; P418 (bibliography CSL real) e P419 (file loading) como pré-requisitos  
**Referências**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B), P418 (CSL real), P419 (file loading de .bib), ADR-0062 (hayagriva IMPLEMENTADO)

---

## FASE A.0 — Sonda do substrato (obrigatória; 5 min)

Execute os 8 grep abaixo **antes de qualquer redação ou código**.  
**Critério de passagem**: 6/8 mínimo; itens 7 e 8 nice-to-have. Se qualquer um dos 6 obrigatórios falhar → **parar imediatamente** e reclassificar.

```bash
# 1. BibliographyElem já tem style: Option<EcoString>?
grep -n "style" 01_core/src/entities/elements/bibliography.rs | head -10

# 2. P418 já resolve style built-in ("ieee", "apa")?
grep -rn "ieee\|apa\|chicago" 01_core/src/rules/layout/bib_csl.rs | head -10

# 3. hayagriva aceita CSL XML de string/bytes?
grep -rn "Style\|from_xml\|from_csl" ~/.cargo/registry/src/*/hayagriva-*/src/ 2>/dev/null | head -20

# 4. P419 já carrega arquivo do disco (World::read_bytes)?
grep -rn "read_bytes\|read_to_string" 01_core/src/entities/world.rs 01_core/src/rules/eval/bibliography.rs | head -10

# 5. CSL style file loading existe no código atual?
grep -rn "\.csl\|csl_file\|style_path" 01_core/src/ | head -20

# 6. O eval de style já distingue built-in vs custom?
grep -rn "style" 01_core/src/rules/eval/bibliography.rs | head -20

# 7. [NICE-TO-HAVE] Existe tratamento de erro para XML parse malformado?
grep -rn "xml\|Xml\|malformed\|parse.*error" 01_core/src/ | head -10

# 8. [NICE-TO-HAVE] Existe test com arquivo temporário .csl?
grep -rn "\.csl" 01_core/src/rules/ 01_core/src/entities/ | head -10
```

**Output esperado**:
1. ≥1 hit com `style: Option<EcoString>` (ou similar) em `BibliographyElem`
2. ≥1 hit com built-in styles em `bib_csl.rs`
3. ≥1 hit com `Style::from_xml` ou similar na API hayagriva (verificar via `cargo doc` se grep falhar)
4. ≥1 hit com `read_bytes` ou `read_to_string` (P419)
5. ≥0 hits (CSL custom pode não existir ainda — este é o gap)
6. ≥0 hits (distinguir built-in vs custom pode não existir ainda)
7. ≥0 hits (XML parse error pode não existir)
8. ≥0 hits (tests .csl podem não existir)

**Se (1) falhar** → `BibliographyElem` não tem `style`; reclassificar para S (adicionar campo).  
**Se (2) falhar** → P418 não implementou built-in styles; reclassificar para L (reabrir P418).  
**Se (4) falhar** → P419 não implementou file loading; reclassificar para L (reabrir P419).  
**Se (3) falhar** (hayagriva não expõe CSL XML parsing):  
→ Verificar via `cargo doc -p hayagriva --open` ou `rg "from_xml\|Style::new"` no source do crate.  
→ Se não houver API pública: reclassificar para L (implementar CSL XML parse próprio — ~5k LOC, não justifica).  
→ Decisão: se hayagriva não parseia CSL XML custom, **scope-out** e documentar limitação.

---

## FASE A.1 — L0 (hash obrigatório)

**Documentar no L0** (`00_nucleo/prompts/entities/elements/bibliography.md` + `rules/layout/bib_csl.md`):

### A.1.1 — Decisão arquitetural: paridade linguagem (ADR-0107)

No Typst vanilla, `#bibliography("refs.bib", style: "custom.csl")` é uma **construção linguística** que:
- **Semântica**: o arg `style` pode ser (a) string built-in ("ieee", "apa", etc.) → resolved via hayagriva archive; (b) path para arquivo `.csl` XML → resolved via file system, parseado como CSL style, e aplicado às citações e bibliografia.
- **Sintaxe**: `style: <string>` onde `<string>` é interpretado como built-in se bater nome no archive; senão, como path relativo/absoluto.
- **Morfologia**: o `BibliographyElem` armazena `style: Option<EcoString>` (não distingue built-in vs custom no struct; a distinção é em eval time).

**O que NÃO é paridade (mecânica; diverge de propósito):**
- A ordem de resolução (vanilla pode tentar built-in primeiro, depois path; crystalline faz o mesmo, mas a mecânica exata é livre).
- A estrutura interna do CSL XML parseado (hayagriva `Style` vs outra representação).
- O cache de styles (vanilla pode cachear por path; crystalline pode recarregar a cada eval).

### A.1.2 — Decisão arquitetural: resolução de style (ADR-0108)

| Opção | Descrição | Magnitude | Risco | Nota |
|-------|-----------|-----------|-------|------|
| **α** — Distinguir em eval: built-in primeiro, path fallback | `resolve_style()` tenta `hayagriva::archive::get_style(name)`; se falhar, tenta `load_csl_from_path(path)` | **M** | **Baixo** | Paridade vanilla; reusa P419 file loading |
| **β** — Prefixo explícito (`style: "file:custom.csl"`) | Usar prefixo `file:` para distinguir path de built-in | S | Baixo | Não é paridade vanilla (sintaxe diferente) |
| **γ** — Só built-ins | Não suportar `.csl` custom; scope-out permanente | — | — | Não fecha gap #1 do P418 |

**Decisão recomendada**: **Opção α** — tenta built-in primeiro, path como fallback.  
**Razão ADR-0108**: medir o custo. A opção α reusa o file loading do P419 (`World::read_bytes`) e a infraestrutura CSL do P418 (`hayagriva::Style`). O custo adicional é o parse CSL XML + integração. A opção β viola sintaxe vanilla. A opção γ não fecha o gap.

### A.1.3 — Decisão arquitetural: atomização forma B (ADR-0109)

A lógica de resolução de style vive na **camada de eval**, não no struct:

- `entities/elements/bibliography.rs` — `BibliographyElem` inalterado (já tem `style: Option<EcoString>`).
- `rules/eval/bibliography.rs` — free function `resolve_style(ctx, style_str) -> Result<Style, EvalError>` (forma B).
- `rules/eval/bibliography.rs` — free function `load_csl_from_path(ctx, path) -> Result<Style, EvalError>` (forma B).
- `rules/layout/bib_csl.rs` — consumer inalterado (recebe `Style` já resolvido, como no P418).

**Não usar Opção A** (método `impl BibliographyElem { fn resolve_style(...) }` em `entities/`) — cria import de I/O e hayagriva no arquivo de dados (ADR-0109, rejeitado).

### A.1.4 — Estrutura de dados

```rust
// BibliographyElem — inalterado (P418/P419)
pub struct BibliographyElem {
    pub path: Option<EcoString>,   // path do .bib/.yaml (P419)
    pub entries: Vec<BibEntry>,     // fallback input literal (P418/P419 compat)
    pub style: Option<EcoString>,  // "ieee", "apa", ou "custom.csl"
    pub locale: Option<EcoString>, // "pt-BR", "en-US"
    pub title: Option<Box<Content>>,
}
```

**Style resolution** (não armazenado no struct; materializado em eval):
```rust
// Em eval/bibliography.rs — não no struct
enum ResolvedStyle {
    BuiltIn(hayagriva::Style),  // do archive
    Custom(hayagriva::Style),   // do arquivo .csl
}
```

### A.1.5 — Algoritmo de resolução de style

1. **Eval de `style` argument**:
   - `native_bibliography` recebe `style: Option<EcoString>`.
   - Se `None`: usar default built-in ("ieee" ou "apa" — paridade P418).
   - Se `Some(style_str)`:
     a. Tentar `hayagriva::archive::get_style(&style_str)` → se Ok, retornar `ResolvedStyle::BuiltIn`.
     b. Se falhar (nome não encontrado no archive), tratar como path.

2. **Path resolution (fallback)**:
   - Resolver path como no P419: absoluto → direto; relativo → `source_path.parent()` ou `current_dir()`.
   - `World::read_bytes(current_file, &path)` → `Result<Vec<u8>, FileError>`.
   - Em erro (`NotFound`, `PermissionDenied`): retornar `EvalError` com mensagem clara:
     ```
     failed to load CSL style file: custom.csl
     tried built-in styles: not found
     tried path: file not found at /absolute/path/to/custom.csl
     ```

3. **CSL XML parsing**:
   - `String::from_utf8(bytes)` → `Result<String, FromUtf8Error>`.
   - Em erro: `EvalError::EncodingError { path, encoding: "UTF-8" }`.
   - `hayagriva::Style::from_xml(&content)` → `Result<Style, HayagrivaError>`.
   - Em erro: `EvalError::CslParseError { path, cause: e.to_string() }`.
   - Retornar `ResolvedStyle::Custom(style)`.

4. **Armazenamento**:
   - O `Style` resolvido (built-in ou custom) é armazenado no `Introspector`/`World` (como no P418).
   - O `BibliographyElem` armazena apenas a string original (`style: Some("custom.csl")`).

5. **Cache** (melhoria P420.X):
   - `Introspector` mantém `HashMap<EcoString, Style>` como cache de styles.
   - `resolve_style` consulta cache primeiro; se miss, carrega e insere.
   - Isso evita re-carregar o mesmo `.csl` em múltiplas chamadas.
   - **Scope-out**: se complexo demais, documentar como "sem cache; recarrega a cada eval".

### A.1.6 — Contrato de erro (incorporando sugestão clipboard #1)

Erros são mapeados para `EvalError` com **mensagens legíveis e estruturadas**:

| Erro | Origem | Mensagem | Tipo EvalError |
|------|--------|----------|--------------|
| Built-in não encontrado | `archive::get_style` falha | `unknown CSL style: "foo". available: ieee, apa, ...` | `EvalError::UnknownStyle { name, available }` |
| File not found (path) | `World::read_bytes` NotFound | `CSL style file not found: custom.csl (resolved: /abs/path/custom.csl)` | `EvalError::FileNotFound { path }` |
| Encoding inválido | `String::from_utf8` falha | `CSL style file is not valid UTF-8: custom.csl` | `EvalError::EncodingError { path, encoding }` |
| XML malformado | `Style::from_xml` falha | `failed to parse CSL style: custom.csl — <cause from hayagriva>` | `EvalError::CslParseError { path, cause }` |
| Permissão negada | `World::read_bytes` PermissionDenied | `permission denied reading CSL style: custom.csl` | `EvalError::PermissionDenied { path }` |

**Nota**: se o sistema de `EvalError` não suportar variantes estruturadas, usar `EvalError::msg(String)` com mensagem formatada. A **forma** (variante estruturada) é mecânica; a **mensagem legível** é linguagem (ADR-0107).

### A.1.7 — Paridade vanilla

- Vanilla: `style: "ieee"` → built-in; `style: "custom.csl"` → carrega arquivo; `style: "nonexistent"` → erro "unknown style" se não for built-in nem arquivo.
- Cristalino P420: paridade comportamental (built-in → archive; path → file; erro claro). A mecânica (ordem de tentativa, cache, estrutura interna do `Style`) é livre.

### A.1.8 — Scope-out explícito

- CSL style via URL (`http://...`) — scope-out; apenas paths locais.
- CSL style em diretório de sistema (ex.: `~/.csl/`) — scope-out; apenas path relativo/absoluto explícito.
- Múltiplos styles simultâneos em um documento — scope-out; primeiro `BibliographyElem` governa (como no P418).
- Modificação de CSL style em runtime (hot-reload) — scope-out.
- Validação completa de CSL schema (relaxNG) — scope-out; hayagriva faz validação básica.
- Cache de styles cross-evaluation (persistente entre compilações) — scope-out P420.X.

---

## CHECKPOINT A

**Só prosseguir para Fase B quando confirmar que:**
1. Guardou e computou hash do L0 (A.1) em `bibliography.md` + `bib_csl.md`
2. Sonda A.0 produziu 6/8 OK (mínimo)
3. hayagriva API `Style::from_xml` (ou equivalente) confirmada
4. P419 file loading (`World::read_bytes`) funcional e reutilizável
5. Decisão α (built-in primeiro, path fallback) validada

**Hash L0 esperado**: `<computar após redação>`

---

## FASE B — Código

### B.1 — Eval (`rules/eval/bibliography.rs` — forma B, ADR-0109)

```rust
use hayagriva::archive;
use hayagriva::Style;

/// Resolve um style string para hayagriva::Style.
/// Tenta built-in primeiro; se falhar, trata como path local.
pub(super) fn resolve_style(
    ctx: &mut EvalContext,
    style_str: &EcoString,
) -> Result<Style, EvalError> {
    // 1. Tentar built-in
    if let Some(style) = archive::get_style(style_str.as_str()) {
        return Ok(style);
    }

    // 2. Fallback: path local
    load_csl_from_path(ctx, style_str)
}

/// Carrega um arquivo .csl do disco e parseia como CSL style.
fn load_csl_from_path(
    ctx: &EvalContext,
    path: &EcoString,
) -> Result<Style, EvalError> {
    // 2a. Resolver path
    let resolved = resolve_csl_path(ctx, path)?;

    // 2b. Ler bytes
    let bytes = ctx.world.read_bytes(ctx.current_file(), &resolved)
        .map_err(|e| match e {
            FileError::NotFound => EvalError::file_not_found(
                path, resolved.display()
            ),
            FileError::PermissionDenied => EvalError::permission_denied(
                path, resolved.display()
            ),
            _ => EvalError::io_error(format!(
                "failed to read CSL style file {}: {}", path, e
            )),
        })?;

    // 2c. UTF-8 decode
    let content = String::from_utf8(bytes)
        .map_err(|_| EvalError::encoding_error(path, "UTF-8"))?;

    // 2d. Parse CSL XML
    let style = Style::from_xml(&content)
        .map_err(|e| EvalError::csl_parse_error(path, &e.to_string()))?;

    Ok(style)
}

/// Resolve path CSL: absoluto → direto; relativo → source dir / current dir.
fn resolve_csl_path(
    ctx: &EvalContext,
    path: &EcoString,
) -> Result<PathBuf, EvalError> {
    let p = Path::new(path.as_str());
    if p.is_absolute() {
        return Ok(p.to_path_buf());
    }
    // Relativo ao source file
    if let Some(source) = ctx.world.source_path() {
        if let Some(parent) = source.parent() {
            return Ok(parent.join(p));
        }
    }
    // Fallback: current dir
    std::env::current_dir()
        .map(|d| d.join(p))
        .map_err(|e| EvalError::io_error(format!("current dir: {}", e)))
}
```

### B.2 — EvalContext / World extension (cache de styles)

```rust
// Em world.rs ou eval_context.rs
pub struct StyleCache {
    styles: HashMap<EcoString, hayagriva::Style>,
}

impl World {
    pub fn get_cached_style(&self, key: &EcoString) -> Option<&hayagriva::Style> {
        self.style_cache.styles.get(key)
    }

    pub fn cache_style(&mut self, key: EcoString, style: hayagriva::Style) {
        self.style_cache.styles.insert(key, style);
    }
}
```

**Scope-out**: se `StyleCache` for complexo de adicionar, documentar "sem cache; recarrega a cada eval" e remover `cache_style`/`get_cached_style`.

### B.3 — Stdlib (`rules/stdlib/structural.rs`)

```rust
fn native_bibliography(args: Args) -> Result<Value, EvalError> {
    let path_or_entries: Value = args.expect("path")?;
    let style_str: Option<EcoString> = args.named("style")?;
    let locale: Option<EcoString> = args.named("locale")?;
    let title: Option<Content> = args.named("title")?;

    // Resolve style (built-in ou custom .csl)
    let style = match &style_str {
        Some(s) => Some(resolve_style(ctx, s)?),
        None => None, // usa default do P418
    };

    // ... resto do eval (carregar entries, armazenar no Introspector)
    // O style resolvido (hayagriva::Style) vai para o Introspector
}
```

### B.4 — Layout (`rules/layout/bib_csl.rs` — inalterado em relação a P418)

O layout continua consumindo `bib_render_cache` do `Layouter`, que já contém o `Style` (seja built-in ou custom). Nenhuma alteração necessária no consumer — o `Style` é opaco para o layout.

### B.5 — Tests

**Mínimo 14 tests**:
- 3 unit `resolve_style`: built-in "ieee" → Ok, built-in "nonexistent" → fallback path, built-in "nonexistent" + path não existe → erro claro.
- 4 unit `load_csl_from_path`: `.csl` válido, file not found, encoding inválido (bytes não UTF-8), XML malformado.
- 2 unit `resolve_csl_path`: absoluto, relativo ao source.
- 5 E2E: `#bibliography(..., style: "custom.csl")` renderiza com style custom, style built-in continua funcionando, style inválido → erro, path relativo funciona, path absoluto funciona.

**Test infrastructure**: criar arquivo `.csl` temporário nos tests:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<style xmlns="http://purl.org/net/xbiblio/csl" version="1.0">
  <info><title>Custom</title></info>
  <citation>
    <layout><text variable="title"/></layout>
  </citation>
  <bibliography>
    <layout><text variable="title"/></layout>
  </bibliography>
</style>
```

---

## FASE C — Validação

```bash
cargo test -p typst-core --lib -- p420
# → 14 passed; 0 failed; 0 ignored

crystalline-lint .
# → 0 errors
# → 0 drift nos prompts tocados
```

**Critério de fecho**:
- [ ] 14 tests verdes
- [ ] Lint zero errors; drift sincronizado
- [ ] `style: "ieee"` continua funcionando (built-in, P418 não regressado)
- [ ] `style: "custom.csl"` carrega e aplica CSL custom
- [ ] `style: "nonexistent"` → erro claro (built-in não encontrado + path não existe)
- [ ] File not found → mensagem com path absoluto resolvido
- [ ] XML malformado → mensagem com causa do hayagriva
- [ ] Encoding inválido → mensagem "not valid UTF-8"
- [ ] Nenhum vtable/`dyn` introduzido (ADR-0109 / ADR-0026)
- [ ] `match` exaustivo preservado
- [ ] Lógica atomizada em free functions (forma B)
- [ ] L0 hashado e propagado

---

## Notas epistêmicas

- **Medir antes de decidir (ADR-0108)**: A.0 verifica se hayagriva expõe `Style::from_xml`. Se não, o passo para e scope-out é documentado. Não se assume que a API existe.
- **Paridade linguagem (ADR-0107)**: O contrato é `style: "custom.csl"` → CSL custom aplicado. A mecânica (ordem de resolução, cache, estrutura do `Style`) é livre. As mensagens de erro são o observável linguístico — devem ser legíveis.
- **Atomização (ADR-0109)**: `BibliographyElem` é struct puro. A lógica de resolução de style vive em `rules/eval/bibliography.rs` como free functions. O layout não muda (P418). O `match` no consumer fica magro.
- **Incorporação de sugestões**: O contrato de erro detalhado (A.1.6) incorpora a sugestão #1 do clipboard. O cache de styles (A.1.5, passo 5) incorpora a sugestão #5 do clipboard (HashMap). As mensagens de erro incluem path absoluto e causa aproximada.
- **Honestidade epistêmica**: Se hayagriva não expuser `Style::from_xml`, este passo scope-out imediatamente. Não se implementa parser CSL XML próprio (~5k LOC) em um passo M.
- **Próximo passo P421**: `repr()` completo (S) ou `link` render visual (S) ou `text.lang` rustybuzz (XL, scope-out).
