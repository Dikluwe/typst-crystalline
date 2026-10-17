from pathlib import Path
import json,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'causal-red-result.json'))['failures']
p=Path('01_core/src/compiler/eval/bindings/field_access/fields.rs');s=p.read_text();start=s.index('    // P1484');end=s.index('    // P509',start);gates=s[start:end].replace('&target, Value::Module(module)', 'target, Value::Module(module)').replace('!ctx\n            .features\n            .contains(', '!features.contains(').replace('access.field().span()', 'span');helper='''pub(in crate::compiler::eval) fn check_module_field_feature(
    target: &Value,
    field: &str,
    span: Span,
    features: crate::entities::compiler_features::Features,
) -> SourceResult<()> {
'''+gates+'''    Ok(())
}

''';s=s[:start]+'''    check_module_field_feature(&target, field, access.field().span(), ctx.features)?;

'''+s[end:];index=s.index('pub(in crate::compiler::eval) fn eval_field_access');s=s[:index]+helper+s[index:];p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/field_access.rs');s=p.read_text();a='    eval_field_access, eval_value_field_access,\n';assert s.count(a)==1;p.write_text(s.replace(a,'    check_module_field_feature, eval_field_access, eval_value_field_access,\n'))
p=Path('01_core/src/compiler/eval/bindings/mod.rs');s=p.read_text();a='    eval_field_access, eval_value_field_access, field_callee_error,\n';assert s.count(a)==1;p.write_text(s.replace(a,'    check_module_field_feature, eval_field_access, eval_value_field_access, field_callee_error,\n'))
p=Path('01_core/src/compiler/eval/call_dispatch/collection_methods.rs');s=p.read_text();a='        let method = access.field().as_str();\n';assert s.count(a)==1;p.write_text(s.replace(a,a+'''        bindings::check_module_field_feature(
            &target, method, access.field().span(), ctx.features,
        )?;
''',1))
p=Path('01_core/src/compiler/eval/rules/set.rs');s=p.read_text();a='''        eval_expr(set.target(), scopes, ctx, engine).ok().and_then(|v| {
            if let Value::Func(f) = v {
                Some(f)
            } else {
                None
            }
        })''';z='''        match eval_expr(set.target(), scopes, ctx, engine)? {
            Value::Func(f) => Some(f),
            _ => None,
        }''';assert s.count(a)==1;s=s.replace(a,z);a='let native_target = target_function.and_then(|f| f.native_fn_addr());';assert s.count(a)==1;s=s.replace(a,'let native_target = target_function.as_ref().and_then(|f| f.native_fn_addr());')
a=s.index('    if let Expr::FieldAccess(fa) = set.target() {\n        if fa.target().to_untyped().text_str() == "format"\n            && fa.field().to_untyped().text_str() == "html"');body=s.index('            let mut seen_pretty = false;',a);end=s.index('        if fa.target().to_untyped().text_str() == "format"\n            && fa.field().to_untyped().text_str() == "svg"',body);bodytext=s[body:end];assert bodytext.endswith('        }\n');bodytext=bodytext[:-10];bodytext='\n'.join(line[4:] if line.startswith('    ') else line for line in bodytext.split('\n'));replacement='''    if matches!(&target_function, Some(function) if matches!(function.repr(),
        crate::entities::func::FuncRepr::Format(crate::entities::func::FormatElementKind::Html, _)))
    {
'''+bodytext+'''    }
    if let Expr::FieldAccess(fa) = set.target() {
''';s=s[:a]+replacement+s[end:];p.write_text(s)
(out/'causal-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_pairs=10),indent=2)+'\n');print('Causal gate and HTML set identity implemented')
