from pathlib import Path
import json,gzip,datetime,hashlib,subprocess
out=Path(__file__).resolve().parents[1];cases=json.load(open(out/'batch-schema.json'))['cases'];source=Path('01_core/src/compiler/eval/bindings/value_methods/element_where.rs');q=json.dumps
s='''
#[cfg(test)]
mod native_format_catalogue_tests {
    use super::*;
    use crate::entities::func::Func;

    fn binding(path: &str) -> Func {
        use crate::entities::compiler_features::{Feature, Features};
        let mut features = Features::empty();
        for feature in [Feature::Html, Feature::A11yExtras, Feature::Bundle] { features.enable(feature); }
        let scope = crate::compiler::stdlib::make_stdlib_with_features(&crate::contracts::world::SysInputs::default(), features);
        let mut parts = path.strip_prefix("std.").unwrap().split('.');
        let mut value = scope.get(parts.next().unwrap()).unwrap().clone();
        for part in parts {
            let next = if let Value::Module(module) = &value { module.scope().get(part) }
                else if let Value::Func(function) = &value { function.namespace().unwrap().get(part) }
                else { panic!("not a namespace") };
            value = next.unwrap().clone();
        }
        let Value::Func(function) = value else { panic!("function binding") };
        function
    }

    fn check(path: &str, name: &'static str, fields: &[&str], excluded: &[&str], locatable: bool, format: bool) {
        let function = binding(path);
        let alias = function.clone();
        for function in [function, alias] {
            let (actual_name, actual_fields, actual_locatable) = native_element_fields(&function).expect("registered format element missing");
            assert_eq!(actual_name, name);
            assert_eq!(actual_fields, fields);
            assert_eq!(actual_locatable, locatable);
            let mut args = Args::positional(vec![]);
            for field in fields { args.named.insert((*field).into(), Value::Auto); }
            args.named.insert("label".into(), Value::Label(crate::entities::label::Label("audit".into())));
            let selector = eval_element_where(&function, &args).unwrap();
            let result = validate_element_locatability(&selector, args.span);
            if locatable { result.unwrap(); } else { assert_eq!(result.unwrap_err()[0].message, format!("{name} is not locatable")); }
            for field in std::iter::once("__audit_missing").chain(excluded.iter().copied()) {
                args.named.insert(field.into(), Value::Auto);
                assert_eq!(eval_element_where(&function, &args).unwrap_err()[0].message,format!("element `{name}` does not have field `{field}`"));
                args.named.shift_remove(field);
            }
            let unmarked = Func::native("alias",function.native_fn_addr().unwrap());
            assert_eq!(native_element_fields(&unmarked).is_none(),format);
            assert!(native_element_fields(&function.with(Args::positional(vec![]))).is_none());
        }
        let fake = Func::native(name, crate::compiler::stdlib::native_assert);
        assert!(native_element_fields(&fake).is_none());
    }
'''
for c in cases:
 s+='\n    #[test]\n    fn native_format_catalogue_'+c['expression'].replace('std.','').replace('.','_')+'() {\n        check('+q(c['expression'])+','+q(c['name'])+', &['+', '.join(map(q,c['fields']))+'], &['+', '.join(map(q,c['excluded']))+'], '+str(c['locatable']).lower()+', '+str(c['format_kind'] is not None).lower()+');\n    }\n'
s+='}\n';source.write_text(source.read_text()+s)
p=Path('01_core/src/entities/func.rs');p.write_text(p.read_text()+'''
#[cfg(test)]
mod native_format_catalogue_compat_tests {
    use super::*;

    fn formats() -> Vec<Func> {
        let scope = crate::compiler::stdlib::make_stdlib_with_features(&crate::contracts::world::SysInputs::default(), crate::entities::compiler_features::Features::empty());
        let Value::Module(module) = scope.get("format").unwrap() else { panic!("format module") };
        ["svg", "png", "pdf", "html", "bundle"].into_iter().map(|name| {
            let Value::Func(function) = module.scope().get(name).unwrap() else { panic!("format function") };
            function.clone()
        }).collect()
    }

    #[test]
    fn native_format_catalogue_public_native_projections() {
        fn fingerprint(function: &Func) -> u64 {
            let mut state=std::collections::hash_map::DefaultHasher::new();
            function.hash(&mut state);state.finish()
        }
        for function in formats() {
            let name=function.name().unwrap();
            let static_name=match name { "svg"=>"svg", "png"=>"png", "pdf"=>"pdf", "html"=>"html", "bundle"=>"bundle", other=>panic!("unexpected format {other}") };
            let namespace=function.namespace().unwrap();
            let native=Func::native_with_namespace(static_name,function.native_fn_addr().unwrap(),Arc::new(namespace.clone()));
            assert_eq!(function,native);
            assert_eq!(fingerprint(&function),fingerprint(&native));
            assert!(function.element_name().is_none());
            assert_eq!(format!("{function:?}"),"<function>");
            let alias=function.clone();assert_eq!(alias,function);
            let bound=function.clone().with(Args::positional(vec![]));
            assert_eq!(bound.name(),function.name());
            assert!(bound.native_fn_addr().is_none());
            assert!(bound.element_name().is_none());
            assert_eq!(bound.namespace().unwrap().len(),namespace.len());
            let mut named=function.clone();named.set_name("renamed".into());
            assert_eq!(named.name(),function.name());
        }
    }

    #[test]
    fn native_format_catalogue_layout_measurement() {
        println!("FORMAT_LAYOUT {{\\"func\\":{},\\"repr\\":{},\\"native\\":{},\\"engine\\":{}}}", std::mem::size_of::<Func>(),std::mem::size_of::<FuncRepr>(),std::mem::size_of::<NativeFunc>(),std::mem::size_of::<NativeFuncWithEngine>());
    }
}
''')
p=Path('01_core/src/compiler/eval/observation/value.rs');p.write_text(p.read_text()+'''
#[cfg(test)]
mod native_format_catalogue_observation_tests {
    use super::*;
    #[test]
    fn native_format_catalogue_observation_witness() {
        let scope=crate::compiler::stdlib::make_stdlib_with_features(&crate::contracts::world::SysInputs::default(),crate::entities::compiler_features::Features::empty());
        let Value::Module(module)=scope.get("format").unwrap() else { panic!("format module") };
        let Value::Func(svg)=module.scope().get("svg").unwrap() else { panic!("svg") };
        assert_eq!(svg.observation_relation(&svg.clone()),ObservationRelation::Same);
        let unmarked=crate::entities::func::Func::native_with_namespace("svg",svg.native_fn_addr().unwrap(),Arc::new(svg.namespace().unwrap().clone()));
        assert_eq!(svg,&unmarked);
        assert_eq!(svg.observation_relation(&unmarked),ObservationRelation::Different);
    }
}
''')
(out/'tests-written.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [source,Path('01_core/src/entities/func.rs'),Path('01_core/src/compiler/eval/observation/value.rs')]},new_tests=18),indent=2)+'\n')
