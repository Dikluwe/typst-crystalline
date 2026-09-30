//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/structural.md
//! @prompt-hash 0c9385c5
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_bibliography, native_cite, native_divider, native_document, native_emph,
    native_enum, native_enum_item, native_footnote, native_footnote_entry,
    native_heading, native_link, native_list, native_list_item, native_lof, native_lot,
    native_outline, native_outline_entry, native_outline_entry_body,
    native_outline_entry_indented, native_outline_entry_inner, native_outline_entry_page,
    native_outline_entry_prefix, native_par, native_par_line, native_quote, native_raw,
    native_ref, native_strong, native_terms, native_terms_item, native_title,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};
use std::sync::Arc;

pub(super) fn register_outline(scope: &mut Scope) {
    scope.define("heading", Value::Func(Func::native("heading", native_heading)));
    scope.define("title", Value::Func(Func::native("title", native_title)));
    {
        let mut entry_namespace = Scope::new();
        entry_namespace.define(
            "body",
            Value::Func(Func::native("outline.entry.body", native_outline_entry_body)),
        );
        entry_namespace.define(
            "prefix",
            Value::Func(Func::native(
                "outline.entry.prefix",
                native_outline_entry_prefix,
            )),
        );
        entry_namespace.define(
            "indented",
            Value::Func(Func::native_with_engine(
                "outline.entry.indented",
                native_outline_entry_indented,
            )),
        );
        entry_namespace.define(
            "inner",
            Value::Func(Func::native_with_engine(
                "outline.entry.inner",
                native_outline_entry_inner,
            )),
        );
        entry_namespace.define(
            "page",
            Value::Func(Func::native_with_engine(
                "outline.entry.page",
                native_outline_entry_page,
            )),
        );
        let entry = Func::native_with_namespace(
            "outline.entry",
            native_outline_entry,
            Arc::new(entry_namespace),
        );
        let mut outline_namespace = Scope::new();
        outline_namespace.define("entry", Value::Func(entry));
        scope.define(
            "outline",
            Value::Func(Func::native_with_namespace(
                "outline",
                native_outline,
                Arc::new(outline_namespace),
            )),
        );
    }
    // P1414 — `par.line` é settable e introspectável, mas a construção manual
    // falha. A numeração nasce no fechamento físico de linha no layout.
    {
        let mut par_namespace = Scope::new();
        par_namespace.define("line", Value::Func(Func::native("line", native_par_line)));
        scope.define(
            "par",
            Value::Func(Func::native_with_namespace(
                "par",
                native_par,
                Arc::new(par_namespace),
            )),
        );
    }
    // **P472** — lof() e lot() como aliases de outline(target: "figures"/"tables").
    scope.define("lof", Value::Func(Func::native("lof", native_lof)));
    scope.define("lot", Value::Func(Func::native("lot", native_lot)));
    scope.define("strong", Value::Func(Func::native("strong", native_strong)));
    scope.define("emph", Value::Func(Func::native("emph", native_emph)));
    {
        let mut raw_namespace = Scope::new();
        raw_namespace.define(
            "line",
            Value::Func(Func::element(
                "raw.line",
                Arc::new(crate::entities::elements::raw::RawLineElem::construct),
            )),
        );
        scope.define(
            "raw",
            Value::Func(Func::native_with_namespace(
                "raw",
                native_raw,
                Arc::new(raw_namespace),
            )),
        );
    }
}

pub(super) fn register_document(scope: &mut Scope) {
    // Passo 154B (ADR-0060 Fase 1): terms + divider.
    // P1473 — `terms` ganha namespace com `item`.
    {
        let mut terms_namespace = Scope::new();
        terms_namespace
            .define("item", Value::Func(Func::native("terms.item", native_terms_item)));
        scope.define(
            "terms",
            Value::Func(Func::native_with_namespace(
                "terms",
                native_terms,
                Arc::new(terms_namespace),
            )),
        );
    }
    scope.define("divider", Value::Func(Func::native("divider", native_divider)));
    // Passo 155 (ADR-0060 Fase 1, sub-passo 2): quote.
    scope.define("quote", Value::Func(Func::native("quote", native_quote)));
    // P397 — document metadata wrapper; P1484 retira o placeholder asset.
    scope.define("document", Value::Func(Func::native("document", native_document)));
    // Passo 295 — footnote Fase 1 (marker only).
    // P1473 — `footnote` ganha namespace com `entry`.
    {
        let mut footnote_namespace = Scope::new();
        footnote_namespace.define(
            "entry",
            Value::Func(Func::native("footnote.entry", native_footnote_entry)),
        );
        scope.define(
            "footnote",
            Value::Func(Func::native_with_namespace(
                "footnote",
                native_footnote,
                Arc::new(footnote_namespace),
            )),
        );
    }
}

pub(super) fn register_references(scope: &mut Scope) {
    // Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado):
    // subset minimal sem hayagriva (input cristalino literal
    // Vec<BibEntry>). Naming flat per padrão P157B; placeholder
    // render per ADR-0033 + ADR-0054 graded; sem validação
    // cross-reference (ADR-0017 adiada). Refinos futuros (CSL,
    // form, hayagriva) NÃO reservados per política P158.
    scope.define(
        "bibliography",
        Value::Func(Func::native("bibliography", native_bibliography)),
    );
    scope.define("cite", Value::Func(Func::native("cite", native_cite)));
    scope.define("link", Value::Func(Func::native("link", native_link)));
    // P1140.2 — label é o valor-tipo chamável; o construtor é despachado
    // estaticamente por call_dispatch.
    scope.define("label", Value::Type(Type::Label));
    scope.define("ref", Value::Func(Func::native("ref", native_ref)));
    // P470 — list/enum com marcadores configuráveis.
    // P1473 — `list`/`enum` ganham namespace com `item`.
    {
        let mut list_namespace = Scope::new();
        list_namespace
            .define("item", Value::Func(Func::native("list.item", native_list_item)));
        scope.define(
            "list",
            Value::Func(Func::native_with_namespace(
                "list",
                native_list,
                Arc::new(list_namespace),
            )),
        );
    }
    {
        let mut enum_namespace = Scope::new();
        enum_namespace
            .define("item", Value::Func(Func::native("enum.item", native_enum_item)));
        scope.define(
            "enum",
            Value::Func(Func::native_with_namespace(
                "enum",
                native_enum,
                Arc::new(enum_namespace),
            )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register_outline(&mut scope);
        register_document(&mut scope);
        register_references(&mut scope);
        let Value::Func(func) = scope.get("outline").unwrap() else {
            panic!("outline function");
        };
        assert_eq!(func.name(), Some("outline"));
    }
}
