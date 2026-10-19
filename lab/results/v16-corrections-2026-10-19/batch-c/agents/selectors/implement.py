from pathlib import Path
import re,json
# Execute only after coordinator authorizes implementation following valid RED.
value_source=Path('01_core/src/entities/value.rs').read_text().split('pub enum Value {',1)[1].split('\n}',1)[0]
value_patterns=[]
for m in re.finditer(r'^    ([A-Z]\w*)(\(|\s*\{|,)',value_source,re.M):
 name,form=m.groups();value_patterns.append((name,'Value::'+name+('(_)' if form=='(' else (' { .. }' if '{' in form else ''))))
def pattern_group(patterns,indent='        '):return ('\n'+indent+'| ').join(patterns)
p=Path('01_core/src/compiler/eval/bindings/value_methods/selector.rs');s=p.read_text();start=s.index('    match value {');end=s.index('\n}\n',start)
accepted={'Selector','Func','Str','Regex','Label','Location'}
match='''    match value {
        Value::Selector(selector) => Some(selector.clone()),
        Value::Func(function) => crate::compiler::stdlib::element_kind_of_native_func(function)
            .map(Selector::Kind)
            .or_else(|| native_element_fields(function).map(|_| Selector::NativeElement(function.clone()))),
        Value::Str(text) => {
            if text.is_empty() { None } else {
                crate::entities::regex::Regex::new(&regex::escape(text.as_str())).ok().map(Selector::Regex)
            }
        }
        Value::Regex(regex) => {
            if regex.pattern().is_empty() || regex.is_match("") { None } else { Some(Selector::Regex(regex.clone())) }
        }
        Value::Label(label) => Some(Selector::Label(label.clone())),
        Value::Location(location) => Some(Selector::Location(*location)),
        '''+pattern_group([pat for name,pat in value_patterns if name not in accepted])+''' => None,
    }'''
s=s[:start]+match+s[end:]
# Distinct receiver policy: newly admitted operand types do not gain methods.
pos=s.index('\npub(in crate::compiler::eval) fn eval_selector_or_and')
helper='''
fn selector_receiver(value: &Value) -> Option<Selector> {
    match value {
        Value::Selector(_) => value_to_query_selector(value),
        Value::Func(function) => function.native_fn_addr().and_then(|call| {
            if std::ptr::fn_addr_eq(call,crate::compiler::stdlib::native_heading as fn(_,_,_,_)->_) {
                Some(Selector::Kind(ElementKind::Heading))
            } else if std::ptr::fn_addr_eq(call,crate::compiler::stdlib::native_figure as fn(_,_,_,_)->_) {
                Some(Selector::Kind(ElementKind::Figure))
            } else { None }
        }),
        '''+pattern_group([pat for name,pat in value_patterns if name not in {'Selector','Func'}])+''' => None,
    }
}
'''
s=s[:pos]+helper+s[pos:];s=s.replace('let Some(base) = value_to_query_selector(target)','let Some(base) = selector_receiver(target)')
s=s.replace('    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))','    crate::compiler::eval::call_dispatch::validate_contextual_selector(&ancestor, args_node.span())?;\n    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))')
p.write_text(s)
# Contextual validator retained here to avoid changing legacy query validator.
p=Path('01_core/src/compiler/eval/call_dispatch/value_methods.rs');s=p.read_text();s=s.replace('                    Ok(Value::Selector(crate::entities::selector::Selector::Within {','                    validate_contextual_selector(&ancestor, args.span)?;\n                    Ok(Value::Selector(crate::entities::selector::Selector::Within {')
pos=s.index('\npub(in crate::compiler::eval) fn selector_before_after')
helper='''
pub(in crate::compiler::eval) fn validate_contextual_selector(selector: &crate::entities::selector::Selector, span: Span) -> SourceResult<()> {
    use crate::entities::selector::Selector;
    match selector {
        Selector::Regex(_) => Err(vec![SourceDiagnostic::error(span,"text is not locatable")]),
        Selector::NativeElement(_) | Selector::Element { .. } => bindings::validate_element_locatability(selector,span),
        Selector::And(children) | Selector::Or(children) => {
            for child in children { validate_contextual_selector(child,span)?; }
            Ok(())
        }
        Selector::Where { base, .. } => validate_contextual_selector(base,span),
        Selector::Within { base, ancestor } => {validate_contextual_selector(base,span)?;validate_contextual_selector(ancestor,span)},
        Selector::Before { base, end, .. } => {validate_contextual_selector(base,span)?;validate_contextual_selector(end,span)},
        Selector::After { base, start, .. } => {validate_contextual_selector(base,span)?;validate_contextual_selector(start,span)},
        Selector::Kind(_) | Selector::Label(_) | Selector::Location(_) => Ok(()),
    }
}
'''
s=s[:pos]+helper+s[pos:];s=s.replace('    let selector = if method == "before" {','    validate_contextual_selector(&boundary, args.span)?;\n    let selector = if method == "before" {');p.write_text(s)
# Native language equality retains each ABI's typed pointer comparison.
p=Path('01_core/src/compiler/eval/operators/equality.rs');s=p.read_text();old='''            match (a.native_fn_addr(), b.native_fn_addr()) {
                (Some(a), Some(b)) => std::ptr::fn_addr_eq(a, b),
                _ => false,
            }'''
new='''            match (a.repr(), b.repr()) {
                (crate::entities::func::FuncRepr::Native(a),crate::entities::func::FuncRepr::Native(b)) => std::ptr::fn_addr_eq(a.call,b.call),
                (crate::entities::func::FuncRepr::Format(ak,a),crate::entities::func::FuncRepr::Format(bk,b)) => ak == bk && std::ptr::fn_addr_eq(a.call,b.call),
                (crate::entities::func::FuncRepr::NativeWithEngine(a),crate::entities::func::FuncRepr::NativeWithEngine(b)) => std::ptr::fn_addr_eq(a.call,b.call),
                (crate::entities::func::FuncRepr::Closure(_)
                    | crate::entities::func::FuncRepr::Native(_)
                    | crate::entities::func::FuncRepr::Format(_, _)
                    | crate::entities::func::FuncRepr::NativeWithEngine(_)
                    | crate::entities::func::FuncRepr::Element(_)
                    | crate::entities::func::FuncRepr::Plugin(_)
                    | crate::entities::func::FuncRepr::With(_),
                    crate::entities::func::FuncRepr::Closure(_)
                    | crate::entities::func::FuncRepr::Native(_)
                    | crate::entities::func::FuncRepr::Format(_, _)
                    | crate::entities::func::FuncRepr::NativeWithEngine(_)
                    | crate::entities::func::FuncRepr::Element(_)
                    | crate::entities::func::FuncRepr::Plugin(_)
                    | crate::entities::func::FuncRepr::With(_)) => false,
            }'''
assert old in s;s=s.replace(old,new,1)
selector_patterns=['Selector::'+x for x in ['Kind(_)','NativeElement(_)','Element { .. }','Label(_)','Location(_)','And(_)','Or(_)','Regex(_)','Where { .. }','Within { .. }','Before { .. }','After { .. }']]
start=s.index('pub(crate) fn selectors_eq(');end=s.index('\n}\n',start);chunk=s[start:end];tail='''        (
            '''+pattern_group(selector_patterns,'            ')+''',
            '''+pattern_group(selector_patterns,'            ')+''',
        ) => false,''';assert chunk.endswith('        _ => false,\n    }');chunk=chunk[:-len('        _ => false,\n    }')]+tail+'\n    }';s=s[:start]+chunk+s[end:];p.write_text(s)
# Kind bridge uses only existing show representations.
p=Path('01_core/src/compiler/eval/selector_matching.rs');s=p.read_text();start=s.index('    fn kind_to_node(');end=s.index('\n    match sel {',start)
nodes=['Heading','Figure','Link','Raw','Quote','Footnote','List','Enum','Equation','Par']
natives={'Table':'native_table','Metadata':'native_metadata','Outline':'native_outline','OutlineEntry':'native_outline_entry','Citation':'native_cite','Bibliography':'native_bibliography'}
kinds='''    fn kind_to_show(kind: ElementKind) -> Option<Selector> {
        match kind {
'''
for n in nodes:kinds+=f'            ElementKind::{n} => Some(Selector::NodeKind(NodeKind::{n})),\n'
for n,f in natives.items():kinds+=f'            ElementKind::{n} => Some(Selector::NativeElement(Func::native("{f.removeprefix("native_")}",crate::compiler::stdlib::{f}))),\n'
kinds+='            '+pattern_group(['ElementKind::'+n for n in ['State','StateUpdate','CounterUpdate','StateDisplay','CounterDisplay','ContextBlock']],'            ')+''' => None,
        }
    }
''';s=s[:start]+kinds+s[end:];s=s.replace('match kind_to_node(kind) {\n            Some(node) => Ok(Selector::NodeKind(node)),','match kind_to_show(kind) {\n            Some(selector) => Ok(selector),')
start=s.index('pub(crate) fn native_element_matches(');end=s.index('\n/// Linguistic fields',start);chunk=s[start:end]
a=chunk.index('    if let crate::entities::func::FuncRepr::NativeWithEngine');b=chunk.index('    macro_rules! native {',a)
chunk=chunk[:a]+'''    let content = match content {
        Content::Source(e) => return native_element_matches(&e.body,function),
        Content::ShowRecipe(e) => return native_element_matches(&e.body,function),
        Content::Located(e) => return native_element_matches(&e.body,function),
        Content::Realized(e) => return native_element_matches(&e.visual,function),
        other => other,
    };
    let addr = function.native_fn_addr();
'''+chunk[b:];chunk=chunk.replace('std::ptr::fn_addr_eq(addr, $function as fn(_, _, _, _) -> _)','addr.is_some_and(|addr| std::ptr::fn_addr_eq(addr, $function as fn(_, _, _, _) -> _))')
chunk=chunk.replace('''        Content::Styled(e) if native!(native_math_equation) => {
            native_element_matches(&e.body, function)
        }''','''        Content::Styled(e) => {
            native!(native_math_equation) && native_element_matches(&e.body, function)
        }''')
map=json.load(open('lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/implementation-plan.json'))['maps'];added=''
for content,callback in map.items():
 added+=f'        Content::{content}('+('e' if content=='MathFrac' else '_')+f') => '+('e.line && ' if content=='MathFrac' else '')+f'native!({callback}),\n'
added+='''        Content::MathLimitsOverride(e) => { if e.limits { native!(native_math_limits) } else { native!(native_math_scripts) } },
        Content::PageRun(_) => match function.repr() {
            crate::entities::func::FuncRepr::NativeWithEngine(native) => std::ptr::fn_addr_eq(native.call,native_page as fn(_,_,_,_,_,_)->_),
            crate::entities::func::FuncRepr::Closure(_)
            | crate::entities::func::FuncRepr::Native(_)
            | crate::entities::func::FuncRepr::Format(_, _)
            | crate::entities::func::FuncRepr::Element(_)
            | crate::entities::func::FuncRepr::Plugin(_)
            | crate::entities::func::FuncRepr::With(_) => false,
        },
        Content::MathOp(_) => match function.repr() {
            crate::entities::func::FuncRepr::NativeWithEngine(native) => std::ptr::fn_addr_eq(native.call,native_op as fn(_,_,_,_,_,_)->_),
            crate::entities::func::FuncRepr::Closure(_)
            | crate::entities::func::FuncRepr::Native(_)
            | crate::entities::func::FuncRepr::Format(_, _)
            | crate::entities::func::FuncRepr::Element(_)
            | crate::entities::func::FuncRepr::Plugin(_)
            | crate::entities::func::FuncRepr::With(_) => false,
        },
'''
# All current Content alternatives come directly from the actual domain declaration.
content_source=Path('01_core/src/entities/content.rs').read_text().split('pub enum Content {',1)[1].split('\n}\n',1)[0]
content_patterns=[]
for m in re.finditer(r'^    ([A-Z]\w*)(\(|\s*\{|,)',content_source,re.M):
 name,form=m.groups();content_patterns.append((name,'Content::'+name+('(_)' if form=='(' else (' { .. }' if '{' in form else ''))))
handled=set(re.findall(r'Content::(\w+)',chunk+added));complement=[pat for name,pat in content_patterns if name not in handled]
# Replace only the selected final Content fallback, preserving Transform's explicit arms.
assert chunk.endswith('        _ => false,\n    }\n}\n')
chunk=chunk[:-len('        _ => false,\n    }\n}\n')]+added+'        '+pattern_group(complement)+' => false,\n    }\n}\n'
s=s[:start]+chunk+s[end:]
# Lexical field specialization with explicit nominal fallback and guard refusal.
start=s.index('fn native_element_field(');end=s.index('\n/// A present occurrence',start)
field='''fn native_element_field(content: &Content, field: &str) -> Option<Value> {
    match content {
        Content::Text(text) => match field { "text" => Some(Value::Str(text.clone())), _ => content.get_field(field) },
        Content::Raw(raw) => match field {
            "text" => Some(Value::Str(raw.text.clone())),
            "lang" => Some(raw.lang.clone().map(Value::Str).unwrap_or(Value::None)),
            "block" => Some(Value::Bool(raw.block)),
            _ => content.get_field(field),
        },
        Content::Styled(styled) => {
            if field == "body" && is_styled_origin(content,true,true,true,true,true) {
                Some(Value::Content(styled.body.clone()))
            } else { content.get_field(field) }
        }
        '''+pattern_group([pat for name,pat in content_patterns if name not in {'Text','Raw','Styled'}])+''' => content.get_field(field),
    }
}
''';s=s[:start]+field+s[end:];p.write_text(s)
