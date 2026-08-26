# Prompt L0 — `compiler/layout/bib_csl` — Renderização CSL via hayagriva
Hash do Código: 78ae94b8


**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/bib_csl.rs`
**Origem**: P418 (XL) — Bibliography/Cite CSL real.
**ADRs**: ADR-0062 (autorização de `hayagriva` em L1), ADR-0107 (paridade linguagem),
ADR-0109 (atomização forma B — free function em `compiler/layout/`).

---

## Propósito

Ponte entre o modelo de dados cristalino (`BibEntry`, `CitationForm`) e o motor
CSL do `hayagriva`/`citationberg`. O módulo é puro (zero I/O): recebe
`Vec<BibEntry>` + style/locale, devolve `Content` já formatado para citações e
lista de referências.

## API pública

```rust
pub struct BibRenderCache {
    pub citations: HashMap<CitationForm, HashMap<String, Content>>,
    pub bibliography: Option<Content>,
}

pub fn build_cache(
    entries: &[BibEntry],
    style: Option<&str>,
    locale: Option<&str>,
) -> Option<BibRenderCache>;

pub fn resolve_style_name(name: &str) -> Option<IndependentStyle>;

pub fn parse_csl_style(content: &str) -> Result<IndependentStyle, String>;
```

- `build_cache` pré-renderiza as 4 forms de citação suportadas pelo cristalino
  (`Normal`, `Prose`, `Author`, `Year`) e a bibliografia num único passo, permitindo
  que `Cite` consuma o mesmo cache que `Bibliography` sem dependência de ordem no
  documento.
- Style built-in resolvido por `hayagriva::archive::ArchivedStyle`; default
  `"ieee"`.

> **Fonte de paridade (P1031)** — vanilla ratificado `e0e8ca4d`:
>
> - **Forms de citação — "4" é o subconjunto do cristalino, não o da linguagem.** O vanilla
>   define **cinco** variantes em `crates/typst-library/src/model/cite.rs:133-147`
>   (`#[derive(Cast)] pub enum CitationForm`), com doc comments que são o texto publicado em
>   `typst.app/docs/reference/model/cite/#parameters-form`: `Normal` — *"Display in the
>   standard way for the active style."* (`#[default]`); `Prose` — *"Produces a citation
>   that is suitable for inclusion in a sentence."*; `Full` — *"Mimics a bibliography entry,
>   with full information about the cited work."*; `Author` — *"Shows only the cited work's
>   author(s)."*; `Year` — *"Shows only the cited work's year."*
>   A ausência de `Full` é **scope-out declarado** em `entities/citation_form.md`
>   §"Divergência do original" (ADR-0054 graded), não uma afirmação de que a linguagem tem
>   quatro forms. Redacção acima corrigida para o dizer.
> - **Default `"ieee"`** — `crates/typst-library/src/model/bibliography.rs:159-163`:
>   `#[default({ let default = ArchivedStyle::InstituteOfElectricalAndElectronicsEngineers; … })]`
>   sobre o campo `pub style: Derived<CslSource, CslStyle>`. O doc comment da tabela de
>   estilos (`bibliography.rs:79-95`) publica `{"ieee"}` como o estilo típico de
>   *"Engineering, IT"*. Página: `typst.app/docs/reference/model/bibliography/#parameters-style`.
- Style custom resolvido a partir de ficheiro `.csl` XML via
  `hayagriva::citationberg::IndependentStyle::from_xml` (P420).
- Locale override opcional via `hayagriva::archive::locales()`.

## Conversão de dados

- `BibEntry → hayagriva::Entry` via YAML intermédio + `hayagriva::io::from_yaml_str`.
- `ElemChildren → Content` suporta: `Strong`, `Emph`, `SmallCaps`, `Underline`,
  `Link`, `Linebreak`, texto plano.

## Integração no layout

- `Layouter` guarda `Option<BibRenderCache>`.
- `layout_with_introspector` pré-computa o cache no primeiro `BibliographyElem`
  que especificar `style` explicitamente.
- `bibliography.rs` e `cite.rs` consomem o cache; fallback local (`format_bib_entry`)
  preservado quando `style` é `None` ou o cache falha.

## Scope-out

- Múltiplas bibliografias com styles diferentes simultâneos.
- Content variants para `Superscript`/`Subscript` (formatação vertical CSL).
- Locales via ficheiro externo (locale built-in apenas).
- CSL via URL, diretórios de sistema, hot-reload e validação RelaxNG completa.

## P420 (M) — CSL customizado via path

**Decisão arquitetural (ADR-0107 / ADR-0108 / ADR-0109):**
- `build_cache` mantém a assinatura simples `style: Option<&str>` (built-in).
  Para styles já resolvidos (built-in ou custom), `build_cache_with_style`
  recebe o `IndependentStyle`.
- Built-in primeiro, path como fallback (Opção α): `resolve_style_name` tenta
  `ArchivedStyle::by_name`; em caso de insucesso o eval em
  `rules/eval/bibliography.rs` trata a string como path, lê via
  `World::read_bytes` e parseia com `parse_csl_style`.
- O módulo continua puro de I/O global — a leitura do disco é feita pelo caller
  através de `World::read_bytes`; `bib_csl.rs` expõe apenas parsing e render.

## Histórico de revisões

| Data | Motivo | Arquivos |
|------|--------|----------|
| 2026-06-23 | P418 (XL): criar prompt L0 do módulo CSL. | `bib_csl.md`, `bib_csl.rs` |
| 2026-06-23 | P420 (M): adicionar resolução de `.csl` customizado via path. | `bib_csl.md`, `bib_csl.rs`, `rules/eval/bibliography.rs`, `compiler/layout/mod.rs`, `rules/stdlib/structural.rs` |
