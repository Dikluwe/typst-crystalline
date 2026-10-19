from pathlib import Path
import json,datetime,hashlib
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));p=Path(scope['sources'][0]);cases=json.load(open(out/'batch-schema.json'))['cases'];q=json.dumps
s='''
#[cfg(test)]
mod native_math_catalogue_tests {
    use super::*;
    use crate::entities::func::Func;

    fn binding(name: &str) -> Func {
        let Value::Module(module) = crate::compiler::stdlib::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(function)) = module.scope().get(name) else {
            panic!("math function missing")
        };
        function.clone()
    }

    fn check(name: &'static str, fields: &[&str], excluded: &[&str]) {
        let function = binding(name);
        let alias = Func::native("alias", function.native_fn_addr().unwrap());
        for function in [function, alias] {
            let (actual_name, actual_fields, locatable) =
                native_element_fields(&function).expect("existing math element missing");
            assert_eq!(actual_name, name);
            assert_eq!(actual_fields, fields);
            assert!(!locatable);
            let mut args = Args::positional(vec![]);
            for field in fields {
                args.named.insert((*field).into(), Value::Auto);
            }
            args.named.insert("label".into(), Value::Label(crate::entities::label::Label("audit".into())));
            let selector = eval_element_where(&function, &args).unwrap();
            for selector in [selector, Selector::NativeElement(function.clone())] {
                assert_eq!(validate_element_locatability(&selector, args.span).unwrap_err()[0].message,
                    format!("{name} is not locatable"));
            }
            for field in std::iter::once("__audit_missing").chain(excluded.iter().copied()) {
                args.named.insert(field.into(), Value::Auto);
                assert_eq!(eval_element_where(&function, &args).unwrap_err()[0].message,
                    format!("element `{name}` does not have field `{field}`"));
                args.named.shift_remove(field);
            }
            assert!(native_element_fields(&function.with(Args::positional(vec![]))).is_none());
        }
        let fake = Func::native(name, crate::compiler::stdlib::native_assert);
        assert!(native_element_fields(&fake).is_none());
    }
'''
for c in cases:
 s+='\n    #[test]\n    fn native_math_catalogue_'+c['name']+'() {\n        check('+q(c['name'])+', &['+', '.join(map(q,c['fields']))+'], &['+', '.join(map(q,c['excluded']))+']);\n    }\n'
s+='''
    #[test]
    fn native_math_catalogue_non_elements_and_global_decorations() {
        for name in ["sqrt", "abs", "norm", "round"] {
            assert!(native_element_fields(&binding(name)).is_none());
        }
        for function in [
            Func::native("math.underline", crate::compiler::stdlib::native_underline),
            Func::native("math.overline", crate::compiler::stdlib::native_overline),
        ] {
            let (_, fields, _) = native_element_fields(&function).unwrap();
            assert!(fields.contains(&"stroke"));
        }
    }
}
'''
p.write_text(p.read_text()+s);(out/'tests-before-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),new_tests=27),indent=2)+'\n')
