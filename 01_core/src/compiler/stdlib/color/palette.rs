//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/color/palette.md
//! @prompt-hash 197c2727
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P1143** — fonte canônica das 18 cores ratificadas, compartilhada pelo
/// scope global e pelo namespace `color`. As quatro escalas de cinza preservam
/// o espaço Luma observável; as demais preservam sRGB.
const PREDEFINED_COLORS: &[(&str, Color)] = &[
    ("black", Color::Luma { l: 0.0, a: 1.0 }),
    ("gray", Color::Luma { l: 170.0 / 255.0, a: 1.0 }),
    ("silver", Color::Luma { l: 221.0 / 255.0, a: 1.0 }),
    ("white", Color::Luma { l: 1.0, a: 1.0 }),
    ("navy", Color::Srgb { r: 0.0, g: 31.0 / 255.0, b: 63.0 / 255.0, a: 1.0 }),
    ("blue", Color::Srgb { r: 0.0, g: 0.454902, b: 0.85098, a: 1.0 }),
    ("aqua", Color::Srgb { r: 127.0 / 255.0, g: 219.0 / 255.0, b: 1.0, a: 1.0 }),
    (
        "teal",
        Color::Srgb {
            r: 57.0 / 255.0,
            g: 204.0 / 255.0,
            b: 204.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "eastern",
        Color::Srgb {
            r: 35.0 / 255.0,
            g: 157.0 / 255.0,
            b: 173.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "purple",
        Color::Srgb {
            r: 177.0 / 255.0,
            g: 13.0 / 255.0,
            b: 201.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "fuchsia",
        Color::Srgb {
            r: 240.0 / 255.0,
            g: 18.0 / 255.0,
            b: 190.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "maroon",
        Color::Srgb {
            r: 133.0 / 255.0,
            g: 20.0 / 255.0,
            b: 75.0 / 255.0,
            a: 1.0,
        },
    ),
    ("red", Color::Srgb { r: 1.0, g: 0.254902, b: 0.211765, a: 1.0 }),
    ("orange", Color::Srgb { r: 1.0, g: 133.0 / 255.0, b: 27.0 / 255.0, a: 1.0 }),
    ("yellow", Color::Srgb { r: 1.0, g: 220.0 / 255.0, b: 0.0, a: 1.0 }),
    (
        "olive",
        Color::Srgb {
            r: 61.0 / 255.0,
            g: 153.0 / 255.0,
            b: 112.0 / 255.0,
            a: 1.0,
        },
    ),
    (
        "green",
        Color::Srgb {
            r: 46.0 / 255.0,
            g: 204.0 / 255.0,
            b: 64.0 / 255.0,
            a: 1.0,
        },
    ),
    ("lime", Color::Srgb { r: 1.0 / 255.0, g: 1.0, b: 112.0 / 255.0, a: 1.0 }),
];

pub(super) fn predefined_color(name: &str) -> Option<Color> {
    PREDEFINED_COLORS
        .iter()
        .find_map(|(candidate, color)| (*candidate == name).then_some(color.clone()))
}

/// Cores nomeadas globais para injeção no scope de eval.
///
/// **P687** — paridade vanilla 0.15.0 (969087ec): as 18 cores oficiais
/// (`lib.rs:359-376` / `visualize/color.rs:291-322`) com bytes sRGB exactos
/// (confirmados via `#repr`). `cyan`/`magenta`/`none` são extras pré-P687 mantidos
/// sem regressão (não fazem parte do conjunto oficial vanilla).
pub fn predefined_color_bindings() -> Vec<(EcoString, Value)> {
    PREDEFINED_COLORS
        .iter()
        .map(|(name, color)| ((*name).into(), Value::Color(color.clone())))
        .chain([
            // Extras cristalinos preservados somente no global.
            ("cyan".into(), Value::Color(Color::rgb(0x00, 0xB3, 0xB3))),
            ("magenta".into(), Value::Color(Color::rgb(0xE5, 0x00, 0xE5))),
            ("none".into(), Value::None),
        ])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_color_behavior() {
        assert!(matches!(predefined_color("red"), Some(Color::Srgb { .. })));
        assert!(predefined_color("not-a-color").is_none());
    }
}
