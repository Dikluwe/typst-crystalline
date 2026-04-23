//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/engine.md
//! @prompt-hash f5977cf7
//! @layer L1
//! @updated 2026-04-23
//!
//! `Engine<'a>` — agregador de estado de eval em L1.
//!
//! Consolida os 8 parâmetros antes propagados individualmente pelas
//! funções `eval_*` (ADR-0036, 5 aplicações nos Passos 92, 94, 95,
//! 98, 107). Materializado no Passo 109 conforme ADR-0044
//! (inversão controlada da ADR-0036).
//!
//! Ordem dos campos segue a coesão por domínio (ADR-0037):
//! 1. Handle externo (`world`).
//! 2. Fluxo de eval (route, styles, show_rules, active_guards,
//!    current_file, figure_numbering).
//! 3. Efeitos laterais (sink).
//!
//! Campos omitidos face ao vanilla (`introspector`, `routines`,
//! `traced`): cristalino ainda não materializou esses subsistemas
//! (ver `00_nucleo/diagnosticos/cristalino-introspection-passo-108.md`).

use std::sync::Arc;

use comemo::{Tracked, TrackedMut};

use crate::contracts::world::World;
use crate::entities::file_id::FileId;
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::sink::Sink;
use crate::entities::style_chain::StyleChain;
use crate::entities::world_types::Route;

/// Agregador de estado de eval em L1 (ADR-0044, Passo 109).
///
/// Paramétrica num único lifetime `'a`. Os campos tracked via
/// `comemo` (`route`, `sink`) conservam tracking individual.
/// `Engine<'a>` em si **não** é `#[comemo::track]`.
pub struct Engine<'a> {
    /// Handle externo — fonte de I/O do eval (world do Typst).
    pub world: &'a dyn World,

    /// Rota de compilação — detecção de ciclos e limite de
    /// profundidade (ADR-0033). Propagada por `comemo::Tracked`.
    pub route: Tracked<'a, Route<'a>>,

    /// Cadeia de estilos activa (ADR-0038).
    pub styles: &'a mut StyleChain,

    /// Regras `#show` activas (snapshot Arc).
    pub show_rules: &'a mut Arc<[ShowRule]>,

    /// Stack de IDs de regras `#show` em execução — anti-recursão.
    pub active_guards: &'a mut Vec<RuleId>,

    /// Ficheiro actual — muda em `eval_module_include`.
    pub current_file: FileId,

    /// Padrão de numeração de figuras activo (Passo 75, DEBT-14).
    pub figure_numbering: &'a mut Option<String>,

    /// Canal de warnings (ADR-0042, ADR-0043). `TrackedMut` garante
    /// que mutações passam por métodos tracked (`warn_note`).
    pub sink: &'a mut TrackedMut<'a, Sink>,

    // Stubs futuros — documentam divergência face ao vanilla:
    // pub introspector: Introspector,    // Passo dedicado.
    // pub routines: &'a Routines,         // Passo dedicado.
    // pub traced: Tracked<'a, Traced>,    // Passo dedicado.
}

// Sem testes unitários próprios — Engine é agregador transparente.
// Cobertura funcional via testes end-to-end em `rules/eval/tests.rs`
// e `03_infra/src/integration_tests.rs` (803 L1 + 184 L3 existentes).
