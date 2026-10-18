//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/show.md
//! @prompt-hash f679f953
//! @layer L1
//! @updated 2026-10-17
//!
//! Comportamento de `eval/rules::show`.

use super::*;

/// **Show-set (P352)** — captura o efeito de um `#set` como `Styles`, **sem**
/// mutar o estilo global. Reusa `eval_set_rule` (fiel a todos os targets, aos
/// warns e aos erros hard) aplicando-o a uma `StyleChain::empty()` temporária e
/// extrai o delta resultante (`collapse`). Sobre `empty()`, o resultado é só o
/// que o `set` definiu — sem os defaults. `engine.styles` é restaurado mesmo
/// em caso de erro (o swap de volta corre antes do `?`).
pub(crate) fn capture_set_styles(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Styles> {
    let mut scratch = StyleChain::empty();
    std::mem::swap(&mut *engine.styles, &mut scratch);
    let result = eval_set_rule(set, scopes, ctx, engine);
    std::mem::swap(&mut *engine.styles, &mut scratch);
    result?;
    Ok(Styles::from_delta(scratch.collapse()))
}

pub(crate) fn eval_show_rule(
    show_rule: ShowRuleNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // P790 — `page`/`par` como alvo de show: identificadores especiais
    // reconhecidos ANTES de avaliar o selector (no vanilla são element
    // functions em scope; na stdlib cristalina não existem como variáveis —
    // caíam em `unknown variable` fatal, achado P786). Paridade medida por
    // execução (`typst-eval/src/rules.rs:67-95`):
    // - `#show page: <qualquer transformação>` → warning específico, nenhuma
    //   regra registada, compilação prossegue (a regra não tem efeito no
    //   vanilla).
    // - `#show par: set block(...)` com `spacing`/`above`/`below` → warning
    //   específico, nenhuma regra registada (vanilla condiciona a
    //   `BlockElem::above`/`below` na chain).
    // - `#show par: <outra transformação>` → erro explícito (scope-out
    //   documentado no L0, eval.md §P790: no vanilla `show par` é regra viva
    //   sobre `ParElem`; element rule de par é candidata a passo futuro).
    // P863: selector especial `par` pode ser resolvido antes de avaliar a
    // expressão (não existe como variável na stdlib cristalina).
    let mut preselected: Option<Selector> = None;
    if let Some(Expr::Ident(ident)) = show_rule.selector() {
        let rule_span = show_rule.to_untyped().span();
        match ident.as_str() {
            "page" => {
                engine.sink.warn_note(
                    rule_span,
                    "`show page` is not supported and has no effect",
                    "customize pages with `set page(..)` instead",
                );
                return Ok(Value::None);
            }
            "par" => {
                let is_set_block_spacing = matches!(
                    show_rule.transform(),
                    Expr::SetRule(set)
                        if set.target().to_untyped().text_str() == "block"
                            && set.args().items().any(|arg| matches!(
                                arg,
                                Arg::Named(named)
                                    if matches!(
                                        named.name().as_str(),
                                        "spacing" | "above" | "below"
                                    )
                            ))
                );
                if is_set_block_spacing {
                    engine.sink.warn_note2(
                        rule_span,
                        "`show par: set block(spacing: ..)` has no effect anymore",
                        "write `set par(spacing: ..)` instead",
                        "this is specific to paragraphs as they are not considered \
                         blocks anymore",
                    );
                    return Ok(Value::None);
                }
                preselected = Some(Selector::NodeKind(NodeKind::Par));
            }
            _ => {}
        }
    }

    // Avaliar o selector — pode ser uma string ou uma função da stdlib.
    // `selector()` retorna `Option<Expr>` — None significa selector omitido (não suportado).
    let selector = if let Some(sel) = preselected {
        sel
    } else {
        match show_rule.selector() {
            None => {
                return Err(vec![SourceDiagnostic::error(
                    show_rule.to_untyped().span(),
                    "show rule requer um selector".to_string(),
                )]);
            }
            Some(sel_expr) => {
                let selector_val = eval_expr(sel_expr, scopes, ctx, engine)?;
                match selector_val {
                    Value::Str(s) => {
                        // P790 — paridade vanilla `selector.rs:110` (medido por
                        // execução): selector de texto vazio é erro, não no-op.
                        if s.is_empty() {
                            return Err(vec![SourceDiagnostic::error(
                                sel_expr.span(),
                                "text selector is empty".to_string(),
                            )]);
                        }
                        Selector::Text(s.to_string())
                    }
                    // P393: regex(pattern) → selector regex sobre texto.
                    Value::Regex(re) => Selector::Regex(re),
                    // P791 — `<lbl>` → selector por label (paridade vanilla
                    // `Selector::Label`). Aplicação dedicada em
                    // `intercept_labelled` (não viaja pela travessia principal).
                    Value::Label(l) => Selector::Label(l),
                    // **P417 (M)** — Selector como valor de primeira classe
                    // (`heading.where(level: 1)`). Converte do selector de query
                    // para o selector de show rule.
                    Value::Selector(sel) => {
                        selector_matching::query_selector_to_show_selector(
                            sel,
                            sel_expr.span(),
                        )?
                    }
                    // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — o
                    // selector `callout` resolve para uma `FuncRepr::Element`, que
                    // não tem fn-ptr nativo. Casa por **kind dinâmico** (nome).
                    Value::Func(ref f) if f.element_name().is_some() => {
                        Selector::DynKind(f.element_name().unwrap().to_string())
                    }
                    Value::Func(ref f) => {
                        // Passo 84.3 (encerra DEBT-21): resolver NodeKind
                        // por identidade do function pointer da nativa
                        // subjacente, não pelo nome textual. Aliasing via
                        // `#let alias = heading` (clone do mesmo Arc<Func>)
                        // ou re-registo da mesma fn com nome diferente
                        // continuam a apontar para o mesmo `fn` — match.
                        //
                        // Closures retornam `None` em `native_fn_addr()` —
                        // function pointers de closures não são estáveis.
                        use crate::compiler::stdlib::{
                            native_emph, native_enum, native_figure, native_footnote,
                            native_heading, native_link, native_list,
                            native_outline_entry, native_overline, native_par,
                            native_quote, native_raw, native_smallcaps, native_strike,
                            native_strong, native_subscript, native_superscript,
                            native_underline,
                        };
                        use std::ptr::fn_addr_eq;
                        let mut base = f;
                        while let crate::entities::func::FuncRepr::With(bound) =
                            base.repr()
                        {
                            base = &bound.0;
                        }
                        if !std::ptr::eq(base, f)
                            && base.native_fn_addr().is_some_and(|call| {
                                [
                                    crate::compiler::stdlib::native_move
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_rotate
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_scale
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_skew
                                        as fn(_, _, _, _) -> _,
                                ]
                                .into_iter()
                                .any(|candidate| fn_addr_eq(call, candidate))
                            })
                        {
                            return Err(vec![SourceDiagnostic::error(
                                sel_expr.span(),
                                "only element functions can be used as selectors",
                            )]);
                        }
                        match f.native_fn_addr() {
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_heading as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Heading)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_figure as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Figure)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_strong as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Strong)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_emph as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Emph)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_raw as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Raw)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_underline as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Underline)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_strike as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Strike)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_overline as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Overline)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_smallcaps as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Smallcaps)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_subscript as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Subscript)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_superscript as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Superscript)
                            }
                            // P494 — selectores de elementos de documento.
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_link as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Link)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_quote as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Quote)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_footnote as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Footnote)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_list as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::List)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_enum as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Enum)
                            }
                            // P863 — `par` como função nativa mapeia para nó de parágrafo.
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_par as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NodeKind(NodeKind::Par)
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    native_outline_entry as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    crate::compiler::stdlib::native_smartquote
                                        as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    crate::compiler::stdlib::native_math_equation
                                        as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    crate::compiler::stdlib::native_table_cell
                                        as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            Some(addr)
                                if fn_addr_eq(
                                    addr,
                                    crate::compiler::stdlib::native_table
                                        as fn(_, _, _, _) -> _,
                                ) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            Some(addr)
                                if [
                                    crate::compiler::stdlib::native_move
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_rotate
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_scale
                                        as fn(_, _, _, _) -> _,
                                    crate::compiler::stdlib::native_skew
                                        as fn(_, _, _, _) -> _,
                                ]
                                .into_iter()
                                .any(|call| fn_addr_eq(addr, call)) =>
                            {
                                Selector::NativeElement(f.clone())
                            }
                            _ if crate::compiler::eval::bindings::native_element_fields(f).is_some() => {
                                Selector::NativeElement(f.clone())
                            }
                            Some(_) => {
                                return Err(vec![SourceDiagnostic::error(
                                    sel_expr.span(),
                                    format!(
                                        "função '{}' não é um tipo de nó suportado como selector. \
                                 Tipos suportados: heading, figure, strong, emph, raw, \
                                 underline, strike, overline, smallcaps, sub, super, \
                                 link, quote, list, enum, par.",
                                        f.name().unwrap_or("<anónima>")
                                    ),
                                )]);
                            }
                            None => {
                                return Err(vec![SourceDiagnostic::error(
                                    sel_expr.span(),
                                    "o selector de show rule deve ser uma função nativa \
                             ou uma string literal. Closures não são suportadas."
                                        .to_string(),
                                )]);
                            }
                        }
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            format!(
                                "selector inválido para show rule: {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
        }
    };

    // Classificar a transformação. Show-set (`#show k: set …`) é detetado pelo
    // tipo do nó (`Expr::SetRule`) e **capturado sem mutar `engine.styles`**
    // globalmente (P352); as restantes formas avaliam para um `Value` e
    // classificam-se em `Transformation`.
    let transform_expr = show_rule.transform();
    let transform = match transform_expr {
        Expr::SetRule(set) => {
            Transformation::Style(Arc::new(capture_set_styles(set, scopes, ctx, engine)?))
        }
        expr => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::None => Transformation::Content(Content::Empty),
                Value::Func(f) => Transformation::Func(f),
                Value::Content(c) => Transformation::Content(c),
                Value::Str(s) => Transformation::Str(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        expr.span(),
                        format!(
                            "transformação de show rule inválida: esperado função, \
                         Content, string ou set rule, recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            }
        }
    };
    let id = ctx.next_rule_id;
    ctx.next_rule_id += 1;
    // Reconstruir o slice com a nova regra (Passo 95 + 109: show_rules
    // é campo do Engine, mutação local via &mut).
    let mut rules = engine.show_rules.to_vec();
    rules.push(ShowRule { id, selector, transform });
    *engine.show_rules = Arc::from(rules);
    Ok(Value::None)
}

#[cfg(test)]
mod v16_table_tests {

    use crate::contracts::world::World;
    use crate::entities::{
        file_id::FileId,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                ),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn run(
        text: &str,
    ) -> crate::entities::source_result::SourceResult<crate::entities::module::Module>
    {
        let w = TestWorld::new(text);
        crate::compiler::eval::tests::eval_for_test(&w, &w.source)
    }
    #[test]
    fn v16_table_show_none_literal_is_empty_content() {
        let m = run("#show \"gone\": none\ngoneZ").unwrap();
        let text = m.content().unwrap().plain_text();
        assert!(!text.contains("gone"));
        assert!(text.contains('Z'));
    }
    #[test]
    fn v16_table_show_none_element_is_empty_content() {
        let m = run("#show strong: none\n*gone* Z").unwrap();
        let text = m.content().unwrap().plain_text();
        assert!(!text.contains("gone"));
        assert!(text.contains('Z'));
    }
    #[test]
    fn v16_table_show_none_does_not_admit_other_scalars() {
        for value in ["1", "auto"] {
            assert!(run(&format!("#show strong: {value}\n*gone*")).is_err());
        }
    }
}
