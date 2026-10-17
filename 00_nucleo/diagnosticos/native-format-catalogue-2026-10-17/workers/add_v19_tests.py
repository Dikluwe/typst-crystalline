from pathlib import Path
import json,gzip,datetime,hashlib
out=Path(__file__).resolve().parents[1]
p=Path('01_core/src/entities/func.rs');s=p.read_text();a=s.index('    fn v16_func_representations_keep_identity_hash_and_metadata_without_calls()');z=s.index('\n    #[test]',a);part=s[a:z];anchor='            native.clone().with(Args::positional(vec![])),\n';assert part.count(anchor)==1;part=part.replace(anchor,anchor+'''            Func::native_format(FormatElementKind::Svg, NativeFunc {
                name: "same",
                call: native.native_fn_addr().unwrap(),
                namespace: Some(Arc::new(native.namespace().unwrap().clone())),
            }),
''');part=part.replace('assert_eq!(left == right, i == j, "representations {i}/{j}");','assert_eq!(left == right, i == j || matches!((i,j),(0,6)|(6,0)), "representations {i}/{j}");');part=part.replace('matches!(i, 0 | 1 | 5)', 'matches!(i, 0 | 1 | 5 | 6)');part=part.replace('            assert_eq!(left.element_name(),', '            assert_eq!(left.native_fn_addr().is_some(), matches!(i, 0 | 6));\n            assert_eq!(left.element_name(),');s=s[:a]+part+s[z:];s+='''
#[cfg(test)]
mod native_format_catalogue_witness_tests {
    use super::*;
    fn never_called(_: &mut crate::compiler::eval::EvalContext, _: &Args, _: &dyn crate::contracts::world::World, _: FileId) -> SourceResult<Value> { panic!("metadata executed callback") }
    #[test]
    fn native_format_catalogue_witness_is_not_name_or_callback() {
        use crate::compiler::eval::bindings::native_element_fields;
        let cases: &[(FormatElementKind,&str,&[&str])] = &[
            (FormatElementKind::Svg,"svg",&["pretty"]),
            (FormatElementKind::Png,"png",&["ppi"]),
            (FormatElementKind::Pdf,"pdf",&["pages","standard","tagged","pretty"]),
            (FormatElementKind::Html,"html",&["pretty"]),
            (FormatElementKind::Bundle,"bundle",&[]),
        ];
        let unmarked=Func::native("same",never_called);
        assert!(native_element_fields(&unmarked).is_none());
        for (kind,name,fields) in cases {
            let function=Func::native_format(*kind,NativeFunc{name:"same",call:never_called,namespace:None});
            assert_eq!(function,unmarked);
            let (actual_name,actual_fields,locatable)=native_element_fields(&function).unwrap();
            assert_eq!(&actual_name,name);assert_eq!(&actual_fields,fields);assert!(!locatable);
            assert!(native_element_fields(&function.with(Args::positional(vec![]))).is_none());
        }
    }
}
''';p.write_text(s)
p=Path('01_core/src/compiler/eval/observation/value.rs');s=p.read_text();s+='''
#[cfg(test)]
mod native_format_catalogue_axis_tests {
    use super::*;
    use crate::entities::func::{FormatElementKind, Func, NativeFunc};
    fn never_called(_: &mut crate::compiler::eval::EvalContext, _: &crate::entities::args::Args, _: &dyn crate::contracts::world::World, _: crate::entities::file_id::FileId) -> crate::entities::source_result::SourceResult<Value> { panic!("observation executed callback") }
    #[test]
    fn native_format_catalogue_observation_kind_and_payload_axes() {
        let kinds=[FormatElementKind::Svg,FormatElementKind::Png,FormatElementKind::Pdf,FormatElementKind::Html,FormatElementKind::Bundle];
        let make=|kind| Func::native_format(kind,NativeFunc{name:"same",call:never_called,namespace:None});
        for (i,left) in kinds.iter().enumerate() {
            for (j,right) in kinds.iter().enumerate() {
                let a=make(*left);let b=make(*right);assert_eq!(a,b);
                assert_eq!(a.observation_relation(&b),if i==j {ObservationRelation::Same}else{ObservationRelation::Different});
            }
        }
        let base=make(FormatElementKind::Svg);
        let changed_call=Func::native_format(FormatElementKind::Svg,NativeFunc{name:"same",call:crate::compiler::stdlib::native_assert,namespace:None});
        let changed_name=Func::native_format(FormatElementKind::Svg,NativeFunc{name:"other",call:never_called,namespace:None});
        let mut first=Scope::new();first.define("x",Value::Int(1));
        let mut second=Scope::new();second.define("x",Value::Int(2));
        let first=Func::native_format(FormatElementKind::Svg,NativeFunc{name:"same",call:never_called,namespace:Some(Arc::new(first))});
        let second=Func::native_format(FormatElementKind::Svg,NativeFunc{name:"same",call:never_called,namespace:Some(Arc::new(second))});
        assert_eq!(base.observation_relation(&changed_call),ObservationRelation::Different);
        assert_eq!(base.observation_relation(&changed_name),ObservationRelation::Different);
        assert_eq!(base.observation_relation(&first),ObservationRelation::Different);
        assert_eq!(first.observation_relation(&second),ObservationRelation::Different);
        assert_eq!(base.observation_relation(&base.clone()),ObservationRelation::Same);
    }
}
''';p.write_text(s)
p=Path('01_core/src/compiler/eval/bindings/field_access/identity.rs');s=p.read_text();s+='''
#[cfg(test)]
mod native_format_catalogue_projection_tests {
    use super::*;
    use crate::entities::args::Args;
    use crate::entities::func::{ClosureRepr,Func};
    use crate::entities::plugin_func::PluginFunc;
    use crate::contracts::plugin_host::{PluginHost,PluginError,PluginModuleId};
    struct Host;
    impl PluginHost for Host {
        fn load(&self,_:&[u8])->Result<PluginModuleId,PluginError>{panic!("metadata called host")}
        fn exports(&self,_:PluginModuleId)->Result<Vec<ecow::EcoString>,PluginError>{panic!("metadata called host")}
        fn call(&self,_:PluginModuleId,_:&str,_:&[crate::entities::bytes::Bytes])->Result<crate::entities::bytes::Bytes,PluginError>{panic!("metadata called host")}
        fn transition(&self,_:PluginModuleId,_:&str,_:&[crate::entities::bytes::Bytes])->Result<PluginModuleId,PluginError>{panic!("metadata called host")}
    }
    #[test]
    fn native_format_catalogue_projection_groups_cover_all_representations() {
        let scope=crate::compiler::stdlib::make_stdlib_with_features(&crate::contracts::world::SysInputs::default(),crate::entities::compiler_features::Features::empty());
        let Value::Module(module)=scope.get("format").unwrap() else {panic!("format")};
        let Value::Func(svg)=module.scope().get("svg").unwrap() else {panic!("svg")};
        let Value::Func(pdf)=module.scope().get("pdf").unwrap() else {panic!("pdf")};
        let native=Func::native("ns.sample",crate::compiler::stdlib::native_assert);
        let source=crate::entities::source::Source::detached("x");
        let closure=Func::closure(ClosureRepr{name:None,params:vec![],sink_name:None,body:source.root().clone(),captured:std::sync::Arc::new(crate::entities::scope::Scope::new()),capturer:crate::entities::scope::Capturer::Function});
        let functions=[native.clone(),svg.clone(),Func::native_with_engine("ns.sample",crate::compiler::stdlib::native_op),closure.clone(),Func::element("sample",std::sync::Arc::new(|_|panic!("metadata called ctor"))),Func::plugin(PluginFunc{host:std::sync::Arc::new(Host),module:PluginModuleId(0),name:"sample".into()}),native.with(Args::positional(vec![]))];
        let names=[Some("sample"),Some("svg"),Some("sample"),None,None,None,Some("sample")];
        for (i,function) in functions.iter().enumerate() {
            assert_eq!(native_public_name(function),names[i]);
            assert_eq!(is_user_defined_closure(function),i==3);
            assert!(!is_format_pdf_function(function));
        }
        assert!(is_user_defined_closure(&closure.with(Args::positional(vec![]))));
        assert!(!is_user_defined_closure(&svg.clone().with(Args::positional(vec![]))));
        let unmarked=Func::native_with_namespace("pdf",pdf.native_fn_addr().unwrap(),std::sync::Arc::new(pdf.namespace().unwrap().clone()));
        for function in [pdf.clone(),unmarked,pdf.clone().with(Args::positional(vec![]))] {assert!(is_format_pdf_function(&function));}
    }
}
''';p.write_text(s)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),new_guard_tests=3,existing_matrix_representations=7,scope='Test-only additions; production methods unchanged since candidate-1')
(out/'v19-test-additions.json').write_text(json.dumps(record,indent=2)+'\n')
