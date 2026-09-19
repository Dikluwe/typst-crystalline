//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/format.md
//! @prompt-hash 58e154ec
//! @layer L1
//! @updated 2026-09-19

use crate::entities::module::Module;
use crate::entities::scope::Scope;

pub(crate) fn make_format_module() -> Module {
    Module::new("format", Scope::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u01_format_module_has_exact_name() {
        let module = make_format_module();

        assert_eq!(module.name(), "format");
    }

    #[test]
    fn u02_format_module_scope_is_empty_and_has_no_children() {
        let module = make_format_module();

        assert_eq!(module.scope().len(), 0);
        for name in ["svg", "png", "pdf", "html", "absent"] {
            assert!(module.scope().get(name).is_none(), "unexpected child: {name}");
        }
    }
}
