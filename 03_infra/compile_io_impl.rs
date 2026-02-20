//! L3 - Infraestrutura para compilação (ex: aberturas de processos)

pub mod compile_io {
    pub trait IProcessOpener {
        fn open(&self, target: &str) -> std::io::Result<()>;
    }
}

pub struct SystemProcessOpener;

impl compile_io::IProcessOpener for SystemProcessOpener {
    fn open(&self, _target: &str) -> std::io::Result<()> {
        // Implementação simulada 
        Ok(())
    }
}
