//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/bidi.md
//! @prompt-hash b1f01cce
//! @layer L3
//! @updated 2026-09-26

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
    if text.is_empty() {
        return vec![];
    }
    let bidi = BidiInfo::new(text, None);
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

// ── fim P484 ─────────────────────────────────────────────────────────────────
