---

# P502 — Materialização de Gaps S/XS Restantes (P500)

> **Passo:** 502
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 4 gaps de tamanho S/XS identificados no audit P500: `image.fit`, `#raw(lang:, block:)`, `footnote.numbering`, e `outline.indent` API. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S-size com validação via bateria P500.
> **Tamanho:** S (~30 min de implementação + 10 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P501 (P1/P2 fechados), P500 (audit de cobertura), P247 (stroke), P304/P305 (footnotes), P419/P450 (bibliography).

---

## Metodologia

Para cada sub-tarefa, implementar o gap, correr a bateria P500 (17 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico P501.

```bash
# Baseline P501 (antes de tocar código):
cargo test -p typst-parity p501_gaps_p1_p2

# Após cada sub-tarefa:
cargo test -p typst-parity p502_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — `image.fit` (S)

### 1.1 — Estado baseline (P501)

```typst
// test-image-fit.typ
#image("test.png", width: 50%, fit: "contain")
```

- **Vanilla:** `ok` — image renderiza com `fit: contain`.
- **Cristalino (pré-502):** `ERRO_DESCRITIVO` — `image(): argumento nomeado inesperado 'fit'`.
- **Classificação P501:** **AUSENTE**

### 1.2 — Diagnóstico de causa raiz

O construtor `image()` aceita `width`, `height` mas não `fit`. O gap é:

1. **H1:** O parser de `image()` rejeita `fit` como arg nomeado desconhecido.
2. **H2:** O eval de `image()` não processa `fit`.

### 1.3 — Implementação

**Arquivo alvo:** `src/stdlib/structural.rs` (ou onde `native_image` é definido)

**Mudança:** Adicionar `fit: Str` como arg nomeado opcional:

```rust
fn native_image(args: Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let width = args.named.get("width").and_then(|v| v.cast::<Length>().ok());
    let height = args.named.get("height").and_then(|v| v.cast::<Length>().ok());
    let fit = args.named.get("fit")
        .and_then(|v| v.cast::<Str>().ok())
        .unwrap_or_else(|| "cover".into()); // default vanilla

    // Validar fit
    if !matches!(fit.as_str(), "contain" | "cover" | "stretch") {
        return Err("fit must be 'contain', 'cover' or 'stretch'");
    }

    Ok(Value::Content(Content::Image {
        path: path.into(),
        width,
        height,
        fit: fit.into(),
    }))
}
```

**Nota:** Se `Content::Image` não tem campo `fit`, adicioná-lo em `entities/content.md` / `src/model/content.rs`.

### 1.4 — Validação

Rodar `test-image-fit.typ` contra vanilla e cristalino. Esperado:
- `image("test.png", width: 50%, fit: "contain")` → **MATCH**
- `image("test.png", width: 50%)` → **MATCH** (default "cover" preservado)

---

## Categoria 2 — `#raw(lang:, block:)` (S)

### 2.1 — Estado baseline (P501)

```typst
// test-raw-advanced.typ
#raw("fn main() {}", lang: "rust", block: true)
#raw("inline", lang: "python", block: false)
```

- **Vanilla:** `ok` — bloco de código com linguagem e highlighting.
- **Cristalino (pré-502):** `ERRO_DESCRITIVO` — `argumento nomeado inesperado: 'lang'`.
- **Classificação P501:** **AUSENTE**

### 2.2 — Diagnóstico de causa raiz

O construtor `raw()` aceita apenas conteúdo posicional. `lang` e `block` não são processados. O gap é:

1. **H1:** O parser de `raw()` rejeita `lang`/`block` como args nomeados.
2. **H2:** O eval de `raw()` não processa `lang`/`block`.

### 2.3 — Implementação

**Arquivo alvo:** `src/stdlib/structural.rs` (ou onde `native_raw` é definido)

**Mudança:** Adicionar `lang: Str` e `block: Bool` como args nomeados opcionais:

```rust
fn native_raw(args: Args) -> SourceResult<Value> {
    let content = args.expect::<Str>("content")?;
    let lang = args.named.get("lang")
        .and_then(|v| v.cast::<Str>().ok());
    let block = args.named.get("block")
        .and_then(|v| v.cast::<Bool>().ok())
        .unwrap_or(true); // default para blocos

    Ok(Value::Content(Content::Raw {
        text: content.into(),
        lang,
        block,
    }))
}
```

**Nota:** Se `Content::Raw` não tem campos `lang`/`block`, adicioná-los. O highlighting pode ser scope-out (fallback monocromo), mas `lang` e `block` devem ser armazenados.

### 2.4 — Validação

Rodar `test-raw-advanced.typ` contra vanilla e cristalino. Esperado:
- `raw("fn main() {}", lang: "rust", block: true)` → **MATCH**
- `raw("inline", lang: "python", block: false)` → **MATCH**
- Triple-backtick com linguagem → **MATCH** (não-regressão P490)

---

## Categoria 3 — `footnote.numbering` (XS)

### 3.1 — Estado baseline (P501)

```typst
// test-footnote-advanced.typ
Texto com nota numerada.#footnote(numbering: "1")[Outra nota.]
```

- **Vanilla:** `ok` — footnote com numbering customizado.
- **Cristalino (pré-502):** `ERRO_DESCRITIVO` — `footnote(): argumento nomeado 'numbering' não suportado`.
- **Classificação P501:** **AUSENTE**

### 3.2 — Diagnóstico de causa raiz

O construtor `footnote()` aceita corpo posicional mas não `numbering`. O gap é trivial:

1. **H1:** Adicionar `numbering: Str` como arg nomeado opcional.
2. **H2:** Armazenar `numbering` em `Content::Footnote` para uso no layout.

### 3.3 — Implementação

**Arquivo alvo:** `src/stdlib/structural.rs` (ou onde `native_footnote` é definido)

**Mudança:** Adicionar `numbering: Str` como arg nomeado opcional:

```rust
fn native_footnote(args: Args) -> SourceResult<Value> {
    let body = args.expect::<Content>("body")?;
    let numbering = args.named.get("numbering")
        .and_then(|v| v.cast::<Str>().ok());

    Ok(Value::Content(Content::Footnote {
        body,
        numbering,
    }))
}
```

**Nota:** Se `Content::Footnote` não tem campo `numbering`, adicioná-lo. O layout de footnote pode ignorar `numbering` por enquanto (scope-out de render), mas o arg deve ser aceito.

### 3.4 — Validação

Rodar `test-footnote-advanced.typ` contra vanilla e cristalino. Esperado:
- `footnote(numbering: "1")[Nota]` → **MATCH** (arg aceito, sem erro)
- `footnote[Nota]` → **MATCH** (não-regressão P490)

---

## Categoria 4 — `outline.indent` API (S)

### 4.1 — Estado baseline (P501)

```typst
// test-outline-advanced.typ
#outline(
  title: [Índice],
  depth: 2,
  indent: true,
)
= Capítulo 1
== Secção 1.1
= Capítulo 2
```

- **Vanilla 0.14.2:** `ERRO_DESCRITIVO` — `indent` espera `length | function | auto`, não `bool`.
- **Cristalino (pré-502):** `ERRO_DESCRITIVO` — `indent` aceita `bool` (implementação cristalino divergente).
- **Classificação P501:** **DIFF** (divergência de API)

### 4.2 — Diagnóstico de causa raiz

O cristalino implementou `outline(indent: bool)` enquanto o vanilla 0.14.2 espera `indent: length | function | auto`. Esta é uma **divergência de API** documentada no P500.

**Decisão:** Alinhar com o vanilla 0.14.2 (ou 0.15.0 se o changelog indicar mudança). Se o vanilla 0.15.0 manteve `length | function | auto`, o cristalino deve aceitar os mesmos tipos.

### 4.3 — Implementação

**Arquivo alvo:** `src/stdlib/structural.rs` (ou onde `native_outline` é definido)

**Mudança:** Alterar `indent` para aceitar `Length | Function | Auto | Bool` (compatibilidade reversa com código cristalino existente):

```rust
fn native_outline(args: Args) -> SourceResult<Value> {
    let title = args.named.get("title").and_then(|v| v.cast::<Content>().ok());
    let depth = args.named.get("depth")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(3);
    let indent = args.named.get("indent");

    // Validar indent: Length | Function | Auto | Bool (compatibilidade)
    let indent_value = match indent {
        Some(Value::Bool(b)) => OutlineIndent::Bool(*b), // compatibilidade cristalino
        Some(Value::Length(l)) => OutlineIndent::Length(*l),
        Some(Value::Func(f)) => OutlineIndent::Function(f.clone()),
        Some(Value::Auto) => OutlineIndent::Auto,
        None => OutlineIndent::Auto, // default
        _ => return Err("indent must be length, function, auto, or bool"),
    };

    Ok(Value::Content(Content::Outline {
        title,
        depth: depth as usize,
        indent: indent_value,
    }))
}
```

**Nota:** Se `Content::Outline` ou `OutlineIndent` não existem, adicionar. A renderização de `indent` como `Length` ou `Function` pode ser scope-out, mas o arg deve ser aceito.

### 4.4 — Validação

Rodar `test-outline-advanced.typ` contra vanilla e cristalino. Esperado:
- `outline(title: [Índice], depth: 2)` → **MATCH** (sem indent)
- `outline(indent: auto)` → **MATCH** (vanilla 0.14.2)
- `outline(indent: 1em)` → **MATCH** (vanilla 0.14.2)
- `outline(indent: true)` → **MATCH** (compatibilidade cristalino)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 502a image.fit | Vanilla: ok | Cristalino: ok | MATCH | contain/cover/stretch |
| 502b raw(lang:, block:) | Vanilla: ok | Cristalino: ok | MATCH | lang e block aceitos |
| 502c footnote.numbering | Vanilla: ok | Cristalino: ok | MATCH | arg aceito, scope-out render |
| 502d outline.indent API | Vanilla: ok | Cristalino: ok | MATCH | length|function|auto|bool |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## Testes de não-regressão

Após as 4 sub-tarefas, rodar a bateria completa P500 (17 ficheiros):

| Ficheiro | Vanilla | Cristalino pré-502 | Esperado pós-502 | Δ |
|---|---|---|---|---|
| test-image-fit.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-raw-advanced.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-footnote-advanced.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-outline-advanced.typ | ok | DIFF | ok | **DIFF → MATCH** |
| (outros 13) | — | MATCH/AUSENTE | preservados | sem regressão |

**AUSENTEs restantes esperados:** 6 - 4 = **2** (list/enum indent, state/counter/context).  
**PANICs esperados:** **0** (preservado).

---

## Critério de fecho

- [ ] 502a implementado: `image.fit` aceita `contain`/`cover`/`stretch`.
- [ ] 502b implementado: `raw(lang:, block:)` aceita args nomeados.
- [ ] 502c implementado: `footnote(numbering:)` aceita arg nomeado.
- [ ] 502d implementado: `outline.indent` aceita `length|function|auto|bool`.
- [ ] 4 testes unitários novos passam (`p502_image_fit`, `p502_raw_lang_block`, `p502_footnote_numbering`, `p502_outline_indent_api`).
- [ ] Bateria P500: 4 AUSENTEs/DIFFs viraram MATCH.
- [ ] AUSENTEs restantes: 2 (list/enum indent, state/counter/context).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/structural.md` para image/raw/footnote/outline).
- [ ] ADR-0107 checklist atualizado (4 itens marcados implementado).
- [ ] Sentinela `p502_gaps_s_xs` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p502.md` produzido com tabela de resultados.

---

## Próximo passo (P503)

Com P502 fechado, os gaps restantes do P500 são:

| Prioridade | Gap | Tamanho | Recomendação |
|---|---|---|---|
| P2 | `list`/`enum` `indent`, `body-indent`, `tight` | M | P503 = layout de listas/enum |
| P3 | `state.update/get` + `context` + `counter` | L | P504 = runtime state (trilha separada) |

**Recomendação:** P503 = **list/enum indent** — M-size, fecha o último gap fechável do P500 antes de atacar o L-size `state`/`counter`/`context`.

Alternativa: P503 = **DEBT-42 Benchmark** — comparar tempo cristalino vs vanilla, avaliar impacto da passagem dupla do P498.

---

## A. Apêndice — Referência rápida dos gaps S/XS

```typst
// 502a: image.fit
#image("test.png", width: 50%, fit: "contain")

// 502b: raw(lang:, block:)
#raw("fn main() {}", lang: "rust", block: true)
#raw("inline", lang: "python", block: false)

// 502c: footnote.numbering
#footnote(numbering: "1")[Nota numerada]

// 502d: outline.indent API
#outline(title: [Índice], depth: 2, indent: auto)
#outline(title: [Índice], depth: 2, indent: 1em)
```
