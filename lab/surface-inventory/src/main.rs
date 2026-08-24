use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{env, fs};
use typst_core::entities::value::Value;
use typst_infra::pipeline::eval_expression_with_sink;
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
    kind: String,
    params: Option<Vec<Param>>,
    source: String,
}

fn main() {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/p1140-crystalline.json"));
    let world = SystemWorld::for_eval(env::current_dir().expect("current directory"));
    let (result, _) = eval_expression_with_sink(&world, "std");
    let Value::Module(module) = result.expect("evaluate std") else {
        panic!("std did not evaluate to a module");
    };
    let mut catalog = BTreeMap::new();
    walk_module(&module, "", &mut catalog, 0);
    if let Some(vanilla_path) = env::args_os().nth(2).map(PathBuf::from) {
        let vanilla: BTreeMap<String, serde_json::Value> = serde_json::from_slice(
            &fs::read(vanilla_path).expect("read vanilla catalog"),
        )
        .expect("parse vanilla catalog");
        for path in vanilla.keys() {
            if catalog.contains_key(path) {
                continue;
            }
            let (result, _) = eval_expression_with_sink(&world, path);
            if let Ok(value) = result {
                catalog.insert(
                    path.clone(),
                    Side {
                        present: true,
                        kind: value.type_name().into(),
                        params: None,
                        source: format!("runtime:probe:{path}"),
                    },
                );
            }
        }
    }
    let json = serde_json::to_string_pretty(&catalog).expect("serialize catalog");
    fs::write(&output, format!("{json}\n")).expect("write catalog");
    eprintln!("wrote {} entries to {}", catalog.len(), output.display());
}

fn walk_module(
    module: &typst_core::entities::module::Module,
    prefix: &str,
    out: &mut BTreeMap<String, Side>,
    depth: usize,
) {
    if depth > 8 {
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
                kind: value.type_name().into(),
                params: None,
                source: "runtime:std".into(),
            },
        );
        match value {
            Value::Module(child) => walk_module(child, &path, out, depth + 1),
            Value::Func(func) => {
                if let Some(scope) = func.namespace() {
                    let child =
                        typst_core::entities::module::Module::new("", scope.clone());
                    walk_module(&child, &path, out, depth + 1);
                }
            }
            _ => {}
        }
    }
}

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.into()
    } else {
        format!("{prefix}.{name}")
    }
}
