//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/bytes.md
//! @prompt-hash e54fd1e3
//! @layer L1
//! @updated 2026-06-22
//!
//! Sequência binária opaca — `Value::Bytes`.
//! Passo 398 (fecha DEBT-62): tipo L1 puro, zero I/O.

/// Sequência de bytes opaca.
///
/// Wrapper sobre `Vec<u8>` — já é heap-allocated, pelo que `Arc` não traz
/// benefício imediato. `EcoVec<u8>` é refino futuro (XS).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    /// Cria `Bytes` a partir de um `Vec<u8>`.
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Número de bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// True se a sequência estiver vazia.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Acesso como slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(bytes: Bytes) -> Self {
        bytes.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_new() {
        let b = Bytes::new(vec![0x89, 0x50, 0x4e, 0x47]);
        assert_eq!(b.len(), 4);
        assert!(!b.is_empty());
    }

    #[test]
    fn bytes_empty() {
        let b = Bytes::default();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn bytes_equality() {
        let a = Bytes::new(vec![1, 2, 3]);
        let b = Bytes::new(vec![1, 2, 3]);
        let c = Bytes::new(vec![3, 2, 1]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn bytes_clone() {
        let a = Bytes::new(vec![0xDE, 0xAD]);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn bytes_from_vec() {
        let v = vec![1, 2, 3];
        let b: Bytes = v.clone().into();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn bytes_into_vec() {
        let b = Bytes::new(vec![4, 5, 6]);
        let v: Vec<u8> = b.into();
        assert_eq!(v, vec![4, 5, 6]);
    }
}
