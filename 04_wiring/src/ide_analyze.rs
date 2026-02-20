// 💎 Tipologia Tekt: L4 (Wiring & Transição Híbrida)
// Composição final: L0(Contrato) + L1(Lógica) + L2(Orquestrador) + L3(Infra) + L20(IdeWorld)
//
// Este módulo é o ponto central de namespace. Ele inclui todos os sub-módulos
// via #[path] para que eles compartilhem o mesmo trait `IIdeEnv` via `super::`.

// L0 - Contrato (único ponto de verdade)
#[path = "../../00_nucleo/contracts/ide_env.rs"]
pub mod ide_env;

// L1 - Lógica Pura
#[path = "../../01_core/ide_analyze_logic.rs"]
pub mod ide_analyze_logic;

// L2 - Orquestrador (usa super::ide_env e super::ide_analyze_logic)
#[path = "../../02_shell/ide_analyze_service.rs"]
pub mod ide_analyze_service;

// L3 - Implementação de I/O (usa super::ide_env)
#[path = "../../03_infra/ide_env_impl.rs"]
pub mod ide_env_impl;

use ide_analyze_service::IdeAnalyzerOrchestrator;
use ide_env_impl::IdeEnvAdapter;

use ecow::EcoVec;
use typst::foundations::{Styles, Value};
use typst::syntax::LinkedNode;
use typst_ide::IdeWorld;

/// Proxy Strangler Fig: O ponto de entrada L4 que compõe a arquitetura Tekt completa.
pub struct AnalyzeProxy<'a> {
    orchestrator: IdeAnalyzerOrchestrator<'a, IdeEnvAdapter<'a>>,
}

impl<'a> AnalyzeProxy<'a> {
    pub fn new(world: &'a dyn IdeWorld) -> Self {
        let env = IdeEnvAdapter::new(world);
        let env_ref = Box::leak(Box::new(env));
        Self {
            orchestrator: IdeAnalyzerOrchestrator::new(env_ref),
        }
    }

    pub fn analyze_expr(&self, node: &LinkedNode) -> EcoVec<(Value, Option<Styles>)> {
        self.orchestrator.analyze_expr(node)
    }

    pub fn analyze_expr_with_fallback(&self, node: &LinkedNode) -> Option<Value> {
        self.orchestrator.analyze_expr_with_fallback(node)
    }

    pub fn analyze_import(&self, source: &LinkedNode) -> Option<Value> {
        self.orchestrator.analyze_import(source)
    }
}
