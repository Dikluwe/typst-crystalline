//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/completions.md
//! @prompt-hash e7265331
//! @layer L2

pub fn generate(shell: clap_complete::Shell) -> Vec<u8> {
    let mut command = crate::cli::command();
    let name = command.get_name().to_string();
    let mut output = Vec::new();
    clap_complete::generate(shell, &mut command, name, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_comes_from_the_real_clap_tree() {
        let output = String::from_utf8(generate(clap_complete::Shell::Bash)).unwrap();
        assert!(output.contains("compile"));
        assert!(output.contains("fonts"));
        assert!(output.contains("query"));
    }
}
