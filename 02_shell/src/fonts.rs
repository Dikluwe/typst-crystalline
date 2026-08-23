//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/fonts-command.md
//! @prompt-hash 95edd126
//! @layer L2

use std::path::PathBuf;

/// DTO de apresentação próprio de L2; L4 converte o inventário L3 para esta
/// forma sem expor dependências de infraestrutura.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontDisplayEntry {
    pub family: String,
    pub path: PathBuf,
    pub index: u32,
    pub style: String,
    pub weight: u16,
    pub stretch: u16,
    pub embedded: bool,
}

pub fn format_fonts(entries: &[FontDisplayEntry], variants: bool) -> String {
    let mut out = String::new();
    let mut current: Option<&str> = None;
    for entry in entries {
        if current != Some(entry.family.as_str()) {
            if variants && current.is_some() {
                out.push('\n');
            }
            out.push_str(&entry.family);
            out.push('\n');
            current = Some(&entry.family);
        }
        if variants {
            let origin = if entry.embedded {
                "(Embedded)".to_string()
            } else if entry.index == 0 {
                entry.path.display().to_string()
            } else {
                format!("{}#{}", entry.path.display(), entry.index)
            };
            out.push_str("  └ ");
            out.push_str(&origin);
            out.push('\n');
            out.push_str(&format!(
                "     Style: {}, Weight: {}, Stretch: {}%\n",
                entry.style,
                entry.weight,
                entry.stretch as f64 / 10.0,
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(family: &str, weight: u16) -> FontDisplayEntry {
        FontDisplayEntry {
            family: family.into(),
            path: PathBuf::from("font.ttf"),
            index: 0,
            style: "normal".into(),
            weight,
            stretch: 1000,
            embedded: false,
        }
    }

    #[test]
    fn families_are_printed_once() {
        let text =
            format_fonts(&[entry("A", 400), entry("A", 700), entry("B", 400)], false);
        assert_eq!(text, "A\nB\n");
    }

    #[test]
    fn variants_include_public_fields() {
        let text = format_fonts(&[entry("A", 700)], true);
        assert!(text.contains("font.ttf"));
        assert!(text.contains("Style: normal, Weight: 700, Stretch: 100%"));
    }
}
