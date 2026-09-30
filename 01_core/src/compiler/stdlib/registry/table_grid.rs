//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/table_grid.md
//! @prompt-hash e5bea3eb
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_grid_cell, native_grid_footer, native_grid_header, native_table,
    native_table_cell, native_table_footer, native_table_header, native_table_hline,
    native_table_vline,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::Value;
use std::sync::Arc;

pub(super) fn register(scope: &mut Scope) {
    // Passo 157A (ADR-0060 Fase 2 sub-passo 1): table minimal
    // (subset 3 fields; reusa layout_grid; TableCell/Header/Footer
    // diferidos para P157B/C). **Primeiro sub-passo Model Fase 2.**
    // P493b — table com namespace anexado para table.header/footer/cell.
    // P512 — adiciona table.hline / table.vline.
    let mut table_namespace = Scope::new();
    table_namespace
        .define("header", Value::Func(Func::native("header", native_table_header)));
    table_namespace
        .define("footer", Value::Func(Func::native("footer", native_table_footer)));
    table_namespace.define("cell", Value::Func(Func::native("cell", native_table_cell)));
    table_namespace
        .define("hline", Value::Func(Func::native("hline", native_table_hline)));
    table_namespace
        .define("vline", Value::Func(Func::native("vline", native_table_vline)));
    scope.define(
        "table",
        Value::Func(Func::native_with_namespace(
            "table",
            native_table,
            Arc::new(table_namespace),
        )),
    );
    // Passo 157B (ADR-0060 Fase 2 sub-passo 2): table cell
    // (subset 5 fields; ADR-0064 Caso A para x/y, Caso C para
    // colspan/rowspan; placement diferido em DEBT-34e).
    // Mantém bindings flat como fallback não-regressão.
    scope
        .define("table_cell", Value::Func(Func::native("table_cell", native_table_cell)));
    // Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha "table foundations"):
    // par simétrico TableHeader/TableFooter. ADR-0064 Caso D para
    // `repeat: bool` default true (primeira aplicação Caso D em
    // Model). Algoritmo de repetição em page breaks diferido em
    // DEBT-56 (refactor multi-region). Mantém bindings flat como fallback.
    scope.define(
        "table_header",
        Value::Func(Func::native("table_header", native_table_header)),
    );
    scope.define(
        "table_footer",
        Value::Func(Func::native("table_footer", native_table_footer)),
    );
    // P224 (ADR-0061 Fase 4 Layout candidata sub-passo 3 — fecha série α
    // "terminar Layout"): grid_cell + grid_header + grid_footer paridade
    // P157B/C literal; grid_cell resolve placement real via P224.C
    // grid_placement.rs (fecha DEBT-34e).
    scope.define("grid_cell", Value::Func(Func::native("grid_cell", native_grid_cell)));
    scope.define(
        "grid_header",
        Value::Func(Func::native("grid_header", native_grid_header)),
    );
    scope.define(
        "grid_footer",
        Value::Func(Func::native("grid_footer", native_grid_footer)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("table").unwrap() else {
            panic!("table function");
        };
        assert_eq!(func.name(), Some("table"));
    }
}
