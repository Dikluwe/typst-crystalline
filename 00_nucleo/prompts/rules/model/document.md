# Prompt L0 — `model/document` — metadata wrapper `document(...)`
Hash do Código: 880da8fd

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/structural.rs` + `01_core/src/entities/content.rs`
**Origem**: Passo 397 — materialização de `document(...)` (M); zero dependências.
**ADRs**: ADR-0033 (paridade / divergência intencional), ADR-0107 (paridade linguagem vs mecânica), ADR-0054 (graded scope-out).

---

## 1. Contexto

`document(...)` é o wrapper de metadata do documento no Typst (título, autor, data, keywords). Não produz output visual — é metadata pura. No cristalino modela-se como `Content::Document` (variante de Content, não novo tipo `Value`).

## 2. Content variant

```rust
Document {
    title: Option<Box<Content>>,
    author: Vec<EcoString>,
    date: Option<Datetime>,
    keywords: Vec<EcoString>,
}
```

- `title` opcional; `None` ↔ sem título.
- `author` pode vir de `Str` ou `Array[Str]`.
- `date` reusa `world_types::Datetime` (P21).
- `keywords` pode vir de `Str` ou `Array[Str]`.

Comportamento:
- `PartialEq` estrutural.
- `plain_text` devolve `title.plain_text()` ou vazio.
- `map_content` recursa no `title`.
- `is_empty` devolve `true` (metadata pura não é observable visualmente).

## 3. Stdlib `native_document`

`document(title:?, author:?, date:?, keywords:?)` → `Value::Content(Content::Document)`.

- Rejeita argumentos nomeados desconhecidos.
- `title`: `Value::Content` → `Some(Box::new(content))`; outro tipo → erro.
- `author`: `Value::Str` → vec![s]; `Value::Array` de Str → Vec<EcoString>; outro → erro.
- `date`: `Value::Datetime` → Some(d); outro → erro.
- `keywords`: idem a `author`.

## 4. Layout

`Content::Document` é metadata pura — `layout_content` e `measure_content_constrained` são no-op (sem frames, zero size).

## 5. Scope-out

- Metadata real no PDF Info dict — scope-out ADR-0054 graded (XL futuro).
- `#set document(...)` set rule — refino futuro S; não bloqueia este M.
- `title()` como função standalone — não existe no vanilla; é campo de `document`/`heading`.

## 6. Testes

- `document(title: [Hello])` → `Content::Document` com `title` Some.
- `document(author: "Ana")` → author vec!["Ana"].
- `document(author: ("Ana", "Bob"))` → author vec!["Ana", "Bob"].
- `document(keywords: ("a", "b"))` → keywords vec!["a", "b"].
- `document()` → todos os defaults.
- Argumento desconhecido → erro.
- Tipo errado em `author` → erro.
- E2E: `#document(title: [T])` não emite frames.
