//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/source.md
//! @prompt-hash b65fb802
//! @layer L1
//! @updated 2026-03-25

use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;
use std::sync::Arc;

use rustc_hash::FxHasher;

use crate::compiler::parse::parse;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::syntax_node::{LinkedNode, SyntaxNode};

/// Ficheiro de texto carregado em memória com a sua CST associada.
///
/// `Source` é domínio puro: recebe texto já carregado e chama `parse()`
/// internamente. O carregamento do filesystem acontece em L3.
///
/// Clone é barato — partilha o interior via `Arc`.
/// `Hash` é implementado manualmente baseado em `(id, content_hash)` — O(1)
/// graças ao hash pré-computado em `new()` (ADR-0031).
#[derive(Clone, Debug)]
pub struct Source(Arc<SourceInner>);

impl Hash for Source {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
        self.0.content_hash.hash(state);
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id && self.0.content_hash == other.0.content_hash
    }
}

impl Eq for Source {}

#[derive(Debug)]
struct SourceInner {
    id: FileId,
    text: String,
    root: SyntaxNode,
    content_hash: u64, // ADR-0031 — pré-computado em new(), nunca muda
    parser: fn(&str) -> SyntaxNode,
    numbering_interval: std::ops::Range<u64>,
}

impl Source {
    /// Cria `Source` com `FileId` explícito — usado por L3 ao carregar ficheiros.
    pub fn new(id: FileId, text: String) -> SourceResult<Self> {
        Self::new_with_parser(id, text, parse)
    }

    /// Builds an immutable revision of this source.
    pub fn reparse(&self, text: String) -> SourceResult<Self> {
        if text == self.0.text {
            return Ok(self.clone());
        }
        let fresh = (self.0.parser)(&text);
        let mut root = self.0.root.clone();
        if !reconcile_revision(&mut root, &fresh) || !root.spanless_eq(&fresh) {
            root = fresh;
            root.numberize(self.id(), self.0.numbering_interval.clone()).map_err(
                |_| {
                    vec![SourceDiagnostic::error(
                        Span::from_range(self.id(), 0..text.len()),
                        "cannot number source spans",
                    )]
                },
            )?;
        }
        let mut hasher = FxHasher::default();
        text.hash(&mut hasher);
        Ok(Self(Arc::new(SourceInner {
            id: self.id(),
            text,
            root,
            content_hash: hasher.finish(),
            parser: self.0.parser,
            numbering_interval: self.0.numbering_interval.clone(),
        })))
    }

    /// Hash do conteúdo — O(1), pré-computado na construção (ADR-0031).
    pub fn content_hash(&self) -> u64 {
        self.0.content_hash
    }

    /// Cria `Source` com parser customizado.
    ///
    /// Útil para contextos que precisam de parsear como `code` ou `math`
    /// em vez de markup (ex: `eval(source)` — P394).
    pub fn new_with_parser(
        id: FileId,
        text: String,
        parser: fn(&str) -> SyntaxNode,
    ) -> SourceResult<Self> {
        Self::with_numbering_interval(id, text, parser, Span::FULL)
    }

    fn with_numbering_interval(
        id: FileId,
        text: String,
        parser: fn(&str) -> SyntaxNode,
        within: std::ops::Range<u64>,
    ) -> SourceResult<Self> {
        let content_hash = {
            let mut h = FxHasher::default();
            text.hash(&mut h);
            h.finish()
        };
        let mut root = parser(&text);
        root.numberize(id, within.clone()).map_err(|_| {
            vec![SourceDiagnostic::error(
                Span::from_range(id, 0..text.len()),
                "cannot number source spans",
            )]
        })?;
        Ok(Self(Arc::new(SourceInner {
            id,
            text,
            root,
            content_hash,
            parser,
            numbering_interval: within,
        })))
    }

    /// Cria `Source` sem `FileId` real — para testes e contextos sem filesystem.
    ///
    /// Usa um `FileId` sentinel (1). Dois `detached()` partilham o mesmo id —
    /// comportamento aceitável fora de um `World` real.
    pub fn detached(text: impl Into<String>) -> SourceResult<Self> {
        let id = FileId::from_raw(NonZeroU16::MIN);
        Self::new(id, text.into())
    }

    /// **P394** — `Source::detached` com parser customizado.
    pub fn detached_with_parser(
        text: impl Into<String>,
        parser: fn(&str) -> SyntaxNode,
    ) -> SourceResult<Self> {
        let id = FileId::from_raw(NonZeroU16::MIN);
        Self::new_with_parser(id, text.into(), parser)
    }

    /// O `FileId` desta source.
    pub fn id(&self) -> FileId {
        self.0.id
    }

    /// O texto completo da source.
    pub fn text(&self) -> &str {
        &self.0.text
    }

    /// O nó raiz da CST (tipo `SyntaxKind::Markup` para ficheiros normais).
    pub fn root(&self) -> &SyntaxNode {
        &self.0.root
    }

    /// Comprimento do texto em bytes.
    pub fn len_bytes(&self) -> usize {
        self.0.text.len()
    }

    /// Resolve `span` para `(linha, coluna)` 1-indexadas (Passo 111, ADR-0045).
    ///
    /// Devolve `None` se:
    /// - o span é detached (`Span::is_detached()`);
    /// - o span refere-se a outro ficheiro (`span.id() != Some(self.id())`);
    /// - o span é numbered e o nó correspondente não é encontrado;
    /// - o byte offset excede o texto (defensivo).
    ///
    /// Coluna em Unicode code points (chars), não bytes — convenção
    /// editor seguida por gcc/clang/rustc. `\r\n` conta como 1 newline
    /// (avança linha em `\n`; `\r` conta como char na linha anterior).
    pub fn span_to_line_col(&self, span: Span) -> Option<(u32, u32)> {
        if span.is_detached() {
            return None;
        }
        if span.id() != Some(self.0.id) {
            return None;
        }

        // Raw-range spans: o range dá byte offset directo.
        // Numbered spans: resolver via LinkedNode::find → offset do nó.
        let offset = if let Some(range) = span.range() {
            range.start
        } else {
            LinkedNode::new(&self.0.root).find(span)?.offset()
        };

        let text = &self.0.text;
        if offset > text.len() {
            return None;
        }

        // Contar linhas/colunas em chars até offset (col 0-indexed per Vanilla codespan).
        let mut line: u32 = 1;
        let mut col: u32 = 0;
        for (i, ch) in text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        Some((line, col))
    }

    /// **P846 (#57)** — resolve `span` para o seu byte range no texto.
    ///
    /// Equivalente ao `world.range(span)` do vanilla (usado por
    /// `Trace::trace`, `typst-library/src/diag.rs:464`): raw-range spans
    /// devolvem o range directamente; numbered spans resolvem via
    /// `LinkedNode::find` → range do nó. Devolve `None` para spans detached,
    /// de outro ficheiro, ou não encontrados (defensivo).
    pub fn span_byte_range(&self, span: Span) -> Option<std::ops::Range<usize>> {
        if span.is_detached() {
            return None;
        }
        if span.id() != Some(self.0.id) {
            return None;
        }
        if let Some(range) = span.range() {
            return Some(range);
        }
        Some(LinkedNode::new(&self.0.root).find(span)?.range())
    }
}

// Every attempt owns its candidate. A failed numbering operation can mutate its
// receiver, so a failed child must never be installed in an otherwise valid parent.
fn reconcile_revision(old: &mut SyntaxNode, fresh: &SyntaxNode) -> bool {
    if old.spanless_eq(fresh) {
        return true;
    }
    let count = old.children().len();
    if old.kind() != fresh.kind() || count == 0 || fresh.children().len() == 0 {
        return false;
    }
    if count != fresh.children().len() {
        return reconcile_children(old, fresh);
    }
    for (index, next) in fresh.children().enumerate() {
        let previous = &old.children_mut()[index];
        if previous.spanless_eq(next) {
            continue;
        }
        let previous_len = previous.len();
        let previous_descendants = previous.descendants();
        let mut candidate = previous.clone();
        if reconcile_revision(&mut candidate, next) {
            let new_len = candidate.len();
            let new_descendants = candidate.descendants();
            old.children_mut()[index] = candidate;
            old.update_parent(
                previous_len,
                new_len,
                previous_descendants,
                new_descendants,
            );
        } else if old.replace_children(index..index + 1, vec![next.clone()]).is_err() {
            return false;
        }
    }
    true
}

// Bounded, ordered exact anchors preserve islands between multiple edits.
fn next_revision_anchor(
    old: &[SyntaxNode],
    fresh: &[SyntaxNode],
) -> Option<(usize, usize)> {
    const LOOKAHEAD: usize = 32;
    for distance in 1..=2 * LOOKAHEAD {
        for left in 0..=distance.min(LOOKAHEAD) {
            let right = distance - left;
            if right > LOOKAHEAD {
                continue;
            }
            if let (Some(a), Some(b)) = (old.get(left), fresh.get(right)) {
                if a.spanless_eq(b) {
                    return Some((left, right));
                }
            }
        }
    }
    None
}

fn reconcile_children(old: &mut SyntaxNode, fresh: &SyntaxNode) -> bool {
    let next: Vec<_> = fresh.children().cloned().collect();
    let mut index = 0;
    let mut cursor = 0;
    while cursor < next.len() {
        let previous = old.children().as_slice();
        if previous
            .get(index)
            .is_some_and(|child| child.spanless_eq(&next[cursor]))
        {
            index += 1;
            cursor += 1;
            continue;
        }
        let anchor = next_revision_anchor(&previous[index..], &next[cursor..]);
        let (old_end, new_end) = match anchor {
            Some((left, right)) => (index + left, cursor + right),
            None => (previous.len(), next.len()),
        };
        if old
            .replace_children(index..old_end, next[cursor..new_end].to_vec())
            .is_err()
        {
            return false;
        }
        index += new_end - cursor;
        cursor = new_end;
    }
    let end = old.children().len();
    old.replace_children(index..end, Vec::new()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{file_id::FileId, syntax_kind::SyntaxKind};

    fn test_id() -> FileId {
        FileId::from_raw(NonZeroU16::MIN)
    }

    #[test]
    fn source_root_markup() {
        let src = Source::new(test_id(), "Hello *world*".into())
            .expect("valid Source test fixture");
        assert_eq!(src.root().kind(), SyntaxKind::Markup);
        assert!(!src.root().erroneous());
    }

    #[test]
    fn source_vazia() {
        let src = Source::new(test_id(), "".into()).expect("valid Source test fixture");
        assert_eq!(src.len_bytes(), 0);
        assert_eq!(src.text(), "");
    }

    #[test]
    fn source_detached_heading() {
        let src = Source::detached("= Heading").expect("valid Source test fixture");
        let has_heading = src.root().children().any(|n| n.kind() == SyntaxKind::Heading);
        assert!(has_heading);
    }

    #[test]
    fn source_com_erros() {
        let src = Source::detached("#{{{broken").expect("valid Source test fixture");
        assert!(src.root().erroneous());
    }

    #[test]
    fn source_id_roundtrip() {
        let id = test_id();
        let src = Source::new(id, "text".into()).expect("valid Source test fixture");
        assert_eq!(src.id(), id);
    }

    #[test]
    fn source_text_preservado() {
        let src = Source::new(test_id(), "Hello *world*".into())
            .expect("valid Source test fixture");
        assert_eq!(src.text(), "Hello *world*");
    }

    // ── Passo 26 — Early hashing ADR-0031 ───────────────────────────────────

    #[test]
    fn source_hash_o1_apos_construcao() {
        let s = Source::detached("hello world").expect("valid Source test fixture");
        // Múltiplas chamadas retornam o mesmo hash — pré-computado
        assert_eq!(s.content_hash(), s.content_hash());
    }

    #[test]
    fn source_eq_mesmo_conteudo() {
        let id = test_id();
        let s1 = Source::new(id, "hello".into()).expect("valid Source test fixture");
        let s2 = Source::new(id, "hello".into()).expect("valid Source test fixture");
        assert_eq!(s1, s2);
    }

    #[test]
    fn source_neq_conteudo_diferente() {
        let id = test_id();
        let s1 = Source::new(id, "hello".into()).expect("valid Source test fixture");
        let s2 = Source::new(id, "world".into()).expect("valid Source test fixture");
        assert_ne!(s1, s2);
    }

    #[test]
    fn source_pode_ser_chave_de_hashmap() {
        use rustc_hash::FxHashMap;
        let mut map: FxHashMap<Source, &str> = FxHashMap::default();
        let s = Source::detached("test").expect("valid Source test fixture");
        map.insert(s.clone(), "value");
        assert_eq!(map.get(&s), Some(&"value"));
    }

    // ── Passo 111 (ADR-0045) — span_to_line_col ────────────────────────

    fn raw_span(src: &Source, range: std::ops::Range<usize>) -> Span {
        Span::from_range(src.id(), range)
    }

    #[test]
    fn span_to_line_col_inicio_do_texto() {
        let src =
            Source::new(test_id(), "abc".into()).expect("valid Source test fixture");
        let sp = raw_span(&src, 0..0);
        assert_eq!(src.span_to_line_col(sp), Some((1, 0)));
    }

    #[test]
    fn span_to_line_col_depois_de_newline() {
        // "abc\nde" — offset 4 é o 'd' (segunda linha, coluna 0).
        let src =
            Source::new(test_id(), "abc\nde".into()).expect("valid Source test fixture");
        let sp = raw_span(&src, 4..4);
        assert_eq!(src.span_to_line_col(sp), Some((2, 0)));
    }

    #[test]
    fn span_to_line_col_multibyte_unicode_coluna_em_chars() {
        // "áéí" — cada char tem 2 bytes em UTF-8.
        // Offset 2 é início do 'é' (coluna 1 em chars).
        let src =
            Source::new(test_id(), "áéí".into()).expect("valid Source test fixture");
        let sp = raw_span(&src, 2..2);
        assert_eq!(src.span_to_line_col(sp), Some((1, 1)));
    }

    #[test]
    fn span_to_line_col_detached_devolve_none() {
        let src = Source::new(test_id(), "qualquer coisa".into())
            .expect("valid Source test fixture");
        assert_eq!(src.span_to_line_col(Span::detached()), None);
    }

    #[test]
    fn span_to_line_col_ficheiro_diferente_devolve_none() {
        use std::num::NonZeroU16;
        let src =
            Source::new(test_id(), "hello".into()).expect("valid Source test fixture");
        let other_id = FileId::from_raw(NonZeroU16::new(42).unwrap());
        let sp = Span::from_range(other_id, 0..0);
        assert_eq!(src.span_to_line_col(sp), None);
    }

    #[test]
    fn span_to_line_col_fim_do_texto() {
        // "abc" — offset 3 == len → posição após o 'c' (coluna 3).
        let src =
            Source::new(test_id(), "abc".into()).expect("valid Source test fixture");
        let sp = raw_span(&src, 3..3);
        assert_eq!(src.span_to_line_col(sp), Some((1, 3)));
    }

    #[test]
    fn span_to_line_col_span_numbered_via_linked_find() {
        // Spans numbered: obter o span dum nó real da CST e confirmar
        // que span_to_line_col devolve a posição do nó.
        let src = Source::new(test_id(), "hello world".into())
            .expect("valid Source test fixture");
        let root = src.root();
        // Pegar no primeiro filho com span numbered.
        let first_child = root.children().next().expect("root tem filhos");
        let sp = first_child.span();
        assert!(!sp.is_detached(), "filho deve ter span numbered");
        // Posição esperada: início da source → (1, 1) para o primeiro leaf.
        let pos = src.span_to_line_col(sp);
        assert!(pos.is_some(), "span_to_line_col deve resolver spans numbered reais");
    }

    #[test]
    fn span_to_line_col_offset_fora_de_limites_retorna_none() {
        // Raw-range com offset além do texto — defensivo.
        let src =
            Source::new(test_id(), "abc".into()).expect("valid Source test fixture");
        let sp = raw_span(&src, 100..200);
        // start=100 excede text.len()=3 → None.
        assert_eq!(src.span_to_line_col(sp), None);
    }

    #[test]
    fn no_panic_source_construction_reports_numbering_failure_with_origin() {
        let id = FileId::from_raw(std::num::NonZeroU16::MIN);
        let errors = Source::with_numbering_interval(id, "αβ".into(), parse, 0..1)
            .expect_err("an invalid numbering interval must not publish a Source");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "cannot number source spans");
        assert_eq!(errors[0].span.id(), Some(id));
        assert_eq!(errors[0].span.range(), Some(0..4));
        assert!(errors[0].hints.is_empty());
        assert!(errors[0].trace.is_empty());
    }

    fn revision_find(node: &SyntaxNode, kind: SyntaxKind) -> Option<&SyntaxNode> {
        if node.kind() == kind {
            return Some(node);
        }
        node.children().find_map(|child| revision_find(child, kind))
    }

    fn revision_check(source: &Source, parser: fn(&str) -> SyntaxNode) {
        let cold =
            Source::new_with_parser(source.id(), source.text().into(), parser).unwrap();
        assert!(source.root().spanless_eq(cold.root()));
        fn walk(
            source: &Source,
            node: &SyntaxNode,
            offset: usize,
            seen: &mut std::collections::HashSet<Span>,
        ) {
            assert!(seen.insert(node.span()), "duplicate span");
            assert_eq!(
                source.span_byte_range(node.span()),
                Some(offset..offset + node.len())
            );
            let mut cursor = offset;
            for child in node.children() {
                walk(source, child, cursor, seen);
                cursor += child.len();
            }
        }
        walk(source, source.root(), 0, &mut std::collections::HashSet::new());
    }

    #[test]
    fn revision_preserves_intact_loop_across_prefix_and_suffix_insertions() {
        let text = "before\n#for i in range(3) [item #i]\nafter";
        let old = Source::detached(text).unwrap();
        let span = revision_find(old.root(), SyntaxKind::ForLoop).unwrap().span();
        for new in [
            format!("= Heading\n{text}"),
            format!("{text}\n= Heading"),
            format!("= Heading\n{text}\n= Suffix"),
        ] {
            let revised = old.reparse(new).unwrap();
            revision_check(&revised, parse);
            assert_eq!(
                revision_find(revised.root(), SyntaxKind::ForLoop).unwrap().span(),
                span
            );
            assert_eq!(old.text(), text);
            assert_eq!(
                revision_find(old.root(), SyntaxKind::ForLoop).unwrap().span(),
                span
            );
        }
    }

    #[test]
    fn revision_preserves_nested_loop_and_old_clones() {
        let old =
            Source::detached("#[before\n#for i in range(3) [item #i]\nafter]").unwrap();
        let clone = old.clone();
        let span = revision_find(old.root(), SyntaxKind::ForLoop).unwrap().span();
        let revised = old
            .reparse("#[before\n#for i in range(3) [item #i]\nafter extra]".into())
            .unwrap();
        revision_check(&revised, parse);
        assert_eq!(
            revision_find(revised.root(), SyntaxKind::ForLoop).unwrap().span(),
            span
        );
        assert!(std::ptr::eq(old.root(), clone.root()));
        assert_ne!(old.text(), revised.text());
        revision_check(&old, parse);
    }

    #[test]
    fn revision_matches_cold_parse_through_edits_errors_and_unicode() {
        let mut source = Source::detached("").unwrap();
        let cases = [
            "α🙂\n#let x = 1\n#x",
            "α🙂\n#let x = 2\n#x",
            "#let x = [",
            "#[a #for i in range(3) [#i] b]",
            "#[a #for i in range(4) [#i] b]",
            "a\na\na",
            "a\n= a\na",
            "a\na",
            "$x + sqrt(y)$",
            "#let a = (",
            "",
            "recovered",
        ];
        for _ in 0..4 {
            for text in cases {
                source = source.reparse(text.into()).unwrap();
                assert_eq!(source.text(), text);
                revision_check(&source, parse);
            }
        }
    }

    #[test]
    fn revision_retains_custom_parser_and_equal_source() {
        for parser in [
            crate::compiler::parse::parse_code as fn(&str) -> SyntaxNode,
            crate::compiler::parse::parse_math,
        ] {
            let old = Source::detached_with_parser("x + 1", parser).unwrap();
            let revised = old.reparse("y + 22".into()).unwrap();
            revision_check(&revised, parser);
            let equal = revised.reparse(revised.text().into()).unwrap();
            assert!(std::ptr::eq(equal.root(), revised.root()));
        }
    }

    #[test]
    fn revision_numbering_pressure_is_fallible_and_keeps_old_source() {
        let old =
            Source::with_numbering_interval(test_id(), "a".into(), parse, 2..10).unwrap();
        let changed = "#for i in range(10) [#i]";
        let error = old.reparse(changed.into()).unwrap_err();
        assert_eq!(error[0].message, "cannot number source spans");
        assert_eq!(error[0].span.id(), Some(old.id()));
        assert_eq!(error[0].span.range(), Some(0..changed.len()));
        revision_check(&old, parse);
        assert_eq!(old.text(), "a");
    }

    #[test]
    fn revision_falls_back_when_custom_parser_changes_root_kind() {
        fn parser(text: &str) -> SyntaxNode {
            if text.starts_with('#') {
                parse(text)
            } else {
                crate::compiler::parse::parse_code(text)
            }
        }
        let old = Source::detached_with_parser("1 + 2", parser).unwrap();
        let new = old.reparse("#let x = 7\n#x".into()).unwrap();
        assert_ne!(old.root().kind(), new.root().kind());
        revision_check(&new, parser);
        revision_check(&old, parser);
    }

    #[test]
    fn revision_deterministic_edit_sequence_preserves_searchable_spans() {
        let fragments = [
            "α🙂",
            "\n= title\n",
            "#let x = [",
            "]",
            "#for i in range(2) [#i]",
            "\n\n",
            "$x_2$",
            "*emph*",
            "#x",
        ];
        let mut source = Source::detached("seed").unwrap();
        let mut seed = 37usize;
        for step in 0..256 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let mut text = source.text().to_owned();
            let boundaries: Vec<_> = text
                .char_indices()
                .map(|(i, _)| i)
                .chain(std::iter::once(text.len()))
                .collect();
            let a = seed % boundaries.len();
            let b = (seed / 17) % boundaries.len();
            let range = boundaries[a.min(b)]..boundaries[a.max(b)];
            let replacement =
                if step % 5 == 0 { "" } else { fragments[step % fragments.len()] };
            text.replace_range(range, replacement);
            let old = source.clone();
            source = source.reparse(text.clone()).unwrap();
            assert_eq!(source.text(), text);
            assert_eq!(source.root().clone().into_text().as_str(), text);
            revision_check(&source, parse);
            revision_check(&old, parse);
        }
    }
}
