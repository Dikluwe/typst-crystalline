// 💎 Tipologia Tekt: L3 (Implementação de I/O)
// Implementação concreta do contrato IIdeEnv usando o IdeWorld legado.

use ecow::EcoVec;
use typst::foundations::{Scope, Styles, Value};
use typst::layout::PagedDocument;
use typst::syntax::{LinkedNode, SyntaxKind, Span};

use typst_ide::IdeWorld;

// Importa o trait do irmão (L4 define o namespace)
use super::ide_env::IIdeEnv;

/// Adaptador L3 que implementa `IIdeEnv` delegando para o `IdeWorld` legado.
pub struct IdeEnvAdapter<'a> {
    world: &'a dyn IdeWorld,
}

impl<'a> IdeEnvAdapter<'a> {
    pub fn new(world: &'a dyn IdeWorld) -> Self {
        Self { world }
    }
}

impl<'a> IIdeEnv for IdeEnvAdapter<'a> {
    fn resolve_globals<'b>(&'b self, leaf: &LinkedNode) -> &'b Scope {
        let in_math = matches!(
            leaf.parent_kind(),
            Some(SyntaxKind::Equation)
                | Some(SyntaxKind::Math)
                | Some(SyntaxKind::MathFrac)
                | Some(SyntaxKind::MathAttach)
        ) && leaf
            .prev_leaf()
            .is_none_or(|prev| !matches!(prev.kind(), SyntaxKind::Hash));

        let library = self.world.library();
        if in_math { library.math.scope() } else { library.global.scope() }
    }

    fn execute_import(&self, path: &str, source_span: Span) -> Option<Value> {
        use comemo::Track;
        use typst::engine::{Engine, Route, Sink, Traced};
        use typst::introspection::Introspector;
        use typst::utils::Protected;

        let introspector = Introspector::default();
        let traced = Traced::default();
        let mut sink = Sink::new();
        let mut engine = Engine {
            routines: &typst::ROUTINES,
            world: self.world.upcast().track(),
            introspector: Protected::new(introspector.track()),
            traced: traced.track(),
            sink: sink.track_mut(),
            route: Route::default(),
        };

        typst_eval::import(&mut engine, path, source_span)
            .ok()
            .map(Value::Module)
    }

    fn trace_expr(&self, span: Span) -> EcoVec<(Value, Option<Styles>)> {
        typst::trace::<PagedDocument>(self.world.upcast(), span)
    }
}
