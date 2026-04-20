//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/ptr-eq-arc.md
//! @prompt-hash 45015de6
//! @layer L1
//! @updated 2026-04-20

use std::sync::Arc;

/// `Arc<T>` com `PartialEq` e `Hash` por ponteiro em vez de por valor.
///
/// `Arc<Vec<u8>>` com `PartialEq` derivado desreferencia e compara byte a byte
/// — O(N) onde N é o tamanho dos dados. Para imagens grandes (JPEGs de 5 MB)
/// isso é inaceitável em estruturas comparadas frequentemente.
///
/// `PtrEqArc` compara apenas o endereço do bloco de controlo do Arc — O(1).
/// Seguro enquanto os dados forem imutáveis (`Vec<u8>` não é mutado após criação).
#[derive(Debug, Clone)]
pub struct PtrEqArc<T>(pub Arc<T>);

impl<T> PartialEq for PtrEqArc<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for PtrEqArc<T> {}

impl<T> std::hash::Hash for PtrEqArc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (Arc::as_ptr(&self.0) as usize).hash(state);
    }
}

impl<T> std::ops::Deref for PtrEqArc<T> {
    type Target = Arc<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptr_eq_arc_compara_por_ponteiro() {
        let arc1 = Arc::new(vec![1u8, 2, 3]);
        let arc2 = Arc::clone(&arc1);
        let arc3 = Arc::new(vec![1u8, 2, 3]); // mesmo conteúdo, ponteiro diferente

        let p1 = PtrEqArc(arc1);
        let p2 = PtrEqArc(arc2);
        let p3 = PtrEqArc(arc3);

        assert_eq!(p1, p2, "Clones do mesmo Arc são iguais por ponteiro");
        assert_ne!(p1, p3, "Arcs diferentes com mesmo conteúdo são desiguais");
    }

    #[test]
    fn ptr_eq_arc_deref_para_arc() {
        let arc = Arc::new(vec![42u8]);
        let p = PtrEqArc(Arc::clone(&arc));
        assert!(Arc::ptr_eq(&p.0, &arc));
    }

    #[test]
    fn ptr_eq_arc_hash_consistente_com_eq() {
        use std::collections::HashSet;
        let arc1 = Arc::new(vec![1u8]);
        let arc2 = Arc::clone(&arc1);
        let p1 = PtrEqArc(arc1);
        let p2 = PtrEqArc(arc2);
        // Dois valores iguais devem ter o mesmo hash (invariante de Hash)
        let mut set = HashSet::new();
        set.insert(p1);
        assert!(set.contains(&p2), "clone deve estar no set");
    }
}
