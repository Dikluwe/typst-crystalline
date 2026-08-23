//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/sys.md
//! @prompt-hash 13598674
//! @layer L1
//! @updated 2026-07-21
//!
//! Módulo builtin `sys` (P694) — `sys.version` e `sys.inputs`.
//!
//! Paridade com a linguagem (ADR-0107): `sys.version` é a versão de **paridade**
//! `version(0, 15, 1)` (não a versão do binário cristalino); `sys.inputs` é um
//! `dict` str→str populado via `--input chave=valor` (vazio por omissão). A
//! forma impressa de `#sys` é um dicionário (como `calc`/`sym`), não
//! `<module sys>` — divergência de repr/mecânica aceite.
//!
//! Passo 796: `PARITY_VERSION` passa a viver em `entities::version` (fonte
//! única, também consumida por `--version` do CLI); aqui só se importa.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::contracts::world::SysInputs;
use crate::entities::value::Value;
use crate::entities::version::{Version, PARITY_VERSION};

/// Constrói o módulo `sys` como `Value::Module` com `version` e `inputs`.
///
/// `inputs` é o `SysInputs` resolvido (lido de `World::inputs()` em
/// `eval_with_full_error`); vazio por omissão. Os valores viram `Value::Str`
/// (paridade vanilla: `--input n=42` → `sys.inputs.n == "42"`).
///
/// **P731** — passou de `Value::Dict` a `Value::Module` (paridade vanilla —
/// medido: `type(sys)` → `module`).
pub fn make_sys_module(inputs: &SysInputs) -> Value {
    let (maj, min, pat) = PARITY_VERSION;

    let mut inputs_dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    for (k, v) in inputs {
        inputs_dict.insert(k.clone(), Value::Str(v.clone()));
    }

    let mut scope = crate::entities::scope::Scope::new();
    scope.define("version", Value::from(Version::new(maj, min, pat)));
    scope.define("inputs", Value::Dict(inputs_dict));
    Value::Module(crate::entities::module::Module::new("sys", scope))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version_components(v: &Value) -> Vec<u64> {
        match v {
            Value::Version(arc) => arc.components.clone(),
            other => panic!("esperava Version, recebeu {}", other.type_name()),
        }
    }

    #[test]
    fn sys_sem_inputs_tem_version_e_inputs_vazio() {
        let m = make_sys_module(&SysInputs::default());
        let scope = match &m {
            Value::Module(m) => m.scope(),
            other => panic!("esperava Module, recebeu {}", other.type_name()),
        };
        // P1137: version == version(0, 15, 1), vanilla ratificado a51e02804.
        let version = scope.get("version").expect("sys.version em falta");
        assert_eq!(version_components(version), vec![0, 15, 1]);
        // inputs == (:)
        let inputs = scope.get("inputs").expect("sys.inputs em falta");
        match inputs {
            Value::Dict(d) => assert!(d.is_empty()),
            other => panic!("esperava Dict em sys.inputs, recebeu {}", other.type_name()),
        }
    }

    #[test]
    fn sys_com_inputs_converte_para_str() {
        let mut inputs = SysInputs::default();
        inputs.insert(EcoString::from("chave"), EcoString::from("valor"));
        inputs.insert(EcoString::from("n"), EcoString::from("42"));

        let m = make_sys_module(&inputs);
        let scope = match &m {
            Value::Module(m) => m.scope(),
            _ => panic!("esperava Module"),
        };
        let inputs_dict = match scope.get("inputs").unwrap() {
            Value::Dict(d) => d,
            _ => panic!("esperava Dict em sys.inputs"),
        };
        assert_eq!(inputs_dict.get("chave"), Some(&Value::Str(EcoString::from("valor"))));
        // n=42 é string "42", não Int 42 (paridade vanilla).
        assert_eq!(inputs_dict.get("n"), Some(&Value::Str(EcoString::from("42"))));
    }
}
