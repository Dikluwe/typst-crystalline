//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/heading.md
//! @prompt-hash 2bfa3bd5
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P383): a lógica de introspeção por-elemento de
//! `Heading` (`compute_heading_auto_toc` + `compute_heading_for_toc`) movida do
//! tronco `introspect.rs` para o arquivo do elemento, na convenção do submódulo
//! `rules/introspect/`. Content-preserving — chamadas pelo walk arm `Heading`.

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::element_kind::ElementKind;
use crate::entities::introspector::Introspector;
use crate::entities::label::Label;
use crate::entities::location::Location;
use crate::entities::selector::Selector;

/// Capture the complete modeled language fields while the causal chain exists.
pub(super) fn snapshot_fields(
    heading: &crate::entities::elements::heading::HeadingElem,
    chain: &crate::entities::style_chain::StyleChain,
    label: Option<&Label>,
) -> indexmap::IndexMap<
    ecow::EcoString,
    crate::entities::value::Value,
    rustc_hash::FxBuildHasher,
> {
    use crate::entities::elements::heading::HEADING_SET_DEPTH;
    use crate::entities::value::Value;

    let numbering =
        if matches!(chain.custom("heading.numbering"), Some(Value::Bool(true))) {
            match chain.custom("heading.numbering.pattern") {
                Some(Value::Str(pattern)) => Value::Str(pattern.clone()),
                _ => Value::None,
            }
        } else {
            Value::None
        };
    let lang = chain.lang().unwrap_or(crate::entities::lang::Lang::ENGLISH);
    let supplement = heading_supplement(lang.as_str());
    let mut fields = indexmap::IndexMap::default();
    fields.insert("level".into(), Value::Int(heading.level as i64));
    fields.insert(
        "depth".into(),
        Value::Int(if heading.set_fields & HEADING_SET_DEPTH != 0 {
            heading.level as i64
        } else {
            1
        }),
    );
    fields.insert("offset".into(), Value::Int(0));
    fields.insert("numbering".into(), numbering);
    fields.insert("supplement".into(), Value::Content(Content::text(supplement)));
    fields.insert("outlined".into(), Value::Bool(heading.outlined));
    fields.insert(
        "bookmarked".into(),
        heading.bookmarked.map(Value::Bool).unwrap_or(Value::Auto),
    );
    fields.insert("hanging-indent".into(), Value::Auto);
    fields.insert("body".into(), Value::Content(heading.body.clone()));
    if let Some(label) = label {
        fields.insert("label".into(), Value::Label(label.clone()));
    }
    fields
}

/// Ratified upstream a51e02804 translations, without runtime lab dependency.
fn heading_supplement(lang: &str) -> &'static str {
    match lang {
        "ar" => "الفصل",
        "bg" | "ru" => "Раздел",
        "ca" => "Secció",
        "cs" | "sk" => "Kapitola",
        "cy" => "Adran",
        "da" => "Afsnit",
        "de" => "Abschnitt",
        "el" => "Κεφάλαιο",
        "eo" => "Sekcio",
        "es" | "gl" => "Sección",
        "et" => "Peatükk",
        "eu" => "Atala",
        "fi" => "Osio",
        "fr" => "Chapitre",
        "ga" => "Ceannteideal",
        "he" => "חלק",
        "hr" => "Odjeljak",
        "hu" => "Fejezet",
        "id" => "Bagian",
        "is" => "Kafli",
        "it" => "Sezione",
        "ja" => "節",
        "la" => "Caput",
        "lt" => "Skyrius",
        "lv" => "Sadaļa",
        "nb" | "nn" => "Kapittel",
        "nl" => "Hoofdstuk",
        "no" | "sv" => "Avsnitt",
        "pl" => "Rozdział",
        "pt" => "Seção",
        "ro" => "Secțiunea",
        "sl" => "Poglavje",
        "sq" => "Kapitull",
        "sr" => "Поглавље",
        "tl" => "Seksyon",
        "tr" => "Bölüm",
        "uk" => "Розділ",
        "vi" => "Phần",
        "zh" => "小节",
        _ => "Section",
    }
}

/// Formata o valor hierárquico de um counter como string terminada em ponto.
///
/// Usado para o número do outline (DEBT-60b / P428): emite `"1."`, `"1.1."`,
/// etc., sem supplement. Retorna `None` se a numeração estiver inactiva ou o
/// counter não tiver valor no momento da `location`.
fn format_heading_number<I: Introspector>(
    intr: &I,
    location: Location,
    numbering_active: bool,
) -> Option<String> {
    if !numbering_active {
        return None;
    }
    let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    intr.formatted_counter_at(&heading_key, location)
        .map(|n| format!("{}.", n))
}

/// Computa `(auto_label, resolved_text)` para a auto-toc de um `Heading`
/// (ADR-0069/P196B). Função pura sobre `(intr, location, auto_label_n,
/// numbering_active)` — sem mutação. Sempre retorna `(Label, String)`
/// (resolved_text vazio quando numbering inactivo — paridade legacy).
pub(super) fn compute_heading_auto_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
    numbering_active: bool,
) -> (Label, String) {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    // Lote F-2 S5 (P335): gate pelo `numbering_active` **assado** no
    // `HeadingElem` (escopo léxico via chain).
    let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    let resolved_text = if numbering_active {
        // P359 (DEBT-60 b): o número do heading, **sem** o supplement "Secção" — o
        // outline mostra o numbering (paridade vanilla). Formato `{n}.` espelha o
        // corpo do heading (o nº do outline == o nº do corpo).
        intr.formatted_counter_at(&heading_key, location)
            .map(|n| format!("{}.", n))
            .unwrap_or_default()
    } else {
        String::new()
    };
    (auto_label, resolved_text)
}

/// Projecta a entry de outline para um `Heading` (ADR-0069/P200B). Função pura
/// sobre `(intr, location, auto_label_n, frozen_body, level, numbering_active)`
/// — sem mutação. `frozen_body` já materializado pelo walk arm Heading. O
/// `number` é computado a partir do counter do Introspector, separando-o do
/// body e do supplement (DEBT-60b / P428). Sempre retorna `Some(...)` (paridade
/// com o push incondicional legacy).
pub(super) fn compute_heading_for_toc<I: Introspector>(
    intr: &I,
    location: Location,
    auto_label_n: usize,
    frozen_body: Content,
    level: usize,
    numbering_active: bool,
) -> Option<(Label, Option<String>, Content, usize)> {
    let auto_label = Label(format!("auto-toc-{}", auto_label_n));
    let number = format_heading_number(intr, location, numbering_active);
    Some((auto_label, number, frozen_body, level))
}
