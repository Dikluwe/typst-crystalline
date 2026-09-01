use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use typst::LibraryExt;
use typst::foundations::{Module, Repr, SilentBindingGuard, Value};
use typst::{Feature, Features};

#[derive(Clone, Debug, Serialize)]
struct Param {
    name: Option<String>,
    default: Option<String>,
    positional: bool,
    named: bool,
    variadic: bool,
    required: bool,
    settable: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Side {
    present: bool,
    availability: String,
    kind: String,
    params: Option<Vec<Param>>,
    source: Option<String>,
    owner_path: Option<String>,
    owner_kind: Option<String>,
    slot_kind: String,
    access_form: String,
    structural_verified: bool,
    symbol: Option<SymbolMeta>,
}

#[derive(Clone, Debug, Serialize)]
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
    vanilla_revision: &'static str,
    entries: &'a BTreeMap<String, Side>,
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/p1140-vanilla.json"));
    let profile = env::args().nth(2).unwrap_or_else(|| "default".into());
    let product_binary = env::args_os()
        .nth(3)
        .map(PathBuf::from)
        .expect("third argument must be the vanilla product binary");
    let features = match profile.as_str() {
        "default" => Features::none(),
        "html" => [Feature::Html].into_iter().collect(),
        other => panic!("unknown profile {other:?}; expected default or html"),
    };
    let library = typst::Library::default();
    let guard = SilentBindingGuard::new(features);
    let mut catalog = BTreeMap::new();
    walk_module(&library.global, "", None, &guard, &mut catalog, 0);
    let envelope = Catalog {
        schema_version: "p1282-catalog-v1",
        side: "vanilla",
        profile: &profile,
        features: if profile == "html" { vec!["html"] } else { vec![] },
        product_sha256: sha256sum(&product_binary),
        vanilla_revision: "a51e02804",
        entries: &catalog,
    };
    let json = serde_json::to_string_pretty(&envelope).expect("serialize catalog");
    fs::write(&output, format!("{json}\n")).expect("write catalog");
    eprintln!("wrote {} entries to {}", catalog.len(), output.display());
}

fn walk_module(
    module: &Module,
    prefix: &str,
    owner_kind: Option<&str>,
    guard: &SilentBindingGuard,
    out: &mut BTreeMap<String, Side>,
    depth: usize,
) {
    if depth > 8 {
        record_truncation(prefix, owner_kind, out);
        return;
    }
    for (name, binding) in module.scope().iter() {
        let Ok(value) = binding.read(guard) else { continue };
        let path = join(prefix, name.as_str());
        if out.contains_key(&path) {
            continue;
        }
        let params = match &value {
            Value::Func(func) => Some(
                func.params()
                    .map(|p| Param {
                        name: p.name().map(str::to_owned),
                        default: p.default().map(|v| v.repr().to_string()),
                        positional: p.positional(),
                        named: p.named(),
                        variadic: p.variadic(),
                        required: p.required(),
                        settable: p.settable(),
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        };
        let access_form = if prefix.is_empty() {
            "not_applicable"
        } else if owner_kind == Some("type") {
            if params
                .as_ref()
                .and_then(|params| params.first())
                .and_then(|param| param.name.as_deref())
                == Some("self")
            {
                "instance_method"
            } else {
                "type_static_member"
            }
        } else {
            "module_member"
        };
        let mut side = Side {
            present: true,
            availability: "active".into(),
            kind: value.ty().short_name().into(),
            params,
            source: None,
            owner_path: (!prefix.is_empty()).then(|| prefix.to_owned()),
            owner_kind: owner_kind.map(str::to_owned),
            slot_kind: if prefix.is_empty() {
                "global_binding".into()
            } else {
                "member".into()
            },
            access_form: access_form.into(),
            structural_verified: true,
            symbol: None,
        };
        match &value {
            Value::Func(func) => {
                side.source = func.def_site().map(|s| format!("{}:{}", s.path, s.key));
            }
            Value::Type(ty) => {
                side.source =
                    Some(format!("{}:{}", ty.def_site().path, ty.def_site().key))
            }
            Value::Symbol(symbol) => {
                side.symbol = Some(SymbolMeta {
                    value: symbol.get().to_owned(),
                    variants: symbol
                        .variants()
                        .map(|(modifiers, value, _)| {
                            let modifiers = modifiers
                                .into_iter()
                                .filter(|modifier| !modifier.is_empty())
                                .collect::<Vec<_>>()
                                .join(".");
                            (modifiers, value.to_owned())
                        })
                        .collect(),
                });
            }
            _ => {}
        }
        out.insert(path.clone(), side);
        match value {
            Value::Module(child) => {
                walk_module(&child, &path, Some("module"), guard, out, depth + 1)
            }
            Value::Func(func) => {
                if let Some(scope) = func.scope() {
                    walk_module(
                        &Module::anonymous(scope.clone()),
                        &path,
                        Some("function"),
                        guard,
                        out,
                        depth + 1,
                    );
                }
            }
            Value::Type(ty) => walk_module(
                &Module::anonymous(ty.scope().clone()),
                &path,
                Some("type"),
                guard,
                out,
                depth + 1,
            ),
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
        source: Some("enumerator:depth-limit".into()),
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

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.into() } else { format!("{prefix}.{name}") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst::foundations::Scope;

    #[test]
    fn depth_limit_emits_unknown_sentinel() {
        let module = Module::anonymous(Scope::new());
        let guard = SilentBindingGuard::new(Features::none());
        let mut catalog = BTreeMap::new();
        walk_module(&module, "deep", Some("module"), &guard, &mut catalog, 9);

        let sentinel = catalog
            .get("deep.__p1282_scope_truncated__")
            .expect("depth truncation must remain observable");
        assert!(!sentinel.present);
        assert_eq!(sentinel.availability, "unknown");
        assert!(!sentinel.structural_verified);
    }
}
