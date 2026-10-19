//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/expression_dispatch.md
//! @prompt-hash ed183332
//! @layer L1
//! @updated 2026-10-07

use super::*;

struct CodeSetTail {
    prefix: Value,
    styles: crate::entities::style::Styles,
    span: Span,
    carrier: Option<Content>,
    recipe: crate::entities::content::ShowRecipeContent,
}

fn style_code_tail(content: Content, styles: crate::entities::style::Styles) -> Content {
    match content {
        Content::Source(source) => {
            style_code_tail(source.body.clone(), styles).with_source_span(source.span)
        }
        Content::Styled(inner) => {
            let styles = StyleChain::empty()
                .push_styles(&styles)
                .push_styles(&inner.styles)
                .collapse();
            Content::styled(
                inner.body.clone(),
                crate::entities::style::Styles::from_delta(styles),
            )
        }
        other => Content::styled(other, styles),
    }
}

pub(crate) fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = expr.span();
    let previous = ctx.replace_context_read_styles(engine.styles.clone());
    let previous_file = ctx.replace_context_read_file(Some(engine.current_file));
    let previous_rules = ctx.replace_context_read_rules(
        engine.show_rules.clone(),
        engine.active_guards.clone(),
    );
    let result = eval_expr_inner(expr, scopes, ctx, engine).map(|value| {
        let content_origin_required =
            matches!(expr, Expr::ContentBlock(_) | Expr::CodeBlock(_))
                || (ctx.defer_show_rules && !engine.show_rules.is_empty());
        match value {
            Value::Func(func) => Value::Func(func.with_diagnostic_span(span)),
            Value::Content(content) if content_origin_required => {
                Value::Content(content.with_source_span(span))
            }
            other => other,
        }
    });
    ctx.replace_context_read_styles(previous);
    ctx.replace_context_read_file(previous_file);
    ctx.replace_context_read_rules(previous_rules.0, previous_rules.1);
    result
}

fn eval_expr_inner(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node) => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node) => Ok(Value::Str(EcoString::from(node.get()?))),
        Expr::Bool(node) => Ok(Value::Bool(node.get())),
        Expr::None(_) => Ok(Value::None),
        Expr::Auto(_) => Ok(Value::Auto),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            if name == "asset" {
                if let Some(value) = scopes.get(name) {
                    return Ok(value.clone());
                }
                if !ctx
                    .features
                    .contains(crate::entities::compiler_features::Feature::Bundle)
                {
                    return Err(vec![SourceDiagnostic::error(
                        ident.span(),
                        "cannot access variable `asset` because the `bundle` feature is not enabled",
                    )
                    .with_hint("try enabling the `bundle` feature")
                    .with_hint("see https://typst.app/help/compiler-features for more details")]);
                }
            }
            if name == "html"
                && !ctx
                    .features
                    .contains(crate::entities::compiler_features::Feature::Html)
            {
                return Err(vec![SourceDiagnostic::error(
                    ident.span(),
                    "cannot access variable `html` because the `html` feature is not enabled",
                )
                .with_hint("try enabling the `html` feature")
                .with_hint("see https://typst.app/help/compiler-features for more details")]);
            }
            // P772r — hint de subtracção quando o nome contém hífen,
            // paridade vanilla `foundations/scope.rs::unknown_variable`.
            scopes
                .get(name)
                .cloned()
                .ok_or_else(|| vec![bindings::unknown_variable(ident.span(), name)])
        }

        Expr::LetBinding(binding) => bindings::eval_let(binding, scopes, ctx, engine),

        Expr::CodeBlock(code_block) => {
            let textual_frame = rules::textual::enter_scope(ctx, engine);
            // Bloco de código — styles e show_rules locais (atomização
            // Passos 94 e 95). `#set`/`#show` dentro do bloco mutam as
            // cópias locais mas não afectam o chamador. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            let mut local_styles = engine.styles.clone();
            let insertion_styles = local_styles.clone();
            let insertion_rules = engine.show_rules.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            let mut output = Value::None;
            let mut set_tails: Vec<CodeSetTail> = Vec::new();
            let mut styled_set_tail = false;
            let mut set_tail_rules = Arc::clone(engine.show_rules);
            // **P772l** — paridade vanilla `ast::CodeBlock::eval`
            // (`typst-eval/src/code.rs:317-323`): o bloco introduz um
            // âmbito léxico próprio via `scopes.enter()`/`exit()`. Sem isto,
            // um `let` interno mutava o âmbito do chamador directamente —
            // medido: `#let x = 1; #{ let x = 2; x }; #x` devolvia `2, 2`
            // em vez de `2, 1` (shadowing vazava para fora do bloco).
            scopes.enter();
            let result = (|| {
                let mut local_engine = Engine {
                    world: engine.world,
                    font_metrics: engine.font_metrics,
                    route: engine.route,
                    styles: &mut local_styles,
                    show_rules: &mut local_show_rules,
                    active_guards: &mut *engine.active_guards,
                    current_file: engine.current_file,
                    sink: &mut local_sink,
                };
                for expr in code_block.body().exprs() {
                    // **P728** — o valor do bloco é o `join` sequencial dos
                    // valores das expressões (paridade vanilla
                    // `typst-eval/src/code.rs:57` + `ops::join`), não só o
                    // da última. Medido: `{ (1,); (2,) }` → `(1, 2)`.
                    let span = expr.span();
                    if matches!(expr, Expr::SetRule(_)) {
                        let before = local_engine.styles.clone();
                        let carrier = eval_expr(expr, scopes, ctx, &mut local_engine)?;
                        if ctx.flow.is_some() {
                            break;
                        }
                        let styles = crate::entities::style::Styles::from_delta(
                            local_engine
                                .styles
                                .local_delta_since(&before)
                                .expect("set styles must descend from their entry"),
                        );
                        styled_set_tail |= !styles.is_empty();
                        set_tail_rules = Arc::clone(local_engine.show_rules);
                        set_tails.push(CodeSetTail {
                            prefix: std::mem::replace(&mut output, Value::None),
                            styles,
                            span,
                            carrier: value_to_display_content(carrier),
                            recipe: crate::entities::content::ShowRecipeContent {
                                body: Content::Empty,
                                textual: false,
                                context_id_floor: ctx.next_context_id,
                                rules: local_engine
                                    .show_rules
                                    .iter()
                                    .filter(|r| {
                                        !insertion_rules
                                            .iter()
                                            .any(|base| base.id == r.id)
                                    })
                                    .cloned()
                                    .collect::<Vec<_>>()
                                    .into(),
                                styles: local_engine.styles.clone(),
                                guards: local_engine.active_guards.clone(),
                                file: local_engine.current_file,
                            },
                        });
                        continue;
                    }
                    let mut value = eval_expr(expr, scopes, ctx, &mut local_engine)?;
                    if let Value::Content(content) = &mut value {
                        *content = rules::textual::capture_local_output(
                            content.clone(),
                            ctx,
                            &local_engine,
                        );
                        *content = crate::compiler::realize::sequences::capture_at(
                            content.clone(),
                            span,
                            ctx,
                            &local_engine,
                        );

                        let local_delta = if styled_set_tail {
                            StyleChain::empty().collapse()
                        } else {
                            local_engine
                                .styles
                                .local_delta_since(&insertion_styles)
                                .expect("local styles must descend from CodeBlock entry")
                        };
                        let local_tail = !local_delta.is_empty()
                            || !Arc::ptr_eq(
                                local_engine.show_rules,
                                if styled_set_tail {
                                    &set_tail_rules
                                } else {
                                    &insertion_rules
                                },
                            );
                        if styled_set_tail
                            && !local_tail
                            && crate::compiler::realize::table_cells_needed(content)
                        {
                            // A fronteira lexical transportará a receita da cauda.
                        } else if local_tail
                            && crate::compiler::realize::table_cells_needed(content)
                        {
                            let recipe = Content::ShowRecipe(Arc::new(
                                crate::entities::content::ShowRecipeContent {
                                    body: content.clone(),
                                    textual: false,
                                    context_id_floor: ctx.next_context_id,
                                    rules: local_engine
                                        .show_rules
                                        .iter()
                                        .filter(|r| {
                                            !insertion_rules
                                                .iter()
                                                .any(|base| base.id == r.id)
                                        })
                                        .cloned()
                                        .collect::<Vec<_>>()
                                        .into(),
                                    styles: local_engine.styles.clone(),
                                    guards: local_engine.active_guards.clone(),
                                    file: local_engine.current_file,
                                },
                            ));
                            let mut styles =
                                crate::entities::style::Styles::from_delta(local_delta);
                            styles = styles
                                .push_custom(
                                    "eval.table.local.recipe",
                                    Value::Content(recipe),
                                )
                                .push_custom("eval.table.local", Value::Bool(true));
                            *content = Content::styled(content.clone(), styles);
                        } else {
                            *content = crate::compiler::realize::capture_table_insertion(
                                content.clone(),
                                ctx,
                                &mut local_engine,
                            )?;
                            if !styled_set_tail
                                && local_engine.styles.collapse()
                                    != insertion_styles.collapse()
                                && crate::compiler::realize::bare_math::needs_preparation(
                                    content,
                                )
                            {
                                *content = Content::styled(
                                    content.clone(),
                                    crate::entities::style::Styles::from_delta(
                                        local_engine.styles.collapse(),
                                    ),
                                );
                            }
                        }
                    }
                    output = operators::join(output, value)
                        .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
                    if ctx.flow.is_some() {
                        break;
                    }
                }
                for mut tail in set_tails.into_iter().rev() {
                    let raw_display = matches!(output, Value::Bool(_) | Value::Auto);
                    let mut content =
                        value_to_display_content(output).unwrap_or(Content::Empty);
                    if raw_display {
                        content =
                            rules::intercept_content(content, ctx, &mut local_engine)?;
                    }
                    if let Some(carrier) = tail.carrier {
                        content = Content::sequence(vec![carrier, content]);
                    }
                    if !tail.styles.is_empty() {
                        if crate::compiler::realize::table_cells_needed(&content) {
                            tail.recipe.body = content.clone();
                            tail.styles = tail
                                .styles
                                .push_custom(
                                    "eval.table.local.recipe",
                                    Value::Content(Content::ShowRecipe(Arc::new(
                                        tail.recipe,
                                    ))),
                                )
                                .push_custom("eval.table.local", Value::Bool(true));
                        }
                        content = style_code_tail(
                            content,
                            tail.styles.push_custom("eval.flow.scope", Value::Bool(true)),
                        );
                    }
                    output = operators::join(tail.prefix, Value::Content(content))
                        .map_err(|message| {
                            vec![SourceDiagnostic::error(tail.span, message)]
                        })?;
                }
                // Emit before the caller handles the return, so this diagnostic
                // can coexist with an invalid-return error. The second hint is
                // a nonexecuting scan for actual state/counter update content.
                if let Some(FlowEvent::Return(span, Some(_), false)) = &ctx.flow {
                    if let Value::Content(c) = &output {
                        let hint2 = if content_has_state_or_counter(c) {
                            "state/counter updates are content that must end up in the document to have an effect"
                        } else {
                            ""
                        };
                        local_engine.sink.warn_note2(
                            *span,
                            "this return unconditionally discards the content before it",
                            "try omitting the `return` to automatically join all values",
                            hint2,
                        );
                    }
                }
                if let Some(FlowEvent::Return(
                    span,
                    Some(Value::Content(content)),
                    conditional,
                )) = ctx.flow.clone()
                {
                    ctx.flow = Some(FlowEvent::Return(
                        span,
                        Some(Value::Content(rules::textual::capture_local_output(
                            content,
                            ctx,
                            &local_engine,
                        ))),
                        conditional,
                    ));
                }
                Ok(output)
            })();
            scopes.exit();
            rules::textual::leave_scope(ctx, textual_frame);
            result
        }

        // **P715** — `Assign`/`AddAssign`/`SubAssign`/`MulAssign`/`DivAssign`
        // não podem avaliar `lhs` como valor (precisam do nome/local para
        // mutar) — intercepta antes do dispatch genérico de operadores.
        // Ver `bindings::eval_assign`.
        Expr::Binary(binary)
            if matches!(
                binary.op(),
                BinOp::Assign
                    | BinOp::AddAssign
                    | BinOp::SubAssign
                    | BinOp::MulAssign
                    | BinOp::DivAssign
            ) =>
        {
            bindings::eval_assign(binary, scopes, ctx, engine)
        }

        // **P728** — short-circuit de `and`/`or` (paridade vanilla
        // `apply_binary` em `typst-eval/src/ops.rs:52-66`): o segundo
        // operando só é avaliado se o primeiro não decidir o resultado.
        // Medido: `false and (1/0 == 0)` → `false` (sem erro de divisão).
        Expr::Binary(binary) if matches!(binary.op(), BinOp::And | BinOp::Or) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, engine)?;
            let decided = matches!(
                (binary.op(), &lhs),
                (BinOp::And, Value::Bool(false)) | (BinOp::Or, Value::Bool(true))
            );
            if decided {
                Ok(lhs)
            } else {
                let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;
                operators::eval_binary_op(binary.op(), lhs, rhs)
                    .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
            }
        }

        Expr::Binary(binary) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, engine)?;
            let rhs = eval_expr(binary.rhs(), scopes, ctx, engine)?;
            operators::eval_binary_op(binary.op(), lhs, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
        }

        Expr::Unary(unary) => {
            let operand = eval_expr(unary.expr(), scopes, ctx, engine)?;
            operators::eval_unary_op(unary.op(), operand)
                .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
        }

        Expr::Conditional(cond) => {
            control_flow::eval_conditional(cond, scopes, ctx, engine)
        }
        Expr::WhileLoop(loop_expr) => {
            control_flow::eval_while(loop_expr, scopes, ctx, engine)
        }
        Expr::ForLoop(loop_expr) => {
            control_flow::eval_for(loop_expr, scopes, ctx, engine)
        }

        Expr::Closure(c) => closures::eval_closure_expr(c, scopes, ctx, engine),
        Expr::FuncCall(c) => call_dispatch::eval_func_call(c, scopes, ctx, engine),

        Expr::Strong(s) => markup::eval_strong(s, scopes, ctx, engine),
        Expr::Emph(e) => markup::eval_emph(e, scopes, ctx, engine),
        Expr::Heading(h) => markup::eval_heading(h, scopes, ctx, engine),
        Expr::Raw(r) => markup::eval_raw(r, scopes, ctx, engine),
        Expr::Link(l) => markup::eval_link(l, &*engine.styles),
        Expr::ListItem(i) => markup::eval_list_item(i, scopes, ctx, engine),
        Expr::EnumItem(i) => markup::eval_enum_item(i, scopes, ctx, engine),

        Expr::FieldAccess(a) => bindings::eval_field_access(a, scopes, ctx, engine),

        Expr::SetRule(s) => rules::eval_set_rule(s, scopes, ctx, engine),

        Expr::ContentBlock(content_block) => {
            // Content block [ ] — styles locais ao bloco. Engine
            // reconstruído localmente (ADR-0044, Passo 109).
            //
            // **Caso 4 / f3s3 (P340, F-realização fatia 2):** `show_rules` também
            // local (clone O(1) do Arc), espelhando o `CodeBlock` (`{}`). Antes,
            // o `[]` partilhava `&mut *engine.show_rules` → um `#show` dentro do
            // bloco VAZAVA para fora (mutava a chain do chamador da declaração em
            // diante). Agora confina ao escopo do bloco — paridade com o vanilla,
            // que confina via `StyledElem` (`content/mod.rs:744-752`). É o
            // confinamento estrutural que faltava ao modelo eager no `[]`.
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            // **P772l** — paridade vanilla `ast::ContentBlock::eval`
            // (`typst-eval/src/code.rs:326-332`): mesmo âmbito léxico
            // próprio do `CodeBlock`. Sem isto, `#let` dentro de `[ ]`
            // vazava para o âmbito do chamador (mesmo bug do CodeBlock).
            scopes.enter();
            let mut local_engine = Engine {
                world: engine.world,
                font_metrics: engine.font_metrics,
                route: engine.route,
                styles: &mut local_styles,
                show_rules: &mut local_show_rules,
                active_guards: &mut *engine.active_guards,
                current_file: engine.current_file,
                sink: &mut local_sink,
            };
            let result = eval_markup_impl(
                content_block.body().to_untyped(),
                scopes,
                ctx,
                &mut local_engine,
                false,
            );
            scopes.exit();
            result
        }

        Expr::Equation(eq) => {
            let block = eq.block();
            let body = math::eval_math_content(scopes, ctx, engine, eq.body())?;
            // F-5a de-bake (P364, `f_fronteira_e1.md` §3a.9): a equação **não
            // baka** mais o gate. O `#set math.equation(numbering:)` vive **só na
            // chain** (`custom("equation.numbering")` no `Content::Styled` da
            // fatia-1). O consumidor lê o gate da chain e mantém `block &&
            // numbering` (só equações de bloco numeram, paridade vanilla). Fonte
            // única.
            let content = crate::compiler::stdlib::equation_content(body, block);
            rules::intercept_content(content, ctx, engine).map(Value::Content)
        }

        Expr::Math(math) => {
            // Math node isolado (fora de Equation) — produzir como sequence.
            let content = math::eval_math_content(scopes, ctx, engine, math)?;
            Ok(Value::Content(content))
        }

        Expr::ModuleImport(i) => modules::eval_module_import(i, scopes, ctx, engine),
        Expr::ModuleInclude(i) => modules::eval_module_include(i, scopes, ctx, engine),

        // Passo 56 — referência cruzada: @nome → Content::Ref placeholder.
        // **P788** — suplemento explícito `@nome[sup]` avaliado como
        // `ContentBlock` normal (antes: descartado — `@sec1[Cap]`
        // renderizava o suplemento default; vanilla: "Cap 1").
        Expr::Ref(ref_node) => {
            let name = ref_node.target().to_string();
            let supplement = match ref_node.supplement() {
                Some(block) => {
                    match eval_expr(Expr::ContentBlock(block), scopes, ctx, engine)? {
                        Value::Content(c) => Some(c),
                        _ => None,
                    }
                }
                None => None,
            };
            Ok(Value::Content(Content::reference_with_supplement(name, supplement)))
        }

        // Passo 56 — label em contexto de código; associação retroactiva em markup
        // acontece via SyntaxKind::Label. Em código, <label> é um valor de primeira
        // classe (P509) para query/locate.
        Expr::Label(label_node) => {
            let name = label_node.get().to_string();
            Ok(Value::Label(crate::entities::label::Label(name)))
        }

        Expr::ShowRule(s) => rules::eval_show_rule(s, scopes, ctx, engine),

        // Passo 81 — array literal `(1fr, 1fr)` / `(10pt, auto, 1fr)`.
        // Necessário para o argumento `columns` de `grid()`.
        // **P718** — mirror de `ast::Array::eval` (vanilla `code.rs:224-271`):
        // `..spread` de `none` é ignorado, de `array` estende, de `dict`
        // erra (com hint se TODOS os itens forem spreads de dict — ver
        // `remaining_are_dict_spreads`), outro tipo erra sem hint.
        Expr::Array(arr) => {
            let items_vec: Vec<ArrayItem> = arr.items().collect();
            let mut result: Vec<Value> = Vec::with_capacity(items_vec.len());
            let mut all_dict_spreads = true;
            for (i, item) in items_vec.iter().enumerate() {
                match *item {
                    ArrayItem::Pos(expr) => {
                        all_dict_spreads = false;
                        result.push(eval_expr(expr, scopes, ctx, engine)?);
                    }
                    ArrayItem::Spread(spread) => {
                        let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                        match value {
                            Value::None => {}
                            Value::Array(a) => {
                                all_dict_spreads = false;
                                result.extend(a);
                            }
                            Value::Dict(dict) => {
                                let all_remaining_are_dict_spreads = all_dict_spreads
                                    && remaining_are_dict_spreads(
                                        &items_vec[i + 1..],
                                        scopes,
                                        ctx,
                                        engine,
                                    );
                                if !all_remaining_are_dict_spreads {
                                    return Err(vec![SourceDiagnostic::error(
                                        spread.span(),
                                        format!(
                                            "cannot spread {} into array",
                                            Value::Dict(dict).type_name()
                                        ),
                                    )]);
                                }
                                let full_text =
                                    arr.to_untyped().clone().into_text().to_string();
                                let fixed = full_text.replacen('(', "(: ", 1);
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    "cannot spread dictionary into array".to_string(),
                                )
                                .with_hint(format!(
                                    "add a colon to create a dictionary instead: `{fixed}`"
                                ))]);
                            }
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    format!(
                                        "cannot spread {} into array",
                                        vanilla_type_name(&other)
                                    ),
                                )]);
                            }
                        }
                    }
                }
            }
            Ok(Value::Array(result))
        }

        // **P466** — dict literal `(a: 1, b: 2)` e `("a": 1)`.
        // Dicts com chaves keyed não-string (ex.: regex) são deixados como
        // `Value::None` para que callers especializados (ex.: `#set text(font:)`)
        // possam inspeccionar o AST directamente.
        // **P718** — `..spread` mirror de `ast::Dict::eval` (vanilla
        // `code.rs:273-306`): `none` ignorado, `dict` estende, outro erra.
        Expr::Dict(dict) => {
            let mut map = indexmap::IndexMap::default();
            for item in dict.items() {
                match item {
                    crate::entities::ast::expr::DictItem::Named(named) => {
                        let key = named.name().as_str();
                        let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                        map.insert(key.into(), value);
                    }
                    crate::entities::ast::expr::DictItem::Keyed(keyed) => {
                        let key_expr = keyed.key();
                        let key = match key_expr {
                            crate::entities::ast::expr::Expr::Str(node) => {
                                EcoString::from(node.get()?)
                            }
                            _ => return Ok(Value::None),
                        };
                        let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                        map.insert(key, value);
                    }
                    crate::entities::ast::expr::DictItem::Spread(spread) => {
                        let value = eval_expr(spread.expr(), scopes, ctx, engine)?;
                        match value {
                            Value::None => {}
                            Value::Dict(d) => map.extend(d),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    spread.span(),
                                    format!(
                                        "cannot spread {} into dictionary",
                                        vanilla_type_name(&other)
                                    ),
                                )]);
                            }
                        }
                    }
                }
            }
            Ok(Value::Dict(map))
        }

        // `(expr)` — parêntese de agrupamento. Expressão única dentro de
        // parênteses avalia para o valor da expressão. Passo 83.
        // (Um tuplo com um elemento requer a vírgula trailing: `(x,)`.)
        Expr::Parenthesized(paren) => eval_expr(paren.expr(), scopes, ctx, engine),

        // Passo 76 — literais numéricos com unidade (ex: 100pt, 1.5em).
        Expr::Numeric(num) => {
            use crate::entities::ast::expr::Unit;
            use crate::entities::layout_types::{Abs, Angle, Length};
            let (value, unit) = num.get();
            match unit {
                Unit::Pt => Ok(Value::Length(Length { abs: Abs(value), em: 0.0 })),
                Unit::Mm => Ok(Value::Length(Length {
                    abs: Abs(value * Length::PT_PER_MM),
                    em: 0.0,
                })),
                Unit::Cm => Ok(Value::Length(Length {
                    abs: Abs(value * Length::PT_PER_CM),
                    em: 0.0,
                })),
                Unit::In => Ok(Value::Length(Length { abs: Abs(value * 72.0), em: 0.0 })),
                Unit::Em => Ok(Value::Length(Length { abs: Abs(0.0), em: value })),
                Unit::Deg => Ok(Value::Angle(Angle::deg(value))),
                Unit::Rad => Ok(Value::Angle(Angle::rad(value))),
                // P842 (#32) — percentual puro materializa `Ratio` (paridade
                // vanilla medida: `type(50%)` → ratio). Pré-P842 era
                // `Value::Relative` com abs zero (P469), o que fundia os
                // tipos ratio/relative. `Ratio + Length` → `Relative` nos
                // operadores, como no vanilla.
                Unit::Percent => Ok(Value::Ratio(
                    crate::entities::layout_types::Ratio::from_percent(value),
                )),
                Unit::Fr => Ok(Value::Fraction(value)),
            }
        }

        // **P506** — `context { body }` cria um bloco de delayed evaluation.
        // O parser expõe `Contextual` como sugar; construímos uma closure
        // sem argumentos que captura o scope actual e devolvemos
        // `Content::ContextBlock`.
        Expr::Contextual(node) => {
            let body_span = node.body().span();
            let body = node.body().to_untyped().clone();
            let captured = std::sync::Arc::new(scopes.snapshot());
            let closure = Func::closure(ClosureRepr {
                name: None,
                params: Vec::new(),
                sink_name: None,
                body,
                captured,
                // P772q — bloco `context { }`, paridade vanilla
                // `CapturesVisitor::new(scopes, Capturer::Context)`.
                capturer: crate::entities::scope::Capturer::Context,
            })
            .with_diagnostic_span(body_span);
            let id = ctx.next_context_id();
            Ok(Value::Content(crate::compiler::realize::capture_context_recipe(
                Content::ContextBlock(Arc::new(ContextBlockElem { id, closure })),
                ctx,
                engine,
            )))
        }

        Expr::Escape(v) => Ok(Value::Str(EcoString::from(v.get()))),
        Expr::Shorthand(v) => Ok(Value::Str(EcoString::from(v.get()))),
        Expr::Linebreak(_) => Ok(Value::Content(Content::linebreak())),

        // P635 — controlo de fluxo: definir `FlowEvent` em `ctx.flow` e
        // devolver `Value::None`. O consumo (e a detecção de "fora de
        // contexto") é feito pelos ciclos, por `apply_closure` e pelo
        // entrypoint `eval_with_full_error`.
        Expr::LoopBreak(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Break(node.span()));
            }
            Ok(Value::None)
        }
        Expr::LoopContinue(node) => {
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Continue(node.span()));
            }
            Ok(Value::None)
        }
        Expr::FuncReturn(node) => {
            let value = node
                .body()
                .map(|body| eval_expr(body, scopes, ctx, engine))
                .transpose()?;
            if ctx.flow.is_none() {
                ctx.flow = Some(FlowEvent::Return(node.span(), value, false));
            }
            Ok(Value::None)
        }

        // Fronteira deliberada — variantes estritamente estruturais que não
        // entram no dispatcher normal (markup/math) ou ainda não migradas.
        Expr::Text(_)
        | Expr::Space(_)
        | Expr::Parbreak(_)
        | Expr::SmartQuote(_)
        | Expr::TermItem(_)
        | Expr::MathText(_)
        | Expr::MathIdent(_)
        | Expr::MathShorthand(_)
        | Expr::MathAlignPoint(_)
        | Expr::MathDelimited(_)
        | Expr::MathAttach(_)
        | Expr::MathPrimes(_)
        | Expr::MathFrac(_)
        | Expr::MathRoot(_) => Ok(Value::None),

        // **P715** — `(a, b) = expr`. Ver `bindings::eval_destruct_assignment`.
        Expr::DestructAssignment(node) => {
            bindings::eval_destruct_assignment(node, scopes, ctx, engine)
        }
    }
}

/// Inspect discarded static content for update nodes without evaluating callbacks.
fn content_has_state_or_counter(content: &crate::entities::content::Content) -> bool {
    fn value_has_update(value: &Value) -> bool {
        match value {
            Value::Content(content) => content_has_state_or_counter(content),
            Value::LocatedContent(content, _) => {
                content_has_state_or_counter(content.content())
            }
            Value::Array(values) => values.iter().any(value_has_update),
            Value::Float(..)
            | Value::Decimal(..)
            | Value::None
            | Value::Bool(..)
            | Value::Int(..)
            | Value::Str(..)
            | Value::Dict(..)
            | Value::Module(..)
            | Value::Datetime(..)
            | Value::Func(..)
            | Value::Auto
            | Value::Length(..)
            | Value::Relative(..)
            | Value::Ratio(..)
            | Value::Angle(..)
            | Value::Color(..)
            | Value::Stroke(..)
            | Value::Fraction(..)
            | Value::Align(..)
            | Value::Location(..)
            | Value::Gradient(..)
            | Value::Regex(..)
            | Value::Tiling(..)
            | Value::Bytes(..)
            | Value::Duration(..)
            | Value::Version(..)
            | Value::Selector(..)
            | Value::Symbol(..)
            | Value::Args(..)
            | Value::State(..)
            | Value::Counter(..)
            | Value::Label(..)
            | Value::Dir(..)
            | Value::Path(..)
            | Value::Type(..)
            | Value::SpotColorant(..) => false,
        }
    }
    let mut found = false;
    content.visit_content_static(&mut |node| {
        if found {
            return false;
        }
        found = match node {
            Content::StateUpdate(_) | Content::CounterUpdate(_) => true,
            Content::Metadata(metadata) => value_has_update(&metadata.value),
            Content::Ref(reference) => reference
                .supplement
                .as_ref()
                .is_some_and(content_has_state_or_counter),
            Content::Dynamic(element) => element
                .as_any()
                .downcast_ref::<crate::entities::elements::raw::RawLineElem>()
                .is_some_and(|line| content_has_state_or_counter(&line.body)),
            Content::Styled(_) => {
                node.constructor_heading().is_some_and(|(_, styles)| {
                    styles
                        .delta()
                        .custom
                        .iter()
                        .rev()
                        .find(|(key, _)| key.as_str() == "heading.constructor.supplement")
                        .is_some_and(|(_, value)| value_has_update(value))
                })
            }
            Content::PageRun(page) => {
                // Page properties are deferred; the static public child is its body.
                found = content_has_state_or_counter(&page.body);
                return false;
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Par { .. }
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::CounterDisplay(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::State(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_) => false,
        };
        !found
    });
    found
}

/// **P718** — lookahead para o hint de `Expr::Array` (mirror do
/// `items.all(...)` de `ast::Array::eval`, vanilla `code.rs:253-265`):
/// devolve `true` sse todos os itens restantes forem `Spread` cuja
/// expressão avalia a `Value::Dict`. Qualquer `Pos`, spread de outro tipo,
/// ou erro de avaliação → `false` (short-circuit, sem consumir o resto).
fn remaining_are_dict_spreads(
    items: &[ArrayItem],
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> bool {
    for item in items {
        match item {
            ArrayItem::Spread(s) => match eval_expr(s.expr(), scopes, ctx, engine) {
                Ok(Value::Dict(_)) => continue,
                _ => return false,
            },
            ArrayItem::Pos(_) => return false,
        }
    }
    true
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}

#[cfg(test)]
mod v16_c_effect_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct MemoryWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl World for MemoryWorld {
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
    fn world(text: &str) -> MemoryWorld {
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        MemoryWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id, text.into()),
        }
    }

    fn warning(text: &str) -> SourceDiagnostic {
        let world = world(text);
        let (result, sink) =
            super::super::tests::eval_for_test_keep_sink(&world, &world.source);
        let errors = result.expect_err("return outside a function must remain invalid");
        assert!(errors.iter().any(|e| e.message.contains("cannot return outside")));
        sink.into_diagnostics()
            .into_iter()
            .find(|d| {
                d.message == "this return unconditionally discards the content before it"
            })
            .expect("discarded-content warning must coexist with the return error")
    }
    fn has_update_hint(warning: &SourceDiagnostic) -> bool {
        warning.hints.iter().any(|h| h == "state/counter updates are content that must end up in the document to have an effect")
    }
    #[test]
    fn v16_c_update_hint_follows_content_block() {
        let warning =
            warning("#{ let s = state(\"k\", 0); [#s.update(1)]; return \"x\" }");
        assert!(has_update_hint(&warning), "{warning:?}");
    }
    #[test]
    fn v16_c_update_hint_follows_strong_body() {
        let warning =
            warning("#{ let s = state(\"k\", 0); strong(s.update(1)); return \"x\" }");
        assert!(has_update_hint(&warning), "{warning:?}");
    }
    #[test]
    fn v16_c_update_hint_preserves_direct_and_opaque_context() {
        assert!(has_update_hint(&warning(
            "#{ state(\"k\", 0).update(1); return \"x\" }"
        )));
        let hidden = warning("#{ context { panic(\"EXECUTED\"); state(\"k\",0).update(1) }; return \"x\" }");
        assert!(!has_update_hint(&hidden));
        assert_eq!(hidden.hints.len(), 1);
        assert_eq!(
            hidden.hints[0],
            "try omitting the `return` to automatically join all values"
        );
    }
    #[test]
    fn v16_c_update_hint_inspects_metadata_content_and_arrays() {
        for field in ["state(\"k\",0).update(1)", "(state(\"k\",0).update(1),)"] {
            let source = format!("#{{ metadata({field}); return \"x\" }}");
            let diagnostic = warning(&source);
            assert!(has_update_hint(&diagnostic), "{source}: {diagnostic:?}");
        }
    }
    #[test]
    fn v16_c_update_hint_preserves_metadata_dictionary_opacity() {
        let diagnostic =
            warning("#{ metadata((value:state(\"k\",0).update(1))); return \"x\" }");
        assert!(!has_update_hint(&diagnostic));
    }
    #[test]
    fn v16_c_update_hint_inspects_ref_supplement() {
        let diagnostic =
            warning(r#"#{ ref(<x>, supplement:state("k",0).update(1)); return "x" }"#);
        assert!(has_update_hint(&diagnostic), "{diagnostic:?}");
    }
    #[test]
    fn v16_c_update_hint_inspects_native_raw_line_body() {
        let diagnostic =
            warning(r#"#{ raw.line(1,1,"x",state("k",0).update(1)); return "x" }"#);
        assert!(has_update_hint(&diagnostic), "{diagnostic:?}");
    }
    #[test]
    fn v16_c_update_hint_inspects_constructor_heading_supplement() {
        let diagnostic =
            warning(r#"#{ heading(supplement:state("k",0).update(1))[A]; return "x" }"#);
        assert!(has_update_hint(&diagnostic), "{diagnostic:?}");
    }
    #[test]
    fn v16_c_update_hint_preserves_heading_supplement_provenance() {
        for source in [
            r#"#{ set heading(supplement:state("k",0).update(1)); heading(numbering:"1")[A]; return "x" }"#,
            r#"#{ set heading(supplement:state("k",0).update(1)); heading(numbering:"1",supplement:none)[A]; return "x" }"#,
        ] {
            let diagnostic = warning(source);
            assert!(!has_update_hint(&diagnostic), "{source}: {diagnostic:?}");
        }
    }
    #[test]
    fn v16_c_update_hint_inspects_explicit_heading_override() {
        let diagnostic = warning(
            r#"#{ set heading(supplement:none); heading(numbering:"1",supplement:state("k",0).update(1))[A]; return "x" }"#,
        );
        assert!(has_update_hint(&diagnostic), "{diagnostic:?}");
    }
    #[test]
    fn v16_c_update_hint_excludes_deferred_page_properties() {
        for field in ["header", "footer", "supplement", "background", "foreground"] {
            let source =
                format!(r#"#{{ page({field}:state("k",0).update(1))[A]; return "x" }}"#);
            let diagnostic = warning(&source);
            assert!(!has_update_hint(&diagnostic), "{field}: {diagnostic:?}");
        }
    }
    #[test]
    fn v16_c_update_hint_preserves_static_field_boundaries() {
        for source in [
            r#"#{ ref(<x>,supplement:none); return "x" }"#,
            r#"#{ raw.line(1,1,"x",context {panic("EXECUTED");state("k",0).update(1)}); return "x" }"#,
            r#"#{ heading(supplement:it=>{panic("EXECUTED");state("k",0).update(1)})[A]; return "x" }"#,
            r#"#{ set heading(supplement:state("k",0).update(1)); heading[A]; return "x" }"#,
        ] {
            let diagnostic = warning(source);
            assert!(!has_update_hint(&diagnostic), "{source}: {diagnostic:?}");
        }
        let diagnostic = warning(r#"#{ page(state("k",0).update(1)); return "x" }"#);
        assert!(has_update_hint(&diagnostic), "{diagnostic:?}");
    }
    #[test]
    fn v16_c_update_hint_excludes_displays_and_observes_only_visual() {
        use crate::entities::content::RealizedContent;
        use crate::entities::state_update::StateUpdate;
        let update =
            Content::state_update("k", StateUpdate::Set(Box::new(Value::Int(1))));
        let display = Content::counter_display(
            crate::entities::counter::CounterKey::Str("k".into()),
        );
        assert!(!content_has_state_or_counter(&display));
        assert!(!content_has_state_or_counter(&Content::state_display("k", None)));
        assert!(!content_has_state_or_counter(&Content::counter_display_callback(
            crate::entities::counter::CounterKey::Str("k".into()),
            None
        )));
        assert!(!content_has_state_or_counter(&Content::state("k", Value::Int(0))));
        assert!(!content_has_state_or_counter(&Content::Realized(Arc::new(
            RealizedContent { original: update.clone(), visual: display.clone() }
        ))));
        assert!(content_has_state_or_counter(&Content::Realized(Arc::new(
            RealizedContent { original: display, visual: update }
        ))));
    }
}
