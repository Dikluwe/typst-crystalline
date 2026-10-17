from pathlib import Path
import json,gzip,re,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-result.json'))['functional_red'];read=lambda p:Path(p).read_text();write=lambda p,s:Path(p).write_text(s)
# Field lookup adapter keeps every AST anchor and gate, but accepts the resolved receiver.
p='01_core/src/compiler/eval/bindings/field_access/fields.rs';s=read(p);a='''    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str();''';z='''    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    eval_field_access_with_target(access, target, scopes, ctx, engine)
}

pub(in crate::compiler::eval) fn eval_field_access_with_target(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    target: Value,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;
    let field = access.field().as_str();''';assert s.count(a)==1;s=s.replace(a,z);# AstNode import now lives only in the lookup helper
s=s.replace('''    use crate::entities::ast::AstNode;

    let target = eval_expr''','''    let target = eval_expr''',1);write(p,s)
p='01_core/src/compiler/eval/bindings/field_access.rs';s=read(p).replace('check_module_field_feature, eval_field_access, eval_value_field_access,','check_module_field_feature, eval_field_access, eval_field_access_with_target, eval_value_field_access,',1);write(p,s)
p='01_core/src/compiler/eval/bindings/mod.rs';s=read(p).replace('check_module_field_feature, eval_field_access, eval_value_field_access,','check_module_field_feature, eval_field_access, eval_field_access_with_target, eval_value_field_access,',1).replace('use method_dispatch::{is_mutating_method, try_eval_mutating_method};','use method_dispatch::{is_mutating_method, try_eval_mutating_method, MutatingCall};',1);write(p,s)
# Mutating calls retain the exceptional arguments-before-place order and carry fallback data.
p='01_core/src/compiler/eval/bindings/method_dispatch.rs';s=read(p);start=s.index('/// **P717** — mirror de `maybe_resolve_mutating`');end=s.index('pub(in crate::compiler::eval) fn try_eval_mutating_method',start);s=s[:start]+'''/// Result of mutable-place dispatch, preserving already evaluated fallback data.
pub(in crate::compiler::eval) enum MutatingCall {
    Handled(Value),
    Fallback { target: Value, args: Args },
}

/// Arguments precede mutable-place resolution. Fallback transfers both values.
'''+s[end:];start=s.index('pub(in crate::compiler::eval) fn try_eval_mutating_method');end=s.index('/// **P717** — mirror de `call_method_mut`',start);t=s[start:end].replace('SourceResult<Option<Value>>','SourceResult<MutatingCall>',1).replace('call_method_mut(target, method.as_str(), args, span).map(Some)','call_method_mut(target, method.as_str(), args, span).map(MutatingCall::Handled)',1).replace('''        Value::Module(_)
        | Value::Func(_)
        | Value::Type(_)
        | Value::Symbol(_)
        | Value::Content(_) => Ok(None),''','''        target @ (Value::Module(_)
        | Value::Func(_)
        | Value::Type(_)
        | Value::Symbol(_)
        | Value::Content(_)) => Ok(MutatingCall::Fallback { target: target.clone(), args }),''');s=s[:start]+t+s[end:];write(p,s)
# Selector adapters receive the actual value instead of re-evaluating syntax.
p='01_core/src/compiler/eval/bindings/value_methods/selector.rs';s=read(p);assert s.count("target_expr: Expr<'_>")==3;s=s.replace("target_expr: Expr<'_>", 'target: &Value').replace('    let target = eval_expr(target_expr, scopes, ctx, engine)?;\n','').replace('value_to_query_selector(&target)','value_to_query_selector(target)');write(p,s)
# One private receiver/callee cache per call, shared by all interceptors.
p='01_core/src/compiler/eval/call_dispatch/call.rs';s=read(p);idx=s.index('pub(in crate::compiler::eval) fn eval_func_call');helpers='''pub(super) fn resolve_receiver<'a>(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    cached_receiver: &'a mut Option<Value>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'a Value> {
    if cached_receiver.is_none() {
        *cached_receiver = Some(eval_expr(access.target(), scopes, ctx, engine)?);
    }
    Ok(cached_receiver.as_ref().expect("receiver initialized"))
}

pub(super) fn resolve_callee<'a>(
    call: FuncCallNode<'_>,
    cached_receiver: &mut Option<Value>,
    resolved_callee: &'a mut Option<Value>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'a Value> {
    if resolved_callee.is_none() {
        let value = if let Expr::FieldAccess(access) = call.callee() {
            if cached_receiver.is_none() {
                resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;
            }
            bindings::eval_field_access_with_target(
                access, cached_receiver.take().expect("receiver initialized"), scopes, ctx, engine,
            )?
        } else {
            eval_expr(call.callee(), scopes, ctx, engine)?
        };
        *resolved_callee = Some(value);
    }
    Ok(resolved_callee.as_ref().expect("callee initialized"))
}

''';s=s[:idx]+helpers+s[idx:];s=s.replace('    let mut resolved_callee = None;','    let mut resolved_callee = None;\n    let mut cached_receiver = None;\n    let mut resolved_args = None;',1).replace('super::selector_intercepts::try_selector_call(call, scopes, ctx, engine)?','super::selector_intercepts::try_selector_call(call, scopes, ctx, engine, &mut cached_receiver)?',1)
a='''                if let Some(result) = bindings::try_eval_mutating_method(''';assert a in s;s=s.replace(a,'''                match bindings::try_eval_mutating_method(''',1);a='''                )? {
                    return Ok(result);
                }
            }
        }

        let mut cached_receiver = None;''';z='''                )? {
                    bindings::MutatingCall::Handled(result) => return Ok(result),
                    bindings::MutatingCall::Fallback { target, args } => {
                        cached_receiver = Some(target);
                        resolved_args = Some(args);
                    }
                }
            }
        }
''';assert a in s;s=s.replace(a,z,1);s=s.replace('super::collection_methods::try_collection_or_with(', 'super::collection_methods::try_collection_method(',1).replace('''            cached_receiver,
        )?''','''            &mut cached_receiver,
        )?''',1).replace('super::contextual_calls::try_contextual_call(call, scopes, ctx, engine)?','super::contextual_calls::try_contextual_call(call, scopes, ctx, engine, &mut cached_receiver, &mut resolved_callee)?',1).replace('super::content_methods::try_content_methods(call, scopes, ctx, engine)?','super::content_methods::try_content_methods(call, scopes, ctx, engine, &mut cached_receiver)?',1)
a='''            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Some(err) = bindings::field_callee_error(&target, access) {
                return Err(err);
            }
        }
    }
    super::invoke::invoke_resolved_call(resolved_callee, call, scopes, ctx, engine)''';z='''            if resolved_callee.is_none() {
                let target = resolve_receiver(access, &mut cached_receiver, scopes, ctx, engine)?;
                if let Some(err) = bindings::field_callee_error(target, access) {
                    return Err(err);
                }
                resolve_callee(call, &mut cached_receiver, &mut resolved_callee, scopes, ctx, engine)?;
            }
        }
    }
    super::invoke::invoke_resolved_call(resolved_callee, resolved_args, call, scopes, ctx, engine)''';assert a in s;s=s.replace(a,z,1);write(p,s)
# Shared receiver borrow in instance/content/selector methods, no clone of a collection.
p='01_core/src/compiler/eval/call_dispatch/instance_methods.rs';s=read(p).replace('let target = eval_expr(access.target(), scopes, ctx, engine)?;','let target = super::call::resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;',1).replace('match &target {','match target {',1).replace('        *cached_receiver = Some(target);\n','');write(p,s)
p='01_core/src/compiler/eval/call_dispatch/selector_intercepts.rs';s=read(p).replace('''    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>>''','''    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>>''',1).replace('''                access.target(),''','''                super::call::resolve_receiver(access, cached_receiver, scopes, ctx, engine)?,''');write(p,s)
p='01_core/src/compiler/eval/call_dispatch/content_methods.rs';s=read(p).replace('''    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>>''','''    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>>''',1).replace('let target = eval_expr(access.target(), scopes, ctx, engine)?;', 'let target = super::call::resolve_receiver(access, cached_receiver, scopes, ctx, engine)?;').replace('eval_location_method(loc, method','eval_location_method(*loc, method').replace('bindings::eval_content_method(&c, method','bindings::eval_content_method(c, method').replace('''                    &c,
                    loc,''','''                    c,
                    *loc,''').replace('let content = match &target {','let content = match target {');write(p,s)
p='01_core/src/compiler/eval/call_dispatch/contextual_calls.rs';s=read(p).replace('''    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>>''','''    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
    resolved_callee: &mut Option<Value>,
) -> SourceResult<Option<Value>>''',1).replace('let target = eval_expr(call.callee(), scopes, ctx, engine)?;', 'let target = super::call::resolve_callee(call, cached_receiver, resolved_callee, scopes, ctx, engine)?;').replace('if let Value::Func(ref f) = target','if let Value::Func(f) = target');write(p,s)
# Deferred, single static collection dispatch table; unknown methods return ownership.
p='01_core/src/compiler/stdlib/collections.rs';s=read(p).replace('use self::bytes::try_dispatch_bytes_method;','#[cfg(test)]\nuse self::bytes::try_dispatch_bytes_method;').replace('#[cfg(test)]\nuse self::bytes::{bytes_at, bytes_len, bytes_slice};','use self::bytes::{bytes_at, bytes_len, bytes_slice};');start=s.index('pub(crate) fn try_dispatch_collection_method(');body=s.index('    match target {',start);end=s.index('\n}\n',body)+2;header=s[start:body];table=s[body:end-2]
# Inline the thin byte-name decoder only; algorithms stay in the bytes owner.
table=table.replace('Value::Bytes(bytes) => try_dispatch_bytes_method(bytes, method, args, call_spans),','''Value::Bytes(bytes) => match method {
            "len" => Some(bytes_len(bytes, args, call_spans)),
            "at" => Some(bytes_at(bytes, args, call_spans)),
            "slice" => Some(bytes_slice(bytes, args, call_spans)),
            _ => None,
        },''')
# Rewrite Some(expression) without parsing or changing its expression body.
def wrap_some(text):
 positions=[];i=0
 while True:
  i=text.find('Some(',i)
  if i<0:break
  depth=1;j=i+5;quoted=False;escape=False
  while depth:
   c=text[j]
   if quoted:
    if escape:escape=False
    elif c=='\\':escape=True
    elif c=='"':quoted=False
   elif c=='"':quoted=True
   elif c=='(':depth+=1
   elif c==')':depth-=1
   j+=1
  positions.append((i,j,text[i+5:j-1]));i=j
 for i,j,body in reversed(positions):text=text[:i]+'bind_args!(args, '+body+')'+text[j:]
 return text
# Per-family open-name fallbacks reconstruct by move, preserving the receiver.
for variant,var in [('Array','arr'),('Dict','dict'),('Str','s'),('Bytes','bytes'),('Args','arguments')]:
 begin=table.index('Value::'+variant+'(');finish=table.index('\n        },',begin)+11;segment=table[begin:finish];segment=segment.replace('_ => None,',f'_ => CollectionDispatch::Unhandled(Value::{variant}({var})),');table=table[:begin]+segment+table[finish:]
table=wrap_some(table);assert table.count('=> None')==1;table=table.replace('=> None,','=> CollectionDispatch::Unhandled(target),',1)
wrapper=header+'''    match try_dispatch_collection_method_with(
        target, method, |_, _, _| Ok(args), call_spans, scopes, ctx, engine,
    ) {
        CollectionDispatch::Handled(result) => Some(result),
        CollectionDispatch::Unhandled(_) => None,
    }
}

pub(crate) enum CollectionDispatch {
    Handled(SourceResult<Value>),
    Unhandled(Value),
}

pub(crate) fn try_dispatch_collection_method_with(
    target: Value,
    method: &str,
    evaluate_args: impl FnOnce(&mut Scopes<'_>, &mut EvalContext, &mut Engine<'_>) -> SourceResult<Args>,
    call_spans: Option<&CollectionCallSpans>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> CollectionDispatch {
    macro_rules! bind_args {
        ($args:ident, $body:expr) => {{
            let $args = match evaluate_args(scopes, ctx, engine) {
                Ok(args) => args,
                Err(errors) => return CollectionDispatch::Handled(Err(errors)),
            };
            CollectionDispatch::Handled($body)
        }};
    }
'''+table+'\n}';s=s[:start]+wrapper+s[end:];write(p,s)
p='01_core/src/compiler/stdlib/collections/bytes.rs';s=read(p).replace('pub(super) fn try_dispatch_bytes_method(', '#[cfg(test)]\npub(super) fn try_dispatch_bytes_method(',1);write(p,s)
p='01_core/src/compiler/stdlib/mod.rs';s=read(p).replace('collection_type_field, try_dispatch_collection_method, CollectionCallSpans,','collection_type_field, try_dispatch_collection_method, try_dispatch_collection_method_with, CollectionCallSpans, CollectionDispatch,',1);write(p,s)
p='01_core/src/compiler/eval/call_dispatch.rs';s=read(p).replace('    try_dispatch_collection_method,','    try_dispatch_collection_method_with,',1);write(p,s)
p='01_core/src/compiler/eval/call_dispatch/collection_methods.rs';s=read(p).replace('try_collection_or_with','try_collection_method',1).replace('cached_receiver: Option<Value>','cached_receiver: &mut Option<Value>',1).replace('let target = match cached_receiver {','let target = match cached_receiver.take() {',1);s=s.replace('''        let args = eval_args(call.args(), scopes, ctx, engine)?;
        if let Some(result) = try_dispatch_collection_method(''','''        match try_dispatch_collection_method_with(''',1).replace('''            args,
            Some(&call_spans),''','''            |scopes, ctx, engine| eval_args(call.args(), scopes, ctx, engine),
            Some(&call_spans),''',1);a='''        ) {
            return if trace_arguments_callback {
                trace_call(result, Some(method), call.span(), engine).map(Some)
            } else {
                result.map(Some)
            };
        }''';z='''        ) {
            crate::compiler::stdlib::CollectionDispatch::Handled(result) => {
                return if trace_arguments_callback {
                    trace_call(result, Some(method), call.span(), engine).map(Some)
                } else {
                    result.map(Some)
                };
            }
            crate::compiler::stdlib::CollectionDispatch::Unhandled(target) => {
                *cached_receiver = Some(target);
            }
        }''';assert a in s;s=s.replace(a,z,1);a=s.index('    // **P702**');s=s[:a]+'    Ok(None)\n}\n';write(p,s)
# Prepare arguments only after callable classification; no mat heuristics or snapshots.
p='01_core/src/compiler/eval/call_dispatch/invoke.rs';s=read(p);s=s.replace('''    resolved_callee: Option<Value>,
''','''    resolved_callee: Option<Value>,
    resolved_args: Option<Args>,
''',1);prep_start=s.index('    let float_is_nan_spans');match_start=s.index('    match callee {',prep_start);prep=s[prep_start:match_start];mat_start=prep.index('    let is_mat_call');span_start=prep.index('    if let Some(spans) = float_is_nan_spans');prep=prep[:mat_start]+'''    let mut args = match resolved_args {
        Some(args) => args,
        None => eval_args(call.args(), scopes, ctx, engine)?,
    };
'''+prep[span_start:];s=s[:prep_start]+s[match_start:];s=s.replace('''        Value::Func(func) => {
            transport_native_call_span''','''        Value::Func(func) => {
            let (mut args, p1293_html_spans) = prepare_call_args(
                &Value::Func(func.clone()), resolved_args, call, scopes, ctx, engine,
            )?;
            transport_native_call_span''',1).replace('''        Value::Type(t) => {
            let world''','''        Value::Type(t) => {
            let (args, _) = prepare_call_args(
                &Value::Type(t), resolved_args, call, scopes, ctx, engine,
            )?;
            let world''',1)
# Closed complement: reject every non-callable without evaluating arguments.
value=read('01_core/src/entities/value.rs');region=value[value.index('pub enum Value {'):value.index('pub enum Type')];variants=[]
for m in re.finditer(r'^    (\w+)([,(])',region,re.M):
 name,kind=m.groups()
 if name not in ['Func','Type']:variants.append('Value::'+name+('(..)' if kind=='(' else ''))
assert len(variants)==37,variants
pattern='\n        | '.join(variants);s=s.replace('''        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),''','''        other @ ('''+pattern+''') => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("expected function, found {}", vanilla_type_name(&other)),
        )]),''',1)
helper='''
fn prepare_call_args(
    callee: &Value,
    resolved_args: Option<Args>,
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(Args, Option<P1293HtmlCallSpans>)> {
'''+prep+'''    Ok((args, p1293_html_spans))
}
''';# Prep match expressions originally borrowed owned callee; callee is already a reference.
helper=helper.replace('&callee,','callee,').replace('match &callee {','match callee {');s+=helper;write(p,s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope=json.load(open(out/'scope.json'))['sources']),indent=2)+'\n');print('Implementation complete')
