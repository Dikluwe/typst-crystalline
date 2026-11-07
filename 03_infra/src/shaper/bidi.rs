//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/bidi.md
//! @prompt-hash ffd7da98
//! @layer L3
//! @updated 2026-10-31

use unicode_bidi::BidiInfo;

// ── P484 — runs bidirectionais ──────────────────────────────────────────────

pub(super) struct BidiRun {
    pub(super) text: String,
    pub(super) rtl: bool,
    /// Offset byte do run no string original (para ajuste de `cluster`).
    #[allow(dead_code)]
    pub(super) byte_start: usize,
}

/// Divide `text` em runs bidirectionais na ordem visual correcta, agrupados
/// por parágrafo bidi. O unicode-bidi separa parágrafos nos caracteres de
/// classe B (`\n`, `\r`, U+2028, …) e mantém o separador no range do
/// parágrafo anterior (regra P1 do UAX#9); cada parágrafo corresponde a uma
/// linha visível do texto. Para texto puramente LTR de uma linha retorna um
/// único parágrafo com um único run. Parágrafos vazios (`\n\n` consecutivos)
/// retornam um `Vec` vazio — a linha existe, só não tem runs.
///
/// **P845** — antes iterava-se só `paragraphs[0]`, truncando texto com `\n`
/// interno na primeira linha (achado #55 de P831).
/// API unicode-bidi 0.3: `BidiInfo::new`, `visual_runs(para, range)`.
pub(super) fn bidi_runs(text: &str) -> Vec<Vec<BidiRun>> {
    bidi_runs_at_level(text, None)
}

pub(super) fn bidi_runs_with_direction(
    text: &str,
    direction: Option<typst_core::entities::dir::Dir>,
) -> Vec<Vec<BidiRun>> {
    let base = match direction {
        Some(typst_core::entities::dir::Dir::LTR) => unicode_bidi::Level::ltr(),
        Some(typst_core::entities::dir::Dir::RTL) => unicode_bidi::Level::rtl(),
        Some(
            typst_core::entities::dir::Dir::TTB | typst_core::entities::dir::Dir::BTT,
        )
        | None => return bidi_runs(text),
    };
    bidi_runs_at_level(text, Some(base))
}

fn bidi_runs_at_level(
    text: &str,
    base: Option<unicode_bidi::Level>,
) -> Vec<Vec<BidiRun>> {
    if text.is_empty() {
        return vec![];
    }
    let bidi = BidiInfo::new(text, base);
    if bidi.paragraphs.is_empty() {
        return vec![vec![BidiRun { text: text.to_owned(), rtl: false, byte_start: 0 }]];
    }
    bidi.paragraphs
        .iter()
        .map(|para| {
            // Excluir o separador de parágrafo final (classe B) do range a
            // shapear — não gera glifo visível e poluiria o texto extraível
            // (ToUnicode/pdftotext) com o `\n` cru.
            let mut range = para.range.clone();
            while range.end > range.start {
                let last = text[range.start..range.end].chars().next_back().unwrap();
                if unicode_bidi::bidi_class(last) == unicode_bidi::BidiClass::B {
                    range.end -= last.len_utf8();
                } else {
                    break;
                }
            }
            if range.is_empty() {
                return Vec::new();
            }
            let (levels, runs) = bidi.visual_runs(para, range);
            runs.into_iter()
                .map(|run_range| {
                    let rtl = levels
                        .get(run_range.start)
                        .map(|l: &unicode_bidi::Level| l.is_rtl())
                        .unwrap_or(false);
                    BidiRun {
                        text: text[run_range.clone()].to_owned(),
                        rtl,
                        byte_start: run_range.start,
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod explicit_direction_tests {
    use super::*;
    use typst_core::entities::dir::Dir;

    fn visual_runs(text: &str, dir: Option<Dir>) -> Vec<(String, bool, usize)> {
        bidi_runs_with_direction(text, dir)
            .into_iter()
            .flatten()
            .map(|run| (run.text, run.rtl, run.byte_start))
            .collect()
    }

    #[test]
    fn issue_8990_resolved_base_controls_neutral_punctuation() {
        assert_eq!(
            visual_runs("p.v.", Some(Dir::RTL)),
            vec![(".".into(), true, 3), ("p.v".into(), false, 0)]
        );
        assert_eq!(
            visual_runs("אבג.", Some(Dir::LTR)),
            vec![("אבג".into(), true, 0), (".".into(), false, 6)]
        );
    }

    #[test]
    fn issue_8990_unresolved_and_empty_paragraphs_preserve_existing_behavior() {
        assert_eq!(visual_runs("p.v.", None), vec![("p.v.".into(), false, 0)]);
        assert_eq!(visual_runs("אבג.", None), vec![("אבג.".into(), true, 0)]);
        let paragraphs: Vec<Vec<_>> = bidi_runs("אבג.\n\np.v.\r\n")
            .into_iter()
            .map(|runs| {
                runs.into_iter()
                    .map(|run| (run.text, run.rtl, run.byte_start))
                    .collect()
            })
            .collect();
        assert_eq!(
            paragraphs,
            vec![
                vec![("אבג.".into(), true, 0)],
                vec![],
                vec![("p.v.".into(), false, 9)],
                vec![],
            ]
        );
        assert!(bidi_runs_with_direction("", Some(Dir::RTL)).is_empty());
    }
}

// ── fim P484 ─────────────────────────────────────────────────────────────────
