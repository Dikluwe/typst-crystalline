//! Private helper-domain contracts, separate from public nominal pipeline tests.
//! C supplies the frozen sibling snapshot.rs or its directed mutant in RAM.
pub use typst_core::{contracts, entities};
use contracts::world::SysInputs;
use entities::compiler_features::{Feature, Features};
use entities::scope::Scope;
use entities::value::Value;
use std::cell::RefCell;
use std::sync::Arc;

fn make_stdlib_with_features(inputs: &SysInputs, features: Features) -> Scope {
    typst_core::compiler::stdlib::make_stdlib_with_features(inputs, features)
}
#[path = "../../watch-startup-neutral/snapshot.rs"]
mod snapshot;

fn flags(mask: u8) -> Features {
    let mut out = Features::empty();
    for (bit, feature) in [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)] {
        if mask & bit != 0 { out.enable(feature); }
    }
    out
}
fn inputs(count: usize) -> SysInputs {
    let mut inputs = SysInputs::default();
    let text = "á中🦀".repeat(28);
    for index in 0..count { inputs.insert(format!("key{index:05}").into(), format!("{index}:{text}").into()); }
    inputs
}
fn seed(inputs: &SysInputs, features: Features) -> RefCell<Option<Arc<snapshot::StdlibSnapshot>>> {
    let scope = make_stdlib_with_features(inputs, features);
    RefCell::new(Some(Arc::new(snapshot::StdlibSnapshot::new(inputs.clone(), features, scope))))
}
fn assert_inputs(scope: &Scope, expected: &SysInputs) {
    let Some(Value::Module(sys)) = scope.get("sys") else { panic!("sys module"); };
    let Some(Value::Dict(actual)) = sys.scope().get("inputs") else { panic!("sys.inputs dict"); };
    assert_eq!(actual.len(), expected.len());
    for (index, ((actual_key, actual_value), (wanted_key, wanted_value))) in actual.iter().zip(expected.iter()).enumerate() {
        let Value::Str(actual_value) = actual_value else { panic!("string at pair {index}"); };
        assert!(actual_key.as_str() == wanted_key.as_str(), "key/order mismatch at pair {index}");
        assert!(actual_value.as_str() == wanted_value.as_str(), "value mismatch at pair {index}");
    }
}
fn assert_features(scope: &Scope, features: Features) {
    let reference = make_stdlib_with_features(&SysInputs::default(), features);
    for name in ["asset", "html", "pdf", "png", "svg", "format"] {
        assert_eq!(scope.get(name).is_some(), reference.get(name).is_some(), "binding {name}");
    }
    let Some(Value::Module(actual_format)) = scope.get("format") else { panic!("format module"); };
    let Some(Value::Module(reference_format)) = reference.get("format") else { panic!("reference format module"); };
    for name in ["html", "bundle", "pdf", "png", "svg"] {
        assert_eq!(actual_format.scope().get(name).is_some(), reference_format.scope().get(name).is_some(), "format.{name}");
    }
    let Some(Value::Func(actual_pdf)) = scope.get("pdf") else { panic!("pdf function"); };
    let Some(Value::Func(reference_pdf)) = reference.get("pdf") else { panic!("reference pdf function"); };
    for name in ["table-summary", "header-cell", "data-cell"] {
        assert_eq!(actual_pdf.namespace().unwrap().get(name).is_some(), reference_pdf.namespace().unwrap().get(name).is_some(), "pdf.{name}");
    }
}
fn raw_line(scope: &Scope) -> Value {
    let Some(Value::Func(raw)) = scope.get("raw") else { panic!("raw function"); };
    raw.namespace().unwrap().get("line").unwrap().clone()
}
fn assert_aliases(scope: &Scope) {
    // The registry builder returns the scope before eval installs its std
    // wrapper. Test only aliases actually composed by registry/formats here.
    let Some(Value::Module(format)) = scope.get("format") else { panic!("format module"); };
    assert!(scope.get("pdf").unwrap() == format.scope().get("pdf").unwrap(), "within-file pdf/format.pdf alias");
    if let Some(asset) = scope.get("asset") {
        let Some(Value::Func(bundle)) = format.scope().get("bundle") else { panic!("format.bundle function"); };
        assert!(Some(asset) == bundle.namespace().unwrap().get("asset"), "within-file asset/format.bundle.asset alias");
    }
}

#[test]
fn seeded_helper_guard_checks_all_feature_transitions_without_claiming_pipeline_reachability() {
    let input = inputs(64);
    for original in 0..8 {
        let slot = seed(&input, flags(original));
        for requested in 0..8 {
            let (scope, _) = snapshot::resolve(&slot, &input, flags(requested), 3);
            assert_inputs(&scope, &input); assert_features(&scope, flags(requested)); assert_aliases(&scope);
            let (back, _) = snapshot::resolve(&slot, &input, flags(original), 3);
            assert_features(&back, flags(original)); assert_inputs(&back, &input);
        }
    }
}

#[test]
fn seeded_helper_guard_checks_mebibyte_last_pair_and_order_without_replacing_parent_seed() {
    let original = inputs(4096);
    let mut modified = original.clone();
    let key = modified.keys().last().unwrap().clone();
    let replacement = modified.get(&key).unwrap().replacen("á", "é", 1);
    modified.insert(key, replacement.into());
    let mut reverse = SysInputs::default();
    for (key, value) in original.iter().rev() { reverse.insert(key.clone(), value.clone()); }
    let slot = seed(&original, Features::empty());
    for requested in [&modified, &reverse] {
        let (scope, child_seed) = snapshot::resolve(&slot, requested, Features::empty(), 3);
        assert_inputs(&scope, requested); assert_aliases(&scope);
        let (parent, _) = snapshot::resolve(&slot, &original, Features::empty(), 3);
        assert_inputs(&parent, &original); assert_aliases(&parent);
        assert!(scope.get("calc").unwrap() != parent.get("calc").unwrap(), "scope module instances remain independent");
        // The returned option belongs to a child context, never overwrites the
        // already seeded parent. Exercise the real returned child seed if any.
        let child = RefCell::new(child_seed);
        let (child_scope, _) = snapshot::resolve(&child, requested, Features::empty(), 3);
        assert_inputs(&child_scope, requested); assert_aliases(&child_scope);
    }
}

#[test]
fn seeded_helper_reinstantiates_language_identities_but_preserves_local_aliases() {
    let input = inputs(1); let slot = seed(&input, Features::empty());
    let (first, _) = snapshot::resolve(&slot, &input, Features::empty(), 3);
    let (second, _) = snapshot::resolve(&slot, &input, Features::empty(), 3);
    assert_aliases(&first); assert_aliases(&second);
    for name in ["calc", "sym", "emoji", "math", "sys", "format"] {
        assert!(first.get(name).unwrap() != second.get(name).unwrap(), "cross-file identity {name}");
    }
    assert!(raw_line(&first) != raw_line(&second), "raw.line must be per file");
}
