//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/figure_image.md
//! @prompt-hash 2da11c4f
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de composição visual (figure, image).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::image_format::{detect_image_format, ImageFormat};
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use ecow::EcoString;

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
        _ => None,
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
    // Argumento posicional: body (obrigatório)
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(_) => Content::Empty,
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "figure() requer um argumento posicional (body)".to_string(),
            )])
        }
    };

    // Argumento nomeado: caption (opcional)
    // Value::None → ausência de legenda (comportamento intencional).
    let caption = args.named.get("caption").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    // Argumento nomeado: kind (Passo 75 DEBT-15; P158A auto-detect;
    // P158C ADR-0064 Caso A estrito — refactor String → Option<String>).
    // Precedência: `kind:` explícito > inferência > **None** (default
    // "image" resolvido em uso, não em construção).
    let kind: Option<String> = args
        .named
        .get("kind")
        .and_then(|v| match v {
            Value::Str(s) => Some(Some(s.to_string())),
            Value::Auto | Value::None => Some(None),
            _ => None, // tipo inválido — cai em fallback
        })
        .unwrap_or_else(|| infer_kind_from_body(&body)); // P158A

    // F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9): `native_figure` **não baka**
    // mais o padrão de numeração. O `#set figure(numbering:)` vive **só na chain**
    // (`custom("figure.numbering")` no `Content::Styled` da fatia-1); a produção cria
    // a figura **simples** (numbering=None). Consumidor lê o gate da chain. Fonte
    // única. (`_figure_numbering` colapsa da assinatura no mesmo lote — abaixo.)
    Ok(Value::Content(Content::figure(body, caption, kind, None)))
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
    // Validar named args: apenas "width", "height" e "fit" são aceites.
    for key in args.named.keys() {
        if !matches!(key.as_str(), "width" | "height" | "fit") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em image(): '{}'", key),
            )]);
        }
    }

    let path = match args.items.first() {
        Some(Value::Str(s)) => s.to_string(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "image() requer string com o caminho, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "image() requer 1 argumento posicional (caminho do ficheiro)".to_string(),
            )])
        }
    };

    let data = match world.read_bytes(current_file, &path) {
        Ok(arc) => arc,
        Err(msg) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("image(): não foi possível ler '{}': {}", path, msg),
            )])
        }
    };

    // P772p — antes: formato não reconhecido/corrompido era omitido em
    // silêncio na exportação PDF (L3, `eprintln!`), sem erro de compilação
    // (P650 item 2, P772k). Validação de assinatura movida para aqui
    // (avaliação, L1) — paridade textual com o vanilla para o caso
    // "unknown image format" (`typst_library::visualize::image::mod.rs:344`).
    // Não decodifica a imagem inteira (pureza de L1) — só a assinatura;
    // corrupção mais funda que a assinatura ainda escapa para o exportador
    // (limitação registada em `entities/image-format.md`).
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

    Ok(Value::Content(Content::image(path, PtrEqArc(data), width, height, fit)))
}
