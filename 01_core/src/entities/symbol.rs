//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/symbol.md
//! @prompt-hash 45feb053
//! @layer L1
//! @updated 2026-07-15
//!
//! Grapheme Unicode nomeado. P471 subset minimal + P765a modifiers/constructor
//! + P1162 valores multi-codepoint.
//!
//! O vanilla representa um symbol como um grapheme base mais uma lista de
//! variantes `(modifiers, value)`. Modificadores aplicados via field access
//! (`sym.arrow.r.filled`) seleccionam a variante que contém todos os
//! modificadores pedidos e o menor número de modificadores extra. O
//! construtor `symbol(...)` cria symbols runtime com a mesma estrutura.

use ecow::EcoString;

/// Uma variante de símbolo: `("r.filled", "➡")`.
pub type SymbolVariant = (EcoString, EcoString);

/// Símbolo Unicode nomeado.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// Grapheme cluster efectivo após modifiers aplicados.
    pub value: EcoString,
    /// Nome canónico base (ex.: `"arrow"`).
    pub name: EcoString,
    /// Variantes disponíveis para modifiers. Para symbols simples ou
    /// runtime, a variante sem modifiers (string vazia) é o caractere base.
    pub variants: Vec<SymbolVariant>,
    /// Modifiers já aplicados via field access, na ordem em que foram
    /// pedidos (a comparação canoniza a ordem alfabética).
    pub applied: Vec<EcoString>,
}

impl Symbol {
    /// Cria um símbolo simples sem variantes nomeadas.
    pub fn new(value: impl Into<EcoString>, name: impl Into<EcoString>) -> Self {
        let value = value.into();
        let name = name.into();
        Self {
            value: value.clone(),
            name: name.clone(),
            variants: vec![(EcoString::default(), value)],
            applied: Vec::new(),
        }
    }

    /// Cria um símbolo com variantes (usado pelo módulo `sym`).
    pub fn with_variants(
        value: impl Into<EcoString>,
        name: impl Into<EcoString>,
        variants: Vec<SymbolVariant>,
    ) -> Self {
        Self {
            value: value.into(),
            name: name.into(),
            variants,
            applied: Vec::new(),
        }
    }

    /// Cria um symbol runtime a partir de uma lista de variantes.
    /// A primeira variante sem modifiers define o caractere base.
    pub fn runtime(variants: Vec<SymbolVariant>) -> Self {
        let value = variants
            .iter()
            .find(|(m, _)| m.is_empty())
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| variants[0].1.clone());
        Self {
            value,
            name: EcoString::default(),
            variants,
            applied: Vec::new(),
        }
    }

    /// Aplica um modifier, devolvendo um novo `Symbol` se existir uma
    /// variante compatível. Segue o algoritmo do vanilla: a variante deve
    /// conter todos os modifiers já aplicados mais o novo; entre as
    /// compatíveis, escolhe-se a com menos modifiers extra.
    pub fn modified(&self, modifier: &str) -> Option<Self> {
        // Symbols simples sem variantes reais não aceitam modifiers.
        if self.variants.len() == 1 && self.variants[0].0.is_empty() {
            return None;
        }

        let mut applied: Vec<EcoString> = self.applied.clone();
        if !applied.iter().any(|m| m.as_str() == modifier) {
            applied.push(modifier.into());
        }

        let best = self
            .variants
            .iter()
            .filter(|(mods, _)| {
                applied
                    .iter()
                    .all(|a| mods.as_str().split('.').any(|m| m == a.as_str()))
            })
            .min_by_key(|(mods, _)| {
                // Conta modifiers extra além dos aplicados.
                let mod_count =
                    if mods.is_empty() { 0 } else { mods.as_str().split('.').count() };
                mod_count.saturating_sub(applied.len())
            });

        best.map(|(_, value)| Self {
            value: value.clone(),
            name: self.name.clone(),
            variants: self.variants.clone(),
            applied,
        })
    }

    /// Representação das variants para `repr()`. Filtra variants que não
    /// contêm todos os modifiers aplicados; omite os modifiers já aplicados
    /// da chave. O caractere base (sem modifiers restantes) aparece como
    /// string literal.
    pub fn repr_variants(&self) -> String {
        fn repr_text(value: &str) -> String {
            let mut escaped = String::new();
            for ch in value.chars() {
                if ch == '\u{200d}'
                    || ('\u{fe00}'..='\u{fe0f}').contains(&ch)
                    || ('\u{e0100}'..='\u{e01ef}').contains(&ch)
                {
                    escaped.push_str(&format!("\\u{{{:x}}}", ch as u32));
                } else {
                    escaped.extend(ch.escape_debug());
                }
            }
            format!("\"{}\"", escaped.replace('"', "\\\""))
        }

        let items: Vec<String> = self
            .variants
            .iter()
            .filter(|(mods, _)| {
                self.applied
                    .iter()
                    .all(|a| mods.as_str().split('.').any(|m| m == a.as_str()))
            })
            .map(|(mods, value)| {
                let trimmed: Vec<&str> = mods
                    .as_str()
                    .split('.')
                    .filter(|m| {
                        !m.is_empty() && !self.applied.iter().any(|a| a.as_str() == *m)
                    })
                    .collect();
                if trimmed.is_empty() {
                    repr_text(value)
                } else {
                    let key = trimmed.join(".");
                    format!("({}, {})", repr_char_str(&key), repr_text(value))
                }
            })
            .collect();

        if items.is_empty() {
            repr_text(&self.value)
        } else {
            pretty_array_inner(&items)
        }
    }
}

/// Conteúdo de um construto array-like sem os parênteses externos.
/// Mantém aqui a mecânica privada para não criar import reverso
/// `entities -> compiler` nem ampliar contrato público por conveniência.
fn pretty_array_inner(parts: &[String]) -> String {
    const MAX_WIDTH: usize = 50;
    let len =
        parts.iter().map(String::len).sum::<usize>() + 2 * parts.len().saturating_sub(1);

    if len <= MAX_WIDTH {
        parts.join(", ")
    } else {
        let mut output = String::from("\n");
        for part in parts {
            output.push_str("  ");
            output.push_str(part.trim());
            output.push_str(",\n");
        }
        output
    }
}

fn repr_char_str(s: &str) -> String {
    format!("\"{}\"", s.escape_debug().collect::<String>().replace('"', "\\\""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_new_preserva_campos() {
        let s = Symbol::new('→', "arrow");
        assert_eq!(s.value, "→");
        assert_eq!(s.name.as_str(), "arrow");
    }

    #[test]
    fn symbol_igualdade() {
        let a = Symbol::new('α', "alpha");
        let b = Symbol::new('α', "alpha");
        assert_eq!(a, b);
    }

    #[test]
    fn symbol_hash_via_derive() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Symbol::new('→', "arrow"));
        set.insert(Symbol::new('→', "arrow"));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn symbol_modified_simples_falha() {
        let s = Symbol::new('→', "arrow");
        assert!(s.modified("r").is_none());
    }

    #[test]
    fn symbol_modified_com_variants() {
        let s = Symbol::with_variants(
            '→',
            "arrow",
            vec![
                (EcoString::default(), "→".into()),
                ("r".into(), "→".into()),
                ("l".into(), "←".into()),
                ("r.filled".into(), "➡".into()),
                ("l.filled".into(), "⬌".into()),
            ],
        );
        let r = s.modified("r").unwrap();
        assert_eq!(r.value, "→");
        let filled = r.modified("filled").unwrap();
        assert_eq!(filled.value, "➡");
    }

    #[test]
    fn symbol_runtime_repr() {
        let s = Symbol::runtime(vec![
            ("bold".into(), "α".into()),
            ("italic".into(), "α".into()),
        ]);
        let repr = s.repr_variants();
        assert!(repr.contains("bold"));
        assert!(repr.contains("italic"));
    }
}
