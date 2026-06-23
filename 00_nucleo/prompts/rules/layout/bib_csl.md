# Prompt L0 — `rules/layout/bib_csl` — Renderização CSL via hayagriva

Hash do Código: 6cb5ec8f

**Camada**: L1 · **Alvo**: `01_core/src/rules/layout/bib_csl.rs`
**Origem**: P418 (XL) — Bibliography/Cite CSL real.
**ADRs**: ADR-0062 (autorização de `hayagriva` em L1), ADR-0107 (paridade linguagem),
ADR-0109 (atomização forma B — free function em `rules/layout/`).

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
```

- `build_cache` pré-renderiza as 4 forms de citação (`Normal`, `Prose`, `Author`,
  `Year`) e a bibliografia num único passo, permitindo que `Cite` consuma o
  mesmo cache que `Bibliography` sem dependência de ordem no documento.
- Style built-in resolvido por `hayagriva::archive::ArchivedStyle`; default
  `"ieee"`.
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

- Ficheiros `.csl` customizados via path.
- Múltiplas bibliografias com styles diferentes simultâneos.
- Content variants para `Superscript`/`Subscript` (formatação vertical CSL).
- Locales via ficheiro externo (locale built-in apenas).

## Histórico de revisões

| Data | Motivo | Arquivos |
|------|--------|----------|
| 2026-06-23 | P418 (XL): criar prompt L0 do módulo CSL. | `bib_csl.md`, `bib_csl.rs` |
