//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/compiler_features.md
//! @prompt-hash 1d4b316b
//! @layer L1

/// Funcionalidades experimentais habilitáveis pelo utilizador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Html,
    A11yExtras,
}

/// Conjunto fechado e idempotente de funcionalidades do compilador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Features {
    html: bool,
    a11y_extras: bool,
}

impl Features {
    pub const fn empty() -> Self {
        Self { html: false, a11y_extras: false }
    }

    pub const fn html() -> Self {
        Self { html: true, a11y_extras: false }
    }

    pub const fn contains(self, feature: Feature) -> bool {
        match feature {
            Feature::Html => self.html,
            Feature::A11yExtras => self.a11y_extras,
        }
    }

    pub fn enable(&mut self, feature: Feature) {
        match feature {
            Feature::Html => self.html = true,
            Feature::A11yExtras => self.a11y_extras = true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_empty_and_features_are_independent() {
        let mut features = Features::default();
        assert!(!features.contains(Feature::Html));
        assert!(!features.contains(Feature::A11yExtras));
        features.enable(Feature::A11yExtras);
        features.enable(Feature::A11yExtras);
        assert!(features.contains(Feature::A11yExtras));
        assert!(!features.contains(Feature::Html));
    }
}
