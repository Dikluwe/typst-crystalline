from pathlib import Path
import json,hashlib,datetime
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));p=Path(scope['sources'][0]);cases=json.load(open(out/'batch-schema.json'))['cases'];q=lambda s:json.dumps(s)
s='''
#[cfg(test)]
mod native_catalogue_tests {
    use super::*;
    use crate::compiler::stdlib::*;
    use crate::entities::func::Func;

    fn check(function: Func, name: &str, fields: &[&str], locatable: bool) {
        let (actual_name, actual_fields, actual_locatable) =
            native_element_fields(&function).expect("existing native element missing");
        assert_eq!(actual_name, name);
        assert_eq!(actual_fields, fields);
        assert_eq!(actual_locatable, locatable);
        let mut args = Args::positional(vec![]);
        for field in fields {
            args.named.insert((*field).into(), Value::Auto);
        }
        args.named.insert("label".into(), Value::Label(crate::entities::label::Label("audit".into())));
        let selector = eval_element_where(&function, &args).unwrap();
        for selector in [selector, Selector::NativeElement(function.clone())] {
            let result = validate_element_locatability(&selector, args.span);
            if locatable {
                result.unwrap();
            } else {
                assert_eq!(result.unwrap_err()[0].message, format!("{name} is not locatable"));
            }
        }
        args.named.insert("__audit_missing".into(), Value::Auto);
        assert_eq!(eval_element_where(&function, &args).unwrap_err()[0].message,
            format!("element `{name}` does not have field `__audit_missing`"));
        assert!(native_element_fields(&function.with(Args::positional(vec![]))).is_none());
    }
'''
for c in cases:
 constructor='native_with_engine' if c['abi']==6 else 'native';s+='\n    #[test]\n    fn native_catalogue_'+c['callback'].removeprefix('native_')+'() {\n        check(Func::'+constructor+'("alias", '+c['callback']+'), '+q(c['name'])+', &['+', '.join(map(q,c['fields']))+'], '+str(c['locatable']).lower()+');\n    }\n'
s+='''
    #[test]
    fn native_catalogue_homonyms_and_page_controls() {
        for name in ["title", "op", "item", "document", "asset", "move"] {
            let fake = Func::native(name, native_assert);
            assert!(native_element_fields(&fake).is_none());
        }
        let page = Func::native_with_engine("alias", native_page);
        let (name, fields, locatable) = native_element_fields(&page).unwrap();
        assert_eq!(name, "page");
        assert!(fields.contains(&"width"));
        assert!(!locatable);
        assert!(native_element_fields(&page.with(Args::positional(vec![]))).is_none());
    }
}
'''
p.write_text(p.read_text()+s)
(out/'tests-before-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),source=str(p),new_tests=16),indent=2)+'\n')
