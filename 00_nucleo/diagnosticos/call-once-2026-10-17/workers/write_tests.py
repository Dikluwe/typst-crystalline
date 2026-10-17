from pathlib import Path
out=Path(__file__).resolve().parents[1];s=Path('01_core/src/compiler/eval/tests.rs').read_text();start=s.index('    struct ImportMockWorld {');end=s.index('    #[test]',start);world=s[start:end];p=Path('01_core/src/compiler/eval/call_dispatch/call.rs')
p.write_text(p.read_text()+'''
#[cfg(test)]
mod call_once_tests {
    use crate::compiler::eval::eval_expression;
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};
    use std::num::NonZeroU16;
'''+world+'''
    fn check(expression: &str, expected: Value) {
        let world = ImportMockWorld::new("", &[("sample.typ", "#let f(x)=x\\n#let measure(x)=x\\n#let layout(x)=x\\n#let within(x)=x\\n#let before(x)=x\\n#let after(x)=x\\n#let func()=42\\n#let has(x)=x\\n#let body(x)=x\\n#let insert(x)=x\\n#let remove(x)=x\\n#let push(x)=x\\n#let pop(x)=x\\n")]);
        let (actual, warnings) = eval_expression(&world, expression);
        assert!(warnings.is_empty(), "{expression}: {warnings:?}");
        assert_eq!(actual.unwrap(), expected, "{expression}");
    }
    fn values(items: &[i64]) -> Value {
        Value::Array(items.iter().copied().map(Value::Int).collect())
    }

    #[test]
    fn call_once_module_type_and_content_receivers_and_arguments() {
        for (receiver, method, arg, expected) in [
            ("calc", "abs", "-3", Value::Int(3)),
            ("array", "len", "(1,2)", Value::Int(2)),
            ("str", "from-unicode", "65", Value::Str("A".into())),
            ("[abc]", "has", "\\\"text\\\"", Value::Bool(true)),
        ] {
            let expression = format!("{{ let n=0; let a=0; let r=({{n+=1; {receiver}}}).{method}({{a+=1; {arg}}}); (n,a,r) }}");
            check(&expression, Value::Array(vec![Value::Int(1), Value::Int(1), expected]));
        }
    }

    #[test]
    fn call_once_non_intercepted_selector_context_and_content_names() {
        for method in ["f", "measure", "layout", "within", "before", "after", "has", "body"] {
            check(&format!("{{ import \\\"sample.typ\\\" as m; let n=0; let a=0; let r=({{n+=1;m}}).{method}({{a+=1;42}}); (n,a,r) }}"), values(&[1,1,42]));
        }
        check("{ import \\\"sample.typ\\\" as m; let n=0; let r=({n+=1;m}).func(); (n,r) }", values(&[1,42]));
    }

    #[test]
    fn call_once_mutating_fallback_reuses_arguments() {
        for method in ["insert", "remove", "push", "pop"] {
            check(&format!("{{ import \\\"sample.typ\\\" as source; let m=source; let a=0; let r=m.{method}({{a+=1;42}}); (a,r) }}"), values(&[1,42]));
        }
    }

    #[test]
    fn call_once_unknown_methods_do_not_execute_arguments() {
        let world = ImportMockWorld::new("", &[]);
        for (receiver, message) in [
            ("calc", "module `calc` does not contain `__missing`"),
            ("array", "type array does not contain field `__missing`"),
            ("[abc]", "element text has no method `__missing`"),
            ("(1,2)", "type array has no method `__missing`"),
            ("(a:1)", "type dictionary has no method `__missing`"),
            ("\\\"abc\\\"", "type string has no method `__missing`"),
            ("42", "type integer has no method `__missing`"),
            ("version(1,2,3)", "type version has no method `__missing`"),
        ] {
            let expr = format!("({receiver}).__missing(panic(\\\"argument executed\\\"))");
            let (result, _) = eval_expression(&world, &expr);
            assert_eq!(result.unwrap_err()[0].message, message, "{expr}");
        }
    }

    #[test]
    fn call_once_collections_where_with_selector_and_mutation_controls() {
        check("{ let n=0; let a=0; let r=({n+=1;(1,2)}).at({a+=1;1}); (n,a,r) }", values(&[1,1,2]));
        check("{ let n=0; let a=0; let r=({n+=1;calc.abs}).with({a+=1;-3}); (n,a,r()) }", values(&[1,1,3]));
        check("{ let n=0; let a=0; let r=({n+=1;heading}).where(level:{a+=1;1}); (n,a,type(r)==selector) }", Value::Array(vec![Value::Int(1),Value::Int(1),Value::Bool(true)]));
        check("{ let n=0; let a=0; let r=({n+=1;heading.where()}).or({a+=1;figure.where()}); (n,a,type(r)==selector) }", Value::Array(vec![Value::Int(1),Value::Int(1),Value::Bool(true)]));
        check("{ let a=(1,); a.push(2); a }", values(&[1,2]));
    }

    #[test]
    fn call_once_matrix_keeps_each_legitimate_effect() {
        check("{ let n=0; let r=math.mat({n+=1;1}); n }", Value::Int(1));
        check("{ let n=(1,); let r=math.mat({n.push(1);1}); n }", values(&[1,1]));
        check("{ let n=(); let r=math.mat({n.push(1);1}); n }", values(&[1]));
    }
}
''');print('six functional tests written')
