//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/mod.md
//! @prompt-hash a7adfea8
//! @layer L1
//! @updated 2026-09-26

use crate::contracts::world::SysInputs;
use crate::entities::compiler_features::Features;
use crate::entities::scope::Scope;

mod contextual;
mod data;
mod diagnostics;
mod figure_image;
mod formats;
mod foundations;
mod layout;
mod math;
mod modules;
mod shapes;
mod structural;
mod table_grid;
mod text;

pub(crate) fn make_stdlib_with_features(inputs: &SysInputs, features: Features) -> Scope {
    let mut scope = Scope::new();
    let pdf = formats::register(&mut scope, features);
    foundations::register_types(&mut scope);
    structural::register_outline(&mut scope);
    text::register_decorations(&mut scope);
    figure_image::register(&mut scope);
    data::register(&mut scope);
    shapes::register(&mut scope);
    layout::register_placement(&mut scope);
    diagnostics::register(&mut scope);
    contextual::register(&mut scope);
    text::register_case(&mut scope);
    structural::register_document(&mut scope);
    math::register_globals(&mut scope);
    layout::register_containers(&mut scope);
    table_grid::register(&mut scope);
    structural::register_references(&mut scope);
    modules::register(&mut scope, inputs, pdf);
    foundations::register_constants(&mut scope);
    scope
}

mod tests;

mod microbenchmarks;
