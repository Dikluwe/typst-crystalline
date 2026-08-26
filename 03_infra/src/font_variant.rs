//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_variant.md
//! @prompt-hash 22fd4e78
//! @layer L3
//! @updated 2026-07-01
//!
//! P530 — Helpers para Variation Fonts.
//!
//! Converte `TextStyle`/`FontVariant` para coordenadas de eixo OpenType,
//! detecta fontes variáveis, e instancia estaticamente uma fonte VF
//! usando fontTools (Python). O shaper (P525) aplica variações no layout;
//! este módulo produz as fontes estáticas que o export PDF embute.
//!
//! Pipeline validada em P529:
//! oxifont-subset → fonte VF pequena → remover GPOS/GSUB/GDEF →
//! fontTools.varLib.instancer → fonte estática por (FontList, FontVariant).

use typst_core::entities::font_book::{FontStretch, FontStyle, FontVariant, FontWeight};
use typst_core::entities::layout_types::TextStyle;

/// Converte `TextStyle` para `FontVariant` usado na selecção de fonte e
/// nas coordenadas de eixo OpenType.
///
/// P525/P530 — considera `weight` e `italic`; `stretch` não está exposto no
/// `TextStyle` actual (rejeitado em P414), e `Oblique` não carrega ângulo no
/// modelo actual (`FontStyle::Oblique` é uma flag).
pub fn text_style_to_font_variant(style: &TextStyle) -> FontVariant {
    let weight = style.weight.map(FontWeight::from_number).unwrap_or_else(|| {
        if style.bold {
            FontWeight::BOLD
        } else {
            FontWeight::REGULAR
        }
    });
    let style = if style.italic { FontStyle::Italic } else { FontStyle::Normal };
    FontVariant { style, weight, stretch: FontStretch::NORMAL }
}

/// Mapeia `FontVariant` para coordenadas de eixo OpenType passáveis ao
/// `rustybuzz::Face::set_variations` ou ao fontTools instancer.
///
/// P525/P530 — MVP: `wght` (weight) e `ital` (italic). `wdth` (stretch) só será
/// mapeado quando `TextStyle` expuser stretch; `slnt` (Oblique com ângulo)
/// requer `FontStyle::Oblique(angle)`, que o modelo actual não tem.
pub fn axis_variations_for_font_variant(
    variant: &FontVariant,
) -> Vec<rustybuzz::Variation> {
    let mut vars = Vec::new();

    let wght_value = variant.weight.to_number() as f32;
    if wght_value != 400.0 {
        vars.push(rustybuzz::Variation {
            tag: ttf_parser::Tag::from_bytes(b"wght"),
            value: wght_value,
        });
    }

    if variant.style == FontStyle::Italic {
        vars.push(rustybuzz::Variation {
            tag: ttf_parser::Tag::from_bytes(b"ital"),
            value: 1.0,
        });
    }

    vars
}

/// **P836** — eixos derivados (`FontVariant`) fundidos com os eixos
/// explícitos de `style.variations` (`#text(variations:)`). Os explícitos
/// vencem por tag — paridade `automatic.chain(custom).normalized()` do
/// vanilla (`text/font/mod.rs:113-120`).
///
/// Passa a ser a função usada por todos os call sites que têm `TextStyle`
/// (shaper, font_metrics, pipeline, export builder);
/// `axis_variations_for_font_variant` fica para os sítios sem style.
pub fn axis_variations_for_text_style(style: &TextStyle) -> Vec<rustybuzz::Variation> {
    let base = axis_variations_for_font_variant(&text_style_to_font_variant(style));
    match &style.variations {
        Some(custom) => merge_explicit_variations(base, custom),
        None => base,
    }
}

/// **P836** — funde eixos explícitos sobre uma lista base (explícitos
/// vencem por tag). Usado pela pipeline/export, que chaveiam fontes por
/// `(FontList, FontVariant, FontVariations)` sem `TextStyle` à mão.
pub fn merge_explicit_variations(
    mut vars: Vec<rustybuzz::Variation>,
    custom: &typst_core::entities::font_variations::FontVariations,
) -> Vec<rustybuzz::Variation> {
    for (tag, value) in &custom.0 {
        let t = ttf_parser::Tag::from_bytes(tag);
        match vars.iter_mut().find(|v| v.tag == t) {
            Some(existing) => existing.value = *value,
            None => vars.push(rustybuzz::Variation { tag: t, value: *value }),
        }
    }
    vars
}

/// Devolve `true` se os bytes da fonte contiverem uma tabela `fvar`.
pub fn is_variable_font(data: &[u8]) -> bool {
    ttf_parser::Face::parse(data, 0)
        .map(|face| face.tables().fvar.is_some())
        .unwrap_or(false)
}

/// Filtra as coordenadas de eixo pelos eixos realmente presentes na fonte.
/// Evita KeyError no fontTools quando se pede, por exemplo, `ital` numa VF
/// que só tem `wght` (como Ubuntu Sans).
fn filter_variations_by_font_axes(
    data: &[u8],
    variations: &[(ttf_parser::Tag, f32)],
) -> Vec<(ttf_parser::Tag, f32)> {
    let face = match ttf_parser::Face::parse(data, 0) {
        Ok(f) => f,
        Err(_) => return variations.to_vec(),
    };
    let axis_tags: std::collections::HashSet<ttf_parser::Tag> = match face.tables().fvar {
        Some(fvar) => fvar.axes.into_iter().map(|a| a.tag).collect(),
        None => return Vec::new(),
    };
    variations
        .iter()
        .filter(|(tag, _)| axis_tags.contains(tag))
        .copied()
        .collect()
}

const INSTANCER_SCRIPT: &str = r#"
import sys
import io
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

axis_arg = "__CRYSTALLINE_AXIS_ARGS__"
requested = {}
if axis_arg:
    for part in axis_arg.split(","):
        tag, value = part.split("=")
        requested[tag.strip()] = float(value.strip())

font_data = sys.stdin.buffer.read()
font = TTFont(io.BytesIO(font_data))

for tag in ("GPOS", "GSUB", "GDEF"):
    if tag in font:
        del font[tag]

axes = dict(requested)
if "fvar" in font:
    for axis in font["fvar"].axes:
        tag = axis.axisTag
        if tag not in axes:
            axes[tag] = axis.defaultValue

instanced = instantiateVariableFont(font, axes, overlap=False)
out = io.BytesIO()
instanced.save(out)
sys.stdout.buffer.write(out.getvalue())
"#;

/// Devolve o interpretador Python a usar para o fontTools instancer.
///
/// Respeita `TYPST_CRYSTALLINE_PYTHON`; senão, usa `python3` no PATH.
fn python_for_instancer() -> std::ffi::OsString {
    std::env::var_os("TYPST_CRYSTALLINE_PYTHON").unwrap_or_else(|| "python3".into())
}

/// Verifica se o Python configurado tem `fontTools` disponível.
///
/// Usado pela pipeline para falhar cedo quando uma fonte variável precisa de
/// ser instanciada mas a dependência não está instalada (P667).
pub fn variable_font_instancer_available() -> bool {
    let python = python_for_instancer();
    std::process::Command::new(&python)
        .arg("-c")
        .arg("import fontTools")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Instancia estaticamente uma fonte VF para as coordenadas de eixo dadas.
///
/// Requer `fontTools` instalado e disponível via `python3` no PATH (ou no
/// caminho configurado em `TYPST_CRYSTALLINE_PYTHON`).
///
/// O processo invoca um script Python embebido que:
/// - lê a fonte subsetada de stdin (binário),
/// - remove GPOS/GSUB/GDEF (já aplicados pelo shaper),
/// - aplica `fontTools.varLib.instancer.instantiateVariableFont`,
/// - escreve a fonte estática em stdout (binário).
///
/// Devolve `None` se o Python/fontTools falhar ou se a fonte não for VF.
/// O chamador deve decidir se trata isto como erro fatal ou fallback.
pub fn instantiate_variable_font(
    data: &[u8],
    variations: &[(ttf_parser::Tag, f32)],
) -> Option<Vec<u8>> {
    if variations.is_empty() {
        return Some(data.to_vec());
    }
    if !is_variable_font(data) {
        return Some(data.to_vec());
    }

    let filtered = filter_variations_by_font_axes(data, variations);
    if filtered.is_empty() {
        return Some(data.to_vec());
    }

    let python = python_for_instancer();

    let mut axis_args = Vec::new();
    for (tag, value) in &filtered {
        axis_args.push(format!(
            "{}={}",
            std::str::from_utf8(&tag.0.to_be_bytes()).unwrap_or("????"),
            value
        ));
    }

    let script =
        INSTANCER_SCRIPT.replace("__CRYSTALLINE_AXIS_ARGS__", &axis_args.join(","));

    let mut child = std::process::Command::new(&python)
        .arg("-c")
        .arg(&script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .ok()?;

    use std::io::Write;
    {
        let stdin = child.stdin.as_mut()?;
        stdin.write_all(data).ok()?;
    }

    let output = child.wait_with_output().ok()?;

    if !output.status.success() {
        eprintln!(
            "fontTools instancer falhou: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    Some(output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::font_variations::FontVariations;
    use typst_core::entities::layout_types::{Pt, TextStyle};

    fn style_with_variations(
        weight: Option<u16>,
        variations: Option<FontVariations>,
    ) -> TextStyle {
        TextStyle {
            weight,
            size: Pt(11.0),
            variations,
            ..TextStyle::default()
        }
    }

    fn to_tuples(vars: &[rustybuzz::Variation]) -> Vec<([u8; 4], f32)> {
        vars.iter().map(|v| (v.tag.to_bytes(), v.value)).collect()
    }

    #[test]
    fn p836_sem_variacoes_explicitas_igual_a_font_variant() {
        let style = style_with_variations(Some(700), None);
        let merged = axis_variations_for_text_style(&style);
        let derived =
            axis_variations_for_font_variant(&text_style_to_font_variant(&style));
        assert_eq!(to_tuples(&merged), to_tuples(&derived));
        assert_eq!(to_tuples(&merged), vec![(*b"wght", 700.0)]);
    }

    #[test]
    fn p836_explicita_sobrepoe_tag_derivada() {
        // weight 700 (derivado wght 700) + variations (wght: 250) → 250
        // (paridade `automatic.chain(custom)` — custom vence).
        let style = style_with_variations(
            Some(700),
            Some(FontVariations(vec![(*b"wght", 250.0)])),
        );
        assert_eq!(
            to_tuples(&axis_variations_for_text_style(&style)),
            vec![(*b"wght", 250.0)]
        );
    }

    #[test]
    fn p836_explicita_acrescenta_tag_nova() {
        let style = style_with_variations(
            Some(700),
            Some(FontVariations(vec![(*b"opsz", 12.0)])),
        );
        assert_eq!(
            to_tuples(&axis_variations_for_text_style(&style)),
            vec![(*b"wght", 700.0), (*b"opsz", 12.0)]
        );
    }

    #[test]
    fn p836_explicita_sozinha_com_weight_default() {
        // weight None → derivado vazio (400 é omitido); explícita fica sozinha.
        let style = style_with_variations(
            None,
            Some(FontVariations(vec![(*b"wght", 250.0), (*b"ital", 1.0)])),
        );
        assert_eq!(
            to_tuples(&axis_variations_for_text_style(&style)),
            vec![(*b"wght", 250.0), (*b"ital", 1.0)]
        );
    }
}
