/// A Facet Lattice do diagnóstico exige que o Core seja silencioso.
/// Esta trait define os "ganchos" que o Shell (L2) deve implementar para
/// dar uma narrativa aos sinais técnicos.
pub trait NarrativeVisitor {
    fn on_access_denied(&mut self, signal: &AccessSignal);
    fn on_type_mismatch(&mut self, signal: &TypeMismatchSignal);
    fn on_missing_field(&mut self, signal: &MissingFieldSignal);
    // Adicione novos sinais conforme a evolução do Core.
}

/// Law II: O sinal é um objeto de dados estável e invariante.
/// Ele não contém mensagens, apenas os dados brutos do colapso lógico.
pub trait VoidSignal: std::fmt::Debug + Send + Sync {
    /// Law III: Double Dispatch para evitar downcasting e manter a pureza.
    fn accept(&self, visitor: &mut dyn NarrativeVisitor);
}

// --- Implementações de Sinais (Exemplos de Colapsos Geométricos) ---

#[derive(Debug, Clone)]
pub struct AccessSignal {
    pub target: String,
    pub property: String,
}

impl VoidSignal for AccessSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_access_denied(self);
    }
}

#[derive(Debug, Clone)]
pub struct TypeMismatchSignal {
    pub expected: String,
    pub found: String,
}

impl VoidSignal for TypeMismatchSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_type_mismatch(self);
    }
}

#[derive(Debug, Clone)]
pub struct MissingFieldSignal {
    pub parent_type: String,
    pub field_name: String,
}

impl VoidSignal for MissingFieldSignal {
    fn accept(&self, visitor: &mut dyn NarrativeVisitor) {
        visitor.on_missing_field(self);
    }
}

/// Tipo utilitário para o Core retornar falhas sem interromper o fluxo com strings.
pub type CrystalResult<T> = Result<T, Box<dyn VoidSignal>>;
