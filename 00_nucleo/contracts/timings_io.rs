//! Contratos de Interação Externa detectados no `timings.rs`
//! L3 - Camada Física / Interfaces de Efeitos Colaterais de Timing

use std::path::Path;

/// Interface abstrata para um exportador de tempos de execução.
/// Abstrai a dependência global do crate `typst_timing` e do sistema de arquivos.
pub trait ITimingExporter {
    /// Ativa a gravação de métricas globais.
    fn enable(&self);
    
    /// Limpa métricas anteriormente gravadas.
    fn clear(&self);
    
    /// Exporta o relatório gravado para o caminho do arquivo JSON específicado,
    /// resolvendo os spans identificadores.
    fn export_json(
        &self,
        path: &Path,
        resolve_span: &dyn Fn(u64) -> (String, u32),
    ) -> Result<(), String>;
}
