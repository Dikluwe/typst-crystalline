//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/world-types.md
//! @prompt-hash 20d76d54
//! @layer L1
//! @updated 2026-03-22

/// Conteúdo binário de um ficheiro carregado.
/// Interior provisório — pode mudar de `Vec<u8>` para o tipo real no Passo 5.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Fonte tipográfica carregada.
/// Opaca até Font ser migrado no Passo 5.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Font(Vec<u8>);

impl Font {
    pub fn from_data(data: Vec<u8>) -> Self { Self(data) }
    pub fn as_slice(&self) -> &[u8] { &self.0 }
}

/// Biblioteca de funções e valores do Typst.
/// Opaca até Library ser migrada no Passo 4.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Library(());

impl Library {
    pub fn new() -> Self { Self(()) }
}

/// Catálogo de fontes com metadados.
/// Opaco até FontBook ser migrado no Passo 5.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FontBook(());

impl FontBook {
    pub fn new() -> Self { Self(()) }
}

/// Data e hora para o método `today()` de `World`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Datetime {
    pub year:  i32,
    pub month: u8,
    pub day:   u8,
}

/// Erro de acesso a ficheiro.
#[derive(Clone, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum FileError {
    #[error("file not found")]
    NotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("{0}")]
    Other(String),
}

/// Resultado de uma operação de ficheiro.
pub type FileResult<T> = Result<T, FileError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_new_and_slice() {
        let b = Bytes::new(vec![1, 2, 3]);
        assert_eq!(b.as_slice(), &[1u8, 2, 3]);
        assert_eq!(b.len(), 3);
        assert!(!b.is_empty());
    }

    #[test]
    fn bytes_empty() {
        let b = Bytes::new(vec![]);
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn file_error_not_found_display() {
        assert_eq!(FileError::NotFound.to_string(), "file not found");
    }

    #[test]
    fn file_error_access_denied_display() {
        assert_eq!(FileError::AccessDenied.to_string(), "access denied");
    }

    #[test]
    fn file_error_other_display() {
        assert_eq!(FileError::Other("custom".to_string()).to_string(), "custom");
    }

    #[test]
    fn library_and_font_book_opaque() {
        // Verifica que compilam; interior não é acessível fora do módulo.
        // Contrato correcto — teste adicionado para prevenir regressão.
        let _lib = Library::new();
        let _book = FontBook::new();
    }
}
