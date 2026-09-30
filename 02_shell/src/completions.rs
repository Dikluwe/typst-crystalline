//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/completions.md
//! @prompt-hash a73b4e86
//! @layer L2

use crate::cli::CompletionShell;

pub fn generate(shell: CompletionShell) -> Vec<u8> {
    let mut command = crate::cli::command();
    let name = command.get_name().to_string();
    let mut output = Vec::new();
    match shell {
        CompletionShell::Bash => clap_complete::generate(
            clap_complete::Shell::Bash,
            &mut command,
            name.as_str(),
            &mut output,
        ),
        CompletionShell::Elvish => clap_complete::generate(
            clap_complete::Shell::Elvish,
            &mut command,
            name.as_str(),
            &mut output,
        ),
        CompletionShell::Fish => clap_complete::generate(
            clap_complete::Shell::Fish,
            &mut command,
            name.as_str(),
            &mut output,
        ),
        CompletionShell::Nushell => clap_complete::generate(
            clap_complete_nushell::Nushell,
            &mut command,
            name.as_str(),
            &mut output,
        ),
        CompletionShell::PowerShell => clap_complete::generate(
            clap_complete::Shell::PowerShell,
            &mut command,
            name.as_str(),
            &mut output,
        ),
        CompletionShell::Zsh => clap_complete::generate(
            clap_complete::Shell::Zsh,
            &mut command,
            name.as_str(),
            &mut output,
        ),
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_comes_from_the_real_clap_tree() {
        let output = String::from_utf8(generate(CompletionShell::Bash)).unwrap();
        assert!(output.contains("compile"));
        assert!(output.contains("fonts"));
        assert!(output.contains("query"));
    }

    #[test]
    fn nushell_comes_from_the_real_clap_tree() {
        let output = String::from_utf8(generate(CompletionShell::Nushell)).unwrap();
        assert!(output.contains("module completions {"));
        assert!(output.contains("export extern typst"));
        for command in ["compile", "fonts", "completions", "watch"] {
            assert!(output.contains(command), "comando ausente: {command}");
        }
    }
}
