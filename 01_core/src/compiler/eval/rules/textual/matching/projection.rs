//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/matching/projection.md
//! @prompt-hash e06b3bad
//! @layer L1
//! @updated 2026-09-30
//!
//! Projeção de faixas originais e achatamento de substituições.

use crate::entities::content::Content;

use super::textual_piece;

/// Reconstitui uma fatia não casada nos nós que a originaram. Espaços
/// explícitos e inferidos fora do match conservam a variante e o estilo;
/// somente um Text parcialmente atravessado vira Text fatiado.
pub(super) fn project_original_range(
    source: &[Content],
    from: usize,
    until: usize,
    out: &mut Vec<Content>,
) {
    let mut cursor = 0;
    for node in source {
        let Some(piece) = textual_piece(node) else {
            continue;
        };
        let end = cursor + piece.len();
        let lower = from.max(cursor);
        let upper = until.min(end);
        if lower < upper {
            if lower == cursor && upper == end {
                out.push(node.clone());
            } else {
                out.push(Content::text(&piece[lower - cursor..upper - cursor]));
            }
        }
        cursor = end;
    }
}

pub(super) fn append_flat_sequence(out: &mut Vec<Content>, content: Content) {
    match content {
        Content::Sequence(children) => {
            for child in children.iter().cloned() {
                append_flat_sequence(out, child);
            }
        }
        other => out.push(other),
    }
}

#[cfg(test)]
mod p1494_matching_tests {
    use super::project_original_range;
    use crate::entities::content::Content;

    #[test]
    fn splice_preserva_espacos_nao_casados_e_fatia_texto() {
        let source = [
            Content::text("ab"),
            Content::SpaceFromNewline,
            Content::text("cd"),
            Content::Space,
            Content::text("ef"),
        ];
        let mut prefix = Vec::new();
        project_original_range(&source, 0, 4, &mut prefix);
        assert!(matches!(&prefix[1], Content::SpaceFromNewline));
        assert_eq!(prefix[2].plain_text(), "c");

        let mut suffix = Vec::new();
        project_original_range(&source, 4, 8, &mut suffix);
        assert_eq!(suffix[0].plain_text(), "d");
        assert!(matches!(&suffix[1], Content::Space));
        assert_eq!(suffix[2].plain_text(), "ef");
    }
}
