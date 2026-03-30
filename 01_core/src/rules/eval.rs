//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash f883240f
//! @layer L1
//! @updated 2026-03-28

use comemo::{Tracked, TrackedMut};
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::contracts::world::TrackedWorld;
use crate::entities::args::Args;
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::ast::code::{Conditional, ForLoop, LetBinding, LetBindingKind, WhileLoop};
use crate::entities::ast::expr::{Arg, BinOp, Expr, Param, Pattern, UnOp};
use crate::entities::syntax_kind::SyntaxKind;
use crate::entities::func::{ClosureParam, ClosureRepr, Func, FuncRepr};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;
use crate::entities::world_types::{Route, Routines, Sink, Traced};
use crate::rules::scopes::Scopes;

/// Profundidade máxima de chamada de função.
///
/// O original usa 80 via `Route::MAX_CALL_DEPTH`. O cristalino usa 200 —
/// limite conservador que previne stack overflow em Rust (sem Route ligado).
/// Será ajustado quando Route migrar (Passo 18+).
const MAX_CALL_DEPTH: usize = 200;

/// Contexto de execução partilhado durante eval().
///
/// Introduzido no Passo 17 para suportar o limite de profundidade
/// de chamada. Substituirá parâmetros avulsos à medida que crescer
/// (Passo 18+).
pub(crate) struct EvalContext<'w> {
    pub world: Tracked<'w, dyn TrackedWorld + 'w>,
    pub depth: usize,
}

impl<'w> EvalContext<'w> {
    pub fn new(world: Tracked<'w, dyn TrackedWorld + 'w>) -> Self {
        Self { world, depth: 0 }
    }

    /// Entra numa chamada de função — retorna Err se profundidade excedida.
    pub fn enter_call(&mut self, span: Span) -> SourceResult<()> {
        self.depth += 1;
        if self.depth > MAX_CALL_DEPTH {
            Err(vec![SourceDiagnostic::error(
                span,
                format!("profundidade máxima de chamada ({MAX_CALL_DEPTH}) excedida"),
            )])
        } else {
            Ok(())
        }
    }

    pub fn leave_call(&mut self) {
        self.depth = self.depth.saturating_sub(1);
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
/// sempre via `TrackedWorld` (L1).
pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    let root = source.root();

    let mut ctx = EvalContext::new(world);

    let mut scopes = Scopes::new(None);
    // Stdlib como scope base — type, len, range visíveis em todo o documento
    let stdlib = make_stdlib();
    for (name, binding) in stdlib.iter() {
        scopes.define(name, binding.value().clone());
    }
    scopes.enter();  // âmbito do módulo

    let content_val = eval_markup(root, &mut scopes, &mut ctx)?;

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

fn eval_markup(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    let mut parts: Vec<Content> = Vec::new();

    for child in node.children() {
        match child.kind() {
            SyntaxKind::Text => parts.push(Content::text(child.text().as_str())),
            SyntaxKind::Space | SyntaxKind::Parbreak => parts.push(Content::Space),
            k if k.is_trivia() => continue,
            _ => {
                if let Some(expr) = Expr::from_untyped(child) {
                    match eval_expr(expr, scopes, ctx)? {
                        Value::Content(c) => parts.push(c),
                        Value::Str(s)     => parts.push(Content::text(s.as_str())),
                        Value::None       => {}
                        _                 => {}
                    }
                }
            }
        }
    }

    Ok(Value::Content(Content::sequence(parts)))
}

fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Int(node)   => Ok(Value::Int(node.get())),
        Expr::Float(node) => Ok(Value::Float(node.get())),
        Expr::Str(node)   => Ok(Value::Str(EcoString::from(node.get()))),
        Expr::Bool(node)  => Ok(Value::Bool(node.get())),
        Expr::None(_)     => Ok(Value::None),

        Expr::Ident(ident) => {
            let name = ident.as_str();
            scopes.get(name)
                .cloned()
                .ok_or_else(|| vec![SourceDiagnostic::error(
                    ident.span(),
                    format!("unknown variable: {name}"),
                )])
        }

        Expr::LetBinding(binding) => eval_let(binding, scopes, ctx),

        Expr::CodeBlock(code_block) => {
            // Bloco de código — avaliar exprs sequencialmente
            // Code::exprs() já filtra trivia
            let mut last = Value::None;
            for expr in code_block.body().exprs() {
                last = eval_expr(expr, scopes, ctx)?;
            }
            Ok(last)
        }

        Expr::Binary(binary) => {
            let lhs = eval_expr(binary.lhs(), scopes, ctx)?;
            let rhs = eval_expr(binary.rhs(), scopes, ctx)?;
            eval_binary_op(binary.op(), lhs, rhs)
                .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
        }

        Expr::Unary(unary) => {
            let operand = eval_expr(unary.expr(), scopes, ctx)?;
            eval_unary_op(unary.op(), operand)
                .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
        }

        Expr::Conditional(cond) => eval_conditional(cond, scopes, ctx),
        Expr::WhileLoop(loop_expr) => eval_while(loop_expr, scopes, ctx),
        Expr::ForLoop(loop_expr) => eval_for(loop_expr, scopes, ctx),

        Expr::Closure(closure_expr) => {
            // Captura eager — snapshot do scope actual no momento da definição.
            // Semântica: valor imutável; shadowing posterior no scope pai não afecta
            // a closure. Divergência do original (comemo lazy) — registada em DEBT.md.
            let mut captured = IndexMap::with_hasher(FxBuildHasher::default());
            for (name, value) in scopes.iter_all() {
                captured.insert(name.to_string(), value.clone());
            }

            // Extrair parâmetros — Param::Pos(Pattern::Normal(Ident)) e Param::Named
            let params: SourceResult<Vec<ClosureParam>> = closure_expr.params()
                .children()
                .filter_map(|param| match param {
                    Param::Pos(Pattern::Normal(Expr::Ident(ident))) => {
                        Some(Ok(ClosureParam { name: ident.as_str().to_string(), default: None }))
                    }
                    Param::Named(named) => {
                        let name = named.name().as_str().to_string();
                        Some(eval_expr(named.expr(), scopes, ctx)
                            .map(|v| ClosureParam { name, default: Some(v) }))
                    }
                    _ => None,  // Spread, Placeholder, Destructuring — adiado
                })
                .collect();
            let params = params?;

            // Body: SyntaxNode clone O(1) via Arc interno
            let body = closure_expr.body().to_untyped().clone();

            Ok(Value::Func(Func::closure(ClosureRepr { name: None, params, body, captured })))
        }

        Expr::FuncCall(call) => {
            let callee = eval_expr(call.callee(), scopes, ctx)?;
            let args = eval_args(call.args(), scopes, ctx)?;

            match callee {
                Value::Func(func) => apply_func(func, args, ctx),
                other => Err(vec![SourceDiagnostic::error(
                    call.callee().span(),
                    format!("não é possível chamar {}", other.type_name()),
                )]),
            }
        }

        Expr::Strong(strong) => {
            let body = eval_markup_body(strong.body().to_untyped(), scopes, ctx)?;
            Ok(Value::Content(Content::strong(body)))
        }

        Expr::Emph(emph) => {
            let body = eval_markup_body(emph.body().to_untyped(), scopes, ctx)?;
            Ok(Value::Content(Content::emph(body)))
        }

        Expr::Heading(heading) => {
            let level = heading.depth().get() as u8;
            let body  = eval_markup_body(heading.body().to_untyped(), scopes, ctx)?;
            Ok(Value::Content(Content::heading(level, body)))
        }

        Expr::Raw(raw) => {
            // Raw não tem método text() — raw.lines() itera nós Text (SyntaxKind::Text)
            // tanto para inline como para block. RawTrimmed são apenas whitespace/newlines.
            let text: EcoString = raw.lines()
                .map(|l| l.get())
                .collect::<Vec<_>>()
                .join("\n")
                .into();
            let lang  = raw.lang().map(|l| EcoString::from(l.get()));
            let block = raw.block();
            Ok(Value::Content(Content::raw(text, lang, block)))
        }

        Expr::Link(link) => {
            let url = link.get().to_string();
            Ok(Value::Content(Content::link(url.clone(), Content::text(url))))
        }

        Expr::ListItem(item) => {
            let body = eval_markup_body(item.body().to_untyped(), scopes, ctx)?;
            Ok(Value::Content(Content::list_item(body)))
        }

        Expr::EnumItem(item) => {
            let number = item.number().map(|n| n as u32);
            let body   = eval_markup_body(item.body().to_untyped(), scopes, ctx)?;
            Ok(Value::Content(Content::enum_item(number, body)))
        }

        Expr::FieldAccess(access) => {
            let target = eval_expr(access.target(), scopes, ctx)?;
            let field  = access.field().as_str().to_string();
            match target {
                Value::Dict(d) => d.get(field.as_str())
                    .cloned()
                    .ok_or_else(|| vec![SourceDiagnostic::error(
                        access.span(),
                        format!("campo '{field}' não existe"),
                    )]),
                other => Err(vec![SourceDiagnostic::error(
                    access.span(),
                    format!("field access não suportado em {}", other.type_name()),
                )]),
            }
        }

        // Fronteira deliberada — requer tipos não migrados (Content, Styles, etc.)
        _ => Ok(Value::None),
    }
}

/// Avalia o corpo de um nó de markup como Content.
fn eval_markup_body(
    node: &SyntaxNode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Content> {
    match eval_markup(node, scopes, ctx)? {
        Value::Content(c) => Ok(c),
        _                 => Ok(Content::Empty),
    }
}

/// Avalia a lista de argumentos de uma chamada de função.
///
/// Posicionais são avaliados em ordem; named args são avaliados e indexados
/// por nome. Spread ignorado (fronteira deliberada, adiado).
fn eval_args(
    args_node: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Args> {
    let mut items = Vec::new();
    let mut named: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    for arg in args_node.items() {
        match arg {
            Arg::Pos(expr) => items.push(eval_expr(expr, scopes, ctx)?),
            Arg::Named(name_expr) => {
                named.insert(
                    name_expr.name().as_str().into(),
                    eval_expr(name_expr.expr(), scopes, ctx)?,
                );
            }
            Arg::Spread(_) => {}  // fronteira deliberada
        }
    }
    Ok(Args { items, named })
}

/// Avalia uma operação binária com semântica Typst.
///
/// Semântica confirmada com `lab/typst-original/crates/typst-library/src/foundations/ops.rs`:
/// - Int/Int → Float (não truncamento): `5/2 = 2.5`
/// - Int overflow → Err (checked_add/sub/mul/neg, como no original)
/// - Float: IEEE 754 propagado silenciosamente (sem guarda NaN/Inf)
/// - Divisão por zero → Err explícito
/// - `Int == Float` — ADR-0025 Opção B: coerção em eval_binary_op,
///   derive(PartialEq) mantido para Rust
pub(crate) fn eval_binary_op(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    // Divisão por zero — verificar antes do match (como no original)
    if matches!(op, BinOp::Div) {
        match &rhs {
            Value::Int(0)   => return Err("cannot divide by zero".into()),
            Value::Float(f) if *f == 0.0 => return Err("cannot divide by zero".into()),
            _ => {}
        }
    }

    match (op, lhs, rhs) {
        // ── Adição ──────────────────────────────────────────────────────────
        (BinOp::Add, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_add(b).ok_or("number too large")?)),
        (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (BinOp::Add, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a + b as f64)),
        (BinOp::Add, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
        (BinOp::Add, Value::Str(a),   Value::Str(b))   => Ok(Value::Str(a + b.as_str())),

        // ── Subtracção ──────────────────────────────────────────────────────
        (BinOp::Sub, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_sub(b).ok_or("number too large")?)),
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Sub, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a - b as f64)),
        (BinOp::Sub, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 - b)),

        // ── Multiplicação ────────────────────────────────────────────────────
        (BinOp::Mul, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_mul(b).ok_or("number too large")?)),
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Mul, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a * b as f64)),
        (BinOp::Mul, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 * b)),

        // ── Divisão — Int/Int → Float (semântica Typst, não truncamento) ────
        (BinOp::Div, Value::Int(a),   Value::Int(b))   => Ok(Value::Float(a as f64 / b as f64)),
        (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        (BinOp::Div, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a / b as f64)),
        (BinOp::Div, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 / b)),

        // ── Comparações ──────────────────────────────────────────────────────
        // ADR-0025: coerção Int↔Float em Eq/Neq e ordenação, como no original.
        // derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados —
        // mas eval_binary_op replica a semântica do Typst (1 == 1.0 → true).
        (BinOp::Eq,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
        (BinOp::Eq,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a == (b as f64))),
        (BinOp::Neq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
        (BinOp::Neq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a != (b as f64))),
        (BinOp::Eq,  a, b) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, a, b) => Ok(Value::Bool(a != b)),
        // Ordenação: coerção Int↔Float confirmada no original (ops::compare)
        (BinOp::Lt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
        (BinOp::Lt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a < (b as f64))),
        (BinOp::Leq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
        (BinOp::Leq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a <= (b as f64))),
        (BinOp::Gt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
        (BinOp::Gt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a > (b as f64))),
        (BinOp::Geq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
        (BinOp::Geq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a >= (b as f64))),

        // ── Lógica booleana ──────────────────────────────────────────────────
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (BinOp::Or,  Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),

        // ── Tipos tipográficos (ADR-0028, ADR-0029) ──────────────────────────
        // Length + Length: sempre válido (abs + abs, em + em, mistos representáveis)
        (BinOp::Add, Value::Length(a), Value::Length(b)) =>
            Ok(Value::Length(a + b)),
        // Ratio * Int ou Int * Ratio → escala o rácio
        (BinOp::Mul, Value::Ratio(r), Value::Int(n)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * n as f64))),
        (BinOp::Mul, Value::Int(n), Value::Ratio(r)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(n as f64 * r.get()))),

        // ── Fronteira — tipos não migrados ou combinações inválidas ──────────
        (op, lhs, rhs) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op, lhs.type_name(), rhs.type_name()
        )),
    }
}

/// Avalia uma operação unária com semântica Typst.
///
/// Int negation usa `checked_neg` para retornar Err em overflow
/// (mesma política do original).
pub(crate) fn eval_unary_op(op: UnOp, operand: Value) -> Result<Value, String> {
    match (op, operand) {
        (UnOp::Neg, Value::Int(i))   =>
            Ok(Value::Int(i.checked_neg().ok_or("number too large")?)),
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        (UnOp::Not, Value::Bool(b))  => Ok(Value::Bool(!b)),
        (UnOp::Pos, Value::Int(i))   => Ok(Value::Int(i)),
        (UnOp::Pos, Value::Float(f)) => Ok(Value::Float(f)),
        (op, operand) => Err(format!(
            "cannot apply {:?} to {}",
            op, operand.type_name()
        )),
    }
}

fn eval_conditional(
    cond: Conditional<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    let condition = eval_expr(cond.condition(), scopes, ctx)?;
    match condition {
        Value::Bool(true) => eval_expr(cond.if_body(), scopes, ctx),
        Value::Bool(false) => match cond.else_body() {
            Some(else_body) => eval_expr(else_body, scopes, ctx),
            None            => Ok(Value::None),
        },
        other => Err(vec![SourceDiagnostic::error(
            cond.condition().span(),
            format!("condição if deve ser bool, encontrado {}", other.type_name()),
        )]),
    }
}

fn eval_while(
    loop_expr: WhileLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    // Limite de segurança: sem comemo+Route não há detecção de ciclos.
    // O original usa memoização incremental e Route para prevenir loops infinitos.
    // Será removido no Passo 16+ quando esses mecanismos forem ligados.
    const MAX_ITER: usize = 10_000;
    let mut count = 0;

    loop {
        if count >= MAX_ITER {
            return Err(vec![SourceDiagnostic::error(
                loop_expr.span(),
                format!("loop excedeu {MAX_ITER} iterações (limite de segurança)"),
            )]);
        }

        let cond = eval_expr(loop_expr.condition(), scopes, ctx)?;
        match cond {
            Value::Bool(true) => {
                scopes.enter();
                eval_expr(loop_expr.body(), scopes, ctx)?;
                scopes.exit();
                count += 1;
            }
            Value::Bool(false) => break,
            other => return Err(vec![SourceDiagnostic::error(
                loop_expr.condition().span(),
                format!("condição while deve ser bool, encontrado {}", other.type_name()),
            )]),
        }
    }
    Ok(Value::None)
}

fn eval_for(
    loop_expr: ForLoop<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    let iterable = eval_expr(loop_expr.iterable(), scopes, ctx)?;
    match iterable {
        Value::Array(items) => {
            let bindings = loop_expr.pattern().bindings();
            let name = bindings.first()
                .map(|ident| ident.as_str().to_string())
                .unwrap_or_default();
            for item in items {
                scopes.enter();
                scopes.define(name.as_str(), item);
                eval_expr(loop_expr.body(), scopes, ctx)?;
                scopes.exit();
            }
            Ok(Value::None)
        }
        // `()` em Typst avalia para None via fronteira deliberada (não há parsing
        // de array literal neste passo). Tratar None como iterável vazio.
        Value::None => Ok(Value::None),
        other => Err(vec![SourceDiagnostic::error(
            loop_expr.iterable().span(),
            format!("não é possível iterar sobre {}", other.type_name()),
        )]),
    }
}

/// Aplica uma função (closure ou native) aos args dados.
fn apply_func(
    func: Func,
    args: Args,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => apply_closure(closure, &func, args, ctx),
        FuncRepr::Native(native)   => (native.call)(&args.items),
    }
}

/// Aplica uma closure: cria scope isolado, injeta capturadas + auto-ref + params.
///
/// **Auto-injecção para recursão**: se a closure tem nome (preenchido por eval_let),
/// injeta `Value::Func(func.clone())` no call_scope sob esse nome. O Arc é destruído
/// quando o call_scope sai do scope — sem ciclo permanente.
fn apply_closure(
    closure: &ClosureRepr,
    func: &Func,
    args: Args,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    ctx.enter_call(closure.body.span())?;

    let mut call_scopes = Scopes::new(None);

    // Variáveis capturadas — eager snapshot do momento da definição da closure
    for (name, value) in &closure.captured {
        call_scopes.define(name.as_str(), value.clone());
    }

    // Auto-injecção para recursão — referência vive apenas durante esta chamada
    if let Some(ref name) = closure.name {
        call_scopes.define(name.clone(), Value::Func(func.clone()));
    }

    // Bind parâmetros: named args têm prioridade sobre posicionais;
    // se nenhum, usar default; se não há default, usar None.
    let mut pos_idx = 0;
    for param in closure.params.iter() {
        let val = if let Some(v) = args.named.get(param.name.as_str()) {
            v.clone()
        } else if let Some(v) = args.items.get(pos_idx) {
            pos_idx += 1;
            v.clone()
        } else {
            param.default.clone().unwrap_or(Value::None)
        };
        call_scopes.define(param.name.as_str(), val);
    }

    // Avaliar o body com o scope da chamada
    let result = if let Some(body_expr) = Expr::from_untyped(&closure.body) {
        eval_expr(body_expr, &mut call_scopes, ctx)
    } else {
        Ok(Value::None)
    };

    ctx.leave_call();
    result
}

fn eval_let(
    binding: LetBinding<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    let mut value = match binding.init() {
        Some(init) => eval_expr(init, scopes, ctx)?,
        None => Value::None,
    };

    if let LetBindingKind::Normal(pattern) = binding.kind() {
        // Binding simples: #let x = ... → Pattern::Normal(Expr::Ident(x))
        let bindings = pattern.bindings();
        if let Some(ident) = bindings.into_iter().next() {
            let name = ident.as_str().to_string();
            // Se a closure ainda não tem nome, dar-lhe o nome da binding (para recursão)
            if let Value::Func(ref mut func) = value {
                func.set_name(name.clone());
            }
            scopes.define(name, value);
        }
    }

    Ok(Value::None)
}

/// Constrói a stdlib: `type`, `len`, `range`, `rgb`, `luma`, `str`, `int`, `float`, `calc`.
fn make_stdlib() -> Scope {
    use crate::rules::stdlib::{
        make_calc_module, native_float, native_int, native_len,
        native_luma, native_range, native_rgb, native_str, native_type,
    };
    let mut scope = Scope::new();
    scope.define("type",  Value::Func(Func::native("type",  native_type)));
    scope.define("len",   Value::Func(Func::native("len",   native_len)));
    scope.define("range", Value::Func(Func::native("range", native_range)));
    scope.define("rgb",   Value::Func(Func::native("rgb",   native_rgb)));
    scope.define("luma",  Value::Func(Func::native("luma",  native_luma)));
    scope.define("str",   Value::Func(Func::native("str",   native_str)));
    scope.define("int",   Value::Func(Func::native("int",   native_int)));
    scope.define("float", Value::Func(Func::native("float", native_float)));
    scope.define("calc",  make_calc_module());
    scope
}

#[cfg(test)]
pub(crate) fn eval_for_test<W: TrackedWorld>(
    world: &W,
    source: &Source,
) -> SourceResult<Module> {
    use comemo::Track;
    let routines = Routines::new();
    let traced   = Traced::new();
    let mut sink = Sink::new();
    let route    = Route::new();

    // Coerce &W → &dyn TrackedWorld para obter Tracked<dyn TrackedWorld>
    // #[comemo::track] em TrackedWorld gera impl Track for dyn TrackedWorld
    let dyn_world: &dyn TrackedWorld = world;
    eval(&routines, dyn_world.track(), traced.track(), sink.track_mut(), route.track(), source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::scope::Scope;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
    use crate::rules::scopes::Scopes;
    use std::num::NonZeroU16;

    // ── MockWorld para integração com eval() ─────────────────────────────────

    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }

    impl MockWorld {
        fn new(text: &str) -> Self {
            let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
            Self {
                library: Library::new(),
                book:    FontBook::new(),
                source:  Source::new(id, text.to_string()),
            }
        }
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.source.id() }
        fn source(&self, _id: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
        fn file(&self, _: FileId)     -> FileResult<Bytes>  { Err(FileError::NotFound) }
        fn font(&self, _: usize)      -> Option<Font>       { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    // ── Testes via Scope directamente ────────────────────────────────────────

    #[test]
    fn scope_define_via_value_real() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));
        scope.define("s", Value::Str("hello".into()));
        assert_eq!(scope.get("x"), Some(&Value::Int(42)));
        assert_eq!(scope.get("s"), Some(&Value::Str("hello".into())));
    }

    #[test]
    fn scopes_lookup_em_pilha() {
        let mut scopes = Scopes::new(None);
        scopes.enter();
        scopes.define("x", Value::Int(1));
        scopes.enter();
        scopes.define("y", Value::Int(2));
        assert_eq!(scopes.get("x"), Some(&Value::Int(1)));
        assert_eq!(scopes.get("y"), Some(&Value::Int(2)));
        assert_eq!(scopes.get("z"), None);
    }

    #[test]
    fn scopes_exit_remove_local() {
        let mut scopes = Scopes::new(None);
        scopes.enter();
        scopes.define("global", Value::Bool(true));
        scopes.enter();
        scopes.define("local", Value::Int(99));
        scopes.exit();
        assert!(scopes.get("local").is_none());
        assert!(scopes.get("global").is_some());
    }

    // ── Testes de parse/AST ───────────────────────────────────────────────────

    #[test]
    fn ast_int_literal_parseable() {
        let source = Source::detached("#let x = 42");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn ast_str_literal_parseable() {
        let source = Source::detached("#let s = \"hello\"");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn ast_bool_literal_parseable() {
        let source = Source::detached("#let b = true");
        assert!(!source.root().erroneous());
    }

    #[test]
    fn nome_modulo_deriva_de_file_id() {
        let id = FileId::from_raw(NonZeroU16::new(7).unwrap());
        let source = Source::new(id, "#let x = 1".to_string());
        let name = source.id().into_raw().get().to_string();
        assert_eq!(name, "7");
    }

    // ── Testes de integração via eval_for_test ───────────────────────────────

    #[test]
    fn eval_let_int_via_world() {
        let world = MockWorld::new("#let x = 42");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source)
            .expect("eval não deve falhar em input válido");
        assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn eval_multiplos_bindings_via_world() {
        let world = MockWorld::new("#let a = 1\n#let b = true\n#let c = \"x\"");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert_eq!(module.scope().get("a"), Some(&Value::Int(1)));
        assert_eq!(module.scope().get("b"), Some(&Value::Bool(true)));
        assert_eq!(module.scope().get("c"), Some(&Value::Str("x".into())));
    }

    #[test]
    fn eval_texto_puro_scope_vazio() {
        let world = MockWorld::new("Apenas texto Typst.");
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        assert!(module.scope().is_empty());
    }

    // ── Testes de control flow ────────────────────────────────────────────────

    #[test]
    fn if_true_branch() {
        let world = MockWorld::new("#let x = if true { 1 } else { 2 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn if_false_branch() {
        let world = MockWorld::new("#let x = if false { 1 } else { 2 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
    }

    #[test]
    fn if_sem_else_retorna_none() {
        let world = MockWorld::new("#let x = if false { 1 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::None));
    }

    /// Prova de vida ADR-0025: if 1 == 1.0 { 42 } else { 0 } → 42
    #[test]
    fn prova_de_vida_adr_0025() {
        let world = MockWorld::new("#let x = if 1 == 1.0 { 42 } else { 0 }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(42)));
    }

    #[test]
    fn while_condicao_falsa_nao_executa() {
        let world = MockWorld::new("#while false { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    #[test]
    fn while_loop_infinito_retorna_err() {
        let world = MockWorld::new("#while true { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "loop infinito deve retornar Err, não bloquear");
        let err = result.unwrap_err();
        assert!(!err.is_empty());
        assert!(
            err[0].message.contains("iterações") || err[0].message.contains("limite"),
            "mensagem de erro deve mencionar limite: {:?}", err[0].message
        );
    }

    #[test]
    fn for_sobre_array_vazio_nao_executa() {
        let world = MockWorld::new("#let arr = ()\n#for x in arr { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    // ── Testes de paridade: eval_binary_op ───────────────────────────────────

    #[test]
    fn paridade_add_int() {
        assert_eq!(eval_binary_op(BinOp::Add, Value::Int(1), Value::Int(2)),
                   Ok(Value::Int(3)));
    }

    #[test]
    fn paridade_add_float() {
        assert_eq!(eval_binary_op(BinOp::Add, Value::Float(1.5), Value::Float(2.5)),
                   Ok(Value::Float(4.0)));
    }

    #[test]
    fn paridade_add_str() {
        assert_eq!(
            eval_binary_op(BinOp::Add, Value::Str("hello ".into()), Value::Str("world".into())),
            Ok(Value::Str("hello world".into()))
        );
    }

    #[test]
    fn paridade_sub_int() {
        assert_eq!(eval_binary_op(BinOp::Sub, Value::Int(5), Value::Int(3)),
                   Ok(Value::Int(2)));
    }

    #[test]
    fn paridade_mul_int() {
        assert_eq!(eval_binary_op(BinOp::Mul, Value::Int(3), Value::Int(4)),
                   Ok(Value::Int(12)));
    }

    #[test]
    fn paridade_div_int_int() {
        // Semântica Typst: 5 / 2 = 2.5 (float), confirmado com ops.rs
        assert_eq!(eval_binary_op(BinOp::Div, Value::Int(5), Value::Int(2)),
                   Ok(Value::Float(2.5)));
    }

    #[test]
    fn paridade_div_por_zero() {
        assert!(eval_binary_op(BinOp::Div, Value::Int(1),   Value::Int(0)).is_err());
        assert!(eval_binary_op(BinOp::Div, Value::Float(1.0), Value::Float(0.0)).is_err());
    }

    #[test]
    fn paridade_eq_int_int() {
        assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Int(1)),
                   Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Int(2)),
                   Ok(Value::Bool(false)));
    }

    #[test]
    fn dualidade_eq_typst_coerce() {
        // ADR-0025 Opção B: no motor Typst, 1 == 1.0 → true (coerção Int→f64)
        assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
                   Ok(Value::Bool(true)));
    }

    #[test]
    fn dualidade_eq_rust_sem_coerce() {
        // derive(PartialEq) em Rust: Value::Int(1) != Value::Float(1.0)
        // Vital para IndexMap, testes unitários de Value, e estruturas de dados.
        assert_ne!(Value::Int(1), Value::Float(1.0));
    }

    #[test]
    fn eq_tipos_radicalmente_distintos() {
        // Bool vs Int — sem coerção em nenhum sistema
        assert_eq!(eval_binary_op(BinOp::Eq, Value::Bool(true), Value::Int(1)),
                   Ok(Value::Bool(false)));
    }

    #[test]
    fn lt_int_float_coerce() {
        // Ordenação Int↔Float também coerce (confirmado em ops::compare)
        assert_eq!(eval_binary_op(BinOp::Lt, Value::Int(1), Value::Float(1.5)),
                   Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Gt, Value::Float(2.0), Value::Int(1)),
                   Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_neq() {
        assert_eq!(eval_binary_op(BinOp::Neq, Value::Int(1), Value::Int(2)),
                   Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Neq, Value::Int(1), Value::Int(1)),
                   Ok(Value::Bool(false)));
    }

    #[test]
    fn paridade_lt_gt() {
        assert_eq!(eval_binary_op(BinOp::Lt,  Value::Int(1), Value::Int(2)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Gt,  Value::Int(2), Value::Int(1)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Leq, Value::Int(2), Value::Int(2)), Ok(Value::Bool(true)));
        assert_eq!(eval_binary_op(BinOp::Geq, Value::Int(3), Value::Int(2)), Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_and_or() {
        assert_eq!(eval_binary_op(BinOp::And, Value::Bool(true),  Value::Bool(false)), Ok(Value::Bool(false)));
        assert_eq!(eval_binary_op(BinOp::Or,  Value::Bool(false), Value::Bool(true)),  Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_overflow_int_retorna_err() {
        // checked_add — overflow retorna Err, não panic
        let r = eval_binary_op(BinOp::Add, Value::Int(i64::MAX), Value::Int(1));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_nan_propagado() {
        // O original propaga NaN silenciosamente (Float(a / b) sem guarda)
        // 0.0 / 0.0 = NaN em IEEE 754 — mas a guarda de is_zero captura 0.0
        // portanto Float(0.0) / Float(0.0) → Err("cannot divide by zero")
        let r = eval_binary_op(BinOp::Div, Value::Float(0.0), Value::Float(0.0));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_tipo_invalido_retorna_err() {
        let r = eval_binary_op(BinOp::Add, Value::None, Value::Int(1));
        assert!(r.is_err());
    }

    // ── Testes de paridade: eval_unary_op ────────────────────────────────────

    #[test]
    fn paridade_not() {
        assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(true)),  Ok(Value::Bool(false)));
        assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(false)), Ok(Value::Bool(true)));
    }

    #[test]
    fn paridade_neg_int() {
        assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(5)),  Ok(Value::Int(-5)));
        assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(-3)), Ok(Value::Int(3)));
    }

    #[test]
    fn paridade_neg_overflow() {
        // i64::MIN.checked_neg() == None (overflow)
        let r = eval_unary_op(UnOp::Neg, Value::Int(i64::MIN));
        assert!(r.is_err());
    }

    #[test]
    fn paridade_pos_noop() {
        assert_eq!(eval_unary_op(UnOp::Pos, Value::Int(42)),    Ok(Value::Int(42)));
        assert_eq!(eval_unary_op(UnOp::Pos, Value::Float(1.5)), Ok(Value::Float(1.5)));
    }

    #[test]
    fn paridade_unary_tipo_invalido() {
        let r = eval_unary_op(UnOp::Not, Value::Int(1));
        assert!(r.is_err());
    }

    // ── Testes de Passo 16 — Closures e FuncCall ─────────────────────────────

    #[test]
    fn closure_cria_value_func() {
        let world = MockWorld::new("#let f = (x) => x + 1");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert!(matches!(m.scope().get("f"), Some(Value::Func(_))));
    }

    #[test]
    fn funcall_soma_dois_args() {
        let world = MockWorld::new(
            "#let add = (x, y) => x + y\n#let r = add(1, 2)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(3)));
    }

    #[test]
    fn funcall_arg_errado_retorna_err() {
        let world = MockWorld::new("#let r = 42(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }

    #[test]
    fn closure_default_param() {
        let world = MockWorld::new(
            "#let greet = (prefix: \"Hi\") => prefix\n#let r = greet()"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("Hi".into())));
    }

    /// Teste de Ouro: valida que eager capture é determinista e isolada.
    #[test]
    fn eager_capture_isolada_do_scope_pai() {
        let world = MockWorld::new(
            "#let x = 1\n\
             #let get_x = () => x\n\
             #let x = 2\n\
             #let r = get_x()"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(1)),
            "eager capture deve isolar a closure do shadowing posterior");
    }

    #[test]
    fn closure_scope_nao_vaza_para_chamador() {
        let world = MockWorld::new(
            "#let f = () => { let local = 99; local }\n\
             #let r = f()"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(99)));
        assert!(m.scope().get("local").is_none(),
            "variáveis locais da closure não devem vazar para o chamador");
    }

    #[test]
    fn closure_recursiva_nao_vaza_memoria() {
        let world = MockWorld::new(
            "#let fact = (n) => if n <= 0 { 1 } else { n }\n\
             #let r = fact(5)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(5)));
    }

    #[test]
    fn func_type_name() {
        use crate::entities::func::{ClosureRepr, Func};
        use crate::entities::source::Source;
        use indexmap::IndexMap;
        use rustc_hash::FxBuildHasher;
        let source = Source::detached("x");
        let body = source.root().clone();
        let f = Func::closure(ClosureRepr {
            name: None,
            params: vec![],
            body,
            captured: IndexMap::with_hasher(FxBuildHasher::default()),
        });
        assert_eq!(Value::Func(f).type_name(), "function");
    }

    // ── Testes de Passo 17 — Recursão ────────────────────────────────────────

    #[test]
    fn recursao_factorial() {
        let world = MockWorld::new(
            "#let fact = (n) => if n <= 1 { 1 } else { n * fact(n - 1) }\n\
             #let r = fact(5)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(120)));
    }

    #[test]
    fn recursao_fibonacci() {
        let world = MockWorld::new(
            "#let fib = (n) => if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }\n\
             #let r = fib(7)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Int(13)));
    }

    /// Teste de estabilidade — valida que recursão infinita não faz crash.
    /// Um Stack Overflow é inaceitável em servidor; Err é a falha correcta.
    #[test]
    fn recursao_infinita_retorna_err_sem_crash() {
        let world = MockWorld::new(
            "#let inf = (n) => inf(n + 1)\n\
             #let r = inf(0)"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let result = eval_for_test(&world, &src);
        assert!(result.is_err(), "recursão infinita deve Err, não crash");
        let msg = &result.unwrap_err()[0].message;
        assert!(
            msg.contains("profundidade") || msg.contains("depth"),
            "mensagem deve mencionar limite: {:?}", msg
        );
    }

    // ── Testes de Passo 17 — Stdlib ──────────────────────────────────────────

    #[test]
    fn stdlib_type_int() {
        let world = MockWorld::new("#let t = type(42)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("t"), Some(&Value::Str("int".into())));
    }

    #[test]
    fn stdlib_type_func() {
        let world = MockWorld::new("#let f = () => 1\n#let t = type(f)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("t"), Some(&Value::Str("function".into())));
    }

    #[test]
    fn stdlib_range_simples() {
        let world = MockWorld::new("#let r = range(3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"),
                   Some(&Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])));
    }

    #[test]
    fn stdlib_range_vazio_se_start_eq_end() {
        let world = MockWorld::new("#let r = range(3, 3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Array(vec![])));
    }

    #[test]
    fn for_com_range_integrado() {
        let world = MockWorld::new("#for i in range(3) { }");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_ok());
    }

    // ── Testes de Passo 17 — Named args ──────────────────────────────────────

    #[test]
    fn named_arg_simples() {
        let world = MockWorld::new(
            "#let greet = (prefix: \"Hi\", name) => prefix\n\
             #let r = greet(\"world\", prefix: \"Hello\")"
        );
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("r"), Some(&Value::Str("Hello".into())));
    }

    // ── Testes de Passo 18 — Pipeline Content ────────────────────────────────

    #[test]
    fn pipeline_completo_texto_simples() {
        use crate::entities::content::Content;
        use crate::rules::layout::layout;

        let world = MockWorld::new("Hello world");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();

        let content = module.content().expect("eval deve produzir Content");
        assert!(!content.is_empty());
        assert!(content.plain_text().contains("Hello"));
        assert!(content.plain_text().contains("world"));

        let result = layout(content);
        assert!(!result.plain_text().is_empty());
    }

    #[test]
    fn pipeline_interpolacao_variavel() {
        use crate::rules::layout::layout;

        let world = MockWorld::new("#let x = \"Mundo\"\nOlá #x");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();

        let content = module.content().expect("Content deve existir");
        let text = content.plain_text();
        assert!(text.contains("Olá"), "texto estático deve estar presente: {:?}", text);
        assert!(text.contains("Mundo"), "variável interpolada deve estar presente: {:?}", text);

        let result = layout(content);
        assert!(!result.plain_text().is_empty());
    }

    #[test]
    fn pipeline_documento_vazio() {
        let world = MockWorld::new("");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        if let Some(c) = module.content() {
            assert!(c.is_empty());
        }
        // Sem pânico — pipeline robusto para input vazio
    }

    #[test]
    fn pipeline_apenas_codigo_sem_markup() {
        let world = MockWorld::new("#let x = 42");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
        // Content pode ser vazio — é correcto
    }

    #[test]
    fn content_type_name() {
        use crate::entities::content::Content;
        let v = Value::Content(Content::text("hello"));
        assert_eq!(v.type_name(), "content");
    }

    // ── Testes de Passo 22 — Rich text ───────────────────────────────────────

    #[test]
    fn pipeline_rich_text_plain_text_correcto() {
        let world = MockWorld::new("Hello *bold* and _italic_");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let text = content.plain_text();
        assert!(text.contains("Hello"),  "plain_text deve ter Hello: {:?}", text);
        assert!(text.contains("bold"),   "plain_text deve ter bold: {:?}", text);
        assert!(text.contains("italic"), "plain_text deve ter italic: {:?}", text);
    }

    #[test]
    fn pipeline_heading_plain_text_correcto() {
        let world = MockWorld::new("= Introduction\nBody text");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let text = content.plain_text();
        assert!(text.contains("Introduction"), "plain_text deve ter Introduction: {:?}", text);
        assert!(text.contains("Body"),         "plain_text deve ter Body: {:?}", text);
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn pipeline_raw_inline() {
        let world = MockWorld::new("Use `cargo build` to compile");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("deve ter content").plain_text();
        assert!(text.contains("cargo") && text.contains("build"), "{:?}", text);
    }

    #[test]
    fn pipeline_lista_bullets() {
        let world = MockWorld::new("- item 1\n- item 2");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let text = module.content().expect("deve ter content").plain_text();
        assert!(text.contains("item 1") && text.contains("item 2"), "{:?}", text);
    }

    // ── Passo 25 — tipos tipográficos ────────────────────────────────────────

    #[test]
    fn pipeline_rgb_em_let() {
        use crate::entities::layout_types::Color;
        let world = MockWorld::new("#let c = rgb(255, 0, 0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("c"), Some(&Value::Color(Color::rgb(255, 0, 0))));
    }

    #[test]
    fn ratio_mul_int() {
        use crate::entities::layout_types::Ratio;
        let r = eval_binary_op(BinOp::Mul, Value::Ratio(Ratio(0.5)), Value::Int(2));
        assert_eq!(r, Ok(Value::Ratio(Ratio(1.0))));
    }

    #[test]
    fn length_add_pt_pt() {
        use crate::entities::layout_types::Length;
        let r = eval_binary_op(BinOp::Add, Value::Length(Length::pt(10.0)), Value::Length(Length::pt(5.0)));
        assert_eq!(r, Ok(Value::Length(Length::pt(15.0))));
    }

    #[test]
    fn length_add_mista_agora_funciona() {
        // ADR-0029: Length struct (abs + em) — soma mista é representável, não Err
        use crate::entities::layout_types::Length;
        let r = eval_binary_op(BinOp::Add, Value::Length(Length::pt(10.0)), Value::Length(Length::em(1.0)));
        let l = r.expect("soma mista deve ser Ok com estrutura vanilla");
        if let Value::Length(len) = l {
            assert_eq!(len.abs.to_pt(), 10.0);
            assert_eq!(len.em, 1.0);
        } else {
            panic!("esperado Value::Length");
        }
    }

    #[test]
    fn stdlib_type_color() {
        let world = MockWorld::new("#let t = type(rgb(0, 0, 0))");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("t"), Some(&Value::Str("color".into())));
    }

    // ── Passo 27 — str/int/float/calc pipeline ───────────────────────────────

    #[test]
    fn pipeline_str_conversao() {
        let world = MockWorld::new("#let s = str(42)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("s"), Some(&Value::Str("42".into())));
    }

    #[test]
    fn pipeline_int_de_str() {
        let world = MockWorld::new("#let n = int(\"99\")");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("n"), Some(&Value::Int(99)));
    }

    #[test]
    fn pipeline_float_de_int() {
        let world = MockWorld::new("#let f = float(3)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("f"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn pipeline_calc_abs() {
        let world = MockWorld::new("#let x = calc.abs(-5)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(5)));
    }

    #[test]
    fn pipeline_calc_pow() {
        let world = MockWorld::new("#let x = calc.pow(2, 8)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(256)));
    }

    #[test]
    fn pipeline_calc_sqrt() {
        let world = MockWorld::new("#let x = calc.sqrt(9.0)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Float(3.0)));
    }

    #[test]
    fn pipeline_calc_clamp() {
        let world = MockWorld::new("#let x = calc.clamp(15, 0, 10)");
        let src = World::source(&world, World::main(&world)).unwrap();
        let m = eval_for_test(&world, &src).unwrap();
        assert_eq!(m.scope().get("x"), Some(&Value::Int(10)));
    }

    #[test]
    fn pipeline_field_access_invalido_retorna_err() {
        let world = MockWorld::new("#let x = calc.inexistente(1)");
        let src = World::source(&world, World::main(&world)).unwrap();
        assert!(eval_for_test(&world, &src).is_err());
    }
}
