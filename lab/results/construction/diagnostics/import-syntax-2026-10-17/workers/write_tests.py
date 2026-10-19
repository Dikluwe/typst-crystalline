from pathlib import Path
out=Path(__file__).resolve().parents[1];p=Path('01_core/src/compiler/eval/modules.rs');template=Path('01_core/src/compiler/eval/tests.rs').read_text();a=template.index('    struct ImportMockWorld {');z=template.index('    #[test]',a);world=template[a:z];extra='''
        fn resolve_path(&self, _: FileId, path: &str) -> Result<crate::entities::path::RootedPath, String> {
            Ok(crate::entities::path::RootedPath::new(crate::entities::path::VirtualRoot::Project,
                crate::entities::path::VirtualPath::new(path).map_err(|e| format!("{e:?}"))?))
        }
        fn include_path(&self, path: &crate::entities::path::RootedPath) -> Result<Source, String> {
            self.include_source(self.main(), path.vpath().get_with_slash().trim_start_matches('/'))
        }
        fn resolve_package(&self, _: &crate::entities::package_spec::PackageSpec) -> Result<Source, String> {
            self.include_source(self.main(), "bad.typ")
        }
''';# final brace closes impl, preceding brace closes include_source
idx=world.rfind('    }');world=world[:idx]+extra+world[idx:]
p.write_text(p.read_text()+'''
#[cfg(test)]
mod import_syntax_tests {
    use super::*;
    use comemo::Track;
    use crate::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source_result::Severity;
    use crate::entities::syntax_node::SyntaxErrorKind;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Routines, Traced};
    use std::num::NonZeroU16;
'''+world+'''
    fn run(world: &ImportMockWorld, features: Features, target: EvalTarget) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(&Routines::new(), world,
            Traced::default().track(), sink.track_mut(), Route::root().track(), &world.main,
            &ElementRegistry::new(), false, target, features);
        (result, sink.into_diagnostics())
    }
    fn features(mask: u8) -> Features {
        let mut features = Features::empty();
        for (bit, flag) in [(1,Feature::Html),(2,Feature::A11yExtras),(4,Feature::Bundle)] {
            if mask & bit != 0 { features.enable(flag); }
        }
        features
    }
    fn hard_errors(world: &ImportMockWorld) -> Vec<SourceDiagnostic> {
        world.files["bad.typ"].root().errors().into_iter().filter(|error| {
            !matches!(error.kind, SyntaxErrorKind::NoTextWithinStars | SyntaxErrorKind::NoTextWithinUnderscores)
        }).map(|error| { let mut d=SourceDiagnostic::error(error.span,error.message.to_string());
            d.hints=error.hints.iter().map(ToString::to_string).collect(); d }).collect()
    }
    fn check_invalid(text: &str, bad: &str, trace: &[&str]) {
        let world = ImportMockWorld::new(text, &[("bad.typ",bad),("nested.typ","#import \\\"bad.typ\\\" as child")]);
        let expected=hard_errors(&world);
        assert!(!expected.is_empty());
        let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&world,Features::empty(),EvalTarget::Paged)));
        let (result, _)=outcome.unwrap_or_else(|_| panic!("invalid source panicked: {text}/{bad}"));
        let errors=result.unwrap_err();
        assert_eq!(errors.len(),expected.len(),"{text}/{bad}: {errors:?}");
        for (actual, expected) in errors.iter().zip(expected) {
            assert_eq!(actual.severity,Severity::Error);
            assert_eq!(actual.message,expected.message);
            assert_eq!(actual.span,expected.span);
            assert_eq!(actual.hints,expected.hints);
            assert!(world.files["bad.typ"].span_byte_range(actual.span).is_some());
            assert_eq!(actual.trace.len(),trace.len(),"{text}: {:?}",actual.trace);
            for (point,name) in actual.trace.iter().zip(trace) {
                match &point.v { Tracepoint::Import(path) | Tracepoint::Include(path) => assert_eq!(path,name),
                    other=>panic!("unexpected trace {other:?}") }
                assert!(!point.span.is_detached());
            }
        }
    }

    #[test]
    fn import_syntax_rejects_malformed_alias_items_and_wildcards_without_panic() {
        for bad in ["#let or(x)=x", "#let x = \\\"unterminated", "#let x =", "#let a = 0xzz\\n#let b = \\\"\\\\u{110000}\\\""] {
            for text in ["#import \\\"bad.typ\\\" as sample", "#import \\\"bad.typ\\\": x", "#import \\\"bad.typ\\\": *"] {
                check_invalid(text,bad,&["bad.typ"]);
            }
        }
    }
    #[test]
    fn import_syntax_include_string_path_and_nested_preserve_error_origin() {
        for text in ["#include \\\"bad.typ\\\"", "#include path(\\\"bad.typ\\\")"] {
            check_invalid(text,"#let x = \\\"unterminated",&[if text.contains("path(") {"/bad.typ"} else {"bad.typ"}]);
        }
        check_invalid("#import \\\"nested.typ\\\" as outer","#let or(x)=x",&["bad.typ","nested.typ"]);
        check_invalid("#include \\\"nested.typ\\\"","#let or(x)=x",&["bad.typ","nested.typ"]);
    }
    #[test]
    fn import_syntax_package_source_is_validated_with_package_trace() {
        check_invalid("#import \\\"@preview/test:0.1.0\\\" as sample","#let or(x)=x",&["@preview/test:0.1.0"]);
    }
    #[test]
    fn import_syntax_parse_error_precedes_prefix_effect_and_preserves_warnings() {
        check_invalid("#import \\\"bad.typ\\\" as sample","#panic(\\\"prefix executed\\\")\\n#let or(x)=x",&["bad.typ"]);
        let world=ImportMockWorld::new("#import \\\"bad.typ\\\" as sample",&[("bad.typ","**\\n#let or(x)=x")]);
        let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&world,Features::empty(),EvalTarget::Paged)));
        let (result,warnings)=outcome.expect("parser diagnostics must not panic");
        assert!(result.is_err());assert_eq!(warnings.len(),1);assert_eq!(warnings[0].severity,Severity::Warning);
        assert_eq!(world.files["bad.typ"].span_byte_range(warnings[0].span),Some(0..2));
        assert!(!warnings[0].hints.is_empty());
    }
    #[test]
    fn import_syntax_empty_markup_warnings_are_nonfatal_and_deduplicated() {
        for text in ["#import \\\"bad.typ\\\" as a\\n#import \\\"bad.typ\\\" as b", "#include \\\"bad.typ\\\"\\n#include \\\"bad.typ\\\""] {
            let world=ImportMockWorld::new(text,&[("bad.typ","** __\\n#let x=42")]);
            let (result,warnings)=run(&world,Features::empty(),EvalTarget::Paged);result.unwrap();
            assert_eq!(warnings.len(),2,"{warnings:?}");
            for (warning,range) in warnings.iter().zip([0..2,3..5]) {
                assert_eq!(warning.severity,Severity::Warning);
                assert_eq!(world.files["bad.typ"].span_byte_range(warning.span),Some(range));
                assert!(!warning.hints.is_empty());assert!(warning.trace.is_empty());
            }
        }
    }
    #[test]
    fn import_syntax_features_are_inherited_in_all_eight_profiles() {
        for mask in 0..8 {
            for (expr,bit,field,feature) in [("std.html",1,"html","html"),("std.pdf.header-cell",2,"header-cell","a11y-extras"),("std.asset",4,"asset","bundle")] {
                let text=format!("#let value={expr}");
                let world=ImportMockWorld::new("#import \\\"bad.typ\\\" as sample",&[("bad.typ",&text)]);
                let (result,warnings)=run(&world,features(mask),EvalTarget::Paged);
                assert!(warnings.is_empty());
                if mask & bit != 0 { let module=result.unwrap(); assert!(matches!(module.scope().get("sample"),Some(Value::Module(_)))); }
                else { assert_eq!(result.unwrap_err()[0].message,format!("cannot access field `{field}` because the `{feature}` feature is not enabled")); }
            }
        }
    }
    #[test]
    fn import_syntax_bundle_target_is_inherited_without_enabling_other_targets() {
        for target in [EvalTarget::Paged,EvalTarget::Html,EvalTarget::Bundle] {
            let world=ImportMockWorld::new("#import \\\"bad.typ\\\" as sample",&[("bad.typ","#let child=document(\\\"child.html\\\")[hello]")]);
            let (result,_)=run(&world,features(5),target);
            if target==EvalTarget::Bundle {
                let module=result.unwrap(); let Some(Value::Module(sample))=module.scope().get("sample") else {panic!("module binding")};
                assert!(matches!(sample.scope().get("child"),Some(Value::Content(_))));
            } else { assert_eq!(result.unwrap_err()[0].message,"constructing a document is only supported in the bundle target"); }
        }
    }
    #[test]
    fn import_syntax_runtime_errors_keep_import_trace_and_valid_exports() {
        let world=ImportMockWorld::new("#import \\\"bad.typ\\\" as sample",&[("bad.typ","#panic(\\\"runtime executed\\\")")]);
        let (result,_)=run(&world,Features::empty(),EvalTarget::Paged);let errors=result.unwrap_err();
        assert_eq!(errors[0].message,"panicked with: runtime executed");
        assert!(matches!(errors[0].trace.last().map(|t|&t.v),Some(Tracepoint::Import(path)) if path=="bad.typ"));
        let world=ImportMockWorld::new("#import \\\"bad.typ\\\": x\\n#let result=x+1",&[("bad.typ","#let x=42")]);
        let (result,warnings)=run(&world,Features::empty(),EvalTarget::Paged);assert!(warnings.is_empty());
        assert_eq!(result.unwrap().scope().get("result"),Some(&Value::Int(43)));
    }
}
''');print('eight functional tests written')
