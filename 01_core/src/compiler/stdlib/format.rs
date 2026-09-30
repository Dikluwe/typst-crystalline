//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/format.md
//! @prompt-hash 78ee2c0e
//! @layer L1
//! @updated 2026-09-20

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn native_format(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

fn projected_pdf_function(pdf_scope: &Scope, name: &str) -> Value {
    let Some(Value::Func(function)) = pdf_scope.get(name) else {
        panic!("pdf composition requires function binding `{name}`");
    };
    Value::Func(function.clone())
}

fn projected_html_function(html_scope: &Scope, name: &str) -> Value {
    let Some(Value::Func(function)) = html_scope.get(name) else {
        panic!("html composition requires function binding `{name}`");
    };
    Value::Func(function.clone())
}

pub(crate) fn make_format_module(pdf: &Value, html: &Value, asset: &Value) -> Module {
    let Value::Func(pdf) = pdf else {
        panic!("format.pdf composition requires the effective pdf function");
    };
    assert_eq!(pdf.name(), Some("pdf"), "format.pdf composition requires function `pdf`");
    let Value::Func(html) = html else {
        panic!("format.html composition requires the effective html function");
    };
    assert_eq!(
        html.name(),
        Some("html"),
        "format.html composition requires function `html`"
    );

    let mut svg_namespace = Scope::new();
    svg_namespace.define("pretty", Value::Bool(false));

    let mut png_namespace = Scope::new();
    png_namespace.define("ppi", Value::Float(144.0));

    let pdf_scope = pdf.namespace().expect("pdf function must carry a namespace");
    let gated_members = ["table-summary", "header-cell", "data-cell"];
    let gated_count = gated_members
        .iter()
        .filter(|name| pdf_scope.get(name).is_some())
        .count();
    assert!(
        matches!(gated_count, 0 | 3),
        "pdf composition requires all or none of the a11y-extras bindings"
    );

    let mut pdf_namespace = Scope::new();
    pdf_namespace.define("pages", Value::None);
    pdf_namespace.define("standard", Value::Array(vec![Value::Str("1.7".into())]));
    pdf_namespace.define("tagged", Value::Auto);
    pdf_namespace.define("pretty", Value::Bool(false));
    for name in ["attach", "artifact"] {
        pdf_namespace.define(name, projected_pdf_function(pdf_scope, name));
    }
    if gated_count == gated_members.len() {
        for name in gated_members {
            pdf_namespace.define(name, projected_pdf_function(pdf_scope, name));
        }
    }

    let html_scope = html.namespace().expect("html function must carry a namespace");
    let mut html_namespace = Scope::new();
    html_namespace.define("pretty", Value::Bool(false));
    for (name, _) in html_scope.iter() {
        html_namespace.define(name, projected_html_function(html_scope, name));
    }

    let mut scope = Scope::new();
    scope.define(
        "svg",
        Value::Func(Func::native_with_namespace(
            "svg",
            native_format,
            Arc::new(svg_namespace),
        )),
    );
    scope.define(
        "png",
        Value::Func(Func::native_with_namespace(
            "png",
            native_format,
            Arc::new(png_namespace),
        )),
    );
    scope.define(
        "pdf",
        Value::Func(Func::native_with_namespace(
            "pdf",
            native_format,
            Arc::new(pdf_namespace),
        )),
    );
    scope.define(
        "html",
        Value::Func(Func::native_with_namespace(
            "html",
            native_format,
            Arc::new(html_namespace),
        )),
    );
    let mut bundle_namespace = Scope::new();
    bundle_namespace.define("asset", asset.clone());
    scope.define(
        "bundle",
        Value::Func(Func::native_with_namespace(
            "bundle",
            native_format,
            Arc::new(bundle_namespace),
        )),
    );
    Module::new("format", scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::stdlib::html::make_html_module;
    use crate::compiler::stdlib::pdf::make_pdf_module;
    use crate::entities::compiler_features::Features;
    use crate::entities::value::Value;

    fn make_default_format_module() -> (Module, Value) {
        let pdf = make_pdf_module(Features::empty());
        let html = make_html_module();
        let asset = Value::Func(Func::native(
            "asset",
            crate::compiler::stdlib::structural::native_asset,
        ));
        (make_format_module(&pdf, &html, &asset), asset)
    }

    #[test]
    fn u01_format_module_has_exact_name() {
        let (module, _) = make_default_format_module();

        assert_eq!(module.name(), "format");
    }

    #[test]
    fn p1485_format_module_exposes_four_formats_and_bundle_asset_identity() {
        let (module, asset) = make_default_format_module();

        assert_eq!(module.scope().len(), 5);
        let Value::Func(svg) = module.scope().get("svg").expect("format.svg") else {
            panic!("format.svg must be a function");
        };
        assert_eq!(svg.name(), Some("svg"));
        let svg_namespace = svg.namespace().expect("format.svg namespace");
        assert_eq!(svg_namespace.len(), 1);
        assert_eq!(svg_namespace.get("pretty"), Some(&Value::Bool(false)));

        let Value::Func(png) = module.scope().get("png").expect("format.png") else {
            panic!("format.png must be a function");
        };
        assert_eq!(png.name(), Some("png"));
        let png_namespace = png.namespace().expect("format.png namespace");
        assert_eq!(png_namespace.len(), 1);
        assert_eq!(png_namespace.get("ppi"), Some(&Value::Float(144.0)));

        let Value::Func(pdf) = module.scope().get("pdf").expect("format.pdf") else {
            panic!("format.pdf must be a function");
        };
        assert_eq!(pdf.name(), Some("pdf"));

        let Value::Func(html) = module.scope().get("html").expect("format.html") else {
            panic!("format.html must be a function");
        };
        assert_eq!(html.name(), Some("html"));
        let html_namespace = html.namespace().expect("format.html namespace");
        let global_html = make_html_module();
        let Value::Func(global_html) = &global_html else {
            panic!("html must be a function");
        };
        let global_html_namespace = global_html.namespace().expect("html namespace");
        assert_eq!(html_namespace.len(), global_html_namespace.len() + 1);
        assert_eq!(html_namespace.get("pretty"), Some(&Value::Bool(false)));
        for (name, _) in global_html_namespace.iter() {
            assert_eq!(
                html_namespace.get(name),
                global_html_namespace.get(name),
                "format.html.{name}"
            );
        }
        let Value::Func(bundle) = module.scope().get("bundle").expect("format.bundle")
        else {
            panic!("format.bundle must be a function");
        };
        assert_eq!(bundle.name(), Some("bundle"));
        let namespace = bundle.namespace().expect("format.bundle namespace");
        assert_eq!(namespace.len(), 1);
        assert_eq!(namespace.get("asset"), Some(&asset));
    }
}

/// P1471 — RED isolado (Agente B): item 5 do passo (identidade preservada
/// entre `format.pdf`/`format.html` e as raízes globais `pdf`/`html`),
/// escrito a partir de `compiler/stdlib/pdf.md`/`html.md`/`format.md`
/// P1471. Nenhuma implementação foi consultada.
///
/// Deliberadamente testado só pelo observável de língua
/// (`eval_expression_with_features` + `==` sobre `Value::Func`), **não**
/// por reconstrução Rust-level via `make_pdf_module`/`make_html_module`:
/// o L0 declara que a assinatura de `make_html_module` muda de `-> Module`
/// para `-> Value` (espelhando `make_pdf_module`) — qualquer teste que
/// dependa dessa forma interna quebraria a compilação (não a asserção)
/// assim que o Agente A materializar a mudança, e deixaria de servir como
/// oráculo GREEN. Pela mesma razão de ADR-0107 (paridade é com a língua,
/// não com a mecânica), a via de língua é também a mais correta aqui.
///
/// Nota de medição (ADR-0108): a igualdade `format.pdf.attach == pdf.attach`
/// já é verdadeira nesta baseline — `Func::eq` compara nativas por nome
/// (`01_core/src/entities/func.rs:384-396`), e `projected_pdf_function`
/// já clona o mesmo binding nativo independentemente do kind da raiz
/// `pdf`. Este teste não é RED pela igualdade em si; é mantido porque o
/// passo pede a cobertura explícita do item 5, e serve de não-regressão
/// (deve continuar GREEN antes e depois, como o item 7 em
/// `field_access.rs`).
#[cfg(test)]
mod p1471_tests {
    use crate::compiler::eval::eval_expression_with_features;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }

    impl crate::contracts::world::World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn world() -> TestWorld {
        TestWorld {
            source: Source::detached(""),
            library: Library::new(),
            book: FontBook::new(),
        }
    }

    fn profiles() -> Vec<Features> {
        (0..4u8)
            .map(|mask| {
                let mut features = Features::empty();
                if mask & 1 != 0 {
                    features.enable(Feature::Html);
                }
                if mask & 2 != 0 {
                    features.enable(Feature::A11yExtras);
                }
                features
            })
            .collect()
    }

    fn assert_identity(world: &TestWorld, expr: &str, features: Features) {
        let (result, warnings) = eval_expression_with_features(world, expr, features);
        assert!(warnings.is_empty(), "{expr} em {features:?}: {warnings:?}");
        assert_eq!(result.unwrap(), Value::Bool(true), "{expr} em {features:?}");
    }

    /// `format.pdf.attach`/`format.pdf.artifact` são a mesma função (não
    /// clone com nome igual) que `pdf.attach`/`pdf.artifact`, nos quatro
    /// perfis; com `A11yExtras`, o mesmo vale para o trio gated.
    #[test]
    fn p1471_format_pdf_members_share_identity_with_global_pdf() {
        let world = world();
        for features in profiles() {
            assert_identity(&world, "format.pdf.attach == pdf.attach", features);
            assert_identity(&world, "format.pdf.artifact == pdf.artifact", features);
            if features.contains(Feature::A11yExtras) {
                for member in ["table-summary", "header-cell", "data-cell"] {
                    let expr = format!("format.pdf.{member} == pdf.{member}");
                    assert_identity(&world, &expr, features);
                }
            }
        }
    }

    /// `format.html.elem`/`format.html.frame` e uma amostra de tag tipada
    /// (`div`) são a mesma função que `html.elem`/`html.frame`/`html.div`,
    /// nos perfis em que `Feature::Html` está ativa (sem a feature, nem o
    /// identificador `html` nem o campo `format.html` são acessíveis).
    #[test]
    fn p1471_format_html_members_share_identity_with_global_html() {
        let world = world();
        for features in profiles() {
            if !features.contains(Feature::Html) {
                continue;
            }
            assert_identity(&world, "format.html.elem == html.elem", features);
            assert_identity(&world, "format.html.frame == html.frame", features);
            assert_identity(&world, "format.html.div == html.div", features);
        }
    }
}
