//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Completions
//! Responsabilidade: Extração de metadados para geração de completions.
//!
//! Nota: Este módulo é extremamente simples. A lógica pura é mínima porque
//! `clap_complete::generate` faz todo o trabalho pesado. Isolamos apenas
//! a lógica de extração do nome do binário e validação de shells.

/// Shells suportados para autocompletion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl SupportedShell {
    /// Retorna o nome do shell como string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::PowerShell => "powershell",
            Self::Elvish => "elvish",
        }
    }

    /// Lista todos os shells suportados.
    pub fn all() -> &'static [SupportedShell] {
        &[
            Self::Bash,
            Self::Zsh,
            Self::Fish,
            Self::PowerShell,
            Self::Elvish,
        ]
    }
}

/// Extrai o nome do binário de uma string de comando.
/// Tipicamente recebe o resultado de `cmd.get_name()`.
pub fn extract_bin_name(name: &str) -> String {
    name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_names() {
        assert_eq!(SupportedShell::Bash.name(), "bash");
        assert_eq!(SupportedShell::Zsh.name(), "zsh");
        assert_eq!(SupportedShell::Fish.name(), "fish");
        assert_eq!(SupportedShell::PowerShell.name(), "powershell");
        assert_eq!(SupportedShell::Elvish.name(), "elvish");
    }

    #[test]
    fn test_all_shells() {
        let all = SupportedShell::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_extract_bin_name() {
        assert_eq!(extract_bin_name("typst"), "typst");
        assert_eq!(extract_bin_name("my-tool"), "my-tool");
    }
}
