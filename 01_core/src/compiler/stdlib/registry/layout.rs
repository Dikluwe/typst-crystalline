//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/layout.md
//! @prompt-hash 1f6ccd47
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_align, native_block, native_box, native_colbreak, native_columns,
    native_flush, native_grid, native_grid_cell, native_grid_footer, native_grid_header,
    native_grid_hline, native_grid_vline, native_h, native_hide, native_layout,
    native_linebreak, native_measure, native_move, native_pad, native_page,
    native_pagebreak, native_parbreak, native_place, native_repeat, native_rotate,
    native_scale, native_skew, native_stack, native_v,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};
use std::sync::Arc;

pub(super) fn register_placement(scope: &mut Scope) {
    // P512 — `grid` com namespace para cell/header/footer/hline/vline.
    {
        let mut grid_namespace = Scope::new();
        grid_namespace
            .define("cell", Value::Func(Func::native("cell", native_grid_cell)));
        grid_namespace
            .define("header", Value::Func(Func::native("header", native_grid_header)));
        grid_namespace
            .define("footer", Value::Func(Func::native("footer", native_grid_footer)));
        grid_namespace
            .define("hline", Value::Func(Func::native("hline", native_grid_hline)));
        grid_namespace
            .define("vline", Value::Func(Func::native("vline", native_grid_vline)));
        scope.define(
            "grid",
            Value::Func(Func::native_with_namespace(
                "grid",
                native_grid,
                Arc::new(grid_namespace),
            )),
        );
    }
    // Lote F-2 S4/D4 (P335): `page(...)` função-forma legacy removida.
    scope.define("move", Value::Func(Func::native("move", native_move)));
    scope.define("rotate", Value::Func(Func::native("rotate", native_rotate)));
    scope.define("scale", Value::Func(Func::native("scale", native_scale)));
    // Passo 156F (ADR-0061 Fase 1, sub-passo 4): skew via matriz unificada.
    scope.define("skew", Value::Func(Func::native("skew", native_skew)));
    scope.define("align", Value::Func(Func::native("align", native_align)));
    {
        let mut place_namespace = Scope::new();
        place_namespace.define("flush", Value::Func(Func::native("flush", native_flush)));
        scope.define(
            "place",
            Value::Func(Func::native_with_namespace(
                "place",
                native_place,
                Arc::new(place_namespace),
            )),
        );
    }
}

pub(super) fn register_containers(scope: &mut Scope) {
    // Passo 156C (ADR-0061 Fase 1, sub-passo 1): pad + hide.
    scope.define("pad", Value::Func(Func::native("pad", native_pad)));
    scope.define("hide", Value::Func(Func::native("hide", native_hide)));
    // Passo 156D (ADR-0061 Fase 1, sub-passo 2): h + v spacing.
    scope.define("h", Value::Func(Func::native("h", native_h)));
    scope.define("v", Value::Func(Func::native("v", native_v)));
    // P1140.10 — constructor público; efeito visual de `justify` em P1140.11.
    scope.define("linebreak", Value::Func(Func::native("linebreak", native_linebreak)));
    // P1140.17 — função pública equivalente ao marker de linha vazia.
    scope.define("parbreak", Value::Func(Func::native("parbreak", native_parbreak)));
    // Passo 156E (ADR-0061 Fase 1, sub-passo 3): pagebreak manual.
    scope.define("pagebreak", Value::Func(Func::native("pagebreak", native_pagebreak)));
    // P1140.26 — o scope é também clonado para `std`, logo esta única
    // definição materializa `page` e `std.page` com o mesmo Func.
    scope.define("page", Value::Func(Func::native_with_engine("page", native_page)));
    // Passo 156G (ADR-0061 Fase 2 sub-passo 1): block container.
    scope.define("block", Value::Func(Func::native("block", native_block)));
    // Passo 156H (ADR-0061 Fase 2 sub-passo 2): box inline container.
    scope.define("box", Value::Func(Func::native("box", native_box)));
    // Passo 156I (ADR-0061 Fase 2 sub-passo 3): stack compositivo.
    // **Último sub-passo Fase 2; atinge target 72% Layout.**
    scope.define("stack", Value::Func(Func::native("stack", native_stack)));
    // Passo 156J (ADR-0061 Fase 3 sub-passo 1): repeat (paridade
    // estrutural; algoritmo dinâmico diferido per ADR-0054 graded).
    // **Primeira aplicação Fase 3.**
    scope.define("repeat", Value::Func(Func::native("repeat", native_repeat)));
    // P218 (DEBT-56 sub-fase b — Layout Fase 3): columns(count, body,
    // gutter: ?). Variant Content::Columns materializado em P217;
    // arm Layouter é stub transparente (consumer real P219).
    scope.define("columns", Value::Func(Func::native("columns", native_columns)));
    // P220 (DEBT-56 sub-fase b 4/4 — Layout Fase 3): colbreak(weak: ?).
    // Variant Content::Colbreak agregado (variant + arm + stdlib);
    // arm Layouter Opção β graded — downgrade a pagebreak literal.
    // **Fecha sub-fase (b) DEBT-56 estructuralmente.**
    scope.define("colbreak", Value::Func(Func::native("colbreak", native_colbreak)));
    // P222 (Fase 4 Layout candidata sub-passo 1; ADR-0066 §"Plano
    // promoção" Bloco C primeira materialização parcial): measure(body)
    // → Dict { width: Length, height: Length }. Helper privado
    // `measure_content` promovido a `pub(crate)`; semantic graded
    // (single-pass; runtime queries genuínas diferidas; width override
    // scope-out Opção β).
    scope.define("measure", Value::Func(Func::native("measure", native_measure)));
    // P792 — layout(func): fornece as dimensões do container via callback.
    // A lógica real vive na intercepção em eval/closures.rs (mesmo padrão
    // de measure/P712). Paridade vanilla: `layout/layout.rs:66` #[func].
    scope.define("layout", Value::Func(Func::native("layout", native_layout)));
    // P227 (ADR-0079 PROPOSTO Fase 5 Categoria A.1 sub-passo 1):
    // stroke(paint: ?, thickness: ?) constructor para Value::Stroke;
    // parametriza borders Grid/Table via Stroke shorthand parsing.
    // Valida ADR-0080 PROPOSTO N=7 → 8 (L0 não tocado em P227).
    scope.define("stroke", Value::Type(Type::Stroke));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register_placement(&mut scope);
        register_containers(&mut scope);
        let Value::Func(func) = scope.get("place").unwrap() else {
            panic!("place function");
        };
        assert_eq!(func.name(), Some("place"));
    }
}
