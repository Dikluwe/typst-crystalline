// Diagnostic extraction of historical Coverage; not a production module.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Coverage(Vec<u32>);

impl Coverage {
    /// Cobertura vazia.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Constrói a partir de um vector de codepoints.
    /// Ordena, remove duplicados e codifica em runs.
    ///
    /// Port do vanilla `Coverage::from_vec` (info.rs:291-310).
    ///
    /// **P942** — duas correcções de desempenho face à versão anterior:
    /// 1. `impl Into<Vec<u32>>` em vez de `impl IntoIterator` — para um `Vec`
    ///    existente a conversão é identidade (sem cópia); a versão anterior
    ///    re-colacionava o Vec inteiro (~115 MB em ~1086 fontes do sistema).
    /// 2. `sort()` (TimSort estável) em vez de `sort_unstable()` — os
    ///    codepoints da `cmap` vêm já ordenados, e o TimSort é O(n) em input
    ///    ordenado/quase-ordenado, enquanto o pdqsort é O(n log n) sempre.
    ///    É também exactamente o método do vanilla (`info.rs:292`).
    pub fn from_codepoints(codepoints: impl Into<Vec<u32>>) -> Self {
        let mut codepoints: Vec<u32> = codepoints.into();
        codepoints.sort();
        codepoints.dedup();

        let mut runs = Vec::new();
        let mut next = 0u32;

        for c in codepoints {
            if let Some(run) = runs.last_mut().filter(|_| c == next) {
                *run += 1;
            } else {
                runs.push(c - next);
                runs.push(1);
            }
            next = c + 1;
        }

        Self(runs)
    }

    /// Verifica se o codepoint está coberto. O(n) no número de runs; o vanilla
    /// usa a mesma abordagem (info.rs:313-326).
    ///
    /// **P942** — terminação antecipada: como os runs estão ordenados, quando
    /// `cursor` ultrapassa o codepoint ele já não pode estar em nenhum run
    /// seguinte. Para fontes que não cobrem o caractere (a maioria, durante o
    /// fallback), o scan pára cedo em vez de percorrer todos os runs — medido:
    /// reduz significativamente o custo de `candidates_for_char`.
    pub fn contains(&self, codepoint: u32) -> bool {
        let mut inside = false;
        let mut cursor = 0u32;

        for &run in &self.0 {
            if cursor > codepoint {
                return false;
            }
            if (cursor..cursor + run).contains(&codepoint) {
                return inside;
            }
            cursor += run;
            inside = !inside;
        }

        false
    }

    /// True se não cobre nenhum codepoint.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Itera todos os codepoints cobertos (útil para testes e debug).
    /// Port do vanilla `Coverage::iter` (info.rs:329-338).
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        let mut inside = false;
        let mut cursor = 0u32;
        self.0.iter().flat_map(move |run| {
            let range = if inside { cursor..cursor + run } else { 0..0 };
            inside = !inside;
            cursor += run;
            range
        })
    }
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Coverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coverage").field(&self.0).finish()
    }
}

