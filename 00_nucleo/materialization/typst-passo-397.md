# Passo 397 — Materialização: `document`/`title`/`asset` (M)

**Tipo**: Materialização (L1 — stdlib + model metadata; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica — sem variant novo em Value), ADR-0054 (graded scope-out).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `document`/`title`/`asset`, M, zero dependências.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Este passo fecha a **fila limpa original** (balde D, sonda 389): square → lorem → panic → #show regex → eval → document/title/asset. Após P397, a fila D está 100% fechada.

---

## 1. Contexto

A sonda 389 identificou `document`/`title`/`asset` como dívida genuína acidental (balde D), M, zero dependências. No vanilla:

```typ
#set document(title: [Hello])
#document(title: [Hello], author: "Ana")
```

`document` é o **wrapper de metadata** do documento — título, autor, data, keywords, etc. No cristalino, `document` já existe como conceito implícito (o root do layout), mas não como elemento manipulável pelo usuário.

Este passo é o **fecho da fila limpa** — último item do balde D da sonda 389. Após ele, a fila original está completa e o projeto entra em nova fase (tipos S pendentes + features com dependências reais).

---

## 2. Decisão de engenharia

### 2.1 — `document` é metadata, não layout

No vanilla, `document` é um elemento especial:
- Não produz output visual direto (não é um frame item).
- Armazena metadados que afetam o PDF export (title, author, keywords, date, etc.).
- Pode ser set via `#set document(...)` ou instanciado diretamente.
- O vanilla usa `DocumentElem` com vtable; o cristalino usa `Content::Document` enum variant.

**Decisão arquitetural**: `document` é um **Content variant** (não Value), porque é um elemento do documento — pode aparecer no markup, ser manipulado por show rules, etc. É paralelo a `Heading`, `Figure`, etc.

### 2.2 — `title` e `asset` — campos de document

No vanilla:
- `title` é um atributo de `document` (e também de `heading`, `figure`, etc. — contexto distinto).
- `asset` não é um elemento vanilla direto — é uma extensão do cristalino para resources externos (imagens, fonts, etc. embutidos).

**Decisão ADR-0107 (língua vs mecânica)**:
- `document` é a **forma** — wrapper de metadata.
- `title` dentro de `document` é **mecânica de transporte** — o valor semântico é o texto do título, não a struct `title`.
- `asset` é **extensão cristalina** — não existe no vanilla como elemento separado; é um helper para resources. Documentar como divergência intencional (ADR-0033).

### 2.3 — Estrutura `Content::Document`

```rust
// entities/content.rs — novo variant
Document {
    title: Option<Box<Content>>,      // Content::Text ou styled
    author: Vec<EcoString>,           // autores múltiplos
    date: Option<Datetime>,           // reusa Datetime existente (P21)
    keywords: Vec<EcoString>,
    // asset é scope-out ou campo separado — ver decisão abaixo
}
```

**Decisão**: `asset` não é campo de `Document`. É um **elemento separado** `Content::Asset` (ou `Value::Asset`) para resources externos. No vanilla, assets são implicitamente gerenciados pelo compiler (fontes, imagens, etc.). No cristalino, explicitar como `asset(path)` permite controle user-facing.

**Simplificação P397**: `asset` como `Content::Asset { path: EcoString }` — placeholder para resource registry. Não implementa registry real (scope-out ADR-0054 graded); apenas modela o elemento.

### 2.4 — `title` como função stdlib vs campo

No vanilla, `title` é usado em múltiplos contextos:
- `#set document(title: [...])` — metadata do documento.
- `#heading(title: [...])` — título de seção (sinônimo de `body` em heading).
- `#figure(caption: [...])` — legenda (não `title`).

**Decisão**: `title` **não é função stdlib independente** neste passo. É um **campo de `document`** e um **campo de `heading`** (já existe em `HeadingElem` desde P22). O stdlib `#title(...)` é scope-out — o vanilla não tem `title()` como função standalone; é sempre atributo.

**Correção**: se a sonda 389 listou `title` como item ausente separado, verificar se é `title` como função stdlib ou como campo. Se for função, é scope-out (não existe no vanilla). Se for campo, já está coberto por `heading`/`document`.

**Decisão para P397**: implementar `document` como stdlib + Content variant. `title` é campo de `document`. `asset` é stdlib + Content variant separado (extensão cristalina). Não implementar `title()` como função standalone.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `document.md`

Novo em `00_nucleo/prompts/engine/model/document.md`:

- **Paridade**: `document(title: [...], author: "...", date: datetime(...), keywords: ("...",))` ≡ vanilla `DocumentElem` morfologicamente.
- **Substrato**: Content variant `Document` + stdlib `native_document` + PDF metadata export stub.
- **Sem tipo novo**: reusa `Content::Text`, `Datetime`, `EcoString`.
- **Campos**:
  - `title` (opcional): `Content` — `None` ↔ sem título.
  - `author` (opcional): `Array[Str]` ou `Str` — `Vec<EcoString>`.
  - `date` (opcional): `Datetime` — reusa P21.
  - `keywords` (opcional): `Array[Str]` — `Vec<EcoString>`.
- **Comportamento**: `document` não emite frame items — é metadata pura. Armazenado no `Layouter` ou `PageState` para export PDF.
- **Set rule**: `#set document(title: [...])` funciona via `SetDocument` style rule (reusa infraestrutura de set rules existente).
- **Erro**: argumentos desconhecidos rejeitados; tipos errados rejeitados.
- **Teste**: `document(title: [Hello])` parse+eval → Content::Document; repr mostra struct; export PDF stub lê metadata.

### A.2 — Prompt L0 `asset.md`

Novo em `00_nucleo/prompts/engine/model/asset.md` (ou `visualize/` se apropriado):

- **Paridade**: `asset(path)` é extensão cristalina — não existe no vanilla como elemento. Documentar como divergência intencional (ADR-0033).
- **Substrato**: Content variant `Asset` + stdlib `native_asset`.
- **Sem tipo novo**: reusa `EcoString`.
- **Campos**:
  - `path` (obrigatório): `Str` — caminho do resource.
  - `kind` (opcional): `Str` — "image", "font", "data", etc. — default inferido da extensão.
- **Comportamento**: placeholder para resource registry. Não implementa registry real (scope-out ADR-0054 graded). Apenas modela o elemento para futuro uso.
- **Teste**: `asset("logo.png")` parse+eval → Content::Asset; repr mostra struct.

### A.3 — CHECKPOINT

Parar. Apresentar `document.md` + `asset.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Content variant `Document`

Em `entities/content.rs`:

```rust
Document {
    title: Option<Box<Content>>,
    author: Vec<EcoString>,
    date: Option<Datetime>,
    keywords: Vec<EcoString>,
}
```

**Derives**: `Clone`, `PartialEq` (via fields), `Debug`.
**Plain text**: title plain_text se Some, else empty.
**Map content**: map sobre title (se Some).

**Arms necessários**:
- `PartialEq` — match Document { title, author, date, keywords }.
- `map_content` — map sobre title.
- `plain_text` — title plain_text ou empty.
- `layout` — no-op (metadata não emite).
- `repr` — `"document(...)"` ou struct detalhada.

### B.2 — Content variant `Asset`

Em `entities/content.rs`:

```rust
Asset {
    path: EcoString,
    kind: Option<EcoString>,  // "image" | "font" | "data" | None (infer)
}
```

**Derives**: `Clone`, `PartialEq`, `Debug`.
**Plain text**: empty (não é texto).
**Map content**: no-op (não contém Content aninhado).

### B.3 — Stdlib `native_document`

Em `rules/stdlib/model.rs` (ou `structural.rs` se não houver model.rs; confirmar path):

```rust
pub fn native_document(args: &Args) -> SourceResult<Value> {
    let title = args.find::<Content>("title")
        .map(Box::new);
    let author = args.find::<Value>("author")
        .map(|v| extract_author_list(&v))
        .transpose()?
        .unwrap_or_default();
    let date = args.find::<Datetime>("date");
    let keywords = args.find::<Value>("keywords")
        .map(|v| extract_keywords(&v))
        .transpose()?
        .unwrap_or_default();

    Ok(Value::Content(Content::Document {
        title,
        author,
        date,
        keywords,
    }))
}

fn extract_author_list(value: &Value) -> SourceResult<Vec<EcoString>> {
    match value {
        Value::Str(s) => Ok(vec![s.clone()]),
        Value::Array(arr) => arr.iter()
            .map(|v| v.cast::<Str>().map(|s| s.clone()))
            .collect::<Result<_, _>>(),
        _ => bail!("author deve ser string ou array de strings"),
    }
}

fn extract_keywords(value: &Value) -> SourceResult<Vec<EcoString>> {
    match value {
        Value::Str(s) => Ok(vec![s.clone()]),
        Value::Array(arr) => arr.iter()
            .map(|v| v.cast::<Str>().map(|s| s.clone()))
            .collect::<Result<_, _>>(),
        _ => bail!("keywords deve ser string ou array de strings"),
    }
}
```

### B.4 — Stdlib `native_asset`

```rust
pub fn native_asset(args: &Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let kind = args.find::<Str>("kind")
        .or_else(|| infer_asset_kind(&path));

    Ok(Value::Content(Content::Asset {
        path: path.clone(),
        kind,
    }))
}

fn infer_asset_kind(path: &str) -> Option<EcoString> {
    let ext = path.rsplit('.').next()?;
    match ext.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "svg" => Some(EcoString::from("image")),
        "ttf" | "otf" | "woff" | "woff2" => Some(EcoString::from("font")),
        "json" | "yaml" | "csv" | "xml" => Some(EcoString::from("data")),
        _ => None,
    }
}
```

### B.5 — Set rule `document`

Reusa infraestrutura de set rules existente (P102, ADR-0040):

```rust
// Em eval/rules.rs ou equivalente
// SetDocument { title, author, date, keywords } → Style::DocumentMeta(...)
// Ou: Content::SetDocument { ... } → similar a SetHeadingNumbering
```

**Decisão**: se o cristalino usa `Style` enum para set rules (ADR-0038), adicionar `Style::DocumentMeta { ... }` ou similar. Se usa `Content::Set*` variants (como `SetHeadingNumbering`), adicionar `Content::SetDocument`.

**Simplificação**: se a infraestrutura de set rules não suporta metadata facilmente, implementar `document` como **elemento standalone** primeiro (funciona como `#document(...)`). Set rule `#set document(...)` é refino futuro (S) — não bloqueia este M.

### B.6 — Export PDF stub (metadata)

Em `export.rs` (ou `stream.rs`):

```rust
// Após layout, antes de emitir PDF:
// Se houver Content::Document no root, extrair metadata para PDF Info dict.
// Stub: println! ou comentário indicando onde injectar metadata.
// Scope-out ADR-0054 graded: metadata real PDF é XL (requires PDF lib integration).
```

**Decisão**: não implementar metadata PDF real neste passo. Apenas garantir que `Content::Document` é preservado no pipeline (não dropado). O stub é um comentário `// TODO: inject metadata into PDF Info dict` no export.

### B.7 — Registar em `make_stdlib`

```rust
scope.define("document", Func::native(native_document));
scope.define("asset", Func::native(native_asset));
```

### B.8 — Testes

1. **Unit content** (4-6 tests em `entities/content.rs`):
   - `document_variant_ctor` — `Document { title: Some(...), author: vec![], date: None, keywords: vec![] }`.
   - `document_partial_eq` — igualdade por campos.
   - `document_map_content` — map sobre title.
   - `document_plain_text` — title plain_text ou empty.
   - `asset_variant_ctor` — `Asset { path: "logo.png", kind: Some("image") }`.
   - `asset_partial_eq` — igualdade por path.

2. **Unit stdlib** (6-8 tests em `stdlib/mod.rs` ou `model.rs`):
   - `native_document_title` — `document(title: [Hello])` retorna Content::Document.
   - `native_document_author_str` — `document(author: "Ana")` → vec!["Ana"].
   - `native_document_author_array` — `document(author: ("Ana", "Bob"))` → vec!["Ana", "Bob"].
   - `native_document_date` — `document(date: datetime(...))` → Some(Datetime).
   - `native_document_keywords` — `document(keywords: ("a", "b"))` → vec!["a", "b"].
   - `native_document_no_args` — `document()` → todos defaults.
   - `native_asset_path` — `asset("logo.png")` → Content::Asset.
   - `native_asset_kind_explicit` — `asset("x", kind: "font")` → kind override.
   - `native_asset_kind_infer` — `asset("logo.png")` → kind Some("image").
   - `native_asset_invalid` — `asset(123)` → erro tipo.

3. **E2E / integration** (2-3 tests):
   - `document_como_root` — parse + eval de `#document(title: [T])` → Content::Document no root.
   - `document_repr` — `repr(document(title: [T]))` retorna string esperada.
   - `asset_no_output` — `asset("x")` não emite frame items.

### B.9 — Linhagem

- `@prompt` aponta para `document.md` + `asset.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P21 (Datetime), P22 (Heading), P102 (Set rules), ADR-0033 (divergência intencional), ADR-0107 (paridade linguagem).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar metadata PDF real no export — scope-out ADR-0054 graded (XL futuro).
- **Não** implementar `title()` como função stdlib standalone — não existe no vanilla; é campo de document/heading.
- **Não** implementar resource registry real para `asset` — scope-out ADR-0054 graded.
- **Não** implementar `#set document(...)` set rule — refino futuro S; não bloqueia este M.
- **Não** adicionar `Value` variant — reusa `Content` variant.
- **Não** tocar em layout/render de document — document é metadata, não emite frames.
- **Não** fazer tipos S (Bytes, Decimal, etc.) — são P398+.
- **Não** numerar passos à frente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `document(title: [Hello])` compila e retorna `Content::Document`.
2. `asset("logo.png")` compila e retorna `Content::Asset`.
3. Zero tipo novo em `Value`; zero I/O; zero variant novo em `Value`.
4. `document` não emite frame items (metadata pura).
5. `asset` não emite frame items (placeholder).
6. Testes verdes (≥12 unit + 2-3 integration); lint zero; hashes propagados.
7. Inventário 148: `document`/`title`/`asset` transita `ausente` → `implementado` (ou `implementado⁺` se graded).
8. L0 salvo e hashado antes do código (protocolo de nucleação).
9. Fila limpa original (balde D, sonda 389) **100% fechada**.

---

## 7. O que pode sair errado

- **`Datetime` não é `Copy` ou não existe em L1.** Mitigação: verificar P21; se Datetime não existe, usar `Option<Str>` para date como placeholder (paridade ADR-0054 graded).
- **Infraestrutura de set rules não suporta metadata.** Mitigação: scope-out `#set document(...)` para passo futuro S; `document()` standalone é suficiente para este M.
- **Content variants já são 62+ — adicionar 2 aumenta o enum.** Mitigação: é esperado; Document e Asset são variants legítimos. Garantir match exhaustivo em todos os arms.
- **Tentação de implementar metadata PDF real.** Mitigação: ADR-0054 graded; stub é suficiente.
- **Tentação de já fazer tipos S (Bytes, Decimal, etc.).** Mitigação: um passo de cada vez; fila limpa fecha aqui.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `document`/`title`/`asset` como M, zero deps.
- P21 — `Datetime` tipo (reuso).
- P22 — `Heading` (title como campo paralelo).
- P102 — Set rules infraestrutura (reuso ou scope-out).
- P390-396 — fila limpa original (square → lorem → panic → show regex → eval → tiling → document).
- ADR-0033 — divergência intencional (asset como extensão cristalina).
- ADR-0107 — paridade linguagem vs mecânica (title é campo, não função).
- ADR-0054 — graded scope-out (metadata PDF real, resource registry).

---

## 9. Nota sobre o Tekt

Este passo é o **fecho da fila limpa original** — o último item do balde D da sonda 389. Após P397:
- Fila D: 100% completa (7/7 passos: square, lorem, panic, show regex, eval, tiling, document).
- Portão ADR-0017: aberto (P395).
- Próxima fase: tipos S (Bytes, Decimal, Duration, Version) em P398-P401, ou features com dependências reais (bibliography CSL, shaping, etc.).

Registar o marco: **fila limpa fechada**. O método Tekt provou que consegue executar uma fila de features independentes sem deriva — cada passo fechou completo, testado, lintado, e documentado.
