//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/figure_image.md
//! @prompt-hash b563560f
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de composição visual (figure, image).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::repr::repr_value;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::figure_caption::CaptionPosition;
use crate::entities::file_id::FileId;
use crate::entities::image_format::{detect_image_format, ImageFormat};
use crate::entities::layout_types::{Align2D, VAlign};
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use ecow::EcoString;

fn figure_argument_value_span(args: &Args, name: Option<&str>) -> Span {
    let occurrence = args.occurrences.as_ref().and_then(|occurrences| {
        if let Some(name) = name {
            occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
        } else {
            occurrences.iter().find(|arg| arg.name.is_none())
        }
    });
    occurrence.map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
}

fn caption_content_value(value: &Value, span: Span) -> SourceResult<Content> {
    match value {
        Value::Content(body) => Ok(body.clone()),
        Value::LocatedContent(body, _) => Ok(body.content().clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::Symbol(symbol) => Ok(Content::text(symbol.value.clone())),
        Value::None => Ok(Content::Empty),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name(other)),
        )]),
    }
}

/// Auto-detecção de `kind` baseada no body — Passo 158A
/// (Model figure-kinds sub-passo 1).
///
/// Inferência: `Image → "image"`, `Table → "table"`, `Raw → "raw"`.
/// **Recursão limitada a `Content::Sequence`** per diagnóstico
/// P158A §8 (paridade vanilla parcial; outros containers
/// scope-out per ADR-0054 graded).
///
/// Devolve `None` se nenhum descendant detectável encontrado;
/// caller aplica default `"image"` em fallback chain.
fn infer_kind_from_body(body: &Content) -> Option<String> {
    match body {
        Content::Image(_) => Some("image".to_string()),
        Content::Table { .. } => Some("table".to_string()),
        Content::Raw(_) => Some("raw".to_string()),
        // Sequence: recurse no primeiro child detectável (paridade
        // vanilla `query_first_naive` simplificada — limitada a
        // Sequence per decisão P158A §8).
        Content::Sequence(seq) => seq.iter().find_map(infer_kind_from_body),
        _ => None, // neutro: N16[β] — Content sem tipo inferível retorna None
    }
}

/// `figure(body, caption: content)` → `Content::Figure`.
///
/// Migrada do interceptador em `eval.rs` para `stdlib.rs` — o avaliador deixa
/// de conhecer o nome "figure" (DEBT-16 encerrado).
///
/// - `body`: argumento posicional obrigatório.
/// - `caption:`: argumento nomeado opcional; `none` → sem legenda.
/// - `kind:` (Passo 158A): se ausente, **auto-detectado** do body
///   via `infer_kind_from_body` (Image/Table/Raw + Sequence
///   recursivo); se inferência falha, **`None` directo** (default
///   `"image"` resolvido em uso por callers — Passo 158C ADR-0064
///   Caso A estrito).
pub fn native_figure(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let _ = ctx;
    let value_span = |name: Option<&str>| figure_argument_value_span(args, name);
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::LocatedContent(c, _)) => c.content().clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(Value::Symbol(s)) => Content::text(s.value.clone()),
        Some(Value::None) => Content::Empty,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                value_span(None),
                format!("expected content, found {}", vanilla_type_name(other)),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body",
            )])
        }
    };

    // Keep the existing caption body extraction; reject invalid casts explicitly.
    let caption = match args.named.get("caption") {
        None | Some(Value::None) => None,
        Some(Value::Content(Content::FigureCaption(fc))) => Some(fc.body.clone()),
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::LocatedContent(c, _)) => Some(c.content().clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::Symbol(s)) => Some(Content::text(s.value.clone())),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                value_span(Some("caption")),
                format!("expected content or none, found {}", vanilla_type_name(other)),
            )])
        }
    };

    let kind = match args.named.get("kind") {
        Some(Value::Str(s)) => Some(s.to_string()),
        None => infer_kind_from_body(&body),
        Some(Value::Auto) => None,
        Some(Value::Func(function)) => {
            let is_element = function.element_name().is_some()
                || crate::compiler::eval::bindings::native_element_fields(function)
                    .is_some();
            if !is_element {
                return Err(vec![SourceDiagnostic::error(
                    value_span(Some("kind")),
                    "expected element",
                )]);
            }
            // Typed element identity needs the separate public FigureKind contract.
            // Preserve the historical path until that contract is approved.
            infer_kind_from_body(&body)
        }
        Some(Value::Type(ty)) => {
            let message = match ty {
                crate::entities::value::Type::Int
                | crate::entities::value::Type::Float
                | crate::entities::value::Type::Str
                | crate::entities::value::Type::Array
                | crate::entities::value::Type::Dictionary
                | crate::entities::value::Type::Datetime
                | crate::entities::value::Type::Stroke
                | crate::entities::value::Type::Regex
                | crate::entities::value::Type::Tiling
                | crate::entities::value::Type::Bytes
                | crate::entities::value::Type::Decimal
                | crate::entities::value::Type::Duration
                | crate::entities::value::Type::Version
                | crate::entities::value::Type::Selector
                | crate::entities::value::Type::Symbol
                | crate::entities::value::Type::Arguments
                | crate::entities::value::Type::State
                | crate::entities::value::Type::Counter
                | crate::entities::value::Type::Label
                | crate::entities::value::Type::Path
                | crate::entities::value::Type::Type
                | crate::entities::value::Type::SpotColorant => {
                    "expected element".to_string()
                }
                crate::entities::value::Type::Bool => {
                    "type boolean does not have a constructor".to_string()
                }
                crate::entities::value::Type::Relative => {
                    "type relative length does not have a constructor".to_string()
                }
                crate::entities::value::Type::None
                | crate::entities::value::Type::Auto
                | crate::entities::value::Type::Module
                | crate::entities::value::Type::Function
                | crate::entities::value::Type::Content
                | crate::entities::value::Type::Length
                | crate::entities::value::Type::Ratio
                | crate::entities::value::Type::Angle
                | crate::entities::value::Type::Color
                | crate::entities::value::Type::Fraction
                | crate::entities::value::Type::Alignment
                | crate::entities::value::Type::Gradient
                | crate::entities::value::Type::Direction
                | crate::entities::value::Type::Location => {
                    format!("type {} does not have a constructor", ty.name())
                }
            };
            return Err(vec![SourceDiagnostic::error(value_span(Some("kind")), message)]);
        }
        Some(Value::Symbol(symbol)) => {
            let value = symbol.value.as_str();
            let callable =
                crate::compiler::stdlib::structural::is_known_math_accent(value)
                    || crate::compiler::stdlib::structural::is_callable_lr_symbol(value);
            let message = if callable {
                "expected element".to_string()
            } else {
                format!("symbol {value} is not callable")
            };
            return Err(vec![SourceDiagnostic::error(value_span(Some("kind")), message)]);
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                value_span(Some("kind")),
                format!(
                    "expected function, string, or auto, found {}",
                    vanilla_type_name(other)
                ),
            )])
        }
    };

    // F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9): `native_figure` **não baka**
    // mais o padrão de numeração. O `#set figure(numbering:)` vive **só na chain**
    // (`custom("figure.numbering")` no `Content::Styled` da fatia-1); a produção cria
    // a figura **simples** (numbering=None). Consumidor lê o gate da chain. Fonte
    // única. (`_figure_numbering` colapsa da assinatura no mesmo lote — abaixo.)
    Ok(Value::Content(Content::figure(body, caption, kind, None)))
}

// ── P1473 — `figure.caption(body, position?, separator?)` ───────────────────

/// `figure.caption(body, position?, separator?)` — constructor qualificado,
/// escopo reduzido (construção sem síntese). `body` é `Content` posicional
/// obrigatório; `position` é um valor de alinhamento (`Value::Align`,
/// restrito à componente vertical `top`/`bottom` — **não** `Str`; medido
/// na integração, `figure.caption(x, position: "top")` é rejeitado no
/// vanilla com `expected alignment, found string`); `separator` é
/// `Content` nomeado opcional. Nenhum dos dois nomeados tem efeito de
/// layout nesta fatia — ficam preservados na entidade, sem síntese
/// (`kind`/`supplement`/`numbering`/`counter`) nem `.realize()`. Ver
/// `compiler/stdlib/figure_image.md` §P1473.
pub fn native_figure_caption(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !matches!(key.as_str(), "position" | "separator") {
            let diagnostic = if key.as_str() == "body" {
                SourceDiagnostic::error(
                    Span::detached(),
                    "the argument `body` is positional",
                )
                .with_hint("try removing `body:`")
            } else {
                SourceDiagnostic::error(
                    Span::detached(),
                    format!("unexpected argument: {key}"),
                )
            };
            return Err(vec![diagnostic]);
        }
    }
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    let body = match args.items.first() {
        Some(value) => {
            caption_content_value(value, figure_argument_value_span(args, None))?
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body",
            )]);
        }
    };
    let position = match args.named.get("position") {
        None => None,
        Some(Value::Align(Align2D { h: None, v: Some(VAlign::Top) })) => {
            Some(CaptionPosition::Top)
        }
        Some(Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) })) => {
            Some(CaptionPosition::Bottom)
        }
        Some(other @ Value::Align(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected `top` or `bottom`, found {}", repr_value(other)),
            )]);
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected alignment, found {}", vanilla_type_name(other)),
            )]);
        }
    };
    let separator = match args.named.get("separator") {
        None => None,
        Some(value) => Some(caption_content_value(
            value,
            figure_argument_value_span(args, Some("separator")),
        )?),
    };
    Ok(Value::Content(Content::figure_caption(body, position, separator)))
}

// ── `image()` — carregamento de imagens do disco (Passo 71, DEBT-24) ─────────

/// `image(path, width?, height?)` → `Content::Image`.
///
/// Lê os bytes do ficheiro através de `world.read_bytes(path)` (Passo 109:
/// `world` passou do `EvalContext` para o ABI directo, ADR-0044).
/// `width` e `height` são preservados no AST para o Passo 72 (dimensões reais).
/// O layouter usa placeholder 100×100 pt neste passo (DEBT-24b).
pub fn native_image(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    // P1481: aceita também alt como descrição semântica.
    for key in args.named.keys() {
        if !matches!(key.as_str(), "width" | "height" | "fit" | "page" | "alt") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em image(): '{}'", key),
            )]);
        }
    }

    // P835 (#20) — `page:` aceite como no-op validado (decisão do dono,
    // 2026-07-22). No vanilla o parâmetro só tem efeito para fontes PDF
    // (`visualize/image/mod.rs:158-161`) — e PDF-como-fonte continua
    // scope-out (P781/DEBT-68) — mas o cast é validado em qualquer fonte:
    // `page: 0` → `number must be positive`; outro tipo →
    // `expected integer, found {tipo}` (mensagens verbatim medidas em P835).
    if let Some(v) = args.named.get("page") {
        match v {
            Value::Int(i) if *i > 0 => {} // aceite, sem efeito até DEBT-68 reabrir
            Value::Int(_) => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "number must be positive".to_string(),
                )])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
                )])
            }
        }
    }

    let path_value = match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_))) => value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("image() requer path ou string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "image() requer 1 argumento posicional (caminho do ficheiro)".to_string(),
            )])
        }
    };

    let (rooted, data) =
        match crate::compiler::stdlib::read_path_value(path_value, world, current_file) {
            Ok(pair) => pair,
            Err(msg) => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("image(): não foi possível ler: {}", msg),
                )])
            }
        };

    // P772p — antes: formato não reconhecido/corrompido era omitido em
    // silêncio na exportação PDF (L3, `eprintln!`), sem erro de compilação
    // (P650 item 2, P772k). Validação de assinatura movida para aqui
    // (avaliação, L1) — paridade textual com o vanilla para o caso
    // "unknown image format" (`typst_library::visualize::image::mod.rs:344`).
    // Não decodifica a imagem inteira (pureza de L1) — só a assinatura;
    // corrupção mais funda que a assinatura é apanhada em L3 por
    // `validate_document_images` no pipeline (P833/#18 — erro de compilação
    // no formato do vanilla), e GIF/WebP são aceites desde P833/#17.
    let path = rooted.vpath().get_with_slash().to_string();
    let lower_path = path.to_lowercase();
    if lower_path.ends_with(".svg") || lower_path.ends_with(".svgz") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "SVG images are not supported yet".to_string(),
        )]);
    }
    // P781 — vanilla SUPORTA `#image("ficheiro.pdf")` (embute uma página do
    // PDF fonte via `hayro`/`hayro-syntax` + `krilla::draw_pdf_page` como
    // Form XObject). Cristalino não tem motor de renderização/parsing PDF
    // nem write vetorial de Form XObject no exportador (hand-rolled, sem
    // krilla) — decisão registada (`paridade-producao-p781.md`): scope-out
    // consciente, dependência pesada (`hayro` pull ~15 crates transitivas,
    // incl. `vello_common`/`vello_cpu`), não implementado. Mensagem
    // dedicada, mesmo padrão de SVG — antes seria "unknown image format"
    // (menos específico, mas já era um erro claro, não omissão silenciosa).
    if lower_path.ends_with(".pdf") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "PDF images are not supported yet".to_string(),
        )]);
    }
    if detect_image_format(&data) == ImageFormat::Unknown {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "unknown image format".to_string(),
        )]);
    }

    let width = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);

    let fit = args
        .named
        .get("fit")
        .and_then(|v| match v {
            Value::Str(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| EcoString::from("cover"));

    if !matches!(fit.as_str(), "contain" | "cover" | "stretch") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("image(fit:): valor '{}' inválido; esperado 'contain', 'cover' ou 'stretch'", fit),
        )]);
    }

    let alt = match args.named.get("alt") {
        Some(Value::Str(value)) => Some(value.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected string, found {}", vanilla_type_name(other)),
            )])
        }
        None => None,
    };
    let mut content = Content::image(path, PtrEqArc(data), width, height, fit);
    if let Content::Image(image) = &mut content {
        std::sync::Arc::make_mut(image).alt = alt;
    }
    Ok(Value::Content(content))
}

/// P1473 (Agente B) — contrato RED de `figure.caption` (escopo reduzido:
/// construção sem síntese). Fonte:
/// `00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/compiler/stdlib/figure_image.md` §P1473 +
/// `00_nucleo/prompts/entities/elements/figure_caption.md`. Escrito sem ver
/// a implementação (isolamento Agente A/B).
///
/// `Content::FigureCaption`/`FigureCaptionElem` ainda não existem neste
/// baseline — para não depender de um tipo Rust inexistente (o que
/// quebraria a compilação de todo o crate, impedindo confirmar o RED de
/// qualquer outro teste), os observáveis internos (`body`/`position`/
/// `separator`) são verificados por três vias genéricas já existentes e
/// exaustivamente casadas sobre `Content` (logo obrigatoriamente
/// actualizadas quando a variante nova for acrescentada):
/// `Content::elem_name()`, `Content::plain_text()` (delega a `body` por
/// especificação do L0 da entidade) e `format!("{:?}", content)` (Debug
/// derivado da entidade — `position: None`/`Some(..)` como substring).
/// `Content::Figure`/`FigureElem` já existem hoje — usados directamente
/// (sem risco de compilação) para confirmar que `native_figure` extrai o
/// `.body` do `Content::FigureCaption` recebido em `caption:`.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::entities::source_result::SourceResult;

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1473).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    fn expect_ok(src: &str) -> Value {
        match eval(src) {
            Ok(v) => v,
            Err(errors) => panic!("{src}: esperava Ok, obteve Err({errors:?})"),
        }
    }

    fn expect_err_message(src: &str) -> String {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                errors[0].message.clone()
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    fn content_plain_text(src: &str) -> String {
        match expect_ok(src) {
            Value::Content(c) => c.plain_text(),
            other => panic!("{src}: esperava Value::Content, obteve {other:?}"),
        }
    }

    #[test]
    fn p1473_figure_caption_type_is_function() {
        assert_eq!(expect_ok("type(figure.caption) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_figure_caption_constructs_body_only() {
        let value = expect_ok("figure.caption([hello])");
        let Value::Content(content) = &value else {
            panic!("esperava Value::Content, obteve {value:?}");
        };
        assert_eq!(content.elem_name(), "figure.caption");
        assert_eq!(content.plain_text(), content_plain_text("[hello]"));
        let debug = format!("{content:?}");
        assert!(
            debug.contains("position: None"),
            "position deve ficar None quando não fornecido: {debug}"
        );
        assert!(
            debug.contains("separator: None"),
            "separator deve ficar None quando não fornecido: {debug}"
        );
    }

    #[test]
    fn p1473_figure_caption_constructs_with_position_and_separator() {
        let value = expect_ok("figure.caption([hello], position: top, separator: [--])");
        let Value::Content(content) = &value else {
            panic!("esperava Value::Content, obteve {value:?}");
        };
        let debug = format!("{content:?}");
        assert!(
            debug.contains("position: Some("),
            "position deve ficar Some quando fornecido: {debug}"
        );
        assert!(
            debug.contains("separator: Some("),
            "separator deve ficar Some quando fornecido: {debug}"
        );
    }

    #[test]
    fn p1473_native_figure_accepts_figure_caption_and_extracts_body() {
        let expected_body = match eval("[x]") {
            Ok(Value::Content(c)) => c,
            other => panic!("[x]: esperava Ok(Value::Content), obteve {other:?}"),
        };
        let value = expect_ok("figure(rect(), caption: figure.caption([x]))");
        match value {
            // `Content::Figure`/`FigureElem` já existem hoje — nenhum risco
            // de compilação em referenciá-los directamente.
            Value::Content(Content::Figure(fe)) => {
                assert_eq!(
                    fe.caption,
                    Some(expected_body),
                    "native_figure deve extrair o `.body` do FigureCaption, \
                     não preservar o wrapper inteiro"
                );
            }
            other => panic!("esperava Content::Figure, obteve {other:?}"),
        }
    }

    #[test]
    fn p1473_figure_caption_missing_body() {
        assert_eq!(expect_err_message("figure.caption()"), "missing argument: body");
    }

    #[test]
    fn p1473_figure_caption_wrong_type() {
        assert_eq!(
            expect_err_message("figure.caption(1)"),
            "expected content, found integer"
        );
    }

    #[test]
    fn p1473_figure_caption_invalid_position() {
        assert_eq!(
            expect_err_message("figure.caption([x], position: left)"),
            "expected `top` or `bottom`, found left"
        );
    }

    #[test]
    fn p1473_figure_caption_position_wrong_type() {
        assert_eq!(
            expect_err_message(r#"figure.caption([x], position: "top")"#),
            "expected alignment, found string"
        );
    }

    #[test]
    fn p1473_figure_parent_keeps_function_kind_and_direct_call() {
        assert_eq!(expect_ok("type(figure) == function"), Value::Bool(true));
        // Chamada directa (sem caption qualificada) não deve regredir.
        eval(r#"figure(rect(), caption: [plain caption])"#)
            .unwrap_or_else(|e| panic!("chamada directa de figure() regrediu: {e:?}"));
    }
    #[test]
    fn figure_domain_rejects_invalid_body_instead_of_empty_content() {
        for (expr, kind) in
            [("1", "integer"), ("true", "boolean"), ("(1,2)", "array"), ("auto", "auto")]
        {
            assert_eq!(
                expect_err_message(&format!("figure({expr})")),
                format!("expected content, found {kind}")
            );
        }
        assert_eq!(expect_err_message("figure()"), "missing argument: body");
    }

    #[test]
    fn figure_domain_rejects_invalid_caption_instead_of_type_text() {
        for (expr, kind) in [("1", "integer"), ("true", "boolean"), ("auto", "auto")] {
            assert_eq!(
                expect_err_message(&format!("figure([A],caption:{expr})")),
                format!("expected content or none, found {kind}")
            );
        }
    }

    #[test]
    fn figure_domain_rejects_invalid_kind_and_non_element_functions() {
        for (expr, kind) in [
            ("none", "none"),
            ("1", "integer"),
            ("true", "boolean"),
            ("(a:1)", "dictionary"),
            ("[A]", "content"),
        ] {
            assert_eq!(
                expect_err_message(&format!("figure([A],kind:{expr})")),
                format!("expected function, string, or auto, found {kind}")
            );
        }
        for expr in ["assert", "x=>x", "int", "table.with(columns:1)"] {
            assert_eq!(
                expect_err_message(&format!("figure([A],kind:{expr})")),
                "expected element"
            );
        }
        for (expr, name) in [
            ("bool", "boolean"),
            ("type(ltr)", "direction"),
            ("type(50%+1pt)", "relative length"),
        ] {
            assert_eq!(
                expect_err_message(&format!("figure([A],kind:{expr})")),
                format!("type {name} does not have a constructor")
            );
        }
        let world = NullWorld::default();
        let mut args = Args::positional(vec![Value::Content(Content::text("A"))]);
        args.named.insert(
            "kind".into(),
            Value::Func(crate::entities::func::Func::native(
                "table",
                crate::compiler::stdlib::native_assert,
            )),
        );
        let errors = native_figure(
            &mut EvalContext::new(),
            &args,
            &world,
            crate::contracts::world::World::main(&world),
        )
        .unwrap_err();
        assert_eq!(
            errors[0].message, "expected element",
            "names must not impersonate element identity"
        );
    }

    #[test]
    fn figure_domain_errors_follow_field_order_and_causal_value_spans() {
        use crate::entities::args::ArgOccurrence;
        let world = NullWorld::default();
        let file = crate::contracts::world::World::main(&world);
        let span = |n| Span::from_range(file, n..n + 1);
        let make = |body, caption| {
            Args::from_occurrences(
                span(1),
                vec![
                    ArgOccurrence {
                        name: None,
                        value: body,
                        span: span(2),
                        value_span: span(3),
                    },
                    ArgOccurrence {
                        name: Some("kind".into()),
                        value: Value::Int(1),
                        span: span(4),
                        value_span: span(5),
                    },
                    ArgOccurrence {
                        name: Some("caption".into()),
                        value: caption,
                        span: span(6),
                        value_span: span(7),
                    },
                ],
            )
        };
        for (body, caption, message, anchor) in [
            (Value::Int(1), Value::Int(1), "expected content, found integer", span(3)),
            (
                Value::Content(Content::text("A")),
                Value::Int(1),
                "expected content or none, found integer",
                span(7),
            ),
            (
                Value::Content(Content::text("A")),
                Value::None,
                "expected function, string, or auto, found integer",
                span(5),
            ),
        ] {
            let errors = native_figure(
                &mut EvalContext::new(),
                &make(body, caption),
                &world,
                file,
            )
            .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, message);
            assert_eq!(errors[0].span, anchor);
        }
    }

    #[test]
    fn figure_domain_valid_controls_keep_body_caption_and_explicit_name() {
        for expr in [
            "figure(none)",
            "figure(\"A\")",
            "figure([A],caption:none)",
            "figure([A],caption:\"C\")",
            "figure([A],caption:[C])",
            "figure([A],kind:auto)",
            "{let t=table;figure([A],kind:t)}",
        ] {
            expect_ok(expr);
        }
        let Value::Content(Content::Figure(f)) = expect_ok("figure([A],kind:\"custom\")")
        else {
            panic!("figure expected")
        };
        assert_eq!(f.kind.as_deref(), Some("custom"));
        assert_eq!(f.body.plain_text(), "A");
    }

    #[test]
    fn figure_domain_located_values_are_content_for_body_and_caption() {
        use crate::entities::value::IntrospectedContent;
        let world = NullWorld::default();
        let file = crate::contracts::world::World::main(&world);
        let origin = Span::from_range(file, 21..22);
        let mut locator = crate::entities::locator::Locator::new();
        let body = Value::LocatedContent(
            IntrospectedContent::new(Content::text("A").with_source_span(origin), None),
            locator.next(),
        );
        let caption = Value::LocatedContent(
            IntrospectedContent::new(Content::text("C"), None),
            locator.next(),
        );
        let mut args = Args::positional(vec![body]);
        args.named.insert("caption".into(), caption);
        let Value::Content(Content::Figure(f)) =
            native_figure(&mut EvalContext::new(), &args, &world, file).unwrap()
        else {
            panic!("expected figure")
        };
        assert_eq!(f.body.plain_text(), "A");
        assert_eq!(f.body.source_span(), origin);
        assert_eq!(f.caption.as_ref().unwrap().plain_text(), "C");
    }
    #[test]
    fn figure_symbol_kind_uses_function_cast_diagnostics() {
        for value in ["α", ")"] {
            assert_eq!(
                expect_err_message(&format!("figure([A],kind:symbol(\"{value}\"))")),
                format!("symbol {value} is not callable")
            );
        }
        for value in ["^", "(", "[", "⌈", "|", "↔\u{fe0e}", "\u{0302}"] {
            assert_eq!(
                expect_err_message(&format!("figure([A],kind:symbol(\"{value}\"))")),
                "expected element"
            );
        }
    }

    #[test]
    fn figure_symbol_body_and_caption_keep_effective_grapheme() {
        let Value::Content(Content::Figure(f)) =
            expect_ok("figure(sym.alpha,caption:sym.beta)")
        else {
            panic!("expected figure")
        };
        assert_eq!(f.body.plain_text(), "α");
        assert_eq!(f.caption.as_ref().unwrap().plain_text(), "β");
    }
    #[test]
    fn caption_shape_constructor_uses_content_casts_and_causal_error() {
        for (expr, text) in [("none", ""), ("\"C\"", "C"), ("sym.alpha", "α")] {
            let Value::Content(Content::FigureCaption(c)) =
                expect_ok(&format!("figure.caption({expr})"))
            else {
                panic!("expected caption")
            };
            assert_eq!(c.body.plain_text(), text);
        }
        let Value::Content(Content::FigureCaption(c)) =
            expect_ok("figure.caption([C],separator:none)")
        else {
            panic!("expected caption")
        };
        assert!(c.separator.as_ref().unwrap().is_empty());
        let Value::Content(Content::FigureCaption(c)) =
            expect_ok("figure.caption([C],separator:\"S\")")
        else {
            panic!("expected caption")
        };
        assert_eq!(c.separator.as_ref().unwrap().plain_text(), "S");
        use crate::entities::args::ArgOccurrence;
        let world = NullWorld::default();
        let file = crate::contracts::world::World::main(&world);
        let anchor = Span::from_range(file, 22..23);
        let args = Args::from_occurrences(
            Span::detached(),
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::text("C")),
                    span: Span::detached(),
                    value_span: Span::detached(),
                },
                ArgOccurrence {
                    name: Some("separator".into()),
                    value: Value::Int(1),
                    span: Span::detached(),
                    value_span: anchor,
                },
            ],
        );
        let errors = native_figure_caption(&mut EvalContext::new(), &args, &world, file)
            .unwrap_err();
        assert_eq!(errors[0].message, "expected content, found integer");
        assert_eq!(errors[0].span, anchor);
    }
}
