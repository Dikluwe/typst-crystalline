//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/bibliography.md
//! @prompt-hash 1a1369d6
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de bibliografia: `bibliography`, `cite`, e os extractores de
//! entradas e de forma/estilo de citação.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use std::sync::Arc;

use crate::entities::file_id::FileId;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// Coage `Value::Array<Value::Dict>` para `Vec<BibEntry>` per
/// diagnóstico P159A §5 + P159D §5.1 + P159E §5.1 + P159G §5.1.
/// Cada Dict valida 4 fields obrigatórios (`key`/`author`/
/// `title`/`year`) + 4 opcionais comuns (`volume`/`pages`/
/// `journal`/`publisher` — Passo 159D) + 2 opcionais identificadores
/// digitais (`url`/`doi` — Passo 159E) + 6 opcionais restantes
/// comuns (`editor`/`series`/`note`/`isbn`/`location`/
/// `organization` — Passo 159G).
///
/// Helper privado P159A extendido em P159D + P159E + P159G; sem
/// promoção (N=1; política consistente N=2-3 mínima — `optional_str`
/// inline helper **N=12 cumulativos** largamente acima limiar).
///
/// **Validações hard**:
/// - Argumento posicional deve ser `Value::Array`.
/// - Cada elemento Array deve ser `Value::Dict`.
/// - Dict deve ter 4 keys obrigatórias.
/// - `key`/`author`/`title` devem ser `Value::Str`.
/// - `year` deve ser `Value::Int` >= 0.
/// - 12 opcionais (volume/pages/journal/publisher/url/doi/
///   editor/series/note/isbn/location/organization) — se
///   presentes, devem ser `Value::Str`; ausência aceite.
fn extract_bib_entries(
    val: Option<&Value>,
) -> SourceResult<Vec<crate::entities::bib_entry::BibEntry>> {
    use crate::entities::bib_entry::BibEntry;
    let arr = match val {
        Some(Value::Array(a)) => a,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "bibliography(entries:) espera array de dict, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => return Ok(Vec::new()), // entries vazio aceitável
    };

    let mut entries = Vec::with_capacity(arr.len());
    for (idx, val) in arr.iter().enumerate() {
        let dict = match val {
            Value::Dict(d) => d,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) espera dict, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
        };

        let key = match dict.get("key") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].key) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'key'",
                        idx
                    ),
                )])
            }
        };

        let author = match dict.get("author") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].author) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'author'",
                        idx
                    ),
                )])
            }
        };

        let title = match dict.get("title") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].title) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'title'",
                        idx
                    ),
                )])
            }
        };

        let year = match dict.get("year") {
            Some(Value::Int(n)) if *n >= 0 => *n as u32,
            Some(Value::Int(n)) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].year) espera int >= 0, recebeu {}",
                        idx, n
                    ),
                )])
            }
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].year) espera int, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'year'",
                        idx
                    ),
                )])
            }
        };

        // Passo 159D — fields opcionais. Helper inline para
        // parsing uniforme de field opcional Str.
        let optional_str = |field: &str| -> SourceResult<Option<String>> {
            match dict.get(field) {
                Some(Value::Str(s)) => Ok(Some(s.to_string())),
                Some(other) => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].{}) espera string, recebeu {}",
                        idx,
                        field,
                        other.type_name()
                    ),
                )]),
                None => Ok(None),
            }
        };
        let volume = optional_str("volume")?;
        let pages = optional_str("pages")?;
        let journal = optional_str("journal")?;
        let publisher = optional_str("publisher")?;
        // Passo 159E — par natural url/doi (reuso optional_str
        // inline helper; cumulativo N=2 P159D + N=2 P159E = N=4).
        let url = optional_str("url")?;
        let doi = optional_str("doi")?;
        // Passo 159G — 6 fields restantes comuns hayagriva
        // (cumulativo N=4 + N=2 + N=6 = N=12 usos do helper).
        let editor = optional_str("editor")?;
        let series = optional_str("series")?;
        let note = optional_str("note")?;
        let isbn = optional_str("isbn")?;
        let location = optional_str("location")?;
        let organization = optional_str("organization")?;

        let mut entry = BibEntry::new(key, author, title, year);
        entry.volume = volume;
        entry.pages = pages;
        entry.journal = journal;
        entry.publisher = publisher;
        entry.url = url;
        entry.doi = doi;
        entry.editor = editor;
        entry.series = series;
        entry.note = note;
        entry.isbn = isbn;
        entry.location = location;
        entry.organization = organization;
        entries.push(entry);
    }
    Ok(entries)
}

/// `bibliography(entries: array, title: ?, style: ?, locale: ?)` → `Content::Bibliography`.
///
/// **Primeiro sub-passo Bibliography + Cite Model Fase 2** (par
/// acoplado com `cite`). Subset minimal per ADR-0054 graded
/// e diagnóstico P159A §1.
///
/// **Naming `bibliography` flat** (sem namespacing — paridade
/// padrão P157B).
///
/// **Atributos**:
/// - `entries`: `Array<Dict>` posicional ou named; cada Dict
///   tem keys obrigatórias `key`/`author`/`title`/`year`.
/// - `title: Content`/`Str` (named); ADR-0064 Caso A
///   (`Smart<Option<Content>>` vanilla → `Option<Box<Content>>`
///   cristalino); None ↔ ausente.
/// - `style: Str` (named); nome CSL built-in (ex: `"ieee"`, `"apa"`).
/// - `locale: Str` (named); locale override (ex: `"en-US"`, `"pt-PT"`).
///
/// **Atributos vanilla scope-out** per ADR-0054 graded:
/// `sources` (parsing externo), `full`, `lang`, `region`.
/// Refinos futuros NÃO reservados per política P158.
///
/// **P418** — input cristalino continua literal; quando `style` é
/// fornecido, o layout pode usar hayagriva/citationberg para CSL.
///
/// **P419** — input via path (`#bibliography("refs.bib")`) carrega
/// `.bib`/`.yaml`/`.json` de disco via `World::read_bytes` + hayagriva.
///
/// **P429 (DEBT-63)** — o style resolvido não fica no `BibliographyElem`;
/// é registado no `EvalContext` e transportado pelo `Module` até ao
/// `BibStore` do `TagIntrospector` no pipeline.
pub fn native_bibliography(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    // Validar named args.
    for key in args.named.keys() {
        if !["entries", "title", "style", "locale"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", key),
            )]);
        }
    }

    // P419: primeiro arg posicional pode ser path (Str) ou entries (Array/Dict).
    let (entries, path) = if let Some(named) = args.named.get("entries") {
        (extract_bib_entries(Some(named))?, None)
    } else if let Some(first) = args.items.first() {
        match first {
            Value::Str(s) => {
                let path = s.as_str();
                let loaded =
                    crate::compiler::eval::bibliography::load_bib_entries_from_path(
                        world,
                        current_file,
                        path,
                    )?;
                (loaded, Some(s.clone()))
            }
            _ => (extract_bib_entries(Some(first))?, None),
        }
    } else {
        (Vec::new(), None)
    };

    // title: named opcional. P479 — default heading "Bibliography" quando não especificado.
    // Sem arg → heading L1 "Bibliography" (paridade vanilla; walk recursivo em e.title conta-o).
    // title: none → None (sem título). title: content/str → conteúdo do arg (sem wrapper).
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) => None,
        Some(other) => Some(Content::text(other.type_name())),
        None => Some(Content::heading(1, Content::text("Bibliography"))),
    };

    // **P1034** — default do Typst vanilla para `bibliography.style` é "ieee".
    // `style: none` desactiva explicitamente (None); ausente usa default.
    let style = match args.named.get("style") {
        Some(Value::Str(s)) => Some(s.clone()),
        Some(Value::None) => None,
        Some(_) => None,
        None => Some("ieee".into()),
    };

    let locale = args.named.get("locale").and_then(|v| match v {
        Value::Str(s) => Some(s.clone()),
        Value::None => None,
        _ => None, // neutro: Value não-Str retorna None no locale de bibliografia
    });

    // P420/P429 — resolve style (built-in ou custom .csl) em eval time e
    // regista-o no EvalContext para transporte via Module → BibStore.
    let elem = crate::entities::elements::bibliography::BibliographyElem {
        entries,
        path,
        title,
        style,
        locale,
    };
    if let Some(s) = &elem.style {
        let resolved = Arc::new(crate::compiler::eval::bibliography::resolve_style(
            world,
            current_file,
            s.as_str(),
        )?);
        ctx.register_bibliography_style(&elem, resolved);
    }

    Ok(Value::Content(Content::Bibliography(Arc::new(elem))))
}

/// `cite(key, supplement: ?, form: ?, style: ?)` → `Content::Cite`.
///
/// Par com `bibliography` (acoplamento semântico vanilla
/// inseparável — cite referencia entries de bibliography).
///
/// **Naming `cite` flat** (paridade P157B).
///
/// **Atributos**:
/// - `key`: `Str` posicional obrigatório (referência a entry).
/// - `supplement: Content`/`Str` (named); ADR-0064 Caso A;
///   None ↔ ausente.
/// - `form: Str` (named); ADR-0064 Caso A (Passo 159C);
///   `"normal"`/`"prose"`/`"author"`/`"year"` ou `auto`/`none`/
///   ausente ↔ None (resolvido a Normal default em layout).
/// - `style: Str` (named); P468;
///   `"numeric"`/`"author-date"`/`"alphabetic"` ou `auto`/`none`/
///   ausente ↔ None (resolvido a Numeric default em layout fallback).
///
/// **Sem validação cross-reference** `key ∈ Bibliography.keys`
/// — diferida per ADR-0017 Introspection runtime adiada.
/// `cite("inexistente")` produz placeholder `[inexistente]`
/// sem erro; forms `Prose`/`Author`/`Year` caem no fallback
/// `[key]` se key não encontrada.
pub fn native_cite(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // key posicional obrigatório.
    let key = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.to_string(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cite() key não pode ser vazia".to_string(),
            )])
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite() espera key como string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cite() exige key como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args.
    for k in args.named.keys() {
        if !["supplement", "form", "style"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", k),
            )]);
        }
    }

    let supplement = args.named.get("supplement").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    let form = extract_citation_form(args.named.get("form"))?;
    let style = extract_citation_style(args.named.get("style"))?;

    Ok(Value::Content(Content::cite_with_style(key, supplement, form, style)))
}

/// Helper privado P159C — parsing `Value::Str` para
/// `Option<CitationForm>`. Strict matching (case-sensitive);
/// `auto`/`none`/ausente → None (resolvido a Normal default em
/// layout). String inválida rejeitada com mensagem listando forms
/// válidas.
fn extract_citation_form(
    val: Option<&Value>,
) -> SourceResult<Option<crate::entities::citation_form::CitationForm>> {
    use crate::entities::citation_form::CitationForm;
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "normal" => Ok(Some(CitationForm::Normal)),
            "prose" => Ok(Some(CitationForm::Prose)),
            "author" => Ok(Some(CitationForm::Author)),
            "year" => Ok(Some(CitationForm::Year)),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cite(): form '{}' inválido (válidos: normal, prose, author, year)",
                    other
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): form espera string, recebeu {}", other.type_name()),
        )]),
    }
}

/// Helper privado P468 — parsing `Value::Str` para
/// `Option<CitationStyle>`. Strict matching (case-sensitive);
/// `auto`/`none`/ausente → None (resolvido a Numeric default em
/// layout fallback). String inválida rejeitada com mensagem listando
/// styles válidos.
fn extract_citation_style(
    val: Option<&Value>,
) -> SourceResult<Option<crate::entities::citation_style::CitationStyle>> {
    use crate::entities::citation_style::CitationStyle;
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "numeric" => Ok(Some(CitationStyle::Numeric)),
            "author-date" => Ok(Some(CitationStyle::AuthorDate)),
            "alphabetic" => Ok(Some(CitationStyle::Alphabetic)),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cite(): style '{}' inválido (válidos: numeric, author-date, alphabetic)",
                    other
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): style espera string, recebeu {}", other.type_name()),
        )]),
    }
}

// ── Passo 295 — `footnote()` cluster Fase 1 (marker only) ─────────────────
//
// **Fase 1 P295 (HE marker only)**: variant minimal `Content::Footnote
// { body }`. Layouter emite apenas marker `[N]` superscript inline;
// body armazenado mas não renderizado no rodapé (sub-passo P295.1).
//
// Simplifications per ADR-0054 graded vs vanilla:
// - `numbering` field scope-out (cosmético; arabic default).
// - `FootnoteBody::Reference(Label)` scope-out (multi-ref futura).
//
// Padrão "variant rico" N=4 cumulativo **inalterado** — A.2 → (a)
// minimal.

