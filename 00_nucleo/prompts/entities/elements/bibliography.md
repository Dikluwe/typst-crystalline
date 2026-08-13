# Prompt L0 — `entities/elements/bibliography` — `BibliographyElem`
Hash do Código: ba379e19

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/bibliography.rs`
**Origem**: modelo D (ADR-0105), **Lote 10 P325** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. Contentor — `map_*` recursam no `title`.

> **Fronteira: LOCATÁVEL** (P181C, M6). Absorve o braço de `extract_payload` no
> trait (precedente Heading/Lote 6): `element_kind` → `ElementKind::Bibliography`,
> `to_payload` → `ElementPayload::Bibliography { entries }`. O consumo por
> `ElementPayload` (`from_tags` arm Bibliography → `BibStore`) é **inalterado**
> (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BibliographyElem {
    pub entries: Vec<BibEntry>,          // P159A — input literal; P419 preenche via path
    pub path:    Option<EcoString>,      // P419 — path do ficheiro .bib/.yaml/.json
    pub title:   Option<Content>,        // era Option<Box<Content>>
    pub style:   Option<EcoString>,      // P418 — CSL built-in ("ieee", "apa", ...)
    pub locale:  Option<EcoString>,      // P418 — locale override ("en-US", "pt-PT", ...)
}
```

`Content::Bibliography { entries, title }` →
`Content::Bibliography(Arc<BibliographyElem>)`. Construtores ergonómicos:
`Content::bibliography(entries, title)` (fallback local),
`Content::bibliography_with_style(entries, title, style, locale)` (CSL),
e `Content::bibliography_from_path(path, title, style, locale)` (P419 — carrega
entries em eval time).
**Deriva `Hash`** (`BibEntry` deriva `Eq + Hash` — `bib_entry.rs:80`; `Content`
tem `impl Hash` manual).

## `impl Element for BibliographyElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | **custom** (`content.rs:1810`): `title` (se houver) + `\n`, depois cada entry formatada `"[{key}] {author}. {title} ({year}).\n"` |
| `is_empty` | **override**: `self.entries.is_empty() && self.title.is_none()` (`content.rs:1654`) |
| `map_content` | **recursivo** no `title`, preserva `entries` (`content.rs:2313`) |
| `map_text` | **recursivo** no `title`, preserva `entries` (`content.rs:2564`) |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Bibliography)` |
| `to_payload` | `Some(ElementPayload::Bibliography { entries: self.entries.clone() })` (absorve `extract_payload.rs:77`) |

## `eq`

`#[derive(PartialEq)]` compara `entries`/`title` (paridade `content.rs:1967`).

---

## P418 (XL) — Renderização CSL real

**Decisão arquitetural (ADR-0107 / ADR-0108 / ADR-0109):**
- Usar `hayagriva` crate (Opção α) para parsing de `.bib`/`.yaml`/`.json` e CSL engine.
- Paridade é linguística (`.bib` + `@key` → citações/bibliografia renderizadas), não mecânica.
- Lógica de render vive em `compiler/layout/bibliography.rs` (forma B, free function).

**Scope-out P418:**
- CSL style via URL; múltiplos arquivos de bibliografia no mesmo doc; `title` customizado; CSL locales.
- `BibliographyElem` continua carregando `Vec<BibEntry>` interno; a conversão para hayagriva acontece no layout/eval.

## P419 (M) — Loading de `.bib`/`.yaml`/`.json` de disco

**Decisão arquitetural (ADR-0107 / ADR-0108 / ADR-0109):**
- Loader específico para bibliography em `rules/eval/bibliography.rs` (forma B,
  free function). Reusa `World::read_bytes(current_file, path)` (L3) para ler
  bytes e `hayagriva::io` para parsear em L1.
- `BibliographyElem` ganha `path: Option<EcoString>`; `entries` é preenchido em
  eval time a partir do path. O campo `entries` é mantido para compatibilidade
  com input literal P159A/P418 (scope-out parcial: paridade linguagem do path
  sem remover a API interna de entries).
- Path relativo resolvido pelo `World` (relativo ao `current_file`); path
  absoluto usado diretamente.

**Scope-out P419:**
- File loader genérico para outras features (imagens/dados já têm; bibliografia
  reusa o mesmo mecanismo via `World::read_bytes`).
- URLs/network, watch/reload, encoding detection, macros BibTeX complexos.

## P420 (M) — CSL customizado via path

**Decisão arquitetural (ADR-0107 / ADR-0108 / ADR-0109):**
- `BibliographyElem` **inalterado** — continua a armazenar `style: Option<EcoString>`. A
  distinção entre built-in e ficheiro `.csl` é feita em *eval time*, não no struct
  (atomização forma B).
- A resolução de style vive em `rules/eval/bibliography.rs` como free functions
  (`resolve_style`, `load_csl_from_path`, `resolve_csl_path`).
- Algoritmo: tenta `hayagriva::archive::ArchivedStyle::by_name` primeiro; se falhar,
  trata a string como path relativo/absoluto, lê via `World::read_bytes` e parseia com
  `hayagriva::citationberg::IndependentStyle::from_xml`.
- Erros legíveis: built-in desconhecido, file not found, encoding inválido, XML
  malformado.

**Scope-out P420:**
- CSL via URL (`http://...`), diretórios de sistema (`~/.csl/`), múltiplos styles
  simultâneos, hot-reload, validação completa RelaxNG, cache persistente cross-run.

## Histórico de revisões

| Data | Motivo | Arquivos |
|------|--------|----------|
| 2026-06-23 | P418 (XL): adicionar seção de renderização CSL real e scope-out. | `bibliography.md`, `bibliography.rs`, `cite.md`, `cite.rs`, `loading.md`, `loading.rs` |
| 2026-06-23 | P419 (M): adicionar `path`, loading de disco e scope-out. | `bibliography.md`, `bibliography.rs`, `content.rs`, `rules/eval/bibliography.rs`, `rules/stdlib/structural.rs` |
| 2026-06-23 | P420 (M): CSL customizado via path; `BibliographyElem` inalterado; resolução em eval. | `bibliography.md`, `rules/eval/bibliography.rs`, `compiler/layout/bib_csl.rs`, `rules/stdlib/structural.rs` |
