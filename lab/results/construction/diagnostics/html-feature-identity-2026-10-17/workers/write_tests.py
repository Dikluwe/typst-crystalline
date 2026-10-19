from pathlib import Path
import re,json
out=Path(__file__).resolve().parents[1]
identity=Path('01_core/src/compiler/eval/bindings/field_access/identity.rs');identity.write_text(identity.read_text()+'''
#[cfg(test)]
mod builtin_module_identity_tests {
    use super::*;
    use crate::entities::compiler_features::Features;
    use crate::entities::module::Module;
    use crate::entities::world_types::SysInputs;

    #[test]
    fn builtin_module_identity_copied_native_bindings_do_not_authenticate_user_module() {
        let scope = crate::compiler::stdlib::make_stdlib_with_features(
            &SysInputs::default(), Features::empty(),
        );
        let Value::Module(format) = scope.get("format").unwrap() else {
            panic!("real format module");
        };
        assert!(is_effective_format_module(format));
        let fake_format = Module::new("format", format.scope().clone());
        assert!(!is_effective_format_module(&fake_format));
        let fake_std = Module::new("global", scope);
        assert!(!is_effective_std_module(&fake_std));
        assert!(!is_effective_std_module(&fake_std.clone()));
        assert!(!is_effective_format_module(&fake_format.clone()));
    }
}
''')
module=Path('01_core/src/entities/module.rs');module.write_text(module.read_text()+'''
#[cfg(test)]
mod builtin_module_identity_layout_tests {
    use super::*;

    #[test]
    fn builtin_module_identity_public_control_and_footprint() {
        let module = Module::new("global", Scope::new());
        let clone = module.clone();
        assert_eq!(module, clone);
        assert_ne!(module, Module::new("global", Scope::new()));
        assert_eq!(format!("{module:?}"), "Module { name: \\\"global\\\" }");
        assert!(module.content().is_none());
        assert!(module.introspection_content().is_none());
        assert!(module.realization_introspector().is_none());
        assert!(module.document_info().is_empty());
        assert!(module.bibliography_styles().is_empty());
        println!("MODULE_LAYOUT {} {}", std::mem::size_of::<Module>(), std::mem::size_of::<ModuleInner>());
    }
}
''')
fields=Path('01_core/src/compiler/eval/bindings/field_access/fields.rs');parent=Path('01_core/src/compiler/eval/bindings/field_access.rs').read_text();world=parent[parent.index('    struct TestWorld {',parent.index('mod p1338_tests')):parent.index('    fn profiles()',parent.index('mod p1338_tests'))]
fields.write_text(fields.read_text()+'''
#[cfg(test)]
mod builtin_module_identity_tests {
    use crate::compiler::eval::{eval_expression_with_features, eval_with_full_error_target_and_features, EvalTarget};
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Features, Feature};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced};
    use comemo::Track;
'''+world+'''
    fn features(mask: u8) -> Features {
        let mut features = Features::empty();
        for (bit, feature) in [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)] {
            if mask & bit != 0 { features.enable(feature); }
        }
        features
    }
    fn world(text: &str) -> TestWorld {
        TestWorld { source: Source::detached(text), library: Library::new(), book: FontBook::new() }
    }

    #[test]
    fn builtin_module_identity_aliases_closures_and_independent_feature_profiles() {
        let world = world("");
        for mask in 0..8 {
            for (root, field, bit, feature) in [
                ("std", "html", 1, "html"), ("std.format", "html", 1, "html"),
                ("format", "html", 1, "html"), ("std", "asset", 4, "bundle"),
                ("std.format", "bundle", 4, "bundle"), ("format", "bundle", 4, "bundle"),
            ] {
                for expr in [format!("{root}.{field}"), format!("{{ let s = {root}; s.{field} }}"),
                    format!("{{ let s = {root}; let get() = s.{field}; let std = \\\"shadow\\\"; get() }}")] {
                    let (result, _) = eval_expression_with_features(&world, &expr, features(mask));
                    if mask & bit == 0 {
                        let errors = result.expect_err(&expr);
                        assert_eq!(errors.len(), 1);
                        assert_eq!(errors[0].message, format!("cannot access field `{field}` because the `{feature}` feature is not enabled"), "{expr}/{mask}");
                        assert_eq!(errors[0].hints, [format!("try enabling the `{feature}` feature"), "see https://typst.app/help/compiler-features for more details".into()]);
                    } else {
                        assert!(matches!(result, Ok(Value::Func(_))), "{expr}/{mask}: {result:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn builtin_module_identity_dictionaries_keep_user_fields_in_all_profiles() {
        let world = world("");
        for mask in 0..8 {
            for field in ["html", "asset", "bundle"] {
                let expr = format!("{{ let std = ({field}: 42); std.{field} }}");
                let (result, _) = eval_expression_with_features(&world, &expr, features(mask));
                assert_eq!(result.unwrap(), Value::Int(42));
            }
        }
    }

    #[test]
    fn builtin_module_identity_document_gate_anchors_before_call_or_set_arguments() {
        for text in ["#std.html", "#let s = std\\n#s.html", "#std.html(panic(\\\"argument executed\\\"))", "#set std.html(pretty: panic(\\\"argument executed\\\"))"] {
            for target in [EvalTarget::Paged, EvalTarget::Html, EvalTarget::Bundle] {
                let world = world(text);
                let mut sink = Sink::new();
                let result = eval_with_full_error_target_and_features(
                    &Routines::new(), &world, Traced::default().track(), sink.track_mut(),
                    Route::root().track(), &world.source, &ElementRegistry::new(), false, target, Features::empty(),
                );
                let errors = result.expect_err(text);
                assert_eq!(errors.len(), 1, "{text}: {errors:?}");
                assert_eq!(errors[0].message, "cannot access field `html` because the `html` feature is not enabled", "{text}");
                let start = text.find("html").unwrap();
                assert_eq!(world.source.span_byte_range(errors[0].span), Some(start..start + 4), "{text}");
                assert_eq!(errors[0].hints, ["try enabling the `html` feature", "see https://typst.app/help/compiler-features for more details"]);
                assert!(errors[0].trace.is_empty());
            }
        }
    }
}
''')
print('tests written')
