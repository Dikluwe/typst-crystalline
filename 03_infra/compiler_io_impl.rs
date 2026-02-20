// -----------------------------------------------------------------------------
// Tipologia: Infrastructure Adapter (L3)
// Módulo: Compiler
// Descrição: Implementação técnica dos efeitos colaterais e ponteiros para I/O externo.
// -----------------------------------------------------------------------------

use crate::contracts::compiler_io::ICompilerEnv;

/// Infra Layer (L3) - Implementação técnica dos efeitos colaterais para o motor.
/// Na vida real, estaria amarrado ao `typst::World` herdado pelo usuário final.
pub struct CompileEnvAdapter {
    // Referência ao `World` ou FS nativo.
}

impl CompileEnvAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICompilerEnv for CompileEnvAdapter {
    fn get_main_file_id(&self) -> u64 {
        // Adapta chamada impura nativa.
        // ex: self.world.main().into()...
        1 // Mock de Main File Id para fins de spec-compilation.
    }

    fn fetch_source(&self, _file_id: u64) -> Result<String, String> {
        // Lê os bytes do arquivo fonte do mundo externo (FS ou rede/VFS)
        Ok("= Mock Content".to_string())
    }

    fn now_milliseconds(&self) -> f64 {
        // Uso de libs impuras como SystemTime
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64
    }
}
