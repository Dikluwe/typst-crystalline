use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{env, fs};
use typst::Features;
use typst::LibraryExt;
use typst::foundations::{Module, Repr, SilentBindingGuard, Value};

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
    kind: String,
    params: Option<Vec<Param>>,
    source: Option<String>,
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/p1140-vanilla.json"));
    let library = typst::Library::default();
    let guard = SilentBindingGuard::new(Features::all());
    let mut catalog = BTreeMap::new();
    walk_module(&library.global, "", &guard, &mut catalog, 0);
    let json = serde_json::to_string_pretty(&catalog).expect("serialize catalog");
    fs::write(&output, format!("{json}\n")).expect("write catalog");
    eprintln!("wrote {} entries to {}", catalog.len(), output.display());
}

fn walk_module(
    module: &Module,
    prefix: &str,
    guard: &SilentBindingGuard,
    out: &mut BTreeMap<String, Side>,
    depth: usize,
) {
    if depth > 8 {
        return;
    }
    for (name, binding) in module.scope().iter() {
        let Ok(value) = binding.read(guard) else { continue };
        let path = join(prefix, name.as_str());
        if out.contains_key(&path) {
            continue;
        }
        let mut side = Side {
            present: true,
            kind: value.ty().short_name().into(),
            params: None,
            source: None,
        };
        match value {
            Value::Func(func) => {
                side.params = Some(
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
                        .collect(),
                );
                side.source = func.def_site().map(|s| format!("{}:{}", s.path, s.key));
            }
            Value::Type(ty) => {
                side.source =
                    Some(format!("{}:{}", ty.def_site().path, ty.def_site().key))
            }
            _ => {}
        }
        out.insert(path.clone(), side);
        match value {
            Value::Module(child) => walk_module(child, &path, guard, out, depth + 1),
            Value::Func(func) => {
                if let Some(scope) = func.scope() {
                    walk_module(
                        &Module::anonymous(scope.clone()),
                        &path,
                        guard,
                        out,
                        depth + 1,
                    );
                }
            }
            Value::Type(ty) => walk_module(
                &Module::anonymous(ty.scope().clone()),
                &path,
                guard,
                out,
                depth + 1,
            ),
            _ => {}
        }
    }
}

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() { name.into() } else { format!("{prefix}.{name}") }
}
