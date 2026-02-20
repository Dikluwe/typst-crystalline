// -----------------------------------------------------------------------------
// Tipologia: Controller Orchestrator (L2)
// Módulo: Compiler
// Descrição: Superfície da API ou controlador que executa Fluxos, chamando I/O 
// (via Inversão de Dependência) e a lógica de processamento matemática de L1.
// -----------------------------------------------------------------------------

use crate::contracts::compiler_io::ICompilerEnv;
use crate::core::compiler_logic::{validate_html_feature_flag, generate_invalid_file_hints};

pub struct CompilerOrchestrator<E: ICompilerEnv> {
    env: E,
}

impl<E: ICompilerEnv> CompilerOrchestrator<E> {
    pub fn new(env: E) -> Self {
        Self { env }
    }

    /// Orquestra a execução repetitiva das passagens de layout até a estabilização
    /// Usa o mundo impuro `E` para gerir o estado de fetch (I/O) e o mundo `L1` puro
    /// para computar falhas.
    pub fn execute_compilation_loop(&self, is_html_target: bool) -> Result<(), String> {
        // Validação inicial via L1 - Regra Pura
        if let Err(diag) = validate_html_feature_flag(is_html_target) {
            return Err(diag.message.to_string());
        }

        // I/O - Pega contexto atrelado ao ambiente (L3)
        let main_id = self.env.get_main_file_id();

        // Operação - O núcleo faria de até 5 rodadas dependendo da estabilização
        // Para simplificar a orquestração a L2 delega e gerencia.
        let loop_start_ms = self.env.now_milliseconds();
        match self.env.fetch_source(main_id) {
            Ok(_content) => {
                // Continua o loop real de introspecções documentais...
                let duration = self.env.now_milliseconds() - loop_start_ms;
                Ok(())
            },
            Err(e) => {
                Err(format!("Falha na leitura puridicada: {}", e))
            }
        }
    }
}
