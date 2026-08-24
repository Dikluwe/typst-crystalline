//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/version.md
//! @prompt-hash 5dafd786
//! @layer L1
//! @updated 2026-07-10
//!
//! Versão do Typst — `Value::Version`.
//! Passo 401: tipo L1 puro, zero I/O.
//! Passo 684: corrigido — Typst `version` é uma sequência arbitrária de
//! componentes **inteiros** (os três primeiros têm nome), sem `pre`/`build`
//! (essa leitura vinha de uma confusão com SemVer 2.0.0, não do Typst).
//! Passo 796: `.at(index)` (método de instância) e `PARITY_VERSION`
//! (constante partilhada com `sys.rs` e `typst-shell::cli`).

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Versão de paridade com a linguagem Typst (não a versão do crate/binário
/// cristalino, que fica em `Cargo.toml`). Fonte única consumida por
/// `sys.version` (`compiler/stdlib/sys.rs`) e por `--version` do CLI
/// (`02_shell/src/cli.rs`) — P796 fecha a inconsistência entre os dois.
pub const PARITY_VERSION: (u64, u64, u64) = (0, 15, 1);

/// Versão do Typst: sequência arbitrária de componentes inteiros.
///
/// Os componentes são guardados **como foram dados** (sem remover zeros à
/// direita), para que `repr(version(0, 11, 0))` seja `"version(0, 11, 0)"`,
/// como no vanilla. A igualdade e a ordenação tratam componentes em falta como
/// `0` (zero-pad): `version(1, 2, 3) == version(1, 2, 3, 0)` e
/// `version(1, 2, 3) < version(1, 2, 3, 4)`. Por isso `PartialEq`/`Eq`/`Hash`
/// e `Ord` são implementados à mão (não derivados).
#[derive(Debug, Clone)]
pub struct Version {
    /// Componentes inteiros (≥ 0), tal como fornecidos (zeros à direita
    /// preservados para `repr`).
    pub components: Vec<u64>,
}

impl Version {
    /// Cria uma versão a partir dos três componentes com nome.
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self::from_components(vec![major, minor, patch])
    }

    /// Cria uma versão a partir de qualquer número de componentes (guardados
    /// como dados).
    pub fn from_components(components: Vec<u64>) -> Self {
        Self { components }
    }

    /// Componente na posição `i` (0 se ausente — semântica zero-pad).
    pub fn component(&self, i: usize) -> u64 {
        self.components.get(i).copied().unwrap_or(0)
    }

    /// Primeiro componente (nome `major`); 0 se ausente.
    pub fn major(&self) -> u64 {
        self.component(0)
    }

    /// Segundo componente (nome `minor`); 0 se ausente.
    pub fn minor(&self) -> u64 {
        self.component(1)
    }

    /// Terceiro componente (nome `patch`); 0 se ausente.
    pub fn patch(&self) -> u64 {
        self.component(2)
    }

    /// Formato canónico `"c0.c1.c2…"` (string vazia para `version()`).
    pub fn to_string(&self) -> String {
        self.components
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }

    /// Parse de uma string de componentes inteiros separados por `.` (ex.:
    /// `"1.2.3"`, `"1.2.3.4"`). Rejeita string vazia, não-inteiros e negativos.
    /// Não aceita `pre`/`build` textuais (não existem no Typst).
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let mut components = Vec::new();
        for part in s.split('.') {
            let n = part.parse::<u64>().ok()?;
            components.push(n);
        }
        Some(Self::from_components(components))
    }

    /// Componente na posição `index` (P796). Índice negativo conta a partir
    /// do fim da lista de componentes **explícita** (`self.components.len()`,
    /// não a sequência infinita zero-pad usada por `component`/`Ord`/`Eq`).
    /// Índice positivo além do comprimento explícito devolve `0` (zero-pad,
    /// igual a `component`). Índice negativo fora de limites é erro — a
    /// mensagem replica o vanilla ao carácter (ADR-0108, excepção: mecânica
    /// é o observável em mensagens de erro).
    pub fn at(&self, index: i64) -> Result<i64, String> {
        let len = self.components.len() as i64;
        let resolved = if index < 0 {
            match len.checked_add(index) {
                Some(pos) if pos >= 0 => pos,
                _ => {
                    return Err(format!(
                        "component index out of bounds (index: {index}, len: {len})"
                    ))
                }
            }
        } else {
            index
        };
        Ok(usize::try_from(resolved)
            .ok()
            .and_then(|i| self.components.get(i).copied())
            .unwrap_or(0) as i64)
    }

    /// Número de componentes sem zeros à direita (representante canónico da
    /// classe de equivalência zero-pad) — usado por `Hash`.
    fn significant_len(&self) -> usize {
        let mut n = self.components.len();
        while n > 0 && self.components[n - 1] == 0 {
            n -= 1;
        }
        n
    }
}

impl Default for Version {
    fn default() -> Self {
        Self { components: Vec::new() }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let len = self.components.len().max(other.components.len());
        for i in 0..len {
            if self.component(i) != other.component(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Version {}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash consistente com `PartialEq` (zero-pad): ignora zeros à direita,
        // de modo que `version(1, 2, 3)` e `version(1, 2, 3, 0)` colidem.
        let n = self.significant_len();
        self.components[..n].hash(state);
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = self.components.len().max(other.components.len());
        for i in 0..len {
            let ord = self.component(i).cmp(&other.component(i));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_new() {
        let v = Version::new(1, 2, 3);
        assert_eq!(v.components, vec![1, 2, 3]);
        assert_eq!(v.major(), 1);
        assert_eq!(v.minor(), 2);
        assert_eq!(v.patch(), 3);
    }

    #[test]
    fn version_from_components_arbitrary() {
        let v = Version::from_components(vec![1, 2, 3, 4, 5]);
        assert_eq!(v.components, vec![1, 2, 3, 4, 5]);
        assert_eq!(v.component(4), 5);
        assert_eq!(v.component(9), 0); // ausente → 0 (zero-pad)
    }

    #[test]
    fn version_keeps_trailing_zeros_in_storage() {
        // `repr` precisa dos componentes como dados; a igualdade ignora-os.
        let a = Version::from_components(vec![1, 2, 3, 0, 0]);
        assert_eq!(a.components, vec![1, 2, 3, 0, 0]);
        assert_eq!(a, Version::new(1, 2, 3));
    }

    #[test]
    fn version_to_string_core() {
        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
        assert_eq!(
            Version::from_components(vec![1, 2, 3, 4, 5]).to_string(),
            "1.2.3.4.5"
        );
        assert_eq!(Version::from_components(vec![1, 2, 3, 0]).to_string(), "1.2.3.0");
        assert_eq!(Version::default().to_string(), "");
    }

    #[test]
    fn version_parse_core() {
        assert_eq!(Version::from_str("1.2.3").unwrap(), Version::new(1, 2, 3));
        assert_eq!(
            Version::from_str("1.2.3.4.5").unwrap(),
            Version::from_components(vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn parity_version_segue_vanilla_ratificado_p1137() {
        assert_eq!(PARITY_VERSION, (0, 15, 1));
    }

    #[test]
    fn version_parse_invalid() {
        assert!(Version::from_str("").is_none());
        assert!(Version::from_str("abc").is_none());
        assert!(Version::from_str("1.2.3-alpha.1").is_none()); // sem pre textual
        assert!(Version::from_str("1.2.3+build.2").is_none()); // sem build textual
    }

    #[test]
    fn version_default_is_empty() {
        assert_eq!(Version::default(), Version::from_components(vec![]));
        assert_eq!(Version::default(), Version::from_components(vec![0, 0])); // zero-pad
    }

    // ── Igualdade / ordenação (zero-pad, paridade vanilla 0.15.0) ────────────

    #[test]
    fn version_eq_zero_pad() {
        assert_eq!(Version::new(1, 2, 3), Version::from_components(vec![1, 2, 3, 0]));
        assert_ne!(Version::new(1, 2, 3), Version::from_components(vec![1, 2, 3, 4]));
    }

    #[test]
    fn version_hash_consistent_with_eq() {
        use std::collections::hash_map::DefaultHasher;
        fn h(v: &Version) -> u64 {
            let mut hasher = DefaultHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        }
        let a = Version::new(1, 2, 3);
        let b = Version::from_components(vec![1, 2, 3, 0]);
        assert_eq!(a, b);
        assert_eq!(h(&a), h(&b));
    }

    #[test]
    fn version_ord_lexicographic() {
        assert!(Version::new(1, 0, 0) < Version::new(2, 0, 0));
        assert!(Version::new(1, 1, 0) < Version::new(1, 2, 0));
        assert!(Version::new(1, 0, 1) < Version::new(1, 0, 2));
        // mais curto (prefixo) < mais longo
        assert!(Version::new(1, 2, 3) < Version::from_components(vec![1, 2, 3, 4]));
        // componente a componente
        assert!(Version::from_components(vec![1, 2, 3, 0]) < Version::new(1, 2, 4));
        // trailing zero não altera a ordem
        assert_eq!(
            Version::new(1, 2, 3).cmp(&Version::from_components(vec![1, 2, 3, 0])),
            Ordering::Equal
        );
    }
}
