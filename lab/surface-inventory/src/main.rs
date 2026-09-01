use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use typst_core::entities::value::Value;
use typst_infra::world::SystemWorld;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Param {
    name: Option<String>,
    default: Option<String>,
    positional: bool,
    named: bool,
    variadic: bool,
    required: bool,
    settable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Side {
    present: bool,
    availability: String,
    kind: String,
    params: Option<Vec<Param>>,
    source: String,
    owner_path: Option<String>,
    owner_kind: Option<String>,
    slot_kind: String,
    access_form: String,
    structural_verified: bool,
    symbol: Option<SymbolMeta>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SymbolMeta {
    value: String,
    variants: Vec<(String, String)>,
}

#[derive(Serialize)]
struct Catalog<'a> {
    schema_version: &'static str,
    side: &'static str,
    profile: &'a str,
    features: Vec<&'static str>,
    product_sha256: String,
    vanilla_revision: Option<&'static str>,
    entries: &'a BTreeMap<String, Side>,
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/p1140-crystalline.json"));
    let vanilla_path = env::args_os().nth(2).map(PathBuf::from);
    let profile = env::args().nth(3).unwrap_or_else(|| "default".into());
    let product_binary = env::args_os()
        .nth(5)
        .map(PathBuf::from)
        .expect("fifth argument must be the crystalline product binary");
    let features = match profile.as_str() {
        "default" => typst_core::entities::html::Features::default(),
        "html" => typst_core::entities::html::Features::html(),
        other => panic!("unknown profile {other:?}; expected default or html"),
    };
    let world = SystemWorld::for_eval(env::current_dir().expect("current directory"));
    let (result, _) = typst_infra::pipeline::eval_expression_with_sink_features(
        &world, "std", features,
    );
    let Value::Module(module) = result.expect("evaluate std") else {
        panic!("std did not evaluate to a module");
    };
    let mut discovered = BTreeMap::new();
    walk_module(&module, "", None, &mut discovered, 0);
    let mut candidates = discovered.keys().cloned().collect::<BTreeSet<_>>();
    let mut vanilla_hints = BTreeMap::new();
    if let Some(vanilla_path) = vanilla_path {
        let vanilla_payload: serde_json::Value = serde_json::from_slice(
            &fs::read(vanilla_path).expect("read vanilla catalog"),
        )
        .expect("parse vanilla catalog");
        let vanilla_catalog: BTreeMap<String, serde_json::Value> =
            serde_json::from_value(
                vanilla_payload
                    .get("entries")
                    .cloned()
                    .expect("vanilla catalog envelope must contain entries"),
            )
            .expect("parse vanilla catalog entries");
        candidates.extend(vanilla_catalog.keys().cloned());
        vanilla_hints.extend(vanilla_catalog);
    }
    if let Some(seed_path) = env::args_os().nth(4).map(PathBuf::from) {
        let seeds: Vec<String> =
            serde_json::from_slice(&fs::read(seed_path).expect("read extra seed list"))
                .expect("parse extra seed list");
        candidates.extend(seeds);
    }
    let mut catalog = BTreeMap::new();
    for path in candidates {
        let (result, _) = typst_infra::pipeline::eval_expression_with_sink_features(
            &world, &path, features,
        );
        let Ok(value) = result else { continue };
        if let Some(mut side) = discovered.remove(&path) {
            side.kind = value.type_name().into();
            side.symbol = symbol_meta(&value);
            catalog.insert(path, side);
        } else {
            let hint = vanilla_hints.get(&path);
            catalog.insert(path.clone(), probed_side(&path, &value, hint));
        }
    }
    let envelope = Catalog {
        schema_version: "p1282-catalog-v1",
        side: "crystalline",
        profile: &profile,
        features: if profile == "html" { vec!["html"] } else { vec![] },
        product_sha256: sha256sum(&product_binary),
        vanilla_revision: None,
        entries: &catalog,
    };
    let json = serde_json::to_string_pretty(&envelope).expect("serialize catalog");
    fs::write(&output, format!("{json}\n")).expect("write catalog");
    eprintln!("wrote {} entries to {}", catalog.len(), output.display());
}

fn walk_module(
    module: &typst_core::entities::module::Module,
    prefix: &str,
    owner_kind: Option<&str>,
    out: &mut BTreeMap<String, Side>,
    depth: usize,
) {
    if depth > 8 {
        record_truncation(prefix, owner_kind, out);
        return;
    }
    for (name, binding) in module.scope().iter() {
        let value = binding.value();
        let path = join(prefix, name);
        if out.contains_key(&path) {
            continue;
        }
        out.insert(
            path.clone(),
            Side {
                present: true,
                availability: "active".into(),
                kind: value.type_name().into(),
                params: None,
                source: "runtime:std".into(),
                owner_path: (!prefix.is_empty()).then(|| prefix.to_owned()),
                owner_kind: owner_kind.map(str::to_owned),
                slot_kind: if prefix.is_empty() {
                    "global_binding".into()
                } else {
                    "member".into()
                },
                access_form: if prefix.is_empty() {
                    "not_applicable".into()
                } else {
                    "module_member".into()
                },
                structural_verified: true,
                symbol: symbol_meta(value),
            },
        );
        match value {
            Value::Module(child) => {
                walk_module(child, &path, Some("module"), out, depth + 1)
            }
            Value::Func(func) => {
                if let Some(scope) = func.namespace() {
                    let child =
                        typst_core::entities::module::Module::new("", scope.clone());
                    walk_module(&child, &path, Some("function"), out, depth + 1);
                }
            }
            _ => {}
        }
    }
}

fn record_truncation(
    prefix: &str,
    owner_kind: Option<&str>,
    out: &mut BTreeMap<String, Side>,
) {
    let path = join(prefix, "__p1282_scope_truncated__");
    out.entry(path).or_insert_with(|| Side {
        present: false,
        availability: "unknown".into(),
        kind: "unknown".into(),
        params: None,
        source: "enumerator:depth-limit".into(),
        owner_path: Some(prefix.to_owned()),
        owner_kind: owner_kind.map(str::to_owned),
        slot_kind: "scope_truncation".into(),
        access_form: "unobserved".into(),
        structural_verified: false,
        symbol: None,
    });
}

fn sha256sum(path: &PathBuf) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("execute sha256sum");
    assert!(output.status.success(), "sha256sum failed for {}", path.display());
    String::from_utf8(output.stdout)
        .expect("sha256sum output is UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256sum emitted a digest")
        .to_owned()
}

fn probed_side(path: &str, value: &Value, hint: Option<&serde_json::Value>) -> Side {
    let owner_path = hint
        .and_then(|hint| hint.get("owner_path"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let owner_kind = hint
        .and_then(|hint| hint.get("owner_kind"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let slot_kind = hint
        .and_then(|hint| hint.get("slot_kind"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(if owner_path.is_some() { "member" } else { "global_binding" })
        .to_owned();
    let access_form = hint
        .and_then(|hint| hint.get("access_form"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(if owner_path.is_some() { "module_member" } else { "not_applicable" })
        .to_owned();
    Side {
        present: true,
        availability: "active".into(),
        kind: value.type_name().into(),
        params: None,
        source: format!("runtime:probe:{path}"),
        owner_path,
        owner_kind,
        slot_kind,
        access_form,
        structural_verified: false,
        symbol: symbol_meta(value),
    }
}

fn symbol_meta(value: &Value) -> Option<SymbolMeta> {
    let Value::Symbol(symbol) = value else { return None };
    Some(SymbolMeta {
        value: symbol.value.to_string(),
        variants: symbol
            .variants
            .iter()
            .map(|(modifiers, value)| (modifiers.to_string(), value.to_string()))
            .collect(),
    })
}

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.into()
    } else {
        format!("{prefix}.{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::module::Module;
    use typst_core::entities::scope::Scope;

    #[test]
    fn depth_limit_emits_unknown_sentinel() {
        let module = Module::new("", Scope::new());
        let mut catalog = BTreeMap::new();
        walk_module(&module, "deep", Some("module"), &mut catalog, 9);

        let sentinel = catalog
            .get("deep.__p1282_scope_truncated__")
            .expect("depth truncation must remain observable");
        assert!(!sentinel.present);
        assert_eq!(sentinel.availability, "unknown");
        assert!(!sentinel.structural_verified);
    }
}
