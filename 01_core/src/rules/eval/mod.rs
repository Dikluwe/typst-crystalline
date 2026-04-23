//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Dispatcher central do eval: `EvalContext` struct + impl, `pub fn eval`
//! entry point, `eval_markup` iterator, `eval_expr` dispatcher delegando
//! cada armo para o submódulo do respectivo domínio (ADR-0037 Regra 4,
//! completada no Passo 96.2).
//!
//! Armos triviais (literais `Int`/`Float`/`Bool`/etc., `Ref`/`Label`/
//! `Parenthesized`) permanecem inline; armos que contêm scoping
//! cross-cutting (`CodeBlock`, `ContentBlock`) também, por não
//! pertencerem a nenhum cluster em particular.

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};
use ecow::EcoString;

use crate::contracts::world::World;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
#[cfg(test)]
use crate::entities::counter_state::CounterAction;
use crate::entities::label::Label;
use crate::entities::ast::expr::{ArrayItem, Expr};
#[cfg(test)]
use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::ast::markup::Label as AstLabel;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::TextStyle;
use crate::entities::style_chain::StyleChain;
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Routines, Sink, Traced};
use crate::rules::scopes::Scopes;

// Submódulos por domínio (Passo 96.1, ADR-0037).
mod math;
pub(crate) mod operators;
mod control_flow;
pub(crate) mod closures;
mod bindings;
pub(crate) mod rules;
mod markup;
mod modules;

/// Contexto de execução partilhado durante eval().
///
/// Limite de segurança para prevenir loops infinitos:
/// - `max_loop_iterations`: limite total global de iterações. Um contador
///   local por loop permite "loop-bombing" (milhares de loops pequenos que
///   colectivamente travam o motor). Counter global impede isso: 1.000.000
///   iterações falham em segundos independentemente da distribuição.
///
/// A profundidade de chamadas **não** é verificada aqui — é verificada pelo
/// `Route<'a>` através de `route.check_call_depth()` em `apply_closure`
/// (Passo 93, ADR-0033 paridade com vanilla, `MAX_CALL_DEPTH = 80`). O
/// campo antigo `depth`/`max_call_depth`/`enter_call`/`leave_call` foi
/// removido (DEBT-45 parcialmente pago).
///
/// A rota de compilação (`Route<'a>`) **não** é campo do contexto — é
/// passada como parâmetro `route: Tracked<'r, Route<'r>>` às funções
/// `eval_*` que participam na recursão. Paridade estrutural com o vanilla
/// e primeira aplicação concreta da ADR-0036 (atomização progressiva,
/// Passo 92). O campo `route: Vec<FileId>` + API `with_route_id` do Passo
/// 90 foram eliminados no Passo 92 (DEBT-44 fechado).
///
/// A cadeia de estilos (`StyleChain`) **também não** é campo do contexto
/// desde o Passo 94 — propaga-se como `&mut StyleChain` nas funções
/// `eval_*`. Cada bloco de scoping (`CodeBlock`, `ContentBlock`,
/// `Strong`/`Emph`/`Heading`, corpo de closure) cria uma cópia local
/// (`let mut local_styles = styles.clone()` ou `styles.push(delta)`),
/// eliminando o antigo par save/restore sobre um campo partilhado.
/// Segunda aplicação concreta da ADR-0036.
pub struct EvalContext<'w> {
    #[allow(dead_code)] // usado quando `import` for implementado (Passo futuro)
    pub world: &'w dyn World,
    pub loop_iterations: usize,
    pub max_loop_iterations: usize,
    /// Próximo ID a atribuir a uma ShowRule (Passo 70).
    ///
    /// Contador monotónico — cumpre Regra 4 da ADR-0036 (alocador global
    /// ao eval, não estado de fluxo). Ficam fora do contexto, desde o
    /// Passo 95:
    /// - `show_rules: Arc<[ShowRule]>` — agora propagado como
    ///   `&mut Arc<[ShowRule]>` parâmetro (terceira aplicação da ADR-0036).
    /// - `active_guards: Vec<RuleId>` — agora propagado como
    ///   `&mut Vec<RuleId>` parâmetro (DEBT-39 encerrado).
    pub next_rule_id: crate::entities::show::RuleId,
    /// FileId do ficheiro Typst actualmente em avaliação (Passo 75, DEBT-25).
    /// Usado por `World::read_bytes` para resolver caminhos relativos ao ficheiro fonte.
    /// Actualizado ao entrar num `#include` e restaurado ao sair.
    pub current_file: FileId,
    /// Padrão de numeração activo para figuras (Passo 75, DEBT-14).
    /// Definido por `#set figure(numbering: "1")` e capturado em `native_figure`.
    /// `None` → figuras sem numeração automática.
    pub figure_numbering: Option<String>,
}

impl<'w> EvalContext<'w> {
    pub fn new(world: &'w dyn World, current_file: FileId) -> Self {
        Self {
            world,
            loop_iterations: 0,
            max_loop_iterations: 1_000_000,
            next_rule_id: 0,
            current_file,
            figure_numbering: None,
        }
    }

    /// Incrementa o contador de iterações e retorna Err se o limite foi atingido.
    pub fn tick_loop(&mut self, span: Span) -> SourceResult<()> {
        self.loop_iterations += 1;
        if self.loop_iterations > self.max_loop_iterations {
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "limite de iterações de loop atingido ({}) — \
                     possível loop infinito",
                    self.max_loop_iterations
                ),
            )])
        } else {
            Ok(())
        }
    }

}

/// Avalia um ficheiro Typst e retorna o módulo resultante.
///
/// Travessia AST parcial (Passo 17): avalia literais, Ident, Let, CodeBlock,
/// Binary, Unary, Conditional, WhileLoop, ForLoop, Closure, FuncCall.
/// Stdlib mínima injectada: `type`, `len`, `range`.
/// Fronteira deliberada: `_ => Ok(Value::None)` para Content, Styles (ADR-0017).
///
/// **Invariante**: não importa nada de `03_infra`. Acesso ao world
/// sempre via `World` (L1).
pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    let root = source.root();

    let mut ctx = EvalContext::new(world, source.id());

    // Route raiz com o FileId do ficheiro principal — primeira aplicação da
    // ADR-0036 (atomização progressiva, Passo 92): `Route<'a>` propagado por
    // `Tracked<'_, Route>` (covariante) entre frames em vez de `Vec<FileId>`
    // partilhado no contexto.
    let route = Route::root().with_id(source.id());

    // Segunda aplicação da ADR-0036 (Passo 94): `StyleChain` local ao
    // entry-point, propagado como `&mut StyleChain` às funções `eval_*`.
    // Eliminou o campo `styles` de `EvalContext` e o par save/restore.
    let mut styles = StyleChain::default_chain();

    // Terceira aplicação da ADR-0036 (Passo 95): `show_rules` (via
    // `&mut Arc<[ShowRule]>`) e `active_guards` (via `&mut Vec<RuleId>`)
    // também extraídos do contexto. Cada bloco cria cópias locais que
    // isolam mutações (`#show` e anti-recursão de show rules).
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards: Vec<RuleId> = Vec::new();

    let mut scopes = Scopes::new(None);
    // Stdlib como scope base — type, len, range visíveis em todo o documento
    let stdlib = make_stdlib();
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    scopes.enter();  // âmbito do módulo

    let content_val = eval_markup(
        root,
        &mut scopes,
        &mut ctx,
        route.track(),
        &mut styles,
        &mut show_rules,
        &mut active_guards,
    )?;

    let module_scope = scopes.exit();
    let content = match content_val {
        Value::Content(c) => Some(c),
        _ => None,
    };
    let mut module = Module::new(
        source.id().into_raw().get().to_string(),
        module_scope,
    );
    module.set_content(content);
    Ok(module)
}

fn eval_markup<'r>(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();

    for child in node.children() {
        match child.kind() {
            SyntaxKind::Text => {
                // Capturar o estilo activo no momento da produção (Passo 30).
                let style = TextStyle::from(&*styles);
                let text_node = Content::Text(child.text().as_str().into(), style);
                // Intercepção eager para Selector::Text (Passo 68).
                parts.push(rules::intercept_content(text_node, ctx, route, styles, show_rules, active_guards)?);
            }
            SyntaxKind::Space | SyntaxKind::Parbreak => parts.push(Content::Space),
            k if k.is_trivia() => continue,
            // Passo 56 — associação retroactiva: <label> envolve o nó precedente.
            // O parser expõe <label> como nó irmão (não filho) do nó anterior.
            // Entre o nó alvo e a label pode haver Space — salta-os para encontrar
            // o elemento real, re-insere-os a seguir ao Labelled.
            SyntaxKind::Label => {
                if let Some(label_ast) = child.cast::<AstLabel<'_>>() {
                    let name = label_ast.get().to_string();
                    // Recolher espaços finais para re-inserir após o Labelled.
                    let mut trailing: Vec<Content> = Vec::new();
                    while matches!(parts.last(), Some(Content::Space) | Some(Content::Empty)) {
                        trailing.push(parts.pop().unwrap());
                    }
                    if let Some(last) = parts.pop() {
                        parts.push(Content::Labelled {
                            target: Box::new(last),
                            label:  Label(name),
                        });
                        trailing.reverse();
                        parts.extend(trailing);
                    }
                    // Se parts estiver vazio após remover espaços, ignorar.
                }
            }
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    match eval_expr(expr, scopes, ctx, route, styles, show_rules, active_guards)? {
                        Value::Content(c) => parts.push(c),
                        Value::Str(s)     => {
                            let style = TextStyle::from(&*styles);
                            parts.push(Content::Text(s, style));
                        }
                        Value::None       => {}
                        _                 => {}
                    }
                }
            }
        }
    }

    Ok(Value::Content(Content::sequence(parts)))
}

fn eval_expr<'r>(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node)   => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node)   => Ok(Value::Str(EcoString::from(node.get()))),
        Expr::Bool(node)  => Ok(Value::Bool(node.get())),
        Expr::None(_)     => Ok(Value::None),
        Expr::Auto(_)     => Ok(Value::Auto),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            scopes.get(name)
                .cloned()
                .ok_or_else(|| vec![SourceDiagnostic::error(
                    ident.span(),
                    format!("unknown variable: {name}"),
                )])
        }

        Expr::LetBinding(binding) => bindings::eval_let(binding, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::CodeBlock(code_block) => {
            // Bloco de código — styles e show_rules locais (atomização
            // Passos 94 e 95). `#set`/`#show` dentro do bloco mutam as
            // cópias locais mas não afectam o chamador.
            let mut local_styles = styles.clone();
            let mut local_show_rules = Arc::clone(show_rules);
            let mut last = Value::None;
            for expr in code_block.body().exprs() {
                last = eval_expr(expr, scopes, ctx, route, &mut local_styles, &mut local_show_rules, active_guards)?;
            }
            Ok(last)
        }

        Expr::Binary(binary) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx, route, styles, show_rules, active_guards)?;
            let rhs = eval_expr(binary.rhs(), scopes, ctx, route, styles, show_rules, active_guards)?;
            operators::eval_binary_op(binary.op(), lhs, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
        }

        Expr::Unary(unary) => {
            let operand = eval_expr(unary.expr(), scopes, ctx, route, styles, show_rules, active_guards)?;
            operators::eval_unary_op(unary.op(), operand)
                .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
        }

        Expr::Conditional(cond) => control_flow::eval_conditional(cond, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::WhileLoop(loop_expr) => control_flow::eval_while(loop_expr, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::ForLoop(loop_expr) => control_flow::eval_for(loop_expr, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::Closure(c)  => closures::eval_closure_expr(c, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::FuncCall(c) => closures::eval_func_call(c, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::Strong(s)   => markup::eval_strong(s, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::Emph(e)     => markup::eval_emph(e, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::Heading(h)  => markup::eval_heading(h, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::Raw(r)      => markup::eval_raw(r),
        Expr::Link(l)     => markup::eval_link(l, styles),
        Expr::ListItem(i) => markup::eval_list_item(i, scopes, ctx, route, styles, show_rules, active_guards),
        Expr::EnumItem(i) => markup::eval_enum_item(i, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::FieldAccess(a) => bindings::eval_field_access(a, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::SetRule(s)  => rules::eval_set_rule(s, scopes, ctx, route, styles, show_rules, active_guards),

        Expr::ContentBlock(content_block) => {
            // Content block [ ] — styles locais ao bloco (atomização Passo 94).
            let mut local_styles = styles.clone();
            eval_markup(content_block.body().to_untyped(), scopes, ctx, route, &mut local_styles, show_rules, active_guards)
        }

        Expr::Equation(eq) => {
            let block = eq.block();
            let body  = math::eval_math_content(scopes, ctx, eq.body())?;
            Ok(Value::Content(Content::Equation {
                body: Box::new(body),
                block,
            }))
        }

        Expr::Math(math) => {
            // Math node isolado (fora de Equation) — produzir como sequence.
            let content = math::eval_math_content(scopes, ctx, math)?;
            Ok(Value::Content(content))
        }

        Expr::ModuleImport(i)  => modules::eval_module_import(i),
        Expr::ModuleInclude(i) => modules::eval_module_include(i, scopes, ctx, route, styles, show_rules, active_guards),

        // Passo 56 — referência cruzada: @nome → Content::Ref placeholder.
        Expr::Ref(ref_node) => {
            let name = ref_node.target().to_string();
            Ok(Value::Content(Content::Ref { target: Label(name) }))
        }

        // Passo 56 — label em contexto de código (raro); a associação retroactiva
        // acontece em eval_markup via SyntaxKind::Label. Aqui apenas ignoramos.
        Expr::Label(_) => Ok(Value::None),

        Expr::ShowRule(s) => rules::eval_show_rule(s, scopes, ctx, route, styles, show_rules, active_guards),

        // Passo 81 — array literal `(1fr, 1fr)` / `(10pt, auto, 1fr)`.
        // Necessário para o argumento `columns` de `grid()`.
        Expr::Array(arr) => {
            let mut items = Vec::new();
            for item in arr.items() {
                if let ArrayItem::Pos(expr) = item {
                    items.push(eval_expr(expr, scopes, ctx, route, styles, show_rules, active_guards)?);
                }
            }
            Ok(Value::Array(items))
        }

        // `(expr)` — parêntese de agrupamento. Expressão única dentro de
        // parênteses avalia para o valor da expressão. Passo 83.
        // (Um tuplo com um elemento requer a vírgula trailing: `(x,)`.)
        Expr::Parenthesized(paren) => eval_expr(paren.expr(), scopes, ctx, route, styles, show_rules, active_guards),

        // Passo 76 — literais numéricos com unidade (ex: 100pt, 1.5em).
        Expr::Numeric(num) => {
            use crate::entities::ast::expr::Unit;
            use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
            let (value, unit) = num.get();
            match unit {
                Unit::Pt      => Ok(Value::Length(Length { abs: Abs(value),            em: 0.0 })),
                Unit::Mm      => Ok(Value::Length(Length { abs: Abs(value * 2.8346),   em: 0.0 })),
                Unit::Cm      => Ok(Value::Length(Length { abs: Abs(value * 28.346),   em: 0.0 })),
                Unit::In      => Ok(Value::Length(Length { abs: Abs(value * 72.0),     em: 0.0 })),
                Unit::Em      => Ok(Value::Length(Length { abs: Abs(0.0),              em: value })),
                Unit::Deg     => Ok(Value::Angle(Angle::deg(value))),
                Unit::Rad     => Ok(Value::Angle(Angle::rad(value))),
                Unit::Percent => Ok(Value::Ratio(Ratio::from_percent(value))),
                Unit::Fr      => Ok(Value::Fraction(value)),
            }
        }

        // Fronteira deliberada — requer tipos não migrados (Content, Styles, etc.)
        _ => Ok(Value::None),
    }
}

/// Avalia o corpo de um nó de markup como Content.
fn eval_markup_body<'r>(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
    route: Tracked<'r, Route<'r>>,
    styles: &mut StyleChain,
    show_rules: &mut Arc<[ShowRule]>,
    active_guards: &mut Vec<RuleId>,
) -> SourceResult<Content> {
    match eval_markup(node, scopes, ctx, route, styles, show_rules, active_guards)? {
        Value::Content(c) => Ok(c),
        _                 => Ok(Content::Empty),
    }
}

// eval_args, apply_func, apply_closure extraídos para eval/closures.rs (Passo 96.1).

// Math eval extraído para eval/math.rs no Passo 96.1 (ADR-0037).

// eval_let extraído para eval/bindings.rs (Passo 96.1).

// apply_show_rules e intercept_content extraídos para eval/rules.rs (Passo 96.1).

/// Constrói a stdlib: `type`, `len`, `range`, `rgb`, `luma`, `str`, `int`, `float`, `figure`, `assert`, `upper`, `lower`, `replace`, `calc`.
///
/// Passo 64 (DEBT-16): `native_figure` migrada do interceptador em eval.rs para cá.
/// O avaliador deixa de conhecer o nome "figure" — desacoplamento total.
fn make_stdlib() -> Scope {
    use crate::rules::stdlib::{
        make_calc_module, native_align, native_assert, native_circle, native_ellipse,
        native_emph, native_figure, native_float, native_grid, native_heading,
        native_image, native_int, native_len, native_line,
        native_lower, native_luma, native_move, native_page, native_place, native_polygon,
        native_range, native_rect, native_replace, native_raw, native_rgb, native_rotate,
        native_scale, native_str, native_strong, native_type, native_upper,
    };
    let mut scope = Scope::new();
    scope.define("type",    Value::Func(Func::native("type",    native_type)));
    scope.define("len",     Value::Func(Func::native("len",     native_len)));
    scope.define("range",   Value::Func(Func::native("range",   native_range)));
    scope.define("rgb",     Value::Func(Func::native("rgb",     native_rgb)));
    scope.define("luma",    Value::Func(Func::native("luma",    native_luma)));
    scope.define("str",     Value::Func(Func::native("str",     native_str)));
    scope.define("int",     Value::Func(Func::native("int",     native_int)));
    scope.define("float",   Value::Func(Func::native("float",   native_float)));
    scope.define("heading",   Value::Func(Func::native("heading",   native_heading)));
    scope.define("strong",    Value::Func(Func::native("strong",    native_strong)));
    scope.define("emph",      Value::Func(Func::native("emph",      native_emph)));
    scope.define("raw",       Value::Func(Func::native("raw",       native_raw)));
    scope.define("figure",  Value::Func(Func::native("figure",  native_figure)));
    scope.define("image",   Value::Func(Func::native("image",   native_image)));
    scope.define("rect",    Value::Func(Func::native("rect",    native_rect)));
    scope.define("ellipse", Value::Func(Func::native("ellipse", native_ellipse)));
    scope.define("circle",  Value::Func(Func::native("circle",  native_circle)));
    scope.define("line",    Value::Func(Func::native("line",    native_line)));
    scope.define("polygon", Value::Func(Func::native("polygon", native_polygon)));
    scope.define("grid",    Value::Func(Func::native("grid",    native_grid)));
    scope.define("page",    Value::Func(Func::native("page",    native_page)));
    scope.define("move",    Value::Func(Func::native("move",    native_move)));
    scope.define("rotate",  Value::Func(Func::native("rotate",  native_rotate)));
    scope.define("scale",   Value::Func(Func::native("scale",   native_scale)));
    scope.define("align",   Value::Func(Func::native("align",   native_align)));
    scope.define("place",   Value::Func(Func::native("place",   native_place)));
    scope.define("assert",  Value::Func(Func::native("assert",  native_assert)));
    scope.define("upper",   Value::Func(Func::native("upper",   native_upper)));
    scope.define("lower",   Value::Func(Func::native("lower",   native_lower)));
    scope.define("replace", Value::Func(Func::native("replace", native_replace)));
    scope.define("calc",    make_calc_module());

    // Constantes de alinhamento (Passo 84.5, encerra DEBT-36).
    // Sintaxe preferida: `align(center, ...)`, `align(center + bottom, ...)`.
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    scope.define("left",    Value::Align(Align2D { h: Some(HAlign::Left),    v: None }));
    scope.define("center",  Value::Align(Align2D { h: Some(HAlign::Center),  v: None }));
    scope.define("right",   Value::Align(Align2D { h: Some(HAlign::Right),   v: None }));
    scope.define("top",     Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
    scope.define("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
    scope.define("bottom",  Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));

    scope
}

// ── Auxiliares para intercepção de counter(...).method() ──────────────────

/// Extrai o nome do contador de uma expressão `counter(key)`.
// extract_counter_key e eval_counter_method extraídos para eval/bindings.rs (Passo 96.1).

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use crate::rules::eval::tests::eval_for_test;
// Re-export para o módulo de tests (que usa `use super::*;`).
#[cfg(test)]
pub(crate) use crate::rules::eval::operators::{eval_binary_op, eval_unary_op};
